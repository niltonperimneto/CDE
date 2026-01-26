# dtcreate - CDE Action Creator

dtcreate_src = [
  'main.c', 'AddFiletype.c', 'Confirmed.c',
  'CreateActionAppShell.c', 'FileCharacteristics.c', 'OpenFile.c',
  'UxXt.c', 'ca_aux.c', 'cmncbs.c', 'cmnutils.c', 'cmnrtns.c', 'ErrorDialog.c',
  'fileio.c', 'af_aux.c', 'icon_selection_dialog.c', 'parser.c'
]

executable('dtcreate',
  dtcreate_src,
  include_directories : [inc],
  dependencies : [
    libtt_dep,
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc
  ],
  c_args : common_c_args,
  install : true
)
