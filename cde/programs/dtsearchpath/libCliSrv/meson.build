# libCliSrv - Static C++ Library for dtsearchpath

libCliSrv_src = [
  'DirIterator.cpp', 'Environ.cpp', 'TTFile.cpp', 'UnixEnv.cpp', 'cstring.cpp'
]

libCliSrv = static_library('CliSrv',
  libCliSrv_src,
  include_directories : [inc],
  cpp_args : common_c_args, # Use common args, maybe need specific C++ ones?
  # dependencies might be needed if it uses Dt includes
  dependencies : [dep_x11, dep_xt, dep_xm, libDtSvc_dep, dep_tirpc],
)

libCliSrv_dep = declare_dependency(
  link_with : libCliSrv,
  include_directories : include_directories('.')
)
