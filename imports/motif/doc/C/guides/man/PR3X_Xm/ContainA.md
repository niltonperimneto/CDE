# XmContainer
library call`XmContainer`The Container widget classXmContainercontainerContainer&npzwc;#include &lt;Xm/Container.h>
## DESCRIPTION


Container manages child widgets that have the`ContainerItem`trait.
These child widgets can be viewed in several different
layout formats,
selected using different selection types and techniques,
and directly manipulated by the user.

Three different formats or views are supported by the Container.
They are specified via the`XmNentryViewType`resource:

`XmLARGE_ICON`

`XmSMALL_ICON`

`XmANY_ICON`

Three different layout types are supported by the Container.
They are specified by the`XmNlayoutType`resource:

`XmSPATIAL`

`XmOUTLINE`

`XmDETAIL`

In the`XmSPATIAL`layout type, several other resources
(`XmNspatialStyle`,`XmNspatialIncludeModel`,`XmNspatialSnapModel`)
control the positioning of the items within the Container;
an application programmer can specify these resources so that the Container
strictly enforces the position of each item or so that the Container positions
items according to constraint resources specified for each item.
The user, however, can alter the position of an item in the`XmSPATIAL`layout type
within the Container by direct manipulation; for example, by pressing`BTransfer`over the item and then dragging and releasing`BTransfer`over some location within the Container.

In the`XmOUTLINE`layout type, the Container controls the positioning of
the items. Items within the Container can have parent-child
relationships between them. Each item's`XmNentryParent`resource can
specify another item in the same Container as its parent;
items with a non-NULL value
for`XmNentryParent`can only be displayed in the`XmOUTLINE`layout
type. In the`XmOUTLINE`layout type,
items are positioned in a tree configuration with connecting lines drawn
to illustrate the relationships.
Items are positioned top to bottom in the order specified by`XmNpositionIndex`within their parent.
Container positions a PushButton next to each item that has a parent
relationship to other items. The PushButton contains a pixmap to illustrate
whether the child items are shown or not; the user can activate the
PushButton to toggle between showing or hiding the child items.
Direct manipulation to alter the
position of the item is not supported in the`XmOUTLINE`layout
type.
Note that the`XmNtraversalOn`resource of the PushButtons created
by Container are set to False.

The`XmDETAIL`layout type is the same as`XmOUTLINE`, except that
each item can also display additional information as rows in
columns with column headers specified in the`XmNdetailColumnHeading`resources. In each item row, the item's detail information
(see the reference page onXmIconGadgetfor a description of the`XmNentryDetail`resource)
is displayed. Items are positioned top to bottom
in the order specified by`XmNpositionIndex`within the parent.
### Selection


When a child widget of the container is selected, the container
specifies that the item should display the appropriate visual
information to the user via the`ContainerItem`trait.
The application program is notified of selection
changes through`XmNselectionCallback`.

The container uses four selection policies:

Single

Browse

Multiple

Extended

In Single Select and Browse Select modes, only one item can be selected at a
time. Pressing`BSelect`on an item selects it and deselects any other selected item.
Pressing`BSelect`over an empty space in the Container deselects
all items. In Browse Select, dragging`BSelect`moves the selection
as the pointer is moved.

In Multiple Select and Extended Select modes, any number of items can be
selected at the same time.
In Multiple Select, pressing and dragging`BSelect`or`BExtend`to specify an item, range of items, or group of discontiguous
items
causes the selection states of those items to be toggled. In Extended Select,
pressing and dragging`BSelect`to indicate an item, range of items,
or group of discontiguous
items selects those items and deselects all others. Pressing
and dragging`BExtend`in Extended Select to indicate an item,
range of items, or discontiguous group of items causes the selection states
of those items to be toggled.

Several techniques are available to indicate an item, range of items, or
group of discontiguous items in the Multiple Select and Extended Select modes.

In the`XmSPATIAL`and`XmOUTLINE`layout types,
the`XmNselectionTechnique`resource specifies the techniques to be used
to indicate items. The default specification of`XmTOUCH_OVER`allows both the Random-Swipe and Marquee techniques to be used when`XmNlayoutStyle`is`XmSPATIAL`. The default specification of`XmTOUCH_OVER`allows the Range-Swipe, Range-Click, and Marquee techniques
to be used when`XmNlayoutStyle`is`XmOUTLINE`.

Discontiguous groups of items can be selected using the Random-Swipe
technique.
In the Random-Swipe
technique, pressing`BSelect`(or`BExtend`) over an item and dragging`BSelect`over other items selects all of those items. Only those
items that pointer passed over are selected.

In the Range-Swipe
technique, the user presses`BSelect`(or`BExtend`) over the first
item and releases`BSelect`over the last item; all items within the range
between the first and last item are selected whether the pointer actually
passed over them or not.
In the Range-Click technique, the user presses and releases`BSelect`(or`BExtend`) over the first item and then presses and
releases`BExtend`over the last item.

In the Marquee technique, pressing`BSelect`(or`BExtend`)
over a blank space within the
Container indicates the starting point of a Marquee rectangle. Dragging`BSelect`draws a Marquee rectangle (rubberband line) between the
starting point and current pointer. All items completely within the Marquee
rectangle are selected.

Specifying`XmTOUCH_ONLY`for`XmNselectionTechnique`enforces
the Random-Swipe technique even when`BSelect`(or`BExtend`)
is pressed over a blank space.
Similarly, specifying`XmMARQUEE`enforces the
Marquee technique even when`BSelect`(or`BExtend`)
is pressed over an item; since the item over which the press occurs is
only partially included in the Marquee rectangle, it is not selected.`XmMARQUEE_EXTEND_START`and`XmMARQUEE_EXTEND_BOTH`enforce the
Marquee technique and also cause the rectangle to extend automatically around
the first item indicated and, for`XmMARQUEE_EXTEND_BOTH`, the last item.

In the`XmDETAIL`layout type, the Range-Swipe and
Range-Click techniques
are available to indicate a range of items for selection.

Container uses the`XmQTcontainerItem`,`XmQTscrollFrame`, and`XmQTspecifyRenderTable`traits and holds the`XmQTcontainer`and`XmQTtransfer`traits.
### Data Transfer Behavior


Container supports dragging of selected items from the widget.
Depending on the value of`XmNprimaryOwnership`, Container can also
support primary selection.

As a source of data, Container supports the following targets and
associated conversions of data to these targets:

* **`locale`** 

If the`locale`target matches the widget's locale, the widget
transfers the selected items in the encoding of the locale.
The value for each item transferred, except the last, includes a trailing
separator.
Each item value is the`XmNlabelString`of the
item.
* **`COMPOUND_TEXT`** 

The widget transfers the selected items as type`COMPOUND_TEXT`.
The value for each item transferred, except the last, includes a trailing
separator.
Each item value is the`XmNlabelString`of the
item.
* **`DELETE`** 

The widget deletes the selected items.
* **`PIXMAP`** 

The widget transfers a list of the pixmap IDs of the selected items as
type`DRAWABLE`.
* **`STRING`** 

The widget transfers the selected items as type`STRING`.
The value for each item transferred, except the last, includes a trailing
separator.
Each item value is the`XmNlabelString`of the
item.
* **`TEXT`** 

If the selected items are fully convertible to the encoding of the
locale, the widget transfers the selected items in the encoding of the
locale.
Otherwise, the widget transfers the selected items as type`COMPOUND_TEXT`.
The value for each item transferred, except the last, includes a trailing
separator.
Each item value is the`XmNlabelString`of the
item.
* **_MOTIF_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets it
supports for immediate transfer for the`CLIPBOARD`selection.
These include_MOTIF_COMPOUND_STRINGand`PIXMAP`.
If the selected items are fully convertible to`STRING`, these also
include`STRING`; otherwise, they also include`COMPOUND_TEXT`.
* **_MOTIF_COMPOUND_STRING** 

