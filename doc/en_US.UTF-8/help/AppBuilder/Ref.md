
# App Builder Reference


&newline;&empty;






# App Builder Windows and Dialog Boxes


&newline; &empty;






















# See Also



# App Builder Primary Window
window, mainmain window

&newline;&empty;

The App Builder primary window is the starting point for building a
graphical user interface (called an interface throughout this volume).
The interface is created by dragging objects from the App Builder
object palettes (Windows, Panes, and Controls) to the desktop, editing
the properties of the resultant interface objects, and adjusting the
layout of the interface. Seefor a summary
of the steps involved in creating an interface.

&newline;&empty;title barprimary window title barmenu barprimary window menu barmode barprimary window mode barobject palettesprimary window object palettes

&newline; &empty;

* **Title bar** 

Includes the name of the application, "Application
Builder," the window manager menu, a minimize button, a maximize
button, the name of the current project (if one is open), and a "(Save
Needed)" indication if the current project has changed since being
saved.
* **Menu bar** 

Includes File, Edit, View, Layout, Editors, and Help menus;
seefor descriptions of each.
* **Mode bar** 

Includes radio buttons for specifying build and test modes:

Build - For designing and building an interface.

Test Shown Modules - For testing help, menus, and connections
in current, shown modules. All objects will be shown, including those
for which the initial state is not set to Visible.

Test Project - For testing help, menus, and connections in the
current project. Objects for which the initial state is not set to
Visible will not be shown.
* **Windows Palette** 

Includes the three App Builder window objects: main
window, custom dialog, and file selection dialog. Window objects are
dropped on the desktop. Seefor details.
* **Panes Palette** 

Includes the four App Builder pane objects: control
pane, text pane, draw area pane, and term pane. Pane objects are
dropped on main windows, custom dialogs, or other panes. Seefor details.
* **Controls Palette** 

Includes 14 App Builder control objects: button,
menu button, combo box, option menu, menu bar, radio box, check box,
gauge, scale, separator, text field, label, list, and spin box.
Control objects are dropped on control panes. Seefor details.
* **Object Information Area** 

Provides information about the object
beneath the cursor. Seefor details.

# Windows Palette
palette, windowswindows paletteprimary window windows palette

The Windows palette contains three objects: main window, custom
dialog, and file selection dialog. Click on an underlined object
type below for descriptions of each of the window or dialog objects.

&newline;&empty;&newline; &newline;&empty;&newline;Main Window&newline;&empty;&newline;&newline;&empty;&newline;&newline;Custom Dialog&newline;&empty;&newline;&newline;&empty;&newline;&newline;File Selection Dialog
# Main Window
main window objectobject, main window

&empty;

A main window is the basic App Builder object. It is created by
dropping a main window icon on the desktop. The starting point for a
user interface is built in a main window. A main window has a
minimize button and therefore can be closed to an icon.

The status region includes the name of the module the window is part
of and indicates when the window object is selected. It does not
appear in the compiled application.

Examples of main windows used in building App Builder itself are the
App Builder primary window, the Project Organizer, the Module Browser,
and the Code Generator.
# See Also



# Custom Dialog
custom dialogobject, custom dialog

&empty;

A custom dialog is a window for displaying information or providing a
pop-up for a specific task within an interface. It is created by
dropping a custom dialog icon on the desktop. A custom dialog might be
"connected" to a button or a menu in a main window, causing the pop-up
dialog to be displayed when the button is clicked or a menu item is
chosen. A custom dialog cannot be closed to an icon.

The status region includes the name of the module the dialog is part
of and indicates when the dialog object is selected. It does not
appear in the compiled application.

Examples of custom dialogs used in building App Builder include the
File Selection Dialog, the Project Name and Module Name dialog boxes,
all of the editors, and the message dialog boxes.
# File Selection Dialog
file selection dialogobject, file selection dialogobject:file selection dialog

&empty;

A file selection dialog is a specialized pop-up dialog for specifying
a file in an Open or Save operation. It is created by dropping a file
selection dialog icon on the desktop.

The status region includes the name of the module the dialog is part
of and indicates when the dialog object is selected. It does not
appear in the compiled application.
# See Also



# Panes Palette
palette, panespanes palette

The Panes palette contains four objects: control pane, text pane, draw
area pane, and term pane. All panes can be dropped on a main window, a
custom dialog, or another pane. If a pane is dropped on a
pane, the dropped pane will become a child of the first pane or a
layered pane will be created. Seefor more
information.

&newline;&empty;&newline; &newline;&empty;&newline;Control Pane&newline;&empty;&newline;&newline;&empty;&newline;&newline;Text Pane&newline;&empty;&newline;&newline;&empty;&newline;&newline;Draw Area Pane&newline;&empty;&newline;&newline;&empty;&newline;&newline;Term Pane
# See Also



# Control Pane
object, control panecontrol pane object

&empty;

A control pane is the drop site for App Builder controls. It is
created by dropping a control pane icon on a main window, a custom
dialog, or another pane. In the figure above, a control pane has been
dropped on the top-left corner of a main window, in anticipation of
resizing it to fill the entire canvas.

Examples of control panes used in building App Builder include the
pane on which the three panes palettes reside on the App Builder
primary window and the pane beneath the controls on each of the
property editors.
# See Also



# Text Pane
object, text panetext pane object

&empty;

A text pane is a multi-line text-entry area in the completed
application. It is created by dropping a text pane icon on a main
window, custom dialog, or another pane.

Examples of the use of text panes in building App Builder include the
Initial Value field in the Text Pane property editor and the Help Text
field in the Help Editor.
# Draw Area Pane
object:draw area panedraw area pane object

&empty;

A draw area pane is used as a drawing or display area in the completed
application. It is created by dropping a draw area pane icon on a main
window, custom dialog, or another pane.

Note the horizontal and vertical scroll bars, which enable you to view
objects outside the current view area.

Examples of the use of draw area panes in building App Builder include
the panes displaying modules and module objects in the Module Browser,
and the pane displaying modules in the Project Organizer.
# Term Pane
object:term paneterm pane object

&empty;

A term pane is a terminal emulation object which accepts user input
and echoes standard output. It is created by dropping a term pane
icon on a main window, custom dialog, or another pane.
# Controls Palette
controls palettepalette, controlsbuttonmenu buttoncombo boxoption menuradio boxcheck boxgaugescaleseparatormenu bartext fieldlabelscrolling listspin box

The Controls palette contains 14 objects, including buttons, lists,
text fields, and a menu bar.

Button: A control which, when
clicked, performs a specified action. A button can be a push button,
a drawn button, or a menu button, settable in the Button property
editor.
&newline; &empty;
A drawn button, like a push button, performs a specific function when
clicked; the label on a drawn button, however, can change dynamically,
depending on the status of the application.

Menu Button: A
specialized button, ready for attachment of a menu. Seeandfor instructions for
creating menus and attaching them to menu buttons. Note that there is
no menu button property editor; edit the properties of a menu button
in the Button Property Editor.

Combo Box: A combination
text field and option menu object. As with an option menu, you can
select an item from a pop-down menu, but you can also edit any of the
items in the list -- if you have checked "Editable" in the property
editor, and if you write code to make it work. Seefor instructions.

Option Menu: One of the
three "choice" objects (option menu, radio box, check box). When you
click on an option menu, a menu is displayed, providing a choice of
items to choose from. The chosen item remains in the option menu box
and becomes the active choice. Examples of option menus in App
Builder are Object Type in the property editors and Source and Target
in the Connections Editor.

Radio Box: One of the three
"choice" objects (option menu, radio box, check box). A radio box is
comprised of a label and two or more round buttons representing
application functions, only one of which can be selected -- hence the
term "radio button," named for the type of buttons on an automobile
radio. Seefor instructions.

Check Box: One of the three
"choice" objects (option menu, radio box, check box). A check box is
comprised of a label and one or more check boxes, each with its own
label. Each check box has a "binary" (on or off) state, and each is
independent of the other. Seefor instructions.

Gauge: One of two "scale"
objects (gauge, scale). A gauge is used to indicate a value. Seefor instructions.

Scale: One of two "scale"
objects (gauge, scale). A scale, like a gauge, indicates a value, but
a user can modify the value of a scale by moving the slider. Seefor instructions.

Separator: A horizontal or
vertical line used to indicate separate functions in an application
window. Seefor instructions.

Menu Bar: A horizontal bar
of menu buttons arrayed across the top of a main window. The buttons are
cascade buttons, for attaching menus. The default menu bar
includes File, Edit, and Help topics. You can change, delete, or add
to this group of topics. Seeandfor instructions for creating menus and attaching
them to menu buttons.

Text Field: A single-line
text-entry area with a label (compare to). Seefor instructions.

Label: A text string or
graphic icon which can be attached to an object for identification
purposes. Seefor instructions.

Scrolling List: An object
for listing selectable options. A scrolling list is comprised of a
variable-length list with scroll bars and an optional label. A list
can allow single or multiple selections, and it can include a pop-up
menu. Seefor instructions.

Spin Box: An object for
selecting from a number of choices, only one of which is visible at
any one time. A spin box is comprised of a text field, a label, and a
set of arrows for sequencing through the choices. Seefor instructions.
# See Also





# Object Information Area
object information areainformation area, object

&newline; &empty;

The Object Information Area provides information about the
object directly beneath the cursor -- either on one of the primary window
palettes or in the user interface. It includes the following
information fields:

* **Object Type** 

The type of object beneath the cursor (main window,
control pane, text field, for example). This field is active in the
App Builder primary window, so you can use it to identify object types in
the object palettes.
* **Object Name** 

The name of the interface object beneath the cursor.
This name, in combination with the module name, uniquely identifies
App Builder objects. Palette objects do not have names, so the field
will be blank if the cursor is over the App Builder primary window.
Note that all palette objects are given unique names when they are
instantiated in the interface; you can change the name in the property
editor for the object.
* **Position** 

