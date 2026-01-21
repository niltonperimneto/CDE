TermLineData_c = custom_target('TermLineData.c',
  input : 'TermLineData.data',
  output : 'TermLineData.c',
  command : [lineToData, '-p', '_DtTerm', '-f', '@INPUT@'],
  capture : true
)

libTerm_sources = [
  'Term.c', 'TermAction.c', 'TermBuffer.c', 'TermColor.c',
  'TermEnhance.c', 'TermFunction.c', 'TermFunctionKey.c',
  TermLineData_c,
  'TermParse.c', 'TermParseTable.c', 'TermVersion.c'
]

libTerm = static_library('Term',
  libTerm_sources,
  include_directories : [inc, inc_dt, include_directories('../TermPrim'), include_directories('.')],
  dependencies : [libTermPrim_dep], # implicit dependency via includes
  c_args : ['-D_POSIX_SOURCE', '-D_XOPEN_SOURCE=500']
)

libTerm_dep = declare_dependency(
  link_with : libTerm,
  include_directories : include_directories('.')
)