The widget transfers the selected items as a compound string in
Byte Stream format.
The value for each item transferred, except the last, includes a trailing
separator.
Each item value is the`XmNlabelString`of the
item.
* **_MOTIF_DEFERRED_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets it
supports for delayed transfer for the`CLIPBOARD`selection.
This widget currently supplies no targets for_MOTIF_DEFERRED_CLIPBOARD_TARGETS.
* **_MOTIF_DRAG_OFFSET** 

The widget transfers a list of two 16-bit numbers, of type`INTEGER`, representing an x and y offset for an item being dragged.
This offset is calculated so that, if the offset were added to the x and
y coordinates at the drop site, and the dragged pixmap placed at that
position, it would correspond to the position the user would expect the
pixmap to placed at, based on the drag icon used at the drop site.
* **_MOTIF_EXPORT_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets to be
used as the value of the DragContext's`XmNexportTargets`in a
drag-and-drop transfer.
These include_MOTIF_COMPOUND_STRING,`PIXMAP`,`COMPOUND_TEXT`, the encoding of the locale,`STRING`,`TEXT`,`BACKGROUND`, and`FOREGROUND`.


As a source of data, Container also supports the following standard
Motif targets:

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
These also include_MOTIF_COMPOUND_STRING,`PIXMAP`,`COMPOUND_TEXT`, the encoding of the locale,`STRING`, and`TEXT`.
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


As a destination for data, Container supports only the dropping of items
being dragged from the same widget.
Subclasses and the`XmNdestinationCallback`procedures are
responsible for any other data transfers to the widget.
### Classes


Container inherits behavior, resources, and traits from the`Core`,`Composite`,`Constraint`,
and`XmManager`classes.

The class pointer is`xmContainerWidgetClass`.

The class name is`XmContainer`.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. The programmer can also set the resource
values for the inherited classes to set attributes for this widget.
To reference a resource by name or by class in a.Xdefaultsfile,
remove the`XmN`or`XmC`prefix and use the remaining letters.
To specify one of the defined values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use the remaining letters (in
either lowercase or uppercase, but include any underscores between
words). The codes in the access column indicate whether the given resource
can be set at creation time (C), set by using`XtSetValues`(S), retrieved by using`XtGetValues`(G), or is not
applicable (N/A).

`XmContainer Resource Set``Name``Class``Type``Default``Access`XmNautomaticSelectionXmCAutomaticSelectionunsigned charXmAUTO_SELECTCSGXmNcollapsedStatePixmapXmCCollapsedStatePixmapPixmapdynamicCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNdefaultActionCallbackXmCCallbackXtCallbackListNULLCXmNdestinationCallbackXmCCallbackXtCallbackListNULLCXmNdetailColumnHeadingXmCDetailColumnHeadingXmStringTableNULLCSGXmNdetailColumnHeadingCountXmCDetailColumnHeadingCountCardinal0CSGXmNdetailOrderXmCDetailOrderCardinal *NULLCSGXmNdetailOrderCountXmCDetailOrderCountCardinaldynamicCSGXmNdetailTabListXmCDetailTabListXmTabListNULLCSGXmNentryViewTypeXmCEntryViewTypeunsigned charXmANY_ICONCSGXmNexpandedStatePixmapXmCExpandedStatePixmapPixmapdynamicCSGXmNfontListXmCFontListXmFontListNULLCSGXmNlargeCellHeightXmCCellHeightDimensiondynamicCSGXmNlargeCellWidthXmCCellWidthDimensiondynamicCSGXmNlayoutTypeXmCLayoutTypeunsigned charXmSPATIALCSGXmNmarginHeightXmCMarginHeightDimension0CSGXmNmarginWidthXmCMarginWidthDimension0CSGXmNoutlineButtonPolicyXmCOutlineButtonPolicyunsigned charXmOUTLINE_BUTTON_PRESENTCSGXmNoutlineChangedCallbackXmCCallbackXtCallbackListNULLCXmNoutlineColumnWidthXmCOutlineColumnWidthDimensiondynamicCSGXmNoutlineIndentationXmCOutlineIndentationDimension40CSGXmNoutlineLineStyleXmCLineStyleunsigned charXmSINGLECSGXmNprimaryOwnershipXmCprimaryOwnershipunsigned charXmOWN_POSSIBLE_MULTIPLECSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNselectColorXmCSelectColorPixeldynamicCSGXmNselectedObjectsXmCSelectedObjectsWidgetListNULLSGXmNselectedObjectCountXmCSelectedObjectCountunsigned int0SGXmNselectionCallbackXmCCallbackXtCallbackListNULLCXmNselectionPolicyXmCSelectionPolicyunsigned charXmEXTENDED_SELECTCSGXmNselectionTechniqueXmCSelectionTechniqueunsigned charXmTOUCH_OVERCSGXmNsmallCellHeightXmCCellHeightDimensiondynamicCSGXmNsmallCellWidthXmCCellWidthDimensiondynamicCSGXmNspatialIncludeModelXmCSpatialIncludeModelunsigned charXmAPPENDCSGXmNspatialResizeModelXmCSpatialResizeModelunsigned charXmGROW_MINORCSGXmNspatialSnapModelXmCSpatialSnapModelunsigned charXmNONECSGXmNspatialStyleXmCSpatialStyleunsigned charXmGRIDCSG

* **`XmNautomaticSelection`** 

Indicates whether the Container invokes selection callbacks when each
item is selected (or toggled) or whether selection callbacks are not
invoked until the user has completed selection actions (for example,
the user has released the mouse button). It can have one of the following
values:

* **`XmAUTO_SELECT`** 

Makes selection callbacks automatically when each
item is selected or toggled. This may also be the value`TRUE`.
* **`XmNO_AUTO_SELECT`** 

Delays selection callbacks until the
user has finished selection actions. This may also be the value`FALSE`.

* **`XmNcollapsedStatePixmap`** 

Specifies the pixmap to display on a PushButton next to a Container item
with child items, when`XmNoutlineButtonPolicy`is`XmOUTLINE_BUTTON_PRESENT`.`XmNcollapsedStatePixmap`indicates that the child items are not displayed.
If set to`XmUNSPECIFIED_PIXMAP`, a default pixmap showing an
arrow pointing up is used.
* **`XmNconvertCallback`** 

Specifies a list of callbacks called when the Container is asked to
convert a selection.
The type of the structure whose address is passed to these callbacks isXmConvertCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNdefaultActionCallback`** 

Specifies a list of callbacks to call when the user double-clicks
an item or pressesEnterorReturnover an item.
The callback structure isXmContainerSelectCallbackStruct.
The reason is`XmCR_DEFAULT_ACTION`.
* **`XmNdestinationCallback`** 

Specifies a list of callbacks called when the Container is the
destination of a transfer operation.
The type of the structure whose address is passed to these callbacks isXmDestinationCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNdetailColumnHeading`** 

Specifies a table ofXmStrings to display as the headings to columns.
If NULL, or if`XmNlayoutType`is not`XmDETAIL`, no heading is displayed.
* **`XmNdetailColumnHeadingCount`** 

Specifies a count ofXmStrings in the table specified for`XmNdetailColumnHeading`.
* **`XmNdetailOrder`** 

Specifies an array ofCardinals that indicate which column detail information,
and in which order, each Container child will display its detail information.
This resource is ignored if`XmNlayoutType`is not`XmDETAIL`.
If NULL, the the default behavior is determined by`XmNdetailOrderCount`.
* **`XmNdetailOrderCount`** 

