executable('dthelpview',
  ['Main.c', 'ManPage.c', 'Util.c'],
  include_directories : inc,
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtHelp_dep, libDtSvc_dep
  ],
  c_args : ['-D_BMS', '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DCDE_CONFIGURATION_TOP="/etc/dt"'],
  install : true
)
