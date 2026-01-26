subdir('raima')

dtsearch_sources = [
  'apndext.c', 'ausdopen.c', 'ausexit.c', 'bmstrstr.c', 'boolpars.c',
  'boolsrch.c', 'cuslang.c', 'dbchange.c', 'dberr.c', 'delspace.c',
  'dtoe.c', 'dtoeinit.c', 'dtsrapi.c', 'dtsrdbrec.c', 'dtsrjoint.c',
  'dtsrswab.c', 'dtsrutil.c', 'dtsrve.c', 'endslash.c', 'fileman.c',
  'globals.c', 'hdecode.c', 'hencode.c', 'hilite.c', 'iscompat.c',
  'isduprec.c', 'jpn.c', 'lang.c', 'langmap.c', 'msgs.c', 'msgutil.c',
  'objdate.c', 'ocf.c', 'opendblk.c', 'ophuf.c', 'readchar.c',
  'strupr.c', 'userint.c', 'vedelete.c', 'vestatis.c', 'vstfunct.c'
]

# Handle boolyac.y
yacc = find_program('yacc', 'bison')
boolyac_c_h = custom_target('boolyac',
  input : 'boolyac.y',
  output : ['boolyac.c', 'boolyac.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -d @INPUT@ && ' +
    'mv y.tab.c $1 && ' +
    'mv y.tab.h $2',
    'sh', '@OUTPUT0@', '@OUTPUT1@'
  ]
)



# We need to copy the library to where meson expects it or just index it?
# Custom target outputs are tricky to link directly as libraries sometimes.
# But 'link_with' usually accepts a static library file target if created via custom_target?
# No, link_with expects a library object.
# dependency() or declare_dependency can use 'link_whole' or 'sources'.
# Actually, the best way to link a .a from custom_target is to treat it as a source file to the consumer, 
# OR use find_library if it was installed.
# But here we can use it as a link_whole argument or similar? 
# Using a custom_target in link_with might work in recent meson.
# Let's try simpler: separate custom_target to build, and find it? 
# Or simpler:
# Just define the .a as a source file to libDtSearch? 
# That works for static libs usually.

# But wait, cargo output moves around.
# Let's use a wrapper script or just command to copy the result to @OUTPUT@.

cargo_builder = custom_target('dtsearch_rs_static',
  input : ['rust/src/lib.rs', 'rust/src/format.rs', 'rust/Cargo.toml'],
  output : 'libdtsearch_rs.a',
  command : [
    'sh', '-c', 
    'cargo build --release --manifest-path @SOURCE_ROOT@/cde/lib/DtSearch/rust/Cargo.toml --target-dir @OUTDIR@/cargo_build && cp @OUTDIR@/cargo_build/release/libdtsearch_rs.a @OUTPUT@'
  ],
  build_by_default : true,
  console : true
)

libDtSearch = shared_library('DtSearch',
  dtsearch_sources + [boolyac_c_h],
  include_directories : [inc, include_directories('raima'), include_directories('../../include/Dt'), include_directories('.'), include_directories('../../..')],
  link_whole : libraima,
  dependencies : [libm, libDtSvc_dep],
  sources : [cargo_builder], 
  c_args : [
    common_c_args,
    '-DI18N_MSG', '-DMULTIBYTE'
  ],
  install : true,
  version : '2.1.0'
)

libDtSearch_dep = declare_dependency(
  link_with : libDtSearch,
  include_directories : include_directories('.')
)

meson.override_dependency('dtsearch', libDtSearch_dep)
