# XmList
library call`XmList`The List widget classXmListwidget classList&npzwc;#include &lt;Xm/List.h>
## DESCRIPTION


List allows a user to select one or more items
from a group of choices. Items are selected from the list in a
variety of ways, using both the pointer and the keyboard.
List operates on an array of compound strings that are defined by the application.
Each compound string becomes an item in the List,
with the first compound string becoming
the item in position 1, the second becoming the item in position 2, and so on.

Specifying the number of items that are visible
sets the size of the List.
If the number of visible items is not specified, the height
of the list controls the number of visible items. Each item assumes
the height of the tallest element in the list.
To create a list that allows the user to scroll easily through a large
number of items, use the`XmCreateScrolledList`convenience
function.

To select items, move the pointer or cursor to the desired item and
press theBtn1mouse button or the key defined asosfSelect. There are several styles of selection behavior, and
they all highlight the selected item or items by displaying them in
inverse colors. An appropriate callback is invoked to notify the
application of the user's choice. The application then takes whatever
action is required for the specified selection.
When a List is insensitive, all of the list items are displayed in a
stippled fill pattern.

List uses the`XmQTspecifyRenderTable`,`XmQTscrollFrame`, and`XmQTnavigator`traits, and
holds the`XmQTtransfer`trait.
### Selection


Each list has one of four selection models:

Single Select

Browse Select

Multiple Select

Extended Select

In Single Select and Browse Select, at most one item is selected at a
time.
In Single Select, pressingBtn1on an item toggles its
selection state and deselects any other selected item.
In Browse Select, pressingBtn1on an item selects it and
deselects any other selected item; draggingBtn1moves the
selection as the pointer is moved. ReleasingBtn1on an
item moves the location cursor to that item.

In Multiple Select, any number of items can be selected at a time.
PressingBtn1on an item toggles its selection state but
does not deselect any other selected items.

In Extended Select, any number of items can be selected at a time,
and the user can easily select ranges of items.
PressingBtn1on an item selects it and deselects any other
selected item.
DraggingBtn1or pressing or draggingShiftBtn1following aBtn1action selects all items between the item under the
pointer and the item on whichBtn1was pressed.
This action also deselects any other selected items outside that
range.

Extended Select also allows the user to select and deselect
discontiguous ranges of items.
PressingCtrlBtn1on an item toggles its selection state but
does not deselect any other selected items.
DraggingCtrlBtn1or pressing or draggingShiftBtn1following aCtrlBtn1action sets the selection state of all items between
the item under the pointer and the item on whichCtrlBtn1was
pressed to the state of the item on whichCtrlBtn1was pressed.
This action does not deselect any other selected items outside that
range.

All selection operations available from the mouse are also available
from the keyboard.
List has two keyboard selection modes, Normal Mode and Add Mode.
In Normal Mode, navigation operations andosfSelectselect the
item at the location cursor and deselect any other selected
items.
In Add Mode, navigation operations have no effect on selection, andosfSelecttoggles the selection state of the item at the location
cursor without deselecting any other selected items, except in Single
Select.

Single and Multiple Select use Add Mode, and Browse Select uses Normal
Mode.

Extended Select can use either mode; the user changes modes by pressingosfAddMode.
In Extended Select Normal Mode, pressingosfSelecthas the same
effect as pressingBtn1;osfExtendand shifted navigation
have the same effect as pressingShiftBtn1following aBtn1action.
In Extended Select Add Mode, pressingosfSelecthas the same
effect as pressingCtrlBtn1;osfExtendand shifted navigation
have the same effect as pressingShiftBtn1following aCtrlBtn1action.

Normal Mode is indicated by a solid location cursor, and Add Mode is
indicated by a dashed location cursor.
### Data Transfer Behavior


List supports dragging of items from the List and transfer of items to
the clipboard.
When the user presses`BTransfer`on a selected item, the widget
transfers all selected items.
When the user presses`BTransfer`on an unselected item, the widget
transfers only that item.
Depending on the value of`XmNprimaryOwnership`, List can also
support primary selection.

When the`XmNconvertCallback`procedures are called, the`location_data`member of theXmConvertCallbackStructmember
is NULL if the selected items are being transferred.
If the selected items are not being transferred, this member has the
following value:
If a single item is being transferred, the value is an integer
representing the position of the item in the List.
A value of 1 transfers the first item in the List; a value of 2
transfers the second item; and so on.
If the entire contents of the List are being transferred, the value is
&minus;1.

As a source of data, List supports the following targets and associated
conversions of data to these targets:

* **`locale`** 

If the`locale`target matches the widget's locale, the widget
transfers the selected list items in the encoding of the locale.
Each item transferred except the last includes a trailing separator.
* **`COMPOUND_TEXT`** 

The widget transfers the selected list items as type`COMPOUND_TEXT`.
Each item transferred except the last includes a trailing separator.
* **`STRING`** 

The widget transfers the selected list items as type`STRING`.
Each item transferred except the last includes a trailing separator.
* **`TEXT`** 

If the selected list items are fully convertible to the encoding of the
locale, the widget transfers the selected list items in the encoding of
the locale.
Otherwise, the widget transfers the selected list items as type`COMPOUND_TEXT`.
Each item transferred except the last includes a trailing separator.
* **_MOTIF_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets it
supports for immediate transfer for the`CLIPBOARD`selection.
These include_MOTIF_COMPOUND_STRING.
If the selected list items are fully convertible to`STRING`, these
also include`STRING`; otherwise, they also include`COMPOUND_TEXT`.
* **_MOTIF_COMPOUND_STRING** 

The widget transfers the selected list items as a compound string in
Byte Stream format.
Each item transferred except the last includes a trailing separator.
* **_MOTIF_DEFERRED_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets it
supports for delayed transfer for the`CLIPBOARD`selection.
This widget currently supplies no targets for_MOTIF_DEFERRED_CLIPBOARD_TARGETS.
* **_MOTIF_EXPORT_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets to be
used as the value of the DragContext's`XmNexportTargets`in a
drag-and-drop transfer.
These include_MOTIF_COMPOUND_STRING,`COMPOUND_TEXT`, the
encoding of the locale,`STRING`,`TEXT`,`BACKGROUND`, and`FOREGROUND`.
* **_MOTIF_LOSE_SELECTION** 

