# ttdbserverd
user cmdttdbserverdRPC-based ToolTalk database serverrpc.ttdbserverd-G-mDTMOUNTPOINT_value-n-v-?
## DESCRIPTION


rpc.ttdbserverdmanages ToolTalk objects created by
&cdeman.tt.spec.create;,
and handles certain queries related to the netfiles returned by
&cdeman.tt.file.netfile;.
One instance ofrpc.ttdbserverd(normally started byinetd)
runs on each host that has a local filesystem.rpc.ttdbserverdserves four purposes:

Mapping a spec to its associated file and a file to its associated specs.

Mapping a spec to its properties.

Mapping a file to a list of sessions with clients having patterns
registered in the scope of that file.

Answering netfile queries; see
&cdeman.tt.file.netfile;
and
&cdeman.tt.host.file.netfile;.

For each filesystem thatrpc.ttdbserverdneeds to store information about, it creates a directory calledTT_DBat the mountpoint of that file system.
In that directory
it creates the databases it needs to store its tables and indices.
If the partition is not writable, thenrpc.ttdbserverdcan be told, viapartition_map(4),
to create the databases in another local partition.
Ifrpc.ttdbserverdis not installed on a particular file server, ToolTalk can be
told, viahostname_map(4),
to manage that file server's partitions using therpc.ttdbserverdon a different host.
## OPTIONS


* **-G** 

Perform garbage collection.
This cleans up theTT_DBdirectories and the
associated internal database files.
* **-mDTMOUNTPOINT_value** 

Sets the`DTMOUNTPOINT`environment variable forrpc.ttdbserverd.
If there is already an environment variable called`DTMOUNTPOINT`, -m
will override it.
* **-n** 

Turn off permission checking.  Normally the protection of the file
passed to
&cdeman.tt.spec.create;
determines who may read and write that spec.  This
option disables this checking and allows anyone to read and write any
spec.  This option should be used with caution.
* **-v** 

Print out the version number.
* **-?** 

Prints out the command usage information.

## ENVIRONMENT


* **DTMOUNTPOINT** 

If set, the value of this environment variable will be used in place
of/netin pathnames constructed to answer
&cdeman.tt.host.netfile.file;
queries. This environment variable can also be set by using the -m
flag forrpc.ttdbserverd.
* **TT_PARTITION_MAP** 

If set, the value of this environment variable will be used
in place of/etc/tt/partition_map.  Seepartition_map(4).

## FILES


* **TT_DB/*** 

spec and session database files are kept in the TT_DB directory
under each disk partition mount point.
* **tt/hostname_map** 

Host redirection map.  Seehostname_map(4).
* **/etc/tt/partition_map** 

Partition redirection map.  Seepartition_map(4).

## SEE ALSO


&cdeman.ttsession;,
&cdeman.tt.file.netfile;,
&cdeman.tt.host.file.netfile;,
&cdeman.tt.spec.create;,hostname_map(4),partition_map(4)