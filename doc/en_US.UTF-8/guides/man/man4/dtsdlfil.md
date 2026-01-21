# dtsdlfile
special filedtsdlfileCDE runtime help volume files
## DESCRIPTION


Semantic Delivery Language (sdl) files are used for presenting online
information or application specific contextual help information in CDE.
They
are referred to as Help Volumes, and contain the equivalent of a runtime
version of the original, authored help information.
Help Volumes can be
viewed via one of the two CDE help dialog widgets provided as part of the CDE
Development Environment.
### File Generation


Refer to the
&cdeman.dthelptag; man page for detailed information on the creation of*.sdlfiles within CDE.
## ENVIRONMENT VARIABLES


The CDE Help system uses two environment variables for locating
Help volumes and Family files
within the desktop environment:

* **DTHELPSEARCHPATH** 

Systemsearch path environment variable for locating
Help volumes on local and/or remote nfs mounted systems.
* **DTHELPUSERSEARCHPATH** 

Userssearch path environment variable for locating user
specific Help volumes on local and/or remote nfs mounted systems.


The environment variables contain colon-separated lists of directory paths.
Each directory path can contain environment variable names as well as
special field descriptors that are expanded at runtime.

Field descriptors consist of a`%`followed by a single character.
Field descriptors and their substitution values are:

* **`%H`** 

This value is replaced with the current volume or Family name being searched
for.
* **`%L`** 

Replaced with the current value of the`LANG`environment variable.
* **`%%`** 

Replaced with a single`%`.


The default value forDTHELPUSERSEARCHPATHis:

$HOME/.dt/help/$DTUSERSESSION/%H:$HOME/.dt/help/$DTUSERSESSION/%H.sdl:$HOME/.dt/help/$DTUSERSESSION/%H.hv:$HOME/.dt/help/%H:$HOME/.dt/help/%H.sdl:$HOME/.dt/help/%H.hv

TheDTHELPUSERSEARCHis first searched for the requested volume.
If
the volume is not found, theDTHELPSEARCHPATHvalue is searched.

The default value forDTHELPSEARCHPATHpath is:

/etc/dt/appconfig/help/%L/%H:/etc/dt/appconfig/help/%L/%H.sdl:/etc/dt/appconfig/help/%L/%H.hv:/etc/dt/appconfig/help/C/%H:/etc/dt/appconfig/help/C/%H.sdl:/etc/dt/appconfig/help/C/%H.hv:/usr/dt/appconfig/help/%L/%H:/usr/dt/appconfig/help/%L/%H.sdl:/usr/dt/appconfig/help/%L/%H.hv:/usr/dt/appconfig/help/C/%H:/usr/dt/appconfig/help/C/%H.sdl:/usr/dt/appconfig/help/C/%H.hv
## SEE ALSO


&cdeman.DtCreateHelpDialog;, &cdeman.DtCreateHelpQuickDialog;, &cdeman.dthelptag;,CDE Help System Author's and Programmer's Guide