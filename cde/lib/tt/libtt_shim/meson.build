# Build the Rust library using Cargo
# We produce both .so (cdylib) and .a (staticlib) but usually link against .so
libtt_shim_target = custom_target('libtt',
  input : ['src/lib.rs', 'Cargo.toml'],
  output : ['libtt.so'], 
  command : [
    'sh', '-c',
    'export RUSTFLAGS="-C relocation-model=pic -C link-arg=-Wl,-soname,libtt.so" && cargo build --release --manifest-path @CURRENT_SOURCE_DIR@/Cargo.toml && cp @CURRENT_SOURCE_DIR@/target/release/libtt_shim.so @OUTPUT0@'
  ],
  build_by_default : true,
  install : true,
  install_dir : get_option('libdir')
)

# Declare dependency for other Meson targets to use
libtt_inc = include_directories('src') # To be safe if we add headers later
libtt_dep = declare_dependency(
  link_with : libtt_shim_target,
  include_directories : libtt_inc,
  link_args : ['-L' + meson.current_build_dir(), '-ltt']
)

# NOTE: For generic usage, we might need a header file 'tt_c.h' for C consumers.
# Since we are essentially replacing libtt, consumers expect <Tt/tt_c.h>.
# We should ensure that search path finds the original headers OR we provide a shim header.
# For now, we reuse original headers in `include/Tt` but link against our shim.
