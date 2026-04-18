libhmphf_sources = [
  'buckets.cpp', 'mphf_funcs.cpp', 'mphf_hash_table.cpp',
  'params.cpp', 'pattern.cpp', 'sorter.cpp'
]

libhmphf = static_library('hmphf',
  libhmphf_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DPORTABLE_DB']
)

libhmphf_dep = declare_dependency(
  link_with : libhmphf,
  include_directories : include_directories('.')
)
