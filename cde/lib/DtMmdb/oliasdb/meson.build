liboliasdb_sources = [
  'asciiIn_filters.C', 'collectionIterator.C', 'dlp_hd.C',
  'doc_hd.C', 'graphic_hd.C', 'locator_hd.C', 'mark.C',
  'mark_base.C', 'mmdb.C', 'node_hd.C', 'olias_funcs.C',
  'stylesheet_hd.C', 'toc_hd.C', 'user_base.C'
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
