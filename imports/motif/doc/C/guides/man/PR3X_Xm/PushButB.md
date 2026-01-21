# XmPushButtonGadget
library call`XmPushButtonGadget`The PushButtonGadget widget classXmPushButtonGadgetwidget classPushButtonGadget&npzwc;#include &lt;Xm/PushBG.h>
## DESCRIPTION


PushButtonGadget issues commands within an application.
It consists of a text label or pixmap surrounded
by a border shadow.
When PushButtonGadget is selected, the shadow changes to give the appearance
that the PushButtonGadget has been pressed in. When PushButtonGadget is
unselected, the shadow changes to give the appearance that the PushButtonGadget is
out.

The default behavior associated with a PushButtonGadget in a menu depends on
the type of menu system in which it resides.
By default,Btn1controls the behavior of the PushButtonGadget.
In addition,Btn3controls the behavior of the PushButtonGadget if
it resides in a PopupMenu system.
The actual mouse button used is determined by its RowColumn parent.

Thickness for a second shadow may be specified with the`XmNshowAsDefault`resource. If it has a nonzero value, the Label's`XmNmarginLeft`,`XmNmarginRight`,`XmNmarginTop`, and`XmNmarginBottom`resources may be modified to accommodate the second shadow.

If an initial value is specified for`XmNarmPixmap`but not for`XmNlabelPixmap`, the`XmNarmPixmap`value is used for`XmNlabelPixmap`.

PushButtonGadget uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits, and
holds the`XmQactivatable`,`XmQTmenuSavvy`, and`XmQTtakesDefault`traits.
### Classes


PushButtonGadget inherits behavior,
resources, and traits from`Object`,`RectObj`,`XmGadget`and`XmLabelGadget`.

The class pointer is`xmPushButtonGadgetClass`.

The class name is`XmPushButtonGadget`.
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

