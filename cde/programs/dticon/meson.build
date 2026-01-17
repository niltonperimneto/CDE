dticon_src = [
  'dtIconShell.c', 'event.c', 'fileIO.c', 'fileIODialog.c', 'globals.c',
  'graphics.c', 'help.c', 'image.c', 'main.c', 'newIconDialog.c',
  'process.c', 'queryDialog.c', 'stdErrDialog.c', 'utils.c'
]

executable('dticon',
  dticon_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xpm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc, libm, libtt_dep
  ],
  c_args : [
    common_c_args,
    '-D__TOOLTALK'
  ],
  install : true
)
