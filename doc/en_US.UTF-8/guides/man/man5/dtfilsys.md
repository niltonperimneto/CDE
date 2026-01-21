# dtfilsys
file formats`dtfilsys`CDE file system; directory tree structure
## DESCRIPTION


The CDE file system is divided into three parts: the root system,/usr/dt, the configuration directory,/etc/dt, and the temporary directory,/var/dt.The root system,/usr/dt, is a read-only directory.
It contains all the files
that comprise the CDE.
The configuration directory,/etc/dt, is the directory
CDE applications will search to locate configuration files that have
been modified by the system administrator.
Default versions of these
configuration files are typically located in/usr/dt. The temporary directory,/var/dt, contains temporary information or information that varies
in size and existence.
Below is a diagram of the directory tree structure for CDE./etc/dt/appconfig/appmanager/<Lang>
                   /help/<Lang>
                   /icons/<Lang>
                   /types/<Lang>
         /backdrops/<Lang>
         /config
         /palettes/<Lang>

  /usr/dt/app-defaults/<Lang>
         /appconfig/appmanager/<Lang>
                   /help/<Lang>
                   /icons/<Lang>
                   /types/<Lang>
         /bin
         /config
         /lib
        /share/backdrops/<Lang>
              /include
              /examples
              /man
              /palettes/<Lang>
 /var/dt/appconfig/appmanager

 $HOME/.dt/appmanager
          /help
          /icons
          /palettes
          /sessions
          /types

The following guidelines apply to the contents of these directories:

* **/usr/dt** 

This directory is for use by the CDE system.
Applications should never
create files in this directory although they may read and execute
them.
* **/usr/dt/app-defaults/<Lang>** 

This directory contains the localized versions of the
app-defaults files for the CDE clients.
* **/usr/dt/appconfig/appmanager/<Lang>** 

This directory contains the localized versions of the actions
files used by the Application Manager.
* **/usr/dt/appconfig/help/<Lang>** 

This directory contains the localized versions of the CDE help
volumes and help files.
* **/usr/dt/appconfig/icons/<Lang>** 

This directory contains the localized versions of the icons
used by the CDE clients.
* **/usr/dt/appconfig/types/<Lang>** 

This directory contains the localized versions of the actions
and data types used by the CDE clients.
* **/usr/dt/bin** 

This directory contains the CDE clients.
* **/usr/dt/config** 

This directory contains default configuration files.
* **/usr/dt/lib** 

This directory contains the shared libraries used by the CDE
clients.
* **/usr/dt/share** 

This directory contains CDE elements, typically ASCII files, that are
independent of the architecture.
* **/usr/dt/share/backdrops/<Lang>** 

This directory contains the localized versions of the
backdrops used by the window manager workspaces and by the
Style Manager.
* **/usr/dt/share/examples** 

This directory contains examples of CDE functionality.
* **/usr/dt/share/include** 

This directory is part of the developmewnt environment and includes
header files associated with CDE.
* **/usr/dt/share/man** 

This directory contains the man pages for the CDE clients.
* **/usr/dt/share/palettes/<Lang>** 

This directory contains the localized versions of the palettes
used by the Style Manager to determine the color scheme.
* **/etc/dt** 

This directory is specified as the CDE configuration
directory.
* **/etc/dt/appconfig** 

This directory is used to hold system-administrator defined
configuration files
* **/etc/dt/appconfig/appmanager/<Lang>** 

This directory is used to hold system-administrator defined
configuration files for the Application Manager.
* **/etc/dt/appconfig/help/<Lang>** 

This directory is used to hold system-administrator defined
configuration files for the Help system.
* **/etc/dt/appconfig/icons/<Lang>** 

This directory is used to hold system-administrator defined
configuration files for icons.
* **/etc/dt/appconfig/types/<Lang>** 

This directory is used to hold system-administrator defined
configuration files for the action and filetype database.
* **/etc/dt/backdrops/<Lang>** 

This directory contains backdrops provided by the system administrator.
* **/etc/dt/config** 

This directory is to contain modified versions of the CDE
configuration files (installed in/usr/dt/config).
* **/etc/dt/palettes/<Lang>** 

This directory contains palettes provided by the system administrator.
* **/var/dt** 

This directory is used by CDE for some log files, temporary files,
and files that vary in size.
* **/var/dt/appconfig/appmanager** 

This directory is created at login and is used by the
Application Manager.
* **$HOME/.dt** 

This directory contains information specific to the user.
* **$HOME/.dt/appmanager** 

This directory is used to hold user-defined
configuration files for the Application Manager.
* **$HOME/.dt/backdrops** 

This directory is used to hold user-customized backdrops for
the Style Manager.
* **$HOME/.dt/help** 

This directory is used to hold user-defined
configuration files for the Help system.
* **$HOME/.dt/icons** 

This directory is used to hold user-defined
configuration files for icons.
* **$HOME/.dt/palettes** 

This directory is used to hold user-customized palettes for
the Style Manager.
* **$HOME/.dt/sessions** 

This directory is used to store session information between CDE
sessions.
* **$HOME/.dt/tmp** 

This directory is used to hold temporary information.
* **$HOME/.dt/types** 

This directory is used to hold user-defined
configuration files for the action and filetypes database.

### The Minimum Runtime Environment


The minimum environment required for a system to support CDE consists
of the following files and directories:/usr/dt/app-defaults/C/
tpad
Dtksh
Dtterm
/usr/dt/appconfig/tttypes/
types.xdr
/usr/dt/bin/
dsdm
dtaction
dtappgather
dtappintegrate
dtexec
dtksh
dtpad
dtspcd
dtterm
suid_exec
rpc.ttdbserver
tt_type_comp
ttsession
## SEE ALSO


&cdeman.dtenvvar;