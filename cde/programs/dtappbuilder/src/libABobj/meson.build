libABobj_sources = [
  'obj.c', 'obj_action.c', 'obj_alloc.c', 'obj_fields.c',
  'obj_geometry.c', 'obj_init.c', 'obj_isa.c', 'obj_names_list.c',
  'obj_list.c', 'obj_lists.c', 'obj_notify.c', 'obj_release.c',
  'obj_scoped.c', 'obj_select.c', 'obj_tree.c', 'obj_utils.c',
  'trav.c', 'travP.c', 'trav_public.c', 'trav_safe.c'
]

libABobj = static_library('ABobj',
  libABobj_sources,
  include_directories : [include_directories('../include'), inc],
  c_args : ['-D_POSIX_SOURCE=1']
)

libABobj_dep = declare_dependency(
  link_with : libABobj,
  include_directories : include_directories('../include')
)
