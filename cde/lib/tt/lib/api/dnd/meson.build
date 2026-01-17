libtt_api_dnd_sources = [
  'ttdnd.c',
]

libtt_api_dnd = static_library('tt_api_dnd',
  libtt_api_dnd_sources,
  include_directories : [inc, tt_inc, include_directories('../../../..')],
  dependencies : dep_tirpc,
  pic : true)
