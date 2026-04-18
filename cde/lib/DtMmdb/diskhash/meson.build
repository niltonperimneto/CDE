libdiskhash_sources = ['disk_bucket.cpp', 'bucket_array.cpp', 'disk_hash.cpp']

libdiskhash = static_library('diskhash',
  libdiskhash_sources,
  include_directories : [include_directories('..'), inc]
)

libdiskhash_dep = declare_dependency(
  link_with : libdiskhash,
  include_directories : include_directories('.')
)
