libobject_sources = [
  'composite.cpp', 'compressed_pstring.cpp', 'cset.cpp', 'dl_list.cpp',
  'dl_list_cell.cpp', 'handler.cpp', 'integer.cpp', 'long_pstring.cpp',
  'oid.cpp', 'oid_list.cpp', 'oid_t.cpp', 'primitive.cpp', 'pstring.cpp',
  'random_gen.cpp', 'root.cpp', 'short_list.cpp', 'tuple.cpp'
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