Specifies a count ofCardinals in the array specified for`XmNdetailOrder`.
If`XmNdetailOrder`is NULL and`XmNdetailOrderCount`is not 0, then
each Container child displays its detail information in order from column
1 to the`XmNdetailOrderCount`column number. If`XmNdetailOrderCount`is 0, then a default is calculated from the detail order count
information of each item accessed via the`ContainerItem`trait.
* **`XmNdetailTabList`** 

Indicates anXmTabListspecifying the start
of each column in the`XmDETAIL`layout.
If this resource is set to NULL, then Container calculates anXmTabList.
This resource is ignored if`XmNlayoutType`is not`XmDETAIL`.
* **`XmNentryViewType`** 

Specifies the view type for all Container children. The view type is
specified for each item via the`ContainerItem`trait.
It can have one of the following values:

* **`XmANY_ICON`** 

No specification is made for Container children. Children use their own
default specifications.
* **`XmLARGE_ICON`** 

The view type for all children is`XmLARGE_ICON`.
* **`XmSMALL_ICON`** 

The view type for all children is`XmSMALL_ICON`.

* **`XmNexpandedStatePixmap`** 

Specifies the pixmap to display on a PushButton next to a Container item
with child items, when`XmNoutlineButtonPolicy`is`XmOUTLINE_BUTTON_PRESENT`.`XmNexpandedStatePixmap`indicates that the child items are displayed.
If set to`XmUNSPECIFIED_PIXMAP`, a default pixmap showing
an arrow pointing down is used.
* **`XmNfontList`** 

Specifies the fontlist associated with`XmContainer`. The fontlist
is an obsolete construct and has been superseded by the render table.
It is included for compatibility with earlier versions of Motif, and
for applications that do not easily support render tables. The
default fontlist is derived from the default render table; and if
both a fontlist and a render table are specified, the render table
takes precedence.
* **`XmNlargeCellHeight`** 

Specifies the height of a cell for`XmGRID`or`XmCELLS`spatial style when`XmNentryViewType`is`XmLARGE_ICON`or`XmANY_ICON`.
* **`XmNlargeCellWidth`** 

Specifies the width of a cell for`XmGRID`or`XmCELLS`spatial style when`XmNentryViewType`is`XmLARGE_ICON`or`XmANY_ICON`.
* **`XmNlayoutType`** 

Specifies the policy for laying out child widgets within the Container.
It can have one of the following values:

* **`XmDETAIL`** 

Displays items in the same manner as when the resource is`XmOUTLINE`,
except that each item displays detail information next to it.
* **`XmOUTLINE`** 

Displays items in a tree configuration,
in`XmNpositionIndex`within`XmNentryParent`order, with connecting
lines drawn to show their parent-child relationships.
* **`XmSPATIAL`** 

Displays items according to`XmNspatialStyle`.
Items with`XmNentryParent`values are not displayed.

* **`XmNmarginHeight`** 

Specifies the margin spacing at the top and bottom of the Container.
* **`XmNmarginWidth`** 

Specifies the margin spacing at the left and right sides of the Container.
* **`XmNoutlineButtonPolicy`** 

Specifies whether or not to display buttons for users to expand and collapse
the display of items.
It can have one of the following values:

* **`XmOUTLINE_BUTTON_ABSENT`** 

Do not display the outline buttons.
* **`XmOUTLINE_BUTTON_PRESENT`** 

Display the outline buttons.

* **`XmNoutlineChangedCallback`** 

Specifies a list of callbacks to call when an item's`XmNoutlineState`is changed.
The callback structure isXmContainerOutlineCallbackStruct.
The reason is`XmCR_COLLAPSED`or`XmCR_EXPANDED`,
depending on the new value of`XmNoutlineState`.
* **`XmNoutlineColumnWidth`** 

Specifies the width of the first column displayed when`XmNlayoutType`is`XmDETAIL`. Specifies the preferred width of the Container (without the
margins) when`XmNlayoutType`is`XmOUTLINE`.
If not specified, Container will determine a default value equal to the
widest space necessary to display an item's pixmap and`XmNoutlineIndentation`.
* **`XmNoutlineIndentation`** 

Specifies the distance to indent for the display of
child items when`XmNlayoutType`is`XmOUTLINE`or`XmDETAIL`.
* **`XmNoutlineLineStyle`** 

Specifies whether to draw lines between items with parent-child
relationships when`XmNlayoutType`is`XmOUTLINE`or`XmDETAIL`.
It can have one of the following values:

* **`XmNO_LINE`** 

Draws no line.
* **`XmSINGLE`** 

Draws a line one pixel wide.

* **`XmNprimaryOwnership`** 

Specifies whether Container takes ownership of the primary selection when a
selection is made inside it. This resource can take the following values:

* **`XmOWN_NEVER`** 

Never takes ownership.
* **`XmOWN_ALWAYS`** 

Always takes ownership.
* **`XmOWN_MULTIPLE`** 

Only takes ownership if more than one element has been
selected.
* **`XmOWN_POSSIBLE_MULTIPLE`** 

Only takes ownership if more than one element
can be selected at a time.

* **`XmNrenderTable`** 

Specifies theXmRenderTablethat is inherited by all children
of the Container.
The default is implementation dependent.
If both a render table and a fontlist are specified, the render table
will take precedence.
* **`XmNselectColor`** 

Specifies a Pixel that can be accessed by children of the Container and
used to indicate that the child is in a selected state. In addition to
a Pixel value, the following symbolic values can be specified:

* **`XmDEFAULT_SELECT_COLOR`** 

Specifies a color between the background and the bottom shadow color.
* **`XmREVERSED_GROUND_COLORS`** 

Forces the select color to the foreground color and causes the default color
of any text rendered over the select color to be the background color.
* **`HIGHLIGHT_COLOR`** 

Forces the fill color to use the highlight color.

* **`XmNselectedObjectCount`** 

Specifies the number of widgets in the selected items list.
The value must be the number of items in`XmNselectedObjects`.
* **`XmNselectedObjects`** 

An array of widgets that represents the Container items that
are currently selected, either by the user or by the application.

If the application sets`XmNselectedObjects`to an array of widgets,
those array elements that are valid Container items are selected.
* **`XmNselectionCallback`** 

Specifies a list of callbacks to call when an item is selected.
The callback structure isXmContainerSelectCallbackStruct.
The reason is`XmCR_SINGLE_SELECT`,`XmCR_BROWSE_SELECT`,`XmCR_MULTIPLE_SELECT`, or`XmCR_EXTENDED_MULTIPLE`, depending on`XmNselectionPolicy`.
* **`XmNselectionPolicy`** 

Defines the interpretation of the selection action. This can be one of the
following values:

* **`XmSINGLE_SELECT`** 

Allows only single selections.
* **`XmBROWSE_SELECT`** 

Allows "drag and browse" selections.
* **`XmMULTIPLE_SELECT`** 

Allows multiple selections.
* **`XmEXTENDED_SELECT`** 

Allows extended selections.

* **`XmNselectionTechnique`** 

Specifies the selection technique to use when the Container displays
items in a 2-dimensional layout
(`XmNentryViewType`is`XmLARGE_ICON`or`XmSMALL_ICON`).
In the`XmDETAIL`layout, the`XmNselectionTechnique`resource is treated as`XmTOUCH_ONLY`. In either case,
it can have one of the following
values:

* **`XmMARQUEE`** 

Uses the Marquee technique only.
* **`XmMARQUEE_EXTEND_START`** 

Uses the Marquee technique only and extends
the Marquee rectangle around any item under the Marquee start point.
* **`XmMARQUEE_EXTEND_BOTH`** 

Uses the Marquee technique only and extends
the Marquee rectangle around any items under the Marquee start and end points.
* **`XmTOUCH_ONLY`** 

