server_src_dir = '../../../../programs/dtcm/server/'

server_sources = [
  'access.c', 'callback.c', 'cmscalendar.c', 'cmsconvert.c', 'cmsentry.c',
  'cmsmatch.c', 'delete.c', 'garbage.c', 'insert.c', 'lexit.c', 'list.c',
  'log.c', 'lookup.c', 'reclotick.c', 'recount.c', 'relasttick.c',
  'reminder.c', 'renexttick.c', 'repeat.c', 'reprevtick.c', 'rerule.c',
  'reutil.c', 'tree.c', 'utility.c', 'v4ops.c', 'v5ops.c', 'cmsfunc.c',
  'programtable.c', 'rtable2.c', 'rtable3.c', 'rtable4.c', 'svcmain.c',
  'update.c'
]

server_sources_with_path = []
foreach s : server_sources
  server_sources_with_path += server_src_dir + s
endforeach

# Parser Yacc Generation
yacc = find_program('yacc', 'bison')
parser_c_h = custom_target('parser',
  input : server_src_dir + 'parser.y',
  output : ['parser.c', 'parser.h'],
  command : [
    'sh', '-c', 
    yacc.full_path() + ' -d @INPUT@ && ' +
    'sed -e "s/yy/yyy/g" -e "/# line/d" y.tab.c > $1 && ' +
    'sed -e "s/yy/yyy/g" y.tab.h > $2 && ' +
    'rm y.tab.c y.tab.h',
    'sh', '@OUTPUT0@', '@OUTPUT1@'
  ]
)

# XDR generation
# Build native rpcgen
rpcgen_src_dir = '../../../../rpcsvc-proto/rpcgen/'
rpcgen_sources = [
  'rpc_clntout.c', 'rpc_cout.c', 'rpc_hout.c', 'rpc_main.c',
  'rpc_parse.c', 'rpc_sample.c', 'rpc_scan.c', 'rpc_svcout.c',
  'rpc_tblout.c', 'rpc_util.c'
]
rpcgen_srcs_with_path = []
foreach s : rpcgen_sources
  rpcgen_srcs_with_path += rpcgen_src_dir + s
endforeach

rpcgen_exe = executable('rpcgen',
  sources : rpcgen_srcs_with_path,
  native : true,
  include_directories : include_directories(rpcgen_src_dir),
  c_args : ['-DPACKAGE="rpcgen"', '-DVERSION="2.5.3"']
)

# XDR generation
sed = find_program('sed')

rtable_xdr_srcs = []
foreach ver : ['2', '3', '4']
  rtable_xdr_srcs += custom_target('rtable' + ver + '_xdr.c',
    input : [
      '../../../lib/csa/rtable' + ver + '.x', 
      '../../../lib/csa/namechange1.sed', 
      '../../../lib/csa/namechange2.sed'
    ],
    output : 'rtable' + ver + '_xdr.c',
    command : [
      'sh', '-c', 
      rpcgen_exe.full_path() + ' -c $1 | ' + 
      sed.full_path() + ' -f $2 -f $3 | ' +
      sed.full_path() + ' \'s|#include ".*rtable|#include "rtable|g\' > $4',
      'sh', '@INPUT0@', '@INPUT1@', '@INPUT2@', '@OUTPUT@'
    ],
    depends : rpcgen_exe
  )
endforeach

executable('rpc.cmsd',
  sources : [server_sources_with_path, parser_c_h, rtable_xdr_srcs],
  include_directories : [
    inc,
    include_directories('../../../../programs/dtcm/libDtCmP'),
    include_directories('../../../../programs/dtcm/server'),
    include_directories('.'),
    include_directories('../../../include/csa'),
    include_directories('../../../lib/csa'),
    include_directories('../../../lib/csa/rust/include')
  ],
  dependencies : [libcsa_dep, libDtSvc_dep, dep_tirpc, dep_xt],
  install : true,
  install_dir : join_paths(get_option('libexecdir'), 'dt')
)
