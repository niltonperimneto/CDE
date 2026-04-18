tt_tttk_dir = join_paths(tt_lib_src_root, 'tttk')
libtt_tttk_sources = [
  join_paths(tt_tttk_dir, 'eclipse.cpp'),
  join_paths(tt_tttk_dir, 'ttdesktop.cpp'),
  join_paths(tt_tttk_dir, 'ttdtfile.cpp'),
  join_paths(tt_tttk_dir, 'ttdtprocid.cpp'),
  join_paths(tt_tttk_dir, 'ttmedia.cpp'),
  join_paths(tt_tttk_dir, 'tttk.cpp'),
  join_paths(tt_tttk_dir, 'tttk2free.cpp'),
  join_paths(tt_tttk_dir, 'tttkmessage.cpp'),
  join_paths(tt_tttk_dir, 'tttkpattern.cpp'),
  join_paths(tt_tttk_dir, 'tttkutils.cpp'),
]

libtt_tttk = static_library('tt_tttk',
  libtt_tttk_sources,
  include_directories : [inc, tt_inc, include_directories('../../../..')],
  dependencies : dep_tirpc,
  pic : true
)
