# XmRowColumn
library call`XmRowColumn`The RowColumn widget classXmRowColumnwidget classRowColumn&npzwc;#include &lt;Xm/RowColumn.h>
## DESCRIPTION


The RowColumn widget is a general purpose RowColumn manager capable
of containing any widget type as a child.
In general, it requires no special
knowledge about how its children function and provides nothing
beyond support for several different layout styles. However, it can be
configured as a menu, in which case, it expects only certain children, and
it configures to a particular layout. The menus supported are MenuBar,
Pulldown or Popup menu panes, and OptionMenu.
RowColumn uses the`XmQTmenuSavvy`trait and holds the`XmQTmenuSystem`trait.

The type of layout performed is controlled by how the application has set
the various layout resources.
It can be configured to lay out its children in either rows or
columns. In addition, the application can specify how the children are
laid out, as follows:

The children are packed tightly together into either rows or columns

Each child is placed in an identically sized
box (producing a symmetrical look)

A specific layout (the currentxandypositions of the children
control their location)

In addition, the application has control over both the spacing that
occurs between each row and column and the margin spacing
present between the edges of the RowColumn widget and any children
that are placed against it.

The default`XmNinsertPosition`procedure for the RowColumn
returns the value of`XmNpositionIndex`if one has been specified
for the child. Otherwise, this procedure returns the number of
children in the RowColumn's`XmNnumChildren`list.
In a MenuBar, Pulldown menu pane, or Popup menu pane the default for the`XmNshadowThickness`resource is 2.
In an OptionMenu or a WorkArea, (such as a RadioBox or CheckBox) this
resource is not applicable and its use is undefined.
If an application wishes to place a 3-D shadow around an OptionMenu or
WorkArea, it can create the RowColumn as a child of a Frame widget.

In a MenuBar, Pulldown menu pane, or Popup menu pane the`XmNnavigationType`resource is not applicable and its use is
undefined.
In a WorkArea, the default for`XmNnavigationType`is`XmTAB_GROUP`.
In an OptionMenu the default for`XmNnavigationType`is`XmNONE`.

In a MenuBar, Pulldown menu pane, or Popup menu pane the`XmNtraversalOn`resource is not applicable and its use is
undefined.
In an OptionMenu or WorkArea, the default for`XmNtraversalOn`is
True.

If the parent of the RowColumn is a MenuShell, the`XmNmappedWhenManaged`resource is forced to False when the widget
is realized.

The user can specify resources in a resource file for the automatically
created widgets and gadgets of an OptionMenu. The following list
identifies the names of these widgets (or gadgets) and the associated
OptionMenu areas.

* **Option Menu Label Gadget** 

`OptionLabel`
* **Option Menu Cascade Button** 

`OptionButton`


For the Popup and Pulldown Menupanes, popup and pulldown menus have
particular behaviors when theBtn1button is pressed outside the menus. These behaviors
are summarized here.

When there is already a popped up menu, a user can either pressBtn1in the same area as the
popped up menu, or can pressBtn1in another area that should
have a menu popped up. WhenBtn1is pressed in the same
area as the already popped up menu, that menu is unposted.
IfBtn1is pressed in a different area,
the associated popup menu is posted for the new area. Note, however,
that if the`XmNpopupHandlerCallback`of either`XmManager`or`XmPrimitive`is available, then the callback may override these
default behaviors.

For pulldown menus, a user can pressBtn1on the
Cascade button to post the pulldown menu, then click on it again. Upon the
second click, the pulldown menu is unposted.

Popup menus are not allowed to have NULL parents.
### Tear-off Menus


Pulldown and Popup menu panes support tear-off menus, which enable the
user to retain a menu pane on the display to facilitate subsequent
menu selections. A menu pane that can be torn-off is identified by
a tear-off button that spans the width of the menu pane and displays
a dashed line. A torn-off menu pane contains a window manager system
menu icon and a title bar. The window title displays the label of the
cascade button that initiated the action when the label type is`XmSTRING`. If the label contains a pixmap the window title is
empty. A tear-off menu from a Popup menu pane also displays
an empty title.
Tear-off menus should not be shared.

The user can tear off a menu pane using the mouse or keyboard.
ClickingBtn1or pressingosfActivate(orosfSelect)
on the tear-off button, tears off the menu pane at the current
position. PressingBtn2on the tear-off button tears off the
menu pane and allows the user to drag the torn-off menu to a new
position designated by releasing the mouse button. Tearing off a
menu pane unposts the current active menu. Only one tear-off copy
for each menu pane is allowed. Subsequent tear-off actions of a
torn menu pane unpost the existing copy first.

The name of the tear-off button of a torn-off menu pane is`TearOffControl`. The name can be used to set resources in a resource
file. An application can also obtain the tear-off button itself using`XmGetTearOffControl`and then set resource values by calling`XtSetValues`.

The tear-off button has Separator-like behavior. Its appearance can be
specified with the following tear-off button resources:`XmNbackground`,`XmNbackgroundPixmap`,`XmNbottomShadowColor`,`XmNforeground`,`XmNheight`,`XmNmargin`,`XmNseparatorType`,`XmNshadowThickness`, and`XmNtopShadowColor`. Refer to the`XmSeparator`reference
page for a complete description of each of these resources.

The`XmNtearOffModel`,`XmNtearOffMenuActivateCallback`, and`XmNtearOffMenuDeactivateCallback`are RowColumn resources that affect tear-off menu behavior.
The`XmNtearOffTitle`resource enables the application to specify
the tear-off menu's title if the menu is torn off.

By default, menus do not tear off. Setting the`XmNtearOffModel`resource to`XmTEAR_OFF_ENABLED`enables tear-off functionality.
There is no resource converter
preregistered for`XmNtearOffModel`. To allow tear-off
functionality to be enabled through the resource database, call the
function`XmRepTypeInstallTearOffModelConverter`.

