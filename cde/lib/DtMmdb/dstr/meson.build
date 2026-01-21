libdstr_sources = [
  'set.C', 'bset.C', 'slist.C', 'dlist_cell.C', 'dlist.C', 'heap.C',
  'void_ptr_array.C', 'void_ptr_stack.C', 'memory_pool.C',
  'dstr_test.C', 'index_agent.C', 'token_stack.C',
  'slist_char_ptr_cell.C'
]

libdstr = static_library('dstr',
  libdstr_sources,
  include_directories : [include_directories('..'), inc]
)

libdstr_dep = declare_dependency(
  link_with : libdstr,
  include_directories : include_directories('.')
)
