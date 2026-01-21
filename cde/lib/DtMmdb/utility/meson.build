libutility_sources = [
  'funcs.C', 'pm_random.C', 'atoi_pearson.C', 'xtime.C', 'buffer.C',
  'atoi_larson.C', 'atomic_lock.C', 'rw_lock.C', 'atoi_fast.C',
  'filter.C', 'mmdb_exception.C', 'randomize.C'
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
