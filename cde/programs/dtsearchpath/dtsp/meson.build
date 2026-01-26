# dtsearchpath - executable

dtsearchpath_src = [
  'AppSearchPath.C', 'DbSearchPath.C', 'HelpSearchPath.C',
  'IconSearchPath.C', 'InfoLibSearchPath.C', 'ManSearchPath.C',
  'Options.C', 'SearchPath.C', 'dtsearchpath.C'
]

executable('dtsearchpath',
  dtsearchpath_src,
  include_directories : [inc, include_directories('../libCliSrv')],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libCliSrv_dep
  ],
  c_args : common_c_args,
  install : true
)
