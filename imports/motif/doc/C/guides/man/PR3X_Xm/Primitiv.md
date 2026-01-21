# XmPrimitive
library call`XmPrimitive`The Primitive widget classXmPrimitivewidget classPrimitive&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Primitive is a widget class used as a supporting superclass
for other widget classes. It handles border drawing and highlighting,
traversal activation and deactivation, and various callback lists needed by
Primitive widgets.
Primitive and all its subclasses hold the`XmQTcareParentVisual`trait.
### Data Transfer Behavior


Primitive has no widget class conversion or destination procedure.
Subclasses and the`XmNconvertCallback`procedures are responsible
for any conversion of selections.
Subclasses and the subclass`XmNdestinationCallback`procedures are
responsible for any data transfers to the widget.
### Classes


Primitive inherits behavior, resources, and traits from`Core`.

The class pointer is`xmPrimitiveWidgetClass`.

The class name is`XmPrimitive`.
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

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

* **`XmNbottomShadowColor`** 

Specifies the color to use to draw the bottom and right sides of the
border shadow.
This color is used if the`XmNtopShadowPixmap`resource is
unspecified.
* **`XmNbottomShadowPixmap`** 

Specifies the pixmap to use to draw the bottom and right sides of the
border shadow.
* **`XmNconvertCallback`** 

