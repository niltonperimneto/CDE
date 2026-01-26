# dtscreen - CDE Screen Saver

dtscreen_src = [
  'blank.c', 'dtscreen.c', 'flame.c', 'hopalong.c', 'hsbramp.c',
  'image.c', 'life.c', 'pyro.c', 'qix.c', 'resource.c', 'rotor.c',
  'swarm.c', 'usleep.c', 'worm.c'
]

executable('dtscreen',
  dtscreen_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc, libm
  ],
  c_args : common_c_args,
  install : true
)
