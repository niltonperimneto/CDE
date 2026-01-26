
# Build Rust static library for IPC
dtbrowser_ipc_lib = custom_target(
    'dtbrowser_ipc',
    output: 'libdtbrowser_ipc.a',
    command: [
        'cargo', 'build',
        '--release',
        '--manifest-path', meson.current_source_dir() / 'Cargo.toml',
        '&&',
        'cp', meson.current_source_dir() / 'target/release/libdtbrowser_ipc.a', '@OUTPUT@'
    ],
    build_by_default: true
)

dtbrowser_ipc_dep = declare_dependency(
    link_with: dtbrowser_ipc_lib,
    include_directories: include_directories('.')
)
