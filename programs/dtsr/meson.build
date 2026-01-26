
# Common defines
dtsr_c_args = ['-DMAIN_PROGRAM', '-DNO_XLIB', '-DANSICPP', '-DMULTIBYTE', '-DNLS16']
dtsearch_dep = dependency('dtsearch')

# MsgCat.c generation (similar to legacy Makefile)
# In meson we can just use the source file directly from the other directory if we can access it,
# or better yet, link to libDtSvc if it contains it. 
# The Makefile says: MsgCat.c: ${top_srcdir}/lib/DtSvc/DtUtil2/MsgCat.c
# Let's assume we can just compile that file directly or use a copy.
# Since we are in programs/dtsr, ../../lib/DtSvc/DtUtil2/MsgCat.c is accessible.
# But Meson prefers not to reach out of subprojects too much.
# Let's see if libDtSvc provides it. DtUtil2 usually is part of libDtSvc.
# If these tools link libDtSvc, they might not need recompiling MsgCat.c.
# However, the legacy makefile compiles it explicitly.
# Let's try linking libDtSvc first, as it's cleaner.
# If symbols are missing, we'll add the source.

dep_dtsvc = dependency('DtSvc', required: false)
if not dep_dtsvc.found()
  # If not found via dependency (because it's internal), find library or use declare_dependency from root.
  # We should use the internal dependency if possible.
  # Assuming DtSvc is built in cde/lib/DtSvc and accessible.
  # But scoping prevents accessing 'libDtSvc_dep' from here directly if not overridden.
  # We should override DtSvc in cde/lib/DtSvc/meson.build too if we want to be clean.
  # For now, let's try to include the source file directly as a fallback or assume it's exposed.
  # Actually, let's compile MsgCat.c locally to match Makefile behavior for now.
  dtsr_common_srcs = ['../../cde/lib/DtSvc/DtUtil2/MsgCat.c']
else
  dtsr_common_srcs = []
endif

# Wait, we can't easily access ../../cde sources if we are in programs/dtsr and root is ., 
# but ../.. from here is root. Yes.
# Meson allows accessing sources from root.

# dtsrcreate
executable('dtsrcreate',
  ['dtsrcreate.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrdbrec
executable('dtsrdbrec',
  ['dtsrdbrec.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrhan
executable('dtsrhan',
  ['dtsrhan.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrload
executable('dtsrload',
  ['dtsrload.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrindex
executable('dtsrindex',
  ['dtsrindex.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrdelete (tomita.c)
executable('dtsrdelete',
  ['tomita.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# huffcode
executable('huffcode',
  ['huffcode.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrkdump
executable('dtsrkdump',
  ['dtsrkdump.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)

# dtsrclean
executable('dtsrclean',
  ['dtsrclean.c', '../../cde/lib/DtSvc/DtUtil2/MsgCat.c'],
  dependencies : [dtsearch_dep, libm],
  c_args : dtsr_c_args,
  include_directories : include_directories('../../cde/lib/DtSearch', '../../cde/lib/DtSearch/raima', '../../cde/include/Dt', '../../cde/include'),
  install : true,
  install_dir : join_paths(get_option('prefix'), 'bin')
)
