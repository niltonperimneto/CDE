flex = find_program('flex')

sgml_C = custom_target('sgml.cpp',
  input : 'sgml.ll',
  output : 'sgml.cpp',
  command : [flex, '-8', '-s', '-Psgml', '-o', '@OUTPUT@', '@INPUT@']
)

ps_C = custom_target('ps.cpp',
  input : 'ps.ll',
  output : 'ps.cpp',
  command : [flex, '-8', '-s', '-Pps', '-o', '@OUTPUT@', '@INPUT@']
)

libcompression_sources = [
  'abs_agent.cpp', 'zip.cpp', 'huffman.cpp', 'trie.cpp', 'code.cpp', 'lzss.cpp',
  sgml_C, ps_C
]

libcompression = static_library('compression',
  libcompression_sources,
  include_directories : [include_directories('..'), inc]
)

libcompression_dep = declare_dependency(
  link_with : libcompression,
  include_directories : include_directories('.')
)
