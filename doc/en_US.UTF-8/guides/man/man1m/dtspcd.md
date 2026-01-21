# dtspcd
user cmddtspcdCDE Subprocess Control Servicedtspcd-debug-log-auth_dirdirectory-timeoutnum_minutes-mount_pointmount_point
## DESCRIPTION


The daemon for the
CDE Subprocess Control service,dtspcd, is not intended to be started directly by the user,
rather it should be started automatically by the inetd
daemon (seeinetd(1m)) in response to a CDE client
requesting a process to be started on the daemon's host.
## OPTIONS


* **`-auth_dir`directory** 

The default authentication directory is the user's
$HOME directory.
This option allows the system
administrator to use a different directory.
Note thatdirectorymust be exported to hosts wishing
to use the`dtspc`service.directory- the name of the directory to use for authentication.
* **`-timeout`minutes** 

By default, the dtspcd process will terminate if it does
not have any activity (process start or process stop)
for 10 minutes anddtspcdhas no child processes running.
To change the timeout, setminutesto the
desired number of minutes.
To force the daemon to not use a timer,
setminutesto -1.minutes- the number of minutes for the timer.
* **`-mount_point`mount_point** 

The file system's mount point is namedmount_point.For example,mount_pointcould
be "/net" or "/nfs".
The daemon sets the
environment variableDTMOUNTPOINTto the
value ofmount_point. This value
ofDTMOUNTPOINTwill override all other
definitions ofDTMOUNTPOINT.
* **`-log`** 

This option turns on logging of status information
to the file/var/dt/tmp/DTSPCD.log. The information logged
includes the name of the client host, the client's
username, error messages and the name of the file
used for authentication.
The default is to not do any logging.
* **`-debug`** 

This option turns on logging of dtspc protocol
to the file/var/dt/tmp/DTSPCD.log.The
protocol information logged includes
the name of the protocol and number of bytes
in the request.
The default is to not log the protocol.

## AUTHENTICATION


When a CDE client attempts to connect to adtspcddaemon,
the client sends the daemon its username.
The
daemon uses the username to determine the user's
home directory on the daemon's host.
The home
directory is used during authentication and it
must be readable by the daemon and writable by
the client.
Therefore, the user's home directory
on the daemon's host must be mounted to the client
host.
If the user's home directory is not readable
and the`-auth_dir`command line option is not used,
the directory/var/dt/tmpwill be used.

To use a directory other than the user's home directory
for authentication, use the`-auth_dir`command
line option.
## CONFIGURATION


Thedtspcddaemon is an Internet service that must
be registered in the file/etc/servicesas follows:dtspc      6112/tcp

and in the file/etc/inetd.confas follows:dtspc  stream  tcp  nowait  root  /usr/dt/bin/dtspcd /usr/dt/bin/dtspcd
## ENVIRONMENT VARIABLE MANAGEMENT


The CDE Subprocess Control service allows
the user and system administrator to
create files of environment variable
definitions to be placed in the
processes environment before a remote
process is started.
See &cdeman.dtspcdenv; for more information.
## OPERATING SYSTEM DEPENDENCIES


On HP-UX, the file/usr/adm/inetd.secmay be used to
control access to thedtspcddaemon.
Seeinetd.sec(4) for more information.
## FILES


* **/usr/dt/bin/dtspcd** 

The CDE Subprocess Control daemon
* **/etc/services** 

The Internet service name data base
* **/etc/inted.conf** 

The inetd configuration file
* **/etc/dt/config/dtspcdenv** 

System-wide, locally defined environment variable
definitions used when a process is executed
* **/usr/dt/config/dtspcdenv** 

System-wide, installed environment variable definitions
used when a process is executed
* **$HOME/.dt/dtspcdenv** 

User-specific environment variable definitions used
when a process is executed
* **/var/dt/tmp/DTSPCD.log** 

The dtspcd log file

## DIAGNOSTICS


Use the command line options`-log`and`-debug`(described above) to get diagnostic
information.
## SEE ALSO


inetd(1m),services(4),inetd.conf(4), &cdeman.dtspcdenv;.