Specifies a list of callbacks called when the widget is asked to convert
a selection.
The type of the structure whose address is passed to these callbacks isXmConvertCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNforeground`** 

Specifies the foreground drawing color used by Primitive widgets.
* **`XmNhelpCallback`** 

Specifies the list of callbacks that is called when the help key
is pressed. The reason sent by the callback is`XmCR_HELP`.
* **`XmNhighlightColor`** 

Specifies the color of the highlighting rectangle.
This color is used if the highlight pixmap resource is`XmUNSPECIFIED_PIXMAP`.
* **`XmNhighlightOnEnter`** 

Specifies if the highlighting rectangle is drawn when the cursor moves
into the widget.
If the shell's focus policy is`XmEXPLICIT`, this resource is
ignored, and the widget is highlighted when it has the focus.
If the shell's focus policy is`XmPOINTER`and if this resource is
True, the highlighting rectangle is drawn when the cursor moves into
the widget.
If the shell's focus policy is`XmPOINTER`and if this resource is
False, the highlighting rectangle is not drawn when the the cursor moves
into the widget.
The default is False.
* **`XmNhighlightPixmap`** 

Specifies the pixmap used to draw the highlighting rectangle.
* **`XmNhighlightThickness`** 

Specifies the thickness of the highlighting rectangle.
* **`XmNlayoutDirection`** 

Specifies the direction in which components of the primitive (including
strings) are laid out. The values are of typeXmDirection. If
the widget's parent is a primitive or shell, the value is inherited from
the widget's parent. Otherwise, it is inherited from the closest
ancestor vendor or menu shell. Refer to the &cdeman.XmDirection;
reference page for the possible direction values.
* **`XmNnavigationType`** 

Determines whether the widget is a tab group.

* **`XmNONE`** 

Indicates that the widget is not a tab group.
* **`XmTAB_GROUP`** 

Indicates that the widget is a tab group, unless
the`XmNnavigationType`of another widget in the hierarchy is`XmEXCLUSIVE_TAB_GROUP`.
* **`XmSTICKY_TAB_GROUP`** 

Indicates that the widget is a tab group, even
if the`XmNnavigationType`of another widget in the hierarchy is`XmEXCLUSIVE_TAB_GROUP`.
* **`XmEXCLUSIVE_TAB_GROUP`** 

Indicates that the widget is a tab group and
that widgets in the hierarchy whose`XmNnavigationType`is`XmTAB_GROUP`are not tab groups.

When a parent widget has an`XmNnavigationType`of`XmEXCLUSIVE_TAB_GROUP`, traversal of non-tab-group widgets within
the group is based on the order of those widgets in their parent's`XmNchildren`list.

When the`XmNnavigationType`of any widget in a hierarchy is`XmEXCLUSIVE_TAB_GROUP`, traversal of tab groups in the hierarchy
proceeds to widgets in the order in which their`XmNnavigationType`resources were specified as`XmEXCLUSIVE_TAB_GROUP`or`XmSTICKY_TAB_GROUP`, whether by creating the widgets with that value,
by calling`XtSetValues`, or by calling`XmAddTabGroup`.

* **`XmNpopupHandlerCallback`** 

Allows the application to control which popup menu will be
automatically posted. The reason can either be`XmCR_POST`or`XmCR_REPOST:`

* **`XmCR_POST`** 

Indicates that this is a regular posting request.
* **`XmCR_REPOST`** 

Indicates that the menu was just unposted and that this callback was
invoked on a replay.


This callback
uses theXmPopupHandlerCallbackStructstructure to pass information.
* **`XmNshadowThickness`** 

Specifies the size of the drawn border shadow.
* **`XmNtopShadowColor`** 

Specifies the color to use to draw the top and left sides of the border
shadow.
This color is used if the`XmNtopShadowPixmap`resource is
unspecified.
If a default top shadow pixmap exists, it will need to be removed for
the`XmNtopShadowColor`to take effect.
* **`XmNtopShadowPixmap`** 

Specifies the pixmap to use to draw the top and left sides of the border
shadow.
A Primitive top shadow pixmap is created in two situations. In either
of these situations, a default 50-foreground top shadow
pixmap is created.

If the Primitive top shadow color is the same as the
Core background pixel color.

If the depth of the screen is only one.

For example, if a widget with the same top shadow and background color
is created, a default shadow pixmap is generated. Such a pixmap
needs to be removed for the`XmNtopShadowColor`resource to take
effect.
* **`XmNtraversalOn`** 

Specifies if traversal is activated for this widget. In CascadeButton
and CascadeButtonGadget, this resource is forced to True unless the parent
is an OptionMenu.
* **`XmNunitType`** 

Provides the basic support for resolution independence.
It defines the type of units a widget uses with sizing and
positioning resources.
If the widget's parent is a subclass of`XmManager`and if the`XmNunitType`resource is not explicitly set, it defaults to the
unit type of the parent widget.
If the widget's parent is not a subclass of`XmManager`, the
resource has a default unit type of`XmPIXELS`.

The unit type can also be specified in resource files, with the
following format:`<floating value><unit>`

where:

* **`unit`** 

is <" ", pixels, inches, centimeters, millimeters, points, font units>
* **pixels** 

is <`pix`,`pixel`,pixels>
* **`inches`** 

is <`in`,`inch`,`inches`>
* **`centimeter`** 

is <`cm`,`centimeter`,`centimeters`>
* **`millimeters`** 

is <`mm`,`millimeter`,`millimeters`>
* **`points`** 

is <`pt`,`point`,`points`>
* **`font units`** 

is <`fu`,`font_unit`,`font_units`>
* **`float`** 

is {"+"|"-"}{{<"0"-"9">*}.}<"0"-"9">*

Note that the type Dimension must always be positive.


For example,xmfonts*XmMainWindow.height: 10.4cm
*PostIn.width: 3inches

`XmNunitType`can have the following values:

* **`XmPIXELS`** 

All values provided to the widget are treated as normal
pixel values.
* **`XmMILLIMETERS`** 

All values provided to the widget are treated as normal millimeter
values.
* **`Xm100TH_MILLIMETERS`** 

All values provided to the widget are treated
as 1/100 of a millimeter.
* **`XmCENTIMETERS`** 

All values provided to the widget are treated as normal centimeter
values.
* **`XmINCHES`** 

All values provided to the widget are treated as normal inch
values.
* **`Xm1000TH_INCHES`** 

All values provided to the widget are treated as
1/1000 of an inch.
* **`XmPOINTS`** 

All values provided to the widget are treated as normal point
values. A point is a unit used in text processing
applications and is defined as 1/72 of an inch.
* **`Xm100TH_POINTS`** 

All values provided to the widget are treated as
1/100 of a point. A point is a unit used in text processing
applications and is defined as 1/72 of an inch.
* **`XmFONT_UNITS`** 

All values provided to the widget are treated as normal font
units. A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.
* **`Xm100TH_FONT_UNITS`** 

All values provided to the widget are
treated as 1/100 of a font unit.
A font unit has horizontal and vertical components.
These are the values of the`XmScreen`resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

* **`XmNuserData`** 

Allows the application to attach any necessary specific data to the widget.
It is an internally unused resource.

### Dynamic Color Defaults


The foreground, background, top shadow, bottom shadow, and
highlight color resources are dynamically defaulted.
If no color data is specified, the colors are
automatically generated. On a single-plane system, a black and white color
scheme is generated. Otherwise, four colors are
generated, which display the correct shading for the 3-D visuals.
If the background is the only color specified for a widget, the top
shadow and bottom shadow colors are generated to give the 3-D appearance.
Foreground and highlight colors are generated to provide sufficient
contrast with the background color.

Colors are generated only at creation. Resetting the background through`XtSetValues`does not regenerate the other colors.`XmChangeColor`can be used to recalculate all associated colors
based on a new background color.
### Inherited Resources


Primitive inherits behavior and resources from the
superclass described in the following table.
For a complete description of each resource, refer to the
reference page for that superclass.

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback for`XmNhelpCallback`:typedef struct
{
        int`reason`;
        XEvent`* event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
