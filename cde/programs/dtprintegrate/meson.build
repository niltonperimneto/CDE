# dtprintegrate - Printer Integration Scripts

# Defines: CDE_INSTALLATION_TOP, CDE_CONFIGURATION_TOP, DTKORNSHELL, KORNSHELL
# Using sed substitution as in other script ports.

script_defs = [
  '-e', 's|CDE_INSTALLATION_TOP|/usr/dt|g',
  '-e', 's|CDE_CONFIGURATION_TOP|/etc/dt|g',
  '-e', 's|DTKORNSHELL|/usr/dt/bin/dtksh|g', # or meson.current_build_dir() + '/../dtksh' if we built it?
  '-e', 's|KORNSHELL|/usr/dt/bin/dtksh|g',
  '-e', 's|XCOMM|#|g'
]

dtprintegrate = custom_target('dtprintegrate',
  input : 'dtprintegrate.src',
  output : 'dtprintegrate',
  command : ['sed'] + script_defs + ['@INPUT@'],
  capture : true,
  install : true,
  install_dir : get_option('bindir'),
  install_mode : 'rwxr-xr-x'
)

dtlp = custom_target('dtlp',
  input : 'dtlp.src',
  output : 'dtlp',
  command : ['sed'] + script_defs + ['@INPUT@'],
  capture : true,
  install : true,
  install_dir : get_option('bindir'),
  install_mode : 'rwxr-xr-x'
)

dtlpsetup = custom_target('dtlpsetup',
  input : 'dtlpsetup.src',
  output : 'dtlpsetup',
  command : ['sed'] + script_defs + ['@INPUT@'],
  capture : true,
  install : true,
  install_dir : get_option('bindir'),
  install_mode : 'rwxr-xr-x'
)
