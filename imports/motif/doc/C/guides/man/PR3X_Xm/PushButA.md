# XmPushButton
library call`XmPushButton`The PushButton widget classXmPushButtonwidget classPushButton&npzwc;#include &lt;Xm/PushB.h>
## DESCRIPTION


PushButton issues commands within an application.
It consists of a text label or pixmap surrounded
by a border shadow.
When a PushButton is selected, the shadow changes to give the appearance
that it has been pressed in. When a PushButton is
unselected, the shadow changes to give the appearance that it is out.

The default behavior associated with a PushButton in a menu depends on
the type of menu system in which it resides.
By default,Btn1controls the behavior of the PushButton.
In addition,Btn3controls the behavior of the PushButton if
it resides in a PopupMenu system.
The actual mouse button used is determined by its RowColumn parent.

Thickness for a second shadow, used when the PushButton is the default
button, may be specified with the`XmNshowAsDefault`resource. If it has a nonzero value, the Label's
resources`XmNmarginLeft`,`XmNmarginRight`,`XmNmarginTop`, and`XmNmarginBottom`may be modified to accommodate the second shadow.

If an initial value is specified for`XmNarmPixmap`but not for`XmNlabelPixmap`, the`XmNarmPixmap`value is used for`XmNlabelPixmap`.

PushButton uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits, and
holds the`XmQactivatable`,`XmQTmenuSavvy`, and`XmQTtakesDefault`traits.
### Classes


PushButton inherits behavior, resources, and traits from`Core`,`XmPrimitive`,
and`XmLabel`.

The class pointer is`xmPushButtonWidgetClass`.

The class name is`XmPushButton`.
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

`XmPushButton Resource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNarmCallbackXmCCallbackXtCallbackListNULLCXmNarmColorXmCArmColorPixeldynamicCSGXmNarmPixmapXmCArmPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNdefaultButtonShadowThicknessXmCDefaultButtonShadowThicknessDimensiondynamicCSGXmNdisarmCallbackXmCCallbackXtCallbackListNULLCXmNfillOnArmXmCFillOnArmBooleanTrueCSGXmNmultiClickXmCMultiClickunsigned chardynamicCSGXmNshowAsDefaultXmCShowAsDefaultDimension0CSG

* **`XmNactivateCallback`** 

Specifies the list of callbacks that is called when PushButton is activated.
PushButton is activated when the user presses
and releases the active mouse button
while the pointer is inside that widget. Activating the PushButton
also disarms it.
For this callback, the reason is`XmCR_ACTIVATE`.
This callback uses the`XmQTactivatable`trait.
* **`XmNarmCallback`** 

Specifies the list of callbacks that is called when PushButton is armed.
PushButton is armed when the user presses
the active mouse button while the pointer is inside that
widget.
For this callback, the reason is`XmCR_ARM`.
* **`XmNarmColor`** 

Specifies the color with which to fill the armed button.`XmNfillOnArm`must be set to True for this resource to have an effect.
The default for a color display is a color between the background and the
bottom shadow color. For a monochrome display, the default is set to the
foreground color, and any text in the label appears in the background
color when the button is armed.This resource is also used when the button is armed in a menu
and theXmNenableEtchedInMenuresource isTrue.
* **`XmNarmPixmap`** 

Specifies the pixmap to be used as the button face if`XmNlabelType`is`XmPIXMAP`and PushButton is armed. This resource is disabled when the
PushButton is in a menu.
* **`XmNdefaultButtonShadowThickness`** 

This resource specifies the width of the default button indicator shadow.
If this resource is 0 (zero), the width of the shadow comes from the value of
the`XmNshowAsDefault`resource. If this resource is greater than
0, the`XmNshowAsDefault`resource is only used to specify whether
this button is the default.
The default value is the initial value of`XmNshowAsDefault`.
* **`XmNdisarmCallback`** 

Specifies the list of callbacks
that is called when PushButton is disarmed. PushButton is disarmed when
the user presses and releases the active mouse button
while the pointer is inside that widget. For this callback, the reason is`XmCR_DISARM`.
* **`XmNfillOnArm`** 

Forces the PushButton to fill the background of the button with the
color specified by`XmNarmColor`when the button is armed and when this
resource is set to True. If False, only the top and bottom
shadow colors are switched.
When the PushButton is in a menu, this resource is ignored and assumed to be
False.
* **`XmNmultiClick`** 

If a button click is followed by another button click within the time
span specified by the display's multiclick time, and this resource is
set to`XmMULTICLICK_DISCARD`, do not process the second click.
If this resource is set to`XmMULTICLICK_KEEP`, process the event
and increment`click_count`in the callback structure.
When the button is in a menu, the default is`XmMULTICLICK_DISCARD`;
otherwise, for a button not in a menu,`XmMULTICLICK_KEEP`is
the default value.
* **`XmNshowAsDefault`** 

If`XmNdefaultButtonShadowThickness`is greater than 0 (zero), a value
greater than 0 in this resource
specifies to mark this button as the default button.
If`XmNdefaultButtonShadowThickness`is 0, a value greater than
0 in this resource specifies to mark this button as the default
button with the shadow thickness specified by this resource.
When the Display resource`XmNdefaultButtonEmphasis`has a value of`XmEXTERNAL_HIGHLIGHT`(the default), PushButton draws the
location cursor outside of the outer shadow. When this resource has a
value of`XmINTERNAL_HIGHLIGHT`, the highlight is drawn between
the inner and outer shadows.
The space between
the shadow and the default shadow is equal to the sum of both shadows.
The default value is 0. When this value is not 0, the Label
resources`XmNmarginLeft`,`XmNmarginRight`,`XmNmarginTop`, and`XmNmarginBottom`may be modified to accommodate the second shadow.
This resource is disabled when the PushButton is in a menu.

### Inherited Resources


PushButton inherits behavior and resources from the
superclasses described the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmLabel Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLCSGXmNacceleratorTextXmCAcceleratorTextXmStringNULLCSGXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapdynamicCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimensiondynamicCSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimensiondynamicCSGXmNmarginRightXmCMarginRightDimensiondynamicCSGXmNmarginTopXmCMarginTopDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringXmFONTLIST_DEFAULT_TAGCSGXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
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

This value is valid only when the reason is`XmCR_ACTIVATE`.
It contains the number of clicks in the last multiclick sequence
if the`XmNmultiClick`resource is set to`XmMULTICLICK_KEEP`,
otherwise it contains 1.
The activate callback is invoked for each click if`XmNmultiClick`is set to`XmMULTICLICK_KEEP`.

### Translations


`XmPushButton`includes translations fromPrimitive.

Note that altering translations in`#override`or`#augment`mode is undefined.

