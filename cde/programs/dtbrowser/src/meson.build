
dtbrowser_inc = include_directories('.', '../include', '../ipc', '../server')

# libXmCefBrowser (Static Widget Library)
libxmcef_src = ['XmCefBrowser.c']

libxmcef = static_library('XmCefBrowser',
  libxmcef_src,
  include_directories : [inc, dtbrowser_inc],
  dependencies : [dep_x11, dep_xt, dep_xm, cef_dep],
  cpp_args : common_c_args
)

# dtbrowser Application
dtbrowser_src = ['main.c', 'TtHandler.c']

executable('dtbrowser',
  dtbrowser_src,
  include_directories : [inc, dtbrowser_inc],
  link_with : libxmcef,
  dependencies : [dep_x11, dep_xt, dep_xm, cef_dep, libtt_dep, dep_tirpc, dtbrowser_ipc_dep, dtbrowser_server_dep],
  cpp_args : common_c_args,
  install : true
)