The (x,y) pixel coordinates of the top-left corner of the
object beneath the cursor, measured in the coordinate system of the
object that contains it. If the object is a window object (main
window, custom dialog, or file selection dialog), the position will be
relative to the top-left corner of the monitor screen.

If the object is a pane that was dropped on the top-left corner of a
window, its position will be 0,0, since 0,0 are the coordinates of the
top-left corner of the parent window. A pane that is dropped on
another pane and made a layered pane also has coordinates of 0,0.

If the object is a control or a pane that has been made a child of a
control pane, its coordinates are measured from the top-left corner of
the parent object to the top-left corner of the child object.
* **Size** 

The size, in pixels, of the object beneath the cursor, in the
form "width X, height Y."
* **Cursor Position** 

The (x,y) pixel coordinate location of the cursor,
measured in the coordinate system of the object that contains it.
Every object, including controls, has its own coordinate system. Some
compound objects, comprised of more than one widget, have multiple
coordinate systems; a custom dialog, for instance, includes a control
pane, a tool bar, and buttons, each with its own coordinate system.
* **Editing Module** 

The name of the module currently being edited. Any
window dragged from the Windows palette becomes part of that module.
If more than one module is shown on the desktop, you can change the
current module by selecting an object in another module. Seefor instructions for opening a new module.

# Project Organizer


The Project Organizer is used to open, save, or close a project, and
to save, show, hide, import, export, or remove modules.
&newline; &empty;

* **Menu Bar** 

A menu bar which includes Project, Module, and Help
menus.
* **Location** 

A control pane with Project Path and Module Path fields;
indicates the full-path location of the current project and the
relative path to modules. The module will normally be in the same
folder as the project, and its path will be noted as "." ("dot,"
signifying the current folder).
* **Module Array** 

A draw area pane that depicts each of the modules in the
current project as a single icon with the module name beneath the App
Builder icon.


Seefor more about the Project Organizer and
its use.
# See Also











# Module Browser
browsermodule browser

The Module Browser (also called the browser) provides a hierarchical,
tree view of a module. The browser has the following components.
&newline; &empty;

* **Menu Bar** 

Includes Edit, View, and Help menus. Seefor
descriptions.
* **Module Name** 

Indicates the module being viewed. Can be changed
through the View menu.
* **Top-level View** 

Shows all direct children of the module --
windows, menus, and messages. A detailed view of each of the objects
selected here is shown in the detailed tree view.
* **Detailed Tree View** 

Shows a detailed view of the top-level objects
selected. All children of the top-level objects are shown.

# Code Generator Window
code generatorgenerating code

&newline;&empty;

The Code Generator Window is used to generate code for the created
interface and to make and run the completed application.

&newline; &empty;

* **Menu Bar** 

Includes File, Options, and Help menus; seefor descriptions. Note that the File menu includes choices that
correspond to each of the push buttons at the bottom of the Output
Pane.
* **Path** 

Indicates the path to the current project, which is included in
the title bar at the top of the window.
* **Output Pane** 

Refers to the text pane below this label. The results
when you click on the buttons below the pane are displayed in this
text pane.
* **Generate Code** 

A push button for generating code for the current
project The output for this action is displayed in the Output Pane.
* **Make** 

A push button for "making" the application for the current
project. The output for this action is displayed in the Output Pane.
* **Run** 

A push button for running the compiled application after
generating code and making the application. The output for this action
is displayed in the Output Pane. The primary window for the compiled
application will be displayed.
* **Make & Run** 

A push button for combining the functions of the first
three buttons (Generate Code, Make, Run). The output for this action
is displayed in the Output Pane. The primary window for the compiled
application will be displayed.
* **Abort** 

A push button for aborting the currently-running function. If
the compiled application is being run, clicking Abort quits the
application.
* **Term Pane** 

A term pane for performing any terminal-emulation
functions.

# See Also



# Code Generator Options Dialog Box
options dialog, code generatorcode generator options dialoggenerate code fordon't mergereport normal messagesmake argumentsrun time argumentsreset to defaults

The Code Generator Options dialog box, accessible from the Options
menu in the Code Generator window, is used to set options that
determine what will happen when various Code Generator functions are
performed.
&newline; &empty;

* **Project** 

The name of the current project.
* **Generate Code For** 

Radio buttons and a scrolling list for specifying
whether code will be generated for Entire Project, Main Only, Specific
Modules Only, or Specific Modules and Main. If one of the latter two
choices is specified, the modules in the scrolling list are active.
* **Don't Merge** 

A check box for specifying whether user-written code
will be merged into the generated code; if you check Don't Merge, any
user-written code will be discarded when code is generated.
* **Report Normal Messages** 

An option menu for choosing whether Normal
Messages will be displayed in the Output Pane when code is generated,
whether no messages will be generated (Be Silent), or whether all
messages will be displayed (Be Verbose).
* **Make Arguments** 

A text field for specifying what arguments will be
appended to the Make command when it is run in the Code Generator.
* **Run Time Arguments** 

A text field for specifying what arguments will be
appended to the Run command when it is run in the Code Generator.
* **Reset to Defaults** 

A push button for resetting all Code Generator
Options settings to their default values.


Seefor descriptions of the five buttons at the
bottom of the dialog box.
# See Also





# Code Generator Environment Options Dialog Box
code generator environment optionsenvironment options, code generatorvariable name, code generatorvalue, code generatorget, code generator valueset, code generator valuereset, code generator value

The Code Generator Environment Options dialog box, accessible from the
Options menu in the Code Generator window, is used for specifying a
Variable Name and a Value for the variable, which value will be used
for functions performed in the Code Generator window.
&newline; &empty;

* **Variable Name** 

A text field for typing the name of an environment
variable.
* **Value** 

A text field for specifying a value for the variable
specified in Variable Name. This value is only set for the Code
Generator window and has no effect on the value of the variable
outside of the Code Generator.
* **Get** 

A push button for getting the current Code Generator value of
Variable Name and displaying it in the Value text field.
* **Set** 

A push button for setting Variable Name to the value in Value.
This value is set for Code Generator window functions only.
* **Reset** 

A push button for resetting Value for Variable Name to its
value as set outside of the Code Generator.
* **Cancel** 

A push button for cancelling any changes made to Value and
closing the Environment Options dialog box.
* **Help** 

A push button for displaying on-item help for the dialog box.

# See Also





# Project Name Dialog Box


The Project Name dialog box is displayed when you choose New Project
from the File menu of the App Builder primary window or when you
choose New from the Module menu of the Project Organizer.

Type a name in the text field and click Apply.

Note thatprojectnames should
be all lower case.

&newline; &empty;
# Module Name Dialog Box


The Module Name dialog box is displayed when you select New Module from
the File menu or when you drag and drop a window on the desktop when
there is no currentmodule.
Type a name in the text field and click Apply.

&newline; &empty;
# File Selection Dialog Box


The File Selection dialog box title varies, depending on what function
is being performed (open, save, import, export), and different buttons
may be available, but the fields are the same on all file selection
dialog boxes.
&newline; &empty;

* **Enter path or folder name** 

A text field that indicates the current
folder (directory). If you change this field, it is updated when the
Update button is activated.
* **Filter** 

A text field for specifying the filter to be applied to file
names. The filter for a project file, for example, is *.bi[px], which
means that only files that end in .bip or .bix will be lised in the
Files field. If you change this field, it is updated when the
Update button is activated.
* **Folders** 

A scrolling list that lists the folders in the current
folder, which is displayed in the Enter path or folder name field. The
current folder is changed if you double-click on a folder name in the
Folders list or if you select a folder name and click Update. Note
that ".." is the UNIX designation for the folder level above the
current folder.
* **Files** 

A scrolling list for selecting one of the files in
the current folder that pass the Filter test (all end in .bip in this
case). The selected file name is displayed in the Enter file name
field.
* **Enter file name** 

is a text field for typing the name of the file
you wish to open, save, import, or export. If a file is selected in
the Files list, the file name is displayed here.
* **Open** 

(or Save, Import, or Export) performs the specified action and
closes the file selection dialog box.
* **Update** 

updates the fields in the file selection dialog box after you
have made changes; clicking Return when Update is the selected button
does the same thing.
* **Cancel** 

closes the file selection dialog box.
* **Help** 

displays on-line help information, if any, about the file
selection dialog box.

# See Also



# Message Dialog Boxes


Message dialog boxes are displayed in a number of circumstances, most
often serving as notification or warning about what will happen if a
specified action is performed.

In this example, you have attempted to close the current project
without saving some changes to the project.

* **Discard Changes** 

A push button for specifying that you want to
discard any changes made to the current project and continue the Close
Project function.
* **Cancel** 

A push button for specifying that you want to cancel the
Close Project operation.
* **Help** 

A push button for requesting on-item help explaining the
message, if help is available.

# Color Chooser Dialog Box
color chooserbackground colorforeground color

The Color Chooser dialog is displayed when you choose Color Chooser
from the Color [Background or Foreground] options menu in a property
editor.

&newline; &empty;

* **[palette of colors]** 

Drawn push buttons for selecting the background
or foreground color for the selected object in the property editor.
* **Color Name** 

A label indicating the name of the color selected in the
palette of colors. This name also appears in the Color [Background or
Foreground] text field of the property editor.
* **OK** 

A push button for applying the selection of the selected color
and dismissing the Color Chooser dialog box.
* **Cancel** 

A push button for cancelling the selection of a color and
dismissing the Color Chooser dialog box.
* **Help** 

A push button for requesting on-item help explaining the
Color Chooser, if help is available.

# App Builder Menus





























# App Builder File Menu


&newline;&empty;

