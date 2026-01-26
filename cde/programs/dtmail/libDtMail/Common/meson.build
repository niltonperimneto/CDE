# libDtMail/Common

libdtmail_common_src = run_command('ls', '*.C', check: false).stdout().strip().split('\n')
# or list manually if ls fails (but wildcard works usually with run_command or just listing)
# Listing manually is safer for meson but tedious. I'll rely on globbing for now via finding if list is too long, 
# but for Artifact quality, I should list them.
# There are 47 files. I will list them all.

libdtmail_common_src = [
  'BigMalloc.C', 'Buffer.C', 'DtMail.C', 'ImplMap.C', 'IO.C', 'Init.C',
  'Mailbox.C', 'Message.C', 'Session.C', 'Utility.C' 
  # Truncated list? No, I need ALL sources.
  # Let me read directory content properly or use a Glob pattern if I could (meson doesn't support wildcards well)
  # I will use a simple list of assumed files or what I saw. 
  # Wait, the copy command copied *.C. I can use a script to generate the list or use `run_command` in meson (discouraged but works).
  # Or I can just trust that I copied them and list them.
]

# To avoid guessing, I will use `run_command` to get sources. 
# "ls *.C" inside the dir.

libdtmail_common = static_library('DtMailCommon',
  sources : run_command('sh', '-c', 'ls *.C', check: true).stdout().strip().split('\n'),
  include_directories : [inc, include_directories('../../include')],
  dependencies : [dep_x11, dep_xt, libDtSvc_dep, libtt_dep],
  cpp_args : common_c_args
)

libdtmail_common_dep = declare_dependency(
  link_with : libdtmail_common,
  include_directories : include_directories('.')
)
