# XmToggleButton
library call`XmToggleButton`The ToggleButton widget classXmToggleButtonwidget classToggleButton#include <Xm/ToggleB.h>
## DESCRIPTION


ToggleButton sets nontransitory state data within an
application. Usually this widget consists of an indicator
(square, diamond, or round)
with either text or a pixmap on one side of it.
However, it can also consist of just text or a pixmap without the indicator.

The toggle graphics display a`1-of-many`,`N-of-many`, or`1-of-many-round`selection state.
When a toggle indicator is displayed, a square indicator shows an`N-of-many`selection state, a diamond-shaped indicator shows a`1-of-many`selection state, and a circle-shaped indicator shows a`1-of-many-round`selection state.

ToggleButton
implies a set or unset state.
In the case of a label and an indicator, an
empty indicator (square, diamond, or round) indicates that ToggleButton
is unset, and a filled indicator shows that it is
set. The indicator may be filled with a check mark, a cross, or the
select color. In the case of a pixmap
toggle, different pixmaps are used to display the set/unset
states.
ToggleButton can also indicate an indeterminate state. In the case of
a label and an indicator, an indeterminate state is indicated by a
stippled flat box.
In the case of a pixmap toggle, a different pixmap is used to display
the indeterminate state.

The default behavior associated with a ToggleButton in a menu depends on
the type of menu system in which it resides.
By default,Btn1controls the behavior of the ToggleButton.
In addition,Btn3controls the behavior of the ToggleButton if
it resides in a PopupMenu system.
The actual mouse button used is determined by its RowColumn parent.

Label's resource`XmNmarginLeft`may
be increased
to accommodate the toggle indicator when it is created.

ToggleButton uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits.
### Classes


ToggleButton inherits behavior, resources, and traits from`Core`,`XmPrimitive`, and`XmLabel`.

The class pointer is`xmToggleButtonWidgetClass`.

The class name is`XmToggleButton`.
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

`XmToggleButton Resource Set``Name``Class``Type``Default``Access`XmNarmCallbackXmCArmCallbackXtCallbackListNULLCXmNdetailShadowThicknessXmCDetailShadowThicknessDimension2CSGXmNdisarmCallbackXmCDisarmCallbackXtCallbackListNULLCXmNfillOnSelectXmCFillOnSelectBooleandynamicCSGXmNindeterminatePixmapXmCIndeterminatePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNindicatorOnXmCIndicatorOnunsigned charXmINDICATOR_FILLCSGXmNindicatorSizeXmCIndicatorSizeDimensiondynamicCSGXmNindicatorTypeXmCIndicatorTypeunsigned chardynamicCSGXmNselectColorXmCSelectColorPixeldynamicCSGXmNselectInsensitivePixmapXmCSelectInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNselectPixmapXmCSelectPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNsetXmCSetunsigned charXmUNSETCSGXmNspacingXmCSpacingDimension4CSGXmNtoggleModeXmCToggleModeunsigned charXmTOGGLE_BOOLEANCSGXmNunselectColorXmCUnselectColorPixeldynamicCSGXmNvalueChangedCallbackXmCValueChangedCallbackXtCallbackListNULLCXmNvisibleWhenOffXmCVisibleWhenOffBooleandynamicCSG

* **`XmNarmCallback`** 

Specifies the list of callbacks called
when the ToggleButton is armed.
To arm this widget, press the active mouse button
while the pointer is inside the ToggleButton.
For this callback, the reason is`XmCR_ARM`.
* **`XmNdetailShadowThickness`** 

Specifies the thickness of the indicator shadow. The
default thickness
is 2 pixels.
* **`XmNdisarmCallback`** 

Specifies the list of callbacks called
when ToggleButton is disarmed.
To disarm this widget, press and release the active
mouse button while the pointer is inside the ToggleButton.
This widget is also disarmed
when the user moves out of the widget
and releases the mouse button when the pointer is outside the widget.
For this callback, the reason is`XmCR_DISARM`.
* **`XmNfillOnSelect`** 

Fills the indicator with the color specified in`XmNselectColor`and switches the top and bottom shadow
colors when set to True. If unset, fills the indicator with the
unselect color. If indeterminate, fills the indicator with
half select color and half unselect color. Otherwise, it switches
only the top
and bottom shadow colors. The default is
True only if a box type of indicator
(such as a check box) is specified, or if the`XmNindicatorType`is a`1-of`type and a toggle indicator is drawn.