Tear-off menu focus policy follows standard window
manager policy. It is recommended that the`startupKeyFocus`and`autoKeyFocus`mwmresources be set to True.
### Descendants


RowColumn automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`OptionButton``XmCascadeButtonGadget`option menu button`OptionLabel``XmLabelGadget`option menu label`TearOffControl`subclass of`XmPrimitive`tear-off button of torn-off menu pane
### Classes


RowColumn inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`, and`XmManager`classes.

The class pointer is`xmRowColumnWidgetClass`.

The class name is`XmRowColumn`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmRowColumn Resource Set``Name``Class``Type``Default``Access`XmNadjustLastXmCAdjustLastBooleanTrueCSGXmNadjustMarginXmCAdjustMarginBooleanTrueCSGXmNentryAlignmentXmCAlignmentunsigned charXmALIGNMENT_BEGINNINGCSGXmNentryBorderXmCEntryBorderDimension0CSGXmNentryCallbackXmCCallbackXtCallbackListNULLCXmNentryClassXmCEntryClassWidgetClassdynamicCSGXmNentryVerticalAlignmentXmCVerticalAlignmentunsigned charXmALIGNMENT_CENTERCSGXmNisAlignedXmCIsAlignedBooleanTrueCSGXmNisHomogeneousXmCIsHomogeneousBooleandynamicCGXmNlabelStringXmCXmStringXmStringNULLCXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimensiondynamicCSGXmNmenuAcceleratorXmCAcceleratorsStringdynamicCSGXmNmenuHelpWidgetXmCMenuWidgetWidgetNULLCSGXmNmenuHistoryXmCMenuWidgetWidgetNULLCSGXmNmenuPostXmCMenuPostStringNULLCSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringXmFONTLIST_DEFAULT_TAGCSGXmNnumColumnsXmCNumColumnsshort1CSGXmNorientationXmCOrientationunsigned chardynamicCSGXmNpackingXmCPackingunsigned chardynamicCSGXmNpopupEnabledXmCPopupEnabledintXmPOPUP_KEYBOARDCSGXmNradioAlwaysOneXmCRadioAlwaysOneBooleanTrueCSGXmNradioBehaviorXmCRadioBehaviorBooleanFalseCSGXmNresizeHeightXmCResizeHeightBooleanTrueCSGXmNresizeWidthXmCResizeWidthBooleanTrueCSGXmNrowColumnTypeXmCRowColumnTypeunsigned charXmWORK_AREACGXmNspacingXmCSpacingDimensiondynamicCSGXmNsubMenuIdXmCMenuWidgetWidgetNULLCSGXmNtearOffMenuActivateCallbackXmCCallbackXtCallbackListNULLCXmNtearOffMenuDeactivateCallbackXmCCallbackXtCallbackListNULLCXmNtearOffModelXmCTearOffModelunsigned charXmTEAR_OFF_DISABLEDCSGXmNtearOffTitleXmCTearOffTitleXmStringNULLCSGXmNunmapCallbackXmCCallbackXtCallbackListNULLCXmNwhichButtonXmCWhichButtonunsigned intdynamicCSG

* **`XmNadjustLast`** 

Extends the last row of children to the bottom edge of RowColumn (when`XmNorientation`is`XmHORIZONTAL`) or extends the last column to the
right edge of RowColumn (when`XmNorientation`is`XmVERTICAL`).
Setting`XmNadjustLast`to False disables this feature.
* **`XmNadjustMargin`** 

Specifies whether the inner minor margins of all
items contained within the RowColumn widget are
forced to the same value. The inner minor margin
corresponds to the`XmNmarginLeft`,`XmNmarginRight`,`XmNmarginTop`,
and`XmNmarginBottom`resources supported by`XmLabel`and`XmLabelGadget`.

A horizontal orientation causes`XmNmarginTop`and`XmNmarginBottom`for all items in a particular row to be forced to the
same value; the value is the largest margin specified
for one of the Label items.

A vertical orientation causes`XmNmarginLeft`and`XmNmarginRight`for all items in a particular
column to be forced to the same value; the value is the largest
margin specified for one of the Label items.

This keeps all text within each row or column
lined up with all other text in its row or column.
If`XmNrowColumnType`is either`XmMENU_POPUP`or`XmMENU_PULLDOWN`and this resource is True, only button children
have their margins adjusted.
* **`XmNentryAlignment`** 

Specifies the alignment type for children that are subclasses of`XmLabel`or`XmLabelGadget`when`XmNisAligned`is enabled.
The following are textual alignment types:

`XmALIGNMENT_BEGINNING`(default)

`XmALIGNMENT_CENTER`

`XmALIGNMENT_END`

See the description of`XmNalignment`in the &cdeman.XmLabel;
reference page for an explanation of these actions.
* **`XmNentryBorder`** 

Imposes a uniform border width upon all RowColumn's children.
The default value is 0 (zero), which disables the feature.
* **`XmNentryCallback`** 

Disables the`XmNactivateCallback`and`XmNvalueChangedCallback`callbacks for all CascadeButton, DrawnButton, PushButton, and
ToggleButton widgets and gadgets contained within the RowColumn widget.
If the application supplies this resource, the`XmNactivateCallback`and`XmNvalueChangedCallback`callbacks are then revectored to the`XmNentryCallback`callbacks.
This allows an application to supply a single callback routine for
handling all items contained in a RowColumn widget.
The callback reason is`XmCR_ACTIVATE`.
If the application does not supply this resource, the`XmNactivateCallback`and`XmNvalueChangedCallback`callbacks for each item in the RowColumn widget work as normal.

The application must supply this resource when this widget is created.
Changing this resource using the`XtSetValues`is not
supported.
* **`XmNentryClass`** 

