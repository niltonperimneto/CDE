# dtstyle
user cmddtstylethe CDE Style Managerdtstyle
## DESCRIPTION


There are no style manager specific options that can be
specified in the command line.

Dtstyle provides interactive customization of visual elements and
system behavior for the desktop through the style manager.

This component consists of the UI and supporting code that allows the
end user to interactively customize most of the visual elements and
system behavior for the CDE. This component also helps define some
resource utilization conventions. The style manager's main window
which consists of an iconic interface is invoked from the Front Panel.
The style manager window contains icons representing customization in
several areas.
### Key Supported Tasks


* **Colors** 

Sets the color palette that is used for window colors in the
workspace.
* **Fonts** 

Sets the application font sizes as applications are started.
* **Backdrop** 

Sets the backdrops for workspaces.
* **Keyboard** 

Sets the keyboard's key click volume and character repeat
capability.
* **Mouse** 

Sets the mouse button click settings, double-click speed,
pointer acceleration and pointer movement threshold.
* **Beep** 

Sets the beeper volume, tone, and duration.
* **Screen** 

Sets the screen lock and time-out intervals, chooses the
screen saver client(s) to be run at time out and/or screen lock
and previews the selected screen saver client(s) to the user.
* **Window** 

Sets the window focus policy, window raise policy and
iconification policy (in an icon box or on the desktop).
* **Startup** 

Sets the session startup policy.

## OPTIONS


There are no style manage specific options that can be set from the
command line.
## RESOURCES
New ResourcesNameClass/TypeAccessDefaultimServerHostsImServerHosts/stringGNULLimserverHostsA comma-separated list of IM server hosts.
### Style Manager specific resources


The style manager used the following resources to control its
appearance and behavior.NameClassTypeDefaultwriteXrdbImmediateWriteXrdbImmediateBooleanTrue
### writeXrdbImmediate


Controls when new font size or new mouse double-click time resources
are used. If True, new resources are used when new clients start. If
False, new resources are used at the next session (after logout, with
"Resume Current Session" selected in the Startup dialog).The following resources are specified in Dtstyle (dtstyle's
app-defaults file).NameClassTypeDefaultmainRC.orientationXmNorientationstringHORIZONTALcomponentListComponentListstring(See description)secondaryColorSetIdSecondaryColorSetIdint6toneScale.maximumXmCMaximumint9000toneScale.minimumXmCMaximumint82durationScale.maximumXmCMaximumint25
### mainRC.orientation


Controls the layout of the style manager main window. If this resource
is set to VERTICAL then the style manager main window is displayed
vertically.
### componentList


Specifies the locale-specific Style Manager components with a list of component
keywords separated by spaces. Valid component keywords include: Color,
Font, Backdrop, Keyboard, Mouse, Audio, Screen, Window, and Startup.Default in the C locale:Color Font Backdrop Keyboard Mouse Beep
Screen Dtwm Startup. Default in the Japanese locale:Color Font Backdrop Keyboard Mouse Beep
Screen Dtwm Startup I18N.
### secondaryColorSetId


Specifies the color to be used for the menubar and all menus and
dialogs of thedtstyleapplication.
### toneScale.maximum


Specifies the maximum value to be used for the tone slider in the Beep
dialog.
### toneScale.minimum


Specifies the minimum value to be used for the tone slider in the Beep
dialog.
### durationScale.maximum


Specifces the maximum value to be used for the duration slider in the
Beep dialog.
### fileMenu.exit.accelerator


Specifies the accelerator key for the Exit menu option of the style
manager main window.
### fileMenu.exit.acceleratorText


Specifies the text for the accelerator key that appears next to the
Exit menu option of the style manager main window.
### dtstyle Specific Resources


The following font resources are specified in Dtstyle.NameClassTypeDefaultnumFontsNumFontsint6systemFont1SystemFont1FontListsee SF1 belowsystemFont2SystemFont2FontListsee SF2 belowsystemFont3SystemFont3FontListsee SF3 belowsystemFont4SystemFont4FontListsee SF4 belowsystemFont5SystemFont5FontListsee SF5 belowsystemFont6SystemFont6FontListsee SF6 belowsystemFont7SystemFont7FontListsee SF7 belowuserFont1UserFont1FontListsee UF1 belowuserFont2UserFont2FontListsee UF2 belowuserFont3UserFont3FontListsee UF3 belowuserFont4UserFont4FontListsee UF4 belowuserFont5UserFont5FontListsee UF5 belowuserFont6UserFont6FontListsee UF6 belowuserFont7UserFont7FontListsee UF7
### numFonts


Specifies the number of System/User Font alias pairs presented in the Font
dialog. The default is 7.
### systemFont[1-7]


Specifies a SystemFont alias for a particular font size that can be selected
in the Font dialog.

The default system font aliases are:SF1     -dt-interface system-medium-r-normal-xxs*-*-*-*-*-*-*-*-*:
SF2     -dt-interface system-medium-r-normal-xs*-*-*-*-*-*-*-*-*:
SF3     -dt-interface system-medium-r-normal-s*-*-*-*-*-*-*-*-*:
SF4     -dt-interface system-medium-r-normal-m*-*-*-*-*-*-*-*-*:
SF5     -dt-interface system-medium-r-normal-l*-*-*-*-*-*-*-*-*:
SF6     -dt-interface system-medium-r-normal-xl*-*-*-*-*-*-*-*-*:
SF7     -dt-interface system-medium-r-normal-xxl*-*-*-*-*-*-*-*-*:
### userFont[1-6]


Specifies a UserFont for a particular font size that can be selected
in the Font dialog.

The default user fonts are local specific. For English language-US
the defaults are:UF1     -dt-interface user-medium-r-normal-xxs*-*-*-*-*-*-*-*-*:
UF2     -dt-interface user-medium-r-normal-xs*-*-*-*-*-*-*-*-*:
UF3     -dt-interface user-medium-r-normal-s*-*-*-*-*-*-*-*-*:
UF4     -dt-interface user-medium-r-normal-m*-*-*-*-*-*-*-*-*:
UF5     -dt-interface user-medium-r-normal-l*-*-*-*-*-*-*-*-*:
UF6     -dt-interface user-medium-r-normal-xl*-*-*-*-*-*-*-*-*:
UF7     -dt-interface user-medium-r-normal-xxl*-*-*-*-*-*-*-*-*:
### Color and backdrop resources: Global to


The following resources are used by more than one desktop
component. The syntax for specifying global resources is:*resource_id.NameClassTypeDefaultcolorUseColorUseintdynamic*dynamicColorDynamicColorBooleantrueforegroundColorForegroundColorintdynamic*shadowPixmapsShadowPixmapsintdynamic*writeXrdbColorsWriteXrdbColorsBooleantrue

NOTE: The display type determines default.
### ColorUse


Specifies the number of colors to use for the user interface. The
default value for this resource is dependent on the number of bit
planes available in the display. However, to reduce the number of col-
ors used by the desktop, the default color use for a high color
display is MEDIUM_COLOR. Valid values are:`B_W"`- Specifies a black and white system. The color palettes
use only Black and White (color cells 0 and 1) and icons are displayed
as bitonal images. In this configuration four color palettes are
available: Black, White, BlackWhite and WhiteBlack. These palettes do
not dynamically change. To change a palette, the session must be
restarted. This resource value forces shadowPixmaps to True, and
foregroundColor to either black or white depending on the palette
chosen.`LOW_COLOR"`- Specifies a low color system. The color palettes use
2 color sets and icons are displayed as bitonal images. The number of
color cells can be further reduced by using the resources
shadowPixmaps and foregroundColor.`MEDIUM_COLOR`- Specifies a medium color system. The color
palettes use 4 color sets and icons are displayed as bitonal images.
The number of color cells can be further reduced by using the
resources shadowPixmaps and foregroundColor.`HIGH_COLOR`- Specifies a high color system. The color palettes
use 8 color sets and icons are displayed as multi-color images. The
number of color cells can be reduced by using the resourcesshadowPixmapsandforegroundColor.

planes

2-3

4

6

8
### dynamicColor


This resource can have values of True or False.dynamicColoris used
to reduce the number of color cells being used. Once a palette has
been selected and it is not likely to be changed,dynamicColorcan be
set to False. If set to False colors cannot be dynamically changed
using the Style Manager. A selected palette will take effect the next
session. The next time the session comes up, the color server uses
Read Only color cells that can be shared by all clients, thus reducing
the number of color cells used.
### foregroundColor


