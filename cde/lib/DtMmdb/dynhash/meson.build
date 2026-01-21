libdynhash_sources = ['data_t.C', 'imp_bucket.C', 'imp_die.C']

libdynhash = static_library('dynhash',
  libdynhash_sources,
  include_directories : [include_directories('..'), inc]
)

libdynhash_dep = declare_dependency(
  link_with : libdynhash,
  include_directories : include_directories('.')
)
