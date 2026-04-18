libbtree_sources = ['btree.cpp']

libbtree = static_library('btree',
  libbtree_sources,
  include_directories : [include_directories('..'), inc]
)

libbtree_dep = declare_dependency(
  link_with : libbtree,
  include_directories : include_directories('.')
)