Uses the Random-Swipe technique only if`XmNlayoutType`is`XmSPATIAL`.
Otherwise, uses the Range-Swipe and Range-Click techniques.
* **`XmTOUCH_OVER`** 

If the selection action begins over an item
and`XmNlayoutType`is`XmSPATIAL`, uses
the Random-Swipe technique.
If the selection action begins over an item
and`XmNlayoutType`is`XmOUTLINE`or`XmDETAIL`, uses
the Range-Swipe and Range-Click techniques.
Uses the Marquee technique if the select action
begins over an unoccupied area in the Container.

* **`XmNsmallCellHeight`** 

Specifies the height of a cell for`XmGRID`or`XmCELLS`spatial style when`XmNentryViewType`is`XmSMALL_ICON`.
* **`XmNsmallCellWidth`** 

Specifies the width of a cell for`XmGRID`or`XmCELLS`spatial style when`XmNentryViewType`is`XmSMALL_ICON`.
* **`XmNspatialIncludeModel`** 

Specifies the layout of an item when the item is managed
in the Container when`XmNlayoutType`is`XmSPATIAL`and`XmNspatialStyle`is`XmGRID`or`XmCELLS`.
It can have one of the following values:

* **`XmAPPEND`** 

Places the item after the last occupied cell according to`XmNlayoutDirection`.
* **`XmCLOSEST`** 

Places the item in the free cell closest to the position
specified by`XmNx`and`XmNy`.
* **`XmFIRST_FIT`** 

Places the item in the first free cell
according to`XmNlayoutDirection`.

* **`XmNspatialResizeModel`** 

Specifies how Container will attempt to grow its dimensions
when`XmNlayoutType`is`XmSPATIAL`and`XmNspatialStyle`is`XmGRID`or`XmCELLS`and there are not enough cells to contain a new Container item.
It can have one of the following values:

* **`XmGROW_BALANCED`** 

Container will request both width and height growth from its parent.
* **`XmGROW_MAJOR`** 

Container will request growth in its major dimension from its parent.
Container's major dimension is width when the precedence of`XmNlayoutDirection`is horizontal, and height when vertical.
* **`XmGROW_MINOR`** 

Container will request growth in its minor dimension from its parent.
Container's minor dimension is height when the precedence of`XmNlayoutDirection`is horizontal, and width when vertical.

* **`XmNspatialSnapModel`** 

Specifies how Container will position an item within the cell layout
when`XmNlayoutType`is`XmSPATIAL`and`XmNspatialStyle`is`XmGRID`or`XmCELLS`.
It can have one of the following values:

* **`XmCENTER`** 

Center the items as follows, depending on the value of`XmNentryViewType`:

* **`XmLARGE_ICON`** 

The child is centered in
the cell horizontally and baseline-aligned vertically.
* **`XmSMALL_ICON`** 

The child is centered in the cell vertically on its baseline and aligned
with the left or right of the cell horizontally, depending on
the value of`XmNlayoutDirection`.

* **`XmSNAP_TO_GRID`** 

Position the item at the upper-left or upper-right corner of the cell(s),
depending on the value of`XmNlayoutDirection`.
* **`XmNONE`** 

Position the item according to the position
specified by`XmNx`and`XmNy`.
If the position is not within the coordinates of the cell(s),
then position the item at the upper-left or upper-right corner of the cell(s),
depending on the value of`XmNlayoutDirection`.

* **`XmNspatialStyle`** 

Specifies the layout of Container items when`XmNlayoutType`is`XmSPATIAL`. It can have one of the following values:

* **`XmCELLS`** 

Lays out items within a grid of same-size cells.
Each item occupies as many cells as required to contain the item dimensions.
* **`XmGRID`** 

Lays out items within a grid of same-size cells.
Each item occupies only one cell. Items that are larger than the cell
size may overlap other items.
* **`XmNONE`** 

Lays out items according to`XmNx`and`XmNy`.



`XmContainer Constraint Resource Set``Name``Class``Type``Default``Access`XmNentryParentXmCWidgetWidgetNULLCSGXmNoutlineStateXmCOutlineStateunsigned charXmCOLLAPSEDCSGXmNpositionIndexXmCPositionIndexintdynamicCSG

* **`XmNentryParent`** 

Specifies the widget that is this item's logical parent.
A value of NULL indicates that this is a root-level item. Parent-child
information is displayed only when the`XmNlayoutPolicy`is`XmOUTLINE`or`XmDETAIL`.
* **`XmNoutlineState`** 

Specifies whether to display child items when`XmNlayoutPolicy`is`XmOUTLINE`or`XmDETAIL`.
It can have one of the following values:

* **`XmCOLLAPSED`** 

Does not display child items.
* **`XmEXPANDED`** 

Displays child items.

* **`XmNpositionIndex`** 

Specifies the order of items in the Container for display.
When`XmNlayoutType`is`XmOUTLINE`or`XmDETAIL`, items are
displayed in`XmNpositionIndex`order within`XmNentryParent`.
Items that
have an`XmNentryParent`resource are ignored when`XmNlayoutType`is`XmSPATIAL`.
If`XmNpositionIndex`is not specified, it defaults to the`XmNpositionIndex`value plus 1 of the item with the highest`XmNpositionIndex`that has the same`XmNentryParent`if such
an item exists; otherwise, it defaults to 0.

### Inherited Resources


Container inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to callbacks for`XmNoutlineChangedCallback`.typedef struct
{
        int`reason`;
        XEvent`* event`;
        Widget`item`;
        unsigned char`new_outline_state`;
} XmContainerOutlineCallbackStruct;

* **`reason`** 

Specifies the reason for the callback.
* **`event`** 

Points to the`XEvent`that triggered the callback. It can be NULL.
* **`item`** 

Specifies the container item affected by the event.
* **`new_outline_state`** 

Specifies the next`XmNoutlineState`for`item`. The user can
change this value in the callback.


A pointer to the following structure is passed to callbacks for`XmNdefaultActionCallback`and`XmNselectionCallback`.typedef struct
{
        int`reason`;
        XEvent`* event`;
        WidgetList`selected_items`;
        int`selected_item_count`;
        unsigned char`auto_selection_type`;
} XmContainerSelectCallbackStruct;

* **`reason`** 

Specifies the reason for the callback. It corresponds to the`XmNselectionPolicy`at the time the selection was made, or indicates
that the default action should be taken.
* **`event`** 

Points to the`XEvent`that triggered the callback. It can be NULL.
* **`selected_items`** 

Specifies a list of items selected at the time of the`event`that
caused the callback.
The`selected_items`field points to a temporary storage space
that is reused
after the callback is finished.
Therefore, if an application needs to save the selected list, it should
copy the list into its own data space.
* **`selected_item_count`** 

Specifies the number of items in the`selected_items`list.
This number must be positive or 0 (zero).
* **`auto_selection_type`** 

Indicates the cause of the selection when`XmNautomaticSelection`is`XmAUTO_SELECT`. Valid values are the following:

* **`XmAUTO_UNSET`** 

Returned when`XmNautomaticSelection`is`XmNO_AUTO_SELECT`.
* **`XmAUTO_BEGIN`** 

Indicates the beginning of automatic selection.
* **`XmAUTO_MOTION`** 

Indicates that there is a button drag selection.
* **`XmAUTO_CANCEL`** 

Indicates that the new selection is canceled.
* **`XmAUTO_NO_CHANGE`** 

Indicates that the currently selected item matches the initial item.
* **`XmAUTO_CHANGE`** 

Indicates that the currently selected item does not match the initial item.



