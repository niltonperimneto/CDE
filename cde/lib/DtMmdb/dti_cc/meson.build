libdti_cc_sources = [
  'CC_Dlist.C', 'CC_Listbase.C', 'CC_Slist.C', 'CC_Stack.C',
  'CC_String.C', 'CC_Tokenizer.C', 'cc_exceptions.C', 'cc_hdict.C',
  'cc_povec.C', 'cc_pvect.C', 'cc_vvect.C', 'hdictinit.C', 'vectinit.C'
]

libdti_cc = static_library('dti_cc',
  libdti_cc_sources,
  include_directories : [include_directories('..'), inc]
)

libdti_cc_dep = declare_dependency(
  link_with : libdti_cc,
  include_directories : include_directories('.')
)
