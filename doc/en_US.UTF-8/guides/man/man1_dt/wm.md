dtwmuser cmddtwmThe CDE
Window ManagerDESCRIPTIONThedtwmwindow manager is an X Window System window
manager based upon the Motif window manager,mwm(version
1.2.4). It providesmwmcompatible window management functionality.
This includes functions that facilitate control (by the user and the programmer)
of elements of window state such as placement, size, icon/normal display,
and input-focus ownership.Thedtwmwindow manager is an integral part of the
CDE. It communicates with and facilitates access to other components in the
environment, such as the Session Manager ( &cdeman.dtsession;)
and the Style Manager ( &cdeman.dtstyle;). Many other components
are started through interactions with the Front Panel.In addition,dtwmprovides workspace management.
Workspaces provide a way of grouping together logically related windows. Each
workspace is shown independent of the other workspaces so that only those
windows related to the immediate task are visible. Workspaces give you a tool
to organize windows by task and make efficient use of screen real estate.Options-displaydisplayThis option specifies the display to use; seeX(1).-xrmresourcestringThis option specifies a resource string to use.-multiscreenThis option causesdtwmto manage all screens on
the display. Sincedtwmdoes this by default, this option
is of limited use. See themultiScreenresource for information
on managing a single screen.-namenameThis option causesdtwmto retrieve its resources
using the specified name, as inname*resource.-screensname [name [...]]This option specifies the resource names to use for the screens managed
bydtwm. Ifdtwmis managing a single
screen, only the first name in the list is used. Ifdtwmis managing multiple screens, the names are assigned to the screens in order,
starting with screen 0. Screen 0 gets the first name, screen 1 the second
name, and so on.StartupNormally, the session managerdtsessionstarts updtwm. You can alter the command line for the window manager via
thewmStartupCommandresource for the session manager
(see &cdeman.dtsession;). Similarly, you can affect the behavior
ofdtwmby saving resources fordtwmas part of your session.AppearanceThe following sections describe the basic default behaviors of windows,
icons, the icon box, input focus, and window stacking. The appearance and
behavior of the window manager can be altered by changing the configuration
of specific resources. Resources are defined under the heading "X DEFAULTS."ScreensBy default,dtwmmanages only the single screen specified
by the-displayoption or theDISPLAYenvironment variable (by default, screen 0). If the-multiscreenoption is specified or if themultiScreenresource is True,dtwmtries to manage all
the screens on the display.Whendtwmis managing multiple screens, the-screensoption can be used to give each screen a unique
resource name. The names are separated by blanks, for example,-screensscr0 scr1. If there are more screens than names,
resources for the remaining screens will be retrieved using the first name.
By default, the screen number is used for the screen name.WindowsDefaultdtwmwindow frames have distinct components
with associated functions:Title AreaIn addition to displaying the client's title, the title area is used
to move the window. To move the window, place the pointer over the title area,
press button 1 and drag the window to a new location. By default, a wire
frame is moved during the drag to indicate the new location. When the button
is released, the window is moved to the new location.Title BarThe title bar includes the title area, the minimize button, the maximize
button, and the window menu button. In shaped windows, such as round windows,
the title bar floats above the window.Minimize ButtonTo turn the window into an icon, click button 1 on the minimize button
(the frame box with asmallsquare in it).Maximize ButtonTo make the window fill the screen (or enlarge to the largest size allowed
by the configuration files), click button 1 on the maximize button (the frame
box with alargesquare in it).Window Menu ButtonThe window menu button is the frame box with a horizontal bar in it.
To pull down the window menu, press button 1. While pressing, drag the pointer
on the menu to your selection, then release the button when your selection
is highlighted. Pressing button 3 in the title bar or resize border handles
also posts the window menu. Alternately, you can click button 1 to pull down
the menu and keep it posted; then position the pointer and select. You can
also post the window menu by pressing <Shift> <Esc> or <Alt> <Space>.
Double-clicking button 1 with the pointer on the window menu button closes
the window.The following table lists the contents of the window menu.Default Window Menu