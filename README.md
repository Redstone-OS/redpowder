# 🔥 Redpowder

**SDK oficial para desenvolvimento no Redstone OS**

[![Crates.io](https://img.shields.io/crates/v/redpowder.svg)](https://crates.io/crates/redpowder)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## O que é?

Redpowder é o kit de desenvolvimento para criar aplicações que rodam no [Redstone OS](https://github.com/redstone-os/redstone). Ele fornece:

- 🔧 **Syscalls** - Wrappers seguros para chamadas ao kernel
- 📝 **IO** - Macros `print!` e `println!` para console
- 🧠 **Memory** - Alocação de memória virtual
- 📨 **IPC** - Comunicação entre processos via portas
- ⏱️ **Time** - Funções de tempo e sleep

## Instalação

```toml
[dependencies]
redpowder = "0.1"
```

## Uso Rápido

```rust
#![no_std]
#![no_main]

use redpowder::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Redstone OS!");
    
    let pid = sys_getpid();
    println!("Meu PID: {}", pid);
    
    sleep(1000); // Dorme 1 segundo
    
    sys_exit(0);
}
```

## Módulos

### `syscall` - Chamadas de Sistema

```rust
use redpowder::syscall::*;

sys_exit(0);           // Encerra processo
sys_yield();           // Cede CPU
sys_getpid();          // Obtém PID
sys_write(0, b"Hi");   // Escreve no console
```

### `io` - Input/Output

```rust
use redpowder::io::*;

print!("Sem quebra de linha");
println!("Com quebra de linha");
println!("Formatado: {}", 42);
```

### `memory` - Memória

```rust
use redpowder::memory::*;

let ptr = alloc_rw(4096)?;  // Aloca 4KB
free(ptr, 4096)?;           // Libera
```

### `ipc` - Comunicação

```rust
use redpowder::ipc::*;

let port = create_port(32)?;        // Cria porta
send(port, b"mensagem")?;           // Envia
let n = recv(port, &mut buf, 0)?;   // Recebe
```

### `time` - Tempo

```rust
use redpowder::time::*;

sleep(1000);              // Dorme 1000ms
let ticks = monotonic();  // Ticks desde boot
let ms = uptime_ms();     // Uptime em ms
```

## Prelude

Para importar tudo de uma vez:

```rust
use redpowder::prelude::*;
```

Inclui: `sys_exit`, `sys_yield`, `sys_getpid`, `print`, `println`, `sleep`, `monotonic`, `SysError`, `SysResult`

## Requisitos

- Rust nightly
- Target: `x86_64-redstone` (ou `x86_64-unknown-none`)
- `#![no_std]`

## Licença

MIT - Veja [LICENSE](LICENSE)

## Links

- [Redstone OS](https://github.com/redstone-os/redstone)
- [Documentação](https://docs.rs/redpowder)
- [Changelog](CHANGELOG.md)
