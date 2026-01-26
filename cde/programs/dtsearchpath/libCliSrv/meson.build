# libCliSrv - Static C++ Library for dtsearchpath

libCliSrv_src = [
  'DirIterator.C', 'Environ.C', 'TTFile.C', 'UnixEnv.C', 'cstring.C'
]

libCliSrv = static_library('CliSrv',
  libCliSrv_src,
  include_directories : [inc],
  cpp_args : common_c_args, # Use common args, maybe need specific C++ ones?
  # dependencies might be needed if it uses Dt includes
  dependencies : [dep_x11, dep_xt, dep_xm, libDtSvc_dep],
)

libCliSrv_dep = declare_dependency(
  link_with : libCliSrv,
  include_directories : include_directories('.')
)
