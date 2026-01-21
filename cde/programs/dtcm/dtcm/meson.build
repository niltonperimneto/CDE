dtcm_src_dir = '../../../../programs/dtcm/dtcm/'

dtcm_sources = [
  'MonthPanel.c', 'RFCMIME.c', 'about.c', 'alarm.c', 'blist.c', 'browser.c',
  'calendarA.c', 'cm_i18n.c', 'cmtt.c', 'datefield.c', 'dayglance.c',
  'deskset.c', 'dnd.c', 'dssw.c', 'editor.c', 'find.c', 'fns.c', 'font.c',
  'format.c', 'goto.c', 'graphics.c', 'group_editor.c', 'help.c',
  'i18nxlate.c', 'icon.c', 'md5.c', 'misc.c', 'monthglance.c', 'namesvc.c',
  'print.c', 'props.c', 'props_pu.c', 'reminders.c', 'rfp.c', 'select.c',
  'session.c', 'submit.c', 'tempbr.c', 'timezone.c', 'todo.c',
  'weekglance.c', 'x_graphics.c', 'yearglance.c'
]

dtcm_sources_with_path = []
foreach s : dtcm_sources
  dtcm_sources_with_path += dtcm_src_dir + s
endforeach

dtcm_editor_sources = [
    'datefield.c', 'deskset.c', 'dssw.c', 'dtcm_editor.c', 
    'help.c', 'misc.c', 'reminders.c', 'rfp.c'
]
dtcm_editor_sources_with_path = []
foreach s : dtcm_editor_sources
    dtcm_editor_sources_with_path += dtcm_src_dir + s
endforeach


executable('dtcm',
  sources : dtcm_sources_with_path,
  include_directories : [
    inc,
    include_directories('../../../../programs/dtcm/libDtCmP'),
    include_directories('../../../../programs/dtcm/dtcm'),
    include_directories('../../../include/csa'),
    include_directories('../../../lib/csa')
  ],
  dependencies : [
    libDtCmP_dep, libcsa_dep, libDtWidget_dep, libDtSvc_dep, 
    libDtHelp_dep, dep_x11, dep_xt, dep_xm, dep_tirpc, libm, libtt_dep,
    libDtPrint_dep
  ],
  c_args : ['-DICONV_CONST='],
  install : true
)

executable('dtcm_delete',
  sources : [dtcm_src_dir + 'cm_delete.c'],
  include_directories : [
      inc,
      include_directories('../../../../programs/dtcm/libDtCmP'),
      include_directories('../../../../programs/dtcm/dtcm'),
      include_directories('../../../include/csa'),
      include_directories('../../../lib/csa')
  ],
  dependencies : [libDtCmP_dep, libcsa_dep, libDtSvc_dep, dep_tirpc],
  install : true
)

executable('dtcm_insert',
  sources : [dtcm_src_dir + 'cm_insert.c'],
  include_directories : [
      inc,
      include_directories('../../../../programs/dtcm/libDtCmP'),
      include_directories('../../../../programs/dtcm/dtcm'),
      include_directories('../../../include/csa'),
      include_directories('../../../lib/csa')
  ],
  dependencies : [libDtCmP_dep, libcsa_dep, libDtSvc_dep, dep_tirpc],
  install : true
)

executable('dtcm_lookup',
  sources : [dtcm_src_dir + 'cm_lookup.c'],
  include_directories : [
      inc,
      include_directories('../../../../programs/dtcm/libDtCmP'),
      include_directories('../../../../programs/dtcm/dtcm'),
      include_directories('../../../include/csa'),
      include_directories('../../../lib/csa')
  ],
  dependencies : [libDtCmP_dep, libcsa_dep, libDtSvc_dep, dep_tirpc],
  install : true
)

executable('dtcm_editor',
    sources : dtcm_editor_sources_with_path,
    include_directories : [
        inc,
        include_directories('../../../../programs/dtcm/libDtCmP'),
        include_directories('../../../../programs/dtcm/dtcm'),
        include_directories('../../../include/csa'),
        include_directories('../../../lib/csa')
    ],
    dependencies : [
        libDtCmP_dep, libcsa_dep, libDtWidget_dep, libDtSvc_dep, 
        libDtHelp_dep, dep_x11, dep_xt, dep_xm, dep_tirpc, libm, libtt_dep,
        libDtPrint_dep
    ],
    c_args : ['-DICONV_CONST='],
    install : true
)
