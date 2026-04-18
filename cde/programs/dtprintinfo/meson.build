# dtprintinfo

dtprintinfo_inc = include_directories(
  '.', 'UI', 'libUI', 'util', 'libUI/MotifUI', 'objects', 'objects/PrintObj'
)

# Source lists
util_src = [
  'util/Invoke.cpp', 'util/Process.cpp'
]

libui_src = [
  'libUI/BaseUI.cpp', 'libUI/MotifUI/Application.cpp', 'libUI/MotifUI/LabelObj.cpp',
  'libUI/MotifUI/Button.cpp', 'libUI/MotifUI/MainWindow.cpp', 'libUI/MotifUI/ComboBoxObj.cpp',
  'libUI/MotifUI/MenuBar.cpp', 'libUI/MotifUI/Container.cpp', 'libUI/MotifUI/Menu.cpp',
  'libUI/MotifUI/Dialog.cpp', 'libUI/MotifUI/MotifThread.cpp', 'libUI/MotifUI/DtDND.cpp',
  'libUI/MotifUI/MotifUI.cpp', 'libUI/MotifUI/Group.cpp', 'libUI/MotifUI/Prompt.cpp',
  'libUI/MotifUI/HelpSystem.cpp', 'libUI/MotifUI/ScaleObj.cpp', 'libUI/MotifUI/Sep.cpp',
  'libUI/MotifUI/IconObj.cpp', 'libUI/MotifUI/Debug.c', 'libUI/MotifUI/Icon.c',
  'libUI/MotifUI/WorkArea.c'
]

obj_src = [
  'objects/BaseObj.cpp', 'objects/PrintObj/ParseJobs.cpp', 'objects/PrintObj/PrintSubSys.cpp',
  'objects/PrintObj/PrintJob.cpp', 'objects/PrintObj/Queue.cpp'
]

ui_src = [
  'UI/DtActions.cpp', 'UI/DtProps.cpp', 'UI/DtFindSet.cpp', 'UI/DtSetPref.cpp',
  'UI/DtApp.cpp', 'UI/DtPrtJobIcon.cpp', 'UI/DtWorkArea.cpp', 'UI/DtMainW.cpp',
  'UI/DtPrtProps.cpp', 'UI/DtDetailsLabel.cpp', 'UI/DtPrinterIcon.cpp', 'UI/DtFindD.cpp',
  'UI/DtSetModList.cpp'
]

dtprintinfo_sources = [
  'DtPrintinfo.cpp'
] + util_src + libui_src + obj_src + ui_src

# Message generation
cpp = find_program('cpp')
awk = find_program('awk')
ksh = find_program('ksh', 'sh')

# sym2num generates dtprintinfo.msg (stdout) and dtprintinfo_msg.h (side effect)
# We need to ensure CPP is set.
sym2num_env = environment()
sym2num_env.set('CPP', cpp.full_path())

msg_targets = custom_target('dtprintinfo_msg',
  input : 'dtprintinfo.msg.src',
  output : ['dtprintinfo.msg', 'dtprintinfo_msg.h'],
  command : ['sh', '-c', ksh.full_path() + ' @CURRENT_SOURCE_DIR@/sym2num dtprintinfo @INPUT@ > dtprintinfo.msg'],
  capture : false,
  env : sym2num_env
)

dtprintinfo_msg = msg_targets[0]
dtprintinfo_msg_h = msg_targets[1]

# dtprintinfo_cat.h generation
cat_h = custom_target('dtprintinfo_cat.h',
  input : 'dtprintinfo.msg.src',
  output : 'dtprintinfo_cat.h',
  command : [awk, '-f', '@CURRENT_SOURCE_DIR@/msg.awk', '@INPUT@'],
  capture : true
)

executable('dtprintinfo',
  dtprintinfo_sources,
  include_directories : [inc, dtprintinfo_inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm, dep_xpm,
    libDtSvc_dep, libDtWidget_dep, libtt_dep, dep_tirpc
  ],
  c_args : [common_c_args, '-DKORNSHELL="/usr/dt/bin/dtksh"'],
  # Ensure generated headers are built before compiling sources that use them
  # Meson handles this if they are included in sources or via dependency.
  # Since they are headers, valid usage is to add to sources or include_directories (if generated in specific dir).
  # Here they are in build dir.
  sources : [dtprintinfo_msg_h, cat_h],
  install : true
)
