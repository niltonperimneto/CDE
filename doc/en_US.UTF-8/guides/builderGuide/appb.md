
# Revolving Property Editor


Used to edit theproperties(the look or functionality) of interface objects, the
Revolving Property Editor customizes your application interface. This
appendix describes the properties common to all property editors, and the
properties and the buttons common to a number of property editors. It also
describes the individual property editors for each object.









The property editor for a separator, which includes the properties common to
almost all property editors, is shown below, with common elements noted.
# Property Editor: Universal Properties


The property editor for each of the App Builder objects is unique, but there are
a number of properties common to almost all of them.Revolving Property Editoruniversal properties in <$startrange>

Object Type

Not a property. Object Type is an option menu for
choosing the type of property editor to be displayed.
Properties change depending on which object type is
chosen.

Objects

Not a property. Objects lists the objects of the selected
object type in the current project. The list displays the
full, unique name for each object, which is comprised
of the name of the module in which the object exists,
two colons, and the Object Name.

Object Name

Displays the default name or the name given by you to
the object selected in the Objects list.

Initial State propertyVisibleInitial State, Visible

Specifies whether the selected object is visible when the
application starts up; all objects except a custom dialog
are visible by default.

Initial State propertyactiveInitial State, Active

A check box for specifying whether the object selected
is &ldquo;active&rdquo; when the application starts up. An inactive
object is not functional: it is dimmed and no functions
can be activated from the object.

colorbackgroundcolorforegroundforeground colorbackground colorColor: Background

Specifies the background color of the selected object.
You can either type in a known color name or choose
Color Chooser from the menu and select a color from
the palette displayed.

Color: Foreground

Specifies the foreground color of the selected object.
You can either type in a known color name or choose
Color Chooser from the menu and select a color from
the palette displayed.Revolving Property Editoruniversal properties in <$endrange>
# Property Editor: Common Properties


The following properties are common to three or more property editors.Revolving Property Editorcommon properties in <$startrange>

Border Frame propertybordertypes ofBorder Frame

Determines the type of border, if any, around certain
objects. Choices are None, Shadow Out, Shadow In,
Etched Out, and Etched In.

Geometry

Geometry propertyobjectsXY location ofobjectswidth and height values ofIndicates the X and Y location of the selected object,
and the W(idth) and H(eight) of the object. X and Y
values specify the position of the selected object in
relation to its parent. The values are in pixels and are
measured from the top-left corner of the parent object
to the top-left corner of the child object. W and H
values are in pixels.

Graphic Filename

Graphic Filename propertyIndicates the name of the pixmap (.pm) or bitmap
(.bm) file that contains the graphic to be used as the
label for the selected object or item. This property is
available only if Label Type or Item Label Type is
&ldquo;Graphic.&rdquo;

Item Label Type

Item Label Type propertySpecifies the type of label (String or Graphic) for the
selected item in the Items list. If Graphic is chosen,
&ldquo;Label&rdquo; becomes &ldquo;Graphic Filename.&rdquo;

Item State, Active

Item State propertySpecifies whether the selected item will be active
when the compiled application is opened.

Items

Lists the labels that represent the items in the list.
When an item is selected in the Items list, its label is
displayed in the Label or Graphic Filename field.

Label (Object/Item)

objectslabel setting forLabel propertySpecifies the label for the selected object or item.
&ldquo;Label&rdquo; becomes &ldquo;Graphic Filename&rdquo; if Graphic Label
Type is chosen. Label is inactive in the Button
property editor if Arrow Label Type is chosen.

Label Type

Label Type propertyobjectslabel type forSpecifies the type of label (String, Graphic, or Arrow)
for the selected object. If Graphic is chosen, &ldquo;Label&rdquo;
becomes &ldquo;Graphic Filename.&rdquo; If you choose Arrow,
the label in the Button property editor becomes an
arrow and the Arrow Direction property becomes
active.

Menu Title

Menu Title propertymenussetting title for pop-upSpecifies the (optional) title of the pop-up menu, if
any.

Popup, Pulldown Menu

menusproperty for attachingsubmenusproperty for attachingA menu button and a text field for creating, attaching,
de-attaching, or editing a pop-up or pull-down menu
for the selected object. When the Menus button is
clicked, a menu with four choices (None, Create New
Menu, Menus, Edit Current) is displayed. Menus and
Edit Current are inactive if no menus exist in the
current project. If a menu is already attached to the
selected object, the menu name will be displayed in
the text field.

Position (Label)

