# dtfile
user cmddtfilethe CDE File Managerdtfileoptions...
## DESCRIPTION
Althoughdtfilemay be invoked from the command line, developers
who wish to interface to File Manager function should use CDE Actions
(see "CDE Actions" in "RELATED INFORMATION").

The CDE File Manager is the Desktop's
primary interface to the file system. It provides a GUI for file
manipulation and application execution.

The main File Manager interface consists of a top-level window which
shows the contents of a single folder or a set of nested folders
(folder tree). A single File Manager process may provide many
top-level windows each of which may show the contents of a different
folder.

The File Manager has many manipulation features. It will allow a user
to:

- Traverse folders via a double-click, menu options, or a
click-and-type mechanism.

- Change folder display modes (i.e. large icons, small icons,
alphabetical order, date order, single folder, folder tree,
etc.) and filtering options.

- Create, move, copy, link, and delete objects via menu options
and/or drag-and-drop.

- Rename objects via menu options or a click-and-type mechanism.

- Modify object file system attributes via menu options.

- Invoke actions (applications such as Edit or Print) on objects
via a double-click, menu options, or drag-and-drop.

- Locate objects by name or by contents.

- Place frequently used objects on a desktop/workspace.

The File Manager also supports additional features for two special
types of top level windows: application views and the trash container.

Application views are designed to organize application objects such as
actions.

Application view windows have the same general appearance and function
as folder windows with the following exceptions:

- The user is not allowed to traverse above the main application
view.

- When a user drags an object from an application view to another
application view or to a folder, the object is copied to the new
location (as opposed to being moved as is the case in a folder.)

The trash container is designed to be a holding area for objects that
users wish to delete. Like application views, the trash container
window has the same general appearance as a folder window; however,
trash container function differs as follows: Users may move, restore,
or delete a trash object and may change the display mode for the trash
container. Users MAY NOT:

- Traverse out of the trash container.

- Copy, rename, or execute any other manipulation on a trash
object.
### Key Supported Tasks



### Desktop


A Desktop is a place where users can store commonly used objects for
easy access. Within the CDE, each workspace is
considered a Desktop. A user may place any object on the background of
a workspace and the object will remain there. The user can also select
actions for that object via a popup menu which is activated by
selecting mouse button 3.
### Folder Window


A folder window is a top-level window which shows the contents of a
single folder or a set of nested folders (folder tree).
Folder windows may be launched from the Front Panel, actions, or a
command line interface. The objects in a folder are displayed in a
user-selected icon/text combination. The icon visual shows the icon
type. Users can use menu options to reread folder views, open new
folder views, and to select/unselect all objects in a folder. In tree
mode, buttons next to icons representing subfolders allow users to
expand or collapse folder branches and to control whether all
objects in a folder or only the folder structure is shown in the
window.
### Application View Window


A application view window is a top-level window which shows the
contents of a single application folder. An application folder
is a restricted folder which is designed to organize application
objects such as actions. Application views are launched from the Front
Panel or a predefined action.
### Trash Container Window


When an object is no longer needed, a user can move the object to the
trash container. The trash container is a restricted folder which
is used to store files temporarily until the user asks to permanently
remove them. A user can view the contents of the trash container in a
top level window which is launched from a menu option in a folder or
application view, the Front Panel, or a predefined action. Users can
also restore objects from the trash container provided that actions
have not been taken to permanently remove them.
### Object Movement and Modification


Menu options are provided which allow users to create objects, rename
objects, copy objects, modify object properties, and move objects to a
desktop or to the trash container. (NOTE: In addition to the menu
options, users can rename objects with the following steps: 1. Click
mouse button 1 on the text portion of an object icon, 2. An edit box
will appear; type the new file name, 3. Select Enter.)
### File System Awareness


The File Manager is file system aware. It can be enabled to launch
dialogs that allow editing of file system specific parameters (See
FILES -dtfile.config).
### Object Search


A menu option/dialog box combination is provided to allow users to
search for objects by name or by content. During a name search, the
File Manager looks for an object whose name matches specified search
criteria. During a content search, the File Manager looks for an
object which contains specified character strings. Content searches
are case insensitive. When an object is located, the File Manager can
either open a window to display the folder where the object is
located or place the object on the desktop.
### Folder Traversal


