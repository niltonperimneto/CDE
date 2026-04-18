yacc = find_program('yacc', 'bison')
flex = find_program('flex')

sheet_C = custom_target('sheet.cpp',
  input : 'sheet.yy',
  output : ['sheet.cpp', 'sheet.tab.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -l -d -p schema -b sheet @INPUT@ && mv sheet.tab.c $1', 
    'sh', '@OUTPUT0@'
  ]
)

token_C = custom_target('token.cpp',
  input : 'token.ll',
  output : 'token.cpp',
  command : [flex, '-8', '-s', '-Pschema', '-o', '@OUTPUT@', '@INPUT@']
)

libschema_sources = [
  'desc.cpp', 'store_desc.cpp', 'object_dict.cpp', 'stored_object_desc.cpp',
  'index_desc.cpp', 'inv_desc.cpp', 'agent_desc.cpp', 'container_desc.cpp',
  sheet_C, token_C
]

libschema = static_library('schema',
  libschema_sources,
  include_directories : [include_directories('..'), inc]
)

libschema_dep = declare_dependency(
  link_with : libschema,
  include_directories : include_directories('.')
)
