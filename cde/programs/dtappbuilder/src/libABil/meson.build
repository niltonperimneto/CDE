yacc = find_program('yacc', 'bison')

bil_parse_c = custom_target('bil_parse.c',
  input : 'bil_parse.y',
  output : ['bil_parse.c', 'bil_parse.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -d @INPUT@ && mv y.tab.c $1 && mv y.tab.h $2', 
    'sh', '@OUTPUT0@', '@OUTPUT1@'
  ]
)

libABil_sources = [
  bil_parse_c,
  'abuil_load.c', 'abuil_print.c', 'abuil_resource_attr_map.c',
  'abuil_trav.c', 'abuil_widget_obj_map.c', 'bil.c', 'bil_lexer.c',
  'bil_loadact.c', 'bil_loadatt.c', 'bil_loadfile.c', 'bil_loadobj.c',
  'bil_store.c', 'gil.c', 'gil_loadact.c', 'gil_loadatt.c',
  'gil_loadfile.c', 'gil_loadobj.c', 'load.c'
]

libABil = static_library('ABil',
  libABil_sources,
  include_directories : [include_directories('../include'), inc],
  c_args : ['-D_POSIX_SOURCE=1']
)

libABil_dep = declare_dependency(
  link_with : libABil,
  include_directories : include_directories('../include')
)
