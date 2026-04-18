libdti_cc_sources = [
  'CC_Dlist.cpp', 'CC_Listbase.cpp', 'CC_Slist.cpp', 'CC_Stack.cpp',
  'CC_String.cpp', 'CC_Tokenizer.cpp', 'cc_exceptions.cpp', 'cc_hdict.cpp',
  'cc_povec.cpp', 'cc_pvect.cpp', 'cc_vvect.cpp', 'hdictinit.cpp', 'vectinit.cpp'
]

libdti_cc = static_library('dti_cc',
  libdti_cc_sources,
  include_directories : [include_directories('..'), inc]
)

libdti_cc_dep = declare_dependency(
  link_with : libdti_cc,
  include_directories : include_directories('.')
)