If`XmNfillOnSelect`is True,`XmNset`is`XmSET`, and`XmNindicatorOn`is`XmINDICATOR_NONE`the ToggleButton's
background is set to`XmNselectColor`. For the other`XmNindicatorOn`values, only the indicator is filled with`XmNselectColor`.
* **`XmNindeterminateInsensitivePixmap`** 

Specifies the pixmap to be displayed as the button face when
the Label`XmNlableType`resource is`XmPIXMAP`,
the ToggleButton`XmNset`resource is`XmINDETERMINATE`,
and the`Core``XmNsensitive`resource is False.
* **`XmNindeterminatePixmap`** 

Specifies the pixmap to be displayed as the button face when
the Label`XmNlableType`resource is`XmPIXMAP`,
the ToggleButton`XmNset`resource is`XmINDETERMINATE`,
and the`Core``XmNsensitive`resource is True.
* **`XmNindicatorOn`** 

Specifies that if a toggle indicator is to be drawn, it will be drawn
to one side of the toggle
text or pixmap, depending on the`XmNlayoutDirection`resource of
the widget. The default value is`XmINDICATOR_FILL.
Toggles accept the following values:

* **`XmINDICATOR_NONE`** 

No space is allocated
for the indicator, and it is not displayed. Any shadows around the
entire widget are switched when the toggle is selected or unselected.
* **`XmINDICATOR_BOX`** 

The toggle indicator is in the shape of a shadowed box.
* **`XmINDICATOR_FILL`** 

If the value of theXmDisplay XmNenableToggleVisualresource isTrue, the visuals are those of`XmINDICATOR_CHECK_BOX`; ifFalse, the indicator visuals are those of`XmINDICATOR_BOX`.
* **`XmINDICATOR_CHECK`** 

The toggle indicator is in the shape of a checkmark in the
foreground color.
* **`XmINDICATOR_CHECK_BOX`** 

The toggle indicator is in the shape of a checkmark enclosed in a box.
This is the default if the`XmDisplay XmNenableToggleVisual`resource is set.
* **`XmINDICATOR_CROSS_BOX`** 

The toggle indicator is in the shape of a cross enclosed in a box.
* **`XmINDICATOR_CROSS`** 

The toggle indicator is in the shape of a cross.


All ToggleButton checks and crosses should be drawn in the
foreground color.

If this resource is not`XmINDICATOR_NONE`, it will control the
appearance of the toggle visual. If`XmNset`is`XmINDETERMINATE`and`XmNindicatorOn`is not`XmINDICATOR_NONE`, this resource
shows a stippled flat box.
If`XmNset`is`XmINDETERMINATE`,`XmNindicatorOn`is`XmINDICATOR_NONE`, and`XmNtoggleMode`is`XmTOGGLE_INDETERMINATE`, the label and the ToggleButton are stippled with
a combination of the`XmNselectColor`and
the`XmNunselectColor`color,
and the border is flat.
* **`XmNindicatorSize`** 

Sets the size of the indicator.
If no value is specified, the size of the indicator is based on the size
of the label string or pixmap.
If the label string or pixmap changes, the size of the indicator is
recomputed based on the size of the label string or pixmap.
Once a value has been specified for`XmNindicatorSize`, the
indicator has that size, regardless of the size of the label string or
pixmap, until a new value is specified.
The size of indicators inside menus may differ from those outside of menus.
Note that a change in this resource may also cause a change in the
values of the inherited resources`XmNmarginTop`,`XmNmarginBottom`, and`XmNmarginLeft`.
* **`XmNindicatorType`** 

Specifies if the indicator is a`1-of`or`N-of`indicator. For the`1-of`indicator, the
value can be`XmONE_OF_MANY`,`XmONE_OF_MANY_ROUND`, or`XmONE_OF_MANY_DIAMOND`.
For the`N-of`indicator,
the value is`XmN_OF_MANY`.
This value specifies only the visuals and does not enforce the
behavior. When the ToggleButton is in a radio box, the default is`XmONE_OF_MANY`; otherwise,
the default is`XmN_OF_MANY`. Legal values
are:

* **`XmONE_OF_MANY`** 

When the Display`XmNenableToggleVisualresource is set,
indicators are drawn with the same appearance as`XmONE_OF_MANY_ROUND;
otherwise, they appear the same as`XmONE_OF_MANY_DIAMOND.
* **`XmN_OF_MANY`** 