Position propertylabelsposition setting forSpecifies the position (Left or Above) of the label in
relation to the selected object. This Position option
menu is next to the Label Type option menu.

Position [XY]

objectsposition in relation to parentPosition propertyIndicates the X and Y location of the selected object in
relation to its parent. The values are in pixels and are
measured from the top-left corner of the parent object
to the top-left corner of the child object.

Scrollbars

Scrollbars settingpanesattaching a scroll bar toSpecifies when scroll bars should be attached to the
selected pane. The choices are Never and Always for
a term pane or a text pane, and Never, When Needed,
and Always for a draw area pane.

Size

Size propertywindowssetting absolute width and height forpanessetting absolute width and height forSpecifies the absolute W(idth) and H(eight) of the
window or pane. These values change if you resize
the window or pane manually in the interface. For a
term pane or a text pane, there is an option menu for
choosing Characters or Pixels as the unit value.

Size Policy

Size Policy propertywindowssetting size ofobjectssetting fixed sizeobjectssetting size to fit contentssizeto fit contentsSpecifies whether the selected object should retain a
fixed size or if it should become bigger or smaller
depending on the contents of the object. The choices
are Size of Label and Fixed for buttons and labels, Fit
Contents and Fixed for main windows and custom
dialogs.Revolving Property Editorcommon properties in <$endrange>

`List item editing`: once you have the appropriate number of items in
the list, the easiest way to perform item editing in those property editors that
have an item list* is to select the first item in the list, thus selecting it in the
label text field. Type a new name for the item and click Return. The new name
will be displayed in the item list and the next item in the list will be selected.
Continue down the list with this select, type, Return sequence until all items
are completed.
*Property editors with item lists include the choice objects (Radio Box, Check
Box, Option Menu), Combo Box, List, Menu, Menubar, and Spin Box.
# Property Editor: Common Buttons


The following functional push buttons or menu buttons are common to many
property editors. The buttons at the bottom of the property editors (OK, Apply,
Reset, Cancel, and Help) are common to other editors and dialog boxes.Revolving Property Editorcommon buttons in <$startrange>

Tear-off

Tear-off buttoneditorretaining on workspaceDisplays a property editor of the selected type; use this
when you want to edit a specific object type while
viewing other types of objects in the Revolving
Property Editor.

Add Item

Add Item buttonaddingitems to Items listAdds an item after the selected item in the Items list.
Added items are given default names starting with
&ldquo;Item1&rdquo; and incrementing, as needed. By default, items
are added after the selected item.

Edit

Edit buttonPerforms edit functions (Add After, Add Before,
Change, Cut, Copy, Paste, Delete) in a list. Add After
and Add Before add an item to the list either after or
before the selected item. Change applies the change
that you have made. Cut, Copy, Paste and Delete act on
the selected item, in the normal way: Cut and Copy
place the selected item in a buffer, ready for Paste.
Delete removes the item, but does not place it in a
buffer.

Attachments

Attachments buttonDisplays the Attachments Editor; there is no
Attachments button on the Main Window, Menubar,
Custom Dialog, or Paned Window property editors.

Help Text

Help Text buttonDisplays the Help Editor.

Connections

Displays the ConnectionsConnections buttonEditor.

OK

Applies the changes made to the selected object and
dismisses the editor; changes to properties are marked
with change bars at the left side of the editor.OK button

Apply

Apply buttonApplies the changes made to the selected object, but
does not dismiss the editor.

Reset

Reset buttonResets all changes made since the last Apply.

Cancel

Cancel buttonResets all changes made since the last Apply and
dismisses the editor.

Help

Displays help for the editor. Seefor information about
App Builder help.Revolving Property Editorcommon buttons in <$endrange>
# Individual Property Editors


An individual property editor is displayed by:

Double-clicking an object in the interface or the Module Browser.

Or, selecting an object of the desired type and choosing Properties from the
Editors menu on the App Builder primary window.

Or, choosing Props from the interface or Browser pop-up menu.

Or, choosing the desired object type from the Object Type options menu at
the top of the Revolving Property Editor.

The individual property editors are described in the following sections.
# Button Property EditorButton property editorproperty editorButton


Only properties unique to a button object are described here. Seefor descriptions of Object Type,
Objects, Object Name, Initial State, and Color. Seefor descriptions of Label Type, Label, Pulldown Menu,
Size Policy, and Geometry.

Button Type

button controlproperties of <$startrange>Specifies what kind of button (Push, Drawn, Menu) the
selected button should be. Push button is the default.
Selecting Menu transforms the push button into a
menu button, as if you had dragged and dropped a
menu button from the controls palette. The Pulldown
Menu property becomes active if you select Menu. See &ldquo;in, for descriptions of button
types.

