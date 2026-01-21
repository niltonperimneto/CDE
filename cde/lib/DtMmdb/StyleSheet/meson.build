yacc = find_program('yacc', 'bison')
flex = find_program('flex')

defParser_C = custom_target('defParser.C',
  input : 'defParser.yy',
  output : ['defParser.C', 'defParser.tab.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -l -d -p defParser -b defParser @INPUT@ && mv defParser.tab.c $1', 
    'sh', '@OUTPUT0@'
  ]
)

defToken_C = custom_target('defToken.C',
  input : 'defToken.ll',
  output : 'defToken.C',
  command : [flex, '-8', '-s', '-PdefParser', '-o', '@OUTPUT@', '@INPUT@']
)

style_C = custom_target('style.C',
  input : 'style.yy',
  output : 'style.C',
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -l -d -p style -b style @INPUT@ && mv style.tab.c $1', 
    'sh', '@OUTPUT0@'
  ]
)

# Note: style.yy also produces style.tab.h but the Makefile didn't explicitly rename/keep it 
# for usage, though it likely exists. If compilation fails missing style.tab.h, check this.

tokenStyle_C = custom_target('tokenStyle.C',
  input : 'tokenStyle.ll',
  output : 'tokenStyle.C',
  command : [flex, '-8', '-s', '-Pstyle', '-o', '@OUTPUT@', '@INPUT@']
)

libStyleSheet_sources = [
  'Attribute.C', 'AttributeList.C', 'BitVector.C', 'DocParser.C',
  'Element.C', 'Expression.C', 'Feature.C', 'FeatureDefDictionary.C',
  'FeatureSet.C', 'FeatureValue.C', 'PathQualifier.C', 'PathTable.C',
  'RendererHCV.C', 'Resolver.C', 'ResolverStack.C', 'SSPath.C',
  'SSTemplates.C', 'StyleSheet.C', 'StyleSheetExceptions.C', 'SymTab.C',
  'VariableTable.C', 
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
