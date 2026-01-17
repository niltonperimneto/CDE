libmini_isam_sources = [
  'isadd1key.c', 'isaddindex.c', 'isaddprimary.c', 'isalloc.c',
  'isamaddindex.c', 'isamdelcurr.c', 'isamdelrec.c', 'isamerror.c', 'isamhooks.c',
  'isamopen.c', 'isamwrite.c', 'isapplmr.c', 'isapplmw.c', 'isbsearch.c',
  'isbtree.c', 'isbtree2.c', 'isbtree3.c', 'isbuild.c', 'isbytearray.c',
  'ischange1key.c', 'isclose.c', 'iscntl.c', 'iscurpos.c', 'isdatconv.c',
  'isdel1key.c', 'isdelcurr.c', 'isdelrec.c', 'isdiskbufs2.c', 'isdlink.c',
  'iserase.c', 'iserror.c', 'isfab.c', 'isfcb.c', 'isfcbindex.c',
  'isfcbwatchfd.c', 'isfd.c', 'isfileio.c', 'isfixrec.c', 'isfname.c',
  'isgarbage.c', 'isindexconv.c', 'isindexinfo.c', 'isindfreel.c',
  'iskeyaux.c', 'iskeycalc.c', 'iskeycmp.c', 'iskeyconv.c', 'iskeyvalid.c',
  'isminmax.c', 'ismngfcb.c', 'isopen.c', 'ispageio.c', 'isperm.c',
  'isread.c', 'isrename.c', 'isrepair.c', 'isrewcurr.c', 'isrewrec.c',
  'issignals.c', 'issort.c', 'isstart.c', 'issync.c', 'isvarrec.c',
  'isvars.c', 'isversion.c', 'iswrite.c', 'iswrrec.c'
]

libmini_isam = static_library('mini_isam',
  libmini_isam_sources,
  include_directories : include_directories('.'),
  pic : true)

mini_isam_inc = include_directories('.')