Simple folder traversal can be accomplished by double-clicking on a
folder icon. This action displays the contents of the selected folder.
If the user moves the mouse before releasing the mouse button on the
second click of a double click ("double-click-drag" action), a new
window will pop up to show the contents of the selected folder. The
new window will be placed at the position where the user released the
mouse button.

In addition, the File Manager provides the following folder
traversal mechanisms:

* **** 

This dialog lists folders that are frequently used.
A user may traverse to a new folder by double-clicking on
a folder in this list.
* **Click-and-Type Mechanism** 

When a user clicks on the "Text Path",
it is remapped to a text edit widget. Again the user may
traverse to a new folder by typing a new folder title and
then selecting Enter.
* **Iconic Path Selection** 

If a user double-clicks on a folder icon in the iconic
path, the File Manager will update the current window to show
the contents of the selected folder.
* **Find Dialog** 

This dialog searches the file system for a folder or
folders whose name(s) match user-supplied criteria. The
user may open new windows for folders located by the
Find dialog.
* **Home and Up Menu Options** 

These options allow the user to traverse
to $HOME and parent folders respectively.
* **Tree Display Mode** 

In this mode, the folder hierarchy is displayed as
a tree that can be traversed by selecting the buttons located
next to the folder icons (See Tree Mode below)
* **Tree Mode** 

In tree mode, the current folder and its subfolders are
shown initially. Traversing into subfolders can be
accomplished by single-clicking on the button next to the
folder icon. Clicking the left mouse button over the folder
button cycles through three states: partially expanded, fully
expanded, and collapsed. Clicking the middle mouse button
cycles through these three states in reverse order. When the
user first clicks on the folder button, the contents of the
subfolder are read and added to the tree (partially
expanded state). If the user clicks on the button a second
time, the non-folder contents of the folder are added to
the tree (fully expanded state). If the user clicks on the button
a third time, the folder contents are removed from the tree
(collapsed state). If a folder does not have subfolders,
the partially expanded state is skipped. If a folder does not
contain non-folder objects, the fully expanded state is
skipped. If a folder is empty, the folder button is
desensitized.

### Setting Display Preferences


The File Manager provides many options for displaying objects. For
example, objects can be displayed in iconic or non-iconic formats. The
user may also choose to position files in either a grid or random
pattern. (NOTE: The File Manager provides a Clean Up menu option which
can be used to align randomly placed objects to a grid pattern.) If
the user has write permission in a folder and the user chooses to
randomly place files, the positioning information is saved when the
user leaves the folder. Therefore, each time the user reenters the
folder the files will be positioned as they were when the user left
the folder.
### Object Filtering


Folders can contain many different types of objects. The File
Manager's filtering mechanism allows the user to selectively display
objects depending on their type. The File Manager also provides a Show
Hidden Files menu toggle option which allows the user to turn the
filtering mechanism on and off.
### Object Type/Action Association


When an object is selected, the File Manager builds an Actions menu
for that object based on the type of the object. For example, data
files are provided with a menu containing the following actions: Open,
Print. Folders are provided with a menu containing: OpenNewView,
OpenInPlace. Actions can be run on an object by selecting an object
and then selecting an action from the Actions menu. Also,
double-clicking on an object will activate the default action for the
object.
### Quick Help


The File Manager provides quick help via F1 throughout thedtfileapplication. This includes quick help on object icons. To access quick
help, position the cursor/pointer over the item for which you wish to
get help, then press F1.
### Popup Menus


The File Manager provides popup menus for objects within a File
Manager view or on the Desktop. To access the popup menu for a
particular object, position the cursor over the object and press and
hold mouse button 3. A popup menu will appear. The menu will contain a
list of the actions which can be performed on that particular object.
If multiple objects are selected, the popup menu will show all of
those actions which can be done on all of the selected objects at
once.
### Direct Manipulation


Objects can be dragged between different
File Manager views, to Desktops, and to cooperating clients.
Direct manipulation may be used to copy, move, or link objects as well
as to supply objects as input to applications.
See the REGISTERING OBJECTS AS DROP SITES section.
The File Manager also supports drops of buffers on its views and objects
such as CDE Mailer attachments.
### Terminal Access


A menu option is provided which gives users access to terminal
windows.
### Exit Services


Menu options are provided which allow users to close File Manager
views and which allow users to save setting information before exiting
a File Manager session.
### Object Name Completion


The File Manager supports object name completion in the following text
widgets: the "Text path", the "Destination Folder" field of
the "Go To" dialog, the "Search Folder" field of the "Find"
dialog. Press control+spacebar and the name will complete to the extent
that it can.
### Multibyte Character Set Support