For this callback,`reason`is set to`XmCR_HELP`.
* **`event`** 

Points to the`XEvent`that triggered the callback.


A pointer to the following callback structure is passed to the`XmNconvertCallback`procedures:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Atom`selection`;
        Atom`target`;
        XtPointer`source_data`;
        XtPointer location_data;
        int`flags`;
        XtPointer`parm`;
        int`parm_format`;
        unsigned long`parm_length`;
        Atom`parm_type`;
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
Otherwise, the type and interpretation of the value are specific to the
widget class.
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
* **`parm_type`** 

Specifies the parameter type of`parm`.
* **`status`** 

An IN/OUT member that specifies the status of the conversion.
The initial value is`XmCONVERT_DEFAULT`.
The callback procedure can set this member to one of the following
values:

* **`XmCONVERT_DEFAULT`** 

This value means that the widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it
overwrites the data provided by the callback procedures in the`value`member.
* **`XmCONVERT_MERGE`** 

This value means that the widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it appends
its data to the data provided by the callback procedures in the`value`member.
This value is intended for use with targets that result in lists of
data, such as`TARGETS`.
* **`XmCONVERT_DONE`** 

This value means that the callback procedure has successfully finished
the conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.
* **`XmCONVERT_REFUSE`** 

This value means that the callback procedure has terminated the
conversion process without completing the requested conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.

* **`value`** 

An IN/OUT parameter that contains any data that the callback procedure
produces as a result of the conversion.
The initial value is NULL.
If the callback procedure sets this member, it must ensure that the`type`,`format`, and`length`members correspond
to the data in`value`.
The callback procedure is responsible for allocating memory when it sets
this member.
The toolkit frees this memory when it is no longer needed.
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
The initial value is 0.


A pointer to the following structure is passed to each callback for`XmNpopupHandlerCallback`:typedef struct
{
        int`reason`;
        XEvent`* xevent`;
        Widget`menuToPost`;
        Boolean`postIt`;
        Widget`target`;
} XmPopupHandlerCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`xevent`** 

Points to the`XEvent`that triggered the handler.
* **`menuToPost`** 

Specifies the popup menu that the menu system believes should be
posted. The application may modify this field.
* **`postIt`** 