This resource can have values of White, Black or Dynamic.foregroundColorcauses all text (foreground) to use either pixel 0 or
1 (Black or White) or to have a color cell dedicated to foreground
that changes in response to the background color (Dynamic) for each
ColorSet. If set to White or Black, the number of color cells used per
ColorSet is reduced by 1.
### shadowPixmaps


For color systems, this resource can have a value of True or False. If
True,topShadowColorandbottomShadowColoruse the same pixel as
background andtopShadowPixmapandbottomShadowPixmapare specified
instead of solid color to create the 3D look. This reduces the number
of color cells per ColorSet by 2.shadowPixmapsdefaults to True for
systems with 4 or less color planes (16 or less color cells), and
False for systems with more than 4 color planes.
### writeXrdbColors


This resource should only be used if non desktop Motif clients are to
be run that have color schemes that conflict with the desktop colors.
This resource has no impact on clients linked with the desktop Motif
library. This resource specifies whether color resource information
should be written out. If set to False, background and foreground
resources are not written out with the values of the current palette.
This means that the above-mentioned clients do not get the desktop
colors when they are started. The default value is True.
### Color resources: client specific


The following resources are specified on a per client basis. The
syntax for specifying client-specific resources is:
client_name_or_class*resource_id.NameClassTypeDefaultprimaryColorSetIdPrimaryColorSetIdint3secondaryColorSetIdSecondaryColorSetIdint4
### primaryColorSetId


This resource specifies the primary color for an application. The
primary color is used for the main background areas of the application
and all children of the main area. The value of this resource is a
number from one to eight that represents a specific color set in a
palette.
### secondaryColorSetId


This resource specifies the secondary color for an application. The
secondary color is used for the menubar and all menus and dialogs of
the application. This allows dialogs on the screen to be visually
associated with their parent applications by matching the dialog color
to the menubar. The value of this resource is a number from one to
eight that represents a specific color set in a palette.
### Color resources: dtwm specific


The following are dtwm-specific resources.NameClassTypeDefaultactiveColorSetIdActiveColorSetIdint1inactiveColorSetIdInactiveColorSetIdint2
### activeColorSetId


Specifies the active frame color fordtwm. The value of this resource
is a number from one to eight, which represents a specific color set
in a palette.
### inactiveColorSetId


Specifies the inactive frame color for dtwm. The value of this
resource is a number from one to eight, which represents a specific
color set in a palette.
### Resources Saved to xrdb by the Style Manager


The following resources are written out to xrdb by the style manager.
### *HelpColorUse


color use mode for help.
### *ColorUse


color use mode.
### *ColorPalette


value of current color palette
### *MonochromePalette


value of the current palette if it is monochrome
### *background


motif resource
### *foreground


motif resource
### Dtwm.keyboardFocusPolicy


motif resource
### Dtwm*focusAutoRaise


motif resource
### Dtwm*moveOpaque


motif resource
### Dtwm*useIconBox


motif resource
### *multiClickTime


motif resource
### *enableBtn1Transfer


when set to true button 2 performs adjust operation at the next
session
### *systemFont: <systemFont[1-7]>


font displayed in labels and other non-editable text areas.
### *userFont: <userFont[1-7]>


font displayed in text widgets.
### *FontList: <systemFont[1-7]>


motif resource.
### *XmText*FontList: <userFont[1-7]>


motif resource.
### *XmTextField*FontList: <userFont[1-7]>


motif resource.
### *Font: <systemFont[1-7]>


motif resource.
### *FontSet: <systemFont[1-7]>


font resource.
### *DtEditor*textFontList: <userFont[1-7]>


dtpad resource.
### *imServerHosts: <host_list>


A comma-separated list of input method server hosts
### *preeditType: <method>


Methods areOnTheSpot,OverTheSpot,OffTheSpot, andRoot. For details,
refer to the description of the`XmNpreeditType`resource in theVendorShellman page.
## ENVIRONMENT

### DTSCREENSAVERLIST


This environment variable specifies the names of the available screen
saver actions (separated by a blank space). Using this variable allows
the style manager to query the actions data base and display a list of
the available screen saver clients for the user and to invoke screen
saver actions for preview of selected screen savers.
## BROADCAST MESSAGES


None.
## PROPERTIES AND SELECTIONS