&empty;
&newline; &empty; &newline; &empty; &newline; &empty;New Projectopens a new
project. &newline;Open Projectopens an existing project. &newline;Save Projectsaves the current project &newline;Save Project Assaves the current project
using a different name or in a different folder; use also to
encapsulate a project into a single file. &newline;New Modulecreates a new module within the
current project. &newline;Import Moduleimports an existing module into
the current project. &newline;Export Modulesaves the selected module as a
standalone file in either BIL or UIL format, unconnected to the
current project. &newline;Code Generatoropens the Code
Generator window. &newline;Project Organizeropens the Project
Organizer. &newline;Close Projectcloses the current
project. &newline;Exitquits App Builder.
# See Also



# App Builder Edit Menu


&newline; &empty;
The edit functions in the Edit menu, shown below, are also available
from the pop-up menus in the user interface and in the browser.

&newline; &empty; &newline; &empty; &newline; &empty;Undoundoes the last action in App Builder.
Actions that can be undone are: cut, delete, paste, group, ungroup,
move (object), and resize (object).
&newline;Cutcuts the selected object and places it on the
App Builder clipboard. &newline;Copycopies the selected object and places it
on the App Builder clipboard. &newline;Pastepastes the object on the App Builder clipboard in the
selected window or pane if the selected window or pane is a
legitimate target. &newline;Deletedeletes the selected object; the object
is not placed on the App Builder clipboard. &newline;
# See Also





# App Builder View Menu


&newline; &empty;

&newline; &empty; &newline; &empty; &newline; &empty;Module Browserbrings up the Module Browser
to view modules hierarchically. Each of the shown current modules is
included in the submenu. The one you choose will be displayed
in the browser. &newline;Next Layerdisplays the layer beneath the current
layer, if any. This item is inactive unless a layered pane is
selected. &newline;
# See Also



# App Builder Layout Menu


&newline; &empty;

&newline; &empty; &newline; &empty; &newline; &empty;

Aligndisplays an alignment tool; use to
temporarily align two or more selected control objects horizontally or
vertically. Seefor instructions. This item is
inactive if no control object is selected. &newline;Distributedisplays a tool for temporarily
adjusting the space between interface objects. Seefor instructions. This item is inactive if no control object is
selected. &newline;Groupcombines two or more selected control
objects for a variety of purposes, including interface layout. Seefor instructions. This item is inactive if no
control object is selected. &newline;Ungroupungroups the objects in the selected
group. Seefor instructions. This item is inactive
if no group is selected. &newline;Make Paned Windowmakes a single, paned window
out of two or more selected panes. Seefor
instructions. This item is inactive if no pane is selected. &newline;Unmake Paned Window"unmakes" a paned window,
leaving its constituent panes as separate objects. Seefor instructions. This item is inactive if a
paned window is not selected. &newline;
# See Also





# App Builder Editors Menu


&newline; &empty;

&newline; &empty; &newline; &empty; &newline; &empty;Propertiesdisplays the--
for changing properties of objects. &newline;Helpdisplays the-- for writing help
for objects in an interface. &newline;Menusdisplays the-- for creating
menus. &newline;Connectionsdisplays the--
for establishing programmatic connections between objects in an interface. &newline;Messagesdisplays the-- for
creating pop-up message dialog boxes for error and other conditions. &newline;Groupsdisplays the--
for grouping interface objects for layout purposes. &newline;Attachmentsdisplays the--
for attaching objects in an interface to each other for layout
purposes. &newline;Drag and Dropdisplays the-- for establishing drag-and-drop behavior for interface objects. &newline;Application Frameworkdisplays the-- for establishing settings for
Internationalization, Generated Code, Session Management, and
ToolTalk. &newline;
# See Also





# App Builder Help Menu


&newline; &empty;

&newline; &empty;
&newline; &empty;
&newline; &empty;Overviewdisplays introductory information
about App Builder.&newline;Tasksdisplays instructions for using App Builder.&newline;Referencedisplays descriptive information about
App Builder components.&newline;On Itemchanges the pointer to a question mark; click the
question mark pointer on an App Builder object for its description.&newline;Using Helpdisplays information about using Help.&newline;About Application Builderdisplays version and copyright
information about App Builder.
# App Builder User Interface Pop-up Menu


&newline; &empty;
The user interface pop-up menu, which is displayed when mouse button 3
is clicked or pressed in an interface window, is shown below.

&newline; &empty;

&newline; &empty;Propsdisplays a property editor -- revolving or
fixed, depending on which item in the submenu you choose --
with the object selected in the interface selected in the property
editor. This item is inactive if no object or more than one
object is selected in the interface. &newline;Browsedisplays the, with the objects
selected in the interface -- if any -- selected in the browser.&newline;Undo, Cut, Copy, Paste, Delete; see.&newline;Aligndisplays an alignment tool; use to
temporarily align two or more selected control objects
horizontally or vertically. Seefor instructions.
This item is inactive if no control object is selected. &newline;Distributedisplays a tool for temporarily
adjusting the space between interface objects. Seefor
instructions. This item is inactive if no control object is selected. &newline;Groupcombines two or more selected control
objects for a variety of purposes, including interface layout. Seefor instructions. This item is inactive if no
control object is selected. &newline;Ungroupungroups the objects in the selected group. Seefor instructions. This item is inactive if no
group is selected. &newline;Make Paned Windowmakes a single, paned window
out of two or more selected panes. Seefor
instructions. This item is inactive if no pane is selected. &newline;Unmake Paned Window"unmakes" a paned window,
leaving its constituent panes as separate objects. Seefor instructions. This item is inactive if a
paned window is not selected. &newline;Attachmentsdisplays the-- for attaching objects in an interface to each other for layout
purposes. This item is inactive if a window object is selected. &newline;NextLayerdisplays the next layer in a layered
pane; this item is inactive unless one of the panes of a layered pane
is selected. See. &newline;
# See Also











# App Builder Module Browser Pop-up Menu


&newline; &empty;
The browser interface pop-up menu, which is displayed when mouse button 3
is clicked or pressed in the Module Browser, is shown below.

&newline; &empty;

&empty;
&newline; &empty; &newline; &empty;Propsdisplays the property editor for the module
being displayed in the browser. &newline;Tearoff Browserdisplays a new browser, enabling
you to view more than one module. &newline;Undo, Cut, Copy, Paste, Delete; see. &newline;Groupcombines two or more selected control
objects for a variety of purposes, including interface layout. Seefor instructions. This item is inactive if no
control object is selected.&newline;Ungroupungroups the objects in the selected group. Seefor instructions. This item is inactive if no
group is selected. &newline;Make Paned Windowmakes a single, paned window
out of two or more selected panes. Seefor
instructions. This item is inactive if no pane is selected. &newline;Unmake Paned Window"unmakes" a paned window,
leaving its constituent panes as separate objects. Seefor instructions. This item is inactive if a
paned window is not selected. &newline;Attachmentsdisplays the-- for attaching objects in an interface to each other for layout
purposes. This item is inactive if a window object is selected. &newline;Expanddisplays the children of selected
collapsed parent objects. &newline;Expand Allexpands all collapsed parent objects. &newline;Collapse"undisplays" the children of selected
parent object. This enables you to see more of the interface in a smaller
space. &newline;
# App Builder Project Organizer Project Menu


&empty;
&newline; &empty; &newline; &empty; &newline; &empty;Openopens an existing project; same as Open
Project in App Builder primary window File menu. &newline;Newcreates a new project; same as New Project in App Builder
primary window File menu. &newline;Savesaves the current project; same as Save
Project in App Builder primary window File menu.&newline;Save Assaves the current project
using a different name or in a different folder; use also to
encapsulate a project into a single file. Same as Save Project As in
App Builder primary window File menu. &newline;Close Projectcloses the current project; same as
Save Project As in App Builder primary window File menu. &newline;Closecloses the Project Organizer.&newline;
# App Builder Project Organizer Module Menu


&empty;
&newline; &empty; &newline; &empty; &newline; &empty;Newcreates a new module within the current
project; same as New Module in App Builder primary window File
menu.&newline;Savesaves the selected module if it has changed
since the last time it was saved. &newline;Save Asrenames the selected module. &newline;Showdisplays the interface for the selected
modules. &newline;Hidehides the interface for the selected
modules. &newline;Browsedisplays the Module Browser for the
selected modules. If more than one module is selected, a separate
browser is displayed for each selected module.&newline;Importimports an existing module into the
current project; same as Import Module in App Builder primary window
File menu. &newline;Exportsaves the selected module as a standalone
file in either BIL or UIL format, unconnected to the current project;
same as Export Module in App Builder primary window File menu.
&newline;Removeremoves the selected modules from the
current project. &newline;
# See Also



# App Builder Module Browser Edit Menu


This is the same as the Edit menu in the App Builder primary window.
Objects can be selected in the Module Browser and edited just as they are in
the interface.

&empty;
&newline; &empty; &newline; &empty;&newline; &empty;Undoundoes the last action in App Builder;
actions that can be undone are: cut, delete, paste, group, ungroup,
move (object), and resize (object).
&newline;Cutcuts the selected object and places it on the
App Builder clipboard. &newline;Copycopies the selected object and places it
on the App Builder clipboard. &newline;Pastepastes the object on the App Builder clipboard in the
selected window or pane if the selected window or pane is a
legitimate target. &newline;Deletedeletes the selected object; the object
is not placed on the App Builder clipboard. &newline;
# App Builder Module Browser View Menu


&empty;
&newline; &empty; &newline; &empty; &newline; &empty;Horizontaldisplays child objects to the right of
their parent object. Toggles with Vertical, which displays child
objects below their parent object. Vertical is the default view. &newline;Hide Object Glyphhides the icons/glyphs that
represent the objects in the interface. Toggles with Show Object
Glyph, which is the default. &newline;Show Object Typedisplays the object types of
objects in the interface. Toggles with Hide Object Type, which is the
default. &newline;Collapse"undisplays" the children of selected
parent objects. This enables you to see more of the interface in a smaller
space.&newline;Expanddisplays the children of selected
collapsed parent objects. &newline;Expand Allexpands all collapsed parent objects. &newline;Moduledisplays the module chosen from the
submenu. &newline;Finddisplays a Find Object dialog box, for finding
objects by object name; if the object is found, the object is selected
and the canvas scrolls to show the object. &newline;Tearoff Browserdisplays a new browser, enabling
you to view more than one module. &newline;
# App Builder Code Generator File Menu


