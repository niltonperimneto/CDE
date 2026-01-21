# XmGadget
library call`XmGadget`The Gadget widget classXmGadgetwidget classGadget&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Gadget is a widget class used as
a supporting superclass for other
gadget classes. It handles shadow-border drawing and highlighting, traversal
activation and deactivation, and various callback lists needed by
gadgets.

The color and pixmap resources defined by`XmManager`are directly used by
gadgets. If`XtSetValues`is used
to change one of the resources for a manager
widget, all of the gadget children within the manager also change.
### Classes


Gadget inherits behavior and resources from`Object`and`RectObj`.

The class pointer is`xmGadgetClass`.

The class name is`XmGadget`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a .Xdefaults file, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a .Xdefaults file, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

* **`XmNbackground`** 

Specifies the background color for the gadget.
* **`XmNbackgroundPixmap`** 

Specifies a pixmap for tiling the background. The first tile is
placed at the upper left corner of the widget's window.
* **`XmNbottomShadowColor`** 

Contains the color to use to draw the bottom and right sides of the
border shadow.
* **`XmNbottomShadowPixmap`** 

Specifies the pixmap to use to draw the bottom and right sides of the
border shadow.
* **`XmNforeground`** 

Specifies the foreground drawing color used by Primitive widgets.
* **`XmNhelpCallback`** 

Specifies the list of callbacks that is called when the help key sequence
is pressed. The reason sent by the callback is`XmCR_HELP`.
* **`XmNhighlightColor`** 

Contains the color of the highlighting rectangle.
* **`XmNhighlightOnEnter`** 

Specifies if the highlighting rectangle is drawn when the cursor moves
into the widget.
If the shell's focus policy is`XmEXPLICIT`, this resource is
ignored, and the widget is highlighted when it has the focus.
If the shell's focus policy is`XmPOINTER`and if this resource is
True, the highlighting rectangle is drawn when the the cursor moves into
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

Specifies the direction in which components of the manager (including
strings) are laid out. The values are of typeXmDirection. If
the widget's parent is a manager or shell, the value is inherited from
the widget's parent. Otherwise, it is inherited from the closest
ancestor vendor or menu shell.
* **`XmNnavigationType`** 

Determines whether the widget is a tab group.

* **`XmNONE`** 

Indicates that the widget is not a tab group.
* **`XmTAB_GROUP`** 

Indicates that the widget is a tab group, unless
the`XmNnavigationType`of another widget in the hierarchy is`XmEXCLUSIVE_TAB_GROUP`.
* **`XmSTICKY_TAB_GROUP`** 

Indicates that the widget is a tab group, even if the`XmNnavigationType`of another widget in the hierarchy is`XmEXCLUSIVE_TAB_GROUP`.
* **`XmEXCLUSIVE_TAB_GROUP`** 

Indicates that the widget is a tab group and
that widgets in the hierarchy whose`XmNnavigationType`is`XmTAB_GROUP`are not tab groups.

When a parent widget has an`XmNnavigationType`of`XmEXCLUSIVE_TAB_GROUP`, traversal of non-tab-group widgets within
the group is based on the order of those widgets in their parent's`XmNchildren`list.

When the`XmNnavigationType`of any widget in a hierarchy is`XmEXCLUSIVE_TAB_GROUP`,
traversal of tab groups in the hierarchy
proceeds to widgets in the order in which their`XmNnavigationType`resources were specified as`XmEXCLUSIVE_TAB_GROUP`or`XmSTICKY_TAB_GROUP`, whether by creating the widgets with that value,
by calling`XtSetValues`, or by calling`XmAddTabGroup`.

* **`XmNshadowThickness`** 

Specifies the size of the drawn border shadow.
* **`XmNtopShadowColor`** 

Contains the color to use to draw the top and left sides of the border
shadow.
* **`XmNtopShadowPixmap`** 

Specifies the pixmap to use to draw the top and left sides of the border
shadow.
* **`XmNtraversalOn`** 

Specifies traversal activation for this gadget.
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
applications and is defined as 1/72 inch.
* **`XmFONT_UNITS`** 

All values provided to the widget are treated as normal font
units. A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.
* **`Xm100TH_FONT_UNITS`** 

All values provided to the widget are
treated as 1/100 of a font unit.
A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

* **`XmNuserData`** 

Allows the application to attach any necessary specific data to the gadget.
This is an internally unused resource.

### Inherited Resources


Gadget inherits resources from the
superclass described in the following table.
For a complete description of each resource, refer to the
reference page for that superclass.

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
For this callback,`reason`is set to`XmCR_HELP`.
* **`event`** 

Points to the`XEvent`that triggered the callback.

### Behavior


Gadgets cannot have translations associated with them.
Because of this, a Gadget's behavior is determined by the Manager
widget into which the Gadget is placed. If focus is on a Gadget,
events are passed to the Gadget by its Manager.
## RELATED


&cdeman.Object;,
&cdeman.RectObj;,
&cdeman.XmManager;, and
&cdeman.XmScreen;.