tt_api_dnd_dir = join_paths(tt_lib_src_root, 'api/dnd')
libtt_api_dnd_sources = [
  join_paths(tt_api_dnd_dir, 'ttdnd.c'),
]

libtt_api_dnd = static_library('tt_api_dnd',
  libtt_api_dnd_sources,
  include_directories : [inc, tt_inc, include_directories('../../../..')],
  dependencies : dep_tirpc,
  pic : true)
