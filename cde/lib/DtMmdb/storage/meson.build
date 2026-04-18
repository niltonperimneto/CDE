libstorage_sources = [
  'abs_storage.cpp', 'chunks_index.cpp', 'heap_comp_funcs.cpp',
  'lru.cpp', 'page.cpp', 'page_cache.cpp', 'page_rep.cpp',
  'page_storage.cpp', 'rep_cell.cpp', 'rep_policy.cpp',
  'store_test.cpp', 'unixf_storage.cpp', 'version.cpp',
  'vm_storage.cpp'
]

libstorage = static_library('storage',
  libstorage_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DPORTABLE_DB']
)

libstorage_dep = declare_dependency(
  link_with : libstorage,
  include_directories : include_directories('.')
)
