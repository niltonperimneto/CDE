dtprint_src = [
  'PrintMsgs.c', 'PrintDlgMgr.c', 'PrintOptions.c', 'PrintSetupB.c',
  'PsubDefProc.c', 'PsubUtil.c'
]

libDtPrint = shared_library('DtPrint',
  dtprint_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep
  ],
  c_args : [
    common_c_args,
    '-DMULTIBYTE', '-DI18N_MSG'
  ],
  install : true,
  version : '2.1.0'
)

libDtPrint_dep = declare_dependency(
  link_with : libDtPrint,
  include_directories : include_directories('.')
)
