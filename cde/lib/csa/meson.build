
# Rust XDR Library Build
# calling cargo build to generate liblibcsa_xdr.a and headers
rust_dir = meson.current_source_dir() / 'rust'

libcsa_xdr_target = custom_target('libcsa_xdr',
  output : 'liblibcsa_xdr.a',
  # Use sh -c to chain commands: build via cargo, then copy artifact to @OUTPUT@
  # We assume rust_dir / Cargo.toml exists.
  command : ['sh', '-c', 'cargo build --release --manifest-path @0@/Cargo.toml && cp @0@/target/release/liblibcsa_xdr.a @OUTPUT@'.format(rust_dir)],
  build_by_default : true
)

libcsa_xdr_inc = include_directories('rust/include')

# Reparser generation (yacc)
yacc = find_program('yacc', 'bison')
sed = find_program('sed')

reparser = custom_target('reparser',
  input : 'reparser.y',
  output : ['reparser.c', 'reparser.h'],
  command : ['sh', '-c', '@0@ -y -d @INPUT@ && sed -e s/yyparse/_DtCm_rule_parser/g -e s/yy/_DtCm_yy/g y.tab.c > @OUTPUT0@ && sed s/yy/_DtCm_yy/g y.tab.h > @OUTPUT1@'.format(yacc.full_path())]
)

csa_generated = [reparser]

csa_sources = [
  'agent.c', 'api.c', 'appt4.c', 'attr.c', 'calendar.c',
  'cmcbxdr.c', 'cmsdata.c', 'cmxdr.c', 'connection.c',
  'convert2-4.c', 'convert3-4.c', 'convert4-2.c', 'convert4-3.c',
  'convert4-5.c', 'convert5-4.c', 'debug.c', 'entry.c', 'free.c',
  'hash.c', 'iso8601.c', 'laccess.c', 'lutil.c', 'match.c',
  'nametbl.c', 'refree.c', 'rescan.c', 'rpccalls.c', 'table.c',
  'updateattrs.c', 'xtclient.c', 'ansi_c.c'
]

# Link with Rust static lib
# We treat the .a file as an object/source or link_args + dependency
libcsa = library('csa',
  csa_sources + csa_generated + [libcsa_xdr_target],
  include_directories : [inc, include_directories('.'), libcsa_xdr_inc, include_directories('../../include/csa')],
  c_args : ['-DRFC_MIME', '-DLINE_COUNT', '-DV2', '-DOW_I18N', '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DCDE_CONFIGURATION_TOP="/etc/dt"'],
  dependencies : [dep_x11, dep_xt, libDtSvc_dep, dep_tirpc],
  install : true,
  version : '2.1.0'
)

libcsa_dep = declare_dependency(
  link_with : libcsa,
  include_directories : include_directories('.')
)