Specifies the only widget class that can be added
to the RowColumn widget; this resource is meaningful only when the`XmNisHomogeneous`resource is set to True.
Both widget and gadget variants of the specified class may be added to
the widget.

When`XmCreateRadioBox`is called or when`XmNrowColumnType`is
set to`XmWORK_AREA`and`XmNradioBehavior`is True, the default
value of`XmNentryClass`is`xmToggleButtonGadgetClass`.
When`XmNrowColumnType`is set to`XmMENU_BAR`, the value of`XmNentryClass`is forced to`xmCascadeButtonWidgetClass`.
* **`XmNentryVerticalAlignment`** 

Specifies the type of vertical alignment for children that are
subclasses of`XmLabel`,`XmLabelGadget, and``XmText`.
This resource is invalid if`XmNorientation`is`XmVERTICAL`and`XmNpacking`is`XmPACK_TIGHT`, because this layout
preserves variable heights among the children. The vertical alignment
types include:

* **`XmALIGNMENT_BASELINE_BOTTOM`** 

Causes the bottom baseline of all
children in a row to be aligned.
This resource is applicable only when all children in a row
contain textual data.
* **`XmALIGNMENT_BASELINE_TOP`** 

Causes the top baseline of all
children in a row to be aligned.
This resource is applicable only when all children in a
row contain textual data.
* **`XmALIGNMENT_CONTENTS_BOTTOM`** 

Causes the bottom of the
contents (text or
pixmap) of all children in a row to be aligned.
* **`XmALIGNMENT_CENTER`** 

Causes the center of all children in a row to be
aligned.
* **`XmALIGNMENT_CONTENTS_TOP`** 

Causes the top of the contents (text or
pixmap) of all children in a row to be aligned.

* **`XmNisAligned`** 

Specifies text alignment for each item within the RowColumn widget;
this applies only to items that are subclasses of`XmLabel`or`XmLabelGadget`.
However, if the item is a Label widget or gadget and its parent is either
a Popup menu pane or a Pulldown menu pane, alignment is not
performed; the Label is treated as the
title within the
menu pane, and the alignment
set by the application is not overridden.`XmNentryAlignment`controls the type of textual alignment.
* **`XmNisHomogeneous`** 

Indicates whether the RowColumn
widget should enforce exact homogeneity among the items
it contains; if this resource is set to True, only the widgets that are
of the class indicated by`XmNentryClass`are allowed as children of the RowColumn widget.
This is most often used when creating a MenuBar.
Attempting to insert a child that is not a member of the
specified class generates a warning message.

In a MenuBar, the value of`XmNisHomogeneous`is forced to True.
In an OptionMenu, the value is forced to False.
When`XmCreateRadioBox`is called the default value is True.
Otherwise, the default value is False.
* **`XmNlabelString`** 

When`XmNrowColumnType`is set to`XmMENU_OPTION`,
this resource points to a text string that displays the label with
respect to the selection area. The positioning of the label relative to the
selection area depends on the layout
direction in the horizontal orientation.
This resource is not meaningful for all other RowColumn types.
If the application wishes to change the label after creation, it must get the
LabelGadget ID (`XmOptionLabelGadget`) and call`XtSetValues`on the
LabelGadget directly. The default value is no label.
* **`XmNmapCallback`** 

Specifies a widget-specific callback function that is
invoked when the window associated with the RowColumn widget
is about to be mapped. The callback reason is`XmCR_MAP`.
* **`XmNmarginHeight`** 

Specifies the amount of blank space between the top
edge of the RowColumn widget and the first item in each column,
and the bottom edge of the RowColumn widget and the last item
in each column.
The default value is 0 (zero) for Pulldown and Popup menu panes, and 3
pixels for other RowColumn types.
* **`XmNmarginWidth`** 

Specifies the amount of blank space between the left
edge of the RowColumn widget and the first item in each row,
and the right edge of the RowColumn widget and the last item in
each row.
The default value is 0 (zero) for Pulldown and Popup menu panes, and 3
pixels for other RowColumn types.
* **`XmNmenuAccelerator`** 

This resource is useful only when the RowColumn widget has been configured
to operate as a Popup menu pane or a MenuBar.
The format of this resource is similar to the left side specification
of a translation string, with the limitation that it must specify a key
event.
For a Popup menu pane, when the accelerator is typed by the user, the Popup
menu pane is posted.
For a MenuBar, when the accelerator is typed by the user, the first item
in the MenuBar is highlighted, and traversal is enabled in the
MenuBar.
The default for a Popup menu pane isosfMenu.
The default for a MenuBar isosfMenuBar.
Setting the`XmNpopupEnabled`resource to False disables
the accelerator.
* **`XmNmenuHelpWidget`** 

Specifies the widget ID for the CascadeButton, which is treated as
the Help widget if`XmNrowColumnType`is set to`XmMENU_BAR`.
Which corner of the MenuBar the Help widget is placed at depends on
the`XmNlayoutDirection`resource of the widget.
If the RowColumn widget is any type other than`XmMENU_BAR`,
this resource is not meaningful.
* **`XmNmenuHistory`** 

Specifies the widget ID of the last menu entry to be activated. It is
also useful for specifying the current selection for an OptionMenu. If`XmNrowColumnType`is set to`XmMENU_OPTION`, the specified
menu item is positioned under the cursor when the menu is displayed.

If the RowColumn widget has the`XmNradioBehavior`resource set to
True,
the widget field associated with this resource
contains the widget ID of the last ToggleButton or ToggleButtonGadget
to change from unselected to selected.
The default value is the widget ID of the first child in
the widget.
* **`XmNmenuPost`** 

