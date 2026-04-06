use std::ffi::CStr;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_char;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

// AtomicBool eliminates the data race that `static mut bool` caused.
static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn dtbrowser_server_start(root: *const c_char) -> u16 {
    // Guard against null pointer before any dereference.
    if root.is_null() {
        return 0;
    }
    let root_path = unsafe { CStr::from_ptr(root) }.to_string_lossy().to_string();

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return 0,
    };

    let addr = match listener.local_addr() {
        Ok(a) => a,
        Err(_) => return 0,
    };

    let port = addr.port();
    SERVER_RUNNING.store(true, Ordering::SeqCst);

    thread::spawn(move || {
        eprintln!("[Rust-Server] Listening on http://127.0.0.1:{}", port);
        for stream in listener.incoming() {
            if !SERVER_RUNNING.load(Ordering::SeqCst) {
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
    SERVER_RUNNING.store(false, Ordering::SeqCst);
    // In a real app we'd need to unblock the listener, e.g. connect to self
}

fn handle_client(mut stream: TcpStream, root: &str) {
    let mut buffer = [0; 1024];
    if let Err(_) = stream.read(&mut buffer) {
        return;
    }

    let request = String::from_utf8_lossy(&buffer);
    let path = parse_request(&request);

    // Security: canonicalize the target path and verify it stays within the allowed root.
    // A simple `..` string check is insufficient — URL-encoded sequences and symlinks
    // can both bypass it.  Canonicalization resolves all components first.
    let base = Path::new(root);
    let relative = path.trim_start_matches('/');

    // Reject any request that still carries a literal ".." component after stripping.
    if relative.split('/').any(|c| c == "..") {
        let response = "HTTP/1.1 403 Forbidden\r\n\r\nAccess Denied";
        let _ = stream.write(response.as_bytes());
        return;
    }

    let file_path = if path.starts_with("/help/") {
        let help_base = Path::new("/usr/dt/appconfig/help");
        help_base.join(path.trim_start_matches("/help/"))
    } else {
        base.join(relative)
    };

    // Default to index.html if dir
    let file_path = if file_path.is_dir() {
        file_path.join("index.html")
    } else {
        file_path
    };

    // Canonical check: resolved path must begin with the declared root.
    let final_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\nFile Not Found";
            let _ = stream.write(response.as_bytes());
            return;
        }
    };
    let allowed_root = match base.canonicalize() {
        Ok(r) => r,
        Err(_) => {
            let response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
            let _ = stream.write(response.as_bytes());
            return;
        }
    };
    if !final_path.starts_with(&allowed_root) && !final_path.starts_with("/usr/dt/appconfig/help") {
        let response = "HTTP/1.1 403 Forbidden\r\n\r\nAccess Denied";
        let _ = stream.write(response.as_bytes());
        return;
    }

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