&newline; &empty;

&newline; &empty; &newline; &empty; &newline; &empty;Make & Runmakesthe project
according to theMakefilein the project folder
and runs the executable. Creates aMakefileif
none exists&newline;Generate Codegenerates C code for the entire
project, if Entire Project submenu item is selected, or for
whatever is specified in the Code Generator Options dialog box, if
According to Options submenu item is selected. &newline;Makemakesthe project according to theMakefilein the project folder. Creates aMakefileif none exists. &newline;Runruns the executable for the current project,
if it exists. If it does not exist, a message dialog box is displayed, giving
you the option of cancelling the Run operation or to build the
application and then Run. If you click Build, the project will be
compiled and run, just as if you had selected Make & Run. &newline;Abortaborts the current process. Abort is
inactive if no Code Generator process is running. &newline;CD to Projectchanges the folder (directory)
location in the Term Pane at the bottom of the Code Generator window
to location of the current project. Used when the current project is
in a different folder than the folder in which App Builder was opened.
&newline;Closecloses the Code Generator window. &newline;
# App Builder Code Generator Options Menu


&newline; &empty;

&newline; &empty; &newline; &empty; &newline; &empty;Generatordisplays the Code Generator Options
dialog, for specifying what code is generated when Generate Code is
selected in the Code Generator window, whether code is merged,
and what arguments should be included when Make or Run are selected.
&newline;Environmentdisplays the Environment Options
dialog box, used to set variables to be used when performing functions in
the Code Generator window. &newline;
# See Also





# App Builder Editors
editors

&newline;&empty;




















# Revolving Property Editor


&newline; &empty; Used to edit theproperties(look and
functionality) of interface objects, property editors enable you to
customize your application interface. The property editor for a
separator, which includes the properties common to almost all property
editors, is shown below. Seefor descriptions of
these common properties. Note that the Group and Menu property editors
are described in this section, even though they are included in the
Editors menu of the App Builder primary window.

&newline; &empty;
# See Also









# Property Editor: Universal Properties
object typeobjects listobject nameinitial statecolor chooserbackground colorforeground coloractive initial stateinactive initial stateinvisible initial statevisible initial state

The property editor for each of the App Builder objects is unique, but
there are a number of properties common to almost all of them.

* **Object Type** 

An option menu for choosing the type of property editor
to be displayed. Properties change depending on which object type is
chosen.
* **Objects [list]** 

Lists the objects of the selected type in
the current project. The list displays the full, unique name for each
object, which is comprised of the name of the module in which the object
exists, two colons, and the Object Name.
* **Object Name** 

Displays the default name or the name
given by you to the object selected in the Objects list.
* **Initial State, Visible** 

Specifies whether the
selected object is visible when the application starts up; all objects
except a custom dialog are visible by default.
* **Initial State, Active** 

A check box for specifying whether the object
selected is "active" when the application starts up. An inactive
object is not functional: it is dimmed and no functions can be
activated from the object.
* **Color &sigspace;-- &sigspace; Background** 

Specifies the background
color of the selected object. You can either type in a known color
name or choose Color Chooser from the menu and select a color from the
palette displayed.
* **Color &sigspace; -- &sigspace; Foreground** 

Specifies the foreground
color of the selected object. You can either type in a known color
name or choose Color Chooser from the menu and select a color from the
palette displayed.

# See Also







# Property Editor: Common Properties
border framegeometrygraphic filenameitems, property editorlabellabel positionlabel typemenu titlepop-up menuposition, labelposition, xyscroll barssize policysize, W,H

The following properties are common to three or more property editors.

* **Border Frame** 

Determines the type of border, if any,
around certain objects. Choices are None, Shadow Out, Shadow In,
Etched Out, and Etched In.
* **Geometry** 

Indicates the X and Y location of the selected
object, and the W(idth) and H(eight) of the object. X and Y values
specify the position of the selected object in relation to its
parent. The values are in pixels and are measured from the top-left
corner of the parent object to the top-left corner of the child
object. W and H values are in pixels.
* **Graphic Filename** 

Indicates the name of the pixmap
(.pm) or bitmap (.bm) file that contains the graphic to be used as the
label for the selected object or item. This property is available only
if Label Type or Item Label Type is "Graphic."
* **Item Label Type** 

Specifies the type of label
(String or Graphic) for the selected item in the Items list. If
Graphic is chosen, "Label" becomes "Graphic Filename."
* **Item State, Active** 

Specifies whether the selected
item will be active when the compiled application is
opened.
* **Items** 

Lists the labels that represent the items in the
list. When an item is selected in the Items list, its label is
displayed in the Label or Graphic Filename field.
* **Label** 

Specifies the label for the selected object or
item. "Label" becomes "Graphic Filename" if Graphic Label Type is
chosen. Label is inactive in the Button property editor if Arrow Label
Type is chosen.
* **Label Type** 

Specifies the type of label (String, Graphic, or Arrow)
for the selected object. If Graphic is chosen, "Label" becomes
"Graphic Filename." If you choose Arrow, the label in the Button
property editor becomes an arrow and the Arrow Direction property
becomes active.
* **Menu Title** 

Specifies the (optional) title of the
pop-up menu, if any.
* **Popup or Pulldown Menu** 

A menu button and a text field for creating,
attaching, de-attaching, or editing a pop-up or pull-down menu for the
selected object. When the Menus button is clicked, a menu with four
choices (None, Create New Menu, Menus, Edit Current) is displayed.
Menus and Edit Current are inactive if no menus exist in the current
project. If a menu is already attached to the selected object, the
menu name will be displayed in the text field.
* **Position [Label]** 

Specifies the position (Left
or Above) of the label in relation to the selected object. This
Position option menu is next to the Label Type option menu.
* **Position [XY]** 

Indicates the X and Y location of the selected
object in relation to its parent. The values are in pixels and are
measured from the top-left corner of the parent object to the top-left
corner of the child object.
* **Scrollbars** 

Specifies when scroll bars should be
attached to the selected pane. The choices are Never and Always for a
term pane or a text pane, and Never, When Needed, and Always for a
draw area pane.
* **Size** 

Specifies the absolute W(idth) and H(eight) of the window or
pane. These values change if you resize the window or pane manually in
the interface. For a term pane or a text pane, there is an option menu
for choosing Characters or Pixels as the unit value.
* **Size Policy** 

Specifies whether the selected
object should retain a fixed size or if it should become bigger or
smaller depending on the contents of the object. The choices are Size
of Label and Fixed for buttons and labels, Fit Contents and Fixed for
main windows and custom dialogs.

# See Also







# Property Editor: Common Buttons
tear-off property editorattachments editor buttonhelp editor buttonconnections buttonok buttonapply buttonreset buttoncancel buttonhelp button

The following functional push buttons or menu buttons are common to
many property editors. The buttons at the bottom of the property
editors (OK, Apply, Reset, Cancel, and Help) are common to other
editors and dialog boxes.

* **Tear-off** 

Displays a property editor of the selected type; use this
when you want to edit a specific object type while viewing other types
of objects in the Revolving Property Editor.
* **Add Item** 

Adds an item after the selected item in
the Items list. Added items are given default names starting with
"Item1" and incrementing, as needed. By default, items are added after
the selected item.
* **Edit** 

Performs edit functions (Add After, Add
Before, Change, Cut, Copy, Paste, Delete) in a list. Add After and Add
Before add an item to the list either after or before the selected
item. Change applies the change that you have made. Cut, Copy, Paste
and Delete act on the selected item, in the normal way: Cut and Copy
place the selected item in a buffer, ready for Paste. Delete removes
the item, but does not place it in a buffer.
* **Attachments** 

Displays the; there is no
Attachments button on the Main Window, Menubar, Custom Dialog, and
Paned Window property editors.
* **Help Text** 

Displays the.
* **Connections** 

Displays the.
* **OK** 

Applies the changes made to the selected object and
dismisses the editor; changes made are marked with change
bars at the left side of the editor.
* **Apply** 

Applies the changes made to the selected object,
but does not dismiss the editor.
* **Reset** 

Resets all changes made since the last Apply.
* **Cancel** 

Resets all changes made since the last Apply and dismisses the
editor.
* **Help** 

Displays on-item help for the editor.

# See Also







# Individual Property Editors


An individual property editor is displayed by double-clicking on an
object in the interface or the browser, by selecting an object of the
desired type and choosing Properties from the Editors menu on the App
Builder primary window or choosing Props from the interface or browser
pop-up menu, or by choosing the desired object type from the options
menu at the top of the Revolving Property Editor.








































# See Also





# Button Property Editor
button property editordrawn button property editormenu button property editorpush button property editorproperty editorbutton typebutton: drawn, menu, pushdrawn buttonmenu buttonpush buttonlabel alignmentarrow directionpull-down menu

Only properties unique to a button object are described here. Seefor descriptions of Object Type, Objects, Object Name,
Initial State, and Color. Seefor
descriptions of Label Type, Label, Size Policy, and Geometry.

* **Button Type** 

Specifies what kind of button (Push,
Drawn, Menu) the selected button should be. Push button is the
default. Selecting Menu transforms the push button into a menu button,
as if you had dragged and dropped a menu button from the controls
palette. The Pulldown Menu property becomes active if you select Menu. Seefor descriptions of button types.
* **Label Alignment** 