Specifies an X event description indicating a button event that posts a menu
system.
The default for`XmMENU_POPUP`is`BMenu Press`.
The default for`XmMENU_OPTION`,`XmMENU_BAR`, and`XmWORK_AREA`isBtn1`Press`.
The`XmNmenuPost`resource for pulldowns should be consistent with that of
the top-level parent menu (although the event type is ignored).
Setting this resource to`BTransfer Press`will conflict with drag and drop
operations, which use`BTransfer Press`as a default button binding.
Therefore, this resource cannot be`BTransfer Press`.
* **`XmNmnemonic`** 

This resource is useful only when`XmNrowColumnType`is
set to`XmMENU_OPTION`.
It specifies a keysym for a key that, when pressed by the user along with
the`MAlt`modifier, posts
the associated Pulldown menu pane.
The first character in the OptionMenu label string that exactly matches
the mnemonic in the character set specified in`XmNmnemonicCharSet`is underlined.
The user can post the menu by pressing either the shifted or the
unshifted mnemonic key.
The default is no mnemonic.
* **`XmNmnemonicCharSet`** 

Specifies the character set of the mnemonic for an OptionMenu.
The default is`XmFONTLIST_DEFAULT_TAG`.
If the RowColumn widget is any type other than`XmMENU_OPTION`,
this resource is not meaningful.
* **`XmNnumColumns`** 

Specifies the number of minor dimension extensions
that are made to accommodate the entries; this
attribute is meaningful only when`XmNpacking`is set to`XmPACK_COLUMN`.

For vertically oriented RowColumn widgets, this attribute
indicates how many columns are built; the number of
entries per column is adjusted to maintain this
number of columns, if possible.

For horizontally oriented RowColumn widgets, this attribute
indicates how many rows are built.

The default value is 1.
In an OptionMenu the value is forced to 1.
The value must be greater than 0 (zero).
* **`XmNorientation`** 

Determines whether RowColumn layouts are row-major or column-major.
In a column-major layout, the children of the RowColumn
are laid out in
columns within the widget. In a row-major layout the children of
the RowColumn are laid out in rows. The direction of the horizontal
layout in the row-major layout (from left or right), and the wrapping in
the column-major layout (vertical), depend on the`XmNlayoutDirection`resource of the widget.
The`XmVERTICAL`resource value
selects a column-major layout.`XmHORIZONTAL`selects a row-major layout.

When creating a MenuBar or an OptionMenu, the default is`XmHORIZONTAL`.
Otherwise, the default value is`XmVERTICAL`.
The results of specifying a value of`XmVERTICAL`for a MenuBar are
undefined.
* **`XmNpacking`** 

Specifies how to pack the items contained within a
RowColumn widget. This can be set to`XmPACK_TIGHT, XmPACK_COLUMN`or`XmPACK_NONE`. When a RowColumn widget
packs the items it contains, it determines its major
dimension using the value of the`XmNorientation`resource.

`XmPACK_TIGHT`indicates that given the current major
dimension (for example, vertical if`XmNorientation`is`XmVERTICAL`), entries
are placed one after the other until
the RowColumn widget must wrap. RowColumn wraps when there is no room left
for a complete child in that dimension.
Wrapping occurs
by beginning a new row or column in the next available
space. Wrapping continues, as often as necessary, until
all of the children are laid out.
In the vertical dimension (columns), boxes are set to the same width; in the
horizontal dimension (rows), boxes are set to the same depth.
Each
entry's position in the major dimension is left unaltered (for example,`XmNy`is left unchanged when`XmNorientation`is`XmVERTICAL`); its
position in the minor
dimension is set to the same value as the greatest entry
in that particular row or column. The position in the minor
dimension of any particular row or column is independent
of all other rows or columns.

`XmPACK_COLUMN`indicates that all entries are placed in
identically sized boxes. The boxes are based on the largest height
and width values of all the children widgets.
The value of the`XmNnumColumns`resource determines how many boxes are placed in the
major dimension, before extending in the minor dimension.

`XmPACK_NONE`indicates that no packing is performed.
Thexandyattributes of each entry are left alone, and
the RowColumn widget attempts to become large enough to enclose all
entries.

When`XmCreateRadioBox`is called or when`XmNrowColumnType`is set to`XmWORK_AREA`and`XmNradioBehavior`is True, the
default value of`XmNpacking`is`XmPACK_COLUMN`.
In an OptionMenu the value is initialized to`XmPACK_TIGHT`.
Otherwise, the value defaults to`XmPACK_TIGHT`.
* **`XmNpopupEnabled`** 

Allows the menu system
to enable keyboard input (accelerators and mnemonics) defined for the Popup
menu pane and any of its submenus.
The Popup menu pane needs to be informed whenever its accessibility to the user
changes because posting of the Popup menu pane is controlled by the
application.
This resource can take four values, including:

* **`XmPOPUP_KEYBOARD`** 

Specifies that the keyboard
input&mdash;accelerators and mnemonics&mdash;defined for the Popup menu pane
and any of its submenus is enabled. This is the default.
* **`XmPOPUP_DISABLED`** 

Specifies that the keyboard input is disabled.
* **`XmPOPUP_AUTOMATIC`** 

Specifies that the keyboard is enabled for automatic popup menus.
* **`XmPOPUP_AUTOMATIC_RECURSIVE`** 

Specifies that the keyboard is enabled for recursive automatic popup menus.

* **`XmNradioAlwaysOne`** 

If True, forces the active ToggleButton or ToggleButtonGadget
to be automatically selected after having
been unselected (if no other toggle was activated).
If False, the active toggle may be unselected.
The default value is True. This resource is important only when`XmNradioBehavior`is True.

The application can always add and subtract toggles from
RowColumn regardless of the selected/unselected state of the toggle. The
application can also manage and unmanage toggle
children of RowColumn at any time regardless of state. Therefore,
the application can sometimes
create a RowColumn that has`XmNradioAlwaysOne`set to
True and none
of the toggle children selected.
The result is undefined if the value of this resource is True and the
application sets more than one ToggleButton at a time.
* **`XmNradioBehavior`** 

