libutility_sources = [
  'funcs.cpp', 'pm_random.cpp', 'atoi_pearson.cpp', 'xtime.cpp', 'buffer.cpp',
  'atoi_larson.cpp', 'atomic_lock.cpp', 'rw_lock.cpp', 'atoi_fast.cpp',
  'filter.cpp', 'mmdb_exception.cpp', 'randomize.cpp'
]

libutility = static_library('utility',
  libutility_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DPORTABLE_DB']
)

libutility_dep = declare_dependency(
  link_with : libutility,
  include_directories : include_directories('.')
)
