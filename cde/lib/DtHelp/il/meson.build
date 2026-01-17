il_sources = [
  'ilX.c', 'ilbigray.c', 'ilcodec.c', 'ilcontext.c', 'ilconvert.c',
  'ilcrop.c', 'ildata.c', 'ildecompg3.c', 'ildecompg4.c', 'ildecompjpeg.c',
  'ildecomplzw.c', 'ildecomppackbits.c', 'ildecompress.c', 'ildither.c',
  'ilformat.c', 'ilgraybi.c', 'ilimage.c', 'ilinvert.c', 'iljpgdecode.c',
  'iljpgdecodejif.c', 'iljpgdedct.c', 'iljpgdehuff.c', 'iljpgutil.c',
  'ilmap.c', 'ilobject.c', 'ilpalette.c', 'ilpipe.c', 'ilrwimage.c',
  'ilscale.c', 'ilscalesample.c', 'iltiff.c', 'iltiffread.c',
  'iltruegray.c', 'ilupsample.c', 'ilutiljpeg.c', 'ilycbcr.c'
]

libil = static_library('il',
  il_sources,
  include_directories : [inc, include_directories('..'), include_directories('../../../include/DtI')],
  c_args : ['-DDTLIB', '-DCDE_INSTALLATION_TOP="/usr/dt"', '-DCDE_CONFIGURATION_TOP="/etc/dt"'],
  dependencies : [dep_x11, libm, dep_jpeg]
)

libil_dep = declare_dependency(
  link_with : libil,
  include_directories : include_directories('.')
)
