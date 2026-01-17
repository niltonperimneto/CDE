
rpcgen = find_program('rpcgen')
sed = find_program('sed')
yacc = find_program('yacc', 'bison')

# Reparser generation
reparser_c = custom_target('reparser_c',
  input : 'reparser.y',
  output : 'reparser.c',
  command : ['sh', '-c', '@0@ -d @1@ && sed -e s/yyparse/_DtCm_rule_parser/g -e s/yy/_DtCm_yy/g y.tab.c > @2@ && rm y.tab.c'.format(yacc.path()), '@INPUT@', '@OUTPUT@']
)

reparser_h = custom_target('reparser_h',
  input : 'reparser.y',
  output : 'reparser.h',
  depends : reparser_c, # Ensure y.tab.h exists
  command : ['sh', '-c', 'sed s/yy/_DtCm_yy/g y.tab.h > @0@ && rm y.tab.h'.format(), '@OUTPUT@']
)

# RPC generation helper command
# args: output_file, input_file
# cat EUSinclude > output
# rpcgen -l input | sed ... >> output
rpc_clnt_cmd = 'cat ' + meson.current_source_dir() + '/EUSinclude > @OUTPUT@ && ' + rpcgen.path() + ' -l @INPUT@ | ' + sed.path() + ' -f ' + meson.current_source_dir() + '/namechange1.sed | ' + sed.path() + ' -f ' + meson.current_source_dir() + '/namechange2.sed >> @OUTPUT@'

# XDR generation helper command
# cat EUSinclude > output
# rpcgen -c input | sed ... >> output
rpc_xdr_cmd = 'cat ' + meson.current_source_dir() + '/EUSinclude > @OUTPUT@ && ' + rpcgen.path() + ' -c @INPUT@ | ' + sed.path() + ' -f ' + meson.current_source_dir() + '/namechange1.sed | ' + sed.path() + ' -f ' + meson.current_source_dir() + '/namechange2.sed >> @OUTPUT@'

# Header generation helper command (for agent.h)
# cat wrapbegin > output
# rpcgen -h input | sed ... >> output
# cat wrapend >> output
agent_h_cmd = 'cat ' + meson.current_source_dir() + '/agent.wrapbegin > @OUTPUT@ && ' + rpcgen.path() + ' -h @INPUT@ | ' + sed.path() + ' -f ' + meson.current_source_dir() + '/namechange1.sed | ' + sed.path() + ' -f ' + meson.current_source_dir() + '/namechange2.sed >> @OUTPUT@ && cat ' + meson.current_source_dir() + '/agent.wrapend >> @OUTPUT@'


csa_generated = [reparser_c, reparser_h]

# List of RPC pairs (base name)
rpc_bases = ['rtable2', 'rtable3', 'rtable4', 'cm']

foreach base : rpc_bases
  csa_generated += custom_target(base + '_clnt.c',
    input : base + '.x',
    output : base + '_clnt.c',
    command : ['sh', '-c', rpc_clnt_cmd]
  )
endforeach

# XDRs (cm_xdr.c is manually listed in sources as cmxdr.c? No, Makefil says cmxdr.c is source, not generated. 
# Generated: rtable2_xdr.c, rtable3_xdr.c, rtable4_xdr.c, agent_xdr.c.
# cm.x generates cm_clnt.c but cmxdr.c is static source? Makefile says: SOURCES = ... cmxdr.c ...
# Generated srcs: agent_xdr.c cm_clnt.c ...
# So cm_xdr.c is NOT generated or named cmxdr.c in sources.
# Let's check agent_xdr.c

xdr_bases = ['rtable2', 'rtable3', 'rtable4', 'agent']

foreach base : xdr_bases
  csa_generated += custom_target(base + '_xdr.c',
    input : base + '.x',
    output : base + '_xdr.c',
    command : ['sh', '-c', rpc_xdr_cmd]
  )
endforeach

# agent.h
csa_generated += custom_target('agent.h',
  input : 'agent.x',
  output : 'agent.h',
  command : ['sh', '-c', agent_h_cmd]
)

csa_sources = [
  'agent.c', 'api.c', 'appt4.c', 'attr.c', 'calendar.c',
  'cmcbxdr.c', 'cmsdata.c', 'cmxdr.c', 'connection.c',
  'convert2-4.c', 'convert3-4.c', 'convert4-2.c', 'convert4-3.c',
  'convert4-5.c', 'convert5-4.c', 'debug.c', 'entry.c', 'free.c',
  'hash.c', 'iso8601.c', 'laccess.c', 'lutil.c', 'match.c',
  'nametbl.c', 'refree.c', 'rescan.c', 'rpccalls.c', 'table.c',
  'updateattrs.c', 'xtclient.c', 'ansi_c.c'
]

libcsa = library('csa',
  csa_sources + csa_generated,
  include_directories : [inc, include_directories('.')],
  c_args : ['-DRFC_MIME', '-DLINE_COUNT', '-DV2', '-DOW_I18N', '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DCDE_CONFIGURATION_TOP="/etc/dt"'],
  dependencies : [dep_x11, dep_tirpc, dep_xt, libDtSvc_dep], # Added dep_tirpc for rpc dependencies
  install : true,
  version : '2.1.0'
)

libcsa_dep = declare_dependency(
  link_with : libcsa,
  include_directories : include_directories('.')
)
