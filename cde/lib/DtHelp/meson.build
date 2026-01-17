subdir('il')

dthelp_sources = [
  'Actions.c', 'AsciiSpc.c', 'Callbacks.c', 'Destroy.c', 'Environ_c.c',
  'FileListUtils.c', 'FileUtils.c', 'Font.c', 'Format.c', 'FormatMan.c',
  'FormatTerm.c', 'GifUtils.c', 'GlobSearch.c', 'Graphics.c', 'HelpAccess.c',
  'HelpDialog.c', 'HelpQuickD.c', 'HelpUtil.c', 'Helpos.c', 'History.c',
  'HourGlass.c', 'HyperText.c', 'JpegUtils.c', 'Messages.c', 'PathArea.c',
  'Print.c', 'Resize.c', 'SetList.c', 'VolSelect.c', 'XInterface.c',
  'XUICreate.c', 'XbmUtils.c', 'Access.c', 'AccessCCDF.c', 'AccessSDL.c',
  'CCDFUtil.c', 'CleanUp.c', 'FontAttr.c', 'FormatCCDF.c', 'FormatSDL.c',
  'FormatUtil.c', 'GenUtils.c', 'Obsolete.c', 'StringFuncs.c', 'UtilSDL.c',
  'bufio.c', 'decompress.c', 'HelpXlate.c', 'Canvas.c', 'CanvasOs.c',
  'CvString.c', 'Layout.c', 'LayoutUtil.c', 'LinkMgr.c', 'Selection.c',
  'VirtFuncs.c'
]

libDtHelp = library('DtHelp',
  dthelp_sources,
  include_directories : [inc, include_directories('il'), include_directories('../DtSvc/DtUtil2'), include_directories('../../include/DtI'), include_directories('jpeg')],
  dependencies : [
    dep_x11, dep_xt, dep_xm, libm,
    libDtSvc_dep, libil_dep, dep_jpeg, dep_iconv, dep_xpm
  ],
  c_args : ['-DDTLIB', '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DCDE_CONFIGURATION_TOP="/etc/dt"', '-DICONV_CONST='],
  install : true
)

install_data('CDE.lcx', install_dir : get_option('prefix') / 'config/svc')

libDtHelp_dep = declare_dependency(
  link_with : libDtHelp,
  include_directories : include_directories('.')
)
