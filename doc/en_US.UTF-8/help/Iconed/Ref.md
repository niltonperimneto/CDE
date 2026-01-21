
# Icon Editor Reference











# Icon Search Paths
search paths: for iconsicons: search paths

The search path for image files is defined by two environment variables:

The system uses DTUSERAPPSEARCHPATH and DTAPPSEARCHPATH unless
the following variables exist:

* **DTUSERICONSEARCHPATH** 

A personal variable. If used, it
should be defined in/`HomeDirectory`/.dtprofile.
* **DTICONSEARCHPATH** 

A system-wide variable.

# Default Search Path


The default value for DTICONSEARCHPATH is:/`HomeDirectory`/.dt/icons/etc/dt/appconfig/icons/C/usr/dt/appconfig/icons/C
# To Modify the Personal Icon Search Path
adding: directory to actions search pathdirectory: adding to actions search pathaction: search path, adding directorysearch path, actionspath: actions search pathDTACTIONSEARCHPATH environment variableenvironment variable: DTACTIONSEARCHPATH

To add a directory to the search path,
edit/`HomeDirectory`/.dtprofile. Add a line defining a value for
the DTUSERICONSEARCHPATH:DTUSERICONSEARCHPATH=`path`[,`path`...]
# To Modify the System-wide Icon Search Path


System-wide search path variables are defined in files located in
the/etc/dt/Xsession.ddirectory.

If DTICONSEARCHPATH is already defined in a file in this directory,
edit the value to add the new directory on the search path.

If DTICONSEARCHPATH is not already defined in this directory, define
it to include the default search path plus others you want to add.
(The default search path is listed in comments in the file/usr/dt/bin/dtsearchpath.)
# Icon Editor Tools
tools: Icon EditorIcon Editor: toolstools: drawing

When you select one of Icon Editor's tools, it remains selected until you
select another one. The tools are listed and described below:

Pencil&newline;For drawing freehand lines and individual pixels.

Line&newline;For drawing straight lines.

Rectangle&newline;For drawing solid or outlined rectangles.

Circle&newline;For drawing solid or outlined circles.

Eraser&newline;For erasing large areas of the image.

Flood&newline;For flooding a region of one color with the
selected color.

Open Polygon&newline;For drawing connected straight lines.

Closed Polygon&newline;For drawing connected straight lines,
where the first line and last line are connected to form a closed polygon.

Ellipse&newline;For drawing solid or outlined ellipses.

Selection&newline;For making a primary selection. Several
commands in the Edit menu require a primary selection first.
# Fill Solids


Select the Fill Solids check box
below the tool palette to change the rectangle, polygon,
circle, and ellipse tools from outlines to solids in the tool palette.
(See also).
# Icon Editor Menus











# Icon Editor File Menu




* **New** 

Prompts for a size (width and height, in pixels), and then
creates a blank drawing area of that size.
(See.)
* **Open** 

Opens an existing bitmap or pixmap file.
(See.)
* **Save** 

Saves the current icon using its existing name. If the
icon is unnamed, Icon Editor prompts you for a name.
(See.)
* **Save As** 

Prompts for a new name, and then saves the current icon.
(See.)
* **Exit** 

Closes Icon Editor. If you have unsaved changes, Icon
Editor warns you that the changes will be lost if you
continue and exit. To save the changes, choose
Cancel, and then save your icon.)

# Icon Editor Edit Menu




* **Undo** 

Cancels the most recent drawing operation, returning the
icon to its previous state. Most operations within Icon
Editor can be undone.
(See.)
* **Cut Area** 

Cuts the selected area from the icon. The region removed
is filled with the transparent color. The cut region is
saved in the Icon Editor clipboard.
(See.)
* **Copy Area** 

Copies the selected area into Icon Editor's clipboard.
(See.)
* **Paste Area** 

Pastes the contents of the clipboard into the icon.
(Seeor.)
* **Rotate Area** 

Rotates the selected area to the left or right 90
degrees. Choose the direction from the submenu.
(See.)
* **Flip Area** 

Creates a mirror image of the selected area by flipping it
vertically or horizontally. Choose the direction
from the submenu.
(See.)
* **Scale Area** 

Enables you to resize (or "scale") the selected area.
(See.)
* **Resize Icon** 

Prompts for a new size for the current icon.
(See.)
* **Add Hotspot** 

Enables you to indicate a single pixel in the current
icon as the "hotspot." After choosing Add Hotspot,
click the pixel you want to be the hotspot.

Hotspots are used when
creating images for mouse pointers -- the hotspot
represents theactuallocation of the pointer.
(See.)
* **Delete Hotspot** 

Removes the hotspot from the current icon.
* **Grab Screen Image** 

Grabs an area of the screen and loads it
into the drawing area.
(See.)
The X-Y Size display above the work area shows the size
in pixels of the area being grabbed.
* **Clear Icon** 

Clears the current drawing area.
(See.)

# Icon Editor Options Menu




* **Visible Grid** 

