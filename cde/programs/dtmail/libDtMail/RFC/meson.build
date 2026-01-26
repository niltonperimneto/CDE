# libDtMail/RFC

libdtmail_rfc = static_library('DtMailRFC',
  sources : run_command('sh', '-c', 'ls *.C', check: true).stdout().strip().split('\n'),
  include_directories : [inc, include_directories('../../include')],
  dependencies : [dep_x11, dep_xt, libDtSvc_dep],
  cpp_args : common_c_args
)

libdtmail_rfc_dep = declare_dependency(
  link_with : libdtmail_rfc,
  include_directories : include_directories('.')
)
