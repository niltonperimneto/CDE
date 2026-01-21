yacc = find_program('yacc', 'bison')
flex = find_program('flex')

sheet_C = custom_target('sheet.C',
  input : 'sheet.yy',
  output : ['sheet.C', 'sheet.tab.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -l -d -p schema -b sheet @INPUT@ && mv sheet.tab.c $1', 
    'sh', '@OUTPUT0@'
  ]
)

token_C = custom_target('token.C',
  input : 'token.ll',
  output : 'token.C',
  command : [flex, '-8', '-s', '-Pschema', '-o', '@OUTPUT@', '@INPUT@']
)

libschema_sources = [
  'desc.C', 'store_desc.C', 'object_dict.C', 'stored_object_desc.C',
  'index_desc.C', 'inv_desc.C', 'agent_desc.C', 'container_desc.C',
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
