yacc = find_program('yacc', 'bison')
flex = find_program('flex')

defParser_C = custom_target('defParser.cpp',
  input : 'defParser.yy',
  output : ['defParser.cpp', 'defParser.tab.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -l -d -p defParser -b defParser @INPUT@ && mv defParser.tab.c $1', 
    'sh', '@OUTPUT0@'
  ]
)

defToken_C = custom_target('defToken.cpp',
  input : 'defToken.ll',
  output : 'defToken.cpp',
  command : [flex, '-8', '-s', '-PdefParser', '-o', '@OUTPUT@', '@INPUT@']
)

style_C = custom_target('style.cpp',
  input : 'style.yy',
  output : 'style.cpp',
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -l -d -p style -b style @INPUT@ && mv style.tab.c $1', 
    'sh', '@OUTPUT0@'
  ]
)

# Note: style.yy also produces style.tab.h but the Makefile didn't explicitly rename/keep it 
# for usage, though it likely exists. If compilation fails missing style.tab.h, check this.

tokenStyle_C = custom_target('tokenStyle.cpp',
  input : 'tokenStyle.ll',
  output : 'tokenStyle.cpp',
  command : [flex, '-8', '-s', '-Pstyle', '-o', '@OUTPUT@', '@INPUT@']
)

libStyleSheet_sources = [
  'Attribute.cpp', 'AttributeList.cpp', 'BitVector.cpp', 'DocParser.cpp',
  'Element.cpp', 'Expression.cpp', 'Feature.cpp', 'FeatureDefDictionary.cpp',
  'FeatureSet.cpp', 'FeatureValue.cpp', 'PathQualifier.cpp', 'PathTable.cpp',
  'RendererHCV.cpp', 'Resolver.cpp', 'ResolverStack.cpp', 'SSPath.cpp',
  'SSTemplates.cpp', 'StyleSheet.cpp', 'StyleSheetExceptions.cpp', 'SymTab.cpp',
  'VariableTable.cpp', 
  defParser_C, defToken_C, style_C, tokenStyle_C
]

libStyleSheet = static_library('StyleSheet',
  libStyleSheet_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DEXPAND_TEMPLATES', '-DCDE_NEXT']
)

libStyleSheet_dep = declare_dependency(
  link_with : libStyleSheet,
  include_directories : include_directories('.')
)
