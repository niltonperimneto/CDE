
custom_target('dtappintegrate',
  input : 'dtappintegrate.src',
  output : 'dtappintegrate',
  command : [
    'sed',
    '-e', 's/XCOMM/#/g',
    '-e', 's|CDE_INSTALLATION_TOP|' + get_option('prefix') + '|g',
    '-e', 's|CDE_CONFIGURATION_TOP|/etc/dt|g',
    '@INPUT@'
  ],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin'),
  install_mode : 'rwxr-xr-x'
)

install_data('dtappintegrate.tmsg',
  install_dir : join_paths(get_option('prefix'), 'appconfig/appmanager/C')
)
