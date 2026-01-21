# XmScrollBar
library call`XmScrollBar`The ScrollBar widget classXmScrollBarwidget classScrollBar&npzwc;#include &lt;Xm/ScrollBar.h>
## DESCRIPTION


The ScrollBar widget allows the user to view data that is too large to be
displayed all at once. ScrollBars are usually located inside a
ScrolledWindow and adjacent to
the widget that contains the data to be viewed. When the user interacts
with the ScrollBar, the data within the other widget scrolls.

A ScrollBar consists of two arrows placed at each end of a rectangle. The
rectangle is called the scroll region. A smaller rectangle, called the
slider, is placed within the scroll region. The data is scrolled by
clicking either arrow, selecting on the scroll region, or dragging the slider.
When an arrow is selected, the slider within the scroll region is
moved in the direction of the arrow by an amount supplied by the
application. If the mouse button is held down, the slider continues to
move at a constant rate.

The ratio of the slider size to the scroll region size typically
corresponds to the
relationship between the size of the visible data and the total size
of the data. For example, if 10 percent of the data is visible, the
slider typically occupies 10 percent of the scroll region. This provides the
user with a visual clue to the size of the invisible data.

If the ScrollBar parent is an automatic ScrolledWindow, the`XmNtraversalOn`default is True. Otherwise, the default is False.

ScrollBar holds the`XmQTnavigator`traits.
### Classes


ScrollBar inherits behavior, resources, and traits from the`Core`and`XmPrimitive`classes.

The class pointer is`xmScrollBarWidgetClass`.

The class name is`XmScrollBar`.
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

`XmScrollBar Resource Set``Name``Class``Type``Default``Access`XmNdecrementCallbackXmCCallbackXtCallbackListNULLCXmNdragCallbackXmCCallbackXtCallbackListNULLCXmNeditableXmCEditableBooleanTrueCSGXmNincrementXmCIncrementint1CSGXmNincrementCallbackXmCCallbackXtCallbackListNULLCXmNinitialDelayXmCInitialDelayint250 msCSGXmNmaximumXmCMaximumint100CSGXmNminimumXmCMinimumint0CSGXmNorientationXmCOrientationunsigned charXmVERTICALCSGXmNpageDecrementCallbackXmCCallbackXtCallbackListNULLCXmNpageIncrementXmCPageIncrementint10CSGXmNpageIncrementCallbackXmCCallbackXtCallbackListNULLCXmNprocessingDirectionXmCProcessingDirectionunsigned chardynamicCSGXmNrepeatDelayXmCRepeatDelayint50 msCSGXmNshowArrowsXmCShowArrowsXtEnumXmEACH_SIDECSGXmNsliderSizeXmCSliderSizeintdynamicCSGXmNsliderMarkXmCSliderMarkXtEnumdynamicCSGXmNsliderVisualXmCSliderVisualXtEnumXmSHADOWEDCSGXmNslidingModeXmCSlidingModeXtEnumXmSLIDERCSGXmNsnapBackMultipleXmCSnapBackMultipleunsigned shortMaxValueCSGXmNtoBottomCallbackXmCCallbackXtCallbackListNULLCXmNtoTopCallbackXmCCallbackXtCallbackListNULLCXmNtroughColorXmCTroughColorPixeldynamicCSGXmNvalueXmCValueintdynamicCSGXmNvalueChangedCallbackXmCCallbackXtCallbackListNULLC

* **`XmNdecrementCallback`** 

Specifies the list of callbacks that is called when the user takes an
action that moves the ScrollBar by one increment and the value
decreases.
The reason passed to the callback is`XmCR_DECREMENT`.
* **`XmNdragCallback`** 

Specifies the list of callbacks that is called
on each incremental change of position
when the slider is being dragged. The reason sent by the
callback is`XmCR_DRAG`.
* **`XmNeditable`** 

Specifies how ScrollBar will react to user input. This
resource can be True or False values, as follows:

* **`True`** 

Allows the scrollbar to be sensitive to user input. This is the
default value.
* **`False`** 

Makes the Scale scrollbar insensitive to user input. The visual is not
greyed out. This value would mostly be used in`XmTHERMOMETER`mode.
When`XmNeditable`is used on a widget
it sets the dropsite to`XmDROP_SITE_ACTIVE`.
* **`XmNincrement`** 

