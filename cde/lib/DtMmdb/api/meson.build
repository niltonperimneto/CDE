libapi_sources = [
  'base.C', 'info_base.C', 'info_lib.C', 'smart_ptr.C', 'transaction.C',
  'utility.C'
]

libapi = static_library('api',
  libapi_sources,
  include_directories : [include_directories('..'), inc]
)

libapi_dep = declare_dependency(
  link_with : libapi,
  include_directories : include_directories('.')
)
