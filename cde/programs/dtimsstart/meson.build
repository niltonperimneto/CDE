# dtimsstart - Input Method Start

dtimsstart_src = [
  'action.c', 'env.c', 'file.c', 'main.c', 'remote.c',
  'select.c', 'start.c', 'util.c', 'win.c'
]

# Generate 0020.dtims
script_defs = [
  '-e', 's|CDE_INSTALLATION_TOP|/usr/dt|g',
  '-e', 's|CDE_CONFIGURATION_TOP|/etc/dt|g',
  '-e', 's|KORNSHELL|/usr/dt/bin/dtksh|g',
  '-e', 's|XCOMM|#|g'
]

dtims = custom_target('0020.dtims',
  input : '0020.dtims.src',
  output : '0020.dtims',
  command : ['sed'] + script_defs + ['@INPUT@'],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'config/Xsession.d'),
  install_mode : 'rwxr-xr-x'
)

# Install config file
install_data('conf/start.conf',
  install_dir : join_paths(get_option('prefix'), 'config/ims')
)

executable('dtimsstart',
  dtimsstart_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc
  ],
  c_args : [common_c_args, '-DNLS'],
  install : true
)