Specifies a Boolean value that when True, indicates
that the RowColumn widget should enforce a RadioBox-type behavior
on all of its children that are ToggleButtons or
ToggleButtonGadgets.

When the value of this resource is True,`XmNindicatorType`defaults to`XmONE_OF_MANY`for ToggleButton and ToggleButtonGadget children.

RadioBox
behavior dictates that when one toggle is selected and the user selects another
toggle, the first toggle is unselected
automatically.
The RowColumn usually does not enforce this behavior if the application,
rather than the user, changes the state of a toggle.
The RowColumn does enforce this behavior if a toggle child is selected
with`XmToggleButtonSetState`or`XmToggleButtonGadgetSetState`with a`notify`argument of True.

When`XmCreateRadioBox`is called, the default value of`XmNradioBehavior`is True.
Otherwise, the default value is False.
* **`XmNresizeHeight`** 

Requests a new height if necessary, when set to True. When this
resource is set to
False, the widget does not request a new height regardless of any
changes to the widget or its children.
* **`XmNresizeWidth`** 

Requests a new width if necessary, when set to True. When set to
False, the widget does not request a new width regardless of any
changes to the widget or its children.
* **`XmNrowColumnType`** 

Specifies the type of RowColumn widget
to be created.
It is a nonstandard resource that cannot be changed after it is set.
If an application uses any of the
convenience routines, except`XmCreateRowColumn`,
this resource is automatically forced to the appropriate
value by the convenience routine. If an application uses
the Xt Intrinsics API to create its RowColumn widgets,
it must specify this resource itself. The set
of possible settings for this resource are

`XmWORK_AREA`(default)

`XmMENU_BAR`

`XmMENU_PULLDOWN`

`XmMENU_POPUP`

`XmMENU_OPTION`

This resource cannot be changed after the RowColumn widget
is created. Any changes attempted through`XtSetValues`are ignored.

The value of this resource is used to determine the value of a number
of other resources. The descriptions of RowColumn resources explain
this when it is the case. The resource`XmNnavigationType`,
inherited from`XmManager`, is changed to`XmNONE`if`XmNrowColumnType`is`XmMENU_OPTION`.
* **`XmNspacing`** 

Specifies the horizontal and vertical spacing between
items contained within the RowColumn widget.
The default value is 3 pixels for`XmOPTION_MENU`and`XmWORK_AREA`and 0 (zero) for other RowColumn types.
* **`XmNsubMenuId`** 

Specifies the widget ID for the Pulldown menu pane to be associated with
an OptionMenu. This resource is useful only when`XmNrowColumnType`is
set to`XmMENU_OPTION`.
The default value is NULL.
* **`XmNtearOffMenuActivateCallback`** 

Specifies the callback list that notifies the application when
the tear-off menu pane is about to be activated. It precedes the tear-off's
map callback.

Use this resource when your application has shared menu panes and when
the torn-off menu can have two or
more parents that can have opposing sensitivity states for the same
menu item.
This resource enables
the application to track
whether a menu item is sensitive or insensitive and to set the state to the
original parent's menu item state when the torn-off menu is reposted.
The application can use`XmGetPostedFromWidget`to determine from which
parent the menu was torn. The callback reason is`XmCR_TEAR_OFF_ACTIVATE`.
The default is NULL.
* **`XmNtearOffMenuDeactivateCallback`** 

Specifies the callback list that notifies the application when
the tear-off menu pane is about to be deactivated. It follows the tear-off's
unmap callback.

Use this resource when your application has shared menu panes and when
the torn-off menu can have two or
more parents that can have opposing sensitivity states for the same
menu item.
This resource enables
the application to track
whether a menu item is sensitive or insensitive and to set the state to the
original parent's menu item state when the torn-off menu is reposted.
The application can use`XmGetPostedFromWidget`to determine from which
parent the menu was torn.
The callback reason is`XmCR_TEAR_OFF_DEACTIVATE`. The default is NULL.
* **`XmNtearOffModel`** 

Indicates whether tear-off functionality is enabled or disabled
when`XmNrowColumnType`is set to`XmMENU_PULLDOWN`or`XmMENU_POPUP`. The values are`XmTEAR_OFF_ENABLED`or`XmTEAR_OFF_DISABLED`(default value). This resource is
invalid for type`XmMENU_OPTION`; however, it does affect
any pulldown
submenus within an OptionMenu.
The function`XmRepTypeInstallTearOffModelConverter`installs
a resource converter for this resource.
* **`XmNtearoffTitle`** 

Used by the TearOff shell to set the title for the window manager to
display.
* **`XmNunmapCallback`** 

Specifies a list of callbacks that is called
after the window associated with the RowColumn
widget has been unmapped. The callback reason is`XmCR_UNMAP`.
The default value is NULL.
* **`XmNwhichButton`** 

This resource is obsolete; it has been replaced by`XmNmenuPost`and
is present for compatibility with older releases of Motif.
This resource cannot be 2.


`XmRowColumn Constraint Resource Set``Name``Class``Type``Default``Access`XmNpositionIndexXmCPositionIndexshortXmLAST_POSITIONCSG

* **`XmNpositionIndex`** 

Specifies the position of the widget in its parent's list of
children (the value of the`XmNchildren`resource). The value
is an integer that is no less than 0 (zero) and no greater than
the number of children in the list at the time the value is
specified. A value of 0 (zero) means that the child is placed at the
beginning of the list. The value can also be specified as`XmLAST_POSITION`(the default), which means that the child
is placed at the end of the list. Any other value is ignored.`XtGetValues`returns the position of the widget in its parent's
child list at the time of the call to`XtGetValues`.

