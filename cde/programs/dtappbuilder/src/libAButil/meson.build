libAButil_sources = [
  'abio.c', 'abio_path.c', 'istr.c', 'strlist.c', 'util.c',
  'util_convert.c', 'util_ds.c', 'util_err.c', 'util_file.c',
  'util_string.c'
]

libAButil = static_library('AButil',
  libAButil_sources,
  include_directories : [include_directories('../include'), inc],
  c_args : ['-D_POSIX_SOURCE=1']
)

libAButil_dep = declare_dependency(
  link_with : libAButil,
  include_directories : include_directories('../include')
)
