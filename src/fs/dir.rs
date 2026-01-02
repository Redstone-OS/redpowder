//! # Directory Abstraction
//!
//! Abstração de alto nível para diretórios.
//!
//! ## Exemplo
//!
//! ```rust
//! use redpowder::fs::{Dir, DirEntry};
//!
//! // Listar diretório
//! let dir = Dir::open("/apps")?;
//! for entry in dir.entries() {
//!     println!("{} - {:?}", entry.name(), entry.file_type());
//! }
//!
//! // Usando iterator
//! let files: Vec<_> = Dir::open("/apps")?
//!     .entries()
//!     .filter(|e| e.is_file())
//!     .collect();
//! ```

use super::types::{DirEntry, OpenFlags, O_DIRECTORY, O_RDONLY};
use crate::io::Handle;
use crate::syscall::{
    check_error, syscall1, syscall3, syscall4, SysResult, SYS_GETDENTS, SYS_HANDLE_CLOSE, SYS_OPEN,
};

/// Diretório aberto
///
/// Representa um handle para um diretório aberto.
/// Permite iterar sobre as entradas do diretório.
pub struct Dir {
    handle: Handle,
    /// Path do diretório (para debug)
    #[allow(dead_code)]
    path: [u8; 256],
    path_len: usize,
}

impl Dir {
    /// Abre um diretório
    ///
    /// # Argumentos
    /// - `path` - Caminho do diretório
    ///
    /// # Exemplo
    /// ```rust
    /// let dir = Dir::open("/apps")?;
    /// ```
    pub fn open(path: &str) -> SysResult<Self> {
        let flags = O_RDONLY | O_DIRECTORY;
        let ret = syscall4(
            SYS_OPEN,
            path.as_ptr() as usize,
            path.len(),
            flags as usize,
            0,
        );
        let handle = Handle::from_raw(check_error(ret)? as u32);

        let mut dir = Self {
            handle,
            path: [0u8; 256],
            path_len: path.len().min(255),
        };
        dir.path[..dir.path_len].copy_from_slice(&path.as_bytes()[..dir.path_len]);

        Ok(dir)
    }

    /// Obtém handle interno
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Obtém valor raw do handle
    pub fn raw_handle(&self) -> u32 {
        self.handle.raw()
    }

    /// Lê entradas do diretório para um buffer
    ///
    /// Retorna o número de bytes escritos no buffer (0 se não há mais entradas).
    /// O buffer contém structs DirEntry serializadas.
    pub fn read_raw(&self, buf: &mut [u8]) -> SysResult<usize> {
        let ret = syscall3(
            SYS_GETDENTS,
            self.handle.raw() as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
        );
        check_error(ret)
    }

    /// Cria um iterador sobre as entradas do diretório
    ///
    /// # Exemplo
    /// ```rust
    /// for entry in dir.entries() {
    ///     println!("{}", entry.name());
    /// }
    /// ```
    pub fn entries(self) -> ReadDir {
        ReadDir::new(self)
    }

    /// Lê todas as entradas de uma vez
    ///
    /// **Nota:** Requer allocator. Para ambientes no_std sem alloc,
    /// use `entries()` iterator ou `read_raw()`.
    #[cfg(feature = "alloc")]
    pub fn read_all(&self) -> SysResult<alloc::vec::Vec<DirEntry>> {
        let mut entries = alloc::vec::Vec::new();
        let mut buf = [0u8; 4096];

        loop {
            let bytes = self.read_raw(&mut buf)?;
            if bytes == 0 {
                break;
            }

            let mut offset = 0;
            while offset < bytes {
                if let Some((entry, rec_len)) = DirEntry::parse_from_buffer(&buf[offset..]) {
                    entries.push(entry);
                    offset += rec_len;
                } else {
                    break;
                }
            }
        }

        Ok(entries)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let _ = syscall1(SYS_HANDLE_CLOSE, self.handle.raw() as usize);
    }
}

// =============================================================================
// ITERATOR
// =============================================================================

/// Iterator sobre entradas de diretório
///
/// Criado por `Dir::entries()`.
pub struct ReadDir {
    dir: Dir,
    /// Buffer interno para getdents
    buffer: [u8; 1024],
    /// Bytes válidos no buffer
    buffer_len: usize,
    /// Offset atual no buffer
    buffer_offset: usize,
    /// Fim do diretório alcançado
    finished: bool,
}

impl ReadDir {
    fn new(dir: Dir) -> Self {
        Self {
            dir,
            buffer: [0u8; 1024],
            buffer_len: 0,
            buffer_offset: 0,
            finished: false,
        }
    }

    /// Recarrega o buffer com mais entradas
    fn refill_buffer(&mut self) -> bool {
        if self.finished {
            return false;
        }

        match self.dir.read_raw(&mut self.buffer) {
            Ok(0) => {
                self.finished = true;
                false
            }
            Ok(bytes) => {
                self.buffer_len = bytes;
                self.buffer_offset = 0;
                true
            }
            Err(_) => {
                self.finished = true;
                false
            }
        }
    }
}

impl Iterator for ReadDir {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Tentar ler do buffer atual
            if self.buffer_offset < self.buffer_len {
                let remaining = &self.buffer[self.buffer_offset..self.buffer_len];
                if let Some((entry, rec_len)) = DirEntry::parse_from_buffer(remaining) {
                    self.buffer_offset += rec_len;
                    return Some(entry);
                }
            }

            // Buffer esgotado, recarregar
            if !self.refill_buffer() {
                return None;
            }
        }
    }
}

// =============================================================================
// FUNÇÕES DE CONVENIÊNCIA
// =============================================================================

/// Lista entradas de um diretório
///
/// # Exemplo
/// ```rust
/// for entry in list_dir("/apps")? {
///     println!("{}", entry.name());
/// }
/// ```
pub fn list_dir(path: &str) -> SysResult<ReadDir> {
    Ok(Dir::open(path)?.entries())
}
