libdti_excs_sources = [
  'Jump_Environment.C', 'Exceptions.C', 'Exception.C',
  'Destructable.C', 'terminate.C'
]

libdti_excs = static_library('dti_excs',
  libdti_excs_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DNATIVE_EXCEPTIONS']
)

libdti_excs_dep = declare_dependency(
  link_with : libdti_excs,
  include_directories : include_directories('.')
)