Specifies the amount by which the value increases or decreases when the
user takes an action that moves the slider by one increment.
The actual change in value is the lesser of`XmNincrement`and
(previous`XmNvalue`&ensp;&minus;`XmNminimum`) when the slider moves to
the end of the ScrollBar with the minimum value,
and the lesser of`XmNincrement`and (`XmNmaximum`&minus;`XmNsliderSize`&ensp;&minus;
previous`XmNvalue`) when the slider moves to the end of the
ScrollBar with the maximum value.
The value of this resource must be greater than 0 (zero).
* **`XmNincrementCallback`** 

Specifies the list of callbacks that is called when the user takes an
action that moves the ScrollBar by one increment and the value
increases.
The reason passed to the callback is`XmCR_INCREMENT`.
* **`XmNinitialDelay`** 

Specifies the amount of time in milliseconds to wait before starting
continuous
slider movement while a button is pressed in an arrow or the scroll region.
The value of this resource must be greater than 0 (zero).
* **`XmNmaximum`** 

Specifies the slider's maximum value.`XmNmaximum`must be greater than`XmNminimum`.
* **`XmNminimum`** 

Specifies the slider's minimum value.`XmNmaximum`must be greater than`XmNminimum`.
* **`XmNorientation`** 

Specifies whether the ScrollBar is displayed vertically or horizontally.
This resource can have values of`XmVERTICAL`and`XmHORIZONTAL`.
* **`XmNpageDecrementCallback`** 

Specifies the list of callbacks that is called when the user takes an
action that moves the ScrollBar by one page increment and the value
decreases.
The reason passed to the callback is`XmCR_PAGE_DECREMENT`.
* **`XmNpageIncrement`** 

Specifies the amount by which the value increases or decreases when
the user takes an action that moves the slider by one page increment.
The actual change in value is the lesser of`XmNpageIncrement`and
(previous`XmNvalue`&ensp;&minus;`XmNminimum`) when the slider moves to
the end of the ScrollBar with the
minimum value, and the lesser of`XmNpageIncrement`and (`XmNmaximum`&minus;`XmNsliderSize`&ensp;&minus;
previous`XmNvalue`) when the slider moves to the end of the
ScrollBar with the maximum value.
The value of this resource must be greater than 0 (zero).
* **`XmNpageIncrementCallback`** 

Specifies the list of callbacks that is called when the user takes an
action that moves the ScrollBar by one page increment and the value
increases.
The reason passed to the callback is`XmCR_PAGE_INCREMENT`.
* **`XmNprocessingDirection`** 

Specifies whether the value for`XmNmaximum`should be on the right or
left side of`XmNminimum`for horizontal ScrollBars
or above or below`XmNminimum`for vertical ScrollBars.
This resource can have values of`XmMAX_ON_TOP, XmMAX_ON_BOTTOM,
XmMAX_ON_LEFT`, and`XmMAX_ON_RIGHT`.
If the ScrollBar is oriented vertically,
the default value is`XmMAX_ON_BOTTOM`.
If the ScrollBar is oriented horizontally,
the default value
depends on the`XmNlayoutDirection`resource of the widget.
* **`XmNrepeatDelay`** 

Specifies the amount of time in milliseconds to wait between
subsequent slider movements after the`XmNinitialDelay`has been processed.
The value of this resource must be greater than 0 (zero).
* **`XmNshowArrows`** 

Specifies whether the arrows are
displayed and how they are to be displayed. This resource can take the
following values:

* **`XmEACH_SIDE`** 

Indicates that one arrow is displayed on each end of the ScrollBar
slider. This corresponds to a value of True in previous releases.
* **`XmMAX_SIDE`** 

Indicates that both arrows are displayed on the`XmNmaximum`side of
the ScrollBar slider.
* **`XmMIN_SIDE`** 

Indicates that both arrows are displayed on the`XmNminimum`side of
the ScrollBar slider.
* **`XmNONE`** 

Indicates that no arrows are displayed. This corresponds to a value
of False in previous releases.


`XmEACH_SIDE`is the default value.
* **`XmNsliderMark`** 

