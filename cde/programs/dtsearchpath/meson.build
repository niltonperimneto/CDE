# dtsearchpath

subdir('libCliSrv')
subdir('dtsp')
subdir('dtappg')

# Generate 0010.dtpaths
# Defines from makefile: CDE_INSTALLATION_TOP, CDE_CONFIGURATION_TOP, CDE_LOGFILES_TOP, KORNSHELL
# We use sed for simplicity, mirroring legacy logic but with meson paths.

dtpaths = custom_target('0010.dtpaths',
  input : '0010.dtpaths.src',
  output : '0010.dtpaths',
  command : [
    'sed',
    '-e', 's|CDE_INSTALLATION_TOP|/usr/dt|g',
    '-e', 's|CDE_CONFIGURATION_TOP|/etc/dt|g',
    '-e', 's|CDE_LOGFILES_TOP|/var/dt|g',
    '-e', 's|KORNSHELL|/usr/dt/bin/dtksh|g', # or /bin/ksh
    '-e', 's|XCOMM|#|g',
    '@INPUT@'
  ],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'config/Xsession.d'),
  install_mode : 'rwxr-xr-x'
)

dttmpdir = custom_target('0030.dttmpdir',
  input : '0030.dttmpdir.src',
  output : '0030.dttmpdir',
  command : [
    'sed',
    '-e', 's|CDE_INSTALLATION_TOP|/usr/dt|g',
    '-e', 's|CDE_CONFIGURATION_TOP|/etc/dt|g',
    '-e', 's|CDE_LOGFILES_TOP|/var/dt|g',
    '-e', 's|KORNSHELL|/usr/dt/bin/dtksh|g',
    '-e', 's|XCOMM|#|g',
    '@INPUT@'
  ],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'config/Xsession.d'),
  install_mode : 'rwxr-xr-x'
)

