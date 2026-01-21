
# App Builder Tasks


&newline; &empty;


























# Managing Projects and Modules
project managementmanaging a project

Whenever you are building an interface with App Builder, you are
working on aproject, which is
comprised of one or moremodules.

A project can be opened, saved, or closed from the App Builder primary
window or in the Project Organizer. Modules can be imported or exported
from the App Builder primary window, but they can be saved, shown,
hidden, or removed from a project only in the Project Organizer.




































# To Open or Start App Builder
open App Builder from iconstart App Builder from iconstart App Builder, command lineopen App Builder, command linecommand line start
# To Open App Builder from an Icon


If App Builder has previously been open and the App Builder icon is on
the desktop, double-click on the icon to open App Builder.
&newline; &empty;Application Builder Icon
# To Open App Builder from the Front Panel


If App Builder is installed on the Front Panel, click on the App
Builder icon in the Personal Applications subpanel to open App
Builder.

To install App Builder on the Front Panel, as shown below, seeTo Put an Application Icon
in the Front Panelin the Application Manager help volume for
instructions. Click Backtrack to return to this location after reading
the instructions.
# To Start App Builder from the Command Line


The command to run App Builder isdtbuilder.Use
one of the following methods to start App Builder.

TypedtbuilderIfdtbuilderis in your path, App Builder will
start up. Ifdtbuilderis not in your path,
modify your path to include it.

Or

Type the full path name todtbuilder,
which, by default, is/usr/dt/bin/dtbuilderOr

Change to the folder wheredtbuilderis located (/usr/dt/bin/dtbuilderby default) and type

dtbuilder
# See Also



# To Exit App Builder
close App Builderexit App Builderquit App Builder

Choose Exit from the File menu to quit App Builder.

If you have not saved all changes, a message dialog box will be displayed,
giving you the opportunity to discard the changes and continue the
exit process or to cancel the exit process and continue running App
Builder. You could then save your changes and exit.
# See Also



# To Create a New Project
open new projectnew projectprojectresource fileproject namingnaming a project

Choose New Project from the File menu.

The Project Name dialog box will be displayed.

&newline; &empty;

If you have made changes to the current project since you last saved
it, a message dialog box will be displayed, giving you the option to discard
the changes and create the new project or to cancel the New Project
operation. You could then save the current project before creating the
new project.

Type a name (all lower case) for the project and click Apply.

The name of the project (with.bipadded as a
suffix) will be displayed in the title bar at the top of the App
Builder primary window. Every module you create or import will be part
of the current project -- until you open another project.

Note: Project names should be all lower case so that there is no
conflict between the name of the project resource file and the project
executable file. The name of the resource file created when you
generate code is the same as the name of the project, minus the .bip
suffix, but it is given an initial capital letter. The name of the
executable file created when you make the project is the same as the
name of the project, minus the .bip suffix, but it is made all lower
case, with an underbar preceding each capital letter (if any) after
the first. Seefor detailed information.

By default, an unnamed project is called "Untitled.bip." You can give the project a different name when you save it.
# See Also









# To Open an Existing Project
open projectproject

Choose Open Project from the File menu of the App Builder primary
window or Open from the Project menu of the Project Organizer.

The Open Project file selection dialog box will be displayed.

If you have made changes to the current project since you last saved
it, a message dialog box will be displayed, giving you the option to
discard the changes and open the other project or to cancel the Open
Project operation. You could then save the current project before
opening the other project.

Change folders, if necessary.

Seefor instructions.

Double-click the appropriate project file (.bipsuffix) in the Files list of the file selection dialog

Or select the file and click Open.

The selected project will be displayed in the Project Organizer. Seefor instructions for displaying the modules in a
project. The name of the project will be displayed in the title bar of
the App Builder primary window.
# See Also











# To Select a File in a File Selection Dialog Box
select filefile selectionfile selection dialog box

The file selection dialog box is displayed when various project and
module operations are performed. The file selection dialog box will be
titled "Open Project," "Save Project," "Import File," or "Export
Module," depending on the operation. The leftmost button at the bottom
of the dialog box will be labelled "Open," "Save," "Import," or
"Export."

Change to the folder where the file is located.

See instructions for changing folders below.

Double-click on the file name in the Files list to select it
and perform the appropriate action.

OrClick on the file name in the Files list and press the Return key or
click Open (or Save, Import, or Export, depending on what file
operation is being performed).
# How to Change Folders


Double-click on one of the folders in the Folders list to
append it to Path and to change the path to that folder.

Or

Double-click on ".." in Folders to move up one folder level.

Or

Click in the Path field to place a text cursor; you can then
type or backspace at the cursor location to designate a change in
Path. Press Return or click Update to make the change. If you
designate a non-existent folder, a warning beep will sound.

Or

Double-click on the Path field to select the entire path; you
can then type in a full path name to replace the current path. Press
Return or click Update to make the change.
# See Also


for an illustration of a file selection
dialog box and descriptions of dialog box fields.




# To Save a Project
save projectproject

A project is only saved when you explicitly choose to save it, so be
sure to save often and regularly. Seefor an
illustration of a file selection dialog box and descriptions of dialog box
fields.

Choose Save Project from the File menu of the App Builder primary
window or Save from the Project menu of the Project Organizer.

If you have saved the project before, the project will be saved
without comment.

If this is the first time you have saved the project, the Save Project
file selection dialog box will be displayed. Seefor instructions.

Change to the appropriate folder.

Note that you will normally want a separate folder for each project
you work on.

Type a file name in the Enter Filename field.

You do not have to append .bip to the project name; this is done
automatically when you save a project.

Click Save.

The project will be saved.
# See Also





# To Rename a Project (Save As)
rename projectprojectsave as, project

To save a project with a different name or in a different folder:

Choose Save Project As from the File menu of the App Builder primary
window or Save As from the Project menu of the Project Organizer.

The Save Project file selection dialog box will be displayed, with the
current project name in the Enter Filename field.

Change to another folder, if necessary.

Seefor instructions. You have to press Return
or click Update before the folder change is registered.

Modify the name or type in a new name in the Enter Filename field.

Click Save.

The project has been renamed -- in the same folder or in another
folder. The old project file will still be in the folder, but it will
not be the active project file. If you generate code for the project
and runmakein the folder, the new project
name will be used.

If you saved the project in a different folder, the contents of the
project file will be written in the new folder, and all of the
modules in the folder will have been copied to the new folder.
# See Also







# To Save a Project to a File (Encapsulate Project)
save encapsulatedsave as,  encapsulatedencapsulate file save

A project is comprised of one or more modules. Normally a project file
is saved in a file with a.bipsuffix, and each
module file is saved in a separate file with a.bilsuffix.)

To save a project as a single file (for convenience in mailing the
project to someone, for instance):

Open the project, as described in.

Choose Save Project As from the File menu of the App Builder primary
window or Save As from the Project menu of the Project Organizer.

Change to the appropriate folder, if necessary. Seefor instructions.

Click the Save As Encapsulated Project check box.

The name of the current project will be displayed in the Enter
Filename field, with a .bix (builder interface exchange) suffix.

Click Save or press Return.
# See Also





# To Close a Project
close projectproject

Select Close Project in the File menu of the App Builder
primary window.

If you have made changes since saving the project a message dialog box
will be displayed, giving you the chance to discard the changes or to
cancel the close operation.
# See Also



# To Create a New Module
new moduleopen new modulemodule

A module is only saved when you explicitly choose to save it -- and
when its project is saved; be sure to save often and regularly. Seeorfor instructions.

To create a new module, which will become part of the current project:

Choose New Module from the File menu of the App Builder primary
window or New from the Module menu of the Project Organizer.

The Module Name dialog box will be displayed, with "Untitled" selected
as the default name.

Note: If you drag and drop a window on the desktop after creating a new
project, the Module Name dialog box will be displayed, just as if you had
chosen New Module from the File menu.