Specifies the shape the slider is to be displayed in. This resource
can take the following values:

* **`XmETCHED_LINE`** 

Specifies the slider as an etched line.
* **`XmNONE`** 

Specifies the slider as a foregrounded rectangle. This is the default
for a regular slider.
* **`XmROUND_MARK`** 

Specifies the slider as a shadowed circle. This is the default when
the slider is a thermometer.
* **`XmTHUMB_MARK`** 

Specifies the slider as a series of three etched lines centered in the
middle of the slider.

* **`XmNslidingMode`** 

Specifies the mode the slider works in. There are two possible modes:

* **`XmSLIDER`** 

Allows the slider to move freely between the minimum and maximum ends
of the scale. This is the default value.
* **`XmTHERMOMETER`** 

Forces the slider to be anchored to one side of the trough area.

* **`XmNsliderSize`** 

Specifies the length of the slider between the values of 1 and
(`XmNmaximum`&ensp;&minus;`XmNminimum`).
The value is constrained to be within these inclusive bounds.
The default value is (`XmNmaximum`&ensp;&minus;`XmNminimum`) divided by
10, with a minimum of 1.
* **`XmNsliderVisual`** 

Specifies the color of the slider visual. This resource can take
the following values:

* **`XmBACKGROUND_COLOR`** 

Specifies that the slider visual is in the background color.
* **`XmFOREGROUND_COLOR`** 

Specifies that the slider visual is in the foreground color.
* **`XmSHADOWED_BACKGROUND`** 

Specifies that the slider visual is in the background color, with a shadow.
This is the
default for a regular slider.
* **`XmTROUGH_COLOR`** 

Specifies that the slider visual is in the trough color. This is the
default when the slider is a thermometer.

* **`XmNsnapBackMultiple`** 

Specifies the distance over which the scrollbar slider snaps back to its
original position when the user drags the mouse outside the ScrollBar
edge. This distance is defined in terms of multiples of
the width of the slider. For example, a multiple of 0 (zero) causes the
slider to snap back as soon as the pointer moves out of the ScrollBar
frame, a multiple of 1 causes the slider to snap back as soon as the
pointer moves beyond 1 ScrollBar width of the ScrollBar edge.
Whenever the slider snaps back, the ScrollBar`dragCallback`is
called if there is one.

The default value is large enough to prevent unwanted snapBack
activity if the mouse is moved within the boundaries of any reasonable
screen. To reset the default, set this resource to a large value,
such as 10000.
* **`XmNtoBottomCallback`** 

Specifies the list of callbacks that is called when the user takes an
action that moves the slider to the end of the ScrollBar with the
maximum value.
The reason passed to the callback is`XmCR_TO_BOTTOM`.
* **`XmNtoTopCallback`** 

Specifies the list of callbacks that is called when the user takes an
action that moves the slider to the end of the ScrollBar with the
minimum value.
The reason passed to the callback is`XmCR_TO_TOP`.
* **`XmNtroughColor`** 

Specifies the color of the slider trough.
This color defaults to the color used for selections.
* **`XmNvalue`** 

Specifies the slider's position, between`XmNminimum`and
(`XmNmaximum`&ensp;&minus;`XmNsliderSize`).
The value is constrained to be within these inclusive bounds.
The initial value of this resource is the larger of 0 (zero) and`XmNminimum`.
* **`XmNvalueChangedCallback`** 

Specifies the list of callbacks that is called when the slider is
released after being dragged.
These callbacks are also called in place of`XmNincrementCallback`,`XmNdecrementCallback`,`XmNpageIncrementCallback`,`XmNpageDecrementCallback`,`XmNtoTopCallback`, or`XmNtoBottomCallback`when one of these callback lists would
normally be called but the value of the corresponding resource is NULL.
The reason passed to the callback is`XmCR_VALUE_CHANGED`.

### Inherited Resources


ScrollBar inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimensiondynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmSTICKY_TAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleandynamicCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        int`value`;
        int`pixel`;
} XmScrollBarCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`value`** 

Contains the new slider location value.
* **`pixel`** 

Is used only for`XmNtoTopCallback`and`XmNtoBottomCallback`.
For horizontal ScrollBars, it contains thexcoordinate of where
the mouse button selection occurred. For vertical ScrollBars, it
contains theycoordinate.

