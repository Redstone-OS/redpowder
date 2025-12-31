//! # Syscall Errors
//!
//! Códigos de erro retornados por syscalls.

/// Resultado de syscall
pub type SysResult<T> = Result<T, SysError>;

/// Erro de syscall
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SysError {
    NotImplemented = -1,
    InvalidSyscall = -2,
    InvalidArgument = -3,
    InvalidHandle = -4,
    PermissionDenied = -5,
    NotFound = -6,
    AlreadyExists = -7,
    Busy = -8,
    Timeout = -9,
    OutOfMemory = -10,
    BufferTooSmall = -11,
    Interrupted = -12,
    EndOfFile = -13,
    BrokenPipe = -14,
    IsDirectory = -15,
    NotDirectory = -16,
    NotEmpty = -17,
    IoError = -18,
    LimitReached = -19,
    NotSupported = -20,
    BadAddress = -21,
    ProtocolError = -22,
    Unknown = -127,
}

impl SysError {
    /// Converte código de retorno em erro
    pub fn from_code(code: isize) -> Self {
        match code as i32 {
            -1 => Self::NotImplemented,
            -2 => Self::InvalidSyscall,
            -3 => Self::InvalidArgument,
            -4 => Self::InvalidHandle,
            -5 => Self::PermissionDenied,
            -6 => Self::NotFound,
            -7 => Self::AlreadyExists,
            -8 => Self::Busy,
            -9 => Self::Timeout,
            -10 => Self::OutOfMemory,
            -11 => Self::BufferTooSmall,
            -12 => Self::Interrupted,
            -13 => Self::EndOfFile,
            -14 => Self::BrokenPipe,
            -15 => Self::IsDirectory,
            -16 => Self::NotDirectory,
            -17 => Self::NotEmpty,
            -18 => Self::IoError,
            -19 => Self::LimitReached,
            -20 => Self::NotSupported,
            -21 => Self::BadAddress,
            -22 => Self::ProtocolError,
            _ => Self::Unknown,
        }
    }

    /// Código numérico do erro
    pub fn code(self) -> i32 {
        self as i32
    }
}

/// Converte retorno de syscall em Result
#[inline]
pub fn check_error(ret: isize) -> SysResult<usize> {
    if ret < 0 {
        Err(SysError::from_code(ret))
    } else {
        Ok(ret as usize)
    }
}
