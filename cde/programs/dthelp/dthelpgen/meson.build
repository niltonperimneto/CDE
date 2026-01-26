# dthelpgen meson build

dthelpgen_src = ['helpgen.c']

# dthelpgen needs internal DtHelp headers and DtI
dthelpgen_inc = include_directories('.', '../../../lib/DtHelp', '../../../include/DtI')

executable('dthelpgen',
  dthelpgen_src,
  include_directories : [inc, dthelpgen_inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtHelp_dep, libDtSvc_dep, dep_tirpc
  ],
  c_args : common_c_args,
  install : true
)

# Install the wrapper script as dthelpgen.ds
install_data('dthelpgen_wrapper',
  rename : 'dthelpgen.ds',
  install_dir : join_paths(get_option('prefix'), 'bin'),
  install_mode : 'rwxr-xr-x'
)