The style manager uses X properties to communicate with the session
manager and the window manager.
## DIAGNOSTICS
This will replace your home session with the current
session. Continue?Startup dialog - Message appears in a warning dialog when the user
pushes Set Home Session...A palette named '%s' already exists. This new palette will
overwrite the old one. Is this what you want to do?Add Palette dialog - Message appears in a warning dialog when an
existing palette name is specified.Delete palette '%s'?Delete Palette dialog - Message appears in a warning dialog when a
palette is being deleted.The new double-click time will take effect as applications
are restarted. Other mouse values take effect immediately.Mouse dialog - Message appears in a warning dialog after the user
selects a new double- click time and presses OK (and the resource
*WriteXrdbImmediate is True).The new double-click time will take effect at your next
session. Other mouse values take effect immediately.Mouse dialog - Message appears in a warning dialog after the user
selects a new double- click time and presses OK (and the resource
*WriteXrdbImmediate is False).The right and left mouse buttons will switch function
immediately after you push OK below. Look at the mouse visual in Style
Manager Mouse window for indication of current handedness.Mouse dialog - Message appears in a warning dialog after the user
selects the left- handed or right-handed toggle to change mouse
handedness.The function of the middle button will switch at your next
session if you push OK below. Note that when the middle button is set
to perform the adjust operation, the transfer operation is integrated
with the select button by holding down the select button and dragging.Mouse dialog - Message appears in a warning dialog after the user
selects the transfer or adjust toggles to change the function of the
middle button.The new Color Use value will take effect at your next
session.ColorUse dialog - Message appears in a warning dialog when the color
use mode is changed and OK is pushed.The selected palette will take effect at your next session.Color dialog - Message appears in a warning dialog when the desktop is
running in black and White mode and a new palette is selected in the
Color dialog.
## ERROR MESSAGES
You must select an item within the Style Manager.

Main window - Message appears in an error dialog when On Item Help is
being used and the user clicks on an area outside of the style manager
main window area.Couldn't open bitmap directory '%s'.

Backdrop dialog - Message appears in an error dialog when the bitmap
directory could not be loaded.There are no backdrop icons available in '%s'. See your
System Administrator or the User's Guide for more details.

Backdrop dialog - Message appears in an error dialog when the bitmap
directory is empty.The palette name cannot contain these characters: :() [ ]
{ } < > ! | ' / \

Color dialog- Messages appear in an error dialog when the specified
palette name is not acceptable.The palette name must be 10 characters or less.

Color dialog- Messages appear in an error dialog when the specified
palette name is not acceptable.The color portion of the Style Manager will not operate
because the color server is not running. Check $HOME/.dt/errorlog.

Color dialog- Messages appear in an error dialog when the color icon
is selected and the color server is not running for some reason.The color portion of the Style Manager will not operate
because the resourceuseColorObjis set to False.

Color dialog- Messages can be caused to appear in an error dialog by
setting the resource*useColorObjto False and then running dtstyle
and trying to post the Color Dialog.The color portion of the Style Manager will not operate
because there are no palette files available. Check
$HOME/.dt/errorlog.

Color dialog- Messages appears in an error dialog when the color icon
is selected and there are no palette files to be found.The new`XmNpreeditType`value will take effect as applications
are restarted.Message appears in a warning dialog when the user presses the`OK`button, and the resource*writeXrdbImmediateis True.Your selection will take effect at your next session.Message appears in a warning dialog when the user presses the`OK`button after selecting a newInput Method Server, or
modifying theIMS Start Mode, or modifying
theXmNpreeditTyperesource value and the resource*writeXrdbImmediateis False.
## FILES


* **Icon files** 

Icons used in the style manger.
* **App-default file** 

Dtstyle - contains application
resources for the style manager.
* **Palette files** 

Files that contain the default and
customized palettes.
* **Backdrop file** 

Files containing the data for backdrop
customization.
* **/usr/dt/app-defaults/locale/Dtstyle** 


* **/usr/dt/appconfig/icons/locale/DtI18N.{_m.bm,pm,bm}** 


* **/usr/dt/lib/nls/msg/locale/dtstyle.cat** 



## RESTRICTION
Only one single copy ofdtsyleis permitted to run per CDE session.
Attempts to start a second copy ofdtsylewill fail and log an error
to the$HOME/.dt/errorlogfile.
## SEE


* **Dtsession** 

Acts as a color server and runs the screen
saver clients.
* **Dtwm** 

The window manager can be restarted when certain
environment customization operations are performed so that changes can
take effect.
