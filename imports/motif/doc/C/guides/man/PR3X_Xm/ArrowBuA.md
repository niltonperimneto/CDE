# XmArrowButton
library call`XmArrowButton`The ArrowButton widget classXmArrowButtonwidget classArrowButton&npzwc;#include &lt;Xm/ArrowB.h>
## DESCRIPTION


ArrowButton consists of a directional arrow surrounded by a
border shadow. When it is selected, the shadow changes
to give the appearance that the
ArrowButton has been pressed in. When the
ArrowButton is unselected, the shadow reverts to give the appearance that the
ArrowButton is released, or out.

ArrowButton holds the`XmQTactivatable`trait.
### Classes


ArrowButton inherits behavior, resources, and traits
from the`Core`and`XmPrimitive`classes.

The class pointer is`xmArrowButtonWidgetClass`.

The class name is`XmArrowButton`.
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

`XmArrowButton Resource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNarmCallbackXmCCallbackXtCallbackListNULLCXmNarrowDirectionXmCArrowDirectionunsigned charXmARROW_UPCSGXmNdetailShadowThicknessXmCDetailShadowThicknessDimension2CSGXmNdisarmCallbackXmCCallbackXtCallbackListNULLCXmNmultiClickXmCMultiClickunsigned chardynamicCSG

* **`XmNactivateCallback`** 

Specifies a list of callbacks that is called
when the ArrowButton is activated.
To activate the button, press and release`BSelect`while the pointer is inside
the ArrowButton widget. Activating the ArrowButton
also disarms it.
The reason sent by this callback is`XmCR_ACTIVATE`.
This callback uses the`XmQTactivatable`trait.
* **`XmNarmCallback`** 

Specifies a list of callbacks that is called
when the ArrowButton is armed.
To arm this widget, press`BSelect`while the pointer is inside the ArrowButton.
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
when the ArrowButton is disarmed.
To disarm this widget, press and release`BSelect`while the pointer is inside the ArrowButton.
The reason for this callback is`XmCR_DISARM`.
* **`XmNmultiClick`** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time, and this resource is
set to`XmMULTICLICK_DISCARD`, the second click.
is not processed.
If this resource is set to`XmMULTICLICK_KEEP`, the event
is processed and`click_count`is incremented in the callback structure.
When the button is not in a menu, the default value is`XmMULTICLICK_KEEP`.

### Inherited Resources


ArrowButton inherits behavior and resources from the
superclasses described in the following table.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
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
if the`XmNmultiClick`resource is set to`XmMULTICLICK_KEEP`;
otherwise it contains 1.
The activate callback is invoked for each click if`XmNmultiClick`is set to`XmMULTICLICK_KEEP`.

### Translations


XmArrowButton includes translations for XmPrimitive.
The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **EnterWindow:** 

Enter()
* **LeaveWindow:** 

Leave()
* **`c<Btn1Down>`:** 

ButtonTakeFocus()
* **`&ap;c`Btn1Down:** 

Arm()
* **`&ap;c`Btn1Down`,&ap;c`Btn1Up:** 

Activate() Disarm()
* **`&ap;c`Btn1Down`(2+)`:** 

MultiArm()
* **`&ap;c`Btn1Up`(2+)`:** 

MultiActivate()
* **`&ap;c`Btn1Up:** 

Activate() Disarm()
* **`:`KeyosfActivate:** 

PrimitiveParentActivate()
* **`:`KeyosfCancel:** 

PrimitiveParentCancel()
* **`:`KeyosfSelect:** 

ArmAndActivate()
* **`:`KeyosfHelp:** 

Help()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

PrimitiveParentActivate()
* **`&ap;s &ap;m &ap;a`Key`space`:** 

ArmAndActivate()

### Action Routines


The`XmArrowButton`action routines are

* **Activate():** 

Draws the shadow in the unselected state.
If the pointer is within the ArrowButton,
calls the callbacks for`XmNactivateCallback`.
* **Arm():** 

Draws the shadow in the selected state and
calls the callbacks for`XmNarmCallback`.
* **ArmAndActivate():** 

Draws the shadow in the selected state and
calls the callbacks for`XmNarmCallback`.
Arranges for the shadow to be drawn in the unselected state and
the callbacks for`XmNactivateCallback`and`XmNdisarmCallback`to be called, either immediately or at a later time.
* **ButtonTakeFocus():** 

Causes the ArrowButton to take keyboard focus
when`Ctrl<Btn1Down>`is pressed, without activating the widget.
* **Disarm():** 

Draws the shadow in the unselected state and
calls the callbacks for`XmNdisarmCallback`.
* **Help():** 

Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help callbacks for this widget, this action calls the
help callbacks for the nearest ancestor that has them.
* **MultiActivate():** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
increments`click_count`in the callback structure and
draws the shadow in the unselected state.
If the pointer is within the ArrowButton, this action calls the
callbacks for`XmNactivateCallback`and`XmNdisarmCallback`.
* **MultiArm():** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.
If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
draws the shadow in the selected state and calls the callbacks for`XmNarmCallback`.

### Additional Behavior


This widget has the following additional behavior:

* **EnterWindow:** 

Draws the ArrowButton shadow in its selected state if the
pointer leaves and re-enters the window whileBtn1is pressed.
* **LeaveWindow:** 

Draws the ArrowButton shadow in its unselected state
if the pointer leaves the window whileBtn1is pressed.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;, &cdeman.XmCreateArrowButton;, and &cdeman.XmPrimitive;.