Additional`XmPushButton`translations for`XmPushButtons`not in a
menu system are described in the following list.

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
* **`&ap;c`Btn2Down:** 

ProcessDrag()
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


`XmPushButton`inherits menu traversal translations
from`XmLabel`.
Additional XmPushButton translations for PushButtons in a
menu system are described in the following list.
In a Popup menu system,Btn3also performs theBtn1actions.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **Btn2Down:** 

ProcessDrag()
* **`c<Btn1Down>`:** 

MenuButtonTakeFocus()
* **`c<Btn1Up>`:** 

MenuButtonTakeFocusUp()
* **`&ap;c`BtnDown:** 

BtnDown()
* **`&ap;c`BtnUp:** 

BtnUp()
* **`:`KeyosfSelect:** 

ArmAndActivate()
* **`:`KeyosfActivate:** 

ArmAndActivate()
* **`:`KeyosfCancel:** 

MenuEscape()
* **`:`KeyosfHelp:** 

Help()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

ArmAndActivate()
* **`&ap;s &ap;m &ap;a`Key`space`:** 

ArmAndActivate()

### Action Routines


The`XmPushButton`action routines are

* **Activate():** 

This action draws the shadow in the unarmed
state.
If the button is not in a menu and if`XmNfillOnArm`is set to True,
the background color reverts to the unarmed color.
If`XmNlabelType`is`XmPIXMAP`,`XmNlabelPixmap`is
used for the button face.
If the pointer is still within the button,
this action calls the callbacks for`XmNactivateCallback`.
* **Arm():** 