Specifies the alignment (Left,
Right, Centered) of the button label within the button border frame.
Label Alignment is relevant only if Fixed is selected as Size Policy.
This menu is inactive if Arrow Label Type is chosen.
* **Arrow Direction** 

Specifies which direction (Up,
Down, Left, Right) the arrow should point if Arrow Label Type is
chosen.
* **Pulldown Menu** 

A menu button (labelled "Menus") and a text field for
creating, attaching, de-attaching, or editing a menu for the selected
button. Active only if Menu Button Type is chosen. When the Menus
button is clicked, a menu with four choices (None, Create New Menu,
Menus, Edit Current) is displayed. Menus and Edit Current items are
inactive if no menus exist in the current project. If a menu is
already attached to the selected button, the menu name will be
displayed in the text field.

# See Also











# Choice Property Editor
choice property editoroption menu property editorradio box property editorproperty editorproperty editor, combo boxproperty editor, option menuproperty editor, radio boxchoice typerows/columnscolumns, choice property editoritem state, choice property editoractive itemselected itemitem, activeitem, selected

Only properties unique to a choice object (Radio Box, Check Box,
Option Menu) are described here. Seefor
descriptions of Object Type, Objects, Object Name, Initial State, and
Color. Seefor descriptions of Label Type,
[Label] Position, Label, Items, [Item] Label, Item State (Active), and
Position [XY].

* **Choice Type** 

Specifies which type of choice
object (Radio Box, Check Box, or Option Menu) the selected object
should be. The object changes form depending on which you choose. Note
that there is a control object for each of the choice types in the
Controls palette. Seefor descriptions of
choice types.
* **Rows/Columns** 

Specifies whether the radio box or check box should be
laid out in rows or columns, and how many rows or columns there should
be. Not relevant for an option menu.
* **Item State, Selected** 

Specifies whether the
selected item will be selected when the compiled application is
opened. Only one item can be selected. For a check box or a radio box
object, the selected item will be marked as selected; for an option
menu, the label for the selected object will be displayed in the
option menu when the application is opened.

# See Also









# Combo Box Property Editor
combo box property editorproperty editorcombo box typestatic combo box typeeditable combo box typelabel selected, combo boxselected label, combo box

Only properties unique to a combo box are described here. Seefor descriptions of Object Type, Objects, Object Name,
Initial State, and Color. Seefor
descriptions of Label Type, [Label] Position, Label, Items, [Item]
Label, and Position [XY].

* **Combo Box Type** 

Specifies whether the text field
for the selected combo box will be Static or Editable in the compiled
application. If Editable is selected, code must be written to
implement the edit functionality.
* **[Label] Selected** 

Specifies which item will be
selected when the compiled application is opened.
* **Width** 

Specifies whether the combo box shrinks or
grows to accommodate the Longest Item in the list, or if the W(idth) of
the box is Fixed. If Fixed is selected, the W(idth) value can be edited.

# See Also









# Control Pane Property Editor
control pane property editorproperty editor

There are no properties unique to a control pane. Seefor descriptions of Object Type, Objects, Object Name,
Initial State, and Color. Seefor
descriptions of Border Frame, Size Policy, Geometry, Popup Menu, and
Menu Title.
# See Also









# Custom Dialog Property Editor
custom dialog property editorproperty editordialog titlewindow parentdialog areasdefault button, custom dialoghelp button

Only properties unique to a custom dialog object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Size Policy and Size.

* **Dialog Title** 

The title that appears at the top of the custom dialog.
* **Window Parent** 

An option menu for specifying a parent main window
for the selected custom dialog. Choices are None and any main window
in the project. If a main window is specified as a window parent, the
custom dialog will be iconified and de-iconified with the main window.
Note that this functionality does not work in test mode, but it does in
the compiled application.
* **User Resize Mode** 

Specifies whether the window is Fixed or Adjustable
(whether it can be resized in the compiled application).
* **Dialog Areas** 

Specifies whether a custom dialog
includes a Button Panel (three buttons, by default) and a Footer area.
* **Default Button** 

Specifies one of the dialog
buttons as the selected button, by default. The function represented
by the selected button will be performed if you press Return while the
mouse cursor is in the custom dialog in the compiled application.
* **Help Button** 

Specifies one of the dialog buttons
as the help button. Seefor a description of the
Help Editor andfor instructions for creating help.

# See Also









# Draw Area Pane Property Editor
draw area pane property editorproperty editorcanvas size, draw area panetotal canvas size, draw area pane

Only properties unique to a draw area pane object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Scrollbars, Border Frame, Geometry, Popup Menu,
and Menu Title.

* **Total Canvas Size** 

Specifies the W(idth) and
H(eight) of the draw area canvas. Note that only a portion of the
canvas will be visible if the draw area pane's size is smaller than
the canvas size (400 by 400 pixels, by default). You can use the
scroll bars to view other parts of the canvas.

# See Also







# File Selection Dialog Property Editor
file selection dialog property editorproperty editorwindow parent, file selection dialogmain window parentinitial folderfolder, initialinitial directorysearch patternOK button labelpopdown behaviorOK button

Only properties unique to a file selection dialog object are described
here. Seefor descriptions of Object Type,
Objects, Object Name, Initial State, and Color.

* **Window Parent** 

Specifies the main window parent of
the file selection dialog. When displayed, the file selection dialog
will appear over its main window. By default, the Primary Main Window is
the parent of all file selection dialogs. See.
* **Dialog Title** 

Specifies the title that appears in
the title bar at the top of the file selection dialog.
* **Initial Directory** 

Specifies the folder (directory)
set as the starting value in the Path field of the file selection
dialog.
* **Search Pattern Type** 

Specifies whether files,
directories (folders), or both will be listed in the Files list of the
file selection dialog.
* **Search Pattern** 

Specifies the value of the Filter
field in the file selection dialog. The Filter value limits the files
that will be listed in the Files field. The default value is *
(star), which means all files in the current folder will be
listed. The Filter value for the Import Module file selection dialog
in App Builder is *.bil, which means that only files that end in .bil
will be listed.
* **OK Button Label** 

Specifies the label that will
appear on the button in the left-most position at the bottom of the
file selection dialog, normally labelled "OK." Clicking this button
completes the file selection process and dismisses the file selection
dialog. This button is labelled "Import" for the Import Module file
selection dialog in App Builder.
* **Popdown Behavior** 

Specifies whether the file selection dialog will be
sutomatically dismissed (the default) when the OK button is clicked.

# See Also







# Group Property Editor
group property editorproperty editorgroup namelayout type, grouprows layoutcolumn layoutlayout, rows or columnsgroup layoutvertical alignment, grouphorizontal alignment, groupalignment, groupspacing, groupgroup spacing

Used to modify the layout and framing of groups, the Group Property
Editor can be displayed by choosing Groups from the Editors menu of
the App Builder primary window or by choosing Group from the Revolving
Property Editor Object Type option menu. A group, unlike most of the
objects edited in the Revolving Property Editor, is a created object
and is not available from an object palette. Seefor instructions.

Only properties unique to a group object are described here. Seefor descriptions of Object Type, Objects, Initial
State, and Color. Seefor descriptions of
Border Frame and Position.

Note that choosing Groups from the Editors menu in the App Builder primary
window is the same as clicking the Tear-off button in the Revolving
Property Editor when the Object Type is Group.

* **Group Name** 

Displays the default name or the name
given by you to the group selected in the Objects list.
* **Layout Type** 

Specifies As-Is, Vertical,
Horizontal, or Row/Column layout of the objects in the selected group.
* **Rows Columns** 

Specifies whether the primary layout will be by rows
(vertical layout) or columns (horizontal layout), and how many rows or
columns to display. Active only if Layout Type is Row/Column.
* **Vert Alignment** 

Specifies left-edge, colon/label,
center-line, or right-edge alignment of the objects in the selected
group. Active only if Layout Type is Vertical or Row/Column.
* **Spacing** 

Specifies the number of pixels separating
the objects in the selected group. Vert[ical] Alignment Spacing is
active only if Layout Type is Vertical or Row/Column. Horiz[ontal]
Alignment Spacing is active only if Layout Type is Horizontal or
Row/Column.
* **Horiz Alignment** 

Specifies top-edge, center-line,
or bottom-edge alignment of the objects in the selected group. Active
only if Layout Type is Horizontal or Row/Column.

# See Also













# Label Property Editor
label property editorproperty editorlabel alignmentlabel, border

Only properties unique to a label object are described here. Seefor descriptions of Object Type, Objects, Object
Name, Initial State, and Color. Seefor
descriptions of Label Type, Label, Size Policy, and Geometry.

Note that no border appears around a label in the compiled
application. Seeif you want a border around a
label.

&newline; &empty;

* **Label Alignment** 

Specifies the alignment (Left, Right, Centered) of
the label within its margins. Label Alignment is relevant only if
Fixed is selected as Size Policy.

# See Also









# List Property Editor
list property editorproperty editorselection modebrowse selectsingle selectmultiple selectbrowse multiple selectheight, list property editorwidth, list property edititem selectedselected item, listlist, selected item

Only properties unique to a list object are described here. Seefor descriptions of Object Type, Objects, Object Name,
Initial State, and Color. Seefor
descriptions of Label Type, [Label] Position, Label, Items, [Item]
Label, Position [XY], Popup Menu, and Menu Title.

* **Selection Mode** 

Specifies how objects can be
selected in a scrolling list. Choices are Single Select, Browse
Select, Multiple Select, and Browse Multiple Select.

In Single Select mode, only one item can be selected, by clicking
mouse button 1.

In Browse Select mode, one item can be selected, but you can press
mouse button 1 and drag through the list until the item you want is
selected.

In Multiple Select mode, you can make multiple, discontiguous
selections with mouse button 1.

In Browse Multiple Select mode, you can drag the cursor over items to
make multiple, contiguous selections, and you can make a multiple,
contiguous selection between a selected item and the cursor location
with BSelect (Shift-mouse button 1).
* **[Item] Selected** 

