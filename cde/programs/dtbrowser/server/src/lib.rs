use std::ffi::{CStr, CString};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::thread;

static mut SERVER_RUNNING: bool = false;

#[no_mangle]
pub extern "C" fn dtbrowser_server_start(root: *const c_char) -> u16 {
    let c_root = unsafe { CStr::from_ptr(root) };
    let root_path = c_root.to_string_lossy().to_string();

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return 0,
    };

    let addr = match listener.local_addr() {
        Ok(a) => a,
        Err(_) => return 0,
    };

    let port = addr.port();
    unsafe { SERVER_RUNNING = true };

    thread::spawn(move || {
        eprintln!("[Rust-Server] Listening on http://127.0.0.1:{}", port);
        for stream in listener.incoming() {
            if unsafe { !SERVER_RUNNING } {
                break;
            }
            match stream {
                Ok(stream) => {
                    let root = root_path.clone();
                    thread::spawn(move || handle_client(stream, &root));
                }
                Err(e) => eprintln!("[Rust-Server] Connection failure: {}", e),
            }
        }
    });

    port
}

#[no_mangle]
pub extern "C" fn dtbrowser_server_stop() {
    unsafe { SERVER_RUNNING = false };
    // In a real app we'd need to unblock the listener, e.g. connect to self
}

fn handle_client(mut stream: TcpStream, root: &str) {
    let mut buffer = [0; 1024];
    if let Err(_) = stream.read(&mut buffer) {
        return;
    }

    let request = String::from_utf8_lossy(&buffer);
    let path = parse_request(&request);

    // Security: Prevent directory traversal
    if path.contains("..") {
        let response = "HTTP/1.1 403 Forbidden\r\n\r\nAccess Denied";
        let _ = stream.write(response.as_bytes());
        return;
    }

    let file_path = Path::new(root).join(path.trim_start_matches('/'));

    // Default to index.html if dir
    let final_path = if file_path.is_dir() {
        file_path.join("index.html")
    } else {
        file_path
    };

    if final_path.exists() && final_path.is_file() {
        let content = match fs::read(&final_path) {
            Ok(c) => c,
            Err(_) => {
                let _ = stream.write(b"HTTP/1.1 500 Internal Server Error\r\n\r\n");
                return;
            }
        };

        let mime = get_mime_type(&final_path);
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            mime,
            content.len()
        );

        let _ = stream.write(header.as_bytes());
        let _ = stream.write(&content);
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\nFile Not Found";
        let _ = stream.write(response.as_bytes());
    }
}

fn parse_request(request: &str) -> String {
    // GET /path HTTP/1.1
    let parts: Vec<&str> = request.split_whitespace().collect();
    if parts.len() > 1 {
        parts[1].to_string()
    } else {
        "/".to_string()
    }
}

fn get_mime_type(path: &Path) -> &str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
}