When a widget is inserted into its parent's child list, the positions
of any existing children that are greater than or equal to the
specified widget's`XmNpositionIndex`are increased by 1.
The effect of a call to`XtSetValues`for`XmNpositionIndex`is to remove the specified widget from its parent's child list, decrease
by 1 the positions of any existing children that are greater than
the specified widget's former position in the list, and then insert
the specified widget into its parent's child list as described in the
preceding sentence.


`Simple Menu Creation Resource Set``Name``Class``Type``Default``Access`XmNbuttonAcceleratorsXmCButtonAcceleratorsStringTableNULLCXmNbuttonAcceleratorTextXmCButtonAcceleratorTextXmStringTableNULLCXmNbuttonCountXmCButtonCountint0CXmNbuttonMnemonicCharSetsXmCButtonMnemonicCharSetsXmStringCharSetTableNULLCXmNbuttonMnemonicsXmCButtonMnemonicsXmKeySymTableNULLCXmNbuttonsXmCButtonsXmStringTableNULLCXmNbuttonSetXmCButtonSetint&minus;1CXmNbuttonTypeXmCButtonTypeXmButtonTypeTableNULLCXmNoptionLabelXmCOptionLabelXmStringNULLCXmNoptionMnemonicXmCOptionMnemonicKeySymNULLCXmNpostFromButtonXmCPostFromButtonint&minus;1CXmNsimpleCallbackXmCCallbackXtCallbackProcNULLC

* **`XmNbuttonAccelerators`** 

This resource is for use with the simple menu creation routines.
It specifies a list of accelerators for the buttons created.
The list contains one element for each button, separator, and title
created.
* **`XmNbuttonAcceleratorText`** 

This resource is for use with the simple menu creation routines.
It specifies a list of compound strings to display for the accelerators for
the buttons created.
The list contains one element for each button, separator, and title
created.
* **`XmNbuttonCount`** 

This resource is for use with the simple menu creation routines.
It specifies the total number of menu buttons, separators, and titles to
create.
The value must not be negative.
* **`XmNbuttonMnemonicCharSets`** 

This resource is for use with the simple menu creation routines.
It specifies a list of character sets with which button mnemonics are to be
displayed.
The list contains one element for each button, separator, and title
created.
The default is determined dynamically depending on the locale of the
widget.
* **`XmNbuttonMnemonics`** 

This resource is for use with the simple menu creation routines.
It specifies a list of mnemonics for the buttons created.
The list contains one element for each button, separator, and title
created.
* **`XmNbuttons`** 

This resource is for use with the simple menu creation routines.
It specifies a list of compound strings to use as labels for the buttons
created.
The list contains one element for each button, separator, and title
created.
* **`XmNbuttonSet`** 

This resource is for use with the simple menu creation routines.
It specifies which button of a RadioBox or OptionMenu Pulldown submenu
is initially set.
The value is an integer`n`indicating the`n`th
ToggleButtonGadget specified for a RadioBox or the`n`th
PushButtonGadget specified for an OptionMenu Pulldown submenu.
The first button specified is number 0.
The value must not be negative.
* **`XmNbuttonType`** 

This resource is for use with the simple menu creation routines.
It specifies a list of button types associated with the buttons to be
created.
The list contains one element for each button, separator, and title
created.
If this resource is not specified, each button in a MenuBar is a
CascadeButtonGadget, each button in a RadioBox or CheckBox is a
ToggleButtonGadget, and
each button in any other type
of RowColumn widget is a PushButtonGadget.
Each button type is of typeXmButtonType, an enumeration with the
following possible values:

* **`XmCASCADEBUTTON`** 

Specifies a CascadeButtonGadget for a MenuBar,
Popup menu pane, or Pulldown menu pane.
* **`XmCHECKBUTTON`** 

Specifies a ToggleButtonGadget for a CheckBox,
Popup menu pane, or Pulldown menu pane.
* **`XmDOUBLE_SEPARATOR`** 

Specifies a SeparatorGadget for a Popup
menu pane, Pulldown menu pane, or OptionMenu Pulldown submenu.
The separator type is`XmDOUBLE_LINE`.
* **`XmPUSHBUTTON`** 

Specifies a PushButtonGadget for a Popup menu pane,
Pulldown menu pane, or OptionMenu Pulldown submenu.
* **`XmRADIOBUTTON`** 

Specifies a ToggleButtonGadget for a RadioBox,
Popup menu pane, or Pulldown menu pane.
* **`XmSEPARATOR`** 

Specifies a SeparatorGadget for a Popup menu pane,
Pulldown menu pane, or OptionMenu Pulldown submenu.
* **`XmTITLE`** 

Specifies a LabelGadget used as a title for a Popup
menu pane or Pulldown menu pane.

* **`XmNoptionLabel`** 

This resource is for use with the simple menu creation routines.
It specifies a compound string for the label string to be used on the left
side of an OptionMenu.
* **`XmNoptionMnemonic`** 

This resource is for use with the simple menu creation routines.
It specifies a keysym for a key that, when pressed by the user along
with the`MAlt`modifier, posts
the associated Pulldown menu pane for an OptionMenu.
* **`XmNpostFromButton`** 

This resource is for use with the simple menu creation routines.
For a Pulldown menu pane, it specifies the button in the parent to which
the submenu is attached.
The menu is then posted from this button.
The value is an integer`n`indicating the`n`th
CascadeButton or CascadeButtonGadget specified for the parent of the
Pulldown menu pane.
The first button specified is number 0.
The value must not be negative.
* **`XmNsimpleCallback`** 

This resource is for use with the simple menu creation routines.
It specifies a callback procedure to be called when a button is
activated or when its value changes.
This callback function is added to each button after creation.
For a CascadeButtonGadget or a PushButtonGadget, the callback is added
as the button's`XmNactivateCallback`, and it is called when the
button is activated.
For a ToggleButtonGadget, the callback is added as the button's`XmNvalueChangedCallback`, and it is called when the button's value
changes.
The button number is passed in the`client_data`field.

