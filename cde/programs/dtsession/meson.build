dtsession_sources = [
  'OWsync.c', 'SmAuth.c', 'SmCommun.c', 'SmConMgmt.c',
  'SmDB.c', 'SmError.c', 'SmGlobals.c', 'SmHelp.c',
  'SmHftRing.c', 'SmLock.c', 'SmMain.c', 'SmMigResources.c',
  'SmProp.c', 'SmProperty.c', 'SmProtocol.c', 'SmRestore.c',
  'SmSave.c', 'SmScreen.c', 'SmStrDefs.c', 'SmUI.c',
  'SmWatch.c', 'SmWindow.c', 'SmXdef.c', 'SmXrm.c',
  'SmXSMP.c', 'SrvFile_io.c', 'SrvPalette.c'
]

executable('dtsession',
  dtsession_sources,
  include_directories : inc,
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xext, dep_ice, dep_sm,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm, dep_tirpc,
    dependency('libcrypt')
  ],
  c_args : [
    '-DCDE_INSTALLATION_TOP="/usr/dt"',
    '-DCDE_CONFIGURATION_TOP="/etc/dt"',
    '-Ddtsession'
  ],
  install : true
)
