# Implementation Plan: Rust Local Help Server

**Goal**: Embed a lightweight HTTP server in `dtbrowser` to serve local legacy help content.

## 1. Rust Implementation (Std Lib Only)
To avoid dependency issues, we use `std::net::TcpListener`.

### Logic
1.  **Start**: `dtbrowser_server_start()`
    - Spawns a background thread.
    - Binds `TcpListener::bind("127.0.0.1:0")`.
    - Gets local port.
    - Loops `accept()`.
2.  **Request Handling**:
    - Read `TcpStream` into buffer.
    - Parse `GET /path ...`.
    - **Path Mapping**:
        - `/` -> Index page
        - `/help/` -> `/usr/dt/appconfig/help/`
    - **Response**:
        - Write `HTTP/1.1 200 OK`.
        - Content-Type detection (html, css, png).
        - Stream file content.
3.  **Return**: The port number to C.

## 2. Integration
- `main.c` calls `start()`.
- Constructs URL `http://127.0.0.1:{port}/index.html`.
- Navigates Browser.

