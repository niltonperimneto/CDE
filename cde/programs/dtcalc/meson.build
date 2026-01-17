dtcalc_src = [
  'calctool.c', 'ds_common.c', 'ds_popup.c', 'ds_widget.c',
  'ds_xlib.c', 'functions.c', 'help.c', 'motif.c', 'mp.c', 'text.c'
]

executable('dtcalc',
  dtcalc_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc, libm, libtt_dep
  ],
  c_args : [
    common_c_args,
    '-DCDE_INSTALLATION_TOP="@0@"'.format(get_option('prefix'))
  ],
  install : true
)
