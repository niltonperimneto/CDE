cpp = find_program('cpp')

dtstyle_src = [
  'Main.c', 'MainWin.c', 'Font.c', 'Audio.c', 'Startup.c', 'Mouse.c',
  'SaveRestore.c', 'Backdrop.c', 'Screen.c', 'Keyboard.c',
  'ColorMain.c', 'ColorPalette.c', 'ColorFile.c', 'ColorEdit.c',
  'Resource.c', 'Protocol.c', 'Dtwm.c', 'Help.c', 'HelpCB.c',
  'I18nMain.c', 'I18nEnv.c', 'I18nUtil.c'
]

# Generated configuration files
gen_srcs = [
  'Dtstyle.src',
  'Backdrops.src',
  'Palettes.src'
]

dtstyle_gen_files = []
foreach src : gen_srcs
  target_name = src.split('.')[0]
  if target_name == 'Dtstyle'
    inst_dir = join_paths(get_option('prefix'), 'lib/X11/app-defaults')
  else
    inst_dir = join_paths(get_option('prefix'), 'share/dt', target_name.to_lower())
  endif

  dtstyle_gen_files += custom_target(
    target_name,
    input : src,
    output : target_name,
    command : [
      cpp,
      '-P', '-DXCOMM=#',
      '-DCDE_INSTALLATION_TOP=@0@'.format(get_option('prefix')),
      '@INPUT@'
    ],
    capture : true,
    install : true,
    install_dir : inst_dir
  )
endforeach

# Note: Install locations for Backdrops and Palettes might need adjustment based on CDE structure
# typically /usr/dt/backdrops and /usr/dt/palettes

executable('dtstyle',
  dtstyle_src,
  include_directories : [inc, include_directories('../dthelp')],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc, libm, libtt_dep, dep_xp, dep_xext
  ],
  c_args : [
    common_c_args,
    '-DCDE_CONFIGURATION_TOP="@0@"'.format(join_paths(get_option('prefix'), 'config')),
    '-DCDE_INSTALLATION_TOP="@0@"'.format(get_option('prefix'))
  ],
  install : true
)
