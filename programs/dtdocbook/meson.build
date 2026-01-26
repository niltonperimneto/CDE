
dtdocbook_datadir = join_paths(get_option('prefix'), get_option('datadir'), 'cde', 'dtdocbook')
dtdocbook_libexecdir = join_paths(get_option('prefix'), get_option('libexecdir'), 'cde', 'dtdocbook')

dtdocbook_c_args = [
  '-DDTDCBK_DATAROOTDIR="' + dtdocbook_datadir + '"',
  '-DINFOLIB_LIBEXECDIR="' + join_paths(dtdocbook_libexecdir, 'infolib') + '"'
]

executable('dtdocbook2infolib',
  'dtdocbook2infolib.c',
  dependencies: [libDtSvc_dep, libDtHelp_dep],
  include_directories: inc,
  c_args: dtdocbook_c_args,
  install: true
)

conf_data = configuration_data()
# Add variables needed by scripts if any?
# .in files usually need replacements.
# Let's check dtdocbook2man.in content.
# Assuming standard replacements or just install as is if no @VAR@.
# But .in usually implies processing.

configure_file(
  input: 'dtdocbook2man.in',
  output: 'dtdocbook2man',
  copy: true, 
  install: true,
  install_dir: get_option('bindir')
)

configure_file(
  input: 'dtdocbook2sdl.in',
  output: 'dtdocbook2sdl',
  copy: true,
  install: true,
  install_dir: get_option('bindir')
)

# Install subdirectories
install_subdir('dtsr', install_dir: dtdocbook_datadir)
install_subdir('infolib', install_dir: dtdocbook_libexecdir)
install_subdir('instant', install_dir: dtdocbook_datadir)
install_subdir('locales', install_dir: dtdocbook_datadir)
install_subdir('sgml', install_dir: dtdocbook_datadir)
install_subdir('spec', install_dir: dtdocbook_datadir)
install_subdir('tcl', install_dir: dtdocbook_datadir)
install_subdir('tpt', install_dir: dtdocbook_datadir)
install_subdir('doc_utils', install_dir: dtdocbook_datadir)

install_data('catalog.sh', install_dir: dtdocbook_datadir)
install_data('env.sh', install_dir: dtdocbook_datadir)
install_data('env.csh', install_dir: dtdocbook_datadir)
