# XmManager
library call`XmManager`The Manager widget classXmManagerwidget classManager&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Manager is a widget class used as a supporting superclass for other
widget classes. It supports the
visual resources, graphics contexts, and traversal resources necessary for the
graphics and traversal mechanisms.
### Classes


Manager inherits behavior and resources from`Core`,`Composite`, and`Constraint`.

The class pointer is`xmManagerWidgetClass`.

The class name is`XmManager`.
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

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

* **`XmNbottomShadowColor`** 

Specifies the color to use to draw the bottom
and right sides of the border shadow.
This color is used if the`XmNbottomShadowPixmap`resource is NULL.
* **`XmNbottomShadowPixmap`** 

Specifies the pixmap to use to draw the bottom and right sides of the border
shadow.
* **`XmNforeground`** 

Specifies the foreground drawing color used by manager widgets.
* **`XmNhelpCallback`** 

Specifies the list of callbacks that are called when the help key
sequence is pressed. The reason sent by this callback is`XmCR_HELP`.
* **`XmNhighlightColor`** 

Specifies the color of the highlighting rectangle.
This color is used if the highlight pixmap resource is`XmUNSPECIFIED_PIXMAP`.
* **`XmNhighlightPixmap`** 

Specifies the pixmap used to draw the highlighting rectangle.
* **`XmNinitialFocus`** 

Specifies the ID of a widget descendant of the manager.
The widget must meet these conditions:

The widget must be either a tab group or a non-tab-group widget that can
receive keyboard focus.
For the definition of a tab group, see the description of the Manager,
Primitive, and Gadget`XmNnavigationType`resources.
In general a widget can receive keyboard focus when it is a primitive, a
gadget, or a manager (such as a DrawingArea with no traversable
children) that acts as a primitive.

The widget must not be a descendant of a tab group that is itself a
descendant of the manager.
That is, the widget cannot be contained within a tab group that is
nested inside the manager.

The widget and its ancestors must have a value of True for their`XmNtraversalOn`resources.

If the widget does not meet these conditions,`XmNinitialFocus`is
treated as if the value were NULL.

This resource is meaningful only when the nearest shell ancestor's`XmNkeyboardFocusPolicy`is`XmEXPLICIT`.
It is used to determine which widget receives focus in these situations:

When the manager is the child of a shell and the shell hierarchy
receives focus for the first time

When focus is inside the shell hierarchy, the manager is a composite tab
group, and the user traverses to the manager via the keyboard

Focus is then determined as follows:

If`XmNinitialFocus`is a traversable non-tab-group widget, that
widget receives focus.

If`XmNinitialFocus`is a traversable tab group, that tab group
receives focus.
If that tab group is a composite with descendant tab groups or
traversable non-tab-group widgets, these procedures are used recursively
to assign focus to a descendant of that tab group.

If`XmNinitialFocus`is NULL, the first traversable non-tab-group
widget that is not contained within a nested tab group receives focus.

If`XmNinitialFocus`is NULL and no traversable non-tab-group widget
exists, the first traversable tab group that is not contained within a
nested tab group receives focus.
If that tab group is a composite with descendant tab groups or
traversable non-tab-group widgets, these procedures are used recursively
to assign focus to a descendant of that tab group.

If a shell hierarchy regains focus after losing it, focus returns to the
widget that had the focus at the time it left the hierarchy.

The use of`XmNinitialFocus`is undefined if the manager is a
MenuBar, PulldownMenu, PopupMenu, or OptionMenu.
* **`XmNlayoutDirection`** 

Specifies the direction in which components of the manager (including
strings) are laid out. The values are of typeXmDirection. If
the widget's parent is a manager or shell, the value is inherited from
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

Indicates that the widget is a tab group, even if
the`XmNnavigationType`of another widget in the hierarchy is`XmEXCLUSIVE_TAB_GROUP`.
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

Specifies the thickness of the drawn border shadow.`XmBulletinBoard`and its descendants set this value dynamically.
If the widget is a top-level window, this value is set to 1.
If it is not a top-level window, this value is set to 0 (zero).
* **`XmNstringDirection`** 

Is a synthetic resource for setting`XmNlayoutDirection`.
The values for this resource are`XmSTRING_DIRECTION_L_TO_R`and`XmSTRING_DIRECTION_R_TO_L`. Refer to the`XmNlayoutDirection`resource description. The`XmNstringDirection`resource is obsoleted by`XmNlayoutDirection`, but is kept here for backward compatibility.
* **`XmNtopShadowColor`** 

Specifies the color to use to draw the top and
left sides of the border shadow.
This color is used if the`XmNtopShadowPixmap`resource is NULL.
* **`XmNtopShadowPixmap`** 

Specifies the pixmap to use to draw the top and left sides of
the border shadow.
* **`XmNtraversalOn`** 

Specifies whether traversal is activated for this widget.
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
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

* **`XmNuserData`** 

Allows the application to attach
any necessary specific data to the widget. This is an internally
unused resource.

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


Manager inherits resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

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


The following set of translations are used by Manager widgets that
have Gadget children. Because Gadgets cannot have translations associated
with them, it is the responsibility of the Manager widget to intercept the
events of interest and pass them to any Gadget child with focus.
These events are ignored if no Gadget child has the focus.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **BtnMotion:** 

ManagerGadgetButtonMotion()
* **`c<Btn1Down>`:** 

ManagerGadgetTraverseCurrent()
* **`&ap;c`Btn1Down:** 

ManagerGadgetArm()
* **`&ap;c`Btn1Down`,&ap;c`Btn1Up:** 

ManagerGadgetActivate()
* **`&ap;c`Btn1Up:** 