Label Alignment

button controlproperties of <$endrange>labelsfor buttonsSpecifies the alignment (Left, Right, Centered) of the
button label within the button border frame. Label
Alignment is relevant only if Fixed is selected as Size
Policy. This menu is inactive if Arrow Label Type is
chosen.

Arrow Direction

Arrow Direction propertylabelsarrow direction forSpecifies which direction (Up, Down, Left, Right) the
arrow should point if Arrow Label Type is chosen.
# Choice Property EditorChoice property editorproperty editorChoice


Only properties unique to a choice object (Radio Box, Check Box, Option
Menu) are described here. See &ldquo;for descriptions of Object Type, Objects, Object Name, Initial State,
and Color. See &ldquo;for
descriptions of Label Type, Label Position, Label (Object), Items, Item Label
Type, Label (Item), Item State (Active), and Position [XY].editor<Emphasis>See also<Default Para Font> property editor<$nopage>[editor aa]

Choice Type

objectschoicechoice objectsSpecifies which type of choice object (Radio Box, Check
Box, or Option Menu) the selected object should be.
The object changes form depending on which you
choose. Note that there is a control object for each of
the choice types in the Controls palette. See &ldquo;in, for descriptions of choice types.

Rows/Columns

Specifies whether the radio box or check box should be
laid out in rows or columns, and how many rows or
columns there should be. Not relevant for an option
menu.

Item State, Selected

choice objectsSpecifies whether the selected item will be selected
when the compiled application is opened. Only one
item can be selected. For a check box or a radio box
object, the selected item will be marked as selected; for
an option menu, the label for the selected object will be
displayed in the option menu when the application is
opened.
# Combo Box Property EditorCombo Box property editorproperty editorCombo Box


Only properties unique to a combo box are described here. Seefor descriptions of Object Type,
Objects, Object Name, Initial State, and Color. Seefor descriptions of Label Type, Label Position, Label,
Items, Item Label, and Position [XY].

Combo Box Type

combo box controlSpecifies whether the text field for the selected combo
box will be Static or Editable in the compiled
application. If Editable is selected, code must be
written to implement the edit functionality.

Selected

Specifies which item will be selected when the
compiled application is opened.

Width

sizesetting for combo boxSpecifies whether the combo box shrinks or grows to
accommodate the Longest Item in the list, or if the
W(idth) of the box is Fixed. If Fixed is selected, the
W(idth) value can be edited.
# Control Pane Property Editorproperty editorControl PaneControl Pane property editor


There are no properties unique to a control pane. Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Border Frame, Size Policy,
Geometry, Popup Menu, and Menu Title.
# Custom Dialog Property Editorproperty editorCustom DialogCustom Dialog property editor


Only properties unique to a custom dialog object are described here. Seefor descriptions of Object
Type, Objects, Object Name, Initial State, and Color. Seefor descriptions of Size Policy and Size.

Dialog Title

namingcustom dialogcustom dialogproperties of <$startrange>The title that appears at the top of the custom dialog.

Window Parent

custom dialogparent main window forwindowsparent for custom dialogAn option menu for specifying a parent main window
for the selected custom dialog. Choices are None and
any main window in the project. If a main window is
specified as a window parent, the custom dialog will
be iconified and de-iconified with the main window.
Note that this functionality does not work in test mode,
but it does in the compiled application.

User Resize Mode

Specifies whether the window is Fixed or Adjustable
(whether it can be resized in the compiled application).

Dialog Areas

custom dialogproperties of <$endrange>Specifies whether a custom dialog includes a Button
Panel (three buttons, by default) and a Footer area.

Default Button

custom dialogdefault buttons indefault buttonsfor custom dialogSpecifies one of the dialog buttons as the selected
button, by default. The function represented by the
selected button will be performed if you press Return
while the mouse cursor is in the custom dialog in the
compiled application.

Help Button

Specifies one of the dialog buttons as the help button.
See, for a description of the Help Editor and
instructions for creating help.
# Draw Area Pane Property EditorDraw Area Pane property editorproperty editorDraw Area Pane


Only the one property unique to a draw area pane object is described here. Seefor descriptions of Object
Type, Objects, Object Name, Initial State, and Color. Seefor descriptions of Scrollbars, Border Frame,
Geometry, Popup Menu, and Menu Title.

Total Canvas Size

