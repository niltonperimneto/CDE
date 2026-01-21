# xmbind
user cmdxmbindConfigures virtual key bindingsxmbindxmbindoptionsfile
## DESCRIPTION


xmbindis an X Window System client that configures the virtual
key bindings for Motif applications. This action is performed bymwmat its startup, so thexmbindclient is only needed whenmwmis not in use, or when you want to change bindings without restartingmwm. If a file is specified,
its contents are used as the virtual key bindings. If a file is not
specified, the file`&npzwc;.motifbind`in the user's home directory
is used. If this file is not found,xmbindloads the default
virtual key bindings, as described in &cdeman.VirtualBindings;.
### Options


* **`&minus;display`** 

This option specifies the display to use; see X(1).

## RELATED


&cdeman.VirtualBindings; andX(1).