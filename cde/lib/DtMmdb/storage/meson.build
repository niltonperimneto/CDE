libstorage_sources = [
  'abs_storage.C', 'chunks_index.C', 'heap_comp_funcs.C',
  'lru.C', 'page.C', 'page_cache.C', 'page_rep.C',
  'page_storage.C', 'rep_cell.C', 'rep_policy.C',
  'store_test.C', 'unixf_storage.C', 'version.C',
  'vm_storage.C'
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
