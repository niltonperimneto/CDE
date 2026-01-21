# XmLabelGadget
library call`XmLabelGadget`The LabelGadget widget classXmLabelGadgetwidget classLabelGadget&npzwc;#include &lt;Xm/LabelG.h>
## DESCRIPTION


LabelGadget is an instantiable widget and is also used as a superclass for
other button gadgets, such as PushButtonGadget and ToggleButtonGadget.

LabelGadget can contain either text or a pixmap. LabelGadget text is a
compound string.
Refer to theMotif Programmer'sGuide for more information on compound strings.
The text can be multilingual,
multiline, and/or multifont. When a LabelGadget is insensitive, its text is
stippled, or the user-supplied insensitive pixmap is displayed.

LabelGadget supports both accelerators and mnemonics primarily for use in
LabelGadget subclass widgets that are contained in
menus. Mnemonics are available in
a menu system when the button is visible. Accelerators in a
menu system are accessible even when the button is not visible.
The LabelGadget displays the mnemonic by underlining the first matching
character in the text string. The accelerator is displayed
as a text string adjacent to the label text or pixmap, depending on
the layout direction.

LabelGadget consists of many margin fields surrounding the text or
pixmap.
These margin fields are resources that may
be set by the user, but LabelGadget subclasses and Manager parents also
modify some of these fields. They tend to modify the`XmNmarginLeft`,`XmNmarginRight`,`XmNmarginTop`, and`XmNmarginBottom`resources and leave the`XmNmarginWidth`and`XmNmarginHeight`resources as set by the application.

LabelGadget takes into account`XmNshadowThickness`in determining
its layout but does not draw the shadow.
That is, if`XmNshadowThickness`is greater than 0 (zero), LabelGadget
leaves space for the shadow, but the shadow does not appear.

In a LabelGadget,`XmNtraversalOn`and`XmNhighlightOnEnter`are
forced to False inside Popup menu panes, Pulldown menu panes, and
OptionMenus.
Otherwise these resources default to False.

LabelGadget uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits, and
holds the`XmQTaccessTextual`,`XmQTcareParentVisual`,`XmQTmenuSavvy`, and`XmQTtransfer`traits.
### Data Transfer Behavior


LabelGadget and it subclasses, except when used in a menu system,
support dragging of the label contents from the LabelGadget.
However, the label contents are draggable only if
the`XmNenableUnselectableDrag`resource of`XmDisplay`is set to True.

As a source of data, LabelGadget and its subclasses support the
following targets and associated conversions of data to these targets:

* **`locale`** 

If the`locale`target matches the widget's locale, the widget
transfers`XmNlabelString`in the encoding of the locale.
This target is supported only when`XmNlabelType`is`XmSTRING`.
* **`COMPOUND_TEXT`** 

The widget transfers`XmNlabelString`as type`COMPOUND_TEXT`.
This target is supported only when`XmNlabelType`is`XmSTRING`.
* **`PIXMAP`** 

The widget transfers`XmNlabelPixmap`as type`DRAWABLE`.
This target is supported only when`XmNlabelType`is`XmPIXMAP`.
* **`STRING`** 

The widget transfers`XmNlabelString`as type`STRING`.
This target is supported only when`XmNlabelType`is`XmSTRING`.
* **`TEXT`** 

If`XmNlabelString`is fully convertible to the encoding of the
locale, the widget transfers`XmNlabelString`in the encoding of
the locale.
Otherwise, the widget transfers`XmNlabelString`as type`COMPOUND_TEXT`.
This target is supported only when`XmNlabelType`is`XmSTRING`.
* **_MOTIF_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets it
supports for the`CLIPBOARD`selection.
When`XmNlabelType`is`XmSTRING`, these include the following
targets:

_MOTIF_COMPOUND_STRING

`COMPOUND_TEXT`

The encoding of the locale, if`XmNlabelString`is fully convertible
to the encoding of the locale

`STRING`, if`XmNlabelString`is fully convertible to`STRING`

When`XmNlabelType`is`XmPIXMAP`, the targets include`PIXMAP`.
* **_MOTIF_COMPOUND_STRING** 

The widget transfers`XmNlabelString`as a compound string in Byte
Stream format.
This target is supported only when`XmNlabelType`is`XmSTRING`.
* **_MOTIF_EXPORT_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets to be
used as the value of the DragContext's`XmNexportTargets`in a
drag-and-drop transfer.
When`XmNlabelType`is`XmSTRING`, these include_MOTIF_COMPOUND_STRING,`COMPOUND_TEXT`, the encoding of the
locale,`STRING`,`TEXT`,`BACKGROUND`, and`FOREGROUND`.
When`XmNlabelType`is`XmPIXMAP`, these include`PIXMAP`,`BACKGROUND`, and`FOREGROUND`.


