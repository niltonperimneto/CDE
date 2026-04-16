cargo = find_program('cargo', required : true)
fs = import('fs')

alacritty_manifest = join_paths(meson.project_source_root(), 'cde/programs/alacritty/Cargo.toml')
if get_option('with_alacritty') and fs.exists(alacritty_manifest)
  # Build Alacritty from submodule
  custom_target('alacritty_bin',
    output : ['alacritty'],
    build_by_default : true,
    install : true,
    install_dir : get_option('bindir'),
    console : true,
    command : ['sh', '-c', 'cargo build --release --manifest-path @SOURCE_ROOT@/cde/programs/alacritty/Cargo.toml --target-dir @SOURCE_ROOT@/cde/programs/alacritty/target && cp @SOURCE_ROOT@/cde/programs/alacritty/target/release/alacritty @OUTPUT@']
  )
else
  message('Skipping Alacritty build (missing submodule or with_alacritty=false).')
endif

# Build dtterm shim
custom_target('dtterm_bin',
  output : ['dtterm'],
  build_by_default : true,
  install : true,
  install_dir : get_option('bindir'),
  console : true,
  command : ['sh', '-c', 'cargo build --release --manifest-path @SOURCE_ROOT@/cde/programs/dtterm_shim/Cargo.toml --target-dir @SOURCE_ROOT@/cde/programs/dtterm_shim/target && cp @SOURCE_ROOT@/cde/programs/dtterm_shim/target/release/dtterm_shim @OUTPUT@']
)
