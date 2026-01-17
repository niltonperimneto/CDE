dtcopy_shared_src = [
  'sharedFuncs.c',
  'fsrtns.c'
]

libdtcopy_shared = static_library('dtcopy_shared',
  dtcopy_shared_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep
  ],
  c_args : common_c_args
)

dtfile_copy_src = [
  'main_dtcopy.c',
  'copydialog.c',
  'overwrtdialog.c',
  'dosync.c',
  'utils.c',
  'errordialog.c'
]

executable('dtfile_copy',
  dtfile_copy_src,
  include_directories : [inc],
  link_with : libdtcopy_shared,
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc, libm
  ],
  c_args : common_c_args,
  install : true
)
