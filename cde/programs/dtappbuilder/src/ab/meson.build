# Note: dtbuilder.c is listed as a source in Makefile.am but also has a rule to copy from .src
# We should check if dtbuilder.c exists or is dtbuilder.c.src

dtbuilder_c = configure_file(
  input : 'dtbuilder.c.src',
  output : 'dtbuilder.c',
  copy : true
)

dtbuilder_msg = configure_file(
  input : 'dtbuilder.msg.src',
  output : 'dtbuilder.msg',
  copy : true
)

 bil_files = [
  'about_box', 'appfw', 'attch_ed', 'brws', 'button', 'cgen_env',
  'cgen_props', 'cgen_win', 'choice', 'color_chooser', 'combobox',
  'conn', 'cpanel', 'custdlg', 'dnd_ed', 'drawp', 'fchooser',
  'group', 'help_ed', 'label', 'list', 'mainwin', 'menubar', 'menu',
  'message_ed', 'palette', 'panedwin_ed', 'proj', 'revolv', 'scale',
  'sep', 'spinbox', 'termp', 'textf', 'textp'
]

generated_sources = []

# Assuming dtcodegen is built and available locally or in path.
# Since we are building it in abmf, we can access it via the executable object if in same meson project,
# but since they are in different directories and potentially built separately or native vs cross...
# For now, we assume native build and use the executable target.

dtbuilder_bip = configure_file(
  input : 'dtbuilder.bip',
  output : 'dtbuilder.bip',
  copy : true
)

dep_uil = cc.find_library('Uil', required : true)
dep_mrm = cc.find_library('Mrm', required : true)


# Construct LD_LIBRARY_PATH for dtcodegen
ld_lib_path = meson.project_build_root() + '/lib/tt/libtt_shim:' + \
              meson.project_build_root() + '/lib/DtWidget:' + \
              meson.project_build_root() + '/lib/DtTerm:' + \
              meson.project_build_root() + '/lib/DtSvc:' + \
              meson.project_build_root() + '/lib/DtHelp'

foreach bil : bil_files
  # Copy .bil file to build directory
  bil_file = configure_file(
    input : bil + '.bil',
    output : bil + '.bil',
    copy : true
  )

  # Output files: name_ui.c, name_ui.h, name_stubs.c
  
  gen = custom_target(bil + '_gen',
    input : [bil_file, dtbuilder_bip],
    output : [bil + '_ui.c', bil + '_ui.h', bil + '_stubs.c'],
    command : [
      'sh', '-c',
      'cp ' + meson.current_source_dir() + '/' + bil + '_stubs.c ' + meson.current_build_dir() + '/ && ' +
      'chmod u+w ' + meson.current_build_dir() + '/' + bil + '_stubs.c && ' +
      '[ -f ' + meson.current_source_dir() + '/' + bil + '_ui.c ] && cp ' + meson.current_source_dir() + '/' + bil + '_ui.c ' + meson.current_build_dir() + '/ || true && ' +
      '[ -f ' + meson.current_source_dir() + '/' + bil + '_ui.h ] && cp ' + meson.current_source_dir() + '/' + bil + '_ui.h ' + meson.current_build_dir() + '/ || true && ' +
      'export LD_LIBRARY_PATH=' + ld_lib_path + ':$LD_LIBRARY_PATH && ' +
      'cd ' + meson.current_build_dir() + ' && ' + dtcodegen.full_path() + ' -changed -merge -p dtbuilder.bip ' + bil + '.bil && ' +
      '(if [ "' + bil + '" = "panedwin_ed" ]; then ' +
      'sed -i "s/ui_set_active(.*width_field_label/\\/\\/ &/" ' + bil + '_stubs.c; ' +
      'sed -i "s/ui_set_active(.*height_field_label/\\/\\/ &/" ' + bil + '_stubs.c; ' +
      'sed -i "s/pw_cgen->width_field_label/NULL/g" ' + bil + '_stubs.c; ' +
      'sed -i "s/dtb_panedwin_ed_dialog.width_field_label/NULL/g" ' + bil + '_stubs.c; ' +
      'sed -i "s/pw_cgen->height_field_label/NULL/g" ' + bil + '_stubs.c; ' +
      'sed -i "s/dtb_panedwin_ed_dialog.height_field_label/NULL/g" ' + bil + '_stubs.c; ' +
      'sed -i "s/pw_cgen->geo_cb/NULL/g" ' + bil + '_stubs.c; ' +
      'sed -i "s/pw_cgen->geo_cb/NULL/g" ' + bil + '_stubs.c; ' +
      'sed -i "s/dtb_panedwin_ed_dialog.geo_cb/NULL/g" ' + bil + '_stubs.c; fi)'
    ],
    depends: dtcodegen
  )
  generated_sources += [gen[0], gen[1], gen[2]] # .c and .h files
