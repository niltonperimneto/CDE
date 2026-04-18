libapi_sources = [
  'base.cpp', 'info_base.cpp', 'info_lib.cpp', 'smart_ptr.cpp', 'transaction.cpp',
  'utility.cpp'
]

libapi = static_library('api',
  libapi_sources,
  include_directories : [include_directories('..'), inc]
)

libapi_dep = declare_dependency(
  link_with : libapi,
  include_directories : include_directories('.')
)