In the dialog box, type in the name you wish to
give the new module.

Click Apply or press Return.

The name of the new module will appear in the Editing Module field at
the bottom of the App Builder primary window. Any windows you drag
from the Windows palette and drop on the desktop will be part of the
new module.
# 


See Also




# To Import a Module into a Project
import modulemodule

To import an existing module into the current project:

Choose Import Module from the File menu of the App Builder
primary window or Import from the Module menu of the Project
Organizer.

The Import File dialog box will be displayed.

Change to the folder where the module (.bilsuffix) file is saved.

Change Import Format type, if necessary.

By default, BIL format is selected. If the file you are importing is a
UIL file, click on the UIL radio button; the file will be converted to
BIL format when it is imported.

Change Import By method, if necessary.

By default, Import By Copy is selected. If you want to import the
module by reference rather than making a copy of it, click the Import
By Reference radio button. Note: Import By Reference, which causes
module files to be shared, can be dangerous, since the actual module
file may be changed or deleted inadvertantly.

Double-click on the module to be imported in the Files list.

OrSelect the file and click Import.

The module will be added to the current project the next time you save
the project.
# See Also



# To Save a Module
save modulemodule

All modules in a project are saved when you save the project. If you
want to save individual modules, you can do so in the Project
Organizer.

Display the Project Organizer by choosing Project Organizer
from the File menu of the App Builder primary window.

Select the module you wish to save.

Choose Save from the Module menu.

If you have saved the module previously during this App Builder
session, the module will be saved without comment.

If this is the first time you have saved the module, the Save BIL File
file selection dialog box will be displayed, with the name of the selected
module (with a .bil suffix) in the Enter Filename field.

Change to the folder where you wish to save the module, if
necessary.

Click Save or press Return.
# See Also





# To Rename a Module (Save As)
modulerename modulesave as, modulemodule

Use Save As to rename a module. When you save the current project, the
new module name will replace the old name in the project (.bip) file.
The original module will still be in the project folder, but it
will not be part of the project.

Display the Project Organizer by choosing Project Organizer
from the File menu of the App Builder primary window.

Select the module you wish to save.

Choose Save As from the Module menu.

The Save BIL File file selection dialog box will be displayed, with the
name of the selected module (with a .bil suffix) in the Enter Filename
field.

Change to the folder where you wish to save the module (if
necessary).

Skip this step if you want to save the module in the same folder, but
with a different name.

Type a file name in the Enter Filename field.

This name will replace the previous module name in the .bil file.

Click Save or press Return.

The new module name will replace the old name in the project (.bip)
file the next time you save the project. To save a module without
affecting the project, see.
# See Also









# To Export a Module
export modulemodulesave module, export

To save a version of a module in the current project folder or to save
a module separately from the current project:

From the File menu of the App Builder primary window choose
Export Module and select one of the currently-open modules from the
submenu displayed.

OrIn the Project Organizer select the module to be exported in the
module array and choose Export from the Module menu.

The Export File dialog box will be displayed, with the selected module
name in the Enter Filename field.

Type a new file name in the Enter Filename field

OrChange to the folder where you want to save the module and type a file
name in the Enter Filename field.

If you want to save a version of the module in the current folder,
do not change folders. Simply give the module a different name.

Click Export or press Return.
# See Also







# To Save a Module in UIL Format
save module, UILmodule save, UILUIL saveexport module, UIL

To save a module in UIL (User Interface Language) format:

Choose Export Module from the File menu of the App Builder primary
window and select the module you wish to export from the submenu
that is displayed.

OrIn the Project Organizer select the module to be exported and choose
Export from the Module menu.

The Export File dialog box will be displayed, with the selected module
name in the Enter Filename field.

Change to the folder where you want to save the module, if
necessary.

Click the Save As UIL check box above the Enter Filename field.

Type a file name in the Enter Filename field.

This name will replace the previous module name in the .uil file.

Click Export or press Return.

The file will be saved with a.uilsuffix.
# See Also





# To Show a Hidden Module
show modulemodule

The Project Organizer displays icons for all of the modules that
comprise a project. You can show or hide the interfaces for any of the
modules in the project. Seeandfor
more about the Project Organizer.

&newline; &empty;

Display the Project Organizer by choosing Project Organizer
from the File menu of the App Builder primary window.

Double-click on the module icons in the module array in the
Project Organizer that you wish to show (or select the module icons and
choose Show from the Module menu).

The user interfaces for the selected modules will be displayed.

Note: If a module you wish to show is in a different project, you
will first have to open the other project. Seefor
instructions.
# 


See Also




# To Hide a Shown Module
hide modulemodule

To hide a module which is displayed (to clean up the desktop so that
you can more easily work on another module, for instance):

Display the Project Organizer by choosing Project Organizer
from the File menu of the App Builder primary window.

Select the modules in the module array that you wish to hide.

Select one module by clicking mouse button 1 on it. Add to the
selection by clicking mouse button 2 on other modules. Select a
number of adjacent modules by drag-selecting with mouse button 1 or
mouse button 2, starting above and to the left of the first module to
be selected.

Choose Hide from the Module menu.

The user interfaces for the selected modules will be hidden.
# 


See Also




# To Remove a Module from a Project
removing module from projectmoduleproject module

Display the Project Organizer by choosing Project Organizer
from the File menu of the App Builder primary window.

Open the project that contains the module to be removed, if
necessary.

Since you can have only one project open at one time, you will have to
close the current project first. Seefor instructions.

Select the module in the module array that you wish to remove.

Choose Remove from the Module menu in the Project Organizer.

The selected module will be removed from the current project. The
module file will still exist in the project folder, but it will no
longer be part of the project. The module file name will be removed
from the project file the next time the project is saved.
# 


See Also




# Laying Out a User Interface











# To Create a Main Window or a Primary Main Window
createmain window, createprimary window, create

Your application might have multiple main windows, but only one
primary main window, which is the starting point for the application.
By default, the first main window created in the current project is
designated as the primary main window. This designation can be changed
in the.

Drag a main window from the Windows palette and drop it on the
desktop.

Seefor an illustration. If there is no
current module, the Module Name dialog box will be displayed; you will
have to give the module a name before proceeding.

Double-click on the main window to display the Revolving
Property Editor

The main window will be selected in the Revolving Property Editor.

Change the Object Name, if necessary.

Change the Window Title to something appropriate.

Type the names of an Icon File, an Icon Mask File, and an Icon
Label, if you want an icon to represent the window when it is
minimized.

The Icon File and Icon Mask File must be xpm or xbm graphics files.

Change the User Resize Mode, if appropriate.

Select Menubar, Toolbar, and Footer, as appropriate, to add
these functional areas to the window.

If you select Menubar, you will want to create menus after you
finish editing main window properties. Seefor
instructions.

If you select Toolbar or Footer, you will want to edit the
properties of the control panes that comprise these objects after you
finish editing main window properties. You can drop controls on the
control panes, make connections to programmatic actions, and do other
things that can be done to any control pane.

Change the Size Policy and Size, as appropriate.

You will probably want to leave the Size Policy as Fixed while you are
creating the application, and change it to Fit Contents as you finish
the application, for internationalization and other purposes. When
Size Policy is Fit Contents, the window will change size to
accommodate changes in the size of objects as the text in the objects
changes -- or if the font size changes, for example.

Set Initial State to Iconic if you want the application to
appear as an icon when it is started.

Set Initial State to not Visible if you want the main window to
be invisible when the application is started.

Click on the Visible check box, which is checked by default.

Set Background and Foreground colors, if appropriate.

Type in a color name if you know it or press mouse button 1 on the
Background or Foreground menu button and choose Color Chooser to
display the Color Chooser. Select a color and click OK.
Background sets the color of the blank pane area of the window.
Foreground does nothing that is visible in the completed interface.

