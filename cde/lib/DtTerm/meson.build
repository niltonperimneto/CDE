inc_dt = include_directories('../../include/Dt')

subdir('util')
subdir('TermPrim')
subdir('Term')
subdir('TermView')

libDtTerm_sources = [] # It seems sources are empty in main lib, just linking sublibs

libDtTerm = library('DtTerm',
  libDtTerm_sources,
  include_directories : inc,
  link_whole : [libTerm, libTermPrim, libTermView],
  dependencies : [
    dep_x11, dep_xm, dep_xt, dep_xmu, dep_xext, dep_ice, dep_sm,
    libm, libDtWidget_dep, libDtSvc_dep, libDtHelp_dep
  ],
  version : '2.1.0',
  install : true
)

libDtTerm_dep = declare_dependency(
  link_with : libDtTerm,
  include_directories : include_directories('.')
)
