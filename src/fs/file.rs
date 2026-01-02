//! # File Abstraction
//!
//! Abstração de alto nível para arquivos.
//!
//! ## Exemplo
//!
//! ```rust
//! use redpowder::fs::{File, OpenFlags, O_RDONLY};
//!
//! // Abrir para leitura
//! let file = File::open("/apps/config.txt")?;
//!
//! // Ler todo o conteúdo
//! let mut buf = [0u8; 1024];
//! let bytes = file.read(&mut buf)?;
//!
//! // Ler com buffer
//! let content = file.read_to_vec()?;
//! ```

use super::types::{
    FileStat, OpenFlags, SeekFrom, O_CREATE, O_DIRECTORY, O_RDONLY, O_TRUNC, O_WRONLY,
};
use crate::io::Handle;
use crate::syscall::{
    check_error, syscall1, syscall2, syscall3, syscall4, SysResult, SYS_FLUSH, SYS_FSTAT,
    SYS_HANDLE_CLOSE, SYS_OPEN, SYS_PREAD, SYS_PWRITE, SYS_READ, SYS_SEEK, SYS_TRUNCATE, SYS_WRITE,
};

/// Arquivo aberto
///
/// Representa um handle para um arquivo aberto no kernel.
/// O arquivo é automaticamente fechado quando o `File` é dropado.
pub struct File {
    handle: Handle,
    /// Flags de abertura (para verificar permissões)
    flags: OpenFlags,
}

impl File {
    // =========================================================================
    // CONSTRUTORES
    // =========================================================================

    /// Abre um arquivo com flags específicas
    ///
    /// # Argumentos
    /// - `path` - Caminho do arquivo
    /// - `flags` - Flags de abertura (O_RDONLY, O_WRONLY, etc)
    ///
    /// # Exemplo
    /// ```rust
    /// let file = File::open_with_flags("/apps/data.txt", OpenFlags::new(O_RDWR))?;
    /// ```
    pub fn open_with_flags(path: &str, flags: OpenFlags) -> SysResult<Self> {
        // Verificar que não está tentando abrir diretório como arquivo
        if (flags.0 & O_DIRECTORY) != 0 {
            return Err(crate::syscall::SysError::InvalidArgument);
        }

        let ret = syscall4(
            SYS_OPEN,
            path.as_ptr() as usize,
            path.len(),
            flags.0 as usize,
            0, // mode
        );
        let handle = Handle::from_raw(check_error(ret)? as u32);
        Ok(Self { handle, flags })
    }

    /// Abre um arquivo para leitura
    ///
    /// Equivalente a `open_with_flags(path, OpenFlags::new(O_RDONLY))`
    pub fn open(path: &str) -> SysResult<Self> {
        Self::open_with_flags(path, OpenFlags::new(O_RDONLY))
    }

    /// Cria um novo arquivo (ou trunca se existir)
    ///
    /// Equivalente a `open_with_flags(path, O_WRONLY | O_CREATE | O_TRUNC)`
    pub fn create(path: &str) -> SysResult<Self> {
        Self::open_with_flags(path, OpenFlags::new(O_WRONLY | O_CREATE | O_TRUNC))
    }

    /// Cria handle de um raw handle existente
    ///
    /// # Safety
    /// O chamador deve garantir que o handle é válido e representa um arquivo.
    pub unsafe fn from_raw_handle(handle: u32, flags: OpenFlags) -> Self {
        Self {
            handle: Handle::from_raw(handle),
            flags,
        }
    }

    // =========================================================================
    // LEITURA
    // =========================================================================

    /// Lê dados do arquivo para o buffer
    ///
    /// # Retorno
    /// Número de bytes lidos, ou 0 para EOF.
    pub fn read(&self, buf: &mut [u8]) -> SysResult<usize> {
        let ret = syscall3(
            SYS_READ,
            self.handle.raw() as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
        );
        check_error(ret)
    }

    /// Lê dados em um offset específico (sem mover cursor)
    ///
    /// Útil para leitura paralela ou random access.
    pub fn pread(&self, buf: &mut [u8], offset: u64) -> SysResult<usize> {
        let ret = syscall4(
            SYS_PREAD,
            self.handle.raw() as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            offset as usize,
        );
        check_error(ret)
    }

    /// Lê exatamente `buf.len()` bytes
    ///
    /// Retorna erro se não conseguir ler todos os bytes.
    pub fn read_exact(&self, buf: &mut [u8]) -> SysResult<()> {
        let mut total = 0;
        while total < buf.len() {
            let bytes = self.read(&mut buf[total..])?;
            if bytes == 0 {
                return Err(crate::syscall::SysError::EndOfFile);
            }
            total += bytes;
        }
        Ok(())
    }

    // =========================================================================
    // ESCRITA
    // =========================================================================

