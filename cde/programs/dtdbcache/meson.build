# dtdbcache - CDE Database Cache

dtdbcache_src = ['Main.c']

executable('dtdbcache',
  dtdbcache_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc
  ],
  c_args : common_c_args,
  install : true
)
