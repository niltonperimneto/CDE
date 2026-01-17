# Sources from Makefile.am
dtwm_sources = [
  'WmCDInfo.c', 'WmCDecor.c', 'WmCEvent.c',
  'WmCPlace.c', 'WmColormap.c', 'WmError.c',
  'WmEvent.c', 'WmEwmh.c', 'WmFeedback.c',
  'WmFunction.c', 'WmGraphics.c', 'WmIDecor.c',
  'WmIPlace.c', 'WmIconBox.c', 'WmImage.c',
  'WmInitWs.c', 'WmKeyFocus.c', 'WmMain.c',
  'WmManage.c', 'WmMenu.c', 'WmProperty.c',
  'WmProtocol.c', 'WmResCvt.c', 'WmResParse.c',
  'WmResource.c', 'WmSignal.c', 'WmWinConf.c',
  'WmWinInfo.c', 'WmWinList.c', 'WmWinState.c',
  'WmMultiHead.c', 'Button.c', 'Callback.c',
  'Clock.c', 'DataBaseLoad.c', 'PanelS.c',
  'Parse.c', 'PopupMenu.c', 'Session.c',
  'UI.c', 'WmBackdrop.c', 'WmFP.c',
  'WmHelp.c', 'WmIPC.c', 'WmInstantTitle.c',
  'WmOL.c', 'WmParse.c', 'WmParseP.c',
  'WmPresence.c', 'WmWrkspace.c', 'WmXSMP.c'
]

dtfplist_sources = [
  'Print.c', 'Parse.c', 'DataBaseLoad.c', 'WmParse.c', 'Session.c'
]

# Defines from Makefile.am
dtwm_c_args = [
  '-DLARGECURSORS', '-DR2_COMPAT', '-DOPAQUE', '-DSHAPE', '-DUNMAP_ON_RESTART',
  '-DBATCH_DRAG_REQUESTS',
  '-DCDE_INSTALLATION_TOP="/usr/dt"',
  '-DCDE_CONFIGURATION_TOP="/etc/dt"'
]

# libDtWidget = cc.find_library('DtWidget', dirs : ['/usr/dt/lib', meson.current_source_dir() + '/../../lib/DtWidget'])
# libDtSvc = cc.find_library('DtSvc', dirs : ['/usr/dt/lib', meson.current_source_dir() + '/../../lib/DtSvc'])
# Use declared dependencies
# libDtWidget_dep and libDtSvc_dep should be available


# Preprocessor generation for config files
gen_cpp = find_program('cpp')

# Dtwm.defs
custom_target('Dtwm.defs',
  input : 'Dtwm.defs.src',
  output : 'Dtwm.defs',
  command : ['sh', '-c', 'cpp -P -DXCOMM="#" -DCDE_INSTALLATION_TOP="/usr/dt" -DCDE_CONFIGURATION_TOP="/etc/dt" @INPUT@ > @OUTPUT@'],
  install : true,
  install_dir : join_paths(get_option('prefix'), 'lib/X11/app-defaults/C') # Check correct path later
)

# sys.dtwmrc
custom_target('sys.dtwmrc',
  input : 'sys.dtwmrc.src',
  output : 'sys.dtwmrc',
  command : ['sh', '-c', 'cpp -P -DXCOMM="#" -DCDE_INSTALLATION_TOP="/usr/dt" -DCDE_CONFIGURATION_TOP="/etc/dt" @INPUT@ > @OUTPUT@'],
  install : true,
  install_dir : join_paths(get_option('prefix'), 'config') # Check correct path later
)

executable('dtwm',
  dtwm_sources,
  include_directories : inc,
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xext, dep_ice, dep_sm,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm, dep_tirpc,
    dependency('xrender')
  ],
  c_args : dtwm_c_args,
  install : true
)

executable('dtfplist',
  dtfplist_sources,
  include_directories : inc,
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm, dep_tirpc
  ],
  c_args : dtwm_c_args, # Reuse args?
  install : true
)
