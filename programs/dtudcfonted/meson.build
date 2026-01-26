
# libfal
libfal_sources = [
  'libfal/_falQuarks.c', 'libfal/_falSetLocale.c', 'libfal/_fallcCT.c',
  'libfal/_fallcCharSet.c', 'libfal/_fallcConv.c', 'libfal/_fallcDB.c',
  'libfal/_fallcDefConv.c', 'libfal/_fallcDynamic.c', 'libfal/_fallcEuc.c',
  'libfal/_fallcFile.c', 'libfal/_fallcGenConv.c', 'libfal/_fallcGeneric.c',
  'libfal/_fallcInit.c', 'libfal/_fallcPrTxt.c', 'libfal/_fallcPubWrap.c',
  'libfal/_fallcPublic.c', 'libfal/_fallcRM.c', 'libfal/_fallcSjis.c',
  'libfal/_fallcStd.c', 'libfal/_fallcTxtPr.c', 'libfal/_fallcUTF.c',
  'libfal/_fallcUtil.c', 'libfal/_fallcWrap.c', 'libfal/_falomGeneric.c',
  'libfal/_falrm.c', 'libfal/_faludcInf.c', 'libfal/_falutilbitmap.c',
  'libfal/falfont.c', 'libfal/readpcf.c', 'libfal/syncx.c'
]

libfal = static_library('fal',
  libfal_sources,
  include_directories : include_directories('include', 'libfal'),
  dependencies : [dep_x11],
  c_args : ['-DANSICPP', '-DMULTIBYTE', '-DNLS16']
)

dtudcfonted_fal_dep = declare_dependency(
  link_with : libfal,
  include_directories : include_directories('include', 'libfal')
)
meson.override_dependency('dtudcfonted_fal', dtudcfonted_fal_dep)

# libfuty
libfuty_sources = [
  'libfuty/getfname.c', 'libfuty/lock.c', 'libfuty/oakfuty.c'
]

libfuty = static_library('futy',
  libfuty_sources,
  include_directories : include_directories('include', 'libfal'),
  dependencies : [dep_x11],
  c_args : ['-DANSICPP', '-DMULTIBYTE', '-DNLS16']
)

dtudcfonted_futy_dep = declare_dependency(
  link_with : libfuty,
  include_directories : include_directories('include')
)
meson.override_dependency('dtudcfonted_futy', dtudcfonted_futy_dep)

# dtudcfonted executable
dtudcfonted_sources = [
  'code.c', 'comsub.c', 'cpyw.c', 'cpyx.c', 'mtfgui.c',
  'selectx.c', 'ufont.c', 'util.c', 'xutil.c'
]

executable('dtudcfonted',
  dtudcfonted_sources,
  include_directories : include_directories('include', 'libfal', 'libfuty'),
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    dtudcfonted_fal_dep, dtudcfonted_futy_dep
  ],
  c_args : ['-DANSICPP', '-DMULTIBYTE', '-DNLS16'],
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)
