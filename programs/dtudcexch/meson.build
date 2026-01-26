
dtudcexch_sources = [
  'udcexc.c', 'udcexp.c', 'udcimp.c', 'getbdffn.c', 'excutil.c',
  'selectx.c', 'exportbdf.c', 'importbdf.c'
]

# We need the path to dtudcfonted headers, which are exposed via the dependencies
# but selectx.c and others might need explicit includes if they assume specific layout.
# The legacy Makefile included -I../dtudcfonted -I../dtudcfonted/include ...
# defined in FONTEDT_PATH1.
# The dependencies 'dtudcfonted_fal' and 'dtudcfonted_futy' provide include dirs.

executable('dtudcexch',
  dtudcexch_sources,
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    dependency('dtudcfonted_fal'),
    dependency('dtudcfonted_futy'),
    dependency('DtSvc')
  ],
  c_args : ['-DANSICPP', '-DMULTIBYTE', '-DNLS16', '-DFONTC="bdftopcf"'],
  include_directories: include_directories('../dtudcfonted', '../../cde/include', '../../include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)
