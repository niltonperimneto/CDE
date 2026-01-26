# dtprintinfo

dtprintinfo_inc = include_directories(
  '.', 'UI', 'libUI', 'util', 'libUI/MotifUI', 'objects', 'objects/PrintObj'
)

# Source lists
util_src = [
  'util/Invoke.C', 'util/Process.C'
]

libui_src = [
  'libUI/BaseUI.C', 'libUI/MotifUI/Application.C', 'libUI/MotifUI/LabelObj.C',
  'libUI/MotifUI/Button.C', 'libUI/MotifUI/MainWindow.C', 'libUI/MotifUI/ComboBoxObj.C',
  'libUI/MotifUI/MenuBar.C', 'libUI/MotifUI/Container.C', 'libUI/MotifUI/Menu.C',
  'libUI/MotifUI/Dialog.C', 'libUI/MotifUI/MotifThread.C', 'libUI/MotifUI/DtDND.C',
  'libUI/MotifUI/MotifUI.C', 'libUI/MotifUI/Group.C', 'libUI/MotifUI/Prompt.C',
  'libUI/MotifUI/HelpSystem.C', 'libUI/MotifUI/ScaleObj.C', 'libUI/MotifUI/Sep.C',
  'libUI/MotifUI/IconObj.C', 'libUI/MotifUI/Debug.c', 'libUI/MotifUI/Icon.c',
  'libUI/MotifUI/WorkArea.c'
]

obj_src = [
  'objects/BaseObj.C', 'objects/PrintObj/ParseJobs.C', 'objects/PrintObj/PrintSubSys.C',
  'objects/PrintObj/PrintJob.C', 'objects/PrintObj/Queue.C'
]

ui_src = [
  'UI/DtActions.C', 'UI/DtProps.C', 'UI/DtFindSet.C', 'UI/DtSetPref.C',
  'UI/DtApp.C', 'UI/DtPrtJobIcon.C', 'UI/DtWorkArea.C', 'UI/DtMainW.C',
  'UI/DtPrtProps.C', 'UI/DtDetailsLabel.C', 'UI/DtPrinterIcon.C', 'UI/DtFindD.C',
  'UI/DtSetModList.C'
]

dtprintinfo_sources = [
  'DtPrintinfo.C'
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
    dep_x11, dep_xt, dep_xm,
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