When the widget loses the selection, it deselects all list items.


As a source of data, List also supports the following standard Motif
targets:

* **`BACKGROUND`** 

The widget transfers`XmNbackground`as type`PIXEL`.
* **`CLASS`** 

The widget finds the first shell in the widget hierarchy that has aWM_CLASSproperty and transfers the contents as text in the
current locale.
* **`CLIENT_WINDOW`** 

The widget finds the first shell in the widget hierarchy and transfers
its window as type`WINDOW`.
* **`COLORMAP`** 

The widget transfers`XmNcolormap`as type`COLORMAP`.
* **`FOREGROUND`** 

The widget transfers`XmNforeground`as type`PIXEL`.
* **`NAME`** 

The widget finds the first shell in the widget hierarchy that has aWM_NAMEproperty and transfers the contents as text in the current
locale.
* **`TARGETS`** 

The widget transfers, as type`ATOM`, a list of the targets it
supports.
These include the standard targets in this list.
These also include_MOTIF_COMPOUND_STRING,`COMPOUND_TEXT`,
the encoding of the locale,`STRING`, and`TEXT`.
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


List has no widget class destination procedure.
Subclasses and the`XmNdestinationCallback`procedures are
responsible for any data transfers to the widget.
### Classes


List inherits behavior, resources, and traits from`Core`and`XmPrimitive`.

The class pointer is`xmListWidgetClass`.

The class name is`XmList`.
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

`XmList Resource Set``Name``Class``Type``Default``Access`XmNautomaticSelectionXmCAutomaticSelectionXtEnumFalseCSGXmNbrowseSelectionCallbackXmCCallbackXtCallbackListNULLCXmNdefaultActionCallbackXmCCallbackXtCallbackListNULLCXmNdestinationCallbackXmCCallbackXtCallbackListNULLCXmNdoubleClickIntervalXmCDoubleClickIntervalintdynamicCSGXmNextendedSelectionCallbackXmCCallbackXtCallbackListNULLCXmNfontListXmCFontListXmFontListdynamicCSGXmNitemCountXmCItemCountint0CSGXmNitemsXmCItemsXmStringTableNULLCSGXmNlistMarginHeightXmCListMarginHeightDimension0CSGXmNlistMarginWidthXmCListMarginWidthDimension0CSGXmNlistSizePolicyXmCListSizePolicyunsigned charXmVARIABLECGXmNlistSpacingXmCListSpacingDimension0CSGXmNmatchBehaviorXmCMatchBehaviorunsigned charXmQUICK_NAVIGATECSGXmNmultipleSelectionCallbackXmCCallbackXtCallbackListNULLCXmNprimaryOwnershipXmCPrimaryOwnershipunsigned charXmOWN_NEVERCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNscrollBarDisplayPolicyXmCScrollBarDisplayPolicyunsigned charXmAS_NEEDEDCSGXmNselectColorXmCSelectColorXmRSelectColorXmREVERSED_GROUND_COLORSCSGXmNselectedItemCountXmCSelectedItemCountint0CSGXmNselectedItemsXmCSelectedItemsXmStringTableNULLCSGXmNselectedPositionCountXmCSelectedPositionCountint0CSGXmNselectedPositionsXmCSelectedPositionsunsigned int *NULLCSGXmNselectionModeXmCSelectionModeunsigned chardynamicCSGXmNselectionPolicyXmCSelectionPolicyunsigned charXmBROWSE_SELECTCSGXmNsingleSelectionCallbackXmCCallbackXtCallbackListNULLCXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSGXmNtopItemPositionXmCTopItemPositionint1CSGXmNvisibleItemCountXmCVisibleItemCountintdynamicCSG

* **`XmNautomaticSelection`** 

Invokes either`XmNbrowseSelectionCallback`or`XmNextendedSelectionCallback`whenBtn1is pressed and the items that are shown as selected change
if the value is True (or`XmAUTO`) and the selection
mode is either`XmBROWSE_SELECT`or`XmEXTENDED_SELECT`respectively.
If False (`XmNO_AUTO_SELECT`), no selection callbacks are invoked until
the user releases the mouse button.
See`Behavior`for
further details on the interaction of this resource with the selection modes.
* **`XmNbrowseSelectionCallback`** 

Specifies a list of callbacks that is called
when an item is selected in the browse selection mode. The reason is`XmCR_BROWSE_SELECT`.
* **`XmNdefaultActionCallback`** 

Specifies a list of callbacks that is called when an item is double
clicked orosfActivateis pressed.
The reason is`XmCR_DEFAULT_ACTION`.
* **`XmNdestinationCallback`** 

