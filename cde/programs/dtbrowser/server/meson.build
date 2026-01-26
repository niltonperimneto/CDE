
# Build Rust static library for Help Server
dtbrowser_server_lib = custom_target(
    'dtbrowser_server',
    output: 'libdtbrowser_server.a',
    command: [
        'cargo', 'build',
        '--release',
        '--manifest-path', meson.current_source_dir() / 'Cargo.toml',
        '&&',
        'cp', meson.current_source_dir() / 'target/release/libdtbrowser_server.a', '@OUTPUT@'
    ],
    build_by_default: true
)

dtbrowser_server_dep = declare_dependency(
    link_with: dtbrowser_server_lib,
    include_directories: include_directories('.')
)