### Translations


`XmScrollBar`includes translations from Primitive.
The`XmScrollBar`translations are described in the following
list.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`&ap;s &ap;c &ap;m &ap;a`Btn1Down:** 

Select()
* **`&ap;s &ap;c &ap;m &ap;a`Btn1Up:** 

Release()
* **`&ap;s &ap;c &ap;m &ap;a Button1`PtrMoved:** 

Moved()
* **`&ap;s &ap;c &ap;m &ap;a`Btn2Down:** 

Select()
* **`&ap;s &ap;c &ap;m &ap;a`Btn2Up:** 

Release()
* **`&ap;s &ap;c &ap;m &ap;a Button2`PtrMoved:** 

Moved()
* **`&ap;s c &ap;m &ap;a`Btn1Down:** 

TopOrBottom()
* **`&ap;s c &ap;m &ap;a`Btn1Up:** 

Release()
* **`:`KeyosfActivate:** 

PrimitiveParentActivate()
* **`:`KeyosfCancel:** 

CancelDrag()
* **`:`KeyosfBeginLine:** 

TopOrBottom()
* **`:`KeyosfEndLine:** 

TopOrBottom()
* **`:`KeyosfPageLeft:** 

PageUpOrLeft(`Left`)
* **`:c`KeyosfPageUp:** 

PageUpOrLeft(`Left`)
* **`:`KeyosfPageUp:** 

PageUpOrLeft(`Up`)
* **`:`KeyosfPageRight:** 

PageDownOrRight(`Right`)
* **`:c`KeyosfPageDown:** 

PageDownOrRight(`Right`)
* **`:`KeyosfPageDown:** 

PageDownOrRight(`Down`)
* **`:`KeyosfHelp:** 

PrimitiveHelp()
* **`:c`KeyosfUp:** 

PageUpOrLeft(`Up`)
* **`:`KeyosfUp:** 

IncrementUpOrLeft(`Up`)
* **`:c`KeyosfDown:** 

PageDownOrRight(`Down`)
* **`:`KeyosfDown:** 

IncrementDownOrRight(`Down`)
* **`:c`KeyosfLeft:** 

PageUpOrLeft(`Left`)
* **`:`KeyosfLeft:** 

IncrementUpOrLeft(`Left`)
* **`:c`KeyosfRight:** 

PageDownOrRight(`Right`)
* **`:`KeyosfRight:** 

IncrementDownOrRight(`Right`)
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

PrimitiveParentActivate()
* **`s &ap;m &ap;a`Key`Tab`:** 

PrimitivePrevTabGroup()
* **`&ap;m &ap;a`Key`Tab`:** 

PrimitiveNextTabGroup()

### Action Routines


The ScrollBar action routines are

* **CancelDrag():** 

If the key press occurs during scrolling, cancels the
scroll and returns the slider to its previous location in the scrollbar,
otherwise, and if the parent is a manager, it passes the event to
the parent.
* **IncrementDownOrRight(`Down|Right`):** 

With an argument of`Down`, or 0 (zero) for compatibility, moves
the slider down by one increment.
With an argument of`Right`, or 1 for compatibility, it moves the
slider right by one increment.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`, movement toward the right or
bottom calls the callbacks for`XmNincrementCallback`.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right or
bottom calls the callbacks for`XmNdecrementCallback`.
The`XmNvalueChangedCallback`is called if the`XmNincrementCallback`or`XmNdecrementCallback`is NULL.
* **IncrementUpOrLeft(`Up|Left`):** 

With an argument of`Up`, or 0 (zero) for compatibility, moves the
slider up by one increment.
With an argument of`Left`, or 1 for compatibility, it moves the
slider left by one increment.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`,
movement to the left or top calls the callbacks for`XmNdecrementCallback`.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`,
movement to the left or top calls the callbacks for`XmNincrementCallback`.
The`XmNvalueChangedCallback`is called if the`XmNincrementCallback`or`XmNdecrementCallback`is NULL.
* **Moved():** 

If the button press occurs within the slider, the subsequent motion
events move the slider to the position of the pointer and call the
callbacks for`XmNdragCallback`.
* **PageDownOrRight(`Down|Right`):** 

With an argument of`Down`, or 0 (zero) for compatibility, moves
the slider down by one page increment.
With an argument of`Right`, or 1 for compatibility, moves the
slider right by one page increment.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`, movement toward the right or
bottom calls the callbacks for`XmNpageIncrementCallback`.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right or
bottom calls the`XmNpageDecrementCallback`callbacks. The`XmNvalueChangedCallback`is called if the`XmNpageIncrementCallback`or`XmNpageDecrementCallback`is NULL.
* **PageUpOrLeft(`Up|Left`):** 