A pointer to the following structure is passed to the`XmNconvertCallback`procedures:typedef struct
{
        int`reason`;
        XEvent  *`event`;
        Atom`selection`;
        Atom`target`;
        XtPointer`source_data`;
        XtPointer`location_data`;
        int`flags`;
        XtPointer`parm`;
        int`parm_format`;
        unsigned long`parm_length`;
        int`status`;
        XtPointer`value`;
        Atom`type`;
        int`format`;
        unsigned long`length`;
} XmConvertCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL.
* **`selection`** 

Indicates the selection for which conversion is being requested.
Possible values are`CLIPBOARD`,`PRIMARY`,`SECONDARY`,
and_MOTIF_DROP.
* **`target`** 

Indicates the conversion target.
* **`source_data`** 

Contains information about the selection source.
When the selection is_MOTIF_DROP,`source_data`is the
DragContext.
Otherwise, it is NULL.
* **`location_data`** 

Contains information about the location of data to be converted.
If the value is NULL, the data to be transferred consists of the
widget's current selection.
Otherwise, it is the widget ID of the item being transferred, or the
widget ID of the Container if all items are being transferred.
* **`flags`** 

Indicates the status of the conversion. Following are the possible
values:

* **`XmCONVERTING_NONE`** 

This flag is currently unused.
* **`XmCONVERTING_PARTIAL`** 

The target widget was able to be converted, but some data was lost.
* **`XmCONVERTING_SAME`** 

The conversion target is the source of the data to be transferred.
* **`XmCONVERTING_TRANSACT`** 

This flag is currently unused.

* **`parm`** 

Contains parameter data for this target.
If no parameter data exists, the value is NULL.

When`selection`is`CLIPBOARD`and`target`is_MOTIF_CLIPBOARD_TARGETSor_MOTIF_DEFERRED_CLIPBOARD_TARGETS, the value is the requested
operation (`XmCOPY`,`XmMOVE`, or`XmLINK`).
* **`parm_format`** 

Specifies whether the data in`parm`should be viewed
as a list of`char`,`short`, or`long`quantities.
Possible values are 0 (when`parm`is NULL),
8 (when the data in`parm`should be viewed as a list of`char`s),
16 (when the data in`parm`should be viewed as a list of`short`s),
or 32 (when the data in`parm`should be viewed as a list of`long`s).
Note that`parm_format`symbolizes a data type, not the number of bits
in each list element.
For example, on some machines, a`parm_format`of 32 means that
the data in`parm`should be viewed as a list of 64-bit quantities,
not 32-bit quantities.
* **`parm_length`** 

Specifies the number of elements of data in`parm`, where each
element has the size specified by`parm_format`.
When`parm`is NULL, the value is 0.
* **`status`** 

An IN/OUT member that specifies the status of the conversion.
The initial value is`XmCONVERT_DEFAULT`.
The callback procedure can set this member to one of the following
values:

* **`XmCONVERT_DEFAULT`** 

The widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it
overwrites the data provided by the callback procedures in the`value`member.
* **`XmCONVERT_MERGE`** 

The widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it appends
its data to the data provided by the callback procedures in the`value`member.
This value is intended for use with targets that result in lists of
data, such as`TARGETS`.
* **`XmCONVERT_DONE`** 

The callback procedure has successfully finished
the conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.
* **`XmCONVERT_REFUSE`** 

The callback procedure has terminated the
conversion process without completing the requested conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.

* **`value`** 

An IN/OUT parameter that contains any data that the callback procedure
produces as a result of the conversion.
The initial value is NULL.
If the callback procedure sets this member, it must ensure that the`type`,`format`, and`length`members correspond
to the data in`value`.
The callback procedure is responsible for allocating, but not for
freeing, memory when it sets this member.
* **`type`** 

An IN/OUT parameter that indicates the type of the data in the`value`member.
The initial value is`INTEGER`.
* **`format`** 

An IN/OUT parameter that specifies whether the data in`value`should
be viewed as a list of`char`,`short`, or`long`quantities.
The initial value is 8.
The callback procedure can set this member to 8 (for a list of`char`),
16 (for a list of`short`), or 32 (for a list of`long`).
* **`length`** 

An IN/OUT member that specifies the number of elements of data in`value`, where each element has the size symbolized by`format`.
The initial value is 0 (zero).


