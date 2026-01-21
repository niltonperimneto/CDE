# dtsession_res
user cmddtsession_resCDE dtsession resource load utilitydtsession_res -load-merge-system-xdefaults-file <name>
## DESCRIPTION
The Common Desktop Environment (CDE)dtsession_resutility is used bydtsessionand theReloadResourcesaction to load session resources. It
is not intended to be run directly from the command line.

The CDE dtsession_res utility usesxrdb(1) to load the session resources into the
RESOURCE_MANAGER property of the root window of the display. The session
resources are gathered from the following files:

- /usr/dt/config/$LANG/sys.resources

- /etc/dt/config/$LANG/sys.resources

- $HOME/.Xdefaults

- $HOME/.dt/<session dir>/dt.resources

Refer to the
&cdeman.dtsession;
man page for more information on these files and their content.

Thedtsession_resutility provides support for display-specific resources by converting
the $DISPLAY value into acpp(1) macro that can be used in a session resource file to limit resource
definitions to a particular display. It does this by converting all
&acute;.' (dot) and ':' (colon) characters to '_', stripping off any screen
specification and finally prefixing 'DISPLAY_' to the result. For example,
a $DISPLAY of ':0' would be 'DISPLAY_0', and a $DISPLAY of 'blanco.gato.com:0.0'
would be 'DISPLAY_blanco_gato_com_0'. The resulting value can be used
as part of a`cpp`test in a session resource file. For example:Myapp*resource: value

   #ifdef DISPLAY_blanco_gato_com_0
   Myapp*resource: specialvalue1
   #endif

   #ifdef DISPLAY_pablo_gato_com_0
   Myapp*resource: specialvalue2
   #endif
## OPTIONS

### -load


Replace the current RESOURCE_MANAGER content with the session resources.
### -merge


Merge the current RESOURCE_MANAGER content with the session resources.
### -system


Load or merge the system session resources.
### -xdefaults


Load or merge the $HOME/.Xdefault file.
### -file <name>


Load or merge the file specified by <name>.
## EXAMPLES


Thedtsession_resutility is not intended to be run from the command line. Refer to the`ReloadResources`action for information on how to reload session resources in a
user's session.

dtsession_res -load -system -xdefaults \
-file $HOME/.dt/sessions/current/dt.resources

Replaces the RESOURCE_MANAGER with the content of the following files:

- /usr/dt/config/$LANG/sys.resources,

- /etc/dt/config/$LANG/sys.resources,

- $HOME/.Xdefaults and

- $HOME/.dt/sessions/current/dt.resources files.