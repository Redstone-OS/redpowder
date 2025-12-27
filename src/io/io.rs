//! # IO Primitives
//!
//! Handle e Rights para operações de I/O.

/// Handle para recurso do kernel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Handle(u32);

impl Handle {
    pub const INVALID: Self = Self(u32::MAX);

    /// Cria handle de valor raw
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Obtém valor raw
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Verifica se é válido
    pub fn is_valid(&self) -> bool {
        *self != Self::INVALID
    }
}

/// Direitos de um handle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandleRights(u64);

impl HandleRights {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const EXEC: Self = Self(1 << 2);
    pub const DUP: Self = Self(1 << 8);
    pub const CLOSE: Self = Self(1 << 10);
    pub const SEEK: Self = Self(1 << 32);
    pub const STAT: Self = Self(1 << 33);

    /// Combina rights
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Verifica se contém
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Valor raw
    pub fn bits(self) -> u64 {
        self.0
    }
}

/// Vetor de IO
#[repr(C)]
pub struct IoVec {
    pub base: *const u8,
    pub len: usize,
}

impl IoVec {
    pub fn new(data: &[u8]) -> Self {
        Self {
            base: data.as_ptr(),
            len: data.len(),
        }
    }
}