Specifies a list of callbacks called when the List is the destination of
a transfer operation.
The type of the structure whose address is passed to these callbacks isXmDestinationCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNdoubleClickInterval`** 

If a button click is followed by another button click within the time
span specified by this resource (in milliseconds), the button clicks
are considered a double-click action, rather than two single-click
actions.
The value must not be negative.
The default value is the display's multiclick time.
* **`XmNextendedSelectionCallback`** 

Specifies a list of callbacks that is called
when items are selected using the extended selection mode.
The reason is`XmCR_EXTENDED_SELECT`.
* **`XmNfontList`** 

Specifies the font list associated with the list items.`XmNfontList`is obsolete and exists only for compatibility with
previous releases. You should now use`XmNrenderTable`instead of`XmNfontList`. If both are specified, the render table will take
precedence. The font list is used in conjunction with the`XmNvisibleItemCount`resource to determine the height of the List widget.
If this
value is NULL at initialization, the parent hierarchy of the widget is
searched for a widget that
holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found, the font list is initialized to the`XmTEXT_RENDER_TABLE`value of the ancestor widget. If no such widget
is found, the default is implementation dependent. Refer to
&cdeman.XmFontList; for more information on a font list structure.
* **`XmNitemCount`** 

Specifies the total number of items.
The value must be the number of items in`XmNitems`and must not be
negative.
It is automatically updated by the list whenever an item is added to or
deleted from the list.
* **`XmNitems`** 

Points to an array of compound strings that are to be displayed as the list
items.
Refer to &cdeman.XmString; for more information on the creation and
structure of compound strings.`XtGetValues`for this resource returns the list items themselves,
not a copy of the list items.
The application must not free the returned items.
* **`XmNlistMarginHeight`** 

Specifies the height of the margin between the list border and the items.
* **`XmNlistMarginWidth`** 

Specifies the width of the margin between the list border and the items.
* **`XmNlistSizePolicy`** 

Controls the reaction of the List when an item grows horizontally beyond
the current size of the list work area.
If the value is`XmCONSTANT`, the list viewing area does not grow,
and a horizontal ScrollBar is added for a List whose parent is a ScrolledWindow.
If this resource is set to`XmVARIABLE`, the List grows to
match the
size of the longest item, and no horizontal ScrollBar appears.

When the value of this resource is`XmRESIZE_IF_POSSIBLE`, the List
attempts to grow or shrink to match the width of the widest item.
If it cannot grow to match the widest size, a horizontal ScrollBar is
added for a List whose parent is a ScrolledWindow
if the longest item is wider than the list
viewing area.

The size policy must be set at the time the List widget is created.
It cannot be changed at a later time through`XtSetValues`.
* **`XmNlistSpacing`** 

Specifies the spacing between list items.
This spacing increases by the value of the`XmNhighlightThickness`resource in Primitive.
* **`XmNmatchBehavior`** 

Specifies the matching behavior followed by XmList.
The current values are`XmNONE`and`XmQUICK_NAVIGATE`, as follows:

* **`XmNONE`** 

Specifies that the typed in characters are ignored.
* **`XmQUICK_NAVIGATE`** 

Specifies that 1-character navigation shall be supported when List
has focus. If the typed character is
the initial character of some set of items in
List, the first of those items following the current
item will be navigated to (become the current
item). If all such items precede the current item, the
first such item becomes the current item.
Subsequently, typing the same character will
cyclically navigate among the items with the same first
character.

* **`XmNmultipleSelectionCallback`** 

Specifies a list of callbacks that is called
when an item is selected in
multiple selection mode. The reason is`XmCR_MULTIPLE_SELECT`.
* **`XmNprimaryOwnership`** 

Specifies whether XmContainer takes ownership of the Primary selection
when a selection is made inside it. This resource can take the
following values:

* **`XmOWN_NEVER`** 

Never takes ownership.
* **`XmOWN_ALWAYS`** 

Always takes ownership.
* **`XmOWN_MULTIPLE`** 

Only takes ownership is more than one element has been selected.
* **`XmOWN_POSSIBLE_MULTIPLE`** 

Only takes ownership if more than one element can be selected at a
time.

* **`XmNrenderTable`** 

Specifies the render table associated with the list items. The render
table is used in conjunction with the`XmNvisibleItemCount`resource to determine the height of the List widget. If this
value is NULL at initialization, List searches its parent hierarchy
for a widget that
holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found, the render table is initialized to the`XmTEXT_RENDER_TABLE`value of the ancestor widget. If no such widget
is found, the default is implementation dependent. If a font list and
a render table are both specified, the
render table will take precedence. Refer to
&cdeman.XmRenderTable; for more information on the creation and
structure of a render table.
* **`XmNscrollBarDisplayPolicy`** 

Controls the display of vertical ScrollBars in a
List whose parent is a ScrolledWindow.
When the value of this resource is`XmAS_NEEDED`, a vertical
ScrollBar is displayed only when the number of items in the List exceeds
the number of visible items.
When the value is`XmSTATIC`, a vertical ScrollBar is always
displayed.
* **`XmNselectColor`** 

Allows the application to specify the color of the background rectangle
that indicates selected text. It takes two values:

* **`XmDEFAULT_SELECT_COLOR`** 

Causes the select color to be set to a color
between the background and the bottom shadow color.
* **`XmREVERSED_GROUND_COLORS`** 

Forces the select color to the
foreground color and the color of any text rendered over the
select color to be in the background color.
* **`HIGHLIGHT_COLOR`** 

Forces the fill color to use the highlight color.

* **`XmNselectedItemCount`** 

Specifies the number of strings in the selected items list.
The value must be the number of items in`XmNselectedItems`and must
not be negative.
* **`XmNselectedItems`** 

Points to an array of compound strings that represents the list items that
are currently selected, either by the user or by the application.`XtGetValues`for this resource returns the list items themselves,
not a copy of the list items.
The application must not free the returned items or the array.

Setting`XmNselectedItems`selects those list items that exactly
match items in the given`XmNselectedItems`list. There may be
additional items in`XmNselectedItems`that do not match items in
the list. These items remain until`XmNselectedItems`is updated.
If`XmNitems`is changed such that the list now contains items
matching previously unmatched items in`XmNselectedItems`, those
new items will also appear selected.

Any user interaction with the list that causes at least one item to be
selected or deselected and any call to`XmListDeleteAllItems`,`XmListDeleteItem`,`XmListDeleteItems`,`XmListDeleteItemsPos`,`XmListDeletePos`,`XmListDeletePositions`,`XmListDeselectAllItems`,`XmListDeselectItem`,`XmListDeselectPos`,`XmListSelectItem`,`XmListSelectPos`, or`XmListUpdateSelectedList`cause`XmNselectedItems`to be
updated immediately to exactly reflect the visual state of the list.
Calls to any other`XmList`functions do not affect`XmNselectedItems`.
* **`XmNselectedPositionCount`** 

Specifies the number of positions in the selected positions list.
The value must be the number of items in`XmNselectedPositions`
* **`XmNselectedPositions`** 

Points to an array of the positions of the selected items in the List.`XtGetValues`for this resource returns the
list items themselves, not a copy of the list
items. The application must not free the returned
items or the array.
* **`XmNselectionMode`** 

Defines what effect keyboard navigations have on selection. The valid
modes are:

* **`XmADD_MODE`** 

Allows no navigation operations to have effect on selection, andosfSelecttoggles the selection state of the item at the location
cursor without deselecting any other selected items, except in Single
Select. However, the widget cannot be put into add mode if the`XmNselectionPolicy`resource is an incompatible mode
(`XmNselectionPolicy`cannot be`XmBROWSE_SELECT`).
* **`XmNORMAL_MODE`** 

Allows navigation operations andosfSelectto select the
item at the location cursor and deselect any other selected
items. However, the widget cannot be put into normal mode if the`XmNselectionPolicy`resource is an incompatible mode
(`XmNselectionPolicy`cannot be`XmSINGLE_SELECT`or`XmMULTIPLE_SELECT`).

* **`XmNselectionPolicy`** 

Defines the interpretation of the selection action. This can be one of the
following:

* **`XmSINGLE_SELECT`** 

Allows only single selections
* **`XmMULTIPLE_SELECT`** 

Allows multiple selections
* **`XmEXTENDED_SELECT`** 

Allows extended selections
* **`XmBROWSE_SELECT`** 

Allows drag-and-browse functionality

* **`XmNsingleSelectionCallback`** 

Specifies a list of callbacks that is called
when an item is selected in
single selection mode. The reason is`XmCR_SINGLE_SELECT`.
* **`XmNstringDirection`** 

Is a synthetic resource for setting`XmNlayoutDirection`.
The values for this resource are`XmSTRING_DIRECTION_L_TO_R`and`XmSTRING_DIRECTION_R_TO_L`. Refer to the`XmNlayoutDirection`resource description. The`XmNstringDirection`resource is obsoleted by`XmNlayoutDirection`, but is kept here for backward compatibility.
* **`XmNtopItemPosition`** 

Specifies the position of the item that is the first visible item in the
list.
Setting this resource is equivalent to calling the`XmListSetPos`function.
The position of the first item in the list is 1; the position of the
second item is 2; and so on.
A position of 0 (zero) specifies the last item in the list.
The value must not be negative.
* **`XmNvisibleItemCount`** 

Specifies the number of items that can
fit in the visible space of the list work area. The List uses this
value to determine its height.
The value must be greater than 0 (zero).

### Inherited Resources


List inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


List defines a new callback structure. The application must first look at the
reason field and use only the structure members that are valid for that
particular reason, because not all fields are relevant for
every possible reason. The callback structure is defined as follows:typedef struct
{
        int`reason`;
        XEvent *`event`;
        XmString`item`;
        int`item_length`;
        int`item_position`;
        XmString *`selected_items`;
        int`selected_item_count`;
        int *`selected_item_positions`;
        char`selection_type`;
        unsigned char`auto_selection_type`;
} XmListCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback. It can be NULL.
* **`item`** 

The last item selected at the time of the`event`that caused the
callback.`item`points to a temporary storage space that is reused after the
callback is finished.
Therefore, if an application needs to save the item, it should copy the
item into its own data space.
* **`item_length`** 

The length in bytes of`item`.
This member is obsolete and exists for compatibility with
earlier releases.
* **`item_position`** 

The position (plus one) of`item`in the List's`XmNitems`array.
An`item_position`value of one symbolizes the first element in the
list.
* **`selected_items`** 

A list of items selected at the time of the`event`that caused
the callback.`selected_items`points to a temporary storage space that is reused
after the callback is finished.
Therefore, if an application needs to save the selected list, it should
copy the list into its own data space.
* **`selected_item_count`** 

The number of items in the`selected_items`list.
This number must be non-negative.
* **`selected_item_positions`** 

An array of integers, one for each selected item, representing the
position of each selected item in the List's`XmNitems`array.`selected_item_positions`points to a temporary storage space that
is reused after the callback is finished.
Therefore, if an application needs to save this array, it should copy
the array into its own data space.
* **`selection_type`** 

Indicates that the most recent extended selection was the initial
selection (`XmINITIAL`), a modification of an existing selection
(`XmMODIFICATION`), or an additional noncontiguous selection
(`XmADDITION`).
* **`auto_selection_type`** 

Indicates the type of automatic selection callback. The types of
callbacks include the following:

* **`XmAUTO_BEGIN`** 

Indicates the beginning of automatic selection.
* **`XmAUTO_MOTION`** 

Indicates that there is a button drag selection.
* **`XmAUTO_CANCEL`** 

Indicates that the new selection is cancelled.
* **`XmAUTO_NO_CHANGE`** 

Indicates that the currently selected item matches the initial item.
* **`XmAUTO_CHANGE`** 

Indicates that the currently selected item does not match the initial item.



The following table describes the reasons for which the individual callback
structure fields are valid.`Reason``Valid Fields`XmCR_SINGLE_SELECTreason, event, item, item_length, item_positionXmCR_DEFAULT_ACTIONreason, event, item, item_length, item_position, selected_items, selected_item_count, selected_item_positionsXmCR_BROWSE_SELECTreason, event, item, item_length, item_positionXmCR_MULTIPLE_SELECTreason, event, item, item_length, item_position, selected_items, selected_item_count, selected_item_positionsXmCR_EXTENDED_SELECTreason, event, item, item_length, item_position, selected_items, selected_item_count, selected_item_positions, selection_type

A pointer to the following callback structure is passed to the`XmNdestinationCallback`procedures:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Atom`selection`;
        XtEnum`operation`;
        int`flags`;
        XtPointer`transfer_id`;
        XtPointer`destination_data`;
        XtPointer`location_data`;
        Time`time`;
} XmDestinationCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL.
* **`selection`** 

Indicates the selection for which data transfer is being requested.
Possible values are`CLIPBOARD`,`PRIMARY`,`SECONDARY`, and_MOTIF_DROP.
* **`operation`** 

Indicates the type of transfer operation requested.

When the selection is`PRIMARY`, possible values are`XmMOVE`,`XmCOPY`, and`XmLINK`.

When the selection is`SECONDARY`or`CLIPBOARD`, possible
values are`XmCOPY`and`XmLINK`.

When the selection is_MOTIF_DROP, possible values are`XmMOVE`,`XmCOPY`,`XmLINK`, and`XmOTHER`.
A value of`XmOTHER`means that the callback procedure must get
further information from theXmDropProcCallbackStructin the`destination_data`member.
* **`flags`** 

Indicates whether or not the destination widget is also the source of
the data to be transferred.
Following are the possible values:

* **`XmCONVERTING_NONE`** 

The destination widget is not the source of the data to be transferred.
* **`XmCONVERTING_SAME`** 

The destination widget is the source of the data to be transferred.

* **`transfer_id`** 

Serves as a unique ID to identify the transfer transaction.
* **`destination_data`** 

Contains information about the destination.
When the selection is_MOTIF_DROP, the callback procedures are
called by the drop site's`XmNdropProc`, and`destination_data`is a pointer to theXmDropProcCallbackStructpassed to the`XmNdropProc`procedure.
When the selection is`SECONDARY`,`destination_data`is an Atom
representing a target recommmended by the selection owner for use in
converting the selection.
Otherwise,`destination_data`is NULL.
* **`location_data`** 

Contains information about the location where data is to be transferred.
The value is always NULL when the selection is`SECONDARY`or`CLIPBOARD`.
If the value is NULL, the data is to be inserted at the widget's cursor
position.
Otherwise, the value is an integer representing the position in the List
where the items are to be transferred.
A value of 1 makes the first new item the first item in the list; a
value of 2 makes it the second item; and so on.
Once`XmTransferDone`procedures start to be called,`location_data`will no longer be stable.
* **`time`** 

Indicates the time when the transfer operation began.

### Translations


`XmList`includes translations from Primitive.
The`XmList`translations are described in the following list.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **Btn1Motion:** 

ListButtonMotion()
* **`s &ap;m &ap;a`Btn1Down:** 

ListBeginExtend()
* **`s &ap;m &ap;a`Btn1Up:** 

ListEndExtend()
* **`c &ap;s &ap;m &ap;a`Btn1Down:** 

ListBeginToggle()
* **`c &ap;s &ap;m &ap;a`Btn1Up:** 

ListEndToggle()
* **`&ap;s &ap;c &ap;m &ap;a`Btn1Down:** 

ListBeginSelect()
* **`&npzwc;&ap;s &ap;c &ap;m &ap;a`Btn1Up:** 

ListEndSelect()
* **Btn2Down:** 

ListProcessDrag()
* **`:s c`KeyosfBeginLine:** 

ListBeginDataExtend()
* **`:c`KeyosfBeginLine:** 

ListBeginData()
* **`:`KeyosfBeginLine:** 

ListBeginLine()
* **`:s c`KeyosfEndLine:** 

ListEndDataExtend()
* **`:c`KeyosfEndLine:** 

ListEndData()
* **`:`KeyosfEndLine:** 

ListEndLine()
* **`:`KeyosfPageLeft:** 

ListLeftPage()
* **`:c`KeyosfPageUp:** 

ListLeftPage()
* **`:`KeyosfPageUp:** 

ListPrevPage()
* **`:`KeyosfPageRight:** 

ListRightPage()
* **`:c`KeyosfPageDown:** 

ListRightPage()
* **`:`KeyosfPageDown:** 

ListNextPage()
* **`s`KeyDownosfSelect`:`** 

ListKbdBeginExtend()
* **`:`KeyDownosfSelect:** 

ListKbdBeginSelect()
* **`:s`KeyUposfSelect:** 

ListKbdEndExtend()
* **`:`KeyUposfSelect:** 

ListKbdEndSelect()
* **`:`KeyosfSelectAll:** 

ListKbdSelectAll()
* **`:`KeyosfDeselectAll:** 

ListKbdDeSelectAll()
* **`:`KeyosfActivate:** 

ListKbdActivate()
* **`:`KeyosfAddMode:** 

ListAddMode()
* **`:`KeyosfHelp:** 

PrimitiveHelp()
* **`:`KeyosfCancel:** 

ListKbdCancel()
* **`:c`KeyosfLeft:** 

ListLeftPage()
* **`:`KeyosfLeft:** 

ListLeftChar()
* **`:c`KeyosfRight:** 

ListRightPage()
* **`:`KeyosfRight:** 

ListRightChar()
* **`:s`KeyosfUp:** 

ListExtendPrevItem()
* **`:`KeyosfUp:** 

ListPrevItem()
* **`:s`KeyosfDown:** 

ListExtendNextItem()
* **`:`KeyosfDown:** 

ListNextItem()
* **`:c`KeyosfInsert:** 

ListCopyToClipboard()
* **`:`KeyosfCopy:** 

ListCopyToClipboard()
* **`&ap;s c &ap;m &ap;a`Key`slash`:** 

ListKbdSelectAll()
* **`&ap;s c &ap;m &ap;a`Key`backslash`:** 

ListKbdDeSelectAll()
* **`s &ap;m &ap;a`Key`Tab`:** 

PrimitivePrevTabGroup()
* **`&ap;m &ap;a`Key`Tab`:** 

PrimitiveNextTabGroup()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

ListKbdActivate()
* **`&ap;s &ap;m &ap;a`KeyDown`space`:** 

ListKbdBeginSelect()
* **`&ap;s &ap;m &ap;a`KeyUp`space`:** 

ListKbdEndSelect()
* **`s &ap;m &ap;a`KeyDown`space`:** 

ListKbdBeginExtend()
* **`s &ap;m &ap;a`KeyUp`space`:** 

ListKbdEndExtend()
* **Key:** 

ListQuickNavigate()


The List button event translations are modified when Display's`XmNenableBtn1Transfer`resource does not have a value of`XmOFF`(in other words, it is either`XmBUTTON2_TRANSFER`or`XmBUTTON2_ADJUST`). This
option allows the
actions for selection and transfer to be integrated onBtn1, and
the actions for extending the selection can be bound toBtn2. The actions forBtn1that are defined above
still apply when theBtn1event occurs over text that is not
selected. The following actions apply when theBtn1event
occurs over text that is selected:

* **Btn1Motion`:`** 

ListProcessBtn1(`ListButtonMotion`)
* **`s &ap;m &ap;a`Btn1Down** 

ListProcessBtn1(`ListBeginExtend`)
* **`s &ap;m &ap;a`Btn1Up** 

ListProcessBtn1(`ListEndExtend`)
* **`c &ap;s &ap;m &ap;a`Btn1Down** 

ListProcessBtn1(`ListBeginToggle`)
* **`c &ap;s &ap;m &ap;a`Btn1Up** 

ListProcessBtn1(`ListEndToggle`)
* **`&ap;s &ap;c &ap;m &ap;a`Btn1Down** 

ListProcessBtn1(`ListBeginSelect`)
* **`&ap;s &ap;c &ap;m &ap;a`Btn1Up** 

ListProcessBtn1(`ListEndSelect`)


When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_ADJUST`, the following actions apply:

* **Btn2Down`:`** 

ListProcessBtn2(`ListBeginExtend`)
* **Btn2Motion`:`** 

ListProcessBtn2(`ListButtonMotion`)
* **Btn2Up`:`** 

ListProcessBtn2(`ListEndExtend`)

### Action Routines


The`XmList`action routines are described in the following list.
The current selection is always shown with inverted colors.

* **ListAddMode():** 

Toggles the state of Add Mode for keyboard selection.
* **ListBeginData():** 

Moves the location cursor to the first item in the list.
In Normal Mode, this also deselects any current selection,
selects the first item in the list, and calls the appropriate selection
callbacks (`XmNbrowseSelectionCallback`when`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,`XmNextendedSelectionCallback`when`XmNselectionPolicy`is set
to`XmEXTENDED_SELECT`).
* **ListBeginDataExtend():** 

If`XmNselectionPolicy`is set to`XmMULTIPLE_SELECT`or`XmEXTENDED_SELECT`,
this action moves the location cursor to the first item in the list.

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action does the following:
If an extended selection has been made from the current anchor point,
restores the selection state of the items in that range to their state
before the extended selection was done;
changes the selection state of the first item and all
items between it and the current anchor point to the state of the
item at the current anchor point;
calls the`XmNextendedSelectionCallback`callbacks.
* **ListBeginExtend():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action does the following:
If an extended selection has been made from the current anchor point,
restores the selection state of the items in that range to their state
before the extended selection was done, and
changes the selection state of the item under the pointer and all
items between it and the current anchor point to the state of the
item at the current anchor point.
If`XmNautomaticSelection`is set to True, this action calls the`XmNextendedSelectionCallback`callbacks.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`.
* **ListBeginLine():** 

Moves the horizontal scroll region to the beginning of the line.
* **ListBeginSelect():** 

If`XmNselectionPolicy`is set to`XmSINGLE_SELECT`,
deselects any current selection and toggles the selection state of the
item under the pointer.

If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
deselects any current selection and selects the item under the pointer.
If`XmNautomaticSelection`is set to True, calls the`XmNbrowseSelectionCallback`callbacks.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`.

If`XmNselectionPolicy`is set to`XmMULTIPLE_SELECT`,
toggles the selection state of the item under the pointer.
Any previous selections remain.

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action deselects any current selection,
selects the item under the pointer, and
sets the current anchor at that item.
If`XmNautomaticSelection`is set to True,
this action calls the`XmNextendedSelectionCallback`callbacks.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`.
* **ListBeginToggle():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action moves the current anchor to the item under the pointer
without changing the current selection.
If the item is unselected, this action selects it; if the item is selected,
this action unselects it.
If`XmNautomaticSelection`is set to True, this action calls the`XmNextendedSelectionCallback`callbacks.

Otherwise, the list takes keyboard focus. No other action occurs.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`.
* **ListButtonMotion():** 

If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
this action deselects any current selection and selects the item under the
pointer.
If`XmNautomaticSelection`is set to True and the pointer has
entered a new list item, this action calls the`XmNbrowseSelectionCallback`callbacks.

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action does the following:
If an extended selection is being made and an extended selection has
previously been made from the current anchor point,
restores the selection state of the items in that range to their state
before the previous extended selection was done and
changes the selection state of the item under the pointer and all
items
between it and the current anchor point to the state of the
item at the current anchor point.
If`XmNautomaticSelection`is set to True and the pointer has
entered a new list item, calls the`XmNextendedSelectionCallback`callbacks.

If the pointer leaves a scrolled list, this action scrolls the list in
the direction of the pointer motion.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_MOTION`.
* **ListCopyToClipboard()** 

Copies the content of the selected items to the clipboard as a single
compound string with each
item separated by a newline.
This action calls the`XmNconvertCallback`procedures, possibly
multiple times, for the`CLIPBOARD`selection.
* **ListEndData():** 

Moves the location cursor to the last item in the list.
In Normal Mode, this also deselects any current selection,
selects the last item in the list, and calls the appropriate selection
callbacks (`XmNbrowseSelectionCallback`when`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,`XmNextendedSelectionCallback`when`XmNselectionPolicy`is set
to`XmEXTENDED_SELECT`).
* **ListEndDataExtend():** 

If`XmNselectionPolicy`is set to`XmMULTIPLE_SELECT`or`XmEXTENDED_SELECT`,
this action moves the location cursor to the last item in the list.

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action does the following:
If an extended selection has been made from the current anchor point,
restores the selection state of the items in that range to their state
before the extended selection was done;
changes the selection state of the last item and all
items between it and the current anchor point to the state of the
item at the current anchor point;
calls the`XmNextendedSelectionCallback`callbacks.
* **ListEndExtend():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action moves the location
cursor to the last item selected or deselected
and
calls the`XmNextendedSelectionCallback`callbacks.

If`XmNautomaticSelection`is set to True, then the`auto_selection_type`field of the callback will be
valid. If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`and the currently selected item position matches the position of the
item that was selected before the browse selection began, or if`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`and the set
of currently selected item positions matches the set of item positions
selected before the extended selection began, the callback will be
called with`auto_selection_type`set to`XmAUTO_NO_CHANGE`.
Otherwise, it will be set to`XmAUTO_CHANGE`.
* **ListEndLine():** 

Moves the horizontal scroll region to the end of the line.
* **ListEndSelect():** 

This action moves the location cursor to the last item selected or
deselected and calls the appropriate selection callbacks
(`XmNsingleSelectionCallback`when`XmNselectionPolicy`is set
to`XmSINGLE_SELECT`,`XmNbrowseSelectionCallback`when`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,`XmNmultipleSelectionCallback`when`XmNselectionPolicy`is set to`XmMULTIPLE_SELECT`,`XmNextendedSelectionCallback`when`XmNselectionPolicy`is
set to`XmEXTENDED_SELECT`).

If`XmNautomaticSelection`is set to True, then the`auto_selection_type`field of the callback will be valid.
If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`and the
currently selected item position matches the position of the item that
was selected before the brose selection began, or if`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`and the set
of currently selected item positions matches the set of item positions
selected before the extended selection began, the callback will be
called with`auto_selection_type`set to`XmAUTO_NO_CHANGE`.
Otherwise, it will be set to`XmAUTO_CHANGE`.
* **ListEndToggle():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`, moves
the location cursor to the last item selected or deselected
and
calls the`XmNextendedSelectionCallback`callbacks.

If`XmNautomaticSelection`is set to True, then the`auto_selection_type`field of the callback will be valid. If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`and the
currently selected item position matches the position of the item that
was selected before the browse selection began, or if`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`and the set
of currently selected item positions matches the set of item positions
selected before the extended selection began, the callback will be
called with`auto_selection_type`set to`XmAUTO_NO_CHANGE`.
Otherwise, it will be set to`XmAUTO_CHANGE`.
* **ListExtendNextItem():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action
does the following:
If an extended selection has been made from the current anchor point,
restores the selection state of the items in that range to their state
before the extended selection was done;
moves the location cursor to the next item and changes the selection
state of the item and all
items between it and the current anchor point
to the state of the item at the current anchor point;
calls the`XmNextendedSelectionCallback`callbacks.
* **ListExtendPrevItem():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action does the following:
If an extended selection has been made from the current anchor point,
restores the selection state of the items in that range to their state
before the extended selection was done;
moves the location cursor to the previous item and changes the selection
state of the item and all items between it and the current anchor point
to the state of the item at the current anchor point;
calls the`XmNextendedSelectionCallback`callbacks.
* **ListScrollCursorVertically(`percentage``)`:** 

Scrolls the line containing the insertion cursor vertically
to an intermediate position in the visible window based on an
input percentage. A value of 0 (zero) indicates the top of the window;
a value of 100, the bottom of the window. If this action is called
with no argument, the line containing the insertion cursor is scrolled
vertically to a new position designated by theyevent passed
to the routine.
* **ListKbdActivate():** 

Calls the callbacks for`XmNdefaultActionCallback`.
If the List's parent is a manager, this action passes the event to the
parent.
* **ListKbdBeginExtend():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
does the following:
If an extended selection has been made from the current anchor point,
restores the selection state of the items in that range to their state
before the extended selection was done;
changes the selection state of the item at the location cursor and all
items between it and the current anchor point to the state of the
item at the current anchor point.
If`XmNautomaticSelection`is set to True, this action calls the`XmNextendedSelectionCallback`callbacks.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`".
* **ListKbdBeginSelect():** 

If the`XmNselectionPolicy`is set to`XmSINGLE_SELECT`,
deselects any current selection and toggles the state of the item at the
location cursor.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`".

If the`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
deselects any current selection and selects the item at the location
cursor.
If`XmNautomaticSelection`is set to True, calls the`XmNbrowseSelectionCallback`callbacks.

If the`XmNselectionPolicy`is set to`XmMULTIPLE_SELECT`,
toggles the selection state of the item at the location cursor.
Any previous selections remain.

If the`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
moves the current anchor to the item at the location cursor.
In Normal Mode, this action
deselects any current selection and selects the item at
the location cursor.
In Add Mode, this action
toggles the selection state of the item at the location
cursor and leaves the current selection unchanged.
If`XmNautomaticSelection`is set to True, this action
calls the`XmNextendedSelectionCallback`callbacks.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_BEGIN`".
* **ListKbdCancel():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`and an extended selection is being made from the current anchor point,
this action
cancels the new selection and restores the selection state of the items
in that range to their state before the extended selection was done.
If`XmNautomaticSelection`is set to True, this action calls the`XmNextendedSelectionCallback`callbacks; otherwise, if the
parent is a manager, it passes the event to the parent.
The`auto_selection_type`component of the callback structure will
be set to`XmAUTO_CANCEL`".
* **ListKbdDeSelectAll():** 

If the`XmNselectionPolicy`is set to`XmSINGLE_SELECT`,`XmMULTIPLE_SELECT`, or`XmEXTENDED_SELECT`in Add Mode,
this action deselects all items in the list.
If the`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`in
Normal Mode, this action deselects all items in the list (except the item at the
location cursor if the shell's`XmNkeyboardFocusPolicy`is`XmEXPLICIT`).
This action also calls the appropriate selection callbacks
(`XmNsingleSelectionCallback`when`XmNselectionPolicy`is set
to`XmSINGLE_SELECT`,`XmNmultipleSelectionCallback`when`XmNselectionPolicy`is set to`XmMULTIPLE_SELECT`,`XmNextendedSelectionCallback`when`XmNselectionPolicy`is set
to`XmEXTENDED_SELECT`).
* **ListKbdEndExtend():** 

If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action calls the`XmNextendedSelectionCallback`callbacks.

If`XmNautomaticSelection`is set to True, then the`auto_selection_type`field of the callback will be valid. If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`and the
currently selected item position matches the position of the item that
was selected before the browse selection began, or if`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`and the set
of currently selected item positions matches the set of item positions
selected before the extended selection began, the callback will be
called with`auto_selection_type`set to`XmAUTO_NO_CHANGE`.
Otherwise, it will be set to`XmAUTO_CHANGE`.
* **ListKbdEndSelect():** 

Calls the appropriate selection callbacks
(`XmNsingleSelectionCallback`when`XmNselectionPolicy`is set
to`XmSINGLE_SELECT`,`XmNbrowseSelectionCallback`when`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,`XmNmultipleSelectionCallback`when`XmNselectionPolicy`is set
to`XmMULTIPLE_SELECT`,`XmNextendedSelectionCallback`when`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`).

If`XmNautomaticSelection`is set to True, then the`auto_selection_type`field of the callback will be valid. If`XmNselectionPolicy`is set to`XmBROWSE_SELECT`and the
currently selected item position matches the position of the item that
was selected before the browse selection began, or if`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`and the set
of currently selected item positions matches the set of item positions
selected before the extended selection began, the callback will be
called with`auto_selection_type`set to`XmAUTO_NO_CHANGE`.
Otherwise, it will be set to`XmAUTO_CHANGE`.
* **ListKbdSelectAll():** 

If`XmNselectionPolicy`is set to`XmSINGLE_SELECT`or`XmBROWSE_SELECT`, this action selects the item at the location cursor.
If`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`or`XmMULTIPLE_SELECT`,
it selects all items in the list.
This action also calls the appropriate selection callbacks
(`XmNsingleSelectionCallback`when`XmNselectionPolicy`is set
to`XmSINGLE_SELECT`,`XmNbrowseSelectionCallback`when`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,`XmNmultipleSelectionCallback`when`XmNselectionPolicy`is set
to`XmMULTIPLE_SELECT`,`XmNextendedSelectionCallback`when`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`).
* **ListLeftChar():** 

Scrolls the list one character to the left.
* **ListLeftPage():** 

Scrolls the list one page to the left.
* **ListNextItem():** 

Moves the location cursor to the next item in the list.

If the`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
this action also selects the next item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.

If the`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action in Normal Mode also selects the next item, deselects any
current selection, moves the current anchor to the next item, and calls
the`XmNextendedSelectionCallback`callbacks.
In Add Mode, this action does not affect the selection or the anchor.
* **ListNextPage():** 

Scrolls the list to the next page, moving the location cursor to a new
item.

If the`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
this action also selects the new item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.

If the`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action in Normal Mode also selects the new item, deselects any
current selection, moves the current anchor to the new item, and calls
the`XmNextendedSelectionCallback`callbacks.
In Add Mode, this action does not affect the selection or the anchor.
* **ListPrevItem():** 

Moves the location cursor to the previous item in the list.

If the`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
this action also selects the previous item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.

If the`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action in Normal Mode also selects the previous item, deselects any
current selection, moves the current anchor to the previous item, and
calls the`XmNextendedSelectionCallback`callbacks.
In Add Mode, this action does not affect the selection or the anchor.
* **ListPrevPage():** 

Scrolls the list to the previous page, moving the location cursor to a
new item.

If the`XmNselectionPolicy`is set to`XmBROWSE_SELECT`,
this action also selects the new item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.

If the`XmNselectionPolicy`is set to`XmEXTENDED_SELECT`,
this action in Normal Mode also selects the new item, deselects any
current selection, moves the current anchor to the new item, and calls
the`XmNextendedSelectionCallback`callbacks.
In Add Mode this action does not affect the selection or the anchor.
* **ListProcessBtn1(`string`)** 

When Display's`XmNenableBtn1Transfer`resource is not`XmOFF`,
the
actions for selection and transfer are integrated onBtn1.
When the button is not performing a transfer or drag, the action that is
performed depends on the value of`string`, which can be one of
the following actions:

`ListButtonMotion`

`ListBeginExtend`

`ListEndExtend`

`ListBeginToggle`

`ListEndToggle`

`ListBeginSelect`

`ListEndSelect`
* **`ListProcessBtn2`** 

When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_TRANSFER`, the
actions for extending selection are bound onBtn2, and a drag
starts immediately.
When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_ADJUST`, the action that is
performed depends on the value of`string`, which can be one of
the following actions:

`ListBeginExtend`

`ListButtonMotion`

`ListEndExtend`
* **ListProcessDrag():** 

Drags the content of one or more selected list items.
Each item is separated by a newline.
If`BTransfer`is pressed on an unselected item, it drags only that
item, excluding any other selected items.
This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures, possibly
multiple times, for the_MOTIF_DROPselection.
* **`ListQuickNavigate`** 

Navigates to an item. When List's`XmNmatchBehavior`resource is`XmQUICK_NAVIGATE`, this
action uses 1-character
navigation to navigate. Refer to the`XmNmatchBehavior`resource
for a description of how this navigation works.
* **ListRightChar():** 

Scrolls the list one character to the right.
* **ListRightPage():** 

Scrolls the list one page to the right.
* **PrimitiveHelp():** 

Calls the callbacks for`XmNhelpCallback`if any exist. If there
are no help callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **PrimitiveNextTabGroup():** 

Moves the focus to the first item contained within the next tab group. If
the current tab group is the last entry in the tab group list, it
wraps to the beginning of the tab group list.
* **PrimitivePrevTabGroup():** 

Moves the focus to the first item contained within the previous tab group.
If the beginning of the tab group list is reached, it wraps to the end
of the tab group list.

### Additional Behavior


The List widget has the following additional behavior:

* **Double&ensp;Click** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time, the List interprets
that as a double click and calls the callbacks for`XmNdefaultActionCallback`.
The item's colors invert to indicate that it is selected.
The`XmNdoubleClickInterval`resource can be used to specify a
time span that overrides the display's multi-click time.
* **FocusIn:** 

If the focus policy is Explicit, this action sets the focus and draw
the location cursor.
* **FocusOut:** 

If the focus policy is Explicit, this action removes the focus and erase
the location cursor.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;, &cdeman.XmCreateList;,
&cdeman.XmCreateScrolledList;,
&cdeman.XmFontListCreate;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmListAddItem;,
&cdeman.XmListAddItems;,
&cdeman.XmListAddItemUnselected;,
&cdeman.XmListAddItemsUnselected;,
&cdeman.XmListDeleteAllItems;,
&cdeman.XmListDeleteItem;,
&cdeman.XmListDeleteItems;,
&cdeman.XmListDeleteItemsPos;,
&cdeman.XmListDeletePos;,
&cdeman.XmListDeletePositions;,
&cdeman.XmListDeselectAllItems;,
&cdeman.XmListDeselectItem;,
&cdeman.XmListDeselectPos;,`XmListGetKbdItemPos`&cdeman.XmListGetMatchPos;,
&cdeman.XmListGetSelectedPos;,
&cdeman.XmListItemExists;,
&cdeman.XmListItemPos;,
&cdeman.XmListPosToBounds;,
&cdeman.XmListReplaceItems;,
&cdeman.XmListReplaceItemsPos;,
&cdeman.XmListReplaceItemsPos;,
&cdeman.XmListReplaceItemsPosUnselected;,
&cdeman.XmListReplaceItemsUnselected;,
&cdeman.XmListSelectItem;,
&cdeman.XmListSelectPos;,
&cdeman.XmListSetAddMode;,
&cdeman.XmListSetBottomItem;,
&cdeman.XmListSetBottomPos;,
&cdeman.XmListSetHorizPos;,
&cdeman.XmListSetItem;,
&cdeman.XmListSetKbdItemPos;,
&cdeman.XmListSetPos;,
&cdeman.XmListUpdateSelectedList;,
&cdeman.XmListYToPos;,
&cdeman.XmPrimitive; and
&cdeman.XmStringCreate;.