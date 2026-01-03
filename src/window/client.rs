//! # Window Client
//!
//! Cliente de janela para comunicação com o compositor Firefly.

use crate::ipc::{Port, SharedMemory, ShmId};
use crate::syscall::{SysError, SysResult};

use gfx_types::color::Color;
use gfx_types::geometry::{Point, Rect, Size};
use gfx_types::window::WindowFlags;

use super::protocol::*;

// =============================================================================
// WINDOW
// =============================================================================

/// Handle para janela no compositor Firefly.
pub struct Window {
    /// ID da janela no compositor.
    pub id: u32,
    /// Memória compartilhada com o buffer de pixels.
    pub shm: SharedMemory,
    /// Largura em pixels.
    width: u32,
    /// Altura em pixels.
    height: u32,
    /// Porta de comunicação com o compositor.
    compositor_port: Port,
    /// Porta de eventos (recebe input, resize, etc).
    event_port: Port,
}

impl Window {
    // =========================================================================
    // CRIAÇÃO
    // =========================================================================

    /// Cria nova janela com flags específicas.
    pub fn create_with_flags(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        flags: WindowFlags,
        title: &str,
    ) -> SysResult<Self> {
        Self::create_internal(x, y, width, height, flags.bits(), title)
    }

    /// Cria nova janela padrão.
    pub fn create(x: u32, y: u32, width: u32, height: u32, title: &str) -> SysResult<Self> {
        Self::create_internal(x, y, width, height, 0, title)
    }

    fn create_internal(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        flags: u32,
        title: &str,
    ) -> SysResult<Self> {
        // 1. Criar porta de resposta única
        let event_port;
        let mut port_name_buf = [0u8; 32];
        let mut seed = 0;

        loop {
            // "win.r.<seed>"
            let prefix = b"win.r.";
            let mut i = 0;
            while i < prefix.len() {
                port_name_buf[i] = prefix[i];
                i += 1;
            }

            // Simple itoa
            let mut n = seed;
            if n == 0 {
                port_name_buf[i] = b'0';
                i += 1;
            } else {
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

            for k in i..32 {
                port_name_buf[k] = 0;
            }

            let name_str = core::str::from_utf8(&port_name_buf[0..i]).unwrap_or("");

            match Port::create(name_str, 16) {
                Ok(p) => {
                    event_port = p;
                    break;
                }
                Err(_) => {
                    seed += 1;
                    if seed > 100 {
                        return Err(SysError::AlreadyExists);
                    }
                }
            }
        }

        // 2. Conectar ao compositor
        let status_port = Port::connect(COMPOSITOR_PORT)?;

        // 3. Enviar request
        let mut title_buf = [0u8; 64];
        let bytes = title.as_bytes();
        let len = bytes.len().min(64);
        for i in 0..len {
            title_buf[i] = bytes[i];
        }

        let req = CreateWindowRequest {
            op: opcodes::CREATE_WINDOW,
            x,
            y,
            width,
            height,
            flags,
            reply_port: port_name_buf,
            title: title_buf,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<CreateWindowRequest>(),
            )
        };

        crate::println!(
            "[RedPower] Enviando CREATE_WINDOW ({}x{}, flags={:#x})...",
            width,
            height,
            flags
        );
        status_port.send(req_bytes, 0)?;

        // 4. Receber response
        let mut resp_msg = ProtocolMessage {
            raw: [0; MAX_MSG_SIZE],
        };
        let resp_bytes = unsafe {
            core::slice::from_raw_parts_mut(
                &mut resp_msg as *mut _ as *mut u8,
                core::mem::size_of::<ProtocolMessage>(),
            )
        };

        match event_port.recv(resp_bytes, 10000) {
            Ok(len) if len < core::mem::size_of::<WindowCreatedResponse>() => {
                crate::println!("[RedPower] Erro: Resposta muito curta (len={})", len);
                return Err(SysError::ProtocolError);
            }
            Err(e) => {
                crate::println!("[RedPower] Erro ao receber resposta: {:?}", e);
                return Err(e);
            }
            Ok(_) => {}
        }

        let resp = unsafe { resp_msg.win_resp };

        if resp.op != opcodes::WINDOW_CREATED {
            crate::println!(
                "[RedPower] Erro: Opcode inválido na resposta (op={})",
                resp.op
            );
            return Err(SysError::ProtocolError);
        }

        // 5. Mapear SHM
        let shm = SharedMemory::open(ShmId(resp.shm_handle))?;

        Ok(Self {
            id: resp.window_id,
            shm,
            width,
            height,
            compositor_port: status_port,
            event_port,
        })
    }

