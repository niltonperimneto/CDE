tt_tttk_dir = join_paths(tt_lib_src_root, 'tttk')
libtt_tttk_sources = [
  join_paths(tt_tttk_dir, 'eclipse.C'),
  join_paths(tt_tttk_dir, 'ttdesktop.C'),
  join_paths(tt_tttk_dir, 'ttdtfile.C'),
  join_paths(tt_tttk_dir, 'ttdtprocid.C'),
  join_paths(tt_tttk_dir, 'ttmedia.C'),
  join_paths(tt_tttk_dir, 'tttk.C'),
  join_paths(tt_tttk_dir, 'tttk2free.C'),
  join_paths(tt_tttk_dir, 'tttkmessage.C'),
  join_paths(tt_tttk_dir, 'tttkpattern.C'),
  join_paths(tt_tttk_dir, 'tttkutils.C'),
]

libtt_tttk = static_library('tt_tttk',
  libtt_tttk_sources,
  include_directories : [inc, tt_inc, include_directories('../../../..')],
  dependencies : dep_tirpc,
  pic : true
)
