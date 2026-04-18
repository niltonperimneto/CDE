libmgrs_sources = [
  'managers.cpp', 'misc.cpp', 'query_mgr.cpp', 'template_mgr.cpp'
]

libmgrs = static_library('mgrs',
  libmgrs_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DCOMPACTED_DISK_REP', '-DPORTABLE_DB']
)

libmgrs_dep = declare_dependency(
  link_with : libmgrs,
  include_directories : include_directories('.')
)
