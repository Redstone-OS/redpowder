# 🔥 Redpowder

**SDK oficial para desenvolvimento no Redstone OS**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)

---

## 📖 O que é?

**Redpowder** é o kit de desenvolvimento para criar aplicações que rodam no [Redstone OS](https://github.com/redstone-os/redstone). Ele fornece uma API de alto nível, type-safe e no_std para interagir com o kernel Forge.

### ✨ Features

| Feature | Descrição |
|---------|-----------|
| 🔧 **Syscalls** | 50+ wrappers seguros para chamadas ao kernel |
| 📂 **Filesystem** | Abstração completa de arquivos e diretórios |
| 📝 **Console** | Macros `print!` e `println!` |
| 🧠 **Memory** | Alocação de memória virtual |
| 📨 **IPC** | Comunicação entre processos via portas |
| ⏱️ **Time** | Funções de tempo e sleep |
| 🎨 **Graphics** | Acesso ao framebuffer |
| 🖱️ **Input** | Mouse e teclado |
| 🪟 **Window** | Sistema de janelas (futuro) |

---

## 📦 Instalação

```toml
[dependencies]
redpowder = { path = "../redpowder" }
```

Ou clone como subcrate no seu projeto Redstone.

---

## 🚀 Uso Rápido

```rust
#![no_std]
#![no_main]

use redpowder::prelude::*;
use redpowder::fs::{File, Dir};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Redstone OS!");
    
    // Ler arquivo
    if let Ok(file) = File::open("/apps/config.txt") {
        let mut buf = [0u8; 256];
        if let Ok(bytes) = file.read(&mut buf) {
            println!("Lido {} bytes", bytes);
        }
    }
    
    // Listar diretório
    if let Ok(dir) = Dir::open("/apps") {
        for entry in dir.entries() {
            println!("  {}", entry.name());
        }
    }
    
    exit(0);
}
```

---

## 📚 Módulos

### 📂 `fs` - Sistema de Arquivos

O módulo de filesystem fornece abstrações de alto nível para arquivos e diretórios.

```rust
use redpowder::fs::{File, Dir, stat, exists, chdir, getcwd};

// === ARQUIVOS ===

// Abrir para leitura
let file = File::open("/apps/hello.txt")?;

// Criar/truncar para escrita
let file = File::create("/apps/output.txt")?;

// Ler dados
let mut buf = [0u8; 1024];
let bytes = file.read(&mut buf)?;

// Escrever dados
file.write(b"Hello World")?;

// Seek
file.seek(0, SeekFrom::Start)?;

// Stat
let info = file.stat()?;
println!("Tamanho: {} bytes", info.size);

// === DIRETÓRIOS ===

// Listar diretório
for entry in Dir::open("/apps")?.entries() {
    if entry.is_file() {
        println!("📄 {}", entry.name());
    } else {
        println!("📁 {}", entry.name());
    }
}

// === OPERAÇÕES ===

// Verificar existência
if exists("/apps/hello.txt") {
    println!("Arquivo existe!");
}

// Verificar tipo
if is_dir("/apps") {
    println!("É um diretório");
}

// Diretório de trabalho
let mut cwd_buf = [0u8; 256];
let cwd = getcwd(&mut cwd_buf)?;
println!("CWD: {}", cwd);

// Mudar diretório
chdir("/apps")?;
```

#### Tipos do Módulo `fs`

| Tipo | Descrição |
|------|-----------|
| `File` | Handle de arquivo aberto |
| `Dir` | Handle de diretório aberto |
| `ReadDir` | Iterator sobre entradas de diretório |
| `DirEntry` | Entrada de diretório (nome, tipo) |
| `FileStat` | Metadados de arquivo (tamanho, tipo, times) |
| `OpenFlags` | Flags de abertura (O_RDONLY, O_CREATE, etc) |
| `SeekFrom` | Origem para seek (Start, Current, End) |
| `FileType` | Tipo de arquivo (Regular, Directory, Symlink) |

#### Funções do Módulo `fs`

| Função | Descrição |
|--------|-----------|
| `stat(path)` | Obtém metadados de arquivo |
| `exists(path)` | Verifica se path existe |
| `is_file(path)` | Verifica se é arquivo regular |
| `is_dir(path)` | Verifica se é diretório |
| `getcwd(buf)` | Obtém diretório atual |
| `chdir(path)` | Muda diretório atual |
| `mkdir(path, mode)` | Cria diretório |
| `rmdir(path)` | Remove diretório vazio |
| `unlink(path)` | Remove arquivo |
| `rename(old, new)` | Renomeia/move arquivo |

---

### 🖥️ `console` - Saída de Console

```rust
use redpowder::console::{print, println, reboot, poweroff};

print!("Sem quebra de linha");
println!("Com quebra de linha");
println!("Formatado: {} + {} = {}", 1, 2, 3);

// Controle do sistema
reboot();     // Reinicia
poweroff();   // Desliga
```

---

### 🧠 `mem` - Memória

```rust
use redpowder::mem::{alloc, free, map};

let ptr = alloc(4096)?;  // Aloca 4KB
free(ptr, 4096)?;        // Libera
```

---

### 📨 `ipc` - Comunicação Entre Processos

```rust
use redpowder::ipc::{Port, create_port, send, recv};

let port = Port::create(32)?;        // Cria porta
port.send(b"mensagem")?;             // Envia
let n = port.recv(&mut buf, 0)?;     // Recebe
```

---

### ⏱️ `time` - Tempo

```rust
use redpowder::time::{sleep, clock};

sleep(1000);              // Dorme 1000ms
let ticks = clock();      // Ticks desde boot
```

---

### 🎨 `graphics` - Framebuffer

```rust
use redpowder::graphics::{Framebuffer, Color};

let fb = Framebuffer::get()?;
let info = fb.info();
println!("Resolução: {}x{}", info.width, info.height);

fb.clear(Color::BLACK)?;
fb.pixel(100, 100, Color::RED)?;
```

---

### 🖱️ `input` - Mouse e Teclado

```rust
use redpowder::input::{poll_mouse, poll_keyboard, KeyEvent};

if let Some(state) = poll_mouse() {
    println!("Mouse: ({}, {})", state.x, state.y);
}

if let Some(key) = poll_keyboard() {
    println!("Tecla: {:?}", key);
}
```

---

### 🔧 `syscall` - Acesso Direto

Para casos onde você precisa de acesso direto às syscalls:

```rust
use redpowder::syscall::*;

// Syscalls raw
let ret = syscall3(SYS_OPEN, ptr, len, flags);
let result = check_error(ret)?;
```

---

## 🎯 Prelude

Para importar os tipos e funções mais comuns:

```rust
use redpowder::prelude::*;
```

Inclui:
- `File`, `Dir`, `DirEntry`, `FileStat`, `OpenFlags`
- `stat`, `exists`, `is_file`, `is_dir`, `getcwd`, `chdir`
- `print!`, `println!`
- `exit`, `getpid`, `yield_now`
- `sleep`
- `Handle`, `HandleRights`
- `Color`, `Framebuffer`
- `SysError`, `SysResult`

---

## ⚙️ Requisitos

- **Rust nightly** (inline asm)
- **Target**: `x86_64-redstone` ou target personalizado
- **Flags**: `#![no_std]`, `#![no_main]`

---

## 📜 Syscalls Suportadas

O SDK expõe todas as syscalls do kernel Forge:

| Range | Categoria | Quantidade |
|-------|-----------|------------|
| 0x01-0x0F | Processo | 9 |
| 0x10-0x1F | Memória | 5 |
| 0x20-0x2F | Handles | 3 |
| 0x30-0x3F | IPC | 8 |
| 0x40-0x4F | Gráficos/Input | 5 |
| 0x50-0x5F | Tempo | 4 |
| 0x60-0x7F | **Filesystem** | **32** |
| 0x80-0x8F | Events | 1 |
| 0xF0-0xFF | Sistema | 6 |

---

## 📁 Estrutura do Projeto

```
redpowder/
├── src/
│   ├── lib.rs           # Módulo principal
│   ├── syscall/         # Syscalls raw
│   │   ├── numbers.rs   # Números de syscall
│   │   ├── raw.rs       # Invocação inline asm
│   │   └── error.rs     # SysError, SysResult
│   ├── fs/              # Filesystem
│   │   ├── types.rs     # OpenFlags, FileStat, etc
│   │   ├── file.rs      # Abstração File
│   │   ├── dir.rs       # Abstração Dir, ReadDir
│   │   ├── ops.rs       # stat, exists, chdir, etc
│   │   └── path.rs      # Utilitários de path
│   ├── console/         # print!, println!
│   ├── mem/             # Alocação de memória
│   ├── ipc/             # Portas e mensagens
│   ├── time/            # Sleep e clock
│   ├── graphics/        # Framebuffer
│   ├── input/           # Mouse e teclado
│   └── ...
├── Cargo.toml
└── README.md
```

---

## 📄 Licença

MIT - Veja [LICENSE](LICENSE)

---

## 🔗 Links

- [Redstone OS](https://github.com/redstone-os/redstone)
- [Forge Kernel](../forge/)
- [Changelog](CHANGELOG.md)

---

<div align="center">
<i>Redpowder SDK — Build the Future with Redstone OS</i>
</div>