Specifies whether an item will be
selected at application startup.
* **Width** 

Specifies whether the list object shrinks or grows to
accommodate the Longest Item in the list, or if the W(idth) of the box
is Fixed. If Fixed is selected, the W(idth) value can be edited.
* **Height** 

Specifies the number of text Lines or Pixels high the list
is.

# See Also







# Main Window Property Editor
main window property editorproperty editor windowwindow titleicon fileicon mask fileicon labelwindow areas

Only properties unique to a main window object are described here. Seefor descriptions of Object Type, Objects, Object
Name, Initial State (Visible), and Color. Seefor descriptions of Size Policy and Size.

* **Window Title** 

Specifies the title that appears at the top of the main
window.
* **Icon File** 

Specifies the name of the graphics file that
contains the graphical representation of the application icon -- the
object that is displayed when the application is "iconified" by
clicking on the minimize button in the title bar.
* **Icon Mask File** 

Specifies the name of the graphics file
that contains the bitmap that determines the shape of the visible
representation of the icon beneath the icon mask. The icon mask acts
like a stencil, allowing only the pixels in the icon that correspond
to pixels in the mask to be visible.
* **Icon Label** 

Specifies the text label that appears beneath the
application icon.
* **User Resize Mode** 

Specifies whether the window size is Fixed or
Adjustable (whether it can be resized in the compiled application).
* **Window Areas** 

Specifies whether the main window will have a menu bar,
a tool bar, or a footer.

Note that a tool bar or a footer will show up as a control pane object
in the Revolving Property Editor. You will probably want to add
controls, such as the radio buttons in the App Builder primary window
Build/Test tool bar, to a tool bar, and to make connections between
the controls and programmatic functions. Code will have to be
written to make a tool bar or footer functional.
* **Initial State, Iconic** 

Specifies whether the window
is displayed as a window or an icon when the compiled application is
opened.

# See Also









# Menu Property Editor
menu property editorproperty editoreditor, menucreated object, menumenu objectadd new menunew menu, addtearoff menumenu, tearoffitem label typeitem mnemonicmnemonic, menu itemacceleratormenu item acceleratorline style, menu item separatorseparator, menu item label typemenu item label typelabel type, menu itemitem submenusubmenu, item

Used to create menus, the Menu Property Editor can be displayed by
choosing Menus from the Editors menu of the App Builder primary window,
by choosing Menu from the Revolving Property Editor Object Type option
menu, or by choosing Create New Menu from the Popup Menu option menu
in a property editor. A menu, unlike most of the objects edited in
the Revolving Property Editor, is a created object and is not
available from an object palette.

Only properties unique to a menu object are described here. Seefor descriptions of Object Type, Objects, Object Name,
and Color. Seefor descriptions of Items,
Label, and Item State (Active).

Note that choosing Menus from the Editors menu in the App Builder
primary window is the same as clicking the Tear-off button in the
Revolving Property Editor when the Object Type is Menu.

* **Add New Menu** 

Adds a new menu to the list of menus.
* **Edit** 

Performs edit functions (Cut, Copy, Paste, Delete) on the
selected item in the list of menu objects. Cut and Copy place the
selected item in a buffer, ready for Paste. Delete removes the item,
but does not place it in a buffer.
* **Tearoff** 

Specifies whether Tearoff is Enabled or Disabled -- whether
the selected menu will be "postable." That is, will the menu be
displayed "permanently" if you click on the Tearoff indicator (a
dotted line)?
* **Item Label Type** 

Specifies the type of label
(String, Graphic, or Separator) for the item selected in the Items
list. If Graphic is chosen, "Label" becomes "Graphic Filename." If
Separator is chosen, Label or Graphic Filename becomes inactive and
Line Style becomes active. A Separator menu item is used to create a
visual division in a menu, such as that seen in the Editors menu of
the App Builder primary window.
* **Item Mnemonic** 

Specifies one of the letters in the selected item as a
keyboard shortcut for choosing the item when the menu is posted. The
letter specified will be underlined. Pressing the mnemonic letter when
the menu is posted will cause that item to be chosen. Note that case
is significant and that a particular letter can be used as a mnemonic
only once within a menu.
* **[Item] Accelerator** 

Specifies a keyboard shortcut for choosing the
selected item. An accelerator is comprised of a prefix (Ctrl, Alt,
Meta, or Shift) plus <Key> plus a letter (upper or lower case). To
make Control-X an accelerator, for instance, type the following:

Ctrl<Key>x

When you display the menu in test mode or in the compiled application,
"Ctrl+x" will be included to the right of the menu item label. If you
press the Control key and type x with the cursor in the window that
contains the menu, the specified action will be performed.

You can combine the Shift key with one of the other keys to form a
compound prefix, if you wish. To make Shift Control-X an accelerator,
type the following:

Shift Ctrl<Key>x
* **Line Style** 

Specifies the type of line style for the selected
separator item; active only when Item Label Type is Separator. Choices
are None, Etched In, Etched Out, Etched In Dash, Etched Out Dash,
Single Line, Double Line, Single Dashed Line, and Double Dashed Line.
A separator of the chosen line style will be displayed in the menu
instead of a graphic or text label.
* **Item SubMenu** 

A menu button and a text field for attaching,
de-attaching, creating, or editing a sub-menu for the selected item in
the Items list. If a sub-menu is attached to the selected item, the
name of the sub-menu will be displayed in the text field.

# Menu Button Property Editor


A menu button is a special type of button; see.
# See Also



# Menubar Property Editor
menubar property editorproperty editoritem mnemonicmnemonic, menu itemhelp menu

Only properties unique to a menubar object are described here. Seefor descriptions of Object Type, Objects, Object Name,
Initial State, and Color. Seefor
descriptions of Items, Item Label Type, Label, Pulldown Menu, and Item
State (Active).

* **Item Mnemonic** 

Specifies one of the letters in the selected item as a
keyboard shortcut for displaying the menu. The letter specified will
be underlined in the menu bar. In test mode and in the compiled
application, the menu will be displayed if you hold down the Alt key
and press the mnemonic letter while the mouse cursor is over the
window that contains the menu bar.
* **Item State, Is Help Item** 

Specifies that the selected item is the
Help menu. The Help menu appears at the right edge of the menu bar and
has a built-in connection to the online help mechanism. The item
labelled "Help" is the help button, by default.

# See Also











# Paned Window Property Editor
paned window property editorproperty editorpane geometry, paned windowpane height, paned window

A paned window, unlike most of the objects edited in the Revolving
Property Editor, is a created object and is not available from an
object palette; seefor instructions for
creating a paned window.

Only properties unique to a paned window object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, and Initial State.

* **Panes** 

Lists the panes that comprise the paned window.
* **Pane Geometry** 

Displays the W(idth) and H(eight) of the pane selected
in the Panes list.
* **Pane Height** 

Specifies the Min(imum) and Max(imum) height (in pixels)
of the selected pane. These values determine the limits for the panes
when you move the sash between panes.

# See Also









# Scale Property Editor
scale property editorproperty editorgauge property editorproperty editor, gaugescale typeorientation, scalescale orientationorientation, gaugegauge orientationdirection, gauge or scalescale directiongauge directionvalue range, gauge or scaledecimal points, gauge or scaleshow value, gauge or scale

Only properties unique to a scale or gauge object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Label Type, [Label] Position, and Label.

* **Scale Type** 

Specifies Scale or Gauge. A scale
includes a slider and is modifiable by a user (in the compiled
application or in test mode). A gauge indicates a
value, does not include a slider, and is not modifiable by a user.
* **Orientation** 

Specifies whether the scale object
will be displayed in Horizontal or Vertical orientation.
* **Direction** 

Specifies Left to Right or Right to
Left incrementing of value for a horizontal scale object, Bottom to
Top or Top to Bottom incrementing of value for a vertical scale
object.
* **Value Range** 

