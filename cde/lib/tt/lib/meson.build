tt_inc = include_directories('.')

subdir('api/c')
subdir('api/dnd')
subdir('db')
subdir('mp')
subdir('util')
subdir('tttk')

# Combine static libraries into libtt (either as a shared lib or just a collection)
# For now, we'll create a shared library linking all the static parts

libtt = shared_library('tt',
  link_whole : [libtt_api_c, libtt_api_dnd, libtt_db, libtt_mp, libtt_util, libtt_tttk, libmini_isam],
  install : true,
  dependencies : [
    dep_x11, dep_xt, dep_ice, dep_sm, dep_tirpc
    # Add other external deps if needed
  ],
  include_directories : [inc, include_directories('../../..')]
)
