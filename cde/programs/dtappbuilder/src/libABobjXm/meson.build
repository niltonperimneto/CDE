libABobjXm_sources = [
  'objxm_args.c', 'objxm_config.c', 'objxm_conn.c', 'objxm_inst.c',
  'objxm_objects.c', 'objxm_strs.c', 'objxm_util.c'
]

libABobjXm = static_library('ABobjXm',
  libABobjXm_sources,
  include_directories : [include_directories('../include'), inc],
  c_args : ['-D_POSIX_SOURCE=1']
)

libABobjXm_dep = declare_dependency(
  link_with : libABobjXm,
  include_directories : include_directories('../include')
)
