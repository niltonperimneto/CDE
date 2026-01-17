libmini_isam_sources = [
  'mini_isam/isadd1key.c', 'mini_isam/isaddindex.c', 'mini_isam/isaddprimary.c', 'mini_isam/isalloc.c',
  'mini_isam/isamaddindex.c', 'mini_isam/isamdelcurr.c', 'mini_isam/isamdelrec.c', 'mini_isam/isamerror.c', 'mini_isam/isamhooks.c',
  'mini_isam/isamopen.c', 'mini_isam/isamwrite.c', 'mini_isam/isapplmr.c', 'mini_isam/isapplmw.c', 'mini_isam/isbsearch.c',
  'mini_isam/isbtree.c', 'mini_isam/isbtree2.c', 'mini_isam/isbtree3.c', 'mini_isam/isbuild.c', 'mini_isam/isbytearray.c',
  'mini_isam/ischange1key.c', 'mini_isam/isclose.c', 'mini_isam/iscntl.c', 'mini_isam/iscurpos.c', 'mini_isam/isdatconv.c',
  'mini_isam/isdel1key.c', 'mini_isam/isdelcurr.c', 'mini_isam/isdelrec.c', 'mini_isam/isdiskbufs2.c', 'mini_isam/isdlink.c',
  'mini_isam/iserase.c', 'mini_isam/iserror.c', 'mini_isam/isfab.c', 'mini_isam/isfcb.c', 'mini_isam/isfcbindex.c',
  'mini_isam/isfcbwatchfd.c', 'mini_isam/isfd.c', 'mini_isam/isfileio.c', 'mini_isam/isfixrec.c', 'mini_isam/isfname.c',
  'mini_isam/isgarbage.c', 'mini_isam/isindexconv.c', 'mini_isam/isindexinfo.c', 'mini_isam/isindfreel.c',
  'mini_isam/iskeyaux.c', 'mini_isam/iskeycalc.c', 'mini_isam/iskeycmp.c', 'mini_isam/iskeyconv.c', 'mini_isam/iskeyvalid.c',
  'mini_isam/isminmax.c', 'mini_isam/ismngfcb.c', 'mini_isam/isopen.c', 'mini_isam/ispageio.c', 'mini_isam/isperm.c',
  'mini_isam/isread.c', 'mini_isam/isrename.c', 'mini_isam/isrepair.c', 'mini_isam/isrewcurr.c', 'mini_isam/isrewrec.c',
  'mini_isam/issignals.c', 'mini_isam/issort.c', 'mini_isam/isstart.c', 'mini_isam/issync.c', 'mini_isam/isvarrec.c',
  'mini_isam/isvars.c', 'mini_isam/isversion.c', 'mini_isam/iswrite.c', 'mini_isam/iswrrec.c'
]

libmini_isam = static_library('mini_isam',
  libmini_isam_sources,
  include_directories : include_directories('mini_isam'),
  pic : true)

mini_isam_inc = include_directories('mini_isam')

# subdir('lib')
subdir('libtt_shim')
# subdir('bin') # later
