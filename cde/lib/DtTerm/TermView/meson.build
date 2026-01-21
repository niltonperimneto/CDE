libTermView_sources = [
  'TermView.c', 'TermViewMenu.c',
  'TermViewGlobalDialog.c', 'TermViewTerminalDialog.c'
]

libTermView = static_library('TermView',
  libTermView_sources,
  include_directories : [inc, inc_dt, include_directories('../TermPrim'), include_directories('../Term'), include_directories('.')],
  c_args : ['-D_POSIX_SOURCE', '-D_XOPEN_SOURCE=500']
)

libTermView_dep = declare_dependency(
  link_with : libTermView,
  include_directories : include_directories('.')
)
