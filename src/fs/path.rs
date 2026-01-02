//! # Path Utilities
//!
//! Utilitários para manipulação de caminhos.

/// Verifica se um path é absoluto
pub fn is_absolute(path: &str) -> bool {
    path.starts_with('/')
}

/// Verifica se um path é relativo
pub fn is_relative(path: &str) -> bool {
    !is_absolute(path)
}

/// Obtém o nome do arquivo (última componente)
///
/// # Exemplo
/// ```rust
/// assert_eq!(file_name("/apps/hello.txt"), "hello.txt");
/// assert_eq!(file_name("/apps/"), "");
/// ```
pub fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or("")
}

/// Obtém o diretório pai
///
/// # Exemplo
/// ```rust
/// assert_eq!(parent("/apps/hello.txt"), "/apps");
/// assert_eq!(parent("/apps"), "/");
/// ```
pub fn parent(path: &str) -> &str {
    if let Some(pos) = path.rfind('/') {
        if pos == 0 {
            "/"
        } else {
            &path[..pos]
        }
    } else {
        ""
    }
}

/// Obtém a extensão do arquivo
///
/// # Exemplo
/// ```rust
/// assert_eq!(extension("hello.txt"), Some("txt"));
/// assert_eq!(extension("hello"), None);
/// ```
pub fn extension(path: &str) -> Option<&str> {
    let name = file_name(path);
    if let Some(pos) = name.rfind('.') {
        if pos > 0 && pos < name.len() - 1 {
            return Some(&name[pos + 1..]);
        }
    }
    None
}

/// Obtém o nome sem extensão
///
/// # Exemplo
/// ```rust
/// assert_eq!(stem("hello.txt"), "hello");
/// assert_eq!(stem("hello"), "hello");
/// ```
pub fn stem(path: &str) -> &str {
    let name = file_name(path);
    if let Some(pos) = name.rfind('.') {
        if pos > 0 {
            return &name[..pos];
        }
    }
    name
}

/// Junta dois paths
///
/// Se `child` for absoluto, retorna `child`.
/// Caso contrário, junta com `/`.
///
/// # Exemplo
/// ```rust
/// let mut buf = [0u8; 256];
/// let len = join("/apps", "hello.txt", &mut buf);
/// // buf contém "/apps/hello.txt"
/// ```
pub fn join<'a>(base: &str, child: &str, buf: &'a mut [u8]) -> Option<&'a str> {
    if child.starts_with('/') {
        // Child é absoluto
        let len = child.len().min(buf.len());
        buf[..len].copy_from_slice(&child.as_bytes()[..len]);
        return core::str::from_utf8(&buf[..len]).ok();
    }

    let base = base.trim_end_matches('/');
    let total = base.len() + 1 + child.len();

    if total > buf.len() {
        return None;
    }

    buf[..base.len()].copy_from_slice(base.as_bytes());
    buf[base.len()] = b'/';
    buf[base.len() + 1..total].copy_from_slice(child.as_bytes());

    core::str::from_utf8(&buf[..total]).ok()
}

/// Normaliza um path (remove //, ., ..)
///
/// # Exemplo
/// ```rust
/// let mut buf = [0u8; 256];
/// let path = normalize("/apps/../system/./services", &mut buf);
/// // path é "/system/services"
/// ```
pub fn normalize<'a>(path: &str, buf: &'a mut [u8]) -> Option<&'a str> {
    let is_abs = path.starts_with('/');
    let mut components: [&str; 64] = [""; 64];
    let mut count = 0;

    for component in path.split('/') {
        match component {
            "" | "." => continue,
            ".." => {
                if count > 0 {
                    count -= 1;
                }
            }
            _ => {
                if count < 64 {
                    components[count] = component;
                    count += 1;
                }
            }
        }
    }

    // Construir resultado
    let mut offset = 0;

    if is_abs && buf.len() > 0 {
        buf[0] = b'/';
        offset = 1;
    }

    for (i, comp) in components[..count].iter().enumerate() {
        if i > 0 {
            if offset >= buf.len() {
                return None;
            }
            buf[offset] = b'/';
            offset += 1;
        }

        let comp_bytes = comp.as_bytes();
        if offset + comp_bytes.len() > buf.len() {
            return None;
        }
        buf[offset..offset + comp_bytes.len()].copy_from_slice(comp_bytes);
        offset += comp_bytes.len();
    }

    if offset == 0 {
        if buf.len() > 0 {
            buf[0] = b'/';
            return Some("/");
        }
        return None;
    }

    core::str::from_utf8(&buf[..offset]).ok()
}
