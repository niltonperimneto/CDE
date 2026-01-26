# Implementation Plan: Rust IPC Safety Layer (`libdtbrowser_ipc`)

**Goal**: Create a memory-safe Rust library to handle insecure ToolTalk strings and bridge them to the C++ CEF browser.

## 1. Rust Crate Configuration
- **Path**: `cde/programs/dtbrowser/ipc`
- **Crate Type**: `staticlib` (to link into `dtbrowser` executable).
- **Dependencies**: `libc` (for C types).

## 2. API Design (C Header)
We will expose a clean C interface to `main.c`:

```c
// dtbrowser_ipc.h
#include <stddef.h>

typedef enum {
    IPC_CMD_UNKNOWN = 0,
    IPC_CMD_NAVIGATE = 1,
    IPC_CMD_SHUTDOWN = 2
} IpcCommandType;

typedef struct {
    IpcCommandType type;
    char* url; // Must be freed by caller (or managed)
} IpcCommand;

// Initialize the IPC layer
void dtbrowser_ipc_init();

// Parse a raw ToolTalk message string (operation + args)
// Returns a structured command, shielding C++ from raw parsing.
IpcCommand dtbrowser_ipc_parse(const char* op, const char* arg);

// Free the command resources
void dtbrowser_ipc_free_command(IpcCommand cmd);
```

## 3. Rust Implementation (`lib.rs`)
- `no_mangle` functions matching the C header.
- Use `CStr` to safely read C strings.
- Use `String` / `ENUM` matching for safe parsing logic.

## 4. Build System (Meson)
- Use `import('rust')` if available, or `custom_target` invoking `cargo build`.
- Link the resulting `libdtbrowser_ipc.a` into `dtbrowser`.

## 5. Verification
- Call `dtbrowser_ipc_parse` from `main.c` instead of doing `strcmp` manually.
