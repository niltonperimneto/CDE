# dtsessionfile
special filedtsessionfileformat and location of desktop session files
## DESCRIPTION
CDEnext session databases (dtsession.db)
supersede CDE1.0 session databases.
The CDEnext Session Manager can read CDE 1.0 session databases but not write
to them. It will write only to CDEnext session databases.
For more information see, &cdeman.dtsessiondb;.

At session startup, the session manager will restart any applications
that were saved as part of the session. The system default set of
applications to be
restored as part of the user's Initial Session can be found in
/usr/dt/config/$LANG/sys.session. This file should not be edited as
it will be unconditionally overwritten upon subsequent desktop
installations.

A system administrator may replace the set of applications that
are restored as part of the user's Initial Session by creating
a file named /etc/dt/config/$LANG/sys.session. Unlike the resource
files, this file will be used as a complete replacement for the
desktop default file, so it is valid to make a copy of the system
default file and make any necessary modifications.
## FILE FORMAT


A session file contains a line for each application to be started
during a user's Initial Session. Empty lines or lines beginning with
a "#" are ignored. Each application to be started should have a line
in the session file of the form:

dtsmcmd -cmd "application command line and options"

For example, to start the file manager with options:

dtsmcmd -cmd "/usr/dt/bin/dtfile -dir ~ -geometry +700+0"

More specifically, a session file consists of one or
more SessionLine's:

* **SessionLine** 

= Comment | SessionSpec | <empty line>
* **Comment** 

= "#" {<any character except null or newline>}
* **SessionSpec** 

= "dtsmcmd -cmd " Quote CommandSpec Quote
* **Quote** 

= """
* **CommandSpec** 

= "<application command line and options>"

Elements separated by vertical bar (|) are alternatives.
Curly braces ({...}) indicate zero or more repetitions of
the enclosed elements. Quotes ("...") are
used around literal characters.

## FILES


* **/usr/dt/config/$LANG/sys.session** 

The desktop default set of applications for the
user's Initial Session.
* **/etc/dt/config/$LANG/sys.session** 

System administrator specified set of applications for
the user's Initial Session.

## SEE ALSO


&cdeman.dtsession;