The indicators are drawn as specified by the`XmNindicatorOnresource.
* **`XmONE_OF_MANY_ROUND`** 

A shadowed circle.
* **`XmONE_OF_MANY_DIAMOND`** 

A shadowed diamond.

* **`XmNselectColor`** 

Allows the application to specify what color fills
the center of the square, diamond-shaped, or round indicator when it is set.
If this color is the same as either the top or the bottom shadow color of the
indicator, a one-pixel-wide margin is left between the shadows and the fill;
otherwise, it is filled completely.
The results of this resource depend on the value of the Display
resource`XmNenableToggleColor`. A value of True causes the fill
color to use the`XmHIGHLIGHT_COLOR`color by default. A value of
False causes the fill
color to use the background color.
This resource's default for a color display is a color between the background
and the bottom shadow color. For a monochrome display, the default is set to
the foreground color. To set the background of the button to`XmNselectColor`when`XmNindicatorOn`is`XmINDICATOR_NONE`,
the value of`XmNfillOnSelect`must be explicitly set to True.This resource is also used as the background color when all of the following conditions
are met: the button is armed in a menu, theXmNenableEtchedInMenuresource isTrue,
theXmNindicatorOnresource isFalse, and theXmNfillOnSelectresource isTrue.

This resource can take the following values:

* **`XmDEFAULT_SELECT_COLOR`** 

Is the same as the current dynamic default, which is a color between
the background and the bottom shadow color.
* **`XmREVERSED_GROUND_COLORS`** 

Forces the select color to the
foreground color and causes the default color of any text rendered over the
select color to be in the background color.
* **`XmHIGHLIGHT_COLOR`** 

Forces the fill color to use the highlight color.

* **`XmNselectInsensitivePixmap`** 

Specifies a pixmap used as the button face when the ToggleButton is selected,
the button is insensitive, and the Label resource`XmNlabelType`is set to`XmPIXMAP`.
If the ToggleButton is unselected and the button is insensitive,
the pixmap in`XmNlabelInsensitivePixmap`is used as the button face.
If no value is specified for`XmNlabelInsensitivePixmap`, that
resource is set to the value specified for`XmNselectInsensitivePixmap`.
* **`XmNselectPixmap`** 

Specifies the pixmap to be used as the button
face when`XmNlabelType`is`XmPIXMAP`and
the ToggleButton is selected.
When the ToggleButton is unselected,
the pixmap specified in the Label's`XmNlabelPixmap`is used.
If no value is specified for`XmNlabelPixmap`, that resource is set
to the value specified for`XmNselectPixmap`.
* **`XmNset`** 

Represents the state of the ToggleButton.
A value of`XmUNSET`indicates that the ToggleButton is not set.
A value of`XmSET`indicates that the ToggleButton is set.
A value of`XmINDETERMINATE`indicates that the
ToggleButton is in an indeterminate state (neither set nor unset).
The ToggleButton states cycle through in the order of`XmSET`,`XmINDETERMINATE`(if`XmNtoggleMode`is set to`XmTOGGLE_INDETERMINATE`), and`XmUNSET`, and then
back around to`XmSET`. If`XmNtoggleMode`is
set to`XmTOGGLE_BOOLEAN`, then the ToggleButton states cycle
through in the order of`XmSET`, then`XmUNSET`, and then
back around to`XmSET`.
Setting this resource sets the state of the
ToggleButton.
* **`XmNspacing`** 

Specifies the amount of spacing between the toggle indicator and the
toggle label (text or pixmap).
* **`XmNtoggleMode`** 

Specifies the mode of the ToggleButton as either`XmTOGGLE_BOOLEAN`or`XmTOGGLE_INDETERMINATE`. The`XmTOGGLE_INDETERMINATE`value allows the`XmNset`resource to
be able to accept the values`XmINDETERMINATE`,`XmSET`, and`XmUNSET`. The`XmNtoggleMode`resource is forced to`XmTOGGLE_BOOLEAN`if the toggle is in an`XmRowColumn`widget
whose radio behavior is`XmONE_OF_MANY`. In`XmTOGGLE_BOOLEAN`mode, the`XmNset`resource can only accept`XmSET`and`XmUNSET`.
* **`XmNunselectColor`** 

Allows the application to specify what color fills
the center of the square, diamond-shaped, or round indicator when it
is not set.
If this color is the same as either the top or the bottom shadow color of the
indicator, a one-pixel-wide margin is left between the shadows and the fill;
otherwise, it is filled completely.
This resource's default for a color display is`XmNbackground`.
For a monochrome display, the default is set to
the background color. To set the background of the button to`XmNunselectColor`when`XmNindicatorOn`is`XmINDICATOR_NONE`, the value of`XmNfillOnSelect`must be explicitly set to True. This resource
acts like the`XmNselectColor`resource, but for the case when`XmNset`is`XmUNSET`.
* **`XmNvalueChangedCallback`** 

Specifies the list of callbacks called
when the ToggleButton value
is changed. To change the value,
press and release the active mouse button while the pointer
is inside the ToggleButton. This action
also causes this widget to be disarmed.
For this callback, the reason is`XmCR_VALUE_CHANGED`.
* **`XmNvisibleWhenOff`** 

Indicates that the toggle indicator is visible in the unselected state when
the Boolean value is True.
When the ToggleButton is in a menu, the default value is False.
When the ToggleButton is in a RadioBox, the default value is True.

### Inherited Resources


ToggleButton inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmLabel Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLCSGXmNacceleratorTextXmCAcceleratorTextXmStringNULLCSGXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimensiondynamicCSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimensiondynamicCSGXmNmarginRightXmCMarginRightDimension0CSGXmNmarginTopXmCMarginTopDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringXmFONTLIST_DEFAULT_TAGCSGXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        int`set`;
} XmToggleButtonCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`set`** 

Reflects the ToggleButton's state, either`XmSET`(selected),`XmUNSET`(unselected), or`XmINDETERMINATE`(neither).
Note that the reported state is the state that the ToggleButton
is in after the`event`has been processed. For example,
suppose that a user clicks on a ToggleButton to change it from
the unselected state to the selected state. In this case,
ToggleButton changes the value of`set`from`XmUNSET`to`XmSET`prior to calling the callback.

### Translations


`XmToggleButton`includes translations fromPrimitive.
Additional`XmToggleButton`translations for buttons not in a
menu system are described in the following list.

Note that altering translations in`#override`or`#augment`mode is undefined.

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
* **`&ap;c`Btn1Up:** 

