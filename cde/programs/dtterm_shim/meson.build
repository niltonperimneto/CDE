cargo = find_program('cargo', required : true)

# Build Alacritty from submodule
custom_target('alacritty_bin',
  output : ['alacritty'],
  build_by_default : true,
  install : true,
  install_dir : get_option('bindir'),
  console : true,
  command : ['sh', '-c', 'cargo build --release --manifest-path @SOURCE_ROOT@/programs/alacritty/Cargo.toml --target-dir @SOURCE_ROOT@/programs/alacritty/target && cp @SOURCE_ROOT@/programs/alacritty/target/release/alacritty @OUTPUT@']
)

# Build dtterm shim
custom_target('dtterm_bin',
  output : ['dtterm'],
  build_by_default : true,
  install : true,
  install_dir : get_option('bindir'),
  console : true,
  command : ['sh', '-c', 'cargo build --release --manifest-path @SOURCE_ROOT@/programs/dtterm_shim/Cargo.toml --target-dir @SOURCE_ROOT@/programs/dtterm_shim/target && cp @SOURCE_ROOT@/programs/dtterm_shim/target/release/dtterm_shim @OUTPUT@']
)
