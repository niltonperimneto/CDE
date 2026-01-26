
custom_target('dtopen',
  input : 'dtopen.src',
  output : 'dtopen',
  command : [
    'sed',
    '-e', 's/XCOMM/#/g',
    '-e', 's|CDE_INSTALLATION_TOP|' + get_option('prefix') + '|g',
    '@INPUT@'
  ],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin'),
  install_mode : 'rwxr-xr-x'
)

meson.add_install_script('make_dtopen_links.sh', 
  join_paths(get_option('prefix'), 'bin')
)