This action arms the PushButton.
It draws the shadow in the armed state.
If the button is not in a menu and if`XmNfillOnArm`is set
to True, it fills the button with the color specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`, the`XmNarmPixmap`is
used for the button face.
It calls the`XmNarmCallback`callbacks.
* **ArmAndActivate():** 

In a menu, unposts all menus in the menu hierarchy and,
unless the button is already armed,
calls the`XmNarmCallback`callbacks.
This action calls the`XmNactivateCallback`and`XmNdisarmCallback`callbacks.

Outside a menu, draws the shadow in the armed state and, if`XmNfillOnArm`is set to True, fills the button with the color
specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`,`XmNarmPixmap`is
used for the button face.
This action calls the`XmNarmCallback`callbacks.

Outside a menu, this action also arranges for the following to happen,
either immediately or at a later time:
the shadow is drawn in the unarmed state and, if`XmNfillOnArm`is set
to True, the background color reverts to the unarmed color.
If`XmNlabelType`is`XmPIXMAP`,`XmNlabelPixmap`is
used for the button face.
The`XmNactivateCallback`and`XmNdisarmCallback`callbacks are
called.
* **BtnDown():** 

This action unposts any menus posted by the PushButton's parent menu,
disables keyboard traversal for the menu, and enables mouse traversal
for the menu.
It draws the shadow in the armed state
and, unless the button is already armed, calls the`XmNarmCallback`callbacks.
* **BtnUp():** 

This action unposts all menus in the menu hierarchy and activates the
PushButton.
It calls the`XmNactivateCallback`callbacks and then the`XmNdisarmCallback`callbacks.
* **ButtonTakeFocus():** 

Causes the PushButton to take keyboard focus
when`Ctrl<Btn1Down>`is pressed, without activating the widget.
* **Disarm():** 

Calls the callbacks for`XmNdisarmCallback`.
* **Help():** 

In a Pulldown or Popup MenuPane, unposts all menus in the menu hierarchy
and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard focus to the widget that had the focus before
the menu system was entered.
This action calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **MenuShellPopdownOne():** 

In a top-level Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar; and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard
focus to the widget that had the focus before the MenuBar was
entered.
In other Pulldown MenuPanes, it unposts the menu.

In a Popup MenuPane, this action
unposts the menu and restores keyboard focus to the
widget from which the menu was posted.
* **MultiActivate():** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
increments`click_count`in the callback structure and
draws the shadow in the unarmed state.
If the button is not in a menu and if`XmNfillOnArm`is set to True,
the background color reverts to the unarmed color.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used for the button face.
If the pointer is within the PushButton, calls the
callbacks for`XmNactivateCallback`and`XmNdisarmCallback`.
* **MultiArm():** 

If`XmNmultiClick`is`XmMULTICLICK_DISCARD`, this action does
nothing.

If`XmNmultiClick`is`XmMULTICLICK_KEEP`, this action
draws the shadow in the armed state.
If the button is not in a menu and if`XmNfillOnArm`is set
to True, this action fills the button with the color specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`, the`XmNarmPixmap`is
used for the button face.
This action calls the`XmNarmCallback`callbacks.
* **ProcessDrag():** 

Drags the contents of a PushButton label, identified when`BTransfer`is pressed.
This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures, possibly
multiple times, for the_MOTIF_DROPselection.
This action is undefined for PushButtons used in a menu system.

### Additional Behavior


This widget has the following additional behavior:

* **EnterWindow:** 

In a menu, if keyboard traversal is enabled, this action does nothing.
Otherwise, it draws the shadow in the armed state and calls the`XmNarmCallback`callbacks.

If the PushButton is not in a menu and the cursor leaves and then
reenters the PushButton's window while the button is pressed, this
action draws the shadow in the armed state.
If`XmNfillOnArm`is set to True, it also fills the button with the
color specified by`XmNarmColor`.
If`XmNlabelType`is`XmPIXMAP`,`XmNarmPixmap`is
used for the button face.
* **LeaveWindow:** 

In a menu, if keyboard traversal is enabled, this action does nothing.
Otherwise, it draws the shadow in the unarmed state and calls the`XmNdisarmCallback`callbacks.

If the PushButton is not in a menu and the cursor leaves the
PushButton's window while the button is pressed, this action draws the
shadow in the unarmed state.
If`XmNfillOnArm`is set to True, the background color reverts to
the unarmed color.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used for the button face.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;,
&cdeman.XmCreatePushButton;, &cdeman.XmLabel;,
&cdeman.XmPrimitive;, and &cdeman.XmRowColumn;.