As a source of data, LabelGadget also supports the following standard
Motif targets:

* **`BACKGROUND`** 

The widget transfers the parent's`XmNbackground`as type`PIXEL`.
* **`CLASS`** 

The widget finds the first shell in the widget hierarchy that has aWM_CLASSproperty and transfers the contents as text in the
current locale.
* **`CLIENT_WINDOW`** 

The widget finds the first shell in the widget hierarchy and transfers
its window as type`WINDOW`.
* **`COLORMAP`** 

The widget transfers the parent's`XmNcolormap`as type`COLORMAP`.
* **`FOREGROUND`** 

The widget transfers the parent's`XmNforeground`as type`PIXEL`.
* **`NAME`** 

The widget finds the first shell in the widget hierarchy that has aWM_NAMEproperty and transfers the contents as text in the current
locale.
* **`TARGETS`** 

The widget transfers, as type`ATOM`, a list of the targets it
supports.
These include the standard targets in this list.
When`XmNlabelType`is`XmSTRING`, these also include_MOTIF_COMPOUND_STRING,`COMPOUND_TEXT`, the encoding of the
locale,`STRING`, and`TEXT`.
When`XmNlabelType`is`XmPIXMAP`, these also include`PIXMAP`.
* **`TIMESTAMP`** 

The widget transfers the timestamp used to acquire the selection as type`INTEGER`.
* **_MOTIF_RENDER_TABLE** 

The widget transfers`XmNrenderTable`if it exists, or else the
default text render table, as type`STRING`.
* **_MOTIF_ENCODING_REGISTRY** 

The widget transfers its encoding registry as type`STRING`.
The value is a list of NULL separated items in the
form of tag encoding pairs.
This target symbolizes the transfer target for the
Motif Segment Encoding Registry.
Widgets and applications can use this Registry to register
text encoding formats for specified render table tags.
Applications access this Registry by calling`XmRegisterSegmentEncoding`and`XmMapSegmentEncoding`.

### Classes


LabelGadget inherits behavior,
resources, and traits from`Object`,`RectObj`and`XmGadget`.

The class pointer is`xmLabelGadgetClass`.

The class name is`XmLabelGadget`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmLabelGadget Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLCSGXmNacceleratorTextXmCAcceleratorTextXmStringNULLCSGXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimension0CSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimension0CSGXmNmarginRightXmCMarginRightDimension0CSGXmNmarginTopXmCMarginTopDimension0CSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringdynamicCSGXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

* **`XmNaccelerator`** 

Sets the accelerator on a button widget in a menu, which
activates a visible or invisible, but managed, button from the keyboard.
This resource is a string that describes a set of modifiers
and the key that may be used to select the button.
The format of this string
is identical to that used by the translations manager, with the exception
that only a single event may be specified and only`KeyPress`events are
allowed.

Accelerators for buttons are supported only for
PushButtonGadgets and ToggleButtonGadgets in Pulldown and Popup menus.
* **`XmNacceleratorText`** 

Specifies the text displayed
for the accelerator.
The text is displayed
adjacent to the label string or pixmap. The direction of its layout depends on
the`XmNlayoutDirection`resource of the widget.
Accelerator text for buttons is displayed only for
PushButtonGadgets and ToggleButtonGadgets in Pulldown and Popup Menus.
* **`XmNalignment`** 

Specifies the label alignment for text or pixmap.

* **`XmALIGNMENT_BEGINNING`(left alignment)** 

Causes the left sides of
the lines of text to be vertically aligned with the
left edge of the gadget. For a pixmap, its left side is vertically
aligned with the left edge of the gadget.
* **`XmALIGNMENT_CENTER`(center alignment)** 

Causes the centers of the
lines of text to be vertically aligned in the center
of the gadget. For a pixmap, its center is vertically aligned with the
center of the gadget.
* **`XmALIGNMENT_END`(right alignment)** 

Causes the right sides of the
lines of text to be vertically aligned with the
right edge of the gadget. For a pixmap, its right side is
vertically aligned with the right edge of the gadget.


The preceding descriptions for text are correct when`XmNlayoutDirection`is`XmLEFT_TO_RIGHT`. When that resource
is`XmRIGHT_TO_LEFT`, the descriptions for`XmALIGNMENT_BEGINNING`and`XmALIGNMENT_END`are switched.

