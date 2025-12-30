//! # Firefly Window Protocol
//!
//! Protocolo de comunicação entre clientes (apps) e compositor.

use crate::ipc::{Port, SharedMemory};
use crate::syscall::SysResult;

// ============================================================================
// CONSTANTES
// ============================================================================

/// Nome da porta do compositor
pub const COMPOSITOR_PORT: &str = "firefly.compositor";

/// Tamanho máximo de mensagem
pub const MAX_MSG_SIZE: usize = 256;

// ============================================================================
// IDs DE MENSAGEM
// ============================================================================

/// Mensagens do cliente para o compositor
pub mod client_msg {
    /// Pede criação de janela
    /// Payload: CreateWindowRequest
    pub const CREATE_WINDOW: u32 = 1;

    /// Notifica que buffer foi atualizado
    /// Payload: BufferReadyRequest
    pub const BUFFER_READY: u32 = 2;

    /// Fecha janela
    /// Payload: window_id (u32)
    pub const CLOSE_WINDOW: u32 = 3;
}

/// Mensagens do compositor para o cliente
pub mod compositor_msg {
    /// Janela criada
    /// Payload: WindowCreatedResponse
    pub const WINDOW_CREATED: u32 = 100;

    /// Erro
    /// Payload: error_code (u32)
    pub const ERROR: u32 = 101;
}

// ============================================================================
// ESTRUTURAS DE MENSAGEM
// ============================================================================

/// Request para criar janela
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CreateWindowRequest {
    pub msg_id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32,
}

/// Response de janela criada
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WindowCreatedResponse {
    pub msg_id: u32,
    pub window_id: u32,
    pub shm_id: u64,
    pub buffer_offset: u64,
}

/// Request de buffer pronto
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferReadyRequest {
    pub msg_id: u32,
    pub window_id: u32,
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

        // Criar buffer local para a janela
        let buffer_size = (width * height * 4) as usize; // ARGB
        let shm = SharedMemory::create(buffer_size)?;

        // Enviar request
        let req = CreateWindowRequest {
            msg_id: client_msg::CREATE_WINDOW,
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
            msg_id: 0,
            window_id: 0,
            shm_id: 0,
            buffer_offset: 0,
        };

        let resp_bytes = unsafe {
            core::slice::from_raw_parts_mut(
                &mut resp as *mut _ as *mut u8,
                core::mem::size_of::<WindowCreatedResponse>(),
            )
        };

        port.recv(resp_bytes, 0)?;

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
        let req = BufferReadyRequest {
            msg_id: client_msg::BUFFER_READY,
            window_id: self.id,
        };

        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &req as *const _ as *const u8,
                core::mem::size_of::<BufferReadyRequest>(),
            )
        };

        self.compositor_port.send(req_bytes, 0)?;
        Ok(())
    }
}
