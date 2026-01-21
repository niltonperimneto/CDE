dtwmrcspecial filedtwmrcthe
dtwm Window Manager Resource Description FileDESCRIPTIONThedtwmrcwindow manager is a supplementary resource
file that controls much of the behavior of the CDE window managerdtwm. It contains descriptions of resources that cannot easily be
written using standard X Window System, Version 11 resource syntax. The resource
description file contains entries that are referred to by X resources in
defaults files (for example,/usr/dt/app-defaults/$LANG/Dtwm)
or in theRESOURCE_MANAGERproperty on the
root window. For example, the resource description file enables you to specify
different types of window menus; however, an X resource is used to specify
which of these window menus thedtwmshould use for a particular
window. The specifications of the resource description file supported by the
dtwm workspace manager are a strict superset of the specifications supported
by the OSF Motif Window Manager (mwm 1.2.4). In other words,
thesystem.mwmrcor$HOME/.mwmrcfile
that you've used formwmis easily made usable bydtwm. To merge your personal mwm resource specifications into dtwm:Copy either/etc/dt/config/$LANG/sys.dtwmrcor/usr/dt/config/$LANG/sys.dtwmrcto$HOME/.dt/dtwmrc.Use a text editor to move menu definitions, key bindings, and button
bindings from your.mwmrcfile to yourdtwmrcfile. To merge your key and button bindings, you'll need to augment
the key and button bindings that are referenced bydtwmby default (refer tobuttonBindingsandkeyBindingsresources in thedtwmapp-defaults file/usr/dt/app-defaults/$LANG/Dtwm). To replace the key or button
bindings, you'll also need to modify your personalkeyBindingsandbuttonBindingsresources.Restartdtwm.LocationThe workspace manager searches for one of the following resource description
files, where$LANGis the value of the language environment
on a per-user basis:$HOME/.dt/$LANG/dtwmrc $HOME/.dt/dtwmrc /etc/dt/config/$LANG/sys.dtwmrc
/etc/dt/config/sys.dtwmrc /usr/dt/config/$LANG/sys.dtwmrc /usr/dt/config/sys.dtwmrcThe first file found is the first used. If no file is found, a
set of built-in specifications is used. A particular resource description
file can be selected using theconfigFileresource. The following shows how a different resource description file can
be specified from the command line:/usr/dt/bin/dtwm -xrm "Dtwm*configFile: mydtwmrc"Resource TypesThe following types of resources can be described in the dtwm resource
description file:ButtonsWorkspace manager functions can be bound (associated) with button events.KeysWorkspace manager functions can be bound (associated) with key press
events.MenusMenu panes can be used for the window menu and other menus posted with
key bindings and button bindings.DTWM RESOURCE DESCRIPTION FILE SYNTAXThedtwmresource description file is a standard
text file that contains items of information separated by blanks, tabs, and
new lines characters. Blank lines are ignored. Items or characters can be
quoted to avoid special interpretation (for example, the comment character
can be quoted to prevent it from being interpreted as the comment character).
A quoted item can be contained in double quotes (" ").
Single characters can be quoted by preceding them by the back-slash character
(\), except for workspace names, which may contain no back-slash
characters. If a line ends with a back-slash, the next line is considered
a continuation of that line. All text from an unquoted#to the end of the line is regarded as a comment and is not interpreted as
part of a resource description. If!is the first character
in a line, the line is regarded as a comment.Workspace Manager FunctionsWorkspace manager functions can be accessed with button and key bindings,
and with workspace manager menus. Functions are indicated as part of the specifications
for button and key binding sets, and menu panes. The function specification
has the following syntax:function=function_name[function_args]function_name=workspace manager functionfunction_args= {quoted_item|unquoted_item}The following functions are supported. If a function is specified that
isn't one of the supported functions then it is interpreted bydtwmasf.nop.f.actionThis function causes the specifiedactionto be invoked
by means of the message server.f.beepThis function causes a beep.f.circle_down[icon|window]This function causes the window or icon that is on the top of the window
stack to be put on the bottom of the window stack (so that it is no longer
obscuring any other window or icon). This function affects only those windows
and icons that are obscuring other windows and icons, or that are obscured
by other windows and icons. Secondary windows (that is, transient windows)
are restacked with their associated primary window. Secondary windows always
stay on top of the associated primary window and there can be no other primary
windows between the secondary windows and their primary window. If aniconfunction argument is specified, then the function
applies only to icons. If awindowfunction
argument is specified then the function applies only to windows.f.circle_up[icon|window]This function raises the window or icon on the bottom of the window
stack (so that it is not obscured by any other windows). This function affects
only those windows and icons that are obscuring other windows and icons, or
that are obscured by other windows and icons. Secondary windows (that is,
transient windows) are restacked with their associated primary window. If
aniconfunction argument is specified then
the function applies only to icons. If anwindowfunction argument is specified then the function applies only to windows.f.create_workspaceThis function creates a new workspace. The new workspace name is generated
automatically and is of the formws_nwherenis an integer.f.delete_workspaceThis function deletes the current workspace. Windows that reside only
in this workspace will be moved to the next workspace. If the last workspace
is being deleted, then windows will be moved to the first workspace.f.execcommand(or!command)This function causescommandto be
executed (using the value of the$MWMSHELLor$SHELLenvironment variable if set; otherwise,/usr/bin/sh). The!notation can be used in place of thef.execfunction name.f.focus_colorThis function sets the colormap focus to a client window. If this function
is done in a root context, then the default colormap (setup by the X Window
System for the screen wheredtwmis running) is installed
and there is no specific client window colormap focus. This function is treated
asf.nopifcolormapFocusPolicyis not explicit.f.focus_keyThis function sets the keyboard input focus to a client window or icon.
This function is treated asf.nopifkeyboardFocusPolicyis not explicit or the function is executed in a root context.f.goto_workspaceworkspaceThis function causes the workspace manager to switch to the workspace
named byworkspace. If no workspace exists by the specified
name, then no action occurs. Note that adding and deleting workspaces dynamically
and affect this function.f.help[topic[volume]]This function displays help on the specifiedtopicandvolume. If novolumeis given,
then the workspace manager volume is assumed. If notopicis given, then help on the front panel is shown.f.help_modeThis function causes the workspace manager to enter into help mode.
In help mode, the pointer changes shape to indicate that the window manager
is waiting for you to select a front panel control. Any help defined for the
control is then shown in a help window.f.killThis function is used to close application windows. The actual processing
that occurs depends on the protocols that the application observes. The application
lists the protocols it observes in theWM_PROTOCOLSproperty
on its top level window. If the application observes theWM_DELETE_WINDOWprotocol, it is sent a message that requests the window be deleted.
If the application observes bothWM_DELETE_WINDOWandWM_SAVE_YOURSELF, it is sent one message requesting the window
be deleted and another message advising it to save its state. If the application
observes only theWM_SAVE_YOURSELFprotocol,it is sent a message advising it to save its state. After a
delay (specified by the resourcequitTimeout), the application's
connection to the X server is terminated. If the application observes neither
of these protocols, its connection to the X server is terminated.f.lower[-client|within|freeFamily]This function lowers a primary window to the bottom of the global window
stack (where it obscures no other window) and lowers the secondary window
(transient window or dialog box) within the client family. The arguments to
this function are mutually exclusive. Theclientargument indicates the name or class of a client to lower. The name or class
of a client appears in theWM_CLASSproperty on the
client's top-level window. If theclientargument is not specified, the context that the function was invoked in indicates
the window or icon to lower. Specifyingwithinlowers the secondary window within the family (staying above the parent) but
does not lower the client family in the global window stack. SpecifyingfreeFamilylowers the window to the bottom of the global windows
stack from its local family stack.f.marquee_selectionThis function is only useful in conjunction with the CDE file manager
(see &cdeman.dtfile;). It enables selection of file manager objects
that have been placed on the root window. It must be bound to a button when
used.f.maximizeThis function causes a client window to be displayed with its maximum
size. Refer to themaximumClientSize,maximumMaximumSize, andlimitResizeresources in &cdeman.dtwm;.f.menumenu_nameThis function associates a cascading (pull-right) menu with a menu
pane entry or a menu with a button or key binding. Themenu_namefunction argument identifies the menu to be used.f.minimizeThis function causes a client window to be minimized (iconified). When
a window is minimized with no icon box in use, and if thelowerOnIconifyresource has the value True (the default), the icon is placed
on the bottom of the window stack (such that it obscures no other window).
If an icon box is used, then the client's icon changes to its iconified form
inside the icon box. Secondary windows (that is, transient windows) are minimized
with their associated primary window. There is only one icon for a primary
window and all its secondary windows.f.moveThis function initiates an interactive move of a client window.f.next_cmapThis function installs the next colormap in the list of colormaps for
the window with the colormap focus.f.next_key[icon|window|transient]This function sets the keyboard input focus to the next window/icon
in the set of windows/icons managed by the workspace manager (the ordering
of this set is based on the stacking of windows on the screen). This function
is treated asf.nopifkeyboardFocusPolicyis not explicit. The keyboard input focus is only moved to windows that do
not have an associated secondary window that is application modal. If thetransientargument is specified, then transient (secondary)
windows are traversed (otherwise, if onlywindowis specified, traversal is done only to the last focused window in a transient
group). If aniconfunction argument is specified,
then the function applies only to icons. If awindowfunction argument is specified, then the function applies only to
windows.f.next_workspaceThis function causes the workspace manager to switch to the next workspace.
If the last workspace is currently active, then this function will switch
to the first workspace.f.nopThis function does nothing.f.normalizeThis function causes a client window to be displayed with its normal
size. Secondary windows (that is, transient windows) are placed in their normal
state along with their associated primary window.f.normalize_and_raiseThis function causes a client window to be displayed with its normal
size and raised to the top of the window stack. Secondary windows (that is,
transient windows) are placed in their normal state along with their associated
primary window.f.occupy_allThis function causes the associated window to be placed in all workspaces.f.pack_iconsThis function is used to relayout icons (based on the layout policy
being used) on the root window or in the icon box. In general this causes
icons to be "packed" into the icon grid.f.pass_keysThis function is used to enable/disable (toggle) processing of key bindings
for workspace manager functions. When it disables key binding processing all
keys are passed on to the window with the keyboard input focus and no workspace
manager functions are invoked. If thef.pass_keysfunction
is invoked with a key binding to disable key binding processing the same key
binding can be used to enable key binding processing.f.post_wmenuThis function is used to post the window menu. If a key is used to post
the window menu and a window menu button is present, the window menu is automatically
placed with its top-left corner at the bottom-left corner of the window menu
button for the client window. If no window menu button is present, the window
menu is placed at the top-left corner of the client window.f.prev_cmapThis function installs the previous colormap in the list of colormaps
for the window with the colormap focus.f.prev_key[icon|window|transient]This function sets the keyboard input focus to the previous window/icon
in the set of windows/icons managed by the workspace manager (the ordering
of this set is based on the stacking of windows on the screen). This function
is treated asf.nopifkeyboardFocusPolicyis not explicit. The keyboard input focus is only moved to windows
that do not have an associated secondary window that is application modal.
If thetransientargument is specified, then
transient (secondary) windows are traversed (otherwise, if onlywindowis specified, traversal is done only to the last focused window
in a transient group). If aniconfunction
argument is specified then the function applies only to icons. If anwindowfunction argument is specified then the function
applies only to windows.f.prev_workspaceThis function causes the workspace manager to switch to the previous
workspace. If the first workspace is currently active, then this function
switches to the last workspace.f.quit_mwmThis function terminates dtwm (but NOT the X window system).f.raise[-client|within|freeFamily]This function raises a primary window to the top of the global window
stack (where it is obscured by no other window) and raises the secondary window
(transient window or dialog box) within the client family. The arguments to
this function are mutually exclusive. Theclientargument indicates the name or class of a client to lower. If theclientis not specified, the context that the function
was invoked in indicates the window or icon to lower. Specifyingwithinraises the secondary window within the family but does not
raise the client family in the global window stack. SpecifyingfreeFamilyraises the window to the top of its local family stack
and raises the family to the top of the global window stack.f.raise_lower[within|freeFamily]This function raises a primary window to the top of the global window
stack if it is partially obscured by another window; otherwise, it lowers
the window to the bottom of the window stack. The arguments to this function
are mutually exclusive. Specifyingwithinraises a secondary window within the family (staying above the parent window),
if it is partially obscured by another window in the application's family;
otherwise, it lowers the window to the bottom of the family stack. It has
no effect on the global window stacking order. SpecifyingfreeFamilyraises the window to the top of its local family stack, if obscured
by another window, and raises the family to the top of the global window
stack; otherwise, it lowers the window to the bottom of its local family stack
and lowers the family to the bottom of the global window stack.f.refreshThis function causes all windows to be redrawn.f.refresh_winThis function causes a client window to be redrawn.f.removeThis function causes a client window to be removed from the current
workspace. If the client window exists only in this workspace, no action
occurs.f.resizeThis function initiates an interactive resize of a client window.f.restoreThis function restores the previous state of an icon's associated window.
If a maximized window is iconified, thenf.restorerestores
it to its maximized state. If a normal window is iconified, thenf.restorerestores it to its normalized state.f.restore_and_raiseThis function restores the previous state of an icon's associated window
and raises the window to the top of the window stack. If a maximized window
is iconified, thenf.restore_and_raiserestores it to
its maximized state and raises it to the top of the window stack. If a normal
window is iconified, thenf.restore_and_raiserestores
it to its normalized state and raises it to the top of the window stack.2f.restartThis function causes dtwm to be restarted (effectively terminated and
re-executed). Restart is necessary fordtwmto incorporate
changes in both thedtwmrcfile and X resources.f.screen[next|prev|back|screen_number]This function causes the pointer to be warp to a specific screen number
or to thenext,previous, or last visited (back)
screen. The arguments to this function are mutually exclusive. Thescreen_numberargument indicates the screen number
that the pointer is to be warped. Screens are numbered starting from screen
0. Specifyingnextcause the pointer to warp
to the next managed screen (skipping over any unmanaged screens). Specifyingprevcause the pointer to warp to the previous managed
screen (skipping over any unmanaged screens). Specifyingbackcause the pointer to warp to the last visited screen.f.send_msgmessage_numberThis function sends anXClientMessageEventof type_MOTIF_WM_MESSAGESwithmessage_typeset tomessage_number.
The client message is sent only ifmessage_numberis included in the client's_MOTIF_WM_MESSAGESproperty. A menu item label is grayed out if the menu item is used to dof.send_msgof a message that is not included in the client's_MOTIF_WM_MESSAGESproperty.f.separatorThis function causes a menu separator to be put in the menu pane at
the specified location (the label is ignored).f.set_behaviorThis function causes the workspace manager to restart with the default
behavior (if a custom behavior is configured) or a custom behavior (if a
default behavior is configured). By default this is bound toShift Ctrl Alt <Key>!.f.titleThis function inserts a title in the menu pane at the specified location.f.toggle_frontpanelIf the front panel is in the normal state, this function causes it to
be minimized. If the front panel is minimized, this function will change it
to the normal state.f.versionThis function causes the workspace manager to display its release version
in a dialog box.f.workspace_presenceThis function displays the workspace presence (or "Occupy Workspace")
dialog box. This dialog allows you to view and set the workspace in which
a particular window resides. The root context is disallowed for this function.Function ConstraintsEach function may be constrained as to which resource types can specify
the function (for example, menu pane) and also what context the function
can be used in (for example, the function is done to the selected client window).
Function contexts are:rootNo client window or icon has been selected as an object for the function.windowA client window has been selected as an object for the function. This
includes the window's title bar and frame. Some functions are applied only
when the window is in its normalized state (for example,f.maximize) or its maximized state (for example,f.normalize).iconAn icon has been selected as an object for the function.If a function is specified in a type of resource where it is not supported
or is invoked in a context that does not apply then the function is treated
asf.nop. The following table indicates the resource
types and function contexts in which workspace manager functions apply.