
# Contrib directory build

# Install X11 session file
# We need to patch the Exec path to point to the correct Xsession location
sed = find_program('sed')
bindir = join_paths(get_option('prefix'), get_option('bindir'))

custom_target('cde.desktop',
  input : 'desktopentry/cde.desktop',
  output : 'cde.desktop',
  command : [
    sed, '-e', 's|/usr/dt/bin/Xsession|' + bindir + '/Xsession|g',
    '@INPUT@'
  ],
  capture : true,
  install : true,
  install_dir : join_paths(get_option('prefix'), 'share/xsessions')
)

# Note: We don't install rc/systemd/dtlogin.service here because 
# cde/programs/dtlogin/meson.build already handles it via configure_file
