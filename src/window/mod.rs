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
// PROTOCOLO FIREFLY (Versão 2.0)
// ============================================================================

/// Identificadores de Mensagem (OpCodes)
pub mod opcodes {
    // Client -> Server
    pub const CREATE_WINDOW: u32 = 0x01;
    pub const DESTROY_WINDOW: u32 = 0x02;
    pub const COMMIT_BUFFER: u32 = 0x03;

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
    pub op: u32, // CREATE_WINDOW
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32,
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

// ... (opcodes stay here)

/// União de todas as mensagens possíveis (para leitura genérica)
#[repr(C)]
pub union ProtocolMessage {
    pub header: u32, // Apenas para ler o OpCode
    pub create_req: CreateWindowRequest,
    pub buf_req: CommitBufferRequest,
    pub win_resp: WindowCreatedResponse,
    pub input_evt: InputEvent,
    pub resize_evt: ResizeEvent,
    pub raw: [u8; 64], // Padding para tamanho fixo máximo
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
}

impl Window {
    /// Cria nova janela
    pub fn create(x: u32, y: u32, width: u32, height: u32) -> SysResult<Self> {
        // Conectar ao compositor
        let port = Port::connect(COMPOSITOR_PORT)?;

        // Enviar request via ProtocolMessage
        let req = CreateWindowRequest {
            op: opcodes::CREATE_WINDOW,
            x,
            y,
            width,
            height,
            flags: 0,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<CreateWindowRequest>(),
            )
        };

        port.send(req_bytes, 0)?;

        // Receber response
        let mut resp = WindowCreatedResponse {
            op: 0,
            window_id: 0,
            shm_handle: 0,
            buffer_size: 0,
        };

        let resp_bytes = unsafe {
            core::slice::from_raw_parts_mut(
                &mut resp as *mut _ as *mut u8,
                core::mem::size_of::<WindowCreatedResponse>(),
            )
        };

        port.recv(resp_bytes, 0)?;

        if resp.op != opcodes::WINDOW_CREATED {
            return Err(crate::syscall::SysError::ProtocolError);
        }

        // Mapear memória compartilhada retornada pelo servidor
        let shm = SharedMemory::open(ShmId(resp.shm_handle))?;

        Ok(Self {
            id: resp.window_id,
            shm,
            width,
            height,
            compositor_port: port,
        })
    }

    /// Obtém ponteiro para buffer de pixels
    pub fn buffer(&mut self) -> &mut [u32] {
        let ptr = self.shm.as_mut_ptr() as *mut u32;
        let len = (self.width * self.height) as usize;
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    /// Desenha um pixel
    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.buffer()[idx] = color;
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

        self.compositor_port.send(req_bytes, 0)?;
        Ok(())
    }

    /// Lê eventos da fila (não bloqueante)
    pub fn poll_events(&self) -> impl Iterator<Item = crate::event::Event> + '_ {
        core::iter::from_fn(move || {
            let mut msg = crate::window::ProtocolMessage { raw: [0; 64] };
            let msg_bytes = unsafe {
                core::slice::from_raw_parts_mut(
                    &mut msg as *mut _ as *mut u8,
                    core::mem::size_of::<crate::window::ProtocolMessage>(),
                )
            };

            // Tenta ler sem bloquear
            match self.compositor_port.recv(msg_bytes, 0) {
                Ok(_) => {
                    // Decodificar mensagem
                    unsafe {
                        match msg.header {
                            opcodes::EVENT_INPUT => Some(crate::event::Event::Input(msg.input_evt)),
                            opcodes::EVENT_RESIZE => {
                                Some(crate::event::Event::Resize(msg.resize_evt))
                            }
                            _ => Some(crate::event::Event::Unknown), // Ignora mensagens desconhecidas por enquanto
                        }
                    }
                }
                Err(_) => None, // Sem mais mensagens ou erro
            }
        })
    }
}
