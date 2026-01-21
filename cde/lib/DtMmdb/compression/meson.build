flex = find_program('flex')

sgml_C = custom_target('sgml.C',
  input : 'sgml.ll',
  output : 'sgml.C',
  command : [flex, '-8', '-s', '-Psgml', '-o', '@OUTPUT@', '@INPUT@']
)

ps_C = custom_target('ps.C',
  input : 'ps.ll',
  output : 'ps.C',
  command : [flex, '-8', '-s', '-Pps', '-o', '@OUTPUT@', '@INPUT@']
)

libcompression_sources = [
  'abs_agent.C', 'zip.C', 'huffman.C', 'trie.C', 'code.C', 'lzss.C',
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