`XmPushButtonGadget Resource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNarmCallbackXmCCallbackXtCallbackListNULLCXmNarmColorXmCArmColorPixeldynamicCSGXmNarmPixmapXmCArmPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNdefaultButtonShadowThicknessXmCdefaultButtonShadowThicknessDimensiondynamicCSGXmNdisarmCallbackXmCCallbackXtCallbackListNULLCXmNfillOnArmXmCFillOnArmBooleanTrueCSGXmNmultiClickXmCMultiClickunsigned chardynamicCSGXmNshowAsDefaultXmCShowAsDefaultDimension0CSG

* **`XmNactivateCallback`** 

Specifies the list of callbacks
that is called when the PushButtonGadget is activated.
It is activated when the user presses
and releases the active mouse button while the
pointer is inside
the PushButtonGadget. Activating PushButtonGadget
also disarms it.
For this callback, the reason is`XmCR_ACTIVATE`.
This callback uses the`XmQTactivatable`trait.
* **`XmNarmCallback`** 

Specifies the list of callbacks
that is called when PushButtonGadget is armed.
It is armed when the user presses the active
mouse button while the pointer is inside the PushButtonGadget.
For this callback, the reason is`XmCR_ARM`.
* **`XmNarmColor`** 

Specifies the color with which to fill the armed button.`XmNfillOnArm`must be set to True for this resource to have an effect.
The default for a color display is a color between the background and the
bottom shadow color. For a monochrome display, the default is set to the
foreground color, and any text in the label appears in the background
color when the button is armed.
* **`XmNarmPixmap`** 

Specifies the pixmap to be used as the button face if`XmNlabeltype`is`XmPIXMAP`and PushButtonGadget is armed.
This resource is disabled when the PushButtonGadget is in a menu.
* **`XmNdefaultButtonShadowThickness`** 

This resource specifies the width of the default button indicator shadow.
If this resource is 0 (zero), the width of the shadow comes from the value of
the`XmNshowAsDefault`resource. If this resource is greater than
zero, the`XmNshowAsDefault`resource is only used to specify whether
this button is the default.
The default value is the initial value of`XmNshowAsDefault`.
* **`XmNdisarmCallback`** 

Specifies the list of callbacks
that is called when the PushButtonGadget is disarmed.
PushButtonGadget is disarmed when the user presses and releases the
active mouse button while the pointer is inside that
gadget.
For this callback, the reason is`XmCR_DISARM`.
* **`XmNfillOnArm`** 

Forces the PushButtonGadget to fill the background of the button with the
color specified by`XmNarmColor`when the button is armed and when this
resource is set to True. If it is False, only the top and bottom
shadow colors are
switched. When the PushButtonGadget is in a menu, this resource is ignored
and assumed to be False.
* **`XmNmultiClick`** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time, and this resource is
set to`XmMULTICLICK_DISCARD`, the second click is not processed.
If this resource is set to`XmMULTICLICK_KEEP`, the event
is processed and`click_count`is incremented in the callback structure.
When the button is in a menu, the default is`XmMULTICLICK_DISCARD`;
otherwise, for a button not in a menu, the default value is`XmMULTICLICK_KEEP`.
* **`XmNshowAsDefault`** 

If`XmNdefaultButtonShadowThickness`is greater than 0 (zero), a value
greater than zero in this resource
specifies to mark this button as the default button.
If`XmNdefaultButtonShadowThickness`is 0, a value greater than
0 in this resource specifies to mark this button as the default
button with the shadow thickness specified by this resource.
The space between
the shadow and the default shadow is equal to the sum of both shadows.
The default value is 0.
When the Display resource`XmNdefaultButtonEmphasis`has a value of`XmEXTERNAL_HIGHLIGHT`(the default), PushButton draws the
location cursor outside of the outer shadow. When this resource has a
value of`XmINTERNAL_HIGHLIGHT`, the highlight is drawn between
the inner and outer shadows.
When this value is not 0, the Label`XmNmarginLeft`,`XmNmarginRight`,`XmNmarginTop`, and`XmNmarginBottom`resources
may be modified to accommodate the second shadow.
This resource is disabled when the PushButton is in a menu.

### Inherited Resources


PushButtonGadget inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmLabelGadget Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLCSGXmNacceleratorTextXmCAcceleratorTextXmStringNULLCSGXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapdynamicCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimensiondynamicCSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimensiondynamicCSGXmNmarginRightXmCMarginRightDimensiondynamicCSGXmNmarginTopXmCMarginTopDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringdynamicCSGXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        int`click_count`;
} XmPushButtonCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`click_count`** 

Valid only when the reason is`XmCR_ACTIVATE`.
It contains the number of clicks in the last multiclick sequence
if the`XmNmultiClick`resource is set to`XmMULTICLICK_KEEP`;
otherwise it contains 1.
The activate callback is invoked for each click if`XmNmultiClick`is set to`XmMULTICLICK_KEEP`.

### Behavior


`XmPushButtonGadget`includes behavior from`XmGadget`.`XmPushButtonGadget`includes menu traversal behavior from`XmLabelGadget`.
Additional behavior for XmPushButtonGadget is described in the following
list.

* **Btn2Down:** 

Drags the contents of a PushButtonGadget label, identified whenBtn2is pressed.
This action is undefined for PushButtonGadgets used in a menu system.
* **Btn1Down:** 

This action arms the PushButtonGadget.

In a menu, this action unposts any menus posted by the PushButtonGadget's
parent menu, disables keyboard traversal for the menu, and enables mouse
traversal for the menu.
It draws the shadow in the armed state.
Unless the button is already armed, it calls the`XmNarmCallback`callbacks.

If the button is not in a menu, this action draws the shadow in the
armed state.
If`XmNfillOnArm`is set
to True, it fills the button with the color specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`, the`XmNarmPixmap`is
used for the button face.
It calls the`XmNarmCallback`callbacks.
* **Btn1`(2+)`:** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
draws the shadow in the armed state.
If the button is not in a menu and if`XmNfillOnArm`is set
to True, it fills the button with the color specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`, the`XmNarmPixmap`is
used for the button face.
This action calls the`XmNarmCallback`callbacks.
* **Btn1Up:** 

In a menu, this action unposts all menus in the menu hierarchy and
activates the PushButtonGadget.
It calls the`XmNactivateCallback`callbacks and then the`XmNdisarmCallback`callbacks.

If the PushButtonGadget is not in a menu, this action draws the shadow in the
unarmed state.
If`XmNfillOnArm`is set to True, the background color reverts to
the unarmed color.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used for the button face.
If the pointer is still within the button,
this action calls the callbacks for`XmNactivateCallback`and`XmNdisarmCallback`.
* **Btn1Up`(2+)`:** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
increments`click_count`in the callback structure and
draws the shadow in the unarmed state.
If the button is not in a menu and if`XmNfillOnArm`is set to True,
the background color reverts to the unarmed color.
If`XmNlabelType`is`XmPIXMAP`,`XmNlabelPixmap`is
used for the button face.
If the pointer is within the PushButtonGadget, this action calls the
callbacks for`XmNactivateCallback`and`XmNdisarmCallback`.
* **KeyosfActivate:** 

In a menu, this action unposts all menus in the menu hierarchy,
unless the button is already armed, and calls the`XmNarmCallback`callbacks, the`XmNactivateCallback`and
the`XmNdisarmCallback`callbacks. Outside a menu,`KActivate`has no effect.
For PushButtonGadgets outside of a menu, if the parent is a manager,
this action passes the event to the parent.
* **KeyosfSelect:** 

In a menu, this action
unposts all menus in the menu hierarchy,
unless the button is already armed,
and calls the`XmNarmCallback`callbacks.
This acton calls the`XmNactivateCallback`and`XmNdisarmCallback`callbacks.

Outside a menu, this action
draws the shadow in the armed state and, if`XmNfillOnArm`is set to True, fills the button with the color
specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`,`XmNarmPixmap`is
used for the button face.
This action calls the`XmNarmCallback`callbacks.

