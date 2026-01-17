dtpad_src = [
  'dtpad.c', 'editCB.c', 'fileCB.c', 'fileDlg.c', 'fileIo.c', 'formatCB.c',
  'helpCB.c', 'helpDlg.c', 'main.c', 'optionsCB.c', 'printJob.c',
  'printOutput.c', 'printSetup.c', 'session.c', 'ttMsgSupport.c',
  'ttSaveSupport.c'
]

executable('dtpad',
  dtpad_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc, libm, libtt_dep,
    libDtPrint_dep
  ],
  c_args : [
    common_c_args,
    '-DDT_LITE', '-DWORDWRAP'
  ],
  install : true
)
