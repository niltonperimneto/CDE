
# File Manager Concepts


For a better understanding of File Manager, you may want to read the
following topics.












# Hierarchical File System
hierarchical file systemfile: system, hierarchicaldefinition: file

If you are new to computers, the idea of a hierarchical file system may
also be new to you. This topic describes the basic components of the
hierarchical file system.
# What's a File?


A`file`is a container that holds information. Most of the
files you use contain information (data) in some particular
format&emdash;a document, a spreadsheet, a chart. The format is
the particular way the data is arranged inside the file.
The format of a file is known as its data type.

When File Manager is in one of its icon-view modes, you can identify the
data type of a file by the icon used to represent the file.
Each data type has a different icon.

Most application programs understand a limited number of data types. For
example, a document editor probably cannot read a spreadsheet file.
The desktop helps you recognize different types of files using adata typedatabase. A data type identifies
the files of a particular format and associates them with the appropriate
applications. In most cases, when you
double-click on a file, the desktop will automatically launch
the application that understands that file's data type.

The maximum allowable size of a file name varies from system to system.
Some operating systems do not allow file names longer than 14 characters.
If necessary, consult your system administrator.
# What's a Folder?
definition: folder

Afolderis a container for files, similar to a folder in
a file cabinet. In fact, File Manager uses a folder icon to represent a
folder. A folder can contain other folders&emdash;
sometimes calledsubfolders. With folders and subfolders,
you can create multiple layers of organization that form a hierarchy.
(in other contexts, folders are often referred to as directories.)

If you drew a picture of the folder hierarchy with each subfolder
underneath the folder that contains it&emdash;its parent
folder&emdash;and drew a line from each folder to its parent, the
picture would look like an upside-down tree. Therefore, we often call
the folder hierarchy a foldertree.

Within any single folder, each file name must
have a unique name. However, files
in different folders can have the same name.

As you navigate from folder to folder, your current location is
referred to as thecurrent folder.
# What's a Path?
definition: pathpath: defined

The location of a file is often specified
by listing the names of the folders and
subfolders that lead to the file&emdash;this list is called apath.
(See.) A file's path is
visible in two places in File Manager. First it is shown in
the iconic path as a string of folders. Second, it is shown in
text form in the text path line above the view area.
# Paths and Path Names


The path to an object is a way to specify where the object is located
in the file system. There are three ways to specify the path:
absolute path, relative path, and fully qualified path.
# Absolute Paths


A path is anabsolutepath if it begins at theroot folder.
The root folder is the highest folder in the hierarchical folder tree.
If a path begins with a slash
(/), it is an absolute path specified from the root folder. For
example, the following is an absolute path to the fileletter:/usr/dt/config/letter
# Relative Paths


A path isrelativeif it describes the location of a file or folder
as it relates to the current folder.
If you are in a folder and you want to move down the folder tree,
you don't need to type the full path name. You
can just type the path starting with the name
of the next folder in the path.
If a path does not begin
with a slash, it is a relative path. For example, if the current
folder is/usr/dt,
and you want to move to the folder ''/usr/dt/config/letters,''
you would use the following relative path:config/letters.. (parent folder)

Two special folder names are useful when specifying relative paths.
The.folder (sometimes called "dot") represents the current
folder. The..folder (sometimes called "dot-dot") represents
the`parent`folder&emdash;the folder one level up in the folder
hierarchy. For example, if your current folder is/usr/dt/config/panels, then the relative path to thesys.dtwmrcfile becomes:../sys.dtwmrc

because the file is in the/usr/dt/configfolder, one level
above the current folder.
# See Also







# Object Ownership and Security


Three groups of users can access objects:owner,group, andother.
Access is divided into three functions:readpermission,writepermission,
andexecutepermission.
# Who Has Access?


The three basic classes of users are:

* **Owner** 

Usually the person who created the file.
* **Group** 

Several users that have been grouped together by
the system administrator. For example, the
members of a department might belong to the same
group.
* **Other** 

All other users on the system.

# What Kind of Access?


The access permissions on a file specifies how that file can be accessed by
the owner, members of the group, and other users.

* **Read Permission** 

Allows access to copy or view the
contents of the object.
* **Write Permission** 

Allows access to change the contents
of the object or remove the object.
* **Execute Permission** 

For a file, allows access torunthe file (for executable files,
scripts, and actions). For a
folder, allows access to run
commands, scripts, and actions
within that folder.


With File Manager, you can view and change the access permissions for
files or folders.
Seeand.
# Examples


To make a folder private:

Change the folder's properties, giving yourself (the owner) read,
write, and execute permission, but giving no permissions for group and
other. This means that only you and the root user can view the
contents of the folder.

To make an object that you've created available for everyone to use, but
protect it so it isn't inadvertently overwritten:

Change the file's properties, giving read and execute permission to
owner, group, and other. Don't give anyone write permission.
# Default Permissions


The default permissions used when you create a new file or folder may be
altered by your system administrator. To determine what your current
defaults are, create a new file or folder, then Select
Change Permissions from the Selected menu to view the
default permissions.
# Making Objects More Accessible - Introducing &newline; Workspace Objects
workspace objectsworkspace: objects

File Manager provides a way to view all the objects in your file
system. However, the object is only visible when you are viewing the
folder it is in.

To make an object more accessible, you can put it directly on the
workspace backdrop.
The workspace is that area or surface on which windows appear to lie.
(See.) When an object is placed there,
it is called aworkspace object.

Placing an object on the workspace does not alter the original file or
folder. In fact, the icon that appears on the desktop is really just
a shortcut (link) for accessing the real file or folder. Any operation you
perform on the workspace object is actually performed on the file or
folder it represents.
# Workspace Objects Appear in One Workspace


When you place an object on the workspace, it appears only in the current
workspace. If you want the object in other workspaces, you must switch
to those workspaces and put the object on them.
# Using Workspace Objects


You use workspace objects exactly like the objects inside
the File Manager or Application Manager windows.
To execute an object's default action, double-click
its icon on the desktop.

Each workspace object also has a pop-up menu containing commands
and actions for the object. To display the pop-up menu for a workspace
object
using the mouse, point to the icon, then press and hold mouse button 3.
To display the menu through the keyboard, press Alt+Tab until
the icon is highlighted, then press Shift+F10.
# Matching Patterns for Finding Files
wildcards, matchingmatching wildcards

When you specify a file or folder name, you can include wildcard
characters such as asterisk (*) and question mark (?). The*matches any string of zero or more characters, and?matches any single character.
# Examples


* **ba*** 

Matches all names that begin with the stringba
* **ba?** 

Matches all three letter names that begin with the stringba
* ***.vf** 

Matches all names that end with the.vfextension
* ***.???** 

Matches all names that have a three-character dot extension


The file name and contents can be specified using the same regular
expression syntax allowed by thefindcommand. (Refer to thefind (1)man page for more information.)
# Using File Manager as an Icon Browser


Files with names that end in.pmor.bmcontain icon drawings.
These are the icons that File Manager uses when it builds icons. By default,
you must open these files to see the drawings they contain. If you enable
icon browsing, File Manager will make the icon for each file look like
whatever drawing is stored inside the file.

To find out how to reconfigure File Manager for icon browsing, see:



