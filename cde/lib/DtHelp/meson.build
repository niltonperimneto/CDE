subdir('il')

dthelp_sources = [
  'Actions.c', 'Callbacks.c', 'Destroy.c', 'Environ_c.c',
  'FileListUtils.c', 'FileUtils.c', 'Font.c', 'Access.c', 'CvString.c',
  'GifUtils.c', 'GlobSearch.c', 'Graphics.c', 'HelpAccess.c',
  'HelpDialog.c', 'HelpQuickD.c', 'HelpUtil.c', 'Helpos.c', 'History.c',
  'HourGlass.c', 'HyperText.c', 'JpegUtils.c', 'Messages.c', 'PathArea.c',
  'Print.c', 'Resize.c', 'SetList.c', 'VolSelect.c',
  'XUICreate.c', 'XbmUtils.c',
  'CleanUp.c', 'FontAttr.c',
  'GenUtils.c', 'Obsolete.c', 'StringFuncs.c',
  'bufio.c', 'decompress.c', 'HelpXlate.c', 'FormatMarkdown.c',
  'FormatCCDF.c', 'FormatSDL.c', 'FormatMan.c', 'FormatTerm.c', 'FormatUtil.c',
  'AccessCCDF.c', 'AccessSDL.c', 'UtilSDL.c', 'Canvas.c', 'CCDFUtil.c',
  'Layout.c', 'LayoutUtil.c', 'Selection.c', 'XInterface.c', 'Format.c', 'LinkMgr.c', 'AsciiSpc.c',
  'VirtFuncs.c', 'CanvasOs.c'
]

# Build Rust static library
rust_dir = meson.current_source_dir() / 'rust'
dthelp_rust_lib = custom_target(
    'dthelp_rust',
    output: 'libdthelp_engine.a',
    command: [
        'cargo', 'build',
        '--release',
        '--manifest-path', meson.current_source_dir() / 'rust/engine/Cargo.toml',
        '&&',
        'cp', meson.current_source_dir() / 'rust/engine/target/release/libdthelp_engine.a', '@OUTPUT@'
    ],
    build_by_default: true
)

dthelp_rust_dep = declare_dependency(
    link_with: dthelp_rust_lib,
    include_directories: include_directories('rust/engine/include')
)

libDtHelp = library('DtHelp',
  dthelp_sources,
  include_directories : [inc, include_directories('il'), include_directories('../DtSvc/DtUtil2'), include_directories('../../include/DtI'), include_directories('jpeg'), include_directories('rust/engine/include')],
  dependencies : [
    dep_x11, dep_xt, dep_xm, libm,
    libDtSvc_dep, libil_dep, dep_jpeg, dep_iconv, dep_xpm, dthelp_rust_dep
  ],
  c_args : ['-DDTLIB', '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DCDE_CONFIGURATION_TOP="/etc/dt"', '-DICONV_CONST=', '-Wno-switch', '-Wno-enum-compare'],
  install : true
)

install_data('CDE.lcx', install_dir : get_option('prefix') / 'config/svc')

libDtHelp_dep = declare_dependency(
  link_with : libDtHelp,
  include_directories : include_directories('.')
)
