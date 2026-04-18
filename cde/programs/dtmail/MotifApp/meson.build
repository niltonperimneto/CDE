# MotifApp - Static Library

motifapp_src = [
  'Application.cpp', 'AskFirstCmd.cpp', 'BasicComponent.cpp', 'BusyPixmap.cpp',
  'ButtonInterface.cpp', 'Cmd.cpp', 'CmdInterface.cpp', 'CmdList.cpp',
  'DialogManager.cpp', 'Help.cpp', 'IconifyCmd.cpp', 'InfoDialogManager.cpp',
  'InterruptibleCmd.cpp', 'Main.cpp', 'MainWindow.cpp', 'ManageCmd.cpp',
  'MenuBar.cpp', 'MenuWindow.cpp', 'MotifCmds.cpp', 'NoUndoCmd.cpp',
  'PixmapCycler.cpp', 'PromptDialogManager.cpp', 'QuestionDialogManager.cpp',
  'QuitCmd.cpp', 'ScrollingList.cpp', 'SelectFileCmd.cpp', 'ToggleButtonInterface.cpp',
  'UIComponent.cpp', 'UndoCmd.cpp', 'WarnNoUndoCmd.cpp', 'WorkingDialogManager.cpp'
]

# Includes: -I../include/MotifApp -I../include -I../include/utils
motifapp_inc = include_directories('.', '../include', '../include/MotifApp', '../include/utils')

libMotifApp = static_library('MotifApp',
  motifapp_src,
  include_directories : [inc, motifapp_inc],
  dependencies : [dep_x11, dep_xt, dep_xm, libDtSvc_dep, dep_tirpc],
  cpp_args : common_c_args
)

libMotifApp_dep = declare_dependency(
  link_with : libMotifApp,
  include_directories : motifapp_inc
)
