//! IO - Input/Output
//!
//! Funções para console e arquivos.

use crate::syscall::{sys_write, SysResult};

/// Imprime string no console (handle 0)
pub fn print(s: &str) -> SysResult<usize> {
    sys_write(0, s.as_bytes())
}

/// Imprime string com nova linha
pub fn println(s: &str) -> SysResult<usize> {
    let mut total = print(s)?;
    total += print("\n")?;
    Ok(total)
}

/// Macro para print formatado
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut w = $crate::io::ConsoleWriter;
        let _ = write!(w, $($arg)*);
    }};
}

/// Macro para println formatado
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }};
}

/// Writer para console (usado pelas macros)
pub struct ConsoleWriter;

impl core::fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = print(s);
        Ok(())
    }
}
