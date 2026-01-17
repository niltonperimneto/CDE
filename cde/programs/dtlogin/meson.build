# CDE_CONFIGURATION_TOP and CDE_INSTALLATION_TOP are defined in top-level meson
# but we need to pass them as defines.

dtlogin_inc = include_directories('.')

# Resource file needs specific defines
resource_c_args = [
  '-DDEF_SESSION="' + get_option('prefix') + '/bin/Xsession"',
  '-DDEF_SYSTEM_SHELL="/bin/sh"',
  '-DDEF_CHOOSER="' + get_option('prefix') + '/bin/dtchooser"',
  '-DDEF_XDM_CONFIG="Xconfig"',
  '-DDEF_SERVER_LINE=":0 Local local ' + get_option('prefix') + '/bin/X :0"',
  '-DXRDB_PROGRAM="' + get_option('prefix') + '/bin/xrdb"',
  '-DDEF_USER_PATH="/usr/bin:/bin:' + get_option('prefix') + '/bin"',
  '-DDEF_SYSTEM_PATH="/usr/bin:/bin:' + get_option('prefix') + '/bin"',
  '-DDEF_FAILSAFE_CLIENT="/usr/bin/xterm"',
  '-DDEF_AUTH_FILE="/var/dt/auth-server"',
  '-DDEF_AUTH_DIR="/var/dt"',
  '-DCPP_PROGRAM="/usr/bin/cpp"',
  '-DDEF_PM_SEARCH_PATH="' + get_option('prefix') + '/appconfig/icons/%L/%B%M.pm:' + get_option('prefix') + '/appconfig/icons/%L/%B%M.bm"',
  '-DDEF_BM_SEARCH_PATH="' + get_option('prefix') + '/appconfig/icons/%L/%B%M.bm:' + get_option('prefix') + '/appconfig/icons/%L/%B%M.pm"',
  '-DCDE_CONFIGURATION_TOP="/etc/dt"',
  '-DCDE_INSTALLATION_TOP="/usr/dt"',
  '-DCDE_LOGFILES_TOP="/var/dt"',
  '-DBINDIR="/usr/bin"',
  '-DXDMDIR="/var/dt"',
  '-DKORNSHELL="/usr/bin/ksh"', 
  '-DUNIXCONN', '-DTCPCONN', '-DXDMCP',
  '-DLARGECURSORS', '-DR2_COMPAT', '-DOPAQUE', '-DSHAPE', '-DPAM'
]

libresource = static_library('resource',
  'resource.c',
  include_directories : [inc, include_directories('.')],
  # Only use external dependencies for c_args/cflags. Internal libs headers are in 'inc'.
  dependencies : [dep_x11, dep_xt],
  c_args : resource_c_args
)

common_c_args = [
  '-DCDE_CONFIGURATION_TOP="/etc/dt"',
  '-DCDE_INSTALLATION_TOP="/usr/dt"',
  '-DCDE_LOGFILES_TOP="/var/dt"',
  '-DBINDIR="/usr/bin"',
  '-DXDMDIR="/var/dt"',
  '-DKORNSHELL="/usr/bin/ksh"',
  '-DUNIXCONN', '-DTCPCONN', '-DXDMCP',
  '-DLARGECURSORS', '-DR2_COMPAT', '-DOPAQUE', '-DSHAPE', '-DPAM'
]

if host_machine.system() == 'linux'
  common_c_args += ['-DHASDES', '-DHASXDMAUTH', '-D__linux__', '-D_GNU_SOURCE']
endif

dtlogin_src = [
  'access.c', 'auth.c', 'genauth.c', 'daemon.c', 'dm.c',
  'dpylist.c', 'error.c', 'file.c', 'mitauth.c', 'protodpy.c', 'policy.c',
  'reset.c', 'server.c', 'session.c', 'socket.c', 'util.c',
  'verify.c', 'sysauth.c', 'fontpath.c', 'qualify.c', 'choose.c', 'netaddr.c',
  'xdmcp.c', 'pam_svc.c', 'solaris.c' 
]
# On linux we also need xdmauth.c
if host_machine.system() == 'linux'
    dtlogin_src += 'xdmauth.c'
endif

executable('dtlogin',
  dtlogin_src,
  include_directories : [inc, dtlogin_inc],
  link_with : libresource,
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xext, dep_ice, dep_sm,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm, dep_tirpc, dep_xmu, dep_xau,
    dependency('xdmcp'),
    dependency('pam', required: false),
    dependency('libcrypt', required: false)
  ],
  c_args : common_c_args,
  install : true
)

dtgreet_src = [
  'vgcallback.c', 'vglogo.c', 'vgmain.c', 'vgutil.c', 'vglang.c'
]

executable('dtgreet',
  dtgreet_src,
  include_directories : [inc, dtlogin_inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xext,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm
  ],
  c_args : common_c_args,
  install : true
)

dtchooser_src = [
  'chooser.c', 'dtchooser.c', 'vglogo.c', 'vgutil.c', 'vgcallback.c', 'vglang.c'
]

executable('dtchooser',
  dtchooser_src,
  include_directories : [inc, dtlogin_inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xext,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm,
    dependency('xdmcp')
  ],
  c_args : common_c_args,
  install : true
)

# Systemd Integration
systemd_config = configuration_data()
systemd_config.set('bindir', join_paths(get_option('prefix'), 'bin'))

configure_file(
  input : '../../contrib/rc/systemd/dtlogin.service.in',
  output : 'dtlogin.service',
  configuration : systemd_config,
  install : true,
  install_dir : 'lib/systemd/system'
)