endforeach

# Main generation
# dtbuilder.h + dtb_utils.c + dtb_utils.h: dtbuilder.bip dtbuilder.c dtbuilder.msg
# command: $(DTCODEGEN) -changed -merge -p dtbuilder.bip -main

main_gen = custom_target('main_gen',
  input : [dtbuilder_bip, dtbuilder_msg],
  output : ['dtb_utils.c', 'dtb_utils.h', 'dtbuilder.h'],
  command : [
    'sh', '-c',
    'cp ' + meson.current_source_dir() + '/dtbuilder.h ' + meson.current_build_dir() + '/ && ' +
    'chmod u+w ' + meson.current_build_dir() + '/dtbuilder.h && ' +
    'export LD_LIBRARY_PATH=' + ld_lib_path + ':$LD_LIBRARY_PATH && ' +
    'cd ' + meson.current_build_dir() + ' && ' + dtcodegen.full_path() + ' -changed -merge -p dtbuilder.bip -main'
  ],
  depends: dtcodegen
)
generated_sources += [main_gen[0], main_gen[1], main_gen[2]]

# Manual sources from Makefile.am (excluding generated ones)
dtbuilder_sources = [
  'ab_bil.c', 'ab_dnd.c', 'ab_globals.c', 'ab_utils.c',
  'abobj_align.c', 'abobj_edit.c', 'abobj_events.c', 'abobj_layers.c',
  'abobj_layout.c', 'abobj_list.c', 'abobj_menu.c', 'abobj_move.c',
  'abobj_resize.c', 'abobj_select.c', 'abobj_set.c', 'abobj_util.c',
  'brws.c', 'brws_find.c', 'brws_mthds.c', 'brws_utils.c',
  'cgen_utils.c', 'conn_drag.c', 'conn_interpret.c', 'conn_obj.c',
  'pal.c', 'pal_button.c', 'pal_choice.c', 'pal_combobox.c',
  'pal_cpanel.c', 'pal_create.c', 'pal_custdlg.c', 'pal_drawp.c',
  'pal_fchooser.c', 'pal_group.c', 'pal_label.c', 'pal_list.c',
  'pal_mainwin.c', 'pal_menu.c', 'pal_menubar.c', 'pal_panedwin.c',
  'pal_scale.c', 'pal_sep.c', 'pal_spinbox.c', 'pal_termp.c',
  'pal_textf.c', 'pal_textp.c', 'proj.c', 'projP_utils.c',
  'proj_utils.c', 'prop.c', 'prop_items.c', 'tmode.c', 'tmodeP.c',
  'ui_list.c', 'ui_msg.c', 'ui_util.c', 'vwr.c', 'x_util.c',
   # And the main dtbuilder.c which is copied/generated?
   # Makefile says: dtbuilder.c: dtbuilder.c.src -> cp
   # We can just use dtbuilder.c if it exists or create a custom target if it is .src
]


# Include directories
ab_inc = [
  include_directories('../include'),
  include_directories('../include/ab_private'),
  include_directories('../libABil'),
  include_directories('.'), # for generated headers
  inc
]

dtbuilder = executable('dtbuilder',
  dtbuilder_sources + generated_sources + [dtbuilder_c],
  include_directories : ab_inc,
  dependencies : [
    libABil_dep, libABobjXm_dep, libABobj_dep, libAButil_dep,
    dep_x11, dep_xm, dep_xt, dep_xmu, libm,
    libtt_dep, libDtWidget_dep, libDtTerm_dep, libDtHelp_dep, libDtSvc_dep,
    dep_ice, dep_sm, dep_xext, dep_xpm, dep_uil, dep_mrm
  ],
  c_args : ['-D_POSIX_SOURCE=1', '-DPIXMAP_WORKAROUND'],
  install : true
)
