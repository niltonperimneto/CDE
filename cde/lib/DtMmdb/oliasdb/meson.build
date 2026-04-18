liboliasdb_sources = [
  'asciiIn_filters.cpp', 'collectionIterator.cpp', 'dlp_hd.cpp',
  'doc_hd.cpp', 'graphic_hd.cpp', 'locator_hd.cpp', 'mark.cpp',
  'mark_base.cpp', 'mmdb.cpp', 'node_hd.cpp', 'olias_funcs.cpp',
  'stylesheet_hd.cpp', 'toc_hd.cpp', 'user_base.cpp'
]

liboliasdb = static_library('oliasdb',
  liboliasdb_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DNO_DB_LOCK']
)

liboliasdb_dep = declare_dependency(
  link_with : liboliasdb,
  include_directories : include_directories('.')
)