If the parent is a RowColumn whose`XmNisAligned`resource is True,`XmNalignment`is forced to the same value as the RowColumn's`XmNentryAlignment`if the RowColumn's`XmNrowColumnType`is`XmWORK_AREA`or if the gadget is a subclass of`XmLabelGadget`.
Otherwise, the default is`XmALIGNMENT_CENTER`.
* **`XmNfontList`** 

Specifies the font of the text used in the gadget.`XmNfontList`is obsolete and exists for compatibility with previous releases. You
should now use`XmNrenderTable`instead of`XmNfontList`. If
both are specified, the render table will take precedence. If`XmNfontList`is NULL at initialization, the parent hierarchy of
the widget is searched for an ancestor that is a subclass of the`XmBulletinBoard`,`VendorShell`, or`XmMenuShell`widget
class. If such an ancestor is found, the font list is initialized to
the`XmNbuttonFontList`(for button gadget subclasses) or`XmNlabelFontList`of the ancestor widget. If no such ancestor is
found, the default is implementation dependent. Refer to
&cdeman.XmFontList; for more information on the creation and
structure of a font list.
* **`XmNlabelInsensitivePixmap`** 

Specifies a pixmap used as the button face if`XmNlabelType`is`XmPIXMAP`and the button is insensitive.
The default value,`XmUNSPECIFIED_PIXMAP`, displays an empty label.
* **`XmNlabelPixmap`** 

Specifies the pixmap when`XmNlabelType`is`XmPIXMAP`.
The default value,`XmUNSPECIFIED_PIXMAP`, displays an empty label.
* **`XmNlabelString`** 

Specifies the compound string when`XmNlabelType`is`XmSTRING`.
If the value of this resource is NULL, it is initialized to name of the
gadget converted to a compound string.
Refer to &cdeman.XmString;
for more information on the
creation and the structure of compound strings.
* **`XmNlabelType`** 

Specifies the label type.

* **`XmSTRING`** 

Text displays`XmNlabelString`
* **`XmPIXMAP`** 

Icon data in pixmap displays`XmNlabelPixmap`or`XmNlabelInsensitivePixmap`

* **`XmNmarginBottom`** 

Specifies the amount of spacing between the bottom of the label text
and the top of the bottom margin specified by`XmNmarginHeight`.
This may be modified by LabelGadget's subclasses.
For example, CascadeButtonGadget may increase this field to make room
for the cascade pixmap.
* **`XmNmarginHeight`** 

Specifies an equal amount of spacing above the margin
defined by`XmNmarginTop`and below the margin defined by`XmNmarginBottom`.`XmNmarginHeight`specifies the amount
of spacing between the top edge of the margin set by`XmNmarginTop`and the bottom edge of the top shadow, and the amount of spacing between
the bottom edge of the margin specified by`XmNmarginBottom`and the
top edge of the bottom shadow.
* **`XmNmarginLeft`** 

Specifies the amount of spacing
between the left edge of the label text and the right side of the left
margin (specified by`XmNmarginWidth`).
This may be modified by LabelGadget's subclasses.
For example, ToggleButtonGadget may increase this field to make room for
the toggle indicator and for spacing between the indicator and label.
Whether this actually applies to the left or right side of the label
depends on the value of`XmNlayoutDirection`.
* **`XmNmarginRight`** 

Specifies the amount of spacing
between the right edge of the label text and the left side of the right
margin (specified by`XmNmarginWidth`).
This may be modified by LabelGadget's subclasses.
For example, CascadeButtonGadget may increase this field to make room
for the cascade pixmap.
Whether this actually applies to the left or right side of the label
depends on the value of`XmNlayoutDirection`.
* **`XmNmarginTop`** 

Specifies the amount of spacing between the top of the label text and
the bottom of the top margin specified by`XmNmarginHeight`.
This may be modified by LabelGadget's subclasses.
For example, CascadeButtonGadget may increase this field to make room
for the cascade pixmap.
* **`XmNmarginWidth`** 

Specifies an equal amount of spacing to the left of the margin defined
by`XmNmarginLeft`and to the right of the margin defined by`XmNmarginRight`.`XmNmarginWidth`specifies the amount
of spacing between the left edge of the margin set by`XmNmarginLeft`and the right edge of the left shadow, and the amount of spacing between
the right edge of the margin specified by`XmNmarginRight`and the
left edge of the right shadow.
* **`XmNmnemonic`** 

