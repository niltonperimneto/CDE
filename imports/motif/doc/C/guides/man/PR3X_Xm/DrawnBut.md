# XmDrawnButton
library call`XmDrawnButton`The DrawnButton widget classXmDrawnButtonwidget classDrawnButton&npzwc;#include &lt;Xm/DrawnB.h>
## DESCRIPTION


The DrawnButton widget consists of an empty widget window surrounded by a
shadow border. It provides the application developer with a graphics area
that can have PushButton input semantics.

Callback types are defined for widget exposure and widget resize to allow the
application to redraw or reposition its graphics. If the DrawnButton
widget has a highlight and shadow thickness, the application should not draw
in that area.
To avoid drawing in the highlight and
shadow area, create the graphics context with
a clipping rectangle for drawing in the widget.
The clipping rectangle should
take into account the size of the widget's highlight thickness and
shadow.
DrawnButton uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits, and
holds the`XmQTactivatable`trait.
### Classes


BulletinBoard inherits behavior, resources, and traits
from the`Core`,`Composite`,`Constraint`,
and`XmManager`classes.

The class pointer is`xmDrawnButtonWidgetClass`.

The class name is`XmDrawnButton`.
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

`XmDrawnButton Resource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNarmCallbackXmCCallbackXtCallbackListNULLCXmNdisarmCallbackXmCCallbackXtCallbackListNULLCXmNexposeCallbackXmCCallbackXtCallbackListNULLCXmNmultiClickXmCMultiClickunsigned chardynamicCSGXmNpushButtonEnabledXmCPushButtonEnabledBooleanFalseCSGXmNresizeCallbackXmCCallbackXtCallbackListNULLCXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_ETCHED_INCSG

* **`XmNactivateCallback`** 

Specifies the list of callbacks that is called
when the widget becomes selected.
The reason sent by the callback is`XmCR_ACTIVATE`.
This callback uses the`XmQTactivatable`trait.
* **`XmNarmCallback`** 

Specifies the list of callbacks that is called
when the widget becomes armed.
The reason sent by the callback is`XmCR_ARM`.
* **`XmNdisarmCallback`** 

Specifies the list of callbacks that is called
when the widget becomes disarmed.
The reason sent by the callback is`XmCR_DISARM`.
* **`XmNexposeCallback`** 

Specifies the list of callbacks that is called
when the widget receives an exposure event.
The reason sent by the callback is`XmCR_EXPOSE`.
* **`XmNmultiClick`** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time, and this resource is
set to`XmMULTICLICK_DISCARD`, the second click is not processed.
If this resource is set to`XmMULTICLICK_KEEP`, the event
is processed and`click_count`is incremented in the callback structure.
When the button is not in a menu, the default value is`XmMULTICLICK_KEEP`.
* **`XmNpushButtonEnabled`** 

Enables or disables the 3-dimensional shadow drawing as in
PushButton.
* **`XmNresizeCallback`** 

Specifies the list of callbacks that is called
when the widget receives a resize event.
The reason sent by the callback is`XmCR_RESIZE`. The event
returned for this callback is NULL.
* **`XmNshadowType`** 

Describes the drawing style for the DrawnButton. This resource can have
the following values:

* **`XmSHADOW_IN`** 

Draws the DrawnButton so that the shadow appears
inset. This means that the bottom shadow visuals and top shadow visuals
are reversed.
* **`XmSHADOW_OUT`** 

Draws the DrawnButton
so that the shadow appears outset.
* **`XmSHADOW_ETCHED_IN`** 

Draws the DrawnButton using a double line. This
gives the effect of a line etched into the window. The thickness of the
double line is equal to the value of`XmNshadowThickness`.
* **`XmSHADOW_ETCHED_OUT`** 

Draws the DrawnButton using a double line. This
gives the effect of a line coming out of the window. The thickness of the
double line is equal to the value of`XmNshadowThickness`.


### Inherited Resources


DrawnButton inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmLabel Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLN/AXmNacceleratorTextXmCAcceleratorTextXmStringNULLN/AXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelStringXmCXmStringXmString"&bsol;0"CSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimension0CSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimension0CSGXmNmarginRightXmCMarginRightDimension0CSGXmNmarginTopXmCMarginTopDimension0CSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNmnemonicXmCMnemonicKeySymNULLN/AXmNmnemonicCharSetXmCMnemonicCharSetStringXmFONTLIST_DEFAULT_TAGN/AXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        Window`window`;
        int`click_count`;
} XmDrawnButtonCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
This is NULL for`XmNresizeCallback`.
* **`window`** 

Is set to the window ID in which the event occurred.
* **`click_count`** 

Contains the number of clicks in the last multiclick sequence
if the`XmNmultiClick`resource is set to`XmMULTICLICK_KEEP`,
otherwise it contains 1.
The activate callback is invoked for each click if`XmNmultiClick`is set to`XmMULTICLICK_KEEP`.

### Translations


XmDrawnButton includes translations from Primitive.
Additional XmDrawnButton translations are
described in the following list.
The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

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


The`XmDrawnButton`action routines are

* **Activate():** 

If`XmNpushButtonEnabled`is True, redraws the shadow in the
unselected state; otherwise, redraws the shadow according to`XmNshadowType`.If the pointer is within the DrawnButton, calls the`XmNactivateCallback`callbacks.
* **Arm():** 

If`XmNpushButtonEnabled`is True, redraws the shadow in the
selected state; otherwise, redraws the shadow according to`XmNshadowType`.Calls the callbacks for`XmNarmCallback`.
* **ArmAndActivate():** 

If`XmNpushButtonEnabled`is True, redraws the shadow in the
selected state; otherwise, redraws the shadow according to`XmNshadowType`.Calls the callbacks for`XmNarmCallback`.

If`XmNpushButtonEnabled`is True, the shadow is redrawn in the
unselected state; otherwise, the shadow is redrawn according to`XmNshadowType`.The callbacks for`XmNactivateCallback`and`XmNdisarmCallback`are called.
These actions happen either immediately or at a later
time.
* **ButtonTakeFocus():** 

Causes the PushButton to take keyboard focus
when`Ctrl<Btn1Down>`is pressed, without activating the widget.
* **Disarm():** 

Marks the DrawnButton as unselected and calls the callbacks for`XmNdisarmCallback`.
* **Help():** 

Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help callbacks for this widget, this action calls the
help callbacks for the nearest ancestor that has them.
* **MultiActivate():** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
increments`click_count`in the callback structure.
If`XmNpushButtonEnabled`is True, this action redraws the shadow in the
unselected state; otherwise, it redraws the shadow according to`XmNshadowType`.If the pointer is within the DrawnButton, this action calls the`XmNactivateCallback`callbacks and calls the
callbacks for`XmNdisarmCallback`.
* **MultiArm():** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`and
if`XmNpushButtonEnabled`is True, this action
redraws the shadow in the selected state;
otherwise, it redraws the shadow according to`XmNshadowType`andcalls the callbacks for`XmNarmCallback`.

### Additional Behavior


This widget has the following additional behavior:

* **EnterWindow:** 

Draws the shadow in its selected state if`XmNpushButtonEnabled`is
True and if the cursor leaves and re-enters the window while`BSelect`is
pressed.
* **LeaveWindow:** 

Draws the shadow in its unselected state if`XmNpushButtonEnabled`is True and if the cursor leaves the window while`BSelect`is pressed.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;,`XmCreateDrawnButton`, &cdeman.XmLabel;,
&cdeman.XmPrimitive;,`XmPushButton`, and &cdeman.XmSeparator;.