    /// Escreve dados no arquivo
    ///
    /// # Retorno
    /// Número de bytes escritos.
    pub fn write(&self, buf: &[u8]) -> SysResult<usize> {
        let ret = syscall3(
            SYS_WRITE,
            self.handle.raw() as usize,
            buf.as_ptr() as usize,
            buf.len(),
        );
        check_error(ret)
    }

    /// Escreve dados em um offset específico (sem mover cursor)
    pub fn pwrite(&self, buf: &[u8], offset: u64) -> SysResult<usize> {
        let ret = syscall4(
            SYS_PWRITE,
            self.handle.raw() as usize,
            buf.as_ptr() as usize,
            buf.len(),
            offset as usize,
        );
        check_error(ret)
    }

    /// Escreve todos os bytes do buffer
    ///
    /// Retorna erro se não conseguir escrever todos os bytes.
    pub fn write_all(&self, buf: &[u8]) -> SysResult<()> {
        let mut total = 0;
        while total < buf.len() {
            let bytes = self.write(&buf[total..])?;
            if bytes == 0 {
                return Err(crate::syscall::SysError::IoError);
            }
            total += bytes;
        }
        Ok(())
    }

    // =========================================================================
    // POSICIONAMENTO
    // =========================================================================

    /// Move o cursor de leitura/escrita
    ///
    /// # Argumentos
    /// - `offset` - Deslocamento (pode ser negativo para Current/End)
    /// - `whence` - Origem do deslocamento
    ///
    /// # Retorno
    /// Nova posição absoluta no arquivo.
    pub fn seek(&self, offset: i64, whence: SeekFrom) -> SysResult<u64> {
        let ret = syscall3(
            SYS_SEEK,
            self.handle.raw() as usize,
            offset as usize,
            whence as usize,
        );
        check_error(ret).map(|v| v as u64)
    }

    /// Move para o início do arquivo
    pub fn rewind(&self) -> SysResult<()> {
        self.seek(0, SeekFrom::Start)?;
        Ok(())
    }

    /// Obtém posição atual
    pub fn position(&self) -> SysResult<u64> {
        self.seek(0, SeekFrom::Current)
    }

    // =========================================================================
    // METADADOS
    // =========================================================================

    /// Obtém informações do arquivo (fstat)
    pub fn stat(&self) -> SysResult<FileStat> {
        let mut st = FileStat::zeroed();
        let ret = syscall2(
            SYS_FSTAT,
            self.handle.raw() as usize,
            &mut st as *mut FileStat as usize,
        );
        check_error(ret)?;
        Ok(st)
    }

    /// Obtém tamanho do arquivo
    pub fn size(&self) -> SysResult<u64> {
        Ok(self.stat()?.size)
    }

    // =========================================================================
    // CONTROLE
    // =========================================================================

    /// Força flush de buffers para disco
    pub fn flush(&self) -> SysResult<()> {
        let ret = syscall1(SYS_FLUSH, self.handle.raw() as usize);
        check_error(ret)?;
        Ok(())
    }

    /// Redimensiona o arquivo
    pub fn truncate(&self, size: u64) -> SysResult<()> {
        let ret = syscall2(SYS_TRUNCATE, self.handle.raw() as usize, size as usize);
        check_error(ret)?;
        Ok(())
    }

    // =========================================================================
    // ACESSO INTERNO
    // =========================================================================

    /// Obtém referência ao handle interno
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Obtém valor raw do handle
    pub fn raw_handle(&self) -> u32 {
        self.handle.raw()
    }

    /// Obtém flags de abertura
    pub fn flags(&self) -> OpenFlags {
        self.flags
    }

    /// Verifica se pode ler
    pub fn can_read(&self) -> bool {
        self.flags.can_read()
    }

    /// Verifica se pode escrever
    pub fn can_write(&self) -> bool {
        self.flags.can_write()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // Usa SYS_HANDLE_CLOSE (não SYS_CLOSE que não existe mais)
        let _ = syscall1(SYS_HANDLE_CLOSE, self.handle.raw() as usize);
    }
}

// =============================================================================
// FUNÇÕES DE CONVENIÊNCIA
// =============================================================================

/// Lê todo o conteúdo de um arquivo para um buffer fixo
///
/// # Exemplo
/// ```rust
/// let mut buf = [0u8; 4096];
/// let bytes = read_file("/apps/config.txt", &mut buf)?;
/// let content = &buf[..bytes];
/// ```
pub fn read_file(path: &str, buf: &mut [u8]) -> SysResult<usize> {
    let file = File::open(path)?;
    file.read(buf)
}

/// Escreve dados em um arquivo (cria ou trunca)
pub fn write_file(path: &str, data: &[u8]) -> SysResult<()> {
    let file = File::create(path)?;
    file.write_all(data)
}