With an argument of`Up`, or 0 (zero) for compatibility, moves the
slider up by one page increment.
With an argument of`Left`, or 1 for compatibility, it moves the
slider left by one page increment.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`,
movement to the left or top calls the callbacks for`XmNpageDecrementCallback`.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`,
movement to the left or top calls the`XmNpageIncrementCallback`callbacks. The`XmNvalueChangedCallback`is called if the`XmNpageIncrementCallback`or`XmNpageDecrementCallback`is NULL.
* **PrimitiveHelp():** 

Calls the callbacks for`XmNhelpCallback`if any exist. If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **PrimitiveNextTabGroup():** 

Traverses to the first item in the next tab group. If
the current tab group is the last entry in the tab group list, it
wraps to the beginning of the tab group list.
* **PrimitiveParentActivate():** 

If the parent is a manager,
passes the event to the parent.
* **PrimitivePrevTabGroup():** 

Traverses to the first item in the previous tab group.
If the beginning of the tab group list is reached, it wraps to the end
of the tab group list.
* **Release():** 

If the button press occurs within the slider and the slider position
is changed, the callbacks for`XmNvalueChangedCallback`are called.
* **Select():** 

`In arrow`:
Moves the slider by one increment in the direction
of the arrow.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`, movement toward the right or
bottom calls the callbacks for`XmNincrementCallback`,
and movement to the left or top calls the callbacks for`XmNdecrementCallback`.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right or
bottom calls the callbacks for`XmNdecrementCallback`,
and movement to the left or top calls the callbacks for`XmNincrementCallback`.
The`XmNvalueChangedCallback`is called if the`XmNincrementCallback`or`XmNdecrementCallback`is NULL.

`In scroll region between an arrow and the slider`:
Moves the slider by one page increment in the direction of the arrow.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`, movement toward the right or
bottom calls the callbacks for`XmNpageIncrementCallback`,
and movement to the left or top calls the callbacks for`XmNpageDecrementCallback`.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right or
bottom calls the callbacks for`XmNpageDecrementCallback`,
and movement to the left or top calls the callbacks for`XmNpageIncrementCallback`.
The`XmNvalueChangedCallback`is called if the`XmNpageIncrementCallback`or`XmNpageDecrementCallback`is
NULL.

`In slider`:
Activates the interactive dragging of the slider.

If the button is held down in either the arrows or the scroll region
longer than the`XmNinitialDelay`resource, the slider is
moved again by the same increment and the same callbacks are called.
After the initial delay has been used, the time delay changes to the
time defined by the resource`XmNrepeatDelay`.
* **TopOrBottom():** 

CtrlBtn1Downin an arrow or in the scroll region between
an arrow and the slider moves the slider as far as possible in the
direction of the arrow. If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`, movement toward the
right or bottom calls the callbacks for`XmNtoBottomCallback`, and
movement to the left or top calls the callbacks for`XmNtoTopCallback`. If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right
or bottom calls the callbacks for`XmNtoTopCallback`, and movement
to the left or top calls the callbacks for`XmNtoBottomCallback`.
The`XmNvalueChangedCallback`is called if the`XmNtoTopCallback`or`XmNtoBottomCallback`is NULL.
PressingKeyosfBeginLineorKeyosfBeginDatamoves the slider to
the minimum value and invokes the`XmNtoTopCallback`. PressingKeyosfEndLineorKeyosfEndDatamoves the slider to the
maximum value and invokes the`XmNtoBottomCallback`.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;, &cdeman.XmCreateScrollBar;, &cdeman.XmPrimitive;,
&cdeman.XmScrollBarGetValues;, and
&cdeman.XmScrollBarSetValues;.