Click Help Text to add on item help, if appropriate.

Seefor instructions.

Click OK to apply the changes and dismiss the Revolving
Property Editor.

Unless you are creating a primary main window, you are finished with
this task.

If you just created a primary main window, choose Application
Framework Editor from the Editors menu.

Type a Vendor Name and Version number in the Application
section of the Application Framework Editor, if appropriate.

Seefor details about the editor.

Click OK in the Application Framework Editor to apply the
changes and close the editor.
# See Also







# To Create a Window with a Spanning Control Pane
createwindow, create with control panespanning control pane in window

Often you will want to drop a control pane on top-left corner of the
blank pane area of a main window or custom dialog and grab the
bottom-right corner of the pane to drag it so that the pane covers the
entire blank pane area. You can then drop controls or other panes on
the control pane to create a complex window such as the App Builder
primary window.

Drag a main window or a custom dialog from the Windows palette
and drop it on the desktop.

Seefor an illustration. If you haven't
previously named the module, the Module Name dialog box will be
displayed. Type in a name and click Apply.

Drag a control pane from the Panes palette and drop it on the
top-left corner of the main window or custom dialog.

Grab the bottom-right corner of the control pane (an arrow
pointing towards a corner will be displayed) and drag the corner of
the control pane beyond the bottom-right corner of the window.

The control pane will be attached to the four sides of the
window; if you resize the window, the control pane will be resized
with it. Seefor details about attachments.

Note: Once a control pane has been resized to cover the blank pane
area, the main window or custom dialog object can only be selected in
the status region.
# To Select Interface Objects
selecting objectsobject, selectingdeselecting objectsdrag-selectmultiple-select

For many actions, including moving, aligning, and grouping, you need
to select one or more control objects in an interface. Note that you
can multiply-select "siblings" only; siblings are objects that are the
children of the same parent. All control objects in a single control
pane are siblings, for instance. Any panes that are dropped on a
control pane and created as children of the control pane are also
siblings of controls in the control pane.

Select one object by clicking on it in the interface or in the
Module Browser.

Note that selecting an object in the Module Browser also selects it in the
interface, and vice versa.

To select a number of adjacent objects, position the mouse
cursor above and to the left of the objects, press mouse button 1,
and drag the mouse to encompass other objects down and
to the right of the first object.

Add or subtract an object to what is currently selected by
clicking mouse button 2 on the object.

If an object is selected, clicking mouse button 2 on it unselects it.

To add a number of adjacent objects to those that are selected,
position the mouse cursor above and to the left of the objects to be
added, press mouse button 2, and drag the mouse to encompass other
objects down and to the right of the first object.

To deselect all but one object, click mouse button 1 on an
object.

Only that object will be selected.

Note that when you have selected a number of objects in the interface,
all the objects will move if you press mouse button 1 on one of the
objects and move the mouse. A rectangular border will be drawn around
the objects as you move the cursor.
# See Also







# To Align Objects in an Interface
align objectsobjects, align

While you are creating an interface, you may want to align the objects
in relation to each other. Use the Align function for this "static"
alignment. Ultimately, you may want to group and attach the objects
for "dynamic" alignment. Seefor
instructions.

Select two or more objects.

Choose Align from the interface pop-up menu (displayed by
pressing mouse button 3). Select one of the alignment icons from the
submenu.

The selected objects will be aligned according to the alignment
choice. Choices are described below. Vertical alignment icons are on
the left and are described first.

Left-edgealigns selected objects vertically
along their left edges.&newline;Vertical-centeraligns selected objects
vertically on their horizontal centers. &newline;Right-edgealigns selected objects vertically
along their right edges. &newline;Colonaligns selected objects vertically along
their colons or labels. &newline;Top-edgealigns selected objects horizontally
along their top edges. &newline;Horizontal-centeraligns selected
objects horizontally on their vertical centers. &newline;Bottom-edgealigns selected objects
horizontally along their bottom edges. &newline;Griddoes no alignment at this time.&newline;

Note that if you select objects that are arranged horizontally and
choose a vertical alignment (or vice versa), the objects will end up
on top of one another. You can unstack the objects by using the
Distribute function.
# See Also









# To Distribute Objects Evenly
distribute objects evenlyobjects, distribute evenlyalign objects, distribute evenly

While you are creating an interface, you may want to adjust the
spacing between objects. Use the Distribute function for this
"static" adjustment. Ultimately, you may want to group and attach the
objects for "dynamic" alignment. Seefor
instructions.

Select one or more objects.

Select one object to center it between the edges of its parent.

Choose Distribute from the interface pop-up menu (displayed by
pressing mouse button 3). Select one of the distribute icons from the
submenu.

The selected objects will be distributed or centered according to your
choice.

Objects will be spaced 10 pixels apart horizontally or vertically if
you choose one of the distribute choices. If you choose one of the
centering choices, the objects will be centered within the
boundaries of the parent control pane.

Horizontal-spacedistributes selected objects
horizontally 10 pixels apart. The left-most object is the anchored
object, which does not move. &newline;Vertical-spacedistributes selected objects
vertically 10 pixels apart. The top-most object is the anchored
object, which does not move. &newline;Horizontal-centercenters selected objects
horizontally between the left and right edges of the parent object,
maintaining the distance between selected objects. &newline;Vertical-centercenters selected objects
vertically between the top and bottom edges of the parent object,
maintaining the distance between selected objects. &newline;
# See Also







# Editing Properties of Interface Objects









# See Also



# To Open a Property Editor
open property editorproperty editor, open

Double-click on an object in the interface or in the
Module Browser to open the Revolving Property Editor with the
clicked-on object selected.

OrSelect an object in the interface or in the Module Browser and choose
Props (Revolving or Fixed) from the pop-up menu to open the property
editor with the object selected.

OrChoose Properties from the Editors menu in the App Builder primary
window.

The Revolving Property Editor will be displayed, with the
object most-recently selected in the interface or the Module Browser
selected in the Revolving Property Editor.

Seefor an illustration and description of the
Revolving Property Editor and its fields.
# See Also





# To Edit the Properties of an Object
edit properties of an objectproperties, editobject properties, edit

All objects dragged from the App Builder palettes havepropertiesthat can be edited. These properties include object name,
color, and a variety of other characteristics, depending on the object
type.

Choose the object type that you wish to edit from the Object
Type menu at the top of the Revolving Property Editor, if necessary.

If a tear-off (Fixed) editor is displayed, there is no Object Type menu.

If you double-clicked on an object to display the Revolving Property
Editor or if the object was selected when you chose Props from one of
the pop-up menus, the object type and the specific object will
already be selected.

Select the object that you wish to edit in the Objects
scrolling list, if necessary.

The object may already be selected.

Modify any of the properties, as appropriate.

Seefor descriptions of each of the elements of
the property editors.

Click the Apply button or press Return to apply the changes and
leave the property editor displayed; click the OK button to apply the
changes and close the property editor.

Click Reset to reset all fields to their values before the last Apply.

Click Cancel to reset all fields to their values before the last Apply
and close the property editor.
# See Also







# To Display a Fixed Property Editor
display fixed property editorfixed property editor, displaytear-off property editor, displaydisplay tear-off property editor

The Revolving Property Editor is a single dialog box that displays one
of 20 property editors, depending on the item you choose from the
Object Type option menu.
To display a separate, fixed property editor of a specific object
type:

Select the object you wish to edit in the interface or in the
Module Browser.

Choose Props from the pop-up menu in the interface or in the
Module Browser and select Fixed from the Props submenu.

Seefor a description of the pop-up menu.

Or

Choose the object type you wish to edit in the Revolving
Property Editor.

Click the Tear-off button at the top-right of the Revolving
Property Editor.
# To Select Colors from the Color Chooser
select colors from color choosercolor chooser, select colorsbackground colorforeground colorchoosing background colorchoosing foreground color

