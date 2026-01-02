//! # Firefly Window Protocol
//!
//! Protocolo de comunicação entre clientes (apps) e compositor.

use crate::ipc::{Port, SharedMemory, ShmId};
use crate::syscall::SysResult;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Nome da porta do compositor
pub const COMPOSITOR_PORT: &str = "firefly.compositor";

/// Tamanho máximo de mensagem
pub const MAX_MSG_SIZE: usize = 256;

// ============================================================================
// ============================================================================
// PROTOCOLO FIREFLY (Versão 2.1)
// ============================================================================

/// Identificadores de Mensagem (OpCodes)
pub mod opcodes {
    // Client -> Server
    pub const CREATE_WINDOW: u32 = 0x01;
    pub const DESTROY_WINDOW: u32 = 0x02;
    pub const COMMIT_BUFFER: u32 = 0x03;
    pub const INPUT_UPDATE: u32 = 0x04;

    // Server -> Client
    pub const WINDOW_CREATED: u32 = 0x10;
    pub const EVENT_INPUT: u32 = 0x20;
    pub const EVENT_RESIZE: u32 = 0x21;
    pub const ERROR: u32 = 0xFF;
}

use crate::event::{InputEvent, ResizeEvent};

// ----------------------------------------------------------------------------
// REQUESTS (Client -> Server)
// ----------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CreateWindowRequest {
    pub op: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    /// Nome da porta onde o servidor deve responder
    pub reply_port: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DestroyWindowRequest {
    pub op: u32, // DESTROY_WINDOW
    pub window_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CommitBufferRequest {
    pub op: u32, // COMMIT_BUFFER
    pub window_id: u32,
    pub x: u32,      // Dirty Rect X
    pub y: u32,      // Dirty Rect Y
    pub width: u32,  // Dirty Rect W
    pub height: u32, // Dirty Rect H
}

// ----------------------------------------------------------------------------
// RESPONSES (Server -> Client)
// ----------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WindowCreatedResponse {
    pub op: u32, // WINDOW_CREATED
    pub window_id: u32,
    pub shm_handle: u64, // Handle para memória compartilhada
    pub buffer_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ErrorResponse {
    pub op: u32, // ERROR
    pub code: u32,
}

/// União de todas as mensagens possíveis (para leitura genérica)
#[repr(C)]
pub union ProtocolMessage {
    pub header: u32, // Apenas para ler o OpCode
    pub create_req: CreateWindowRequest,
    pub buf_req: CommitBufferRequest,
    pub win_resp: WindowCreatedResponse,
    pub input_evt: InputEvent,
    pub resize_evt: ResizeEvent,
    pub raw: [u8; 256], // Padding aumentado para acomodar novas structs
}

// ============================================================================
// CLIENTE DE JANELA
// ============================================================================

/// Handle para janela no compositor
pub struct Window {
    pub id: u32,
    pub shm: SharedMemory,
    pub width: u32,
    pub height: u32,
    compositor_port: Port,
    event_port: Port,
}

impl Window {
    /// Cria nova janela com flags específicas
    pub fn create_with_flags(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        flags: u32,
    ) -> SysResult<Self> {
        // 1. Criar porta de resposta única
        // Tenta gerar nome único
        let event_port;
        let mut port_name_buf = [0u8; 32];
        let mut seed = 0;

        loop {
            // "win.r.<seed>"
            // Formatação manual (no_std)
            let prefix = b"win.r.";

            // Copia prefixo manualmente
            let mut i = 0;
            while i < prefix.len() {
                port_name_buf[i] = prefix[i];
                i += 1;
            }

            // Simples itoa para seed (0..999)
            let mut n = seed;
            if n == 0 {
                port_name_buf[i] = b'0';
                i += 1;
            } else {
                // conta digitos
                let mut temp = n;
                let mut digits = 0;
                while temp > 0 {
                    temp /= 10;
                    digits += 1;
                }

                let mut pos = i + digits;
                let end = pos;
                while pos > i {
                    port_name_buf[pos - 1] = b'0' + (n % 10) as u8;
                    n /= 10;
                    pos -= 1;
                }
                i = end;
            }

            // Zera o resto
            for k in i..32 {
                port_name_buf[k] = 0;
            }

            // Tenta criar
            let name_str = core::str::from_utf8(&port_name_buf[0..i]).unwrap_or("");
            /*
            crate::println!(
                "[RedPower] Tentando criar porta de resposta: '{}'",
                name_str
            );
            */

            match Port::create(name_str, 16) {
                Ok(p) => {
                    // crate::println!("[RedPower] Porta criada com sucesso: '{}'", name_str);
                    event_port = p;
                    break;
                }
                Err(e) => {
                    // crate::println!("[RedPower] Falha ao criar porta '{}': {:?}", name_str, e);
                    seed += 1;
                    // Tenta até 100 portas diferentes
                    if seed > 100 {
                        return Err(crate::syscall::SysError::AlreadyExists);
                    }
                }
            }
        }

        // 2. Conectar ao compositor
        let status_port = Port::connect(COMPOSITOR_PORT)?;

        // 3. Enviar request com nome da porta de resposta
        let req = CreateWindowRequest {
            op: opcodes::CREATE_WINDOW,
            x,
            y,
            width,
            height,
            flags,
            reply_port: port_name_buf,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<CreateWindowRequest>(),
            )
        };

        crate::println!(
            "[RedPower] Enviando CREATE_WINDOW ({}x{}, flags={:#x}) ao compositor...",
            width,
            height,
            flags
        );
        status_port.send(req_bytes, 0)?;

        // 4. Receber response na NOSSA porta de eventos
        let mut resp_msg = ProtocolMessage { raw: [0; 256] };
        let resp_bytes = unsafe {
            core::slice::from_raw_parts_mut(
                &mut resp_msg as *mut _ as *mut u8,
                core::mem::size_of::<ProtocolMessage>(),
            )
        };

        // Bloqueante (timeout alto para garantir init)
        match event_port.recv(resp_bytes, 10000) {
            Ok(len) if len < core::mem::size_of::<WindowCreatedResponse>() => {
                crate::println!(
                    "[RedPower] Erro: Resposta muito curta ou timeout (len={})",
                    len
                );
                return Err(crate::syscall::SysError::ProtocolError);
            }
            Err(e) => {
                crate::println!("[RedPower] Erro ao receber resposta: {:?}", e);
                return Err(e);
            }
            Ok(_) => {} // Sucesso
        }

        let resp = unsafe { resp_msg.win_resp };

        if resp.op != opcodes::WINDOW_CREATED {
            crate::println!(
                "[RedPower] Erro: Opcode inválido na resposta (op={})",
                resp.op
            );
            return Err(crate::syscall::SysError::ProtocolError);
        }

        // 5. Mapear SHM
        // crate::println!("[RedPower] Mapeando SHM handle {}", resp.shm_handle);
        let shm = match SharedMemory::open(ShmId(resp.shm_handle)) {
            Ok(s) => s,
            Err(e) => {
                crate::println!("[RedPower] Erro ao mapear SHM: {:?}", e);
                return Err(e);
            }
        };

        // Sucesso!
        Ok(Self {
            id: resp.window_id,
            shm,
            width,
            height,
            compositor_port: status_port,
            event_port, // Mantém a porta aberta para receber eventos futuros (Input, Resize)
        })
    }

    /// Cria nova janela padrão
    pub fn create(x: u32, y: u32, width: u32, height: u32) -> SysResult<Self> {
        Self::create_with_flags(x, y, width, height, 0)
    }

    /// Obtém ponteiro para buffer de pixels
    pub fn buffer(&mut self) -> &mut [u32] {
        let ptr = self.shm.as_mut_ptr() as *mut u32;
        let len = (self.width * self.height) as usize;
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    /// Desenha um pixel (usando write_volatile para garantir commit na RAM)
    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            unsafe {
                // self.buffer()[idx] = color;
                core::ptr::write_volatile(&mut self.buffer()[idx], color);
            }
        }
    }

    /// Preenche retângulo
    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Notifica compositor que buffer foi atualizado
    pub fn present(&self) -> SysResult<()> {
        // crate::println!("[RedPower] Presenting window {}", self.id); // Debug flood
        let req = CommitBufferRequest {
            op: opcodes::COMMIT_BUFFER,
            window_id: self.id,
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<CommitBufferRequest>(),
            )
        };

        // crate::println!(
        //     "[RedPower] Enviando COMMIT_BUFFER (janela {}) ao compositor...",
        //     self.id
        // );
        self.compositor_port.send(req_bytes, 0)?;
        // crate::println!("[RedPower] COMMIT_BUFFER enviado!");
        // TODO: Debug
        Ok(())
    }

    /// Lê eventos da fila (não bloqueante)
    pub fn poll_events(&self) -> impl Iterator<Item = crate::event::Event> + '_ {
        core::iter::from_fn(move || {
            let mut msg = crate::window::ProtocolMessage { raw: [0; 256] };
            let msg_bytes = unsafe {
                core::slice::from_raw_parts_mut(
                    &mut msg as *mut _ as *mut u8,
                    core::mem::size_of::<crate::window::ProtocolMessage>(),
                )
            };

            // Fix do loop infinito: Verifica se realmente lemos algo (len > 0)
            // O IPC (SysVMsg) pode retornar Ok(0) se não tiver mensagem?
            // Na implementação atual do kernel, se não tiver mensagem e timeout=0, ele retorna Err(WouldBlock).
            // Mas vamos garantir.
            match self.event_port.recv(msg_bytes, 0) {
                Ok(len) if len > 0 => {
                    // Decodificar mensagem
                    unsafe {
                        match msg.header {
                            opcodes::EVENT_INPUT => Some(crate::event::Event::Input(msg.input_evt)),
                            opcodes::EVENT_RESIZE => {
                                Some(crate::event::Event::Resize(msg.resize_evt))
                            }
                            _ => {
                                // Se recebermos lixo ou op desconhecido, ignoramos e tentamos próxima?
                                // Por enquanto retornamos Unknown para debug
                                Some(crate::event::Event::Unknown)
                            }
                        }
                    }
                }
                // Se len == 0, ou Err (WouldBlock/Empty), paramos o iterador
                _ => None,
            }
        })
    }

    /// Destrói a janela e libera os recursos no compositor.
    pub fn destroy(&self) -> SysResult<()> {
        let req = DestroyWindowRequest {
            op: opcodes::DESTROY_WINDOW,
            window_id: self.id,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<DestroyWindowRequest>(),
            )
        };

        self.compositor_port.send(req_bytes, 0)?;
        Ok(())
    }
}
