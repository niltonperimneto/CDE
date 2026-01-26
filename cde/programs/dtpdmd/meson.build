# dtpdmd - CDE Print Dialog Manager Daemon

dtpdmd_src = [
  'dispatch.c', 'dtpdmd.c', 'mailbox.c', 'manager.c',
  'records.c', 'nlmsg.c', 'setup.c', 'util.c'
]

executable('dtpdmd',
  dtpdmd_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc,
    libtt_dep
  ],
  c_args : [common_c_args, '-DCDE_INSTALLATION_TOP="/usr/dt"'],
  install : true,
  install_dir : join_paths(get_option('prefix'), 'lib/dt') # Usually daemons go to libexec or similar? Legacy puts it in bin? 
  # Makefile says bin_PROGRAMS, so install to bin.
)