### Inherited Resources


RowColumn inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypedynamicCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleandynamicCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcdefault procedureCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        Widget`widget`;
        char`* data`;
        char`* callbackstruct`;
} XmRowColumnCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback


The following fields apply only when the callback reason is`XmCR_ACTIVATE`;
for all other callback reasons, these fields are set to NULL.
The`XmCR_ACTIVATE`callback reason is generated only when the application
has supplied an entry callback, which overrides any activation callbacks
registered with the individual RowColumn items.

* **`widget`** 

Is set to the widget ID of the RowColumn item that has been activated
* **`data`** 

Contains the client-data value supplied by the
application when the RowColumn item's activation callback was registered
* **`callbackstruct`** 

Points to the callback structure
generated by the RowColumn item's activation callback

### Translations


`XmRowColumn`translations depend on the value of
the`XmNrowColumnType`resource.

If`XmNrowColumnType`is set to`XmWORK_AREA`,`XmRowColumn`inherits translations from`XmManager`.

If`XmNrowColumnType`is set to`XmMENU_OPTION`,`XmRowColumn`inherits traversal,osfActivate, andosfCanceltranslations from`XmManager`and has the following additional translations.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **Btn2Down:** 

MenuGadgetDrag()
* **`c<Btn1Down>`:** 

MenuGadgetTraverseCurrent()
* **`c<Btn1Up>`:** 

MenuGadgetTraverseCurrentUp()
* **`&ap;c`BtnDown:** 

MenuBtnDown()
* **`&ap;c`BtnUp:** 

MenuBtnUp()
* **`:`KeyosfHelp:** 

MenuHelp()


The translations for`XmRowColumn`if`XmNrowColumnType`is set to`XmMENU_BAR``XmMENU_PULLDOWN`, or`XmMENU_POPUP`are described in the following list.
In a Popup menu system,Btn3also performs theBtn1actions.

* **`:`KeyosfHelp:** 

MenuHelp()
* **`:`KeyosfLeft:** 

MenuGadgetTraverseLeft()
* **`:`KeyosfRight:** 

MenuGadgetTraverseRight()
* **`:`KeyosfUp:** 

MenuGadgetTraverseUp()
* **`:`KeyosfDown:** 

MenuGadgetTraverseDown()

### Action Routines


The`XmRowColumn`action routines are

* **Help():** 

Calls the callbacks for`XmNhelpCallback`if any exist. If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **ManagerGadgetSelect():** 

When a gadget child of the menu has the focus, invokes the gadget
child's behavior associated withosfSelect.
This generally has the effect of unposting the menu hierarchy and arming
and activating the gadget, except that, for a CascadeButtonGadget with a
submenu, it posts the submenu.
* **MenuBtnDown():** 

When a gadget child of the menu has focus, invokes the gadget
child's behavior associated withBtn1Down.
This generally has the effect of unposting any menus posted by the
parent menu, enabling mouse traversal in the menu, and arming the
gadget.
For a CascadeButtonGadget with a submenu, it also posts the associated
submenu.
* **MenuBtnUp():** 

When a gadget child of the menu has focus, invokes the gadget
child's behavior associated withBtn1Up.
This generally has the effect of unposting the menu hierarchy and
activating the gadget,
except that for a CascadeButtonGadget with a
submenu, it posts the submenu and enables keyboard traversal in the
menu.
* **MenuGadgetEscape():** 

In a top-level Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and, when the shell's
keyboard focus policy is`XmEXPLICIT`, restores keyboard focus to
the widget that had the focus before the MenuBar was entered.
In other Pulldown MenuPanes, unposts the menu.

In a Popup MenuPane, unposts the menu and, when the shell's keyboard
focus policy is`XmEXPLICIT`, restores keyboard focus to the widget
from which the menu was posted.
In a TearOff MenuPane that has no submenus posted, dismisses the
menu; otherwise, if one or more submenus are posted, unposts the last
menu pane.
* **MenuGadgetTraverseDown():** 

If the current menu item has a submenu and is in a MenuBar, then this
action posts the submenu, disarms the current menu item, and arms
the submenu's first traversable menu item.

If the current menu item is in a MenuPane, then this action disarms the
current menu item and arms the item below it. This action wraps within the
MenuPane. The direction of the wrapping depends on the`XmNlayoutDirection`resource.
* **MenuGadgetTraverseLeft():** 

When the current menu item is in a MenuBar, this action disarms the
current item and arms the MenuBar item to the left.
This action wraps within the MenuBar.

In MenuPanes, if the current menu item is not at the left edge of a MenuPane,
this action disarms the current item and arms the item to its left.
If the current menu item is at the left edge of a submenu attached to a
MenuBar item, then this action unposts the submenu and traverses to the
MenuBar item to the left, wrapping if necessary. If that MenuBar item
has a submenu, it posts the submenu and arms the first traversable
item in the submenu.
If the current menu item is at the left edge of a submenu not directly
attached to a MenuBar item, then this action unposts the current submenu only.

In Popup or Torn-off MenuPanes, when the current menu item is at the
left edge, this action wraps within the MenuPane. If the current menu
item is at the left edge of the MenuPane and not in the top row, this
action wraps to the rightmost menu item in the row above. If the current
menu item is in the upper, leftmost corner, this action wraps
to the tear-off control, if present, or else it wraps to the bottom,
rightmost menu item in the MenuPane.

The preceding description applies when the`XmNlayoutDirection`horizontal
direction is`XmLEFT_TO_RIGHT`. If the`XmNlayoutDirection`horizontal
direction is`XmRIGHT_TO_LEFT`, then the following applies.