The File Manager is fully internationalized. It supports both
single-byte and multi-byte locales.
### Error Condition Notification


The File Manager uses dialog boxes to report various error conditions.
## OPTIONS


The following options are available from the command line:
### -noview


Dtfileruns in "server mode". In other words, a window is not
displayed until a cooperating client makes a request to display a
folder.
### -sessionsession_file


Dtfileruns with the session file specified in the session_file
parameter. Session files are generated as adtfilesession shuts down.
### -dirfolder


OR
### -folderfolder


Dtfiledisplays a window for each folder specified in the folder
parameter (Note: No spaces are allowed in folder). If this option is
not used, the user's current folder (the folder from whichdtfilewas started) is displayed.
### -titletitle_name


This option allows the user to specify a title (title_name) for the
File Manager windows. All windows and dialogs generated by this
session will use this title. If this option is not used, the name of
the folder shown in a window will be used as the window title.
### -help_volumehelp_volume_name


This option allows the user to specify a help volume
(help_volume_name) to use with the File Manager windows. This option
is useful if the user is using the File Manager to display a specific
folder and wants to have specific help for that folder. Note:
All File Manager views that are created from this folder will also
use this help volume.
### -restricted


Dtfilewill not display folders above the restricted folder. If
the-diroption is used, the folder specified by that option is the
restricted folder. If the-diroption is not used, the user's
current folder is the restricted folder.
### -grid on/off


Dtfiledisplays files in a pattern specified by the on/off parameter.
As the name implies, on/off will either have a value ofonor a value
ofoff.

* **on** 

Files are always displayed in a grid pattern. The
File Manager automatically rearranges the files if a file is moved.
This is the default pattern.
* **off** 

Files are always displayed where they are placed
by the user. The File Manager does not rearrange the files.

### -tree on/off


This option controls whetherdtfiledisplays files in single folder
mode (off) or in folder tree mode (on).
### -tree_filesnever/always/choose


This option controls whether or not files may be displayed in the
folder tree mode.

* **never** 

Files are never displayed. The user may toggle
between two states: partially expanded and collapsed.
* **always** 

Files are always displayed. The user may toggle
between two states: fully expanded and collapsed.
* **choose** 

Files may or may not be displayed. The user may
toggle between three states: partially expanded, fully expanded, or
collapsed.

### -orderorder_type


Dtfiledisplays files in the order specified by the order_type
parameter. Order_type must have one of the following values:

* **alphabetical** 

Files are displayed in alphabetical
order. This is the default order.
* **file_type** 

Files are displayed in groups based on their
filetypes.
* **date** 

Files are displayed in an order based on the date
when they were last modified.
* **size** 

Files are displayed in an order based on their size.

### -viewview_type


Dtfiledisplays files in the format specified by the view_type
parameter.View_typemust have one of the following values:

* **no_icon** 

Files are displayed by name.
* **large_icon** 

Files are displayed by name and large icon.
The icon shows the type of the file. This is the default format.
* **small_icon** 

Files are displayed by name and small icon.
The icon shows the type of the file.
* **attributes** 

Files are displayed by attributes. A small
icon is used to represent the file type. (This format is similar to
the listing obtained by issuing ls -l from an aixterm command line.)

### -directiondirection


Dtfiledisplays files in the direction specified by the direction
parameter. Direction must have one of the following values:

* **ascending** 

Files are displayed in an ascending
direction. This is the default direction.
* **descending** 

Files are displayed in a descending
direction.

Both the-noviewand the-sessionoptions are normally used by the
session manger to startdtfile. The-title,-help_volume, and-restrictedoptions can also be set via resources (See the RESOURCES
section.) The-grid,-tree,-treefiles,-order,-view, and-directionoptions can also be set by 1) selecting the Set Preferences option
from the View pulldown menu or by 2) resources (See the RESOURCES
section.)

### -small_icon_widthsize


The default small icon width is 24 pixels. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this option to specify small icon width. Size must be in pixel.
### -small_icon_heightsize


The default small icon height is 24 pixels. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this option to specify small icon height. Size must be in pixel.
### -large_icon_widthsize


The default large icon width is 38 pixels. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this option to specify large icon width. Size must be in pixel.
### -large_icon_heightsize


The default large icon height is 38 pixels. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this option to specify large icon height. Size must be in pixel.
## EXAMPLES

### dtfile -view no_icon -order date -direction


The File Manager will display files in text format, ordered by date,
most current to oldest date.
### dtfile -dir /u/guest -restricted


The File Manager will begin execution by displaying a window showing
the /u/guest folder. The user will not be allowed to navigate above
this folder.
### dtfile -session session.jan.12


The File Manager will run with the session file called session.jan.12.
## RESOURCES
NameClassTypeDefaultdirWidthDirWidthXmRDimension555dirHeightDirHeightXmRDimension305appWidthAppWidthXmRDimension365appHeightAppHeightXmRDimension365rereadTimeRereadTimeXmRInt10 (s)checkBrokenLinkCheckBrokenLinkXmRInt180 (s)maxDirectoryProcessesMaxDirectoryProcessesXmRInt10maxRereadProcessesMaxRereadProcessesXmRInt5maxRereadProcsPerTickMaxRereadProcsPerTickXmRInt1moveThresholdMoveThresholdXmRInt4 (pix)titleTitleXmRStringNULLrootTitleRootTitleXmRString"ROOT"help_volumeHelp_volumeXmRStringNULLfileManagerIconFileManagerIconXmRStringhome.i.bmrestrictModeRestrictModeXmRBooleanfalseshowFilesystemShowFilesystemXmRBooleantruegridGridstringontreeViewTreeViewstringofftreeFilesTreeFilesstringneverorderOrderstringalphaviewViewstringlarge_icondirectionDirectionstringascenddesktopIconDesktopIconstringlargeobjectPlacementObjectPlacementstringtop rtopenFolderOpenFolderstringcurrentsmallIconWidthSmallIconWidthXmRInt16smallIconHeightSmallIconHeightXmRInt16largeIconWidthLargeIconWidthXmRInt32largeIconHeightLargeIconHeightXmRInt32
### Dtfile *dirWidth


Specifies the width of a File Manager folder window.
### Dtfile *dirHeight


Specifies the height of a File Manager folder window.
### Dtfile *appWidth


Specifies the width of a File Manager application view window
### Dtfile *appHeight


Specifies the height of a File Manager application view window.
### Dtfile *rereadTime


Determines how often the File Manager rereads the open folders and
monitors the Desktop objects. This resource must be specified in
seconds. If it is set to 0, the reread will not occur and the user
must manually reread the folders. This will conserve processor
cycles but may cause views to become stale as well.
### Dtfile *checkBrokenLink


Determines how often the File Manager checks open folders for
broken links. This resource must be specified in seconds. If it is set
to 0, the check for broken links will not occur.
### Dtfile *maxDirectoryProcesses


Specifies the maximum number of background processes which may be
devoted to folder activities (ie. reading the folder, updating
the folder, writing positional information for the folder,
checking for broken links or checking for other folder updates).
### Dtfile *maxRereadProcesses


Specifies the maximum number of background processes which may be
devoted to folder reread activities (ie. checking for broken links
or checking for other folder updates).
### Dtfile *maxRereadProcsPerTick


Specifies the number of reread processes that may be started per
reread timer tick. In other words, the currently displayed folders
will be checked for changes in a round- robin fashion, x folders
per timer tick where x is equal to maxRereadProcsPerTick.
### Dtfile *moveThreshold


Specifies the number of pixels that the cursor must move while a
button is held down before the drag controller recognizes the button
down action as a drag.
### Dtfile *title


Specifies the title for all File Manager windows. If this resource is
set to NULL, then the title of each File Manger window will be the
name of the folder displayed in the window.
### Dtfile *rootTitle


Specifies the title of the root folder. This title will appear in
the title bar of any File Manager window which is showing the root
folder. It will also be the name shown with the icon representing
the root folder on the Desktop.
### Dtfile *help_volume


Specifies the help volume.
### Dtfile`*fileManagerIcon`


Specifies the icon to display when a File Manager window is minimized.
### Dtfile`*restrictMode`


Determines whether or not the user is operating in restricted mode. If
this resource is set to true, the user is operating in restricted
mode. The user is restricted to the user's $HOME folder and below.
All folder change requests are interpreted relative to the user's
$HOME folder. All folders above the user's $HOME folder are
hidden from the user. If this resource is set to false, the user's
folder requests are not limited in any way.
### Dtfile`*showFilesystem`


Determines whether or not the path name for the current folder is
visible. If this resource is set to TRUE, the path name is visible. If
it is set to FALSE, the path name is not visible. This resource can be
used to hide the file system from the user.
### Dtfile`*grid`


Determines the pattern used to display files. This resource must have
one of two values:

* **on** 

Files are always displayed in a grid pattern. The
File Manager automatically rearranges the files if a file is moved.
This is the default pattern.
* **off** 

Files are always displayed where they are placed
by the user. The File Manager does not rearrange the files.

### Dtfile`treeView`


Determines whether files are displayed in single folder mode (on)
or folder tree mode (off).
### Dtfile`*treeFiles`


* **This** 

folder tree mode.
* **never** 

Files are never displayed. The user may toggle
between two states: partially expanded and collapsed.
* **always** 

Files are always displayed. The user may toggle
between two states: fully expanded and collapsed.
* **choose** 

Files may or may not be displayed. The user may
toggle between three states: partially expanded, fully expanded, or
collapsed.

### Dtfile`*order`


Determines the order used to display files. This resource must have
one of four values:

* **alphabetical** 

Files are displayed in alphabetical
order. This is the default order.
* **file_type** 

Files are displayed in groups based on their
filetypes.
* **date** 

Files are displayed in an order based on the date
when they were last modified.
* **size** 

Files are displayed in an order based on their
size.

### Dtfile`*view`


Determines the format used to display files. This resource must have
one of four values:

* **no_icon** 

Files are displayed by name.
* **large_icon** 

Files are displayed by name and large icon.
The icon shows the type of the file. This is the default format.
* **small_icon** 

Files are displayed by name and small icon.
The icon shows the type of the file.
* **attributes** 

Files are displayed by attributes. A small
icon is used to represent the file type. (This format is similar to
the listing obtained by issuing ls -l from an aixterm command line.)

### Dtfile`*direction`


Determines the direction used to display files. This resource must
have one of two values:

* **ascending** 

Files are displayed in an ascending
direction. This is the default direction.
* **descending** 

Files are displayed in a descending
direction.

### Dtfile`*desktopIcon`


Determines the type of icon used to represent files/folders on the
Desktop. This resource must have one of two values:

* **large** 

Large icons are used to represent
files/folders on the Desktop.
* **small** 

Small icons are used top represent
files/folders on the Desktop.

### Dtfile *objectPlacement


Specifies the placement scheme used to place objects on the Desktop.
This resource has the following syntax: primary_layout
secondary_layout. The primary_layout determines whether an object
placed on the Desktop is put into a row or a column. The
secondary_layout determines where to place new rows or columns. The
layouts must have one of four values:

* **top** 

Objects are placed top to bottom. (vertical value)
* **bottom** 

Objects are placed bottom to top. (vertical
value)
* **left** 

Objects are placed left to right. (horizontal
value)
* **right** 

Objects are placed right to left. (horizontal
value)
The user should use one value from each
category--vertical and horizontal. For example if the primary_layout
is top, then the secondary_layout should be either left or right. In
this case, the secondary_layout should NOT be top or bottom. Likewise,
if the primary_layout is left, then the secondary_layout should be
either top or bottom. In this case, the secondary_layout should NOT be
left or right.

### Dtfile`*openFolder`


Determines how a folder is opened when a user double clicks on a
folder icon. This resource must have one of two values:

* **current** 

Open the folder in the current window.
(This is the same as using the OpenInPlace action.)
* **new** 

Open the folder in a new window or, if a window showing this
folder currently exists, move the existing window to this
workspace and place it in the forefront. (Note: Only one view
of a folder can be opened at once under these conditions.
If the user defines an 'open folder' file type, the folder icon
will change state to indicate when a given folder is currently
open somewhere in the user's environment. In order to define
an 'open folder' file type, the user should 1) define a file
type with the same name as the normal folder file type
except that the name is preceded withOPEN_, 2) create an
'open folder' icon, and 3) use the name of the 'open folder' icon
as the icon name in the 'open folder' file type definition.

### Dtfile`*smallIconWidth`


The default small icon size is 16 pixel. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this resource to specify small icon width.
### Dtfile`*smallIconHeight`


The default small icon height is 16 pixel. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this resource to specify small icon height.
### Dtfile`*largeIconWidth`


The default large icon width is 32 pixel. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this resource to specify large icon width.
### Dtfile`*largeIconHeight`


The default large icon height is 16 pixel. Any customized icons
that have larger size will be clipped. To avoid clipping,
use this resource to specify large icon height.
## ASYNCHRONOUS EVENTS

### Tool Talk Messages


The following Tool Talk Desktop and Media requests
are supported by the File Manager:

* **Edit (Document and Media Exchange Message Set)** 

The specified file is opened for editing in a separate window.
* **Display (Document and Media Exchange Message Set)** 

The specified file is opened for viewing only in a separate window.
* **Quit (Desktop Message Set)** 

This request will terminate the File Manager and its children.

## RELATED INFORMATION

### CDE Actions


The following Desktop actions can be used to access File Manager
folder and application view windows as well as the File Manager trash
container window. These actions are defined in/usr/dt/appconfig/types/$LANG/dtfile.dt.

* **Dtfile** 

Opens a folder window for a specified folder.
* **Dtappmgr** 

Opens an application view window.
* **Dttrash** 

Opens a window to display the contents of the Trash Container.
* **OpenNewView** 

Opens a new window to show the contents of the selected folder.
* **OpenInPlace** 

Displays the contents of the selected folder in the current window.
* **FILESYSTEM_MOVE** 

Moves a set of dragged objects to the selected drop site.
* **FILESYSTEM_COPY** 

Copies a set of dragged objects to the selected drop site.
* **FILESYSTEM_LINK** 

Creates symbolic links to a set of dragged objects
from the selected drop site.

### Registering Objects As Drop Sites


Every CDE data type has three associated drop
attributes:MOVE_TO_ACTION,COPY_TO_ACTION,
andLINK_TO_ACTION. The File
Manager registers every object whose data type has a value for at
least one of these attributes as a drop site.

When an object is dragged to a drop site, the File Manager is
triggered by the drag-and-drop API. The drag-and-drop API provides the
File Manager with a gesture code (Move, Copy, or Link) which
is dictated by a combination of the modifier keys used and the manner
in which the drop site was registered. Based on the gesture code and
the data type of the drop site, the File Manager retrieves a drop
attribute (ie.MOVE_TO_ACTION) from the Datatypes database. The File
Manager then calls the CDE API,DtActionInvokeaccording to the following rules:

1. If objects A and B are dropped on object C, then the parameters toDtActionInvokeare as follows:

* **DtActionInvoke (drop action name, C, A, B)** 

2. There is one exception, if object C is an action, then the
parameters will not include object C. ie
* **DtActionInvoke (drop action name, A, B)** 



## FILES


File Manager uses the following files.
### dtfile


Executable file. This file is located in/usr/dt/bin.
### dtfile_copy


Utility to support folder copy. This executable file is located in/usr/dt/bin.
### dtfile_error


This script can be used by applications to display an error
dialog when it would be difficult or impossible to do in the
context of the executing program. For example, it can be used
when exec fails in a child process or if an error is detected
before an applications main window can be realized.
This executable file is located in/usr/dt/bin.
### Dtfile


App-defaults file. This file is located in/usr/dt/app-defaults/$LANG.
See the RESOURCES section for a list of those resources which can be
set using the app-defaults file.
### dtfile.config


Configuration file. This file is located in/usr/dt/config/$LANG. It may be
used to add a file system specific button to the File Properties
dialog. This button launches an additional dialog allowing the user to
modify file system specific parameters. For example, the user can add
a stanza defining a dialog for setting the Access Control List for a
file from the Andrew File System. If the user also adds a stanza
defining a dialog for setting the Access Control List for the
Distributed File System, then the File Manager will determine which
dialog is associated with the selected file and will display that
dialog. See the text in thedtfile.configfile for instructions on how
to create stanzas.
### dtfile.dt


CDE type/action definition file. This file is
located in/usr/dt/appconfig/types/$LANG.
### Dtinf.*, Dtlink.*, DtdirR.*, Fphome*, Fpapps*, Fptrsh*


Icon files. These files are located in/usr/dt/appconfig/icons/$LANG.
### dtfile.cat


Message catalog. This file is located in/usr/dt/lib/nls/msg/$LANG.
### Filemgr.sdl, FM*.tif, FM*.pm


Help files. These files are located in/usr/dt/appconfig/help/$LANG/Filemgrand/usr/dt/appconfig/help/$LANG/Filemgr/graphics.
### dtfile.1


Man page.
## SEE ALSO


- Data Interchange Mechanisms (drag-and-drop library)

- Window Manager

- Object Services (action/filetype database)

- ICCCM Messaging

- ToolTalk