draw area panesize of canvas insizeof draw area canvasSpecifies the W(idth) and H(eight) of the draw area
canvas. Note that only a portion of the canvas will be
visible if the draw area pane's size is smaller than the
canvas size (400 by 400 pixels, by default). You can use
the scroll bars to view other parts of the canvas.
# File Selection Dialog Property Editorproperty editorFile Selection DialogFile Selection Dialog property editor


Only properties unique to a file selection dialog object are described here. Seefor descriptions of Object
Type, Objects, Object Name, Initial State, and Color.

Window Parent

file selection dialogproperties of <$startrange>Specifies the main window parent of the file selection
dialog. When displayed, the file selection dialog will
appear over its main window. By default, the Primary
Main Window is the parent of all file selection dialogs.

Dialog Title

Specifies the title that appears in the title bar at the top
of the file selection dialog.

Initial Directory

Specifies the folder (directory) set as the starting value
in the Path field of the file selection dialog.

Search Pattern Type

Specifies whether files, directories (folders), or both
will be listed in the Files list of the file selection dialog.

Search Pattern

Specifies the value of the Filter field in the file selection
dialog. The Filter value limits the files that will be
listed in the Files field. The default value is * (asterisk),
which means all files in the current folder will be
listed. The Filter value for the Import Module file
selection dialog in App Builder is*.bil, which means
that only files that end in.bilwill be listed.

OK Button Label

labelsfor OK button in file selection dialogSpecifies the label that will appear on the button in the
left-most position at the bottom of the file selection
dialog, normally labelled &ldquo;OK.&rdquo; Clicking this button
completes the file selection process and dismisses the
file selection dialog. This button is labelled &ldquo;Import&rdquo; for
the Import Module file selection dialog in App Builder.

Popdown Behavior

Specifies whether the file selection dialog will be
automatically dismissed (the default) when the OK
button is clicked.file selection dialogproperties of <$endrange>
# Group Property Editorproperty editorGroupGroup Property Editor


Used to modify the layout and framing of groups, the Group Property Editor
can be displayed by choosing Groups from the Editors menu of the App
Builder primary window or by choosing Group from the Revolving Property
Editor Object Type option menu. A group, unlike most of the objects edited in
the Revolving Property Editor, is a created object and is not available from an
object palette. See, for instructions.

Only properties unique to a group object are described here. Seefor descriptions of Object Type,
Objects, Initial State, and Color. Seefor descriptions of Border Frame and Position.

groupsproperties for <$startrange>Note that choosing Groups from the Editors menu in the App Builder primary
window is the same as clicking the Tear-off button in the Revolving Property
Editor when the Object Type is Group.

Group Name

naminggroupsDisplays the default name or the name given by you to
the group selected in the Objects list.

Layout Type

Specifies As-Is, Vertical, Horizontal, or Row/Column
layout of the objects in the selected group.

Rows Columns

Specifies whether the primary layout will be by rows
(vertical layout) or columns (horizontal layout), and
how many rows or columns to display. Active only if
Layout Type is Row/Column.

Vert Alignment

Specifies left-edge, colon/label, center-line, or right-
edge alignment of the objects in the selected group.
Active only if Layout Type is Vertical or Row/Column.

Spacing

Specifies the number of pixels separating the objects in
the selected group. Vert Alignment Spacing is active
only if Layout Type is Vertical or Row/Column. Horiz
Alignment Spacing is active only if Layout Type is
Horizontal or Row/Column.

Horiz Alignment

Specifies top-edge, center-line, or bottom-edge
alignment of the objects in the selected group. Active
only if Layout Type is Horizontal or Row/Column.groupsproperties for <$endrange>
# Label Property Editorproperty editorLabelLabel property editor


Only the property unique to a label object is described here. Seefor descriptions of Object Type,
Objects, Object Name, Initial State, and Color. Seefor descriptions of Label Type, Label, Size Policy, and
Geometry.

Note that no border appears around a label in the compiled application. See
&ldquo;in
&ldquo;, if you want a border around a label.

Label Alignment

label controlalignment ofSpecifies the alignment (Left, Right, Centered) of the
label within its margins. Label Alignment is relevant
only if Fixed is selected as Size Policy.
# List Property Editor


Only properties unique to a list object are described here. Seefor descriptions of Object Type, Objects,
Object Name, Initial State, and Color. Seefor descriptions of Label Type, Position (Label), Label,
Items, Item Label, Position [XY], Popup Menu, and Menu Title.property editorListList property editor

Selection Mode

Specifies how objects can be selected in a scrolling list.
Choices are Single Select, Browse Select, Multiple
Select, and Browse Multiple Select.