Most property editors have background and foreground color properties.
If you know the name of the color you wish to use, type it in the text
field next to Color:Background or Color:Foreground.
To select a color from the Color Chooser palette:

Click on the Background or Foreground menu button and choose
Color Chooser.

The color chooser, with an array of color choices available, will be
displayed.

Click on the desired color in the palette and click on the OK
button to select the color and dismiss the color chooser.

Repeat the process for Background or Foreground, if desired.

Click Apply in the property editor to apply the changes.
# Creating and Viewing Pane Objects













# To Create a Child Pane
createchild pane, create

Drop a text pane, draw area pane, or term pane on a control
pane in the interface.

A message dialog box will be displayed, asking if you want to create
the dropped pane as a child of the control pane or as a layered pane.

Click Child.

The pane will be instantiated at the drop location, just as if it were
a control object. The pane will be a sibling of the control objects on
the control pane. You will be able to select the pane and move it
around on the control pane just like any other control object.

Click Cancel if you do not want to create a child pane or a layered
pane.
# To Create a Layered Pane
createlayered pane, createunmake layered panelayered pane, unmake

Drop a pane on another pane in the interface.

A message dialog box will be displayed, giving you the option to
create a layered pane.

If you have dropped a text pane, draw area pane, or term pane on a
control pane, you will also have the option to create the object as a
child of the control pane.

Click Layer.

The pane will be instantiated as a layered pane on top of the pane it
was dropped on. Because it will be the same size as the original pane,
it will obscure the original pane completely. Seefor instructions for viewing the layers of a
layered pane.

Note that if you resize one of the layers of a layered pane, all panes
are resized.

Click Cancel if you do not want to create a child pane or a layered
pane.

Note: The only way to unmake a layered pane is to select one of the
layers and choosing Delete or Cut from the pop-up menu or from the
Edit menu.
# To View Layered Panes
view layered paneslayered panes, view

Select the visible pane of the layered panes in the interface
or in the Module Browser.

Choose NextLayer from the interface pop-up menu (displayed by
pressing mouse button 3).

The layer immediately beneath the current pane will be displayed.
# To Make a Paned Window
make paned windowpaned window, makecreate

A paned window is a combination of two or more panes (control, text,
draw area, or term panes, in any combination) into one virtual window
with multiple panes, one above the other, separated by a movable sash.
While the paned window maintains a constant height, the individual
panes become smaller or larger as you move the sash between them.

Drag a pane from the Panes palette and drop it on a
main window or a custom dialog.

If you want the paned window to span the top of the parent window,
drop the pane on the top-left corner of the parent. The pane will be
attached to the window at its left and top margins.

Resize the pane, if necessary.

If you want the paned window to span its parent window, drag the right
edge of the pane beyond the right edge of the window. The pane will be
attached to this edge, also.

&newline; &empty;

Drag one or more additional panes to the main window or dialog
and drop them on an unoccupied portion of the window.

Select all panes that you want to be part of the paned window.

Seefor instructions.

Choose Make Paned Window from the Layout menu or from the
interface or Module Browser pop-up menu (seeor).

The paned window will be created. If one of the panes is attached to
the right (East) edge of its parent, a message dialog box will be
displayed, explaining that the children of the paned window have
different East attachments. Click OK.

A paned window's size and position are determined by the position and
size of its panes: the left margin of the paned window is determined
by the left (West) edge of the pane that is furthest to the left.
The width of the paned window is determined by the width of its widest
pane.

In test mode or in the compiled application, you can resize the panes
by pressing mouse button 1 or mouse button 2 on the sash (the little
square between the panes near the right edge) and moving the mouse up
or down. You can set limits on the minimum and maximum heights of any
of the panes by setting Pane Height in the Paned Window property
editor.
# To Add a Pane to a Paned Window
add a pane to a paned windowpaned window, add a pane

Drop a pane on the paned window.

A message dialog box will be displayed, giving you the option to
include the new pane as a child of the control pane (if you drop a
text pane, draw area pane, or a term pane on a control pane), create
as a layered pane, or to add it to the paned window.

Click Pane to add the pane to the paned window.

The new pane will be added to the bottom of the paned window.
# See Also



# To Unmake a Paned Window
unmake a paned windowpaned window, unmake

Select the paned window.

You select a paned window by clicking right at its edge. Be sure you
select the paned window and not one of its panes. You will know you
have selected the paned window if a dark box is drawn around the paned
window and if grab handles appear at the four corners and four
mid-points of the paned window.

You could also open the Module Browser and select the paned window
there.

Choose Unmake Paned Window from the Layout menu or from the
pop-up menu in the interface or the Module Browser (seeor).

The panes that made up the pane will become separate panes again.
# Creating and Editing Menus


Menus are available within modules only. Be sure the menu created is
in the same module as the object you wish to attach the menu to. Menus
are created in the current module, which is determined by what is
selected in the interface. See the Editing Module field in the Object
Information Area of the App Builder primary window to see the current
module.












# To Create a Menu
createmenunew menu, createadd new menu

Display the Menu Property Editor by choosing Menus from the
Editors menu in the App Builder primary window.

Or you could display the Revolving Property Editor and choose Menu as
the Object Type.

If no menus exist in the current project, the Menu Objects list will
be blank and only the Add New Menu and Edit buttons will be active.

If any menus exist in the current project, they will be listed in the
Menu Objects list. One of the menus will be selected in the list and
the menu's properties will be displayed for editing.

Click Add New Menu.

A menu will be created with a default Object Name ("menu,"
"menu2," etc., depending on how many menus there are in the current
module), and with two items in the Items list ("Item1" and "Item2").

If you know you are going to need three menus, you could create them
all at one time by clicking Add New Menu three times. You can also
create submenus, which will be attached to menu items, at this time.

Edit the menu, as described in.

You can do this right after creating the menu or you can do this
later.

After you have created and edited a menu you will want to attach it to
an interface object and to make the menu functional by creating
connections between menu items and specific actions. You may also want
to attach a submenu to a menu item.
# See Also









# To Edit a Menu
edit menumenuenable tearoff menutearoff menu, enableadd menu itemsmenu items, additem mnemonicmnemonic, itemacceleratoritem acceleratorline style, itemitem line stylesubmenu, attachattach submenucreate submenusubmenu, createitem state, menumenu item state

Seefor descriptions of each of the menu
properties.

Change Object Name, if necessary.

The automatically-generated Object names, which are unique within
modules, do not usually need to be changed.

Click Enabled to enable the Tearoff function, if necessary.

This will make the menu "postable," meaning that it will be displayed
"permanently" if you click on the Tearoff indicator (a dotted line).

Add menu items to the Items list, if necessary.

Click Add Item to add an item after the selected item; choose from the
Edit menu button to perform other edit functions.

Change Item Label Type for menu items in the Items list, if
necessary.

Choices are String (text), Graphic, or Separator. Label becomes
Graphic Filename if Graphic is chosen; Line Style becomes active if
Separator is chosen.

Type a different Label or Graphic Filename for the selected
item, if necessary.

If Graphic Item Label Type was chosen, the Graphic Filename must be an
xpm or xbm graphic file.

Type an Item Mnemonic, if necessary.

Type one of the letters in the item label. That letter will be
underlined in the menu item label. If the menu is posted, pressing
that key will cause the action connected with the menu item to be
performed.

Note that case is significant for mnemonics and that the same mnemonic
cannot be used more than once in a menu.

Type an [Item] Accelerator, if necessary.

Type Ctrl<Key>x (or other letter -- upper or lower case) to create the
accelerator with the Control key as prefix. Substitute "Alt," "Meta,"
or "Shift" for "Ctrl" to specify one of those keys as the accelerator
prefix. Seefor more information.

