subdir('utility')
subdir('object')
subdir('compression')
subdir('index')
subdir('storage')
subdir('mgrs')
subdir('schema')
subdir('oliasdb')
subdir('api')
subdir('btree')
subdir('diskhash')
subdir('dstr')
subdir('dti_cc')
subdir('dti_excs')
subdir('dynhash')
subdir('hmphf')
subdir('misc')
subdir('StyleSheet')
subdir('HardCopy')

libDtMmdb_sources = [] # No top-level sources in Makefile.am

libDtMmdb = shared_library('DtMmdb',
  libDtMmdb_sources,
  link_whole : [
    libutility, libobject, libcompression, libindex, libstorage,
    libmgrs, libschema, liboliasdb, libapi, libbtree, libdiskhash,
    libdstr, libdti_cc, libdti_excs, libdynhash, libhmphf, libmisc,
    libStyleSheet, libHardCopy
  ],
  include_directories : [include_directories('.'), include_directories('../..')],
  dependencies : [libDtSearch_dep, libm, liblmdb],
  install : true
)

libDtMmdb_dep = declare_dependency(
  link_with : libDtMmdb,
  include_directories : include_directories('.')
)