If the current menu item is in a MenuBar, then this action disarms the
current item and arms the MenuBar item to the left.
This action wraps within the MenuBar.

In MenuPanes, if the current menu item is a CascadeButton, then this
action posts its associated submenu.
If the current menu item is not a CascadeButton and is not at the left
edge of a MenuPane, this action disarms the current item and arms the
item to its left, wrapping if necessary.
If the current menu item is not a CascadeButton and is at the left edge of a
submenu that is a descendent of a MenuBar, then this action unposts all
submenus and traverses to the MenuBar item to the left.
If that MenuBar item has a submenu, it posts the submenu and arms
the submenu's first traversable item.

In Popup or Torn-off menus, if the current menu item is not a
CascadeButton and is at the left edge of a row (except the
bottom row), this action wraps to the rightmost menu item in the
row below. If the current menu item is not a CascadeButton and
is in the bottom, leftmost corner of a Popup or Pulldown MenuPane, this
action wraps to the tear-off control, if present, or else it wraps to
the top, rightmost menu item of the MenuPane.
* **MenuGadgetTraverseRight():** 

If the current menu item is in a MenuBar, then this action disarms the
current item and arms the MenuBar item to the right.
This action wraps within the MenuBar.

In MenuPanes, if the current menu item is a CascadeButton, then this
action posts its associated submenu.
If the current menu item is not a CascadeButton and is not at the right
edge of a MenuPane, this action disarms the current item and arms the
item to its right, wrapping if necessary.
If the current menu item is not a CascadeButton and is at the right edge of a
submenu that is a descendent of a MenuBar, then this action unposts all
submenus and traverses to the MenuBar item to the right.
If that MenuBar item has a submenu, it posts the submenu and arms
the submenu's first traversable item.

In Popup or Torn-off menus, if the current menu item is not a
CascadeButton and is at the right edge of a row (except the
bottom row), this action wraps to the leftmost menu item in the
row below. If the current menu item is not a CascadeButton and
is in the bottom, rightmost corner of a Popup or Pulldown MenuPane, this
action wraps to the tear-off control, if present, or else it wraps to
the top, leftmost menu item of the MenuPane.

The preceding description applies when the`XmNlayoutDirection`horizontal
direction is`XmLEFT_TO_RIGHT`. If the`XmNlayoutDirection`horizontal
direction is`XmRIGHT_TO_LEFT`, then the following applies.
When the current menu item is in a MenuBar, this action disarms the
current item and arms the MenuBar item to the left.
This action wraps within the MenuBar.

In MenuPanes, if the current menu item is not at the right edge of a MenuPane,
this action disarms the current item and arms the item to its right.
If the current menu item is at the right edge of a submenu attached to a
MenuBar item, then this action unposts the submenu and traverses to the
MenuBar item to the right, wrapping if necessary. If that MenuBar item
has a submenu, it posts the submenu and arms the first traversable
item in the submenu.
If the current menu item is at the right edge of a submenu not directly
attached to a MenuBar item, then this action unposts the current submenu only.

In Popup or Torn-off MenuPanes, when the current menu item is at the
right edge, this action wraps within the MenuPane. If the current menu
item is at the right edge of the MenuPane and not in the top row, this
action wraps to the leftmost menu item in the row above. If the current
menu item is in the upper, rightmost corner, this action wraps
to the tear-off control, if present, or else it wraps to the bottom,
leftmost menu item in the MenuPane.
* **MenuGadgetTraverseUp():** 

When the current menu item is in a MenuPane, then
this action disarms the current menu item and arms the item above it.
This action wraps within the MenuPane. The direction of the wrapping
depends on the`XmNlayoutDirection`resource.

### Related Behavior


The following menu functions are available:

* **osfMenuBar:** 

In any non-popup descendant of a MenuBar's parent, excluding the MenuBar
itself, this action enables keyboard traversal and moves keyboard focus
to the first item in the MenuBar.
In the MenuBar or any menu cascaded from it, this action unposts the
menu hierarchy and, when the shell's keyboard focus policy is`XmEXPLICIT`, restores focus to the widget that had the focus
when the menu system was entered.
* **osfMenu:** 

Pops up the menu associated with the control that has the keyboard focus.
Enables keyboard traversal in the menu.
In the Popup menu system or any menu cascaded from it, this action
unposts the menu hierarchy and, when the shell's keyboard focus policy
is`XmEXPLICIT`, restores focus to the widget that had the focus
when the menu system was entered.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmCreateMenuBar;,
&cdeman.XmCreateOptionMenu;,
&cdeman.XmCreatePopupMenu;,
&cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreateRadioBox;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimpleCheckBox;,
&cdeman.XmCreateSimpleMenuBar;,
&cdeman.XmCreateSimpleOptionMenu;,
&cdeman.XmCreateSimplePopupMenu;,
&cdeman.XmCreateSimplePulldownMenu;,
&cdeman.XmCreateSimpleRadioBox;,
&cdeman.XmCreateWorkArea;,
&cdeman.XmGetMenuCursor;,
&cdeman.XmGetPostedFromWidget;,`XmGetTearOffControl`,
&cdeman.XmLabel;,
&cdeman.XmManager;,
&cdeman.XmMenuPosition;,
&cdeman.XmOptionButtonGadget;,
&cdeman.XmOptionLabelGadget;,`XmRepTypeInstallTearOffModelConverter`,
&cdeman.XmSetMenuCursor;,
&cdeman.XmUpdateDisplay;,
&cdeman.XmVaCreateSimpleCheckBox;,
&cdeman.XmVaCreateSimpleMenuBar;,
&cdeman.XmVaCreateSimpleOptionMenu;,
&cdeman.XmVaCreateSimplePopupMenu;,
&cdeman.XmVaCreateSimplePulldownMenu;, and
&cdeman.XmVaCreateSimpleRadioBox;.