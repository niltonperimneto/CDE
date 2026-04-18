libindex_sources = [
  'btree_index.cpp', 'dyn_disk_index.cpp', 'dyn_index.cpp',
  'dyn_memory_index.cpp', 'fast_mphf.cpp', 'hash.cpp', 'index.cpp',
  'inv_lists.cpp', 'mphf_index.cpp'
]

libindex = static_library('index',
  libindex_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DPORTABLE_DB']
)

libindex_dep = declare_dependency(
  link_with : libindex,
  include_directories : include_directories('.')
)