ManagerGadgetActivate()
* **`&ap;c`Btn1Down`(2+)`:** 

ManagerGadgetMultiArm()
* **`&ap;c`Btn1Up`(2+)`:** 

ManagerGadgetMultiActivate()
* **Btn2Down:** 

ManagerGadgetDrag()
* **`:`KeyosfActivate:** 

ManagerParentActivate()
* **`:`KeyosfCancel:** 

ManagerParentCancel()
* **`:`KeyosfSelect:** 

ManagerGadgetSelect()
* **`:`KeyosfHelp:** 

ManagerGadgetHelp()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

ManagerParentActivate()
* **`&ap;s &ap;m &ap;a`Key`space`:** 

ManagerGadgetSelect()
* **Key:** 

ManagerGadgetKeyInput()
* **`:`KeyosfBeginLine:** 

ManagerGadgetTraverseHome()
* **`:`KeyosfUp:** 

ManagerGadgetTraverseUp()
* **`:`KeyosfDown:** 

ManagerGadgetTraverseDown()
* **`:`KeyosfLeft:** 

ManagerGadgetTraverseLeft()
* **`:`KeyosfRight:** 

ManagerGadgetTraverseRight()
* **`s &ap;m &ap;a`Key`Tab`:** 

ManagerGadgetPrevTabGroup()
* **`&ap;m &ap;a`Key`Tab`:** 

ManagerGadgetNextTabGroup()

### Action Routines


The`XmManager`action routines are

* **GadgetTakeFocus():** 

Causes the current gadget to take keyboard focus when`Ctrl<Btn1Down>`is pressed.
* **ManagerGadgetActivate():** 

Causes the current gadget to be activated.
* **ManagerGadgetArm():** 

Causes the current gadget to be armed.
* **ManagerGadgetButtonMotion():** 

Causes the current gadget to process a mouse motion event.
* **ManagerGadgetDrag():** 

Causes the current gadget to begin a drag operation.
This action is undefined for gadgets used in a menu system.
* **ManagerGadgetHelp():** 

Calls the callbacks for the current gadget's`XmNhelpCallback`if
any exist.
If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **ManagerGadgetKeyInput():** 

Causes the current gadget to process a keyboard event.
* **ManagerGadgetMultiActivate():** 

Causes the current gadget to process a multiple mouse click.
* **ManagerGadgetMultiArm():** 

Causes the current gadget to process a multiple mouse button press.
* **ManagerGadgetNextTabGroup():** 

This action depends on the value of the Display resource`XmNenableButtonTab`. When`XmNenableButtonTab`is False
(default), this action traverses to the first item in the next tab
group. If the current tab group is the last entry in the tab group
list, it wraps to the beginning of the tab group list.

When`XmNenableButtonTab`is True, this action moves to the next
item within the current tab group, unless it is the last item in the
tab group. When the item is the last in the group, the action
traverses to the first item in the next tab group. The`XmNenableButtonTab`behavior applies only to PushButton, ArrowButton,
and DrawnArrow.
* **ManagerGadgetPrevTabGroup():** 

This action depends on the value of the Display resource`XmNenableButtonTab`. When`XmNenableButtonTab`is False
(default), this action traverses to the first item in the previous tab
group. If the beginning of the tab group list is reached, it wraps to
the end of the tab group list.

When`XmNenableButtonTab`is True, this action moves to the
previous item within the current tab group unless it is the first item
in the tab group. When the item is the first in the group, the action
traverses to the first item in the previous tab group. The`XmNenableButtonTab`behavior applies only PushButton, ArrowButton, and
DrawnButton.
* **ManagerGadgetSelect():** 

Causes the current gadget to be armed and activated.
* **`ManagerGadgetTraverseCurrent`** 

Causes the current gadget to take keyboard focus
when`Ctrl<Btn1Down>`is pressed. Gadget is not activated.
* **ManagerGadgetTraverseDown():** 

Traverses to the next item below the current gadget in the current tab
group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **ManagerGadgetTraverseHome():** 

Traverses to the first widget or gadget in the current tab group.
* **ManagerGadgetTraverseLeft():** 

Traverses to the next item to the left of the current gadget in the
current tab group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **ManagerGadgetTraverseNext():** 

Traverses to the next item in the current tab group, wrapping if
necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **ManagerGadgetTraversePrev():** 

Traverses to the previous item in the current tab group, wrapping if
necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **ManagerGadgetTraverseRight()** 

Traverses to the next item to the right of the current gadget in the
current tab, wrapping if necessary.
widget tab group.
The wrapping direction depends on the layout direction of the
widget tab group.
* **ManagerGadgetTraverseUp():** 

Traverses to the next item above the current gadget in the current tab
group, wrapping if necessary.
The wrapping direction depends on the layout direction of the
widget tab group.
* **ManagerParentActivate():** 

If the parent is a manager,
passes theosfActivateevent received by the current widget/gadget
to its parent.
* **ManagerParentCancel():** 

If the parent is a manager,
passes theosfCancelevent received by the current widget/gadget
to its parent.

### Additional Behavior


This widget has the additional behavior described below:

* **FocusIn:** 

If the shell's keyboard focus policy is`XmEXPLICIT`and the event
occurs in a gadget, causes the gadget to be highlighted and to take the
focus.
* **FocusOut:** 

If the shell's keyboard focus policy is`XmEXPLICIT`and the event
occurs in a gadget, causes the gadget to be unhighlighted and to lose
the focus.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmDirection;,
&cdeman.XmChangeColor;,
&cdeman.XmGadget;, and
&cdeman.XmScreen;.