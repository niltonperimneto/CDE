libtt_tttk_sources = [
  'eclipse.C',
  'ttdesktop.C',
  'ttdtfile.C',
  'ttdtprocid.C',
  'ttmedia.C',
  'tttk.C',
  'tttk2free.C',
  'tttkmessage.C',
  'tttkpattern.C',
  'tttkutils.C',
]

libtt_tttk = static_library('tt_tttk',
  libtt_tttk_sources,
  include_directories : [inc, tt_inc, include_directories('../../../..')],
  dependencies : dep_tirpc,
  pic : true)
