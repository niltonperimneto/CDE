libHardCopy_sources = [
  'FPset.C', 'HardCopyFP.C', 'TemplatesAutoNumber.C',
  'autoNumber.C', 'autoNumberFP.C'
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
