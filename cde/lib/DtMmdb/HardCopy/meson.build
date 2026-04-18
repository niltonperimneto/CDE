libHardCopy_sources = [
  'FPset.cpp', 'HardCopyFP.cpp', 'TemplatesAutoNumber.cpp',
  'autoNumber.cpp', 'autoNumberFP.cpp'
]

libHardCopy = static_library('HardCopy',
  libHardCopy_sources,
  include_directories : [include_directories('..'), inc],
  cpp_args : ['-DEXPAND_TEMPLATES', '-DCDE_NEXT']
)

libHardCopy_dep = declare_dependency(
  link_with : libHardCopy,
  include_directories : include_directories('.')
)