Outside a menu, this action also arranges for the following to happen,
either immediately or at a later time:
the shadow is drawn in the unarmed state and, if`XmNfillOnArm`is set
to True, the background color reverts to the unarmed color.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used for the button face.
The`XmNactivateCallback`and`XmNdisarmCallback`callbacks are
called.
* **KeyosfHelp:** 

In a Pulldown or Popup MenuPane, unposts all menus in the menu hierarchy
and restores keyboard focus to the widget that had the focus before
the menu system was entered.
This action calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **KeyosfCancel:** 

In a toplevel Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and restores keyboard
focus to the widget that had the focus before the MenuBar was
entered.
In other Pulldown MenuPanes, unposts the menu.

In a Popup MenuPane, unposts the menu and restores keyboard focus to the
widget from which the menu was posted.
For a PushButtonGadget outside of a menu, if the parent is a manger, this
action passes the event to the parent.
* **Enter:** 

In a menu, if keyboard traversal is enabled, this action does nothing.
Otherwise, it draws the shadow in the armed state and calls the`XmNarmCallback`callbacks.

If the PushButtonGadget is not in a menu and the cursor leaves and then
reenters the PushButtonGadget while the button is pressed, this
action draws the shadow in the armed state.
If`XmNfillOnArm`is set
to True, it also fills the button with the
color specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`, the`XmNarmPixmap`is
used for the button face.
* **Leave:** 

In a menu, if keyboard traversal is enabled, this action does nothing.
Otherwise, it draws the shadow in the unarmed state and calls the`XmNdisarmCallback`callbacks.

If the PushButtonGadget is not in a menu and the cursor leaves the
PushButtonGadget while the button is pressed, this action draws the
shadow in the unarmed state.
If`XmNfillOnArm`is set to True, the background color reverts to
the unarmed color.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used for the button face.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Object;, &cdeman.RectObj;,
&cdeman.XmCreatePushButtonGadget;,
&cdeman.XmGadget;,
&cdeman.XmLabelGadget;, and &cdeman.XmRowColumn;.