# XmSeparatorGadget
library call`XmSeparatorGadget`The SeparatorGadget widget classXmSeparatorGadgetwidget classSeparatorGadget&npzwc;#include &lt;Xm/SeparatoG.h>
## DESCRIPTION


SeparatorGadget separates items in a display.
Several
line drawing styles are provided, as well as horizontal or vertical
orientation.

Lines drawn within the SeparatorGadget are automatically
centered within the height of the gadget for a horizontal orientation
and centered within the width of the gadget for a vertical orientation.
An`XtSetValues`with a new`XmNseparatorType`resizes the
widget to its minimal height (for horizontal orientation) or its
minimal width (for
vertical orientation) unless height or width is explicitly set in the`XtSetValues`call.

SeparatorGadget does not draw shadows around the separator.
The Gadget resource`XmNshadowThickness`is used for the
SeparatorGadget's thickness when`XmNseparatorType`is`XmSHADOW_ETCHED_IN`,`XmSHADOW_ETCHED_IN_DASH`,`XmSHADOW_ETCHED_OUT`, or`XmSHADOW_ETCHED_OUT_DASH`.

SeparatorGadget does not highlight and allows no traversing. The Gadget
resource`XmNtraversalOn`is forced to False.
SeparatorGadget
holds the`XmQTmenuSavvy`trait.
### Classes


SeparatorGadget inherits behavior,
resources, and traits from`Object`,`RectObj`, and`XmGadget`.

The class pointer is`xmSeparatorGadgetClass`.

The class name is`XmSeparatorGadget`.
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

`XmSeparatorGadget Resource Set``Name``Class``Type``Default``Access`XmNmarginXmCMarginDimension0CSGXmNorientationXmCOrientationunsigned charXmHORIZONTALCSGXmNseparatorTypeXmCSeparatorTypeunsigned charXmSHADOW_ETCHED_INCSG

* **`XmNmargin`** 

For horizontal orientation, specifies the space on the left and right sides between the border of
SeparatorGadget and the line drawn.
For vertical
orientation, specifies the space
on the top and bottom between the border
of SeparatorGadget and the line drawn.
* **`XmNorientation`** 

Specifies whether SeparatorGadget is displayed vertically or horizontally.
This resource can have values of`XmVERTICAL`and`XmHORIZONTAL`.
* **`XmNseparatorType`** 

Specifies the type of line drawing to be done in the Separator widget.

* **`XmSINGLE_LINE`** 

Single line.
* **`XmDOUBLE_LINE`** 

Double line.
* **`XmSINGLE_DASHED_LINE`** 

Single-dashed line.
* **`XmDOUBLE_DASHED_LINE`** 

Double-dashed line.
* **`XmNO_LINE`** 

No line.
* **`XmSHADOW_ETCHED_IN`** 

A line whose shadows give the
effect of a line etched into the window. The thickness of the
line is equal to the value of`XmNshadowThickness`. For
horizontal orientation, the top shadow is drawn in`XmNtopShadowColor`and the bottom shadow is drawn in`XmNbottomShadowColor`. For
vertical orientation, the left edge is drawn in`XmNtopShadowColor`and the right edge is drawn in`XmNbottomShadowColor`.
* **`XmSHADOW_ETCHED_OUT`** 

A line whose shadows give the effect
of an etched line coming out of the window. The thickness of the
line is equal to the value of`XmNshadowThickness`. For horizontal
orientation, the top shadow is drawn in`XmNbottomShadowColor`and
the bottom shadow is drawn in`XmNtopShadowColor`. For vertical
orientation, the left edge is drawn in`XmNbottomShadowColor`and
the right edge is drawn in`XmNtopShadowColor`.
* **`XmSHADOW_ETCHED_IN_DASH`** 

Identical to`XmSHADOW_ETCHED_IN`except a series of lines creates a dashed line.
* **`XmSHADOW_ETCHED_OUT_DASH`** 

Identical to`XmSHADOW_ETCHED_OUT`except a series of lines creates a dashed line.


### Inherited Resources


SeparatorGadget inherits behavior and resources from the
superclasses in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension0CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanFalseGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Behavior


`XmSeparatorGadget`has no behavior.
## RELATED


&cdeman.Object;, &cdeman.RectObj;,
&cdeman.XmCreateSeparatorGadget;, and
&cdeman.XmGadget;.