libobject_sources = [
  'composite.C', 'compressed_pstring.C', 'cset.C', 'dl_list.C',
  'dl_list_cell.C', 'handler.C', 'integer.C', 'long_pstring.C',
  'oid.C', 'oid_list.C', 'oid_t.C', 'primitive.C', 'pstring.C',
  'random_gen.C', 'root.C', 'short_list.C', 'tuple.C'
]

libobject = static_library('object',
  libobject_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DPORTABLE_DB']
)

libobject_dep = declare_dependency(
  link_with : libobject,
  include_directories : include_directories('.')
)
