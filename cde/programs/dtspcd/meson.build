# dtspcd - CDE Subprocess Control Daemon

dtspcd_src = ['main.c', 'spcd_event.c']

# Convert dtspcdenv.src to dtspcdenv
# We can use a custom target with sed or cpp. The legacy build uses cpp.
# Defines: CDE_CONFIGURATION_TOP, CDE_LOGFILES_TOP, DEF_SYSTEM_PATH
# For simplification, we'll assume standard paths or use meson options.

dtspcdenv = custom_target('dtspcdenv',
  input : 'dtspcdenv.src',
  output : 'dtspcdenv',
  command : [
    'sed',
    '-e', 's|CDE_CONFIGURATION_TOP|/etc/dt|g', # Replace with actual path or variable
    '-e', 's|CDE_LOGFILES_TOP|/var/dt|g',
    '-e', 's|DEF_SYSTEM_PATH|/bin:/usr/bin|g',
    '-e', 's|XCOMM|#|g',
    '@INPUT@'
  ],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'config'), # check makefile
  install_mode : 'rwxr-xr-x'
)

# Makefile uses dtspcd_CFLAGS with paths
dtspcd_c_args = [
  common_c_args,
  '-DCDE_CONFIGURATION_TOP="/etc/dt"', # Should match prefix logic
  '-DCDE_INSTALLATION_TOP="/usr/dt"',
]

executable('dtspcd',
  dtspcd_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc
  ],
  c_args : dtspcd_c_args,
  install : true
)
