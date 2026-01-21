libTermPrim_sources = [
  'TermPrim.c', 'TermPrimAction.c', 'TermPrimBuffer.c',
  'TermPrimBufferWc.c', 'TermPrimCursor.c', 'TermPrimDebug.c',
  'TermPrimFunction.c', 'TermPrimLineDraw.c', 'TermPrimParse.c',
  'TermPrimParseTable.c', 'TermPrimParser.c', 'TermPrimPendingText.c',
  'TermPrimRender.c', 'TermPrimRenderMb.c', 'TermPrimRenderFont.c',
  'TermPrimRenderFontSet.c', 'TermPrimRenderLineDraw.c',
  'TermPrimRepType.c', 'TermPrimScroll.c', 'TermPrimSelect.c',
  'TermPrimSetPty.c', 'TermPrimSetUtmp.c', 'TermPrimSubproc.c',
  'TermPrimUtil.c', 'TermPrimVersion.c', 'TermPrimWMProtocols.c'
]

if host_machine.system() == 'sunos'
  libTermPrim_sources += ['TermPrimGetPty-svr4.c']
else
  libTermPrim_sources += ['TermPrimGetPty-pts.c']
endif

libTermPrim = static_library('TermPrim',
  libTermPrim_sources,
  include_directories : [inc, inc_dt, include_directories('.')],
  c_args : ['-D_POSIX_SOURCE', '-D_XOPEN_SOURCE=500']
)

libTermPrim_dep = declare_dependency(
  link_with : libTermPrim,
  include_directories : include_directories('.')
)