Select() Disarm()
* **Btn2Down:** 

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


`XmToggleButton`inherits menu traversal translations
from`XmLabel`.
Additional`XmToggleButton`translations for`ToggleButtons`in a
menu system are described in the following list.
In a Popup menu system,Btn3also performs theBtn1actions.

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
* **`:`KeyosfHelp:** 

Help()
* **`:`KeyosfCancel:** 

MenuEscape()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

ArmAndActivate()
* **`&ap;s &ap;m &ap;a`Key`space`:** 

ArmAndActivate()

### Action Routines


The`XmToggleButton`action routines are

* **Arm():** 

If the button was previously unset, this action does the following:
if`XmNindicatorOn`is True, it draws the indicator shadow so that
the indicator looks pressed; if`XmNfillOnSelect`is True, it fills
the indicator with the color specified by`XmNselectColor`.
If`XmNindicatorOn`is False, it draws the
button shadow so
that the
button looks pressed.
If`XmNlabelType`is`XmPIXMAP`, the`XmNselectPixmap`is
used as the button face.
This action calls the`XmNarmCallback`callbacks.

If the button was previously set, this action does the following:
if both`XmNindicatorOn`and`XmNvisibleWhenOff`are True, it
draws the indicator shadow so that the indicator looks raised; if`XmNfillOnSelect`is True, it fills the indicator with the
background color.
If`XmNindicatorOn`is False, it draws the button shadow
so that the button looks raised.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used as the button face.
This action calls the`XmNarmCallback`callbacks.
* **ArmAndActivate():** 

If the ToggleButton was previously set, unsets it; if the ToggleButton
was previously unset, sets it.

