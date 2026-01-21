dtsessionuser cmddtsessionthe
CDE Session Managerdtsession-norestore-sessionsession_nameDESCRIPTIONThedtsessionclient provides X11R6 XSMP and (by
proxy) ICCCM 1.1 compliant session management functionality during a user
session, which is the time from login to logout. It launches a window manager
and allows for saving a session, restoring a session, locking a session, launching
screen savers and allocating colors for desktop compatible clients.Key Supported TasksThedtsessionclient supports the following key tasks:Initializing a session.Launching a window manager.Restoring a home, current,display-specifichome, ordisplay-specificcurrent session.Providing session locking on command or timeout.Providing session screen saving on command or timeout.Acting as a color allocation server for other DT
clients.Saving a home, current,display-specifichome, ordisplay-specificcurrent session.Displaying confirmation dialog at logout.Displaying session selection dialog at logout.Terminating a session.A SessionA session is the collection of applications, settings and resources
that are present on the user's desktop. Session management is a set of conventions
and protocols that allow a special session manager such asdtsessionto save and restore a user's session. A user is able to log into
their system and be presented with the same set of running applications, settings
and resources as were present when the user logged off. When a user logs into
the desktop for the first time, a default initial (new user) session is loaded.
Afterward,dtsessionsupports the notion of a current, a home, and adisplay-specificsession, which may be either current or home.The Initial SessionWhen a user logs into the desktop for the first time,dtsessionwill generate the user's initial session using system default values.
The initial session is sometimes referred to as the "new user" session. Refer
to Session Resource Management and Session Application Management for more
information.The Current SessionThe user's running session is always considered the current session,
whether restored upon login from a saved home session, a saved current session
or the system default initial session. Based on the user's Style Manager
Startup settings, when the user exits the session, the current session is
automatically saved. When the user next logs into the desktop,dtsessionallows the user to select the previously saved current
session as the session to start. This means that the desktop will be restored
to the same state it was in when the user last logged out.The Home SessionAnother option is having the desktop be restored to the same state every
time the user logs in, regardless of its state when the user logged out. The
user may save the state of the current session, then via the Style Manager
Startup settings, have the desktop start that session every time the user
logs in. Alternatively,dtsessionwill allow the user to
select the home session as the session to start at login.Display-specific SessionsTo run a specific session for a specific display, a user can create
a display-specific session. To do this, the user can copy the$HOME/.dt/sessionsdirectory to the$HOME/.dt/sessions/displaydirectory
wheredisplayis the real,
unqualified hostname (for example, pablo:0 is valid, pablo.gato.com:0 or local:0
is not). When the user logs in,dtsessionwill allow the
user to select a display-specific home or current session as the session to
start.Dtsessionwill actually launch the session only
if it is compatible with the user's login display.The X11R6 XSMP Session Management ProtocolFor an application to be saved upon logout and restarted upon login,
it must participate in a simple session management protocol.dtsessionsupports the X11R6 XSMP Session Management Protocol.Applications that wish to save their state can take part in theWM_SAVE_YOURSELFprotocol. To do this, an application needs to set theWM_SAVE_YOURSELFproperty
on one and only one of its top-level windows. When the user presses theSet Home Sessionbutton in the Style Manager,dtsessionsends aSaveYourselfmessage to the top-level
window of each of its clients. The message sets the save-type toLocal, shutdown to 0, interact-style toNoneand fast to 0.When an application receives the message, it sends aSaveYourselfPhase2Requestmessage to the Session Manager, which will reply with aSaveYourselfPhase2message. Only when the Window Manager receives
theSaveYourselfPhase2message will it save its state.
The information saved by the Window Manager for each of its client's set
of windows will include: geometry, icon state and workspace.If interact-style on the initialSaveYourselfmessage
isNone, the application cannot interact with the user
in any way as it is saving its state. If it isAny, the
application can interact with the user for any purpose. This includes the
ability to de-iconify itself, as well as to change its current workspace.
To minimize confusion, the Session Manager issues a ToolTalk message to the
Window Manager requesting the recommended order to interact. The Window Manager
responds with the list of SM_CLIENT_IDs and their associated workspace numbers
in workspace order. After receiving an Interact message (with interact-style
set toAny), an application should go system-modal to
prevent other, less well-behaved non-interacting applications from allowing
user interaction while the application is interacting with its user.Because an application will usually save its state into a file, the
Session Manager providesDtSessionSavePathas a convenience
function that returns a full pathname of a file in which an application can
save its state. While the application is saving its state,dtsessionawaits notice from the application that it is done. To telldtsessionthat the state save is complete, the application updates
theWM_COMMANDproperty on its top-level window.TheWM_COMMANDproperty on an application's top-level window serves
two purposes. First, a change of this property is the indication todtsessionthat an application is done saving its state anddtsessioncan proceed to the next application. Second, theWM_COMMANDproperty value is expected to contain the command line thatdtsessionwill use to restart the application at session startup. If an application
is launched with a full pathname, then it should use the full pathname when
settingWM_COMMAND. Applications that do not need to save their state, but
wish to be restarted, can simply setWM_COMMANDduring application startup
and forget about it.Refer to theXmAddWmProtocols,XmAddWMProtocolCallback, andXSetCommandAPIs for more information.In addition to the "save-yourself" client session management support,
the X11R6 XSMP protocol provides the following features (which are not found
in the ICCCM protocol):It allows applications to interact with the
user during normal shut down to confirm or discard unsaved changes.It provides a mechanism to explicitly tell applications
to exit.Most importantly, it provides a common framework
to support applications that are not X clients and applications that exit
before the session is saved but that must remain part of the session for
restart purposes (for example, an input method server).The ICCCM Session Management Protocoldtsessionimplements the X11R6 XSMP session
management protocol and provides backward compatibility by acting as a proxy
for client applications that continue to use the older ICCCM session management
protocol.Restoring A SessionAt session startup time,dtsessionpresents a dialog
that allows the user to select which of the following sessions to start:CurrentStart the user's most recent session.HomeStart the user's home session.display-name- CurrentCreate a new display-specific session and start the first of the following
sessions that exists:display-specific Homegeneric Homenew user (initial) sessiondisplay-name- HomeCreate a new display-specific session and start the user's generic home
session if it exists. Otherwise, start a new user session.Fail-safe SessionStart a fail-safe session (Xfailsafe)Although a user's list of sessions is not known until after the user
logs in, the dialog presents all of the session choices. If a user selects
a session that does not exist,dtsessiontakes the following
actions. If the user selects:Homedtsessionstarts a new user session.Currentdtsessionstarts the user's home session if it exists.
If it does not, it starts a new user session.If the user selects a display-specific session and one does not exist,dtsessionposts a warning dialog stating that a new session will
be created. The warning dialog contains three buttons:Cancel LoginCancels the login and returns the user to the login screen.`OK`If a display-specific Home session was selected,dtsessioncreates a new display-specific session and starts the user's generic
home session if it exists. If it does not exist, it starts a new user session.If a display-specific Current session was selected,dtsessioncreates a new display-specific session and starts a display-specific
home, generic home, or new user session, depending on which it finds first.HelpDisplays help text about the warning dialog.Session Resource ManagementThe session manager uses the X ServerRESOURCE_MANAGERproperty on which
to make available desktop resources to all applications. The session manager
will load theRESOURCE_MANAGERin the following manner:load the system default resources, ANDmerge any system administrator specified resources,
ANDmerge any user specified resources.The desktop default resources can be found in/usr/dt/config/$LANG/sys.resources.
These resources will be made available to each user's session
via theRESOURCE_MANAGERproperty. This file should not be edited as it will
be unconditionally overwritten upon subsequent desktop installations.A system administrator may augment the system default resources by creating/etc/dt/config/$LANG/sys.resources. In this file, a
system administrator may override system default resources or specify additional
resources. As this file is merged into the desktop default resources during
session startup, it is preferable that only new or updated resource specifications
be placed in this file, rather than a copy being made of the desktop default
resource file. Resources specified in this file will be made available to
each user's session via theRESOURCE_MANAGERproperty. Resources specified
in this file take precedence over those specified in the desktop default resource
file.A user may augment the desktop default and system administrator resources
via their$HOME/.Xdefaultsfile. Resources specified
in this file will be made available to only that user's session via theRESOURCE_MANAGERproperty. Resources specified in this file take precedence over those specified
in the desktop default or system administrator resource files.The X Toolkit Intrinsics specifies that it will load resources for an
application from eitherRESOURCE_MANAGERor from$HOME/.Xdefaults,
but not both. Ordinarily, this would mean that the user's$HOME/.Xdefaultsfile would be ignored. However, the session manager
accommodates$HOME/.Xdefaultsby merging it into theRESOURCE_MANAGERat session startup as described above. If a user changes$HOME/.Xdefaults,
the changes will not be visible to new applications until the
user invokes theReloadResourcesaction.TheReloadResourcesaction will instruct the session
manager to reload theRESOURCE_MANAGERwith the system, system administrator,
and user specified resources. This is useful to make available to new applications
changes made to system administrator or user specified resource files.See also &cdeman.dtresourcesfile; and &cdeman.dtsessionaction;.Session Application ManagementAt session startup, the session manager will restart any applications
that were saved as part of the session. The system default set of applications
to be restored as part of the user's Initial Session can be found in/usr/dt/config/$LANG/sys.session. This file should not be edited
as it will be unconditionally overwritten upon subsequent desktop installations.See also &cdeman.dtsessionfile;.A system administrator may replace the set of applications that are
restored as part of the user's Initial Session by creating a file named/etc/dt/config/$LANG/sys.session. Unlike the resource files, this
file will be used as a complete replacement for the desktop default file,
so it is valid to make a copy of the system default file and make any necessary
modifications.Session Manager AuthenticationThe Session Manager uses ICE-based authentication as described in the
Inter-Client Exchange (ICE) Library v1.0.The Session Manager is built with a table of available authentication
protocols. Selection of the protocol is done via theAuthNameresource. TheAuthNamedefault is the stringMIT-MAGIC-COOKIE-1. The Session Manager supports the same protocols
as the Login Manager.The Window Managerdtsessionis responsible for starting the window
manager. By default/usr/dt/bin/dtwmis started. An alternate
window manager can be specified with the wmStartupCommand resource. Refer
to the Workspace Manager specification for more information.The Style ManagerThe style manager provides the interface by which a user can change
various desktop and X server settings for the current session. Refer to the
Style Manager specification for more information.The Color Serverdtsessionserves as the color server for the desktop
and provides a set of resources that can be used to configure it.
TheforegroundColorresource controls whether a pixel is allocated for
the foreground color. ThedynamicColorresource specifies
whether read-only colors are allocated. TheshadowPixmapsresource specifies whether colors are allocated for top shadow or bottom shadow.
ThecolorUseresource limits color allocation. Finally,
thewriteXrdbColorsresource specifies whether
the*backgroundand*foregroundresources are placed in the resource database. See the Color
Server Resources section for more information.The Color Sharing ProtocolThe CDE desktop makes use of a color sharing protocol betweendtsessionand the rest of the desktop (dtstylein particular)
and Motif. This protocol allows:Non-CDE (non Motif/XmColorObject) applications to
make use of the CDE color scheme.Applications (Motif or not) that create their own private colormap to
copy thedtsessiondesktop pixels and avoid or
limit technicolor effect.Motif applications to easily share the desktop GUI pixels when they use
a private colormap.The color sharing protocol involves:The selection name (CUSTOMIZE_DATA), type (TYPE_OF_MONITOR), and format used bydtsessionto communicate the monitor
characterization todtstyle(or any other style
manager).The selection name (CUSTOMIZE_DATA), type (PIXEL_SET), and format used bydtsessionto communicate its palette pixel ids to
theColorObject(inlibXm) and
todtstyle.Two functions (XmeGetColorObjDataandXmeGetColorObjCells) that deliver the desktop
pixels to the application in a form easily suitable for its use
(`XColor`).A Color Set is a set of five colors that are used to represent a single
logical color in the Motif toolkit. For each background color (the
logical color), there are associated top shadow, bottom shadow,
foreground, and select colors, all generated from the background color.
These associated colors are the mechanism for giving widgets their 3-D
appearance.A Color Palette is a named set of a maximum of eight background colors.
A single palette is used to color the desktop components. A list of
default palettes is provided from which the user can select. The user
can also add and delete palettes, as well as modify an existing palette.dtsession, the color server, uses ICCCM X Selection
based mechanisms to communicate color use, palette and color set pixel
information to the desktop clients.The name of the (CUSTOMIZE_DATA) selection is the atomCustomize Data:i, whereiis the screen number.The names of the targets are the atomsType Of MonitorandPixel Sets.Type of Monitor TargetTheType Of Monitortarget is used to convey color
settings to the desktop clients, such asdtstyleor
the Motif toolkit, that need this information. The content corresponds
to the value of thedtsessioncolor usage
resources.When asked to convert the screenCustomize
Data:iselection to the targetType Of Monitor, the format and content encoding used
is the following:type:STRINGlength:20format:8content:a series of 4 numbers, in hexadecimal
format, separated by the underscore (_) character (that is, using the
printf/scanf format%x_%x_%x_%x), and including from
left to right:ThecolorUseresource, which corresponds to the type
of monitor in use by the desktop.B_W= 0LOW_COLOR= 1MEDIUM_COLOR= 2HIGH_COLOR= 3TheshadowPixmapsresource, which controls if
dithered pixmaps are used to render the shadows in the target GUI.FALSE= 0TRUE= 1TheforegroundColorresource, which controls whether
or not a pixel was allocated for the foreground or ifWhitePixelorBlackPixelis used.DYNAMIC= 0BLACK= 1WHITE= 2ThedynamicColorresource, which controls whether or
not the pixels allocated are read/write or read-only cells.FALSE= 0TRUE= 1This information, especiallydynamicColor, is
currently used only bydtstylein CDE to
implement the color manager GUI.Pixel Sets TargetThePixel Setstarget is used to convey palette and
color set pixel information to the desktop clients. The content is
always 8 color set values, but depending on the color usage settings,
some entries will be the duplicated. For the receiver of this
information, however, it simply means it can access the index as
specified in the high color scheme.When asked to convert the screenCustomize
Data:iselection to the targetPixel Sets, the format used is the following:type:STRINGlength:400format:8content:a first number (%x_) for
thecolorUseresource (see "Type of Monitor Target"
above for encoding) and a series of 8 (corresponding to the maximum
number of color sets in CDE) sets of 5 numbers, in hexadecimal format
(%x_%x_%x_%x_%x), each describing from left to right:backgroundpixel (bg)foregroundpixel (fg)top_shadowpixel (ts)bottom_shadowpixel (bs)select_colorpixel (sc)The mapping between color set Ids andcolorUseis as follows:HIGH_COLORActive window borders.Inactive window borders.Switch for workspace 1 and every fourth additonal workspace (workspace 5, 9,...).Text and list areas.Main window background (primaryColorSetId) and switch
for workspace 4 and every fourth additional workspace (workspace 8,
12,...).Dialog box background and menu bar
(secondaryColorSetId) and switch for workspace 3 and
every fourth additional workspace (workspace 7, 11,...).Switch for workspace 2 and every fourth addional workspace (workspace 6, 10, ...).Front panel background.MEDIUM_COLORActive window borders.Window bodies:Inactive window bordersMain window and dialog box backgrounds and menu barFront Panel backgroundWorkspace switches and backgroundsText and list backgroundssame as 2same as 2same as 3same as 2LOW_COLORandBLACK_WHITEActive window borders.Everything else... to 8: same as 2.For each color set, the cell allocation scheme is the following
(dynamicColordetermines if the pixels are allocated
read/write or read-only):colorUseshadowPixmapsforegroundColorCells allocationNumberHIGH_COLORFALSEDYNAMIC(fg,bg,ts,bs,sc)5*8 = 40HIGH_COLORFALSEBLACK or WHITE(bg,ts,bs,sc)4*8 = 32HIGH_COLORTRUEDYNAMIC(fg,bg,sc)3*8 = 24HIGH_COLORTRUEBLACK or WHITE(bg,sc)2*8 = 16MEDIUM_COLORFALSEDYNAMIC(fg,bg,ts,bs,sc)5*4 = 20MEDIUM_COLORFALSEBLACK or WHITE(bg,ts,bs,sc)4*4 = 16MEDIUM_COLORTRUEDYNAMIC(fg,bg,sc)3*4 = 12MEDIUM_COLORTRUEBLACK or WHITE(bg,sc)2*4 = 8LOW_COLORFALSEDYNAMIC(fg,bg,ts,bs,sc)5*2 = 10LOW_COLORFALSEBLACK or WHITE(bg,ts,bs,sc)4*2 = 8LOW_COLORTRUEDYNAMIC(fg,bg,sc)3*2 = 6LOW_COLORTRUEBLACK or WHITE(bg,sc)2*2 = 4B_WTRUEoppositeBG0Session LockDtsession provides session locking. The current session can be locked
directly by pressing the lock icon on the front panel. If supported by the
X server, the current session can be locked after a specified period of inactivity.
To unlock the session, the user must enter their login password.By default,dtsessionsupports traditional local UNIX authentication
for unlocking the session. Additional re-authentication functions such as
those required by DCE may be added by individual vendors.Screen SaversDtsession provides support for the launching of external screen savers
as a part of session locking from the front panel or, if supported by the
X server, after a specified period of inactivity. Refer to the Screen Saver
specification for information as to how screen savers are integrated into
the desktop.X Server Screen Saver ExtensionsDtsession's ability to provide session lock or screen saver launch after
a specified period of inactivity depends upon the availability of an X server
screen saver extension.dtsessionsupports two such extensions:X Consortium Sample X11 Screen Saver Extension
1.0HP X Screen Saver ExtensionThe ability ofdtsessionto recognize both, either or none of these
extensions is vendor specific.Launching The Session Managerdtsessionshould be launched from the Xsession script.
Xsession is described in the login manager specification. It is recommended
that Xsession be launched fromdtloginas part of the login
sequence as the default, but there are alternative methods of starting Xsession:dtloginthe default dtlogin configuration launchesXsessionwhen a user logs inproxysome systems will allow programs such asxinit,x11startorstartxto start XsessionStarting Services Before the Session ManagerIf you must start a service after login but before the Session Manager
(for example, an input method server), start the service by placing a script
in the fdirectory/usr/dt/config/Xsession.d.To avoid being restarted by the Session Manager when a session is started,
a service that must be started before the Session Manager should explicitly
set theRestartStyleHintproperty toRestartNever(possibly by setting an Xt resource).OPTIONSThedtsessionclient is automatically invoked by
the DT Login Manager (dtlogin). If desired,dtsessionmay also be started on an existing X server. Note thatdtsessionautomatically starts a window manager.The following options are available:-norestoreInstructsdtsessionnot to restore a previous session
nor save the session upon logout.-sessionsession_nameInstructsdtsessionto start the specified session.
Valid session names are:currenthomedisplay_name:display_number/currentdisplay_name:display_number/homewheredisplay_nameis the unqualified display host
name anddisplay_numberis the display number.RETURNExit values are:0Successful completion.>1Error condition occurred.EXAMPLESdtsession -norestoreStart session manager from command line without restoring previous session.RESOURCESColor ServerNameClassTypeDefaultcolorUseColorUseStringDEFAULTdynamicColorDynamicColorBooleanTrueforegroundColorForegroundColorStringDYNAMICshadowPixmapsShadowPixmapsStringDEFAULTwriteXrdbColorsWriteXrdbColorsBooleanTrueScreen Lock/Screen SaveNameClassTypeDefaultkeysKeysunsigned charNULLpasswordTimeoutpasswordTimeoutunsigned int10Miscellaneous