Specifies Min(imum), Max(imum, and Incr(ement) values
for a scale object. All values must be integers. The increment value
is used when you click with the mouse at either end of the scale
object (in the compiled application or in test mode). See Decimal
Points.
* **Decimal Points** 

Specifies the number of decimal places to shift the
scale value when displaying it (if Show Value is checked). For
example, a scale value of 250 with a Decimal Points value of 1 would
display as 25.0; a scale value of 250 with a Decimal Points value of 2
would display as 2.50.
* **Show Value** 

Specifies whether the numerical value of the scale
position will be displayed. See Decimal Points above.

# See Also







# Separator Property Editor
separator property editorproperty editororientation, separatorline style, separatorseparator line styleseparator orientation

Only properties unique to a separator object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor a description of Geometry.

* **Orientation** 

Specifies whether the separator object will be
displayed in Horizontal or Vertical orientation.
* **Line Style** 

Specifies the type of line style for the separator.
Choices are None, Etched In, Etched Out, Etched In Dash, Etched Out
Dash, Single Line, Double Line, Single Dashed Line, and Double Dashed
Line.

# See Also







# Spin Box Property Editor
spin box property editorproperty editorspin box typenumeric spin boxstring list spin boxarrow style, spin boxspin box arrow stylevalue range, spin boxspin box value rangedecimal points, spin box

Only properties unique to a spin box object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Label Type, [Label] Position, Label, Items, [Item]
Label, and Geometry.

* **Spin Box Type** 

Specifies the type of spin box. If Numeric is chosen,
the Items, Label, Add Item, Edit, and Selected properties are
inactive. If String List is chosen, the Value Range, Initial Value,
and Decimal Points properties are inactive.
* **Arrow Style** 

Specifies the style of arrow to be displayed on the spin
box. Choices are Flat Beginning, Flat End, Beginning, End, and Split.
* **Value Range** 

Specifies Min(imum), Max(imum, and Incr(ement) values
for a spin box object. All values must be integers. The increment
value is used when you click with the mouse on one of the spin box
arrows (in the compiled application or in test mode). Value Range is
inactive if Spin Box Type is String List. See Decimal Points.
* **Initial Value** 

Specifies the starting value in the spin box in the
compiled application. Initial Value is inactive if Spin Box Type is
String List.
* **Decimal Points** 

Specifies the number of decimal places to shift the
spin box value when displaying it. For example, a spin box value of
250 with a Decimal Points value of 1 would display as 25.0; a spin box
value of 250 with a Decimal Points value of 2 would display as 2.50.
Decimal Points is inactive if Spin Box Type is String List.
* **[Item], Selected** 

Specifies whether the selected item will be
selected when the compiled application is opened. Only one item can be
selected. Selected is inactive if Spin Box Type is Numeric.

# See Also







# Term Pane Property Editor
term pane property editorproperty editorscroll bars, term paneterm pane scroll barsprocess string, term paneterm pane process string

Only properties unique to a term pane object are described here. Seefor descriptions of Object Type, Objects, Object
Name, Initial State, and Color. Seefor
descriptions of Scrollbars, Border Frame, Position [XY], Size, Popup
Menu, and Menu Title.

* **Process String** 

A text field for specifying the process (command)
that will be run in the term pane in the compiled application. The
default value is/bin/csh.

# See Also







# Text Field Property Editor
text field property editorproperty editoroperation, text fieldtext field, editable or read-onlytext field, read-only or editabletext field, maximum charsmaximum chars, text fieldcharacters, maximum, text fieldtext field, initial valueinitial value, text field

Only properties unique to a text field object are described here.
Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Label Type, [Label] Position, Label, Position
[XY], and Width.

* **Operation** 

Specifies whether the text field in the compiled
application and in test mode will be Editable or Read-Only.
* **Maximum Chars** 

Specifies the maximum number of characters that can be
typed in the text field. This field is independent of the W(idth)
field, which specifies the width of the displayed text.
* **Initial Value** 

Specifies the initial value to be displayed in the
text field in the compiled application or in test mode.

# See Also







# Text Pane Property Editor
text pane property editorproperty editoroperation, text panetext pane, editable or read-onlytext pane, read-only or editabletext pane, initial valueinitial value, text paneword wrap, text panetext pane, word wrap

Only properties unique to a text pane object are described here. Seefor descriptions of Object Type, Objects, Object
Name, Initial State, and Color. Seefor
descriptions of Border Frame, Position [XY], Size, Popup Menu, and
Menu Title.

* **Operation** 

Specifies whether the text pane in the compiled
application and in test mode will be Editable or Read-Only.
* **Word Wrap** 

Specifies whether words will be wrapped to the following
line when the Size W(idth) value is reached.
* **Initial Value** 

Specifies the initial value to be displayed in the
text pane in the compiled application or in test mode.

# See Also







# Help Editor
help editoreditor, helphelp textvolume name, help editorlocation ID, help editor

Used to create on-item help for interface objects and to specify the
name and location of a help volume. The Help Editor
is described below the figure.
&newline; &empty;

* **Object Type** 

Specifies the type of object for which on-item help is
to be written.
* **Objects** 

Lists all of the objects of the type chosen as Object Type.
* **Help Text** 

The on-item help text for the object selected in the
Objects list.
* **Volume Name** 

Specifies the name of the HelpTag help
volume to be accessed if the More button is clicked in the on-item
help window displayed for the selected object. Note that on-item Help
Text and Location ID are required.
* **Location ID** 

Specifies a HelpTag location ID in the help volume named
in Volume Name; this field is required if Volume Name is included.


Seefor descriptions of the buttons at the
bottom of the editor.
# See Also



# Menus Editor


The Menus Editor is a property editor; see.
# Connections Editor
connections editoreditor, connectionssource, connections editortarget, connections editorwhen action, connections editoraction type, connections editoractivated, when actioncreated, when actiondestroyed, when actiondragged from, when actiondropped on, when actionhidden, when actionshown, when actionresized, when actionrepaint needed, when actioncancel activated, when actionok activated, when actiondouble-clicked on, when actionpopped down, when actionpopped up, when actionaction1 activatedaction2 activatedaction3 activateddragged, when actionvalue changed, when actiontext changed, when actionbefore text changed, when actionpredefined, action typecall function, action typeexecute code, action typeactivate on-item helpaccess help volume, action typeaction, predefineddisable, predefined actionenable, predefined actionhide, predefined actionshow, predefined actionset label, predefined actionset value, predefined actionset text, predefined actionview, connections editorconnect buttonchange button, connections editordelete button, connections editorfunction, connections editorexecute code editorcode, connections editorvolume, connections editorlocation, connections editor

Used to make programmatic connections between interface objects. The
Connections Editor is described below the figure.
&newline; &empty;

* **Source** 

Specifies a source type and a source object for a connection.
Source types are primarily App Builder objects (Button, Choice,
Control Pane, for example) and object items (Choice Item, Menu Item,
Menubar Item). In addition, there is a Source type called Application,
which is relevant for Session Management and ToolTalk functions. See
the description of When actions below.
* **Target** 

Specifies a target type and a target object for a connection.
* **When** 

Specifies which When action on the Source object causes the
action specified in Action Type to be performed on the Target object.
Available When actions, which vary depending on the Source object
type, include Action1/2/3 Activated, Activated, Before Text Changed,
Cancel Activated, Created, Destroyed, Double-Clicked On, Dragged,
Dragged From, Dropped On, Hidden, Item Selected, OK Activated, Popped
Down, Popped Up, Repaint Needed, Resized, Shown, Text Changed, and
Value Changed.

If Application is chosen as the Source, available When actions are
Session Save, Session Restore, ToolTalk Do Command, ToolTalk Get
Status, ToolTalk Pause/Resume, and ToolTalk Quit. Seefor more about Session Management and ToolTalk.
* **Action Type** 

Specifies one of five choices: Predefined, Call
Function, Execute Code, Activate On-Item Help, or Access Help Volume.
Note that the specified connections are not made until you click on
the Connect button at the bottom of the Connections Editor.

If Predefined is chosen, a second option menu is activated, for choosing
what predefined action is to be performed on the Target object;
choices are Disable, Enable, Hide, Show, Set Label, Set Text, and Set
Value. For the "Set" actions the Argument text field is active, for
typing text or values to be set when the When function is performed.

If Call Function is chosen, a text field labelled "Function" is
activated, for typing the name of a function to be called when the
When action is executed in the compiled application; this function
name will be included in the .bil file as a connection, with an
action-type of "call-function." Note that a Call Function action will
not work in test mode.

If Execute Code is chosen, the Execute Code Editor will be displayed,
for typing in C code to be performed when the When action is executed
in the compiled application; this code will be included in the .bil
file as a connection, with an action-type of "execute-code." Note that
an Execute Code action will not work in test mode.

If Activate On-Item Help is chosen, a connection will be included in
the .bil file, with an action-type of "on-item-help." Note that the
Connect button will be inactive unless a menu item is selected in the
Source list.

If Access Help Volume is chosen, text fields labelled "Volume" and
"Location" are activated, for typing in the volume name and location
ID for displaying a help volume.
* **View** 

An option menu and a scrolling list for choosing what type of
source connections to view and for selecting a connection to edit or
delete. View types are primarily App Builder objects (Button, Choice,
Control Pane, etc.) and object items (Choice Item, Menu Item, Menubar
Item). In addition, there is a View type called Application, which is
relevant for Session Management and ToolTalk functions, and a View
type called Source Object, for viewing connections for the object
selected in the Source scrolling list.
* **Connect** 

A push button for creating a connection between the Source
and Target objects, according to the When and Action Type choices. The
connection is displayed in the View scrolling list.
* **Change** 

A push button for applying changes made to the connection
selected in the View scrolling list.
* **Delete** 

A push button for deleting the connection selected in the View
scrolling list.
* **Cancel** 

A push button to cancel any changes made in the Connections
Editor since the last time Connect was clicked; this alos closes the editor.
* **Help** 

A push button to display on-item help for the Connections
Editor.

# Message Editor
message editordialog title, message editormessage typeerror messageinformation messageworking messagequestion messagewarning messageaction buttons, message editor

Used to create various types of messages to be displayed at
appropriate times in the compiled application; the Message Editor
is described below. Seefor a discussion of
how to connect messages to the functions that cause them to be
displayed, with examples.

* **Messages** 

A scrolling list displaying all messages for the current
project; the module name precedes the message name in the list.
* **[Module option menu]** 

An option menu below the Messages list for
selecting the module for which you wish to add a new message. The
module name precedes the message name in the list.
* **Add Message** 

A push button for adding a new message to the Messages
list -- to the module selected in the module option menu.
* **Delete Message** 

A push button for deleting the currently-selected
message.
* **Name** 

A text field for specifying the name of the current message;
messages are given names such as "message," "message2," "message3," by
default.
* **Dialog Title** 

A text field for specifying the title that will appear
at the top of the message dialog box.
* **Type** 

An option menu for specifying the type of message to be
created; the choices, which appear above the message text pane, are
Error, Information, Working, Question, and Warning. The appropriate
message icon also appears in the message dialog box.
* **[Message text pane** 

A text pane for entering the text of the message.
Note that you need to press Return when you want the text to start
a new line. The label above the text pane varies, depending on what
type of message you have chosen.
* **[Button check boxes]** 

Check boxes for specifying which buttons will
be included at the bottom of the message dialog box. Each message type
has a different set of buttons specified by default; these default
choices can be changed. The Action1, Action2, Action3, and Cancel
buttons are used in the Connections Editor; seefor detailed instructions.
* **Connections [button]** 

A push button for displaying the Connections
Editor to specify what functions to call for each of the Action
buttons and the Cancel button.
* **Help Text** 

A push button for displaying the Help Editor and writing
help text to be displayed when the Help button is clicked in the
message dialog box.
* **Show Dialog** 

A push button for displaying the selected message in a
message dialog box that looks like the actual dialog box in the
compiled application. Click on one of the buttons other than Help to
dismiss the dialog box.


Seefor descriptions of the buttons at the
bottom of the editor.
# See Also







# Groups Editor


The Groups Editor is a property editor; see.
# Attachments Editor


Used to attach objects to each other for layout purposes, the
Attachments Editor is described below. Seefor
instructions. Seefor descriptions of the
buttons at the bottom of the editor.

* **Object Type** 

An option menu for choosing the type of object for which
you wish to make attachments. Some object types (custom dialog, file
selection dialog, main window) do not have parents and are not
included in the menu.
* **Objects** 

A scrolling list for selecting the object for which you wish
to make attachments.
* **Parent** 

A text field that indicates the parent of the selected
object.
* **Children** 

A scrolling list that lists the children of the Parent
object.
* **Parent attachments / Attachments in child** 

Radio buttons for
displaying the attachments of the parent of the selected object or the
attachments of the children of the selected object.
* **Attach To** 

Option menus for choosing the type of attachment for the
selected object and what to attach the object to, and text fields for
specifying the Offset (in pixels) from the selected object and its
parent or sibling and for specifying the Percentage offset of the
selected object from its parent. The option menu below "Attach To:" is
for choosing which sibling to attach to and is active only for sibling
attachments (two small squares). The Offset text field is active for
absolute (pixel) attachments only; the Percentage text field is active
for percentage attachments only.

The selected object is shown in the center of its four possible
attachments. The attachments, starting at the top and going clockwise,
are top edge of selected object, right edge of selected object, bottom
edge of selected object, and left edge of selected object. Each of the
possible types of attachments is described below; by default an object
is attached at its top and left edges to the top and left edges of its
parent. The selected object (the object at the center of the four
Attach To boxes) is thecontrollingobject: if you move
this controlling object, the pixel or percentage offset is changed;
click Reset to see current values after moving an attached object.


An ascending line from the top
edge of a small square to the top edge of its surrounding box
represents an absolute (pixel offset) attachment of the top edge of
the selected object to the top edge of its parent. If the parent
object is resized, the selected object will retain its pixel offset
from the top edge of its parent. The offset will change if the
selected object is moved.

A descending line from the
top edge of a small square to the bottom edge of its surrounding box
represents an absolute (pixel offset) attachment of the top edge of
the selected object to the bottom edge of its parent. The offset will
change if the selected object is moved. This value will be negative,
since x values are positive as they ascend and negative as they
descend.

Two vertically-aligned
squares connected by a vertical line represents an absolute (pixel
offset) attachment of the top edge of the selected object to the
bottom edge of its sibling (a sibling is another object with the same
parent). The offset will change if the selected object is moved. This
icon is inactive if the selected object has no siblings. This value
will be negative if the top edge of the selected object is above the
bottom edge of its sibling.

Two horizontally-aligned
squares connected by a horizontal line to the centers of their top
edges represents an absolute (pixel offset) attachment of the vertical
center of the selected object to the vertical center of its sibling (a
sibling is another object with the same parent). The offset will
change if the selected object is moved. This icon is inactive if the
selected object has no siblings. This value will be negative if the
center of the selected object is above the center of its sibling.

A square with a
two-headed arrow and a percentage sign above it represents a
percentage offset attachment of the top edge of the selected object to
the top edge of its parent. The offset will change if the selected
object is moved. If the parent object is resized, the selected object
will retain its percentage offset from the top edge of the parent.

A square with a
percentage sign above it and a two-headed arrow between the center
line of the square and the top of the surrounding box represents a
percentage offset attachment of the center of the selected object to
the top edge of its parent. The offset will change if the selected
object is moved. If the parent object is resized, the selected object
will retain its percentage offset from the top edge of the parent.

A circle with a diagonal
line through it represents no attachment from the edge (top, left,
bottom, or right) to another object. By default a dropped object has
no right or bottom edge attachments.

Note: descriptions of the attachments to the right, bottom, and left
edges of the selected object are correlatives of the descriptions of
the top-edge attachments above. Substitute "bottom" for "top" and
"top" for "bottom" for bottom-edge attachments. Substitute "right" for
"top" and "left" for "bottom" for right-edge attachments. Substitute
"left" for "top" and "right" for "bottom" for left-edge attachments.
# See Also









# Drag and Drop Editor


Used to establish drag and drop behavior, the Drag and Drop Editor is
described below. Seefor descriptions of the
buttons at the bottom of the editor.

* **Object Type** 

An option menu for choosing the type of object (Control
Pane, Custom Dialog, Draw Area Pane, Label, or Main Window) for which
you wish to establish drag and drop behavior.
* **Objects** 

A scrolling list for selecting a specific object for which
you wish to establish drag and drop behavior.
* **Drag Operations** 

Check boxes for specifying which types of operations
(Copy, Move, Link) will be legal for the selected object.
* **Cursor Filename** 

A text field for typing the name of the graphics
file that contains the graphical representation of the cursor that
will be displayed as a drag from the selected object is being
performed.
* **Cursor Mask Filename** 

A text field for typing the name of the
graphics file that contains the bitmap which determines the shape of
the visible representation of the cursor beneath the cursor mask. The
cursor mask acts like a stencil, allowing only the pixels in the
cursor that correspond to pixels in the mask to be visible.
* **Data Types** 

Check boxes for specifying Text, Filename, and User
Defined as legal data types for drag operations.
* **Drag Connection** 

A push button to display the Connections Editor for
creating the Call Function connection that makes the dragged-from
operation functional.
* **Drop Operations** 

Check boxes for specifying which types of operations
(Copy, Move, Link) will be legal for the selected object.
* **Data Types** 

Check boxes for specifying Text, Filename, User
Defined, and Any Other Type as legal data types for drop operations.
* **Drop on Children** 

A check box for specifying whether a child of the
selected object will be a legal drop site; this is relevant only if
the child object is specified as a legal drop site.
* **Drop Connection** 

A push button to display the Connections Editor for
creating the Call Function connection that makes the dropped-on
operation functional.

# See Also





# Application Framework Editor


Use to specify basic functionality in the application being built
with App Builder. Each section of the Application Framework Editor is
described below.
&newline; &empty;

* **Application Vendor Name** 

A text field for typing an optional string,
which will be stored in the source code. Used in the call to
initialize ToolTalk (if ToolTalk is enabled).
* **Application Version** 

A text field for typing an optional string,
which will be stored in the source code. Used in the call to
initialize ToolTalk.
* **Application Primary Main Window** 

An option menu fo specifying the
primary main window of the application being developed. An application
may have more than one main window, but only one primary window. This
window is typically the window which is first displayed when the
application is opened. By default the first main window dropped on
the desktop in a new project is the primary window.
* **Internationalization Enabled** 

A check box for specifying whether
internationalization is enabled; if checked, turns on XPG4-compliant
internationalization in the generated code for the project. In the
[module]_ui.c file, all labels and strings for objects are generated,
enclosed by the catgets(3C) call, which is used to fetch the
appropriate localized version of the string at runtime. If
internationalization is turned on, dtcodegen will also automatically
generate and maintain the message catalog ([project].msg) which maps
to the generated catgets(3C) calls.
* **Generated Code** 

Check boxes for specifying which categories of object
attributes (which map to Xt Resources) should be written into a
Resource file instead of placing them directly in the [module]_ui.c
file -- which is the default. Any attribute (resource) which is
specified in a Resource file -- and not directly in the code -- can be
modified without recompiling the application. The Attribute categories
are as follows:

Colors: Background, Foreground

Label Strings: Label String, Title

Initial Values: Initial Value

Geometry: X, Y, Width, Height, all attachment
attributes

Other Strings

Other
* **Session Management Method** 

An option menu for specifying the method
of session management (None, Command Line, Session File, or Both), and
two push buttons (Session Save Connection, Session Restore Connection)
for displaying the Connections Editor and making appropriate
connections.
* **ToolTalk Desktop Message Handling** 

An option menu for specifying what
level of the ToolTalk Desktop Message Alliance protocol the
application will participate in, and a push button (Advanced ToolTalk
Connections) for displaying the Connections Editor. The ToolTalk
desktop protocol is a set of predefined ToolTalk messages which
communicate desktop-type events or requests to a running application.
App Builder support for ToolTalk is provided at three levels: None,
Basic, or Advanced, as described below.

None. There is no participation in the ToolTalk Desktop
Protocol; no ToolTalk code is generated.

Basic. The ToolTalk library responds to Desktop messages in
categories 1-3 in a predefined and standard way. Code is generated in
main() which initializes ToolTalk and calls the function which tells
ToolTalk to handles these messages. At this level, you do not need to
write any special application code.

Advanced. The ToolTalk library responds to messages in
categories 1 and 2, but the application is notified (via callback)
when messages in categories 3 & 4 are received.

If you choose Advanced, you must use the Connections Editor to
identify which messages the application wishes to handle. If you
click the Advanced ToolTalk Connections button, the Connections Editor
will be displayed with Application as the Source object type. The When
option menu lists four ToolTalk choices: ToolTalk Do Command, ToolTalk
Get Status, ToolTalk Pause/Resume, and ToolTalk Quit. The only valid
action a ToolTalk connection is Call Function; your callback function
will be called when the ToolTalk message is received.

At this level code is generated in [project].c:main() which
initializes ToolTalk and sets up the Desktop Protocol so that the
callbacks defined in the Connections Editor will be called when the
corresponding message is received. Each user-defined callback
contains descriptive comments describing what the application is
expected to do in response to the message. These callbacks are also
generated in [project].c.


Seefor descriptions of the buttons at the
bottom of the editor.
# See Also




