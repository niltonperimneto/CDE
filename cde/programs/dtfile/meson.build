subdir('dtcopy')

dtfile_src = [
  'ChangeDir.c', 'ChangeDirP.c', 'Command.c', 'Common.c',
  'Desktop.c', 'Directory.c', 'Encaps.c', 'File.c',
  'FileDialog.c', 'FileManip.c', 'FileMgr.c', 'FileOp.c',
  'Filter.c', 'FilterP.c', 'Find.c', 'FindP.c',
  'Help.c', 'HelpCB.c', 'HelpP.c', 'IconWindow.c',
  'IconicPath.c', 'Main.c', 'Menu.c', 'MkDir.c',
  'ModAttr.c', 'ModAttrP.c', 'MultiView.c', 'OverWrite.c',
  'Prefs.c', 'PrefsP.c', 'SharedMsgs.c', 'SharedProcs.c',
  'ToolTalk.c', 'Trash.c', 'Utils.c', 'fsDialog.c'
]

# dtfile needs include path to dtcopy for headers
dtfile_inc = include_directories('.', 'dtcopy')

# Generated configurations
# Note: Makefile uses -P which handles pre-processing without line markers
cpp_cmd = [find_program('cpp'), '-P', '-DXCOMM=#']

# SCRIPTFLAGS from Makefile
script_flags = [
  '-DSHAPE', '-D_ILS_MACROS', '-DSUN_PERF',
  '-DCDE_INSTALLATION_TOP="/usr/dt"', # Using fixed path for now or get_option
  '-DCDE_CONFIGURATION_TOP="/etc/dt"'
]
# We should probably use common c_args logic for defines, but CPP needs string literals carefully

custom_target('dtfile.config',
  input : 'dtfile.config.cpp',
  output : 'dtfile.config',
  command : [cpp_cmd, script_flags, '@INPUT@'],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'config', 'C') # configdir
)

custom_target('dtfile_error',
  input : 'dtfile_error.cpp',
  output : 'dtfile_error',
  command : [cpp_cmd, script_flags, '@INPUT@'],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

custom_target('dterror.ds',
  input : 'dterror.src',
  output : 'dterror.ds',
  command : [cpp_cmd, script_flags, '@INPUT@'],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin') 
)

executable('dtfile',
  dtfile_src,
  include_directories : [inc, dtfile_inc],
  link_with : libdtcopy_shared,
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xext,
    libDtWidget_dep, libDtSvc_dep, libtt_dep, libm, dep_tirpc
  ],
  c_args : [common_c_args, '-DSHAPE', '-D_ILS_MACROS', '-DSUN_PERF'],
  install : true
)
