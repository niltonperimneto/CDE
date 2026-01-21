libmisc_sources = ['unique_id.c']

libmisc = static_library('misc',
  libmisc_sources,
  include_directories : [include_directories('..'), inc]
)

libmisc_dep = declare_dependency(
  link_with : libmisc,
  include_directories : include_directories('.')
)