A pointer to the following callback structure is passed to the`XmNdestinationCallback`procedures:typedef struct
{
        int`reason`;
        XEvent  *`event`;
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
Otherwise, the value is a pointer to an`XPoint`structure
containing the x and y coordinates at the location where the data is to
be transferred.
Once`XmTransferDone`procedures start to be called,`location_data`will no longer be stable.
* **`time`** 

Indicates the time when the transfer operation began.

### Translations


The`XmContainer`translations are listed below.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`&ap;c &ap;s &ap;m &ap;a`Btn1Down:** 

ContainerBeginSelect()
* **Btn1Motion:** 

ContainerButtonMotion()
* **`&ap;c &ap;s &ap;m &ap;a`Btn1Up:** 

ContainerEndSelect()
* **`c &ap;s &ap;m &ap;a`Btn1Down:** 

ContainerBeginToggle()
* **`c &ap;s &ap;m &ap;a`Btn1Up:** 

ContainerEndToggle()
* **`&ap;c s &ap;m &ap;a`Btn1Down:** 

ContainerBeginExtend()
* **`&ap;c s &ap;m &ap;a`Btn1Up:** 

ContainerEndExtend()
* **`c s &ap;m &ap;a`Btn1Down:** 

ContainerBeginExtend()
* **`c s &ap;m &ap;a`Btn1Up:** 

ContainerEndExtend()
* **`&ap;c &ap;s &ap;m &ap;a`Btn2Down:** 

ContainerStartTransfer(`Copy`)
* **`c s &ap;m &ap;a`Btn2Down:** 

ContainerStartTransfer(`Link`)
* **`&ap;c s &ap;m &ap;a`Btn2Down:** 

ContainerStartTransfer(`Move`)
* **`&ap;m &ap;a`Btn2Up:** 

ContainerEndTransfer()
* **`:c s a`KeyosfInsert:** 

ContainerPrimaryLink()
* **`:c s m`KeyosfInsert:** 

ContainerPrimaryLink()
* **`:a`KeyosfInsert:** 

ContainerPrimaryCopy()
* **`:m`KeyosfInsert:** 

ContainerPrimaryCopy()
* **`:s a`KeyosfDelete:** 

ContainerPrimaryMove()
* **`:s m`KeyosfDelete:** 

ContainerPrimaryMove()
* **`:`KeyosfCancel:** 

ContainerCancel()
* **`:s`KeyosfSelect:** 

ContainerExtend()
* **`:`KeyosfSelect:** 

ContainerSelect()
* **`:`KeyosfSelectAll:** 

ContainerSelectAll()
* **`:`KeyosfDeselectAll:** 

ContainerDeselectAll()
* **`:`KeyosfAddMode:** 

ContainerToggleMode()
* **`:`KeyosfActivate:** 

ContainerActivate()
* **`s &ap;c &ap;m &ap;a`Key`space`:** 

ContainerExtend()
* **`&ap;s &ap;c &ap;m &ap;a`Key`space`:** 

ContainerSelect()
* **`&ap;s &ap;c &ap;m &ap;a`Key`Return`:** 

ContainerActivate()
* **`&ap;s c &ap;m &ap;a`Key`slash`:** 

ContainerSelectAll()
* **`&ap;s c &ap;m &ap;a`Key`backslash`:** 

ContainerDeselectAll()
* **`:c s`KeyosfBeginLine:** 

ContainerExtendCursor(`First`)
* **`:c s`KeyosfEndLine:** 

ContainerExtendCursor(`Last`)
* **`:c`KeyosfBeginLine:** 

ContainerMoveCursor(`First`)
* **`:c`KeyosfEndLine:** 

ContainerMoveCursor(`Last`)
* **`:c`KeyosfLeft:** 

ContainerExpandOrCollapse(`Left`)
* **`:c`KeyosfRight:** 

ContainerExpandOrCollapse(`Right`)
* **`:s`KeyosfUp:** 

ContainerExtendCursor(`Up`)
* **`:s`KeyosfDown:** 

ContainerExtendCursor(`Down`)
* **`:s`KeyosfLeft:** 

ContainerExtendCursor(`Left`)
* **`:s`KeyosfRight:** 

ContainerExtendCursor(`Right`)
* **`:`KeyosfUp:** 

ContainerMoveCursor(`Up`)
* **`:`KeyosfDown:** 

ContainerMoveCursor(`Down`)
* **`:`KeyosfLeft:** 

ContainerMoveCursor(`Left`)
* **`:`KeyosfRight:** 

ContainerMoveCursor(`Right`)
* **`s &ap;m &ap;a`Key`Tab`:** 

ManagerGadgetPrevTabGroup()
* **`&ap;s &ap;m &ap;a`Key`Tab`:** 

ManagerGadgetNextTabGroup()


The Container button event translations are modified when Display's`XmNenableBtn1Transfer`resource does not have a value of`XmOFF`(in other words, it is either`XmBUTTON2_TRANSFER`or`XmBUTTON2_ADJUST`). This
option allows the
actions for selection and transfer to be integrated onBtn1, and
the actions for extending the selection can be bound toBtn2. The actions forBtn1that are defined in the
preceding list
still apply when theBtn1event occurs over text that is not
selected. The following actions apply when theBtn1event
occurs over text that is selected:

* **`~c ~s ~m ~a`Btn1Down`:`** 

ContainerHandleBtn1Down(`ContainerBeginSelect,Copy`)
* **`c ~s ~m ~a <Btn1Down>:`** 

ContainerHandleBtn1Down(`ContainerBeginToggle,Copy`)
* **`c s ~m ~a <Btn1Down>:`** 

ContainerHandleBtn1Down(`ContainerNoop,Link`)
* **`~c s ~m ~a <Btn1Down>:`** 

ContainerHandleBtn1Down(`ContainerBeginExtend,Move`)
* **Btn1Motion`:`** 

ContainerHandleBtn1Motion(`ContainerButtonMotion`)
* **`~c ~s ~m ~a <Btn1Up>:`** 

ContainerHandleBtn1Up(`ContainerEndSelect`)
* **`c ~s ~m ~a <Btn1Up>:`** 

ContainerHandleBtn1Up(`ContainerEndToggle`)
* **`~c s ~m ~a <Btn1Up>:`** 

ContainerHandleBtn1Up(`ContainerEndExtend`)
* **`c s ~m ~a <Btn1Down>:`** 

ContainerHandleBtn1Down(`ContainerBeginExtend`)
* **`c s ~m ~a <Btn1Up>:`** 

ContainerHandleBtn1Up(`ContainerEndExtend`)


When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_ADJUST`, the following actions apply:

* **`~c ~s ~m ~a <Btn2Down>:`** 

ContainerHandleBtn2Down(`ContainerStartTransfer,Copy`)
* **`c s ~m ~a <Btn2Down>:`** 

ContainerHandleBtn2Down(`ContainerStartTransfer,Link`)
* **`~c s ~m ~a <Btn2Down>:`** 

ContainerHandleBtn2Down(`ContainerStartTransfer,Move`)
* **Btn2Motion`:`** 

ContainerHandleBtn2Motion(`ContainerButtonMotion`)
* **`~m ~a <Btn2Up>:`** 

ContainerHandleBtn2Up(`ContainerEndTransfer`)

### Action Routines


The Container action routines are described below.
The current selections are always shown
with the background color specified by the`XmNselectColor`resource.

* **ContainerActivate():** 

This action calls`XmNdefaultActionCallback`with reason`XmCR_DEFAULT_ACTION`.
* **ContainerBeginExtend():** 

Simply returns if`XmNselectionPolicy`is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`.
Simply returns if`XmNlayoutType`is`XmSPATIAL`.

Otherwise, this action sets the selection state of all items between
the anchor item and the item under the pointer to
the selection state of the anchor item.
The location cursor is moved to the item under the pointer.
If`XmNautomaticSelection`is`XmAUTO_SELECT`, the`XmNselectionCallback`(s) is called with either`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`as the reason depending on`XmNselectionPolicy`, and with`auto_selection_type``XmAUTO_CHANGE`.
* **ContainerBeginSelect():** 

If this is a secondContainerBeginSelect()action that has occurred
within the time specified by the display's multiclick time, this action calls`XmNdefaultActionCallback`with reason`XmCR_DEFAULT_ACTION`and returns.

Otherwise, processing depends on the value of`XmNselectionPolicy`as follows:

* **`XmSINGLE_SELECT`** 

This action deselects all items and toggles the item (if any) under the pointer.
* **`XmBROWSE_SELECT`** 

This action deselects all items and toggles the item (if any) under the pointer.
This item is now the anchor item for further selection.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and a change in any
item's selection state is made, the`XmNselectionCallback`(s)
is called with reason`XmCR_BROWSE_SELECT`and`auto_selection_type``XmAUTO_BEGIN`.
* **`XmMULTIPLE_SELECT`** 

If the pointer is over an item and`XmNselectionTechnique`is not`XmMARQUEE`, this action toggles the selection state of that item.
The item becomes the anchor item for further selection.
If`XmNselectionTechnique`is`XmMARQUEE`,`XmMARQUEE_EXTEND_START`,
or`XmMARQUEE_EXTEND_BOTH`, this action sets the start point for the
Marquee rectangle.
If`XmNselectionTechnique`is`XmMARQUEE_EXTEND_START`or`XmMARQUEE_EXTEND_BOTH`and the pointer is over an item, this action
draws the Marquee rectangle around the item.
If`XmNautomaticSelection`is`XmAUTO_SELECT`, the`XmNselectionCallback`(s)
is called with reason`XmCR_MULTIPLE_SELECT`and`auto_selection_type``XmAUTO_BEGIN`.
* **`XmEXTENDED_SELECT`** 

All items are first deselected. Processing is then identical to the
case where`XmNselectionPolicy`is`XmMULTIPLE_SELECT`, except that`XmCR_EXTENDED_SELECT`is the callback reason given if`XmNselectionCallback`is called.

* **ContainerBeginToggle():** 

Simply returns if`XmNselectionPolicy`is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`.

Otherwise, if the pointer is over an item and`XmNselectionTechnique`is not`XmMARQUEE`, this action toggles the selection state of that item.
The item becomes the anchor item for further selection.
If`XmNselectionTechnique`is`XmMARQUEE`,`XmMARQUEE_EXTEND_START`,
or`XmMARQUEE_EXTEND_BOTH`this action sets the start point for the
Marquee rectangle.
If`XmNselectionTechnique`is`XmMARQUEE_EXTEND_START`or`XmMARQUEE_EXTEND_BOTH`and the pointer is over an item, this action
draws the Marquee rectangle around the item.
If`XmNautomaticSelection`is`XmAUTO_SELECT`, the`XmNselectionCallback`(s) is called with either`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`as the reason, depending on`XmNselectionPolicy`, and with`auto_selection_type``XmAUTO_BEGIN`.
* **ContainerButtonMotion():** 

Processing depends on the value of`XmNselectionPolicy`, as follows:

* **`XmSINGLE_SELECT`** 

This action simply returns to the caller.
* **`XmBROWSE_SELECT`** 

Simply returns if this action follows aContainerBeginExtend()action orContainerBeginToggle()action.

If the pointer is no longer over the current anchor item, this action
toggles the current anchor item and then toggles the item under the pointer
(if any) and makes it the new anchor item for further processing.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and a change in any item's
selection state is made, the`XmNselectionCallback`(s)
is called with reason`XmCR_BROWSE_SELECT`and`auto_selection_type``XmAUTO_MOTION`.
* **`XmMULTIPLE_SELECT`** 

If a previous action has set a Marquee rectangle start point,
this action draws the Marquee rectangle
between the current pointer position and the Marquee start point. If the`XmNselectionTechnique`is`XmMARQUEE_EXTEND_BOTH`and the pointer is
over an item, the end point of the Marquee rectangle is extended to include
the item. The selection states of all items within the Marquee rectangle are
toggled to match the state of the anchor item.

If no Marquee rectangle start point is set and the pointer is over an
item, processing depends on the`XmNlayoutType`resource.
The anchor item from the previous action is used. If`XmNlayoutType`is`XmSPATIAL`,
the selection state of the item
under the pointer is toggled to match the selection state of the anchor item.
If`XmNlayoutType`is`XmOUTLINE`or`XmDETAIL`, the selection state of all items between the anchor item
and the item under the pointer are toggled to match the selection state of
the anchor item.

If`XmNautomaticSelection`is`XmAUTO_SELECT`and a change in any
item's selection state is made, the`XmNselectionCallback`(s) is called with reason`XmCR_MULTIPLE_SELECT`and`auto_selection_type``XmAUTO_MOTION`.
* **`XmEXTENDED_SELECT`** 

Processing is identical to the
case where`XmNselectionPolicy`is`XmMULTIPLE_SELECT`, except that`XmCR_EXTENDED_SELECT`is the callback reason given if`XmNselectionCallback`is called.

* **ContainerCancel():** 

If a selection is in progress, this action
restores selection states of all items
to their state before the selection began.
If`XmNautomaticSelection`is True and a change in any item's
selection state is made, the`XmNselectionCallback`is called with reason`XmCR_BROWSE_SELECT`,`XmMULTIPLE_SELECT`,
or`XmCR_EXTENDED_SELECT`depending on the`XmNselectionPolicy`resource and`auto_selection_type``XmAUTO_CANCEL`.
* **ContainerDeselectAll():** 

This action deselects all items and calls`XmNselectionCallback`with
reason depending on`XmNselectionPolicy`.
* **ContainerEndExtend():** 

Simply returns if`XmNselectionPolicy`is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`.
Simply returns if`XmNlayoutType`is`XmSPATIAL`.

Otherwise,
if`XmNautomaticSelection`is`XmNO_AUTO_SELECT`,`XmNselectionCallback`(s) is called with either`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`as the reason depending on`XmNselectionPolicy`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and no change
is made in any item's selection state by this action,`XmNselectionCallback`(s)
is called with either`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`as the reason depending on`XmNselectionPolicy`and`auto_selection_type``XmAUTO_CHANGE`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and this action makes no
change in any item's selection state,`XmNselectionCallback`(s)
is called with either`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`as the reason depending on`XmNselectionPolicy`and`auto_selection_type``XmAUTO_NO_CHANGE`.
* **ContainerEndSelect():** 

Processing depends on the value of`XmNselectionPolicy`, as follows:

* **`XmSINGLE_SELECT`** 

This action calls`XmNselectionCallback`with reason`XmCR_SINGLE_SELECT`.
* **`XmBROWSE_SELECT`** 

If the pointer is no longer over the current anchor item, this action
toggles the current anchor item and then toggles the item under the pointer
(if any). If`XmNautomaticSelection`is`XmNO_AUTO_SELECT`,
the`XmNselectionCallback`(s) is called with reason`XmCR_BROWSE_SELECT`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and a change in any item's
selection state is made,`XmNselectionCallback`(s) is called with reason`XmCR_BROWSE_SELECT`and`auto_selection_type``XmAUTO_CHANGE`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and no change
is made in any item's selection state by this action,`XmNselectionCallback`(s)
is called with reason`XmCR_BROWSE_SELECT`and`auto_selection_type``XmAUTO_NO_CHANGE`.
* **`XmMULTIPLE_SELECT`** 

This action first
performs the same processing as theContainerButtonMotion()action, except that`XmNselectionCallback`is not called.
If`XmNautomaticSelection`is`XmNO_AUTO_SELECT`, the`XmNselectionCallback`(s) is called with reason`XmCR_MULTIPLE_SELECT`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and a change in any item's
selection state is made, the`XmNselectionCallback`(s)
is called with reason`XmCR_MULTIPLE_SELECT`and`auto_selection_type``XmAUTO_CHANGE`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and this action makes no
change in any item's selection state,`XmNselectionCallback`(s)
is called with reason`XmCR_MULTIPLE_SELECT`and`auto_selection_type``XmAUTO_NO_CHANGE`.
* **`XmEXTENDED_SELECT`** 

This action first
performs the same processing as theContainerButtonMotion()action, except that`XmNselectionCallback`is not called.
If`XmNautomaticSelection`is`XmNO_AUTO_SELECT`, the`XmNselectionCallback`(s) is called with reason`XmCR_EXTENDED_SELECT`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and a change in any item's
selection state is made,`XmNselectionCallback`(s)
is called with reason`XmCR_EXTENDED_SELECT`and`auto_selection_type``XmAUTO_CHANGE`.
If`XmNautomaticSelection`is`XmAUTO_SELECT`and this action makes
no change in any item's selection state,`XmNselectionCallback`(s)
is called with reason`XmCR_EXTENDED_SELECT`and`auto_selection_type``XmAUTO_NO_CHANGE`.

* **ContainerEndToggle():** 

Simply returns if`XmNselectionPolicy`is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`.
If`XmNselectionPolicy`is`XmMULTIPLE_SELECT`or`XmEXTENDED_SELECT`, this action performs the same processing as theContainerEndSelect()action.
* **ContainerEndTransfer()::** 

If the elapsed time since aContainerStartTransfer()action has occurred exceeds the time
span specified by the display's multiclick time, this action returns.

Otherwise, theContainerPrimaryCopy(),ContainerPrimaryLink(), orContainerPrimaryMove()action is invoked, depending on the value
of the operation parameter saved byContainerStartTransfer().
* **ContainerExpandOrCollapse(`Left|Right|Collapse|Expand`):** 

This action changes the value of the`XmNoutlineState`of the
current focus widget. If the argument value is`Collapse`or`Left`, the`XmNoutlineState`resource value is set to`XmCOLLAPSED`. If the argument value is`Expand`or`Right`, the`XmNoutlineState`resource value is set to`XmEXPANDED`.

If the argument is`Left`or`Right`and the layout is right to
left, then the setting of the`XmNoutlineState`value is reversed
from that described in the preceding paragraph.

Simply returns if`XmNlayoutType`is`XmSPATIAL`.
* **ContainerExtend():** 

Processing depends on the value of`XmNselectionPolicy`, as follows:

If the selection policy is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`,
this action returns.
If`XmNlayoutType`is`XmSPATIAL`, this action returns.

If the selection policy is`XmMULTIPLE_SELECT`,
this action sets the selection state
of all items between the anchor item and the location cursor to
the selection state of the anchor item.

If the selection policy is`XmEXTENDED_SELECT`and the
Container is in Normal Mode, this action deselects all items and selects
all items between the anchor item and the location cursor.
If the selection policy is`XmEXTENDED_SELECT`and the Container is in Add Mode, this action
sets the selection state
of all items between the anchor item and the location cursor to
the selection state of the anchor item.

`XmNselectionCallback`is called with reason`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`depending on`XmNselectionPolicy`.
* **ContainerExtendCursor(`Left|Right|Up|Down`):** 

Processing depends on the value of`XmNselectionPolicy`, as follows:

If the selection policy is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`,
this action returns.
If`XmNlayoutType`is`XmSPATIAL`, this action returns.

This action
moves the location cursor one item in the indicated direction, if possible.
If the value of the argument string is`First`or`Last`, this action
moves the location cursor to the indicated item. For other values of the
argument string, the location cursor is not affected.

If the selection policy is`XmMULTIPLE_SELECT`,
this action sets the selection state
of all items between the anchor item and the location cursor to
the selection state of the anchor item.

If the selection policy is`XmEXTENDED_SELECT`and the
Container is in Normal Mode, this action deselects all items and selects
all items between the anchor item and the location cursor.
If the selection policy is`XmEXTENDED_SELECT`and the Container is in Add Mode, this action
sets the selection state
of all items between the anchor item and the location cursor to
the selection state of the anchor item.

`XmNselectionCallback`is called with reason`XmCR_MULTIPLE_SELECT`or`XmCR_EXTENDED_SELECT`depending on`XmNselectionPolicy`.
* **ContainerHandleBtn1Down(`string``)`** 

When Display's`XmNenableBtn1Transfer`resource is not`XmOFF`, the
actions for selection and transfer are integrated onBtn1.
If the pointer is over an unselected item or background, the item is
first selected before
the transfer is started. Otherwise, if the item is already selected, the
transfer is started.
The value of`string`can be one of
the following actions:

`ContainerBeginSelect,Copy`

`ContainerBeginToggle,Copy`

`ContainerNoop,Link`

`ContainerBeginExtend,Move`
* **ContainerHandleBtn1Motion(`string``)`** 

When Display's`XmNenableBtn1Transfer`resource is not`XmOFF`, the actions for selection
and transfer are integrated
onBtn1. When this action is invoked, and a selection is in
progress, a drag is performed. Otherwise,
the default
action as specified in`string`is performed. The value of`string`can be`ContainerButtonMotion`.
* **ContainerHandleBtn1Up(`string``)`** 

If a Button 1 transfer was in progress, then when this action
is invoked, that
transfer is cancelled. Otherwise,
the default
action as specified in`string`is performed. The value of`string`can be one of the following actions:

`ContainerEndSelect`

`ContainerEndToggle`

`ContainerEndExtend`
* **ContainerHandleBtn2Down(`string``)`** 

When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_ADJUST`, the
actions for extending selection are bound onBtn2. Otherwise,
the action that is
performed depends on the value of`string`, which can be one of
the following actions:

`ContainerStartTransfer,Copy`

`ContainerStartTransfer,Link`

`ContainerStartTransfer,Move`
* **ContainerHandleBtn2Motion(`string``)`** 

When Display's`XmNenableBtn1Transfer`resource is not`XmBUTTON2_ADJUST`, and a selection is in
progress, a drag is performed.
Otherwise, the default action that is
performed depends on the value of`string`, which can be`ContainerButtonMotion`.
* **ContainerHandleBtn2Up(`string``)`** 

When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_ADJUST`, this action ends an extend. Otherwise, the action
that is performed depends on the value of`string`, which can be`ContainerEndTransfer`.
* **ContainerMoveCursor(`Left|Right|Up|Down|First|Last`):** 

If the argument is`Left`,`Right`,`Up`, or`Down`, this action
moves the location cursor one item in the indicated direction, if possible.
If the value of the argument string is`First`or`Last`, this action
moves the location cursor to the indicated item. Any other arguments
are ignored.

If`XmNselectionPolicy`is`XmBROWSE_SELECT`,
or if`XmNselectionPolicy`is`XmEXTENDED_SELECT`and the
Container is in Normal Mode,
this action deselects all items, selects the item at the location cursor,
and calls`XmNselectionCallback`with the reason depending on`XmNselectionPolicy`.
* **ContainerPrimaryCopy():** 

This action requests that primary selection data be copied to the
Container.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmCOPY`operation.
By default, the Container does not do any transfer, and copying the
selection is the responsibility of the`XmNdestinationCallback`procedures.
* **ContainerPrimaryLink():** 

This action requests that primary selection data be linked to the
Container.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmLINK`operation.
By default, the Container does not do any transfer, and linking the
selection is the responsibility of the`XmNdestinationCallback`procedures.
* **ContainerPrimaryMove():** 

This action requests that primary selection data be copied to the
Container and deleted from the primary source.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmMOVE`operation.
By default, the Container does not do any transfer, and moving the
selection is the responsibility of the`XmNdestinationCallback`procedures.
If the transfer is successful, this action then calls the selection
owner's`XmNconvertCallback`procedures for the`PRIMARY`selection and the`DELETE`target.
* **ContainerSelect():** 

Processing depends on the value of`XmNselectionPolicy`, as follows:

If the selection policy is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`,
this action deselects all items and selects the item at the location cursor.

If the selection policy is`XmMULTIPLE_SELECT`, this action
toggles the selection state of the item at the location cursor.
This item becomes the anchor item for further selections.

If the selection policy is`XmEXTENDED_SELECT`and the
Container is in Normal Mode, this action deselects all items and selects
the item at the location cursor. If the selection policy is`XmEXTENDED_SELECT`and the Container is in Add Mode, this action
toggles the selection state of the item at the location cursor.
The selected/toggled item becomes the anchor item for further selections.

`XmNselectionCallback`is called with the reason depending on`XmNselectionPolicy`.
* **ContainerSelectAll():** 

If`XmNselectionPolicy`is`XmSINGLE_SELECT`or`XmBROWSE_SELECT`,
this action deselects all items and selects the item at the location cursor
position.

If`XmNselectionPolicy`is`XmMULTIPLE_SELECT`or`XmEXTENDED_SELECT`, this action selects all items.

`XmNselectionCallback`is called with the
reason depending on the value
of`XmNselectionCallback`.
* **ContainerStartTransfer(`Copy|Move|Link`):** 

This action saves the event and the operation specified in the argument
string for use by subsequent actions.
If noContainerEndTransfer()actions occur within the time
span specified by the display's multiclick time and if`XmNlayoutType`is`XmSPATIAL`, this action
creates a DragContext and starts a drag transfer by using`string`to
specify the transfer operation. If no argument string is
specified,`Copy`is the default value.

Unless default drag and drop behavior has been overridden by a`XmNconvertCallback`procedure,
if the drop operation occurs within the Container, then
the item(s) being dragged are relocated at the position of the
drop operation. If the item targeted by the Drag operation is
not in the selected state, then only that item is moved. If the item is
in the selected state, however, all items in the selected state are moved.
* **ContainerToggleMode():** 

If`XmNselectionPolicy`is`XmEXTENDED_SELECT`, this action
toggles the Container
between Normal Mode and Add Mode.

### Additional Behavior


The Container widget has the following additional behavior:

* **Btn1Down`(2+)`** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time, the Container interprets
that as a double-click and calls the`XmNdefaultActionCallback`callbacks.
* **FocusIn:** 

If the focus policy is explicit, sets the focus and draws
the location cursor.
* **FocusOut:** 

If the focus policy is explicit, removes the focus and erases
the location cursor.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see
&cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,`XmContainerCopy`,`XmContainerCopyLink`,`XmContainerCut`,`XmContainerGetItemChildren`,`XmContainerPaste`,`XmContainerPasteLink`,
&cdeman.XmContainerRelayout;,
&cdeman.XmContainerReorder;,
&cdeman.XmCreateContainer;,
&cdeman.XmCreateIconGadget;,
&cdeman.XmIconGadget;, and
&cdeman.XmManager;.