    // =========================================================================
    // PROPRIEDADES
    // =========================================================================

    /// Largura em pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Altura em pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Retorna Size.
    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Retorna o retângulo da janela (origem em 0,0).
    #[inline]
    pub fn bounds(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    // =========================================================================
    // BUFFER
    // =========================================================================

    /// Obtém ponteiro para buffer de pixels.
    pub fn buffer(&mut self) -> &mut [u32] {
        let ptr = self.shm.as_mut_ptr() as *mut u32;
        let len = (self.width * self.height) as usize;
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    /// Limpa o buffer com uma cor.
    pub fn clear(&mut self, color: Color) {
        let color_u32 = color.as_u32();
        self.buffer().fill(color_u32);
    }

    /// Desenha um pixel.
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            unsafe {
                core::ptr::write_volatile(&mut self.buffer()[idx], color.as_u32());
            }
        }
    }

    /// Desenha um pixel em Point.
    pub fn put_pixel_at(&mut self, p: Point, color: Color) {
        if p.x >= 0 && p.y >= 0 {
            self.put_pixel(p.x as u32, p.y as u32, color);
        }
    }

    /// Preenche retângulo.
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let bounds = self.bounds();
        let clipped = match rect.intersection(&bounds) {
            Some(r) => r,
            None => return,
        };

        let color_u32 = color.as_u32();
        let width = self.width; // Salvar antes de emprestar
        let buffer = self.buffer();

        for y in clipped.y as u32..(clipped.y as u32 + clipped.height) {
            let start = (y * width + clipped.x as u32) as usize;
            let end = start + clipped.width as usize;
            for i in start..end {
                if i < buffer.len() {
                    unsafe {
                        core::ptr::write_volatile(&mut buffer[i], color_u32);
                    }
                }
            }
        }
    }

    // =========================================================================
    // APRESENTAÇÃO
    // =========================================================================

    /// Notifica compositor que buffer foi atualizado.
    pub fn present(&self) -> SysResult<()> {
        self.present_region(self.bounds())
    }

    /// Notifica compositor que uma região foi atualizada.
    pub fn present_region(&self, dirty: Rect) -> SysResult<()> {
        let req = CommitBufferRequest {
            op: opcodes::COMMIT_BUFFER,
            window_id: self.id,
            x: dirty.x as u32,
            y: dirty.y as u32,
            width: dirty.width,
            height: dirty.height,
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

    // =========================================================================
    // EVENTOS
    // =========================================================================

    /// Lê eventos da fila (não bloqueante).
    pub fn poll_events(&self) -> impl Iterator<Item = crate::event::Event> + '_ {
        core::iter::from_fn(move || {
            let mut msg = ProtocolMessage {
                raw: [0; MAX_MSG_SIZE],
            };
            let msg_bytes = unsafe {
                core::slice::from_raw_parts_mut(
                    &mut msg as *mut _ as *mut u8,
                    core::mem::size_of::<ProtocolMessage>(),
                )
            };

            match self.event_port.recv(msg_bytes, 0) {
                Ok(len) if len > 0 => unsafe {
                    match msg.header {
                        opcodes::EVENT_INPUT => Some(crate::event::Event::Input(msg.input_evt)),
                        opcodes::EVENT_RESIZE => Some(crate::event::Event::Resize(msg.resize_evt)),
                        _ => Some(crate::event::Event::Unknown),
                    }
                },
                _ => None,
            }
        })
    }

    // =========================================================================
    // OPERAÇÕES DE JANELA
    // =========================================================================

    /// Destrói a janela.
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

    /// Minimiza a janela.
    pub fn minimize(&self) -> SysResult<()> {
        self.send_op_request(opcodes::MINIMIZE_WINDOW)
    }

    /// Restaura a janela.
    pub fn restore(&self) -> SysResult<()> {
        self.send_op_request(opcodes::RESTORE_WINDOW)
    }

    fn send_op_request(&self, op: u32) -> SysResult<()> {
        let req = WindowOpRequest {
            op,
            window_id: self.id,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<WindowOpRequest>(),
            )
        };

        self.compositor_port.send(req_bytes, 0)?;
        Ok(())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let _ = self.destroy();
    }
}
