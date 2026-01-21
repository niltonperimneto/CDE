# XmArrowButtonGadget
library call`XmArrowButtonGadget`The ArrowButtonGadget widget classXmArrowButtonGadgetwidget classArrowButtonGadget&npzwc;#include &lt;Xm/ArrowBG.h>
## DESCRIPTION


ArrowButtonGadget consists of a directional arrow surrounded by a border shadow.
When it is selected, the shadow changes to give the appearance that the
ArrowButtonGadget has been pressed in. When it is unselected, the
shadow reverts to give the appearance that the button is released, or out.

ArrowButtonGadget holds the`XmQTactivatable`trait.
### Classes


ArrowButtonGadget inherits behavior, resources, and traits
from the`Object`,`RectObj`, and`XmGadget`classes.

The class pointer is`xmArrowButtonGadgetClass`.

The class name is`XmArrowButtonGadget`.
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

`XmArrowButtonGadget Resource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNarmCallbackXmCCallbackXtCallbackListNULLCXmNarrowDirectionXmCArrowDirectionunsigned charXmARROW_UPCSGXmNdetailShadowThicknessXmCDetailShadowThicknessDimension2CSGXmNdisarmCallbackXmCCallbackXtCallbackListNULLCXmNmultiClickXmCMultiClickunsigned chardynamicCSG

* **`XmNactivateCallback`** 

Specifies a list of callbacks that is called
when the ArrowButtonGadget is activated.
To activate the button, press and release`BSelect`while the pointer is inside
the ArrowButtonGadget. Activating the ArrowButtonGadget
also disarms it.
The reason sent by this callback is`XmCR_ACTIVATE`.
This callback uses the`XmQTactivatable`trait.
* **`XmNarmCallback`** 

Specifies a list of callbacks that is called
when the ArrowButtonGadget is armed.
To arm this widget, press`BSelect`while the pointer is inside the ArrowButtonGadget.
The reason sent by this callback is`XmCR_ARM`.
* **`XmNarrowDirection`** 

Sets the arrow direction.
The values for this resource are

`XmARROW_UP`

`XmARROW_DOWN`

`XmARROW_LEFT`

`XmARROW_RIGHT`
* **`XmNdetailShadowThickness`** 

Specifies the thickness of the inside arrow shadows. The default thickness
is 2 pixels.
* **`XmNdisarmCallback`** 

Specifies a list of callbacks that is called
when the ArrowButtonGadget is disarmed.
To disarm this widget, press and release`BSelect`while the pointer is inside the ArrowButtonGadget.
The reason sent by this callback is`XmCR_DISARM`.
* **`XmNmultiClick`** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time and this resource is
set to`XmMULTICLICK_DISCARD`, the second click is not processed.
If this resource is set to`XmMULTICLICK_KEEP`, the event
is processed and`click_count`is incremented in the callback structure.
When the ArrowButtonGadget is not in a menu, the default value is`XmMULTICLICK_KEEP`.

### Inherited Resources


`XmArrowButtonGadget`inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        int`click_count`;
} XmArrowButtonCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`click_count`** 

This value is valid only when the reason is`XmCR_ACTIVATE`.
It contains the number of clicks in the last multiclick sequence
if the`XmNmultiClick`resource is set to`XmMULTICLICK_KEEP`,
otherwise it contains 1.
The activate callback is invoked for each click if`XmNmultiClick`is set to`XmMULTICLICK_KEEP`.

### Behavior


`XmArrowButtonGadget`includes behavior from`XmGadget`.
The following list describes additional
XmArrowButtonGadget behavior:

* **Btn1Down:** 

Draws the shadow in the selected state and
calls the callbacks for`XmNarmCallback`.
* **Btn1Down`&ensp;or&ensp;`Btn1Up:** 

Draws the shadow in the unselected state.
If the pointer is within the ArrowButtonGadget,
calls the callbacks for`XmNactivateCallback`.
Calls the callbacks for`XmNdisarmCallback`.
* **Btn1Down`(2+)`:** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.
If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
draws the shadow in the selected state and calls the callbacks for`XmNarmCallback`.
* **Btn1Up`(2+)`:** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
increments`click_count`in the callback structure and
draws the shadow in the unselected state.
If the pointer is within the ArrowButtonGadget, this action calls the
callbacks for`XmNactivateCallback`and`XmNdisarmCallback`.
* **KeyosfSelect:** 

Draws the shadow in the selected state and
calls the callbacks for`XmNarmCallback`.
Arranges for the shadow to be drawn in the unselected state and
the callbacks for`XmNactivateCallback`and`XmNdisarmCallback`to be called, either immediately or at a later time.
* **KeyosfHelp:** 

Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help callbacks for this widget, this action calls the help
callbacks for the nearest ancestor that has them.
* **Enter:** 

Draws the ArrowButtonGadget shadow in its selected state if the
pointer leaves and re-enters the gadget while <Btn1> is pressed.
* **Leave:** 

Draws the ArrowButtonGadget shadow in its unselected state
if the pointer leaves the gadget while <Btn1> is pressed.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Object;, &cdeman.RectObj;,
&cdeman.XmCreateArrowButtonGadget;, and
&cdeman.XmGadget;.