In a menu, this action
unposts all menus in the menu hierarchy.
Unless the button is already armed,
it calls the`XmNarmCallback`callbacks.
This action calls the`XmNvalueChangedCallback`and`XmNdisarmCallback`callbacks.

Outside a menu, if the button was previously unset, this action does the
following:
if`XmNindicatorOn`is True, it draws the indicator shadow so that
the indicator looks pressed; if`XmNfillOnSelect`is True, it fills
the indicator with the color specified by`XmNselectColor`.
If`XmNindicatorOn`is False, it draws the button shadow so that the
button looks pressed.
If`XmNlabelType`is`XmPIXMAP`, the`XmNselectPixmap`is
used as the button face.
This action calls the`XmNarmCallback`,`XmNvalueChangedCallback`, and`XmNdisarmCallback`callbacks.

Outside a menu, if the button was previously set, this action does the
following:
if both`XmNindicatorOn`and`XmNvisibleWhenOff`are True, it
draws the indicator shadow so that the indicator looks raised; if`XmNfillOnSelect`is True, it fills the indicator with the
background color.
If`XmNindicatorOn`is False, it draws the button shadow so that the
button looks raised.
If`XmNlabelType`is`XmPIXMAP`, the`XmNlabelPixmap`is
used as the button face.
This action calls the`XmNarmCallback`,`XmNvalueChangedCallback`, and`XmNdisarmCallback`callbacks.
* **BtnDown():** 

This action unposts any menus posted by the ToggleButton's parent menu,
disables keyboard traversal for the menu, and enables mouse traversal
for the menu.
It draws the shadow in the armed state
and, unless the button is already armed, calls the`XmNarmCallback`callbacks.
* **BtnUp():** 

This action unposts all menus in the menu hierarchy.
If the ToggleButton was previously set, unsets it; if the ToggleButton
was previously unset, sets it.
It calls the`XmNvalueChangedCallback`callbacks and then the`XmNdisarmCallback`callbacks.
* **ButtonTakeFocus():** 

Causes the ToggleButton to take keyboard focus
when`Ctrl<Btn1Down>`is pressed, without activating the widget.
* **Disarm():** 

Calls the callbacks for`XmNdisarmCallback`.
* **Help():** 

In a Pulldown or Popup MenuPane, unposts all menus in the menu hierarchy
and restores keyboard focus to the widget that had the focus before
the menu system was entered.
Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help callbacks for this widget, this action calls the
help callbacks for the nearest ancestor that has them.
* **MenuShellPopdownOne():** 

In a toplevel Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and restores keyboard
focus to the widget that had the focus before the MenuBar was
entered.
In other Pulldown MenuPanes, unposts the menu.

In a Popup MenuPane, unposts the menu and restores keyboard focus to the
widget from which the menu was posted.
* **ProcessDrag():** 

Drags the contents of a ToggleButton label, identified when`BTransfer`is pressed.
This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures, possibly
multiple times, for the_MOTIF_DROPselection.
This action is undefined for ToggleButtons used in a menu system.
* **Select():** 

If the pointer is within the button, takes the following actions:
If the button was previously unset, sets it; if the button was
previously set, unsets it.
This action calls the`XmNvalueChangedCallback`callbacks.

### Additional Behavior


This widget has the following additional behavior:

* **EnterWindow:** 

In a menu, if keyboard traversal is enabled, this action does nothing.
Otherwise, it draws the shadow in the armed state and calls the`XmNarmCallback`callbacks.

If the ToggleButton is not in a menu and the cursor leaves and then
reenters the ToggleButton's window while the button is pressed, this
action restores the button's armed appearance.
* **LeaveWindow:** 

In a menu, if keyboard traversal is enabled, this action does nothing.
Otherwise, it draws the shadow in the unarmed state and calls the`XmNdisarmCallback`callbacks.

If the ToggleButton is not in a menu and the cursor leaves the
ToggleButton's window while the button is pressed, this action restores
the button's unarmed appearance.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;,
&cdeman.XmCreateRadioBox;,
&cdeman.XmCreateToggleButton;, &cdeman.XmLabel;,
&cdeman.XmPrimitive;, &cdeman.XmRowColumn;, &cdeman.XmToggleButtonGetState;,
and &cdeman.XmToggleButtonSetState;.