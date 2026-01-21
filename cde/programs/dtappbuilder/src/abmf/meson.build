dtcodegen_sources = [
  'abmf.c', 'args.c', 'cdefs.c', 'connect.c', 'create_decls.c',
  'dtb_utils.c', 'generate_code.c', 'instances.c', 'lib_func_strings.c',
  'make_file.c', 'merge_c.c', 'motifdefs.c', 'msg_cvt.c', 'msg_file.c',
  'obj_names.c', 'parse_c.c', 'proj_c_file.c', 'proj_header_file.c',
  'resource_file.c', 'stubs_c_file.c', 'ui_c_file.c', 
  'ui_header_file.c', 'utils.c', 'utils_c_file.c',
  'utils_header_file.c', 'write_c.c', 'write_code.c'
]

gencat = find_program('gencat')

dtcodegen_cat = custom_target('dtcodegen.cat',
  input : [
    'dtcodegen.msg',
    '../libAButil/libAButil.msg',
    '../libABobjXm/libABobjXm.msg',
    '../libABil/libABil.msg'
  ],
  output : 'dtcodegen.cat',
  command : [gencat, '@OUTPUT@', '@INPUT@']
)

# We need dtcodegen to be built for the build machine (native) if we are cross compiling,
# but for now we assume host == build. 
# It is used to generate code for dtbuilder.

dtcodegen = executable('dtcodegen',
  dtcodegen_sources,
  include_directories : [
    include_directories('../include'),
    include_directories('../libABil'),
    inc
  ],
  dependencies : [
    libABil_dep, libABobjXm_dep, libABobj_dep, libAButil_dep,
    dep_x11, dep_xm, dep_xt, dep_xmu, libm,
    libtt_dep, libDtWidget_dep, libDtTerm_dep, libDtHelp_dep, libDtSvc_dep
  ],
  c_args : ['-D_POSIX_SOURCE=1'],
  install : true
)
