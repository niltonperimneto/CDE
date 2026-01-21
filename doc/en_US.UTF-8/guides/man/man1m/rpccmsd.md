# rpc.cmsd
user cmdrpc.cmsdcalendar manager service daemonrpc.cmsd
[-d]-s
## DESCRIPTION


Therpc.cmsdis a small database manager for appointment and
resource-scheduling data. Its primary client isdtcma productivity tool included with CDE.rpc.cmsdis normally invoked byinetd(1m) when adtcmrequest is received.
## OPTIONS


The following options are available:

* **-d** 

Enables debugging output.
* **-s** 

Runs rpc.cmsd in the foreground. This option should
be used when rpc.cmsd is invoked manually for debugging purposes.

## OPERANDS


None.
## STDIN


Not used.
## INPUT FILES


The database format used by therpc.cmsdis private.
## ENVIRONMENT VARIABLES


None.
## RESOURCES


None.
## ASYNCHRONOUS EVENTS


None.
## STDOUT


When the`-d`option is used,rpc.cmsdwrites the debugging information to stdout.
## STDOUT


None.
## OUTPUT FILES


None.
## EXTENDED DESCRIPTION


None.
## EXIT STATUS


When the process exits,rpc.cmsdexits with the status of the exited child.
Otherwise, the following exit values are returned:

* **0** 

Normal termination.
A zero exit status meansrpc.cmsdhas received a HUP signal, and terminated normally.
* **-1** 

Abnormal termination. Permissions on the database file or
directory were incorrect.
* **1** 

RPC memory management errors have prevented successful operation,
a socket operation failed in a nonrecoverable way, or could not create
a udp service.
* **2** 

Could not create a TLI connection.
* **3** 

Could not register the service.

## CONSEQUENCES OF ERRORS


Therpc.cmsdutility takes the standard action for all signals.
## APPLICATION USAGE


To runrpc.cmsdin standalone mode, the user must be root.
## EXAMPLES


None.
## SEE ALSO


&cdeman.dtcm;, &cdeman.dtcm.lookup;, &cdeman.dtcm.delete;, &cdeman.dtcm.insert;,inetd(1m).