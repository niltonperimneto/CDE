
# Application Manager Reference
reference

This is a list of reference topics for Application Manager, arranged into various
categories.
# General Reference







# Application Manager Menus











# Application Manager Window and Dialog Boxes





















# Application Manager Mouse Reference
mouse buttonsselecting objects
# Mouse Button 1&emdash;Select and Drag


Mouse button 1 (the left button, by default) is used to select and drag
objects.

To select an object, click its icon.

To extend selection, hold down the Control key and click on another icon.
# Mouse Button 2&emdash;Drag


Dragging objects:

Moving objects: hold mouse button 1 down while dragging an icon.

Copying objects: Press Control and drag.

Symbolic link: Press Shift and drag.

On a two-button mouse, press both buttons simultaneously.
# Mouse Button 3&emdash;Menus
context menus

Mouse button 3 (the right button, by default) is used to display pop-up
menus.

Point to the object or area where the menu is available, then pressand holdmouse button 3.
# File Names in Application Manager
Application Manager:file namesfile names:in Application Manageraction labelslabels for action icons

In File Manager and Application Manager, files and folders are represented as icons,
and these icons are usually labeled with the file name.

Action icons are sometimes an exception to this rule. For example,
display the pop-up menu for the action icon in the Desktop_Tools
application group labeled Digital Clock. Notice that the file name,
which is shown at the top of the pop-up menu, is not the same
as the label.

The actual file name appears in the Copy File and Link File dialog boxes.
# Folder Location of Application Manager
Application Manager:folder location

Application Manager behaves very much like File Manager. This
is because Application Manager is a File Manager view of a special
folder on your system used to gather registered applications.
Login Manager creates the Application Manager folder
each time you log in.

Ordinarily, you do not need to know the location the Application
Manager folder. However,
it may be useful to you if you are trying
to troubleshoot problems.

Its location is:/var/dt/appconfig/appmanager/`special_folder_name`

wherespecial_folder_nameis a name assigned by the system
that is unique for your system and login name.

You shouldneverattempt to directly modify thespecial_folder_namefolder.dtappgatherReload Applications action

After Login Manager creates the folder, it runs a desktop program
nameddtappgatherthat gathers all the application groups.

During a session, you can rerundtappgatherby double-clicking
Reload Applications in the Desktop_Tools application gorup.
# Application Manager Menu Bar


The Application Manager menu bar contains:








# Application Manager File Menu


Application Manager:File menu

* **New Folder** 

Prompts for a folder name to create a new folder.
* **New File** 

Prompts for a file name to create a new file.
* **Go Up** 

Moves up one level in the folder hierarchy.
* **Go To** 

Displays the Go To dialog box, which lets you type
in a new folder name or choose one from a list of
folders you have previously gone to.
* **Find** 

Displays the Find dialog box, which lets you search for files
and folders based on file name patterns or file
contents.
* **Close** 

Closes the current Application Manager view.

# Application Manager Selected Menu


Application Manager:Selected menu

Many of the application groups can be modified only by
your system administrator. Therefore, many of these menu items
may be unavailable.

* **Move to** 

Prompts for the new full path name of
the file you want to move.
* **Copy to** 

Prompts for a new file name to create a copy of the
selected file. The Copy command is available only when
there is exactly one file selected.
* **Copy as Link** 

Prompts for the new full path name of the link that will be
created for
the selected object.
* **Put In Workspace** 

Puts the selected object into the right
corner of the workspace backdrop.
* **Put in Trash** 

Puts the selected objects in the
Trash Can.
* **Rename** 

Opens the edit name field for the
selected object.
* **Change Permissions** 

Displays the Change Permissions
dialog box for the selected
object. This dialog box shows the
permissions for the object.
* **Select All** 

Selects all objects in the current Application Manager view.
* **Deselect All** 

Deselects all objects in the current Application Manager view.
* **`actions`** 

If the selected object has any actions, they are appended to the
bottom of the menu. These are the same actions that
appear in the object's pop-up menu.

# Application Manager View Menu


Application Manager:View menuviewing preferences in Application Manager

* **Open New View** 

Opens another Application Manager view of the current folder.
* **Set View Options** 

Displays the Set View Options dialog box for changing the
appearance and behavior of the current Application Manager view.
* **Save As Default Options** 

Saves the current options, window size, and filter
list as the default.
* **Show Hidden Objects** 

Toggles the display of hidden files. You can specify which
data is hidden by choosing Set Filter Options.
* **Set Filter Options** 

Displays the Set Filter Options dialog box, which you use to
specify files that you want to be hidden (based on data type
or file name).
* **Update** 

Refreshes the current folder contents and redisplays
it with any changes.

Update does not gather applications that have been added
since you logged in. To rebuild the Application Manager's
contents, you must
double-click Reload Applications in the Desktop_Tools
application group.

# Application Manager Help Menu


Application Manager:Help menu

* **Overview** 

Displays general introductory information about Application
Manager.
* **Tasks** 

Displays specific "how to" task instructions for using
Application Manager.
* **Reference** 

Displays information on the windows, menus, and dialog
boxes in Application Manager.
* **On Item** 

Changes the display cursor to the On Item help cursor.
Then, click an item in the
Application Manager window to get help on that item.
* **Using Help** 

Provides help on using the help windows.
* **About Application Manager** 

Displays Application Manager version and copyright information.

# Application Manager Pop-Up Menu for Objects


Application Manager:pop-up menuspop-up menus:Application ManagerMost objects in Application Manager have their own pop-up menus.

At the top of each pop-up menu are two standard commands:
Put In Workspace and Put in Trash.

The actions for the object's
data type are at the bottom of the pop-up menu. These are the same
actions that appear in the Actions menu when the object's icon is selected.

* **Put In Workspace** 

Puts the object on the desktop of the current workspace.
The location of the object is determined by theobjectPlacementresource, which defaults to the
top right corner of the screen.
* **Put in Trash** 

Deletes the object to the Trash Can.
* **Help** 

Displays the help for pop-up menus.
* **Actions** 

If the object has any actions, they are appended to the
bottom of the pop-up menu.

# Application Manager Window


Application Manager:main windowThe top level of the Application Manager contains icons representing
application groups.

Each application group is a folder containing
one or moreapplication icons(application icons are also
calledaction icons). An application group may also
contain other types of application files such as
"read me" files and templates.
# Application Manager Copy File Dialog Box


Application Manager:copying objectscopying objects

* **Selected object** 

Displays the objected that will be copied.
* **Destination Folder** 

Type the new full path name of the destination folder.
* **Name for copy** 

Type the name of the new object.
* **OK** 

Performs the copy and closes the
dialog box.
* **Cancel** 

Cancels the Copy command and closes the
dialog box.
* **Show Icon** 

&newline;Shows the icon that will be used for the new file.
* **Help** 

Displays the help for this dialog box.


You can also use the mouse to copy files or folders.
# Application Manager Link File/Folder Dialog Box




* **Selected object** 

Displays the objected that will be linked.
* **Destination Folder** 

Type the new full path name of the destination folder.
* **Name for copy** 

Type the name of the new object.
* **OK** 

Performs the copy and closes the
dialog box.
* **Cancel** 

Cancels the Copy command and closes the
dialog box.
* **Show Icon** 

&newline;Shows the icon that will be used for the new file.
* **Help** 

Displays the help for this dialog box.


You can also use the mouse to link files or folders.
# Application Manager Move File Dialog Box


Application Manager:moving objectsmoving objects

* **Selected Object** 

Displays the file or folder that will be moved.
* **Destination Folder** 

Type the new full path name of the destination folder.
* **OK** 

Performs the move and closes the
dialog box.
* **Cancel** 

Cancels the Move command and closes the
dialog box.
* **Help** 

Displays help about renaming files and folders.

# Application Manager New File Dialog Box




* **New File Name** 

&newline;Type the name of the new file or folder. If the
file is being created in a different folder, you
must type the full path name of the new folder or
file you want to create.
* **OK** 

Creates the file and closes the dialog box.
* **Apply** 

Creates the file and retains the dialog box
so you can create another new file.
* **Cancel** 

Cancels the New File command and closes the dialog box.
* **Show Icon** 

If you change the file to a different file type when you type
in the New File Name, its icon may change. To see a preview
of the new icon, click Show Icon and the icon inside the
dialog box will update. For example,
if you type a name ending in.tif, then choose Show
Icon, you will see the TIFF data type icon.
* **Help** 

Displays help about this dialog box.

# Application Manager New Folder Dialog Box




* **New Folder Name** 

&newline;Type the name of the new folder. If the
folder is being created in a different folder, you
must type the full path name of the new folder
you want to create.
* **OK** 

Creates the folder and closes the dialog box.
* **Apply** 

Creates the folder and retains the dialog box
so you can create another folder.
* **Cancel** 

Cancels the New Folder command and closes the dialog box.
* **Show Icon** 

If you change the data type when you type
in the New Folder Name, its icon may change. To see a preview
of the new icon, click Show Icon and the icon inside the
dialog box will update.
* **Help** 

Displays help about this dialog box.

# Application Manager Set Filter Options Dialog Box


Application Manager:filtering objectsfiltering objects

* **Select Filetypes to be Hidden or Shown** 

This button toggles between Hidden and Shown.
A list of all the data types defined on your system
is displayed.
Selected data types will or will
not be displayed in Application Manager depending on whether
the toggle button shows shown or hidden.
* **Select All** 

Selects all data types. Unless you then unselect some,
your Application Manager viewing area will be empty.
* **Deselect All** 

Deselects all the data types.
* **Filter String (Optional)** 

Lets you filter by file name. For example, if you type*.o, then Application Manager will not display files with
names ending in.o. Note that any data type that
you type into this field is added to the list of data types
selected in the icon list in the top part of the dialog box.
* **OK** 

Applies the current filter settings and closes the dialog box.
* **Apply** 

Applies the current filter settings without closing the
dialog box.
* **Defaults** 

Restores the default filter list (which includes DOT_FILE,
DOT_FOLDER, and CURRENT_FOLDER). The filter list is not
applied until you choose Apply or OK.
* **Cancel** 

Restores the last applied settings and closes the dialog box.
* **Help** 

Displays the help for filtering objects.

# Application Manager Find Files or Folder Dialog Box


Use the Find Files or Folders dialog box to search a folder and the
folders it contains for files with a particular name or contents.

* **File or Folder Name** 

Type the name of the file or folder you want
to find. You can use wildcard characters.
* **File Contents** 

Find will search inside files and folders for the
text you type in this field.
* **Search Folder** 

Type the path of the folder where you want the
search to start. The search will start at that
folder and include all its subfolders.
* **Files Found** 

Lists the files or folders found in the search.
Double-click a file or folder in the list to open a new
Application Manager view showing that file or folder.
* **Open New View** 

Opens a Application Manager view of the folder containing the
file that is selected in Files Found. If a folder was found,
the view is the contents of that folder.
* **Put In Workspace** 

Put the selected file or folder on the backdrop of
the current workspace.
* **Start** 

Starts the search.
* **Stop** 

Stops a search in progress. Note that this button can be
used even when the hourglass cursor is shown.
* **Cancel** 

Stops a search in progress and closes the dialog box.
* **Help** 

Displays help about finding objects.

# Application Manager Change Permissions Dialog Box


Use the Change Permissions dialog box to change the read, write, and execute
permissions of files or folders you own. Only the owner of the file or
folder can change the permissions. If you do not own the file
or folder, the box will show the current settings; you cannot change
them. The Change Permissions dialog box also shows the full path name, size,
and the date and time last modified of the file or folder.

* **Owner Name** 

The name of the user that owns the object. Only a system administrator
(root user) can change the owner of an object.
* **Group Name** 

This is the name of the group of users who receive the permissions shown in the
Group row in the permissions list. If you are the owner, you can change the group
to another group you belong to. A root user can change it to any group.
* **Permissions** 

If you are the owner, you can change the read, write, and execute
permissions. Select a check box to provide the permission.
* **OK** 

Applies the current settings and closes the dialog box.
* **Cancel** 

Closes the dialog box without making any changes.
* **Help** 

Displays help about changing permissions.

# Application Manager Set View Options Dialog Box


Application Manager:view optionsview options in Application ManagerUse the Set View Options dialog box to change how files and folders
are represented in Application Manager.
# Headers


The Headers preference indicates which header lines you want to be displayed
in the Application Manager window.

* **Iconic Path** 

Displays the current folder as a chain of folder
icons.
* **Text Path** 

Displays the full path name of the current folder in a text line,
just above the main viewing area. You can click in the text
and edit the path name or double-click on one of the folder
names to change to that folder.
* **Message Line** 

Displays the total number of files, folders, and
hidden files in the current folder.



# Placement


The Placement preference indicates how you want icons arranged within the
Application Manager view.

* **As Placed** 

Objects are put exactly where you place
them.
* **Rows and Columns** 

Objects are re-sorted and arranged in
rows and columns.



# Show
folder treetree view (Application Manager)

* **By Single Folder** 

Displays the contents of the current folder.
* **By Tree** 

Displays the contents of
the current folder in the form of a tree.
* **Folders only** 

If By Tree is selected, view just shows folders.
This is the default.
* **Folders, then Files** 

If By Tree is selected, view shows folders, and
then files if you click on the +.
* **Folders and Files** 

If By Tree is selected, view shows all folders and
files.



# Representation


* **By Name Only** 

Each object is displayed only as a name.
* **By Large Icons** 

Each object is displayed with its name and a large icon.
(This is the default.)
* **By Small Icons** 

Each object is displayed with its name and a small icon.long listing format
* **By Name, date, size, ...** 

Each object is displayed in a long listing,
showing name, modification date, size, permissions, owner,
and group.



# Order


Choose the order in which files and folders are displayed:

* **Alphabetically** 

Choose Ascending (A to Z, then a to z) or Descending (Z to A, then z to a).
(The default is alphabetical, A to Z.)
* **By File Type** 

Files are grouped together according to data type.
* **By Date** 

Choose Ascending (oldest to youngest) or Descending
(youngest to oldest).
* **By Size** 

Choose Ascending (smallest to largest) or Descending
(largest to smallest).

# Direction


Choose the direction in which files and folders are displayed:

* **Ascending** 

Oldest to youngest, smallest to largest
* **Descending** 

Youngest to oldest, largest to smallest
