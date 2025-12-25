//! Códigos de Erro do Redstone OS

/// Enum de erros do sistema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SysError {
    // Erros Gerais (1-15)
    PermissionDenied = 1,
    NotFound = 2,
    AlreadyExists = 3,
    InvalidArgument = 4,
    WouldBlock = 5,
    Interrupted = 6,
    TimedOut = 7,
    Busy = 8,

    // Erros de Handle (16-31)
    BadHandle = 16,
    HandleTypeMismatch = 17,
    InsufficientRights = 18,
    HandleTableFull = 19,

    // Erros de Memória (32-47)
    OutOfMemory = 32,
    BadAddress = 33,
    AddressInUse = 34,
    BadAlignment = 35,

    // Erros de IO (48-63)
    IoError = 48,
    EndOfFile = 49,
    BrokenPipe = 50,

    // Erros de IPC (64-79)
    PortFull = 64,
    PortClosed = 65,
    MessageTooLarge = 66,
    NoMessage = 67,

    // Erros de Processo (80-95)
    ProcessNotFound = 80,
    TooManyProcesses = 81,

    // Erros de Sistema (240-255)
    NotImplemented = 254,
    Unknown = 255,
}

impl SysError {
    /// Cria erro a partir de código negativo
    pub fn from_code(code: isize) -> Self {
        if code >= 0 {
            return Self::Unknown;
        }
        let abs = (-code) as i32;
        match abs {
            1 => Self::PermissionDenied,
            2 => Self::NotFound,
            3 => Self::AlreadyExists,
            4 => Self::InvalidArgument,
            5 => Self::WouldBlock,
            6 => Self::Interrupted,
            7 => Self::TimedOut,
            8 => Self::Busy,
            16 => Self::BadHandle,
            17 => Self::HandleTypeMismatch,
            18 => Self::InsufficientRights,
            19 => Self::HandleTableFull,
            32 => Self::OutOfMemory,
            33 => Self::BadAddress,
            34 => Self::AddressInUse,
            35 => Self::BadAlignment,
            48 => Self::IoError,
            49 => Self::EndOfFile,
            50 => Self::BrokenPipe,
            64 => Self::PortFull,
            65 => Self::PortClosed,
            66 => Self::MessageTooLarge,
            67 => Self::NoMessage,
            80 => Self::ProcessNotFound,
            81 => Self::TooManyProcesses,
            254 => Self::NotImplemented,
            _ => Self::Unknown,
        }
    }

    /// Código numérico do erro
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

/// Resultado de syscall
pub type SysResult<T> = Result<T, SysError>;