Choose a Line Style, if Item Label Type is Separator.

Attach an Item Submenu, if appropriate.

Seefor instructions.

Change Item State, if necessary.

By default the item state is Active. If you want it to be inactive
when the application is started, click the Active check box to uncheck
it.

Select Background and Foreground Colors, if necessary.

Type in a color or choose Color Chooser from the menu and select a
color from the Color Chooser. Seefor details.

Click Connections to add programmatic connections to menu
items, as necessary.

Seefor instructions.

Click Apply or OK to apply the changes.

If you click Apply the property editor will remain displayed.
# See Also





# To Attach a Menu to an Object
attach menu to objectmenu to an object

The following instructions assume you have created one or more menus
as described inand that you are ready to attach a
menu to an object in the interface.

Display the Revolving Property Editor with the object to which
you wish to attach a menu selected in the editor.

Double-click on the object in the interface or the Module Browser or
choose the appropriate Object Type in the Revolving Property Editor
and select the desired object in the Objects list.

Choose Menus from the menu displayed when you press mouse
button 1 on the Popup Menu or Pulldown Menu menu button in the
Revolving Property Editor and select the appropriate menu from the
menus displayed in the pull-right submenu.

The name of the selected menu will be displayed in the text field of
the Popup Menu or Pulldown Menu.

Click OK or Apply.

The menu will be attached to the selected object.
# See Also









# To Attach a Submenu to a Menu Item
attach submenu to menu itemsubmenu, attach to a menu item

The following instructions assume you have created one or more menus
as described inand that you are ready to attach a
submenu to a menu item.

Display the Menu Property Editor by choosing Menus from the
Editors menu in the App Builder primary window.

Choose Menus from the menu displayed when you press mouse
button 1 on the Item SubMenu menu button and select the appropriate
menu from the menus displayed in the pull-right submenu.

The name of the selected menu will be displayed in the text field of
the Item SubMenu.

Click OK or Apply.

The submenu will be attached to the selected menu item.
# See Also







# To Create and Attach a Menu
menucreate

One method of creating and attaching a menu to an object is described
inand. With the method
described here you create and attach the menu at one time. Use
whichever method is most convenient.

Display the Revolving Property Editor with the object to which
you wish to attach a menu selected in the editor.

Double-click on the object in the interface or the Module Browser or
choose the appropriate Object Type in the Revolving Property Editor
and select the desired object in the Objects list.

Choose Create New Menu from the Pulldown or Popup Menu button,
available for some objects.

Pulldown menus are available for menu buttons and menu bars. Popup
menus are available for all pane objects and for lists. An Item
Submenu is available for menus themselves.

The Menu Property Editor will be displayed, with a newly-created menu
selected in the Menu Objects list. The menu will have default values
for Object Name, Tearoff, Items, Item Label Type, Label, and Item
State. The menu will be attached to the object selected in the
property editor when Create New Menu was chosen.

The Object Name will be of the form "object_type_menu" or
"object_type_menu2," etc., depending on what type of object was
selected in the property editor when Create new Menu was chosen and
how many menus have been created for the current module. The menu will
be added to the end of the Menu Objects list, with the name of the
current module preceding the menu name.

Edit the menu and click OK to apply the changes and dismiss the
Menu Property Editor.

Seefor instructions.

Click Apply or OK in the Revolving Property Editor to attach
the menu to the selected object.
# See Also







# To Create and Attach a Submenu
createsubmenu, create and attach

The following instructions assume you have created one or more menus
and that you want to attach a submenu to one of the items in one of
the menus. With this method you create the submenu and attach it as
part of a single procedure. Another method for accomplishing this task
is to create the menu as described inand to
attach it to a menu item as described in. Use
whichever method is most convenient.

Display the Menu Property Editor.

Select a menu item in the Items list.

This is the item to which you will append the submenu.

Choose Create New Menu from the Item SubMenu menu.

The Revolving Property Editor will be displayed, with the new menu
selected in the Objects list.

Edit the menu.

Seefor instructions. You can do this later if you
like.

Click OK to apply the editing changes you made and to dismiss
the Revolving Property Editor.

Click Apply in the Menu Property Editor to attach the submenu
to the menu item selected in Step 2.

Note: When you are creating a submenu you switch back and forth
between the Menu Property Editor and the Revolving Property Editor,
creating the submenu in one editor and attaching the submenu to the
menu in the other. In the description above, you started by selecting
a menu in the Menu Property Editor and you created the submenu in the
Revolving Property Editor. If you started in the Revolving Property
Editor, you would create the submenu in the Menu Property Editor and
attach the submenu to the menu in the Revolving Property Editor.
# See Also







# Creating and Editing Messages







# See Also



# To Create a Message Dialog Box
createmessage dialog box, createdialog, message: to create

Seefor descriptions of each of the fields in
the editor. Seefor a discussion of
how to connect messages to the functions that cause them to be
displayed, with examples.

To create error, information, working, question, or
warning messages to be displayed at appropriate times in the compiled
application:

Choose Messages from the Editors menu of the App Builder
primary window to display the Message Editor.

Choose the module to which you want to add a message in the
option menu below the Messages list.

Click Add Message.

A unique name ("message," "message2," etc., depending on how many
messages their are in the current module) will be displayed in the
Name field. The module name and the message name will be added to the
Messages list.

Modify the Name if you wish.

This is the name used to identify the message internally -- in the
Connections Editor, for instance. This name is not displayed in the
compiled message dialog box.

Type a Dialog Title.

This will appear in the title bar of the compiled message dialog box.

Choose a Message Type (Error, Information, Working, Question, or
Warning).

The icon for the message type will be displayed in the Type option
menu and the message type will be displayed above the text pane where
the message text will be typed.

&newline; &empty;

Type the message in the text pane, pressing Return when you
want a new line to start in the compiled message.

Specify which buttons will appear in the message dialog box by
clicking the check boxes below the message text pane and typing the
labels you want on the Action1, Action2, and Action3 buttons.

Each of the message types includes a default set of buttons which you
can modify:

* **Error** 

Action2 (Retry label), Cancel, Help; Default Button = Action2.
* **Information** 

Action1 (OK), Help; Default Button = Action1.
* **Working** 

Action1 (Close), Action2 (Stop), Help; Default Button =
Action1.
* **Question** 

Action1 (Yes), Action2 (No), Help; Default Button =
Action1.
* **Warning** 

Action2 (Continue), Cancel, Help; Default Button = Action2.


Choose a Default Button from the option menu.

This is the button that will have an extra "moat" border when the
message dialog box is displayed. This button will be activated if
Return is pressed.

Click the Help Text button and create help text, as
appropriate. Seefor instructions.
# See Also







# To Edit a Message Dialog Box
edit message dialog boxmessage dialog box, edit

Choose Messages from the Editors menu of the App Builder
primary window to display the Message Editor.

Select the message you want to edit in the Messages list.

To delete a message, click Delete Message.

To modify the dialog box title, click in the Dialog Title text
field and type the new label.

To change the message type, choose a different Type icon.

To modify the message, click in the message text pane and type
the appropriate changes.

To change the available buttons, click on the check boxes and
type new button labels, if appropriate.

To change the Default Button, choose another from the option
menu.

To modify help text, click on Help Text, make the changes in
the Help Editor, and click OK in the Help Editor.

Click Apply or OK to apply the changes.

The Message Editor will be dismissed if you click OK.
# See Also





# To Connect a Message Dialog Box to a Function
connect message dialog box to a functionmessage dialog box, connect to function

Seefor a discussion of how to connect a
message to the function that causes it to be displayed, with
examples. In particular read that section to see how to attach a modal
(blocking) message to a function.

To connect a non-modal message to a function:

Display the Connections Editor by clicking Connections in the
Message Editor or by choosing Connections in the Editors menu on the
App Builder primary window.

