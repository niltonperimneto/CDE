# dtappgather
user cmddtappgathergather application files for presentation by the
Application Managerdtappgather-r
## DESCRIPTION


The
Application Manager
presents personal, system, and factory applications in
an easy-to-access window.
The
Application Manager
is implemented as a special File Manager view of a
subdirectory that is built on a per-user, per-session basis.
Thedtappgatherutility is responsible for creating and refreshing
the user's
Application Manager
subdirectory.

Thedtappgatherutility
is run at login and whenever the user invokes the`ReloadApps`action.
The user's individual subdirectory for the
Application Manager,/var/dt/appconfig/appmanager/$DTUSERSESSION, is opened when the user presses the Application Manager control
on the Front Panel.

The sequence of events is as follows:

When the user logs in, theXsessionscript sources in the`Xsession.d`script that sets the`DTUSERSESSION`environment
variable.

Xsessionthen invokes thedtsearchpathutility to set the`DTAPPSEARCHPATH`environment variable.

Afterdtsearchpathreturns with the`DTAPPSEARCHPATH`assembled,
theXsessionscript callsdtappgather.

As its main function,dtappgathertraverses the`DTAPPSEARCHPATH`,
examining each possible source of applications and, where
there are existing source subdirectories, creates symbolic links between
the source and the user's`DTUSERSESSION`subdirectory.

Finally,dtappgatherturns off write permissions on the resulting subdirectory to ensure its
integrity.

A similar sequence occurs when the user double-clicks the`ReloadApps`action after logging in.

Although the value of the`DTAPPSEARCHPATH`can be modified in a
local shell, the
Application Manager
view based on its value is not functional until you log out and
log back in.
Since this can be a tedious venture, you can verify the
Application Manager
view by executing
&cdeman.dtsearchpath; anddtappgatherin the local shell.
For
example, to add host`trout`as a system-wide
application server, set the
&cdeman.dtsearchpath; input environment variable,DTSPSYSAPPHOSTS:`DTSPSYSAPPHOSTS=trout`: Then, execute
&cdeman.dtsearchpath; to update the`DTAPPSEARCHPATH`environment
variable.`eval```dtsearchpath``Finally, gather the new applications by executingdtappgather. The
Application Manager
will show the new application groups but will not be functional.
## OPTIONS


The`-r`option causesdtappgatherto retain the previous contents of the Application Manager, although
discarding broken links.
At login,dtappgatherdestroys the user's previous`DTUSERSESSION`subdirectory before
creating a new one.
At`ReloadApps`time, the`-r`option
is used to minimize visual disruption of any opened
Application Manager
views.
## RETURN


The command always returns 0 (zero) for successful completion.
## ENVIRONMENT


* **`DTAPPSEARCHPATH`** 

Set by the
&cdeman.dtsearchpath; utility.`DTAPPSEARCHPATH`controls the places
wheredtappgatherwill gather applications.
The default locations consist of$HOME/.dt/appmanager(for end users),/etc/dt/appconfig/appmanager/$LANG(for system administrators), and/usr/dt/appconfig/appmanager/$LANG(for factory applications).
* **`DTUSERSESSION`** 

Controls the location of the end user's subdirectory where the
Application Manager will be rooted.
The subdirectory name includes
both the user's$LOGNAMEand$DISPLAYin order to
ensure the user's view of the Application Manager remains consistent
across sessions.

## LOCALES


When searching for system administrator and factory default
Application Manager
files,dtappgatheralways uses a value of$LANGequal to`C`.
When the
user picks a language at login, the corresponding$LANGsubdirectories are searched.
If multiple$LANGsubdirectories exist
in/etc/dt/appconfig/appmanager, all$LANGsubdirectories will
appear in the Application Manager; however, if multiple$LANGsubdirectories exist in/usr/dt/appconfig/appmanagerthen the
language-specific subdirectory based on the current value of$LANGis
chosen instead of the`C`subdirectory.
Regardless of locale, all the action files that exist under the user's$HOME/.dt/appmanagersubdirectory will be symbolically linked
to the user's
Application Manager
subdirectory.
## FILES


* **/usr/dt/bin/Xsession** 

Among its tasks at login,
theXsessionscript invokes
&cdeman.dtsearchpath;, and thendtappgather.
* **/var/dt/appconfig/appmanager/$DTUSERSESSION** 

This subdirectory is wheredtappgatherassembles the
Application Manager
view for the particular user and CDE session.

## NOTES


In the case of multiple search locations having the same name,dtappgatherpicks only the first and discards the rest.
For example, if the
system administrator sets`DTSPSYSAPPHOSTS=tuna:,trout:`and if both hosts,`tuna`and`trout`, have an/etc/dt/appconfig/appmanager/C/Editorsfolder, then only the`Editors`folder from`tuna`(the first host) will appear
in the user's Application Manager.
After building the user's`DTUSERSESSION`,dtappgatherturns off write permissions on that subdirectory to disallow alteration
by the end user.
Although the end user can resize the
window and rearrange the icons within the window, the Application
Manager is intended to be a read-only source of local
and networked applications.
## SEE ALSO


&cdeman.dtappintegrate;, &cdeman.dtsearchpath;.