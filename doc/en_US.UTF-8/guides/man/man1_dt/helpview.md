# dthelpview
user cmddthelpviewview a CDE help volumedthelpviewoptions
## DESCRIPTION


Thedthelpviewutility supports the viewing of CDE online Help volumes, ASCII files
or manual pages.
Thedthelpviewutility is an integral part of the CDE application Help
environment.
Thedthelpviewutility's functionality and user interface is almost completely that
of the standard CDE general Help and quick Help dialog widgets.
(See
&cdeman.DtCreateHelpDialog; and
&cdeman.DtCreateHelpQuickDialog;.)
## OPTIONS


The following options are available:

* **`-helpVolume`volume** 

Specifies the name of the`<filename>.sdl`file you want to
view.
If`locationId`is not set, the default`_HOMETOPIC`ID is used.
If the requested volume is in the current working directory, or the Help volume
has been properly registered, no path is required.
* **`-locationId`location_id** 

Specifies the location ID representing the desired Help topic to be viewed.
By default, Helpview uses`_HOMETOPIC`if an ID is not specified.
* **`-file``file`** 

Specifies a particular ASCII text file to be displayed.
* **`-manPage`man_page** 

Specifies a particular man page to be displayed.
* **`-man`** 

Displays a dialog that prompts for a man page to view,
then displays the requested man page.

## ENVIRONMENT VARIABLES


The CDE Help system uses two environment variables for locating
Help volumes and Family files within the desktop environment:

* **DTHELPSEARCHPATH** 

Systemsearch path environment variable for locating
Help volumes on local and remote nfs mounted systems.
* **DTHELPUSERSEARCHPATH** 

Userssearch path environment variable for locating user
specific Help volumes on local and remote nfs mounted systems.


The environment variables contain colon-separated lists of directory paths.
Each directory path can contain both environment variable names as well as
special field descriptors that are expanded at runtime via each Help component
that uses these environment variables.

Field descriptors consist of a`%`followed by a single character.
Field descriptors and their substitution values are:

* **`%H`** 

This value is replaced with the current volume name being searched
for.
* **`%L`** 

Replaced with the current value of the`LANG`environment variable.
* **`%%`** 

Replaced with a single`%`.


The default value forDTHELPUSERSEARCHPATHis:

`$HOME/.dt/help/$DTUSERSESSION/%H:
$HOME/.dt/help/$DTUSERSESSION/%H.sdl:
$HOME/.dt/help/$DTUSERSESSION/%H.hv:
$HOME/.dt/help/%H:
$HOME/.dt/help/%H.sdl:
$HOME/.dt/help/%H.hv`

TheDTHELPUSERSEARCHis first searched for the requested volume.
If
the volume is not found, theDTHELPSEARCHPATHvalue is searched.

The default value forDTHELPSEARCHPATHpath is:`/etc/dt/appconfig/help/%L/%H:
/etc/dt/appconfig/help/%L/%H.sdl:
/etc/dt/appconfig/help/%L/%H.hv:
/etc/dt/appconfig/help/C/%H:
/etc/dt/appconfig/help/C/%H.sdl:
/etc/dt/appconfig/help/C/%H.hv:
/usr/dt/appconfig/help/%L/%H:
/usr/dt/appconfig/help/%L/%H.sdl:
/usr/dt/appconfig/help/%L/%H.hv:
/usr/dt/appconfig/help/C/%H:
/usr/dt/appconfig/help/C/%H.sdl:
/usr/dt/appconfig/help/C/%H.hv`
## RESOURCES


For information on Help dialog widget resources, refer to
&cdeman.DtCreateHelpDialog; or
&cdeman.DtCreateHelpQuickDialog;.

Thedthelpviewutility's specific resources set in the Dthelpview app-defaults file are:Dthelpview*manBox*rows: 32
Dthelpview*manBox*columns: 80
Dthelpview*fileBox*rows: 32
Dthelpview*fileBox*columns: 80
Dthelpview*man_text.columns: 20
## FILE


Following files are used in conjunction with thedthelpviewapplication:

* **`Dthelpview`** 

App-defaults file used bydthelpview.

## EXAMPLES


* **dthelpview -helpVolume dtintro.sdl -locationId _hometopic** 

Displays the topic associated with _hometopic in the Help
volume dtintor.sdl.
* **dthelpview -file /etc/checklist** 

Displays the file /etc/checklist in a general Help dialog
* **dthelpview -man grep** 

Displays the grep man page in a quick Help dialog

## SEE ALSO


&cdeman.dtsearchpath;,CDE Help System Author's and Programmer's Guide.