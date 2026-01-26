# CDE Types (Action Definitions)

# List of source files to convert
dt_srcs = [
  'autoStart.dt.src', 'compat.dt.src', 'datatypes.dt.src', 'develop.dt.src',
  'dtappman.dt.src', 'dt.dt.src', 'dtfile.dt.src', 'dthelp.dt.src',
  'dthelpprint.dt.src', 'dthelptag.dt.src', 'dtims.dt.src', 'dtinfo.dt.src',
  'dtpad.dt.src', 'dtscreen.dt.src', 'dt.softinit.src', 'dtwm.fp.src',
  'error.ds.src', 'hp.dt.src', 'ibm.dt.src', 'IconBrowse.dt.src',
  'laserrom.dt.src', 'miscActions.dt.src', 'miscImages.dt.src', 'print.dt.src',
  'printerNN.dt.src', 'sunOW.dt.src', 'unsupported.dt.src', 'user-prefs.dt.src',
  'uxstd.dt.src', 'xclients.dt.src'
]

# Simple .dt files that don't need processing (already .dt)
static_dts = [
  'DisplayImage.dt', 'DisplayPDF.dt', 'DisplayPS.dt', 'DisplayVideo.dt',
  'Education.dt', 'Games.dt', 'Graphics.dt', 'Internet.dt',
  'Media_Tools.dt', 'Office.dt', 'System.dt', 'TeX.dt'
]

# Defines: CDE_INSTALLATION_TOP, CDE_CONFIGURATION_TOP, KORNSHELL, DTKORNSHELL
# Using sed substitution again because 'cpp' might vary in behavior across platforms regarding comments.
# Makefile uses GENCPP which is cpp + args. 
# However, .dt files often contain C-like comments which cpp handles well.
# Let's use `sed` as it's safer for simple variable replacement if logic allows, 
# but .dt files might use #ifdef. 
# Checking one file...
# meson's `custom_target` with `cpp` command is fine.

cpp = find_program('cpp')

# Common CPP flags
cpp_args = [
  '-P', '-C', # preserve comments? Makefile says -C.
  '-D', 'XCOMM=#',
  '-D', 'CDE_INSTALLATION_TOP=/usr/dt',
  '-D', 'CDE_CONFIGURATION_TOP=/etc/dt',
  '-D', 'KORNSHELL=/usr/dt/bin/dtksh',
  '-D', 'DTKORNSHELL=/usr/dt/bin/dtksh'
]

foreach src : dt_srcs
  target_name = src.split('.src')[0]
  custom_target(target_name,
    input : src,
    output : target_name,
    command : [cpp] + cpp_args + ['@INPUT@'],
    capture : true,
    install : true,
    install_dir : join_paths(get_option('prefix'), 'appconfig/types/C') # Default locale
  )
endforeach

install_data(static_dts,
  install_dir : join_paths(get_option('prefix'), 'appconfig/types/C')
)

install_data(['actions.ddf', 'actions.fs'],
  install_dir : join_paths(get_option('prefix'), 'appconfig/types/C')
)
