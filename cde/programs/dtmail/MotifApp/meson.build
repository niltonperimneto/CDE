# MotifApp - Static Library

motifapp_src = [
  'Application.C', 'AskFirstCmd.C', 'BasicComponent.C', 'BusyPixmap.C',
  'ButtonInterface.C', 'Cmd.C', 'CmdInterface.C', 'CmdList.C',
  'DialogManager.C', 'Help.C', 'IconifyCmd.C', 'InfoDialogManager.C',
  'InterruptibleCmd.C', 'Main.C', 'MainWindow.C', 'ManageCmd.C',
  'MenuBar.C', 'MenuWindow.C', 'MotifCmds.C', 'NoUndoCmd.C',
  'PixmapCycler.C', 'PromptDialogManager.C', 'QuestionDialogManager.C',
  'QuitCmd.C', 'ScrollingList.C', 'SelectFileCmd.C', 'ToggleButtonInterface.C',
  'UIComponent.C', 'UndoCmd.C', 'WarnNoUndoCmd.C', 'WorkingDialogManager.C'
]

# Includes: -I../include/MotifApp -I../include -I../include/utils
motifapp_inc = include_directories('.', '../include', '../include/MotifApp', '../include/utils')

libMotifApp = static_library('MotifApp',
  motifapp_src,
  include_directories : [inc, motifapp_inc],
  dependencies : [dep_x11, dep_xt, dep_xm, libDtSvc_dep],
  cpp_args : common_c_args
)

libMotifApp_dep = declare_dependency(
  link_with : libMotifApp,
  include_directories : motifapp_inc
)
