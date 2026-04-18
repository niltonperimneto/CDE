libdynhash_sources = ['data_t.cpp', 'imp_bucket.cpp', 'imp_die.cpp']

libdynhash = static_library('dynhash',
  libdynhash_sources,
  include_directories : [include_directories('..'), inc]
)

libdynhash_dep = declare_dependency(
  link_with : libdynhash,
  include_directories : include_directories('.')
)