If you select a message in the Message Editor and click Connections,
the selected message will be selected in the Source list of the
Connections Editor. You can skip the next two steps.

Display messages in the Source list by choosing Messages in the
Source menu.

Select a message in the Source list.

Choose Call Function as the Action Type.

This activates the When menu on the Source side of the Connections
Editor.

Choose a When item (Action1, Action2, Action3, or Cancel
Activated, depending on which buttons were checked in the Message
Editor.

Type the name of the Function to be called when the selected
button is selected.

When code is generated, this call function will create a stub routine
instubs.c. You will have to substitute appropriate code
before runningmake.

Click Connect to create the connection.

The connection will be displayed in the View list at the bottom of the
Connections Editor.

Repeat the previous three steps for each button except Help.

Click Cancel to dismiss the Connections Editor.
# See Also







# Creating Help









# To Create a Help Volume
createhelp volume

A help volume, such as the one one you are viewing now, is created
separately from App Builder, and is accessed from your compiled
application through the Help menu or by clicking More in an on item
help dialog box.

See theHelp System Author's and Programmer's Guide,which is included in the desktop Help Developer's Kit, for
instructions for creating a help volume.
# See Also



# To Create On Item Help
createon item help, createhelp

Display the Revolving Property Editor.

Choose the Object Type for which you want to write help.

Select the object for which you want to write help.

Click Help Text to display the Help Editor with the appropriate
object selected.

Type on item help text in the Help Text pane.

Press Return when you want a new line to start in the compiled help
dialog box.

Click Apply to apply the changes.

If you want to add on item help to other objects, choose the
appropriate Object Type in the menu, select the appropriate object,
and repeat the previous two steps.

Click OK to apply the changes and dismiss the Help Editor.
# To Connect a Help Menu to a Help Volume
connect help menu to help volumehelp menuhelp volumeconnection

After creating a help menu and attaching it to the Help item in a menu
bar, do the following to connect menu items to specific locations in a
help volume. Seefor instructions
for connecting the on item help item in the Help menu to the on item
help function.

Display the Connections Editor.

Click Connections in the Revolving Property Editor or in the Menu
Property Editor or choose Connections from the Editors menu.

Choose Menu Item from the Source option menu.

Select one of the Help menu items from the Source scrolling
list.

Choose Access Help Volume from the Action Type option menu.

Type the name of the help volume in the Volume text field.

Type the appropriate location ID in the Location text field.

Click Connect to make the connection.
# See Also



# To Connect a Help Menu to On Item Help
on item help, create menu itemhelp menucreate on item help menu itemconnection, on item help menu itemconnect help menu to on item help

One of the standard items in a Help menu is On Item Help, which is
used to display help for a specific object in an interface. The
instructions below assume you have included a menu bar in a main
window and that you have attached a Help menu to the Help item in the
menu bar.

Choose Menus from the Editors menu in the App Builder primary
window to display the Menu Property Editor.

Select the Help menu in the Objects list.

Select one of the items in the Items list as the On Item help
item.

Type "On Item" or other appropriate text in the Label text
field.

Include an Item Mnemonic, if appropriate.

An item mnemonic specifies one of the letters in the selected item as
a keyboard shortcut for activating the menu item when the menu is
posted. The letter specified will be underlined in the menu item.
Note that case is significant for mnemonics.

Include an Item Accelerator, if appropriate.

An item accelerator specifies a keyboard shortcut for choosing the
selected item. An accelerator is comprised of a prefix (Ctrl, Alt,
Meta, or Shift) plus <Key> plus a letter (upper or lower case). To
make Control-X an accelerator, for instance, type the following:

Ctrl<Key>x

Click OK or Apply.

Click Connections to display the Connections Editor.

Choose Menu Item in the Source menu.

Select the On Item help item in the Source list.

Choose Activate On Item Help from the Action Type menu.

Click Connect.

When you choose the On Item help item in the Help menu in test mode or
in the compiled application, the cursor will become an arrow with a
question mark. Move the cursor over an object and click mouse button 1
to display On Item help for the selected object (or for one of its
parent objects if no help is available for the object itself).
# See Also







# Making Connections Between Interface Objects





# To Make a Connection Between Two Objects
make connection between objectsconnection, makedrag-link connectionconnection, drag-link

In its simplest form a connection is a programmatic relationship
between a source object and a target object. The first step in making
such a connection is to select the source and target objects.

The quickest way to select the two objects is to place the cursor over
one of the objects and to"drag-link" a connection to the other. This
can be done in the interface or in the Module Browser (or between the
interface and the Module Browser).

While holding down the Control key, position the mouse cursor
over the intended source object, press mouse button 1, drag the
cursor to the intended target object, and release the mouse button.

A line with a "plug" at its end will extend from the source as you
move the mouse. The target object will be highlighted with a dark box;
release the mouse button when the proper target is highlighted.

The Connections Editor will be displayed, with the source and target
objects selected.

Alternatively, you can display the Connections Editor by choosing
Connections in the Editors menu, choose the object type you want as
the source object in the Source menu, select the object you want as
the source in the Source list, choose the object type you want as the
target object in the Target menu, and select the object you want as
the target in the Target list.

Choose an action in the When menu.

This is the action on the source object that will cause an action to
be performed on the target object. Choices vary, depending on the
source object type.

Choose an action to be performed on the target in the Action
Type menu.

Seefor descriptions of the options. If you
choose Predefined, you will also have to choose an action from a
second option menu.

If you choose Call Function you will have to type the name of a
function in the Function text field.

If you choose Execute Code you will have to type the code to be
performed in the Execute Code Editor and to click OK in the editor.

Click Connect or press Return to make the connection.

The connection will be displayed in the View list at the bottom of the
Connections Editor.
# See Also









# To Edit Connections
edit connectionsconnections, edit

To edit existing connections:

Choose Connections from the Editors menu in the App Builder
primary window to display the Connections Editor.

Choose the source object type whose connection you wish to view
in the View menu at the bottom of the Connections Editor.

If you want to edit a connection with a button as a source object, for
instance, choose Button from the View menu. All connections in the
current project with button as source object will be displayed in the
View list.

If you want to view all connections for a particular source object,
choose Source Object in the View menu and select the object in the
Source menu. All connections for the selected object will be
displayed.

Click on the connection you wish to edit to select it in the
View list.

The source and target objects will be selected in the Source and
Target lists at the top of the editor. Their When and Action Type
choices will be displayed.

To delete the selected connection, click Delete.

To modify the selected connection, make changes to any of the choices
(source object, When action, target object, Action Type) and click
Change.

To add a connection similar to the selected connection, modify any of
the choices and click Connect.
# See Also





# Testing Menus, Help, and Connections









# To Test a Project
test a projectproject

Click Test Project.

All modules in the current project will be displayed. Windows which
have an initial state of not Visible, as do custom dialogs by default,
will not be shown.

Test help, if appropriate.

Seefor detailed instructions.

Test menu displays, if appropriate.

Seefor detailed instructions.

Test connections, if appropriate.

Since windows with an initial state of not Visible are hidden in Test
Project mode, you can test connections that show hidden windows.

Click Build to return to build mode.
# See Also







# To Test Shown Modules
test shown modules

Click Test Shown Modules in the App Builder primary window.

All build windows except the App Builder primary window will be
closed, and the primary window will be inactive except for the Build
button and the Help menu. All windows in the currently-shown modules
will be displayed, including those that have an initial state set to
not Visible. Seeif you want to test the entire
project, with not-Visible windows hidden.

Display the module to be tested, if necessary.

Seefor instructions.

Test help, if appropriate.

Seefor detailed instructions.

Test menu displays, if appropriate.

Seefor detailed instructions.

Test connections, if appropriate.

Note thatwindows with an initial state of not Visible will not be
hidden in Test Shown Modules mode; seefor
instructions.

Click Build to return to build mode.
# See Also







# To Test Help in a Module
test helphelp

Display the module to be tested, if necessary.

Seefor instructions.

Click Test Shown Modules.

All build windows except the App Builder primary window will be
closed, and the primary window will be inactive except for the Build
button and the Help menu. All windows in the currently-shown modules
will be displayed, including those that have an initial state set to
not Visible. Seeif you want to test the entire
project, with not-Visible windows hidden.

Test help volume access by choosing Overview, Tasks, or
Reference from the Help menu.

A help volume window will be displayed. Seefor
instructions for creating help and making connections to it. Dismiss
the help window when you are finished with it.

Test On Item help by choosing On Item from the Help menu.

The cursor will turn into an arrow and a question mark.

Move the cursor over an App Builder object and click.

If the clicked-on object (or one of its parent objects) has help text,
it will be displayed in a quick-help window.

Click on the More button in the quick-help window, if it is
active.

The help volume will be displayed, at the location specified in the
Location Id for the selected object in the Help Editor. Dismiss the
help window when you are finished with it.

Click on the Close button in the quick-help window to dismiss
it.

Click on Using Help in the Help menu to learn about help.

Click Build to return to build mode.
# See Also







# To Test Menus in a Module
test menusmenus, test

Display the module to be tested, if necessary.

Seefor instructions.

Click Test Shown Modules.

All build windows except the App Builder primary window will be
closed, and the primary window will be inactive except for the Build
button and the Help menu. All windows in the currently-shown modules
will be displayed, including those that have an initial state set to
not Visible. Seeif you want to test the entire
project, with not-Visible windows hidden.

Click or press on the items in a menu bar, if appropriate.

The menus will be displayed.

Press on each button menu, as appropriate.

The menus will be displayed.

Press mouse button 3 on a pane or list to display a pop-up
menu, if appropriate.

Choose items in displayed menus.

If menu items have been connected to functions (Show or Hide a dialog,
Access Help Volume, Activate On Item Help, for example), the function
should be performed.

Click on push buttons connected to functions, if appropriate.

If you connect a button to a custom dialog, for instance specifying
that the custom dialog should be hidden when the button is pushed,
this function should work.

Click Build to return to build mode.
# See Also







# Grouping and Attaching Objects in an Interface
grouping and attaching objectsattaching objects

In order to ensure that interface objects maintain consistent spacing
and size relationships, regardless of text changes (including
internationalization changes) and resizing of windows, you may need to
group control objects and to attach objects to each other.

A group is a collection of objects that can be treated as a unit. Once
the objects in a group are positioned as desired, the group can be
moved, maintaining the relative positioning of the individual objects.
Because groups use dynamic layout for positioning objects, spacing and
alignment in the group are maintained if any of the objects in the
group change size.










# See Also



# To Create a Group
creategroup, create

Select the control objects you wish to be part of the group.

You can select the objects either in the interface or the Module
Browser, and you can select the objects in whatever manner is most
convenient. Seefor instructions.

Choose Group from the Layout menu or the pop-up menu (displayed
by pressing mouse button 3 with the cursor in the window interface or
in the Module Browser).

A rectangular box will be drawn around the group in the interface,
indicating that the group is selected. Note that Ungroup is active in
the Layout and pop-up menus; this is only true when a group is selected.

A new object will be displayed and selected in the Module Browser --
an object called "group" (or "group2," etc., if other groups exist in
the module). The group object is the parent of the objects that
comprise the group. Note that group members cannot be moved
independently; any attempt to move an object in a group will cause the
entire group to move.
# See Also













# To Edit Group Properties
edit group propertiesgroup properties, edit

Group properties, including horizontal or vertical alignment and spacing
between objects, are set in the Group Property Editor.

Double-click on the group in the interface or in the Module
Browser to display the Group Property Editor.

In the interface you will have to click in the space between group
members to select the group. The group will be selected in the Group
Property Editor.

You can also display the Group Property Editor by choosing Groups from
the Editors menu.

Select the group to be edited from the Group Objects list, if
necessary.

Type a new name for the group, if necessary.

Choose a border frame style if you wish the group to have a
border in the completed interface (no border is the default).

Border frame style choices are shadow out, shadow in, etched out,
etched in, and none.

Select a Layout Type.

Choices are as-is, vertical, horizontal, and row-column.

Depending on what you select, either the Vert Alignment or Horiz
Alignment option menu, or both, will be active. If you select
rows-columns, the Rows and Columns radio buttons will be active, also.

Designate the number of Rows or Columns (if row-column layout
was selected).

The number of columns will be determined automatically if you
designate the number of rows, and the number of rows will be
determined automatically if you designate the number of columns.

Choose a vertical alignment (if either vertical alignment or
row-column layout type was chosen).

The choices are align on left edge of objects (the default), align on
colons/labels, align on middle of objects, or align on right edge of
objects.

Designate vertical spacing (if either vertical alignment or
row-column layout type was chosen).

The absolute values are in pixels; 10 is the default.

Choose a horizontal alignment (if either horizontal alignment or
row-column layout type was chosen).

The choices are align on top edge of objects (the default), align on
middle of objects, or align on bottom edge of objects.

Designate horizontal spacing (if either horizontal alignment or
row-column layout type was chosen).

The absolute values are in pixels; 10 is the default.

Deselect Visible if you do not want the objects in
the group to be visible when the application is opened.

Deselect Active if you do not want the objects in
the group to be active when the application is opened.

Click OK or Apply to apply the changes.

The Group Property Editor will be dismissed if you click OK.
# See Also



# To Ungroup Objects in an Interface
ungroup objects

Select the group in the in the Module Browser or in the
interface.

In the interface, click between objects in a group to select the
group. You will know the group is selected if a box appears around two
or more objects, as shown below.

&newline; &empty;

If you can't select a group in the interface or if you want to be sure
to select the right group in an interface with many groups, open the
Module Browser. Groups are shown in the Module Browser by name of
group; if you select the group in the Module Browser, it is also
selected in the interface.

Choose Ungroup from the interface pop-up menu (displayed by
pressing mouse button 3 in an interface window).

The objects are no longer part of the group; you can now select any of
the objects and move it independently of the other objects.
# See Also







# To Create a Border Around an Object
createborder, create

The group function can be used to create a border around an
individual object, such as a label.

Select the object in the interface.

Choose Group from the Layout menu or the interface pop-up menu.

The object will be part of a group.

Display the Group Property Editor.

Select the group you wish to put a border around.

If you double-click on the group in the Module Browser, the Group
Property Editor will be displayed, with the group selected.

Choose the Border Frame style you wish to add to the object.

Click OK to apply the change and dismiss the Group Property
Editor.
# See Also



# To Attach Objects in an Interface
attach objects

Choose Attachments from the Editors menu in the App Builder
primary window to display the Attachments Editor.

The Attachments Editor can also be displayed by clicking the
Attachments button in a property editor or by choosing Attachments
from the interface or Module Browser pop-up menu.

Choose the object type you wish to attach to its parent or
siblings.

Select the object that you wish to attach.

Select an attachment type.

If you choose an icon with one small square you are making an
attachment from a child object to its parent. If you choose an icon
with two small squares you are making a sibling attachment. Seefor descriptions of the types of attachments.

When you make an attachment, the selected object -- the
object in the center of the four Attach To boxes -- is the controlling
object. That is, this object can be moved without affecting its parent
or sibling. The offset value or percentage value will change to
reflect the changed relationship between the two objects.

On the other hand, if you move the other object -- the object to
which the selected object is attached -- the selected object will move
so as to maintain its relationship with the other object.

Note that you may have to click Reset after moving an object in the
interface before the change is noted in the Attachments Editor.

Click OK or Apply to apply the changes.

If you click OK the Attachments Editor will be dismissed.
# See Also











# To Establish Drag and Drop Behavior
drag and drop behavior, establishdrag operationsdrop operationsdrag connectiondrop connection

Choose Drag and Drop from the Editors menu of the App Builder
primary window to display the Drag and Drop Editor.

Choose an Object Type.

Select an object in the Objects list.

Select the Drag Operations you want to be legal for the
selected object.

To display a special cursor when a drag operation is being
performed from the selected object, type the names of graphics files
in the Cursor Filename and Cursor Mask Filename fields.

Select the Data Types that will be legal for drag operations.

Click Drag Connection to display the Connections Editor.

Choose Dragged From as the When action in the Connections
Editor.

Choose Call Function as the Action Type in the Connections
Editor.

Type a name for the called function in the Function text field
in the Connections Editor.

You will have to edit thestubs.cfile to make
the called function do something useful.

Click Connect in the Connections Editor.

Select which Drop Operations will be legal.

Select the Data Types that will be legal for drop operations.

Check Drop on Children if you want a drop operation on a child
of the selected object to be legal.

This is relevant only if the selected object has a child which is
designated as a legal drop site.

Click Drop Connection to display the Connections Editor.

Choose Dropped On as the When action in the Connections
Editor.

Choose Call Function as the Action Type in the Connections
Editor.

Type a name for the called function in the Function text field
in the Connections Editor.

You will have to edit the stubs.c file to make the called function do
something useful.

Click Connect in the Connections Editor.

Click OK or Apply in the Drag and Drop Editor to apply the
changes.
# See Also





# To Establish Application Framework Behavior
establish application framework behaviorapplication framework behaviorvendor name, application frameworkversion, application frameworkprimary main window, application frameworkinternationalization, application frameworkresource file, application frameworksession management, application frameworktooltalk message handling, application frameworkapplication framework editor

The Application Framework Editor establishes basic functionality for
the application as a whole. To establish application behavior for
internationalization, resource file attributes, session management,
and ToolTalk message handling:

Choose Application Framework Editor from the Editors menu in
the App Builder primary window to display the editor.

Type a Vendor Name and Version number in the text fields in the
Application section, if appropriate.

These are used in the call to initialize ToolTalk, if ToolTalk is
enabled.

Choose a different Primary Main Window, if appropriate.

Set Internationalization to Enabled, if appropriate.

Internationalization generates labels and strings for objects with a
call that fetches the appropriate localized version of the string at
run time. It also generates and maintains a similar message catalog.

Select the attributes you want to be written to the Resource
file in the Generated Code section.

The categories you select are written to a resource file instead of
directly to the module file; these attributes, therefore, can be
modified without recompiling the application.

Choose a Method (None, Command Line, Session File, or Both) in
the Session Management section, as appropriate.

Select Session Save Connection and/or Session Restore
Connection, as appropriate, to make connections in the Connections
Editor.

Choose a Desktop Message Handling level (None, Basic, or
Advanced) in the ToolTalk section, as appropriate.

Seefor details about ToolTalk message
handling.

If you did not choose Advanced in the previous step, click OK
to apply the changes made and dismiss the Application Framework
Editor.

Click Advanced ToolTalk Connections if you chose Advanced
in the previous step.

Choose the appropriate ToolTalk function from the When menu in
the Connections Editor.

Choose Call Function as the Action Type.

Type in the name of the appropriate call function.

Click Connect to make the connection.

Click OK in the Application Framework Editor to apply
the changes and dismiss the editor.
# See Also





# Generating Code and Running an Application









# To Make and Run the Application
generate codemake and run applicationmake command, code generatorcode generator, generate coderun application, code generatorgenerate codemake the application

Two scenarios are described below: in the first, you build and run the
application in one step. In the second case, you generate code,
compile the code, and run the application in separate steps.
# One-Step Make and Run


Choose Code Generator from the File menu of the App Builder
primary window to display the Code Generator.

Click Make & Run to generate code, build the application, and
run it.

If you have saved the project and all goes well, a number of messages
will be displayed in the Output Pane at the top of the Code Generator.
The final message will be "Running: ./[projectname]" and the
application will run.

At the least, the application primary window will be displayed. Any
windows whose visibility is not set to yes at application startup will
be hidden. Depending on what functionality you included that does not
require user code, the application might do a variety of things. Menus
can be displayed, some connections can be tested, on item help can be
displayed, and so on.

Ultimately, someone is going to have to write some code to complete
the application. Any Call Function callbacks specified in the
Connections Editor will have to be substituted for, for one thing.
# Generate Code, Make, and Run Separately


Click Generate Code to generate code for the current project.

If you have made changes to the project that have not been saved, a
message dialog box will be displayed, telling you that you have
unsaved edits and giving you the choice of cancelling the generate
code process or saving the project. If you choose to save the project,
you will have to specify where to save the project if it has not been
saved before.

Asdtcodegenruns, messages are displayed in the
output pane at the top of the Code Generator window. The final message
should be "Completed successfully." A number of files will be created,
including Makefiles, project files, module files, and twodtb.utilsfiles. You can look at the files in the
Term Pane at the bottom of the Code Generator window.

Click Make to build the application.

More messages will be displayed in the Output Pane as the application
is compiled. The final message again should be "Completed
successfully." A few more files will be created, including object
files and the executable application file, which has the name you gave
the project.

Click Run to run the application.

Click Abort to quit the application.
# To Set Code Generator Options
code generator options, setset code generator optionsoptions, code generator

To change the options that determine what code is generated and other
Code Generator functions:

Choose Code Generator from the File menu of the App Builder
primary window to display the Code Generator window.

Choose Generator from the Options menu to display the Code
Generator Options dialog box.

Select one of the Generate Code For options (Entire Project,
Main Only, Specific Modules Only, Specific Modules and Main).

If you select Specific Modules or Specific Modules and Main, type the
names of the modules you want to generate code for in the text pane,
pressing Return after each module name.

Click Don't Merge if you wish to change that setting.

Choose a different message reporting option if you wish.

Type Make Arguments, if appropriate.

Type Run Time Arguments, if appropriate.

Click Reset to Defaults to set all fields to their default
values.

Click OK or Apply to make the changes.

The Options dialog box will be dismissed if you click OK.
# See Also



# To Set Environment Options
set environment optionsenvironment options, set

Choose Code Generator from the File menu of the App Builder
primary window to display the Code Generator window.

Choose Environment from the Options menu to display the
Environment Options dialog box.

Type a variable in the Variable Name text field.

Click Get to display the current value for the variable in
Variable Name.

The value of the variable will be displayed in the Value pane.

Make a change to Value and click Set to change the value of the
variable.

This change is made for this App Builder session only.

Click Reset to reset Value to its value outside of this session
of App Builder.

Click Cancel to dismiss the dialog box.
# See Also



# To Generate Code from the Command Line


To generate App Builder code from the command line, rundtcodegen. Usage is described below.

Usage: dtcodegen [options] [project-file] [module-file [module-file] ...]

Code is generated for each module specified on the command line, or for
all modules in the project, if no modules are specified. If no project
file is specified, a project file containing the specified module(s) is
searched for in the current directory.

Files with extension .bip are assumend to be BIL project files, files
with.bixextension are assumed to be
encapsulated BIL files, and files With a.bilextension are assumed to be BIL module files.Options (* = default, + = default with no project file):
  -help (-h)        Print out this help message
  -main             Write file containing main()
  -changed          Only generate files that have changed
* -merge	    Merge generated _stubs.c files with previous version
  -nomerge          Don't merge existing and new stubs file
* -project (-p)     Specify a project to generate code for
  -noproject (-np)  Use default project settings, ignore project file
+ -showall          Application shows (maps) all windows at startup
* -noshowall        Application shows (maps) only initially-visible windows
  -silent (-s)      Silent mode, no messages written
  -verbose (-v)     Verbose mode, detailed progress messages