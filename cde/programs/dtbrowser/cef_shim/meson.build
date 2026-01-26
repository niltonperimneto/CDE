# cef_shim library

cef_shim_lib = static_library('cef_shim',
  'cef_shim.c',
  include_directories : include_directories('../include'),
  install : false
)

cef_dep = declare_dependency(
  link_with : cef_shim_lib,
  include_directories : include_directories('../include')
)
