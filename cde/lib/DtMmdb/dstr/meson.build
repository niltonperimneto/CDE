libdstr_sources = [
  'set.cpp', 'bset.cpp', 'slist.cpp', 'dlist_cell.cpp', 'dlist.cpp', 'heap.cpp',
  'void_ptr_array.cpp', 'void_ptr_stack.cpp', 'memory_pool.cpp',
  'dstr_test.cpp', 'index_agent.cpp', 'token_stack.cpp',
  'slist_char_ptr_cell.cpp'
]

libdstr = static_library('dstr',
  libdstr_sources,
  include_directories : [include_directories('..'), inc]
)

libdstr_dep = declare_dependency(
  link_with : libdstr,
  include_directories : include_directories('.')
)
