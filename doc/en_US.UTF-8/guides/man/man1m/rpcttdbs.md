# ttdbserver
user cmdttdbserverRPC-based ToolTalk database serverrpc.ttdbserver-G-m  DTMOUNTPOINT_value-n-v-?
## DESCRIPTION


rpc.ttdbserveris the ToolTalk database server daemon. This process (normally
started by`inetd`) performs all database operations for databases
stored on the host.
## OPTIONS


* **-G** 

Perform garbage collection.
This cleans up the TT_DB directories and the associated internal
database files.
* **-m DTMOUNTPOINT_value** 

Sets the DTMOUNTPOINT environment variable for`rpc.ttdbserver.`If there is already an environment variable called DTMOUNTPOINT, -m
will override it.
* **-n** 

Turn off permission checking. Normally the protection of the file
containing the spec determines who may read and write that spec. This
option disables this checking and allows anyone to read and write any
spec. This option should be used with caution.
* **-v** 

Print out the version number.
* **-?** 

Prints out the command usage information.

## ENVIRONMENT


* **DTMOUNTPOINT** 

If set, the value of this environment variable will be used in place
of "/net" in pathnames constructed for the automounters host map.
This environment variable can also be set by using the -m flag for`rpc.ttdbserver.`

## FILES


* **TT_DB/*** 

The NetISAM database files are kept in the TT_DB directory under each
disk partition mount point.

## SEE


&cdeman.ttsession; &cdeman.tt.file.netfile;