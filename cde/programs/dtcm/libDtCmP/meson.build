libdtcomp_src_dir = '../../../../programs/dtcm/libDtCmP/'

libdtcomp_sources = [
  'cm_tty.c', 'cmfns.c', 'dtfns.c', 'props.c', 
  'resource.c', 'timeops.c', 'util.c'
]

libdtcomp_srcs = []
foreach s : libdtcomp_sources
  libdtcomp_srcs += libdtcomp_src_dir + s
endforeach

yacc = find_program('yacc', 'bison')

# Custom target for getdate.y to generate both header and source
getdate_gen = custom_target('getdate',
  input : libdtcomp_src_dir + 'getdate.y',
  output : ['getdate.c', 'getdate.h'],
  command : [yacc, '-d', '@INPUT@', '-o', '@OUTPUT0@']
)

# Move header to output path if needed or just use it
# If generator doesn't support multiple outputs cleanly in older meson, custom_target is better.

libdtcomp = static_library('DtCmP',
  [libdtcomp_srcs, getdate_gen],
  include_directories : [
      inc, 
      inc_dtcm_include, 
      include_directories('../../../lib/csa'), # For iso8601.h
      include_directories('../../../include/csa'), # For csa.h
      include_directories('../../../include')      # standard includes
  ],
  c_args : ['-DANSICPP'],
  install : false
)

libDtCmP_dep = declare_dependency(
  link_with : libdtcomp,
  include_directories : [include_directories('.')]
)
