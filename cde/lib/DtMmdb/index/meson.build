libindex_sources = [
  'btree_index.C', 'dyn_disk_index.C', 'dyn_index.C',
  'dyn_memory_index.C', 'fast_mphf.C', 'hash.C', 'index.C',
  'inv_lists.C', 'mphf_index.C'
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
