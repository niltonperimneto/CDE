# dtdspmsg - Display Message Utility

# MsgCat.c is copied from lib/DtSvc/DtUtil2/
dtdspmsg_src = ['dtdspmsg.c', 'MsgCat.c']

executable('dtdspmsg',
  dtdspmsg_src,
  include_directories : [inc], # Needs Dt includes? MsgCat.c includes Dt/MsgCat.h
  dependencies : [
    # No X libraries per Makefile -DNO_XLIB
    # But usually CDE code needs some basic headers.
    # Makefile says: dtdspmsg_LDADD = (empty) + -lgen (solaris)
    # So it is self-contained C.
  ],
  c_args : [common_c_args, '-DNO_XLIB'],
  install : true
)