Provides the user with an alternate means of activating a button.
A button in a MenuBar, a Popup menu pane, or a Pulldown
menu pane can have a mnemonic.

This resource contains a keysym as listed in the X11 keysym table.
The first character in the label string that exactly matches
the mnemonic in the character set specified in`XmNmnemonicCharSet`is underlined when the button is displayed.

When a mnemonic has been specified, the user activates the button by
pressing the mnemonic key while the button is visible.
If the button is a CascadeButtonGadget in a MenuBar and the MenuBar does
not have the focus, the user must use the`MAlt`modifier while
pressing the mnemonic.
The user can activate the button by pressing either the shifted or the
unshifted mnemonic key.
* **`XmNmnemonicCharSet`** 

Specifies the character set of the mnemonic for the label.
The default is`XmFONTLIST_DEFAULT_TAG`.
* **`XmNrecomputeSize`** 

Specifies a Boolean value that indicates whether the gadget
shrinks or expands to accommodate its contents (label string or
pixmap) as a result of an`XtSetValues`resource value that
would change the size of the gadget. If True, the gadget shrinks
or expands to exactly fit the label string or pixmap.
If False, the gadget never attempts to change size on its own.
* **`XmNrenderTable`** 

Specifies the render table associated with the`labelString`. If
this value is NULL at initialization, Label searches its parent
hierarchy for an ancestor that holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is
found, Label initializes`XmNrenderTable`to the`XmLABEL_RENDER_TABLE`value of the ancestor widget. Similarly, button
subclasses of Label initialize`XmNrenderTable`to the`XmBUTTON_RENDER_TABLE`value of the ancestor widget. (Note that all
current subclasses of Label are button subclasses.) If no such
ancestor is found, the default is implementation dependent. If a font
list and a render table are both specified, the render table will take
precedence. Refer to &cdeman.XmRenderTable; for more information on
the creation and structure of a render table.
* **`XmNstringDirection`** 

Is a synthetic resource for setting`XmNlayoutDirection`.
The values for this resource are`XmSTRING_DIRECTION_L_TO_R`and`XmSTRING_DIRECTION_R_TO_L`. Refer to the`XmNlayoutDirection`resource description. The`XmNstringDirection`resource is obsoleted by`XmNlayoutDirection`, but is kept here for backward compatibility.

### Inherited Resources


LabelGadget inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension0CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanFalseCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Behavior


XmLabelGadget includes behavior from XmGadget.
Additional XmLabelGadget behavior is described in the following list:

* **Btn2Down:** 

Drags the contents of a LabelGadget, identified when`BTransfer`is
pressed.
This action is undefined for LabelGadgets used in a menu system.
* **KeyosfHelp:** 

In a Popup or Pulldown MenuPane, unposts all menus in the menu hierarchy
and, when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard focus to the widget that had the focus before the
menu system was entered.
Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **keyosfCancel:** 

In a MenuBar, disarms the CascadeButton and the menu and, when the
shell's keyboard focus policy is`XmEXPLICIT`, restores keyboard
focus to the widget that had the focus before the menu was entered.

In a toplevel Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and, when the shell's
keyboard focus policy is`XmEXPLICIT`, restores keyboard focus to
the widget that had the focus before the MenuBar was entered.
In other Pulldown MenuPanes, unposts the menu.

In a Popup MenuPane, unposts the menu and, when the shell's keyboard
focus policy is`XmEXPLICIT`, restores keyboard focus to the widget
from which the menu was posted.
* **KeyosfDown:** 

If the current menu item has a submenu and is in a MenuBar, then this
action posts the submenu, disarms the current menu item, and arms
the submenu's first traversable menu item.

If the current menu item is in a MenuPane, then this action disarms the
current menu item and arms the item below it. This action wraps within the
MenuPane. The direction of the wrapping depends on the`XmNlayoutDirection`resource.
* **KeyosfLeft:** 

When the current menu item is in a MenuBar, then this action disarms the
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
* **KeyosfRight:** 

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

When the current menu item is in a MenuBar, then this action disarms the
current item and arms the MenuBar item to the right.
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
* **KeyosfUp:** 

When the current menu item is in a MenuPane, then
this action disarms the current menu item and arms the item above it.
This action wraps within the MenuPane. The direction of the wrapping
depends on the`XmNlayoutDirection`resource.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Object;, &cdeman.RectObj;,
&cdeman.XmCreateLabelGadget;,
&cdeman.XmFontListCreate;,
&cdeman.XmStringCreate;,
&cdeman.XmStringCreateLtoR;,
and &cdeman.XmGadget;.