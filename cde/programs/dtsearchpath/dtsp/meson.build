# dtsearchpath - executable

dtsearchpath_src = [
  'AppSearchPath.cpp', 'DbSearchPath.cpp', 'HelpSearchPath.cpp',
  'IconSearchPath.cpp', 'InfoLibSearchPath.cpp', 'ManSearchPath.cpp',
  'Options.cpp', 'SearchPath.cpp', 'dtsearchpath.cpp'
]

executable('dtsearchpath',
  dtsearchpath_src,
  include_directories : [inc, include_directories('../libCliSrv')],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libCliSrv_dep, dep_tirpc
  ],
  c_args : common_c_args,
  install : true
)
