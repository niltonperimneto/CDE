dtwidget_sources = [
  'ComboBox.c', 'Control.c', 'DialogBox.c', 'DtMsgs.c', 'DtStrDefs.c',
  'EditAreaData.c', 'EditCalls.c', 'Editor.c', 'Icon.c', 'MenuButton.c',
  'NewCvt.c', 'SearchCalls.c', 'SearchDlg.c', 'SpinBox.c', 'TitleBox.c',
  'View.c'
  # UNIXbindings.cpp needs special handling usually, maybe build as C++
]

libDtWidget = library('DtWidget',
  dtwidget_sources,
  include_directories : [inc, include_directories('../../include/Dt'), include_directories('../../include')],
  dependencies : [dep_x11, dep_xt, dep_xm, libm, libDtSvc_dep],
  install : true
)

cpp = find_program('cpp')
custom_target('UNIXbindings',
  input : 'UNIXbindings.cpp',
  output : 'UNIXbindings',
  command : [cpp, '-P', '-DXCOMM=#', '-I' + meson.project_source_root() + '/include', '@INPUT@', '@OUTPUT@'],
  install : true,
  install_dir : get_option('prefix') / 'lib/X11/app-defaults/C'
)

libDtWidget_dep = declare_dependency(
  link_with : libDtWidget,
  include_directories : include_directories('.')
)