Indicates whether the posting process should continue. The
application may modify this field.
* **`target`** 

Specifies the most specific widget or gadget that the menu sytem found
from the event that matches the event.

### Translations


The`XmPrimitive`translations are listed below.

Note that for buttons in menus, altering translations in`#override`or`#augment`mode is undefined.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`:`KeyosfActivate:** 

PrimitiveParentActivate()
* **`:`KeyosfCancel:** 

PrimitiveParentCancel()
* **`:`KeyosfBeginLine:** 

PrimitiveTraverseHome()
* **`:`KeyosfUp:** 

PrimitiveTraverseUp()
* **`:`KeyosfDown:** 

PrimitiveTraverseDown()
* **`:`KeyosfLeft:** 

PrimitiveTraverseLeft()
* **`:`KeyosfRight:** 

PrimitiveTraverseRight()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

PrimitiveParentActivate()
* **`s &ap;m &ap;a`Key`Tab`:** 

PrimitivePrevTabGroup()
* **`&ap;m &ap;a`Key`Tab`:** 

PrimitiveNextTabGroup()
* **KeyosfHelp:** 

PrimitiveHelp()

### Action Routines


The`XmPrimitive`action routines are

* **PrimitiveHelp():** 

Calls the callbacks for`XmNhelpCallback`if any exist. If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **PrimitiveNextTabGroup():** 

This action depends on the value of the Display resource`XmNenableButtonTab`. When`XmNenableButtonTab`is False
(default), this action traverses to the first item in the next tab
group. If the current tab group is the last entry in the tab group
list, it wraps to the beginning of the tab group list.

When`XmNenableButtonTab`is True, this action moves to the next
item within the current tab group, unless it is the last item in the
tab group. When the item is the last in the group, the action
traverses to the first item in the next tab group. The`XmNenableButtonTab`behavior applies only to PushButton, ArrowButton,
and DrawnArrow.
* **PrimitiveParentActivate():** 

If the parent is a manager,
passes the`KActivate`event received by the widget
to the parent.
* **PrimitiveParentCancel():** 

If the parent is a manager,
passes the`KCancel`event received by the widget
to the parent.
* **PrimitivePrevTabGroup():** 

This action depends on the value of the Display resource`XmNenableButtonTab`. When`XmNenableButtonTab`is False
(default), this action traverses to the first item in the previous tab
group. If the beginning of the tab group list is reached, it wraps to
the end of the tab group list.

When`XmNenableButtonTab`is True, this action moves to the
previous item within the current tab group unless it is the first item
in the tab group. When the item is the first in the group, the action
traverses to the first item in the previous tab group. The`XmNenableButtonTab`behavior applies only PushButton, ArrowButton, and
DrawnButton.
* **PrimitiveTraverseDown():** 

Traverses to the next item below the current widget in the current tab
group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **PrimitiveTraverseHome():** 

Traverses to the first widget or gadget in the current tab group.
* **PrimitiveTraverseLeft():** 

Traverses to the next item to the left of the current widget in the
current tab group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **PrimitiveTraverseNext():** 

Traverses to the next item in the current tab group, wrapping if
necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **PrimitiveTraversePrev():** 

Traverses to the previous item in the current tab group, wrapping if
necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **PrimitiveTraverseRight():** 

Traverses to the next item to the right of the current gadget in the
current tab group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **PrimitiveTraverseUp():** 

Traverses to the next item above the current gadget in the current tab
group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.

### Additional Behavior


This widget has the following additional behavior:

* **FocusIn:** 

If the shell's keyboard focus policy is`XmEXPLICIT`, highlights the
widget and gives it the focus
* **FocusOut:** 

If the shell's keyboard focus policy is`XmEXPLICIT`, unhighlights
the widget and removes the focus

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;,
&cdeman.XmDirection;,
&cdeman.XmChangeColor;, and
&cdeman.XmScreen;.