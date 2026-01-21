libhmphf_sources = [
  'buckets.C', 'mphf_funcs.C', 'mphf_hash_table.C',
  'params.C', 'pattern.C', 'sorter.C'
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
