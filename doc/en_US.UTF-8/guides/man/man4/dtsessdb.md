dtsessiondbspecial filedtsessiondbformat and location of desktop session databaseDESCRIPTIONDuring a session checkpoint, the Session Manager creates a database
of its clients and for each client saves the values of the following
properties:CurrentDirectoryProgramRestartCommandCloneCommandDiscardCommandEnvironmentRestartStyleHintThe Session Manager also saves a client's host name and session id.
If a client's screen number is available, the Session Manager
will save the screen number.CDEnext session databases (dtsession.db)
supersede CDE1.0 session databases (described in
&cdeman.dtsessionfile;). The CDEnext Session Manager
can read CDE 1.0 session databases but not write
to them. It will write only to CDEnext session databases.

At session startup, the session manager restarts any applications
that were saved as part of the selected session. The system default set of
applications to be restored as part of the user's Initial Session can be found in
/usr/dt/config/$LANG/sys.session. This file should not be edited as
it will be unconditionally overwritten upon subsequent desktop
installations.

A system administrator may replace the set of applications that
are restored as part of the user's Initial Session by creating
a file named /etc/dt/config/$LANG/sys.session. Unlike the resource
files, this file will be used as a complete replacement for the
desktop default file, so it is valid to make a copy of the system
default file and make any necessary modifications.FORMATThe CDEnext Session Manager database format is based upon Xrm and
is as follows:! dtsession.db
!
.version: <dtsession version>
.dtsessionID: <dtsession's session ID>
!
Client.<ClientID>: <ClientID>
<ClientID>.SessionID: <client's session ID>
<ClientID>.Program:
<ClientID>.host:
<ClientID>.screen:
<ClientID>.RestartCommand.0:
<ClientID>.RestartCommand.1:
<ClientID>.RestartCommand.<n>:
<ClientID>.CloneCommand.0:
<ClientID>.CloneCommand.1:
<ClientID>.CloneCommand.<n>:
<ClientID>.DiscardCommand.0:
<ClientID>.DiscardCommand.1:
<ClientID>.DiscardCommand.<n>:
<ClientID>.Environment.0:
<ClientID>.Environment.1:
<ClientID>.Environment.<n>:
!
ProxyClient.<proxy_client_num>: <proxy_client_num>
<proxy_client_num>.screen:
<proxy_client_num>.host:
<proxy_client_num>.cmd:<ClientID> is a sequential number for the XSMP-based session clients.
<proxy_client_num> is a sequential number for the proxy or ICCCM-based
session clients.TheRestartCommand,CloneCommand,DiscardCommand, andEnvironmentproperties
are of format ARRAY8. To facilitate the storage of these properties, the
Session Manager stores them as separate resources, with appropriate suffixes
for their positions in the array.The CDEnext Session Manager includes its version number when it writes a
session database. CDE 1.0 session databases do not contain version information.The Session Manager sets only the_DT_SESSION_HINTSproperty if the started session is from a CDE 1.0 session database.
The CDEnext Window Manager uses the_DT_SESSION_HINTSproperty if it is defined. It saves this information in its state file.
The Session Manager does not store any window manager
information in its database.SEE ALSO&cdeman.dtsession;,
&cdeman.dtsessionfile;