Toggles the grid lines on and off. The default is
on.
* **Output Format** 

Determines the output format in which the icon will be saved:

XBM for two-color X bitmap format. Bitmap files
are normally identified by a.bmfile-name
suffix.

XPM for multicolor X pixmap format (the
default). Pixmap files are normally identified
by a.pmfile-name suffix.

See.
* **Magnification** 

Changes the viewing size of the image in the
drawing area. You can choose magnification
factors from 2x (twice normal size) to 12x (twelve
times normal size).

# Icon Editor Help Menu




* **Overview** 

Displays the introduction help topic for Icon Editor
* **Tasks** 

Displays task instructions for most Icon Editor
operations
* **Reference** 

Displays reference summaries for various Icon Editor
features such as windows and dialog boxes, menus, and
resources
* **On Item** 

Presents a description of the item in an Icon Editor
window you have selected
* **Using Help** 

Provides help on using the help windows
* **About the Icon Editor** 

Displays the version and copyright information for
Icon Editor

# Icon Editor Windows and Dialog Boxes

# Subtopics







# Icon Editor Main Window


The Icon Editor main window has five important areas:

Thestatus linejust below the menu bar describes the
tool that is currently selected and the coordinates of the pixel
currently pointed to by the pointer.

Thedrawing areais where you draw the icon image.

Thetool paletteprovides several drawing
tools, including an eraser and a selection tool.

Thecolor paletteprovides drawing colors:
eight static colors, eight static grays, and six dynamic colors.

Theactual size icons areashow what your icon looks like
in actual size. It shows both the full-color and the two-color versions.
# Icon Editor Color Palettes




* **Static Colors** 

Eight standard colors; black,
white, the three primary colors,
and the three secondary colors
* **Static Grays** 

Eight shades of gray (from
10% to 90% gray)
* **Dynamic Colors** 

Six dynamic colors that respond
when you change colors using
Style Manager


You may also want to refer to these topics in the Style Manager Help Viewer:

To
Select a Palette

To
Change the Number of Colors Used by the Desktop.
# Icon Editor Open and Save As Dialog Boxes


* **Enter path or folder name:** 

Type the full path name of the folder
containing the icon you want to open or the folder
where you want to save the icon.
* **Folders** 

Displays a list of the folders that are inside the folder shown
in the "Enter path or folder name" text field.
If you double-click on a folder in this list, the Folder and Files lists will
change to show the contents of that folder. Or you can select a folder in the
Folder list and then click on the Update button.
* **Files** 

Displays a list of the files contained in the folder that is shown
in the "Enter path or folder name" field. If you change the name in the Enter
path or folder name field, you must click the Update button to get the Files
list to show the new list of files.
* **Enter file name:** 

Displays the name of the icon that will be loaded or saved.
The easiest way to specify the icon you want is to double-click on the
icon name in the Files list. Or, you can type in the name for the icon you want
and then click on the Open button.

Note that
the correct format for icon names is name.size.format. The size and format
information must be in the icon name to make icons work properly. Icon editor
should automatically fill in the correct size and format values based on the
size and output format you chose from the menu bar in the editor.
* **Open or Save** 

Opens or Saves the file and closes the dialog box.
* **Update** 

Changes the Folders and Files lists to show the contents of
the folder the is shown in the Enter path or folder name field. You can type
the folder name in the field and then click the Update button. Or, if the
folder you want to open is shown in the Folders list, just double-click on its
name.
* **Cancel** 

Cancels the open or save and closes the dialog box.
* **Help** 

Displays help about this dialog box.

# Icon Editor save As Dialog Box




* **Enter path or folder name:** 

Type the full path name of the folder
where you want to save the icon.
* **Folders** 

Displays a list of the folders that are inside the folder shown
in the "Enter path or folder name" text field.
If you double-click on a folder in this list, the Folder and Files lists will
change to show the contents of that folder. Or you can select a folder in the
Folder list and then click on the Update button.
* **Files** 

Displays a list of the files contained in the folder that is shown
in the "Enter path or folder name" field. If you change the name in the Enter
path or folder name field, you must click the Update button to get the Files
list to show the new list of files.
* **Enter file name:** 

Type the name for the icon you want to save. Note that
the correct format for icon names is name.size.format. The size and format
information must be in the icon name to make icons work properly. Icon editor
should automatically fill in the correct size and format values based on the
size and output format you chose from the menu bar in the editor.
* **Save** 

Saves the file and closes the dialog box.
* **Update** 

Changes the Folders and Files lists to show the contents of
the folder the is shown in the Enter path or folder name field. You can type
the folder name in the field and then click the Update button. Or, if the
folder you want to open is shown in the Folders list, just double-click on its
name.
* **Cancel** 

Cancels the save and closes the dialog box.
* **Help** 

Displays help about this dialog box.

# Icon Editor Confirmation Dialog Box


The confirmation dialog box protects you from inadvertently losing data by
verifying that you want to perform the command (such as exiting
Icon Editor without saving your icon).

Click OK to continue, or Cancel to cancel the command.