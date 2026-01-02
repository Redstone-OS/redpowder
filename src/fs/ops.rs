//! # Filesystem Operations
//!
//! Operações de filesystem de alto nível.
//!
//! ## Funções Disponíveis
//!
//! | Função | Descrição |
//! |--------|-----------|
//! | `stat` | Obtém informações de arquivo |
//! | `exists` | Verifica se path existe |
//! | `is_file` | Verifica se é arquivo regular |
//! | `is_dir` | Verifica se é diretório |
//! | `getcwd` | Obtém diretório atual |
//! | `chdir` | Muda diretório atual |
//! | `mkdir` | Cria diretório |
//! | `rmdir` | Remove diretório |
//! | `unlink` | Remove arquivo |
//! | `rename` | Renomeia arquivo |

use super::types::FileStat;
use crate::syscall::{
    check_error, syscall2, syscall3, syscall4, SysResult, SYS_ACCESS, SYS_CHDIR, SYS_GETCWD,
    SYS_MKDIR, SYS_RENAME, SYS_RMDIR, SYS_STAT, SYS_UNLINK,
};

// =============================================================================
// METADADOS
// =============================================================================

/// Obtém informações de arquivo por caminho
///
/// # Exemplo
/// ```rust
/// let info = stat("/apps/hello.txt")?;
/// println!("Tamanho: {} bytes", info.size);
/// ```
pub fn stat(path: &str) -> SysResult<FileStat> {
    let mut st = FileStat::zeroed();
    let ret = syscall3(
        SYS_STAT,
        path.as_ptr() as usize,
        path.len(),
        &mut st as *mut FileStat as usize,
    );
    check_error(ret)?;
    Ok(st)
}

/// Verifica se um path existe
///
/// # Exemplo
/// ```rust
/// if exists("/apps/hello.txt") {
///     println!("Arquivo existe!");
/// }
/// ```
pub fn exists(path: &str) -> bool {
    // F_OK = 0
    let ret = syscall3(
        SYS_ACCESS,
        path.as_ptr() as usize,
        path.len(),
        0, // F_OK
    );
    check_error(ret).is_ok()
}

/// Verifica se path é um arquivo regular
pub fn is_file(path: &str) -> bool {
    stat(path).map(|s| s.is_file()).unwrap_or(false)
}

/// Verifica se path é um diretório
pub fn is_dir(path: &str) -> bool {
    stat(path).map(|s| s.is_dir()).unwrap_or(false)
}

/// Verifica permissão de leitura
pub fn can_read(path: &str) -> bool {
    // R_OK = 4
    let ret = syscall3(
        SYS_ACCESS,
        path.as_ptr() as usize,
        path.len(),
        4, // R_OK
    );
    check_error(ret).is_ok()
}

/// Verifica permissão de escrita
pub fn can_write(path: &str) -> bool {
    // W_OK = 2
    let ret = syscall3(
        SYS_ACCESS,
        path.as_ptr() as usize,
        path.len(),
        2, // W_OK
    );
    check_error(ret).is_ok()
}

/// Verifica permissão de execução
pub fn can_execute(path: &str) -> bool {
    // X_OK = 1
    let ret = syscall3(
        SYS_ACCESS,
        path.as_ptr() as usize,
        path.len(),
        1, // X_OK
    );
    check_error(ret).is_ok()
}

// =============================================================================
// DIRETÓRIO DE TRABALHO
// =============================================================================

/// Obtém diretório de trabalho atual
///
/// # Exemplo
/// ```rust
/// let mut buf = [0u8; 256];
/// let cwd = getcwd(&mut buf)?;
/// println!("CWD: {}", cwd);
/// ```
pub fn getcwd(buf: &mut [u8]) -> SysResult<&str> {
    let ret = syscall2(SYS_GETCWD, buf.as_mut_ptr() as usize, buf.len());
    let len = check_error(ret)?;

    // len inclui null terminator
    let path_len = if len > 0 { len - 1 } else { 0 };
    core::str::from_utf8(&buf[..path_len]).map_err(|_| crate::syscall::SysError::InvalidArgument)
}

/// Muda diretório de trabalho atual
///
/// # Exemplo
/// ```rust
/// chdir("/apps")?;
/// ```
pub fn chdir(path: &str) -> SysResult<()> {
    let ret = syscall2(SYS_CHDIR, path.as_ptr() as usize, path.len());
    check_error(ret)?;
    Ok(())
}

// =============================================================================
// CRIAÇÃO/REMOÇÃO
// =============================================================================

/// Cria um diretório
///
/// # Argumentos
/// - `path` - Caminho do diretório
/// - `mode` - Permissões (ex: 0o755)
pub fn mkdir(path: &str, mode: u32) -> SysResult<()> {
    let ret = syscall3(SYS_MKDIR, path.as_ptr() as usize, path.len(), mode as usize);
    check_error(ret)?;
    Ok(())
}

/// Remove um diretório vazio
pub fn rmdir(path: &str) -> SysResult<()> {
    let ret = syscall2(SYS_RMDIR, path.as_ptr() as usize, path.len());
    check_error(ret)?;
    Ok(())
}

/// Remove um arquivo
pub fn unlink(path: &str) -> SysResult<()> {
    let ret = syscall2(SYS_UNLINK, path.as_ptr() as usize, path.len());
    check_error(ret)?;
    Ok(())
}

/// Remove um arquivo (alias para unlink)
pub fn remove(path: &str) -> SysResult<()> {
    unlink(path)
}

/// Renomeia/move um arquivo
pub fn rename(old_path: &str, new_path: &str) -> SysResult<()> {
    let ret = syscall4(
        SYS_RENAME,
        old_path.as_ptr() as usize,
        old_path.len(),
        new_path.as_ptr() as usize,
        new_path.len(),
    );
    check_error(ret)?;
    Ok(())
}
