project('cde', ['c', 'cpp'],
  version : '2.5.3',
  default_options : ['warning_level=1', 'c_std=gnu23', 'cpp_std=gnu++98'])

cc = meson.get_compiler('c')

# Global defines from configure.ac
add_project_arguments('-DANSICPP', '-DMULTIBYTE', '-DNLS16', '-D_GNU_SOURCE', '-DTT_VERSION_STRING="2.5.3"', '-DHAVE_POLL_H', language : ['c', 'cpp'])

common_c_args = []

# Include directories (adjusted for root)
inc = include_directories('cde/include', 'cde')

# Dependencies
libm = cc.find_library('m', required : false)
libdl = cc.find_library('dl', required : false)
libcrypt = cc.find_library('crypt', required : false)
liblmdb = cc.find_library('lmdb', required : true)

# X11 Dependencies
dep_x11 = dependency('x11')
dep_xt = dependency('xt')
dep_xm = dependency('motif', required : false)
if not dep_xm.found()
  dep_xm = cc.find_library('Xm', required : true, dirs : ['/usr/lib', '/usr/local/lib', '/usr/X11R6/lib', '/usr/lib/x86_64-linux-gnu'])
endif
dep_xext = dependency('xext')
dep_xmu = dependency('xmu')
dep_ice = dependency('ice')
dep_sm = dependency('sm')
dep_xau = dependency('xau')
dep_xpm = dependency('xpm', required : false)
dep_xinerama = dependency('xinerama', required : false)
if dep_xinerama.found()
  add_project_arguments('-DUSE_XINERAMA', language : ['c', 'cpp'])
endif

dep_tirpc = dependency('libtirpc', required : false)

dep_xrender = dependency('xrender', required : false)
if dep_xrender.found()
  add_project_arguments('-DUSE_XRENDER', language : ['c', 'cpp'])
endif

dep_xp = dependency('xp', required : false)

# Image and I18N Dependencies
dep_jpeg = dependency('libjpeg', required : false)
if not dep_jpeg.found()
  dep_jpeg = cc.find_library('jpeg', required : false)
endif

dep_iconv = dependency('iconv')

# Version configuration
ver_arr = meson.project_version().split('.')
conf_data = configuration_data()
conf_data.set('CDE_VERSION_MAJOR', ver_arr[0])
conf_data.set('CDE_VERSION_MINOR', ver_arr[1])
conf_data.set('CDE_VERSION_MICRO', ver_arr[2])
conf_data.set('CDE_VERSION_DEV', '') 

# Configure cde_config.h
if host_machine.endian() == 'big'
  conf_data.set('WORDS_BIGENDIAN', 1)
endif
configure_file(output : 'cde_config.h', configuration : conf_data)

# Recurse into subdirectories
subdir('cde')
subdir('programs')
