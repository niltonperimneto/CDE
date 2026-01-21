# dtresourcesfile
special filedtresourcesfileformat and location of desktop resource files
## DESCRIPTION


The desktop default resources can be found in
/usr/dt/config/$LANG/sys.resources.
These resources will be
made available to each user's session via the`RESOURCE_MANAGER`property.
This file should not be edited as it will be unconditionally
overwritten upon subsequent desktop installations.

A system administrator may augment the system default resources
by creating/etc/dt/config/$LANG/sys.resources. In this file, a
system administrator may override system default resources or
specify additional resources.
As this file is merged into the
desktop default resources during session startup, it is preferrable
that only new or updated resource specifications be placed in this
file, rather than a copy being made of the desktop default resource
file.
Resources specified in this file will be made available to
each user's session via the`RESOURCE_MANAGER`property.
Resources
specified in this file take precedence over those specified in
the desktop default resource file.

A user may augment the desktop default and system administrator
resources via their $HOME/.Xdefaults file.
Resources specified in
this file will be made available to only that user's session via
the RESOURCE_MANAGER property.
Resources specified in this file
take precedence over those specified in the desktop default or
system administrator resource files.

`NOTE``:`The X Toolkit Intrinsics specifies that it will load resources
for an application from either`RESOURCE_MANAGER`or from$HOME/.Xdefaults, but not both.
Ordinarily, this would mean that
the user's$HOME/.Xdefaultsfile would be ignored.
However, the
session manager accommodates$HOME/.Xdefaultsby merging it into
the`RESOURCE_MANAGER`at session startup as described above.
If
a user changes the$HOME/.Xdefaults, the changes will not be
visible to new applications until the user invokes the
ReloadResources action.

The ReloadResources action will instruct the session manager to
reload the`RESOURCE_MANAGER`with the system, system administrator
and user specified resources.
This is useful to make available to
new applications changes made to system administrator or user
specified resource files.
## FILE FORMAT


The desktop resource files use X resource syntax.
SeeX(1) for
the precise syntax.
## FILES


* **/usr/dt/config/$LANG/sys.resources** 

The desktop default resources.
* **/etc/dt/config/$LANG/sys.resources** 

System administrator specified resources.
* **$HOME/.Xdefaults** 

User specified resources.

## SEE ALSO


&cdeman.dtsession;, &cdeman.dtsessionaction;,X(1),`Xlib Resource Manager documentation`,`Xt resource documentation`.