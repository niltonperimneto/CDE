# dtpdm - CDE Print Dialog Manager

dtpdm_src = [
  'JobBox.c', 'Main.c', 'MainWindow.c', 'PdmMsgs.c', 'PdmOid.c',
  'PdmXp.c', 'PrinterBox.c'
]

executable('dtpdm',
  dtpdm_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc,
    libtt_dep
  ],
  c_args : [common_c_args, '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DI18N_MSG'],
  install : true
)
