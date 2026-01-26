# dtmail - CDE Mail Client

dtmail_src = run_command('sh', '-c', 'ls *.C', check: true).stdout().strip().split('\n')
# Note: MotifApp's Main.C is linked from libMotifApp.
# dtmail_src contains the Application implementation.

executable('dtmail',
  dtmail_src,
  include_directories : [inc, include_directories('../include'), include_directories('../include/MotifApp')],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, libtt_dep, dep_tirpc,
    libMotifApp_dep, libDtMail_dep
  ],
  cpp_args : [common_c_args, '-DDTMAIL_TOOLTALK', '-DDTEDITOR', '-DRELEASE_NOTES'],
  install : true
)
