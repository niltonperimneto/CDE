# dtdialog - CDE Dialog Utility
# Replacement for dtksh dialog scripts
message('Processing dtdialog meson.build')

dtdialog_src = ['dtdialog.c']

executable('dtdialog',
  dtdialog_src,
  include_directories : [inc],
  dependencies : [dep_x11, dep_xt, dep_xm],
  c_args : common_c_args,
  install : true
)

install_symlink('dterror',
  pointing_to : 'dtdialog',
  install_dir : join_paths(get_option('prefix'), 'bin')
)

install_symlink('dtfile_error',
  pointing_to : 'dtdialog',
  install_dir : join_paths(get_option('prefix'), 'bin')
)
