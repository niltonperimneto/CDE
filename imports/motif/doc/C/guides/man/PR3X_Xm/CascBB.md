# XmCascadeButtonGadget
library call`XmCascadeButtonGadget`The CascadeButtonGadget widget classXmCascadeButtonGadgetwidget classCascadeButtonGadget&npzwc;#include &lt;Xm/CascadeBG.h>
## DESCRIPTION


CascadeButtonGadget links two menu panes, a MenuBar to a menu pane, or an
OptionMenu to a menu pane.

It is used in menu systems and must have a RowColumn parent
with its`XmNrowColumnType`resource set to`XmMENU_BAR`,`XmMENU_POPUP`,`XmMENU_PULLDOWN`, or`XmMENU_OPTION`.

It is the only gadget that can have a Pulldown menu pane attached to it as
a submenu. The submenu is displayed when this gadget is activated within
a PopupMenu, a PulldownMenu, or an OptionMenu. Its visuals can
include a label or pixmap and a cascading indicator when it is in a
Popup or Pulldown menu pane; or it can include only a label or a pixmap
when it is in an OptionMenu.
The positioning of the PulldownMenu with respect to the CascadeButton
depends on the`XmNlayoutDirection`resource of the MenuShell.

The default behavior associated with a CascadeButtonGadget depends on the type
of menu system in which it resides.
By default,`BSelect`controls the behavior of the CascadeButtonGadget.
In addition,`BMenu`controls the behavior of the CascadeButtonGadget if
it resides in a PopupMenu system.
The actual mouse button used is determined by its RowColumn parent.`BMenu`also performs the`BSelect`actions in all
types of menu systems.

A CascadeButtonGadget's visuals differ from most other button gadgets. When
the button
becomes armed, its visuals change from a 2-D to a 3-D look, and it displays
the submenu that has been attached
to it. If no submenu is attached, it simply changes its visuals.

When a CascadeButtonGadget
within a Pulldown or Popup menu pane is armed as the result of the user
moving the mouse pointer into the gadget, it does not immediately display its
submenu. Instead, it waits a short time to see if the arming
was temporary (that is, the user was simply passing through the gadget), or
the user really wanted the submenu posted. This delay is
configurable using`XmNmappingDelay`.

CascadeButtonGadget provides a single mechanism for
activating the gadget from the keyboard. This mechanism is referred to as
a keyboard mnemonic.
If a mnemonic has been
specified for the gadget, the user may activate it
by simply typing the mnemonic while the CascadeButtonGadget is visible.
If the CascadeButtonGadget is in a MenuBar and the MenuBar does not have
focus, the`MAlt`modifier must be
pressed with the mnemonic.
Mnemonics are typically used to interact with a menu
using the keyboard.

If a CascadeButtonGadget is in a Pulldown or Popup menu pane and there is a submenu attached, the`XmNmarginBottom`,`XmNmarginLeft`,`XmNmarginRight`, and`XmNmarginTop`resources may enlarge to accommodate`XmNcascadePixmap`.`XmNmarginWidth`defaults
to 6 if this
resource is in a MenuBar; otherwise, it takes LabelGadget's default, which
is 2.

CascadeButtonGadget uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits.
### Classes


CascadeButtonGadget inherits behavior, resources, and traits
from the`Object`,`RectObj`,`XmGadget`,
and`XmLabelGadget`classes.

The class pointer is`xmCascadeButtonGadgetClass`.

The class name is`XmCascadeButtonGadget`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmCascadeButtonGadget``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNcascadePixmapXmCPixmapPixmapdynamicCSGXmNcascadingCallbackXmCCallbackXtCallbackListNULLCXmNmappingDelayXmCMappingDelayint180 msCSGXmNsubMenuIdXmCMenuWidgetWidgetNULLCSG

* **`XmNactivateCallback`** 

Specifies the list of callbacks that is
called when the user activates the CascadeButtonGadget,
and there is no submenu attached to pop up. The activation occurs
when a mouse button is released
or when the mnemonic associated with the gadget is typed. The specific mouse
button depends on information in the RowColumn parent. The reason
sent by the callback is`XmCR_ACTIVATE`.
* **`XmNcascadePixmap`** 

Specifies the cascade pixmap displayed on one end of the gadget when
a CascadeButtonGadget is used within a Popup or Pulldown menu pane
and a submenu is attached.
The LabelGadget class resources`XmNmarginBottom`,`XmNmarginLeft`,`XmNmarginRight`, and`XmNmarginTop`may be modified to
ensure that room is left for the cascade pixmap.
The default cascade pixmap in menus other than option menus is an arrow
pointing to the side of the menu where the submenu will appear.
The default for the CascadeButtonGadget in an option menu is`XmUNSPECIFIED_PIXMAP`.

The positioning of the cascade pixmap
to either the left of right of the widget, and the direction of the
arrow, depend on the`XmNlayoutDirection`resource of the MenuShell.
* **`XmNcascadingCallback`** 

Specifies the list of callbacks that is
called just prior to the mapping of the submenu
associated with the
CascadeButtonGadget. The reason sent by the callback is`XmCR_CASCADING`.
* **`XmNmappingDelay`** 

Specifies the amount of time, in milliseconds, between when a
CascadeButtonGadget
becomes armed and when it maps its submenu. This delay is used only when the
gadget is within a Popup or Pulldown menu pane.
The value must not be negative.
* **`XmNsubMenuId`** 

Specifies the widget ID for the Pulldown menu pane to be
associated with this CascadeButtonGadget. The specified menu pane is
displayed when the CascadeButtonGadget becomes armed. The menu pane must
have been created with the appropriate parentage depending on the type
of menu used. See &cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreatePopupMenu;, and &cdeman.XmCreateOptionMenu; for
more information on the menu systems.

### Inherited Resources


CascadeButtonGadget inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmLabelGadget Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLN/AXmNacceleratorTextXmCAcceleratorTextXmStringNULLN/AXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimensiondynamicCSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimension0CSGXmNmarginRightXmCMarginRightDimensiondynamicCSGXmNmarginTopXmCMarginTopDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimensiondynamicCSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringdynamicCSGXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension0CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback or is NULL if this
callback was not triggered by an`XEvent`

### Behavior


`XmCascadeButtonGadget`includes behavior from`XmGadget`.`XmCascadeButton`includes the menu traversal behavior from`XmLabel`.
Additional`XmCascadeButtonGadget`behavior is described in
the following list (in a Popup
menu system,Btn3also performs theBtn1actions).

* **Btn1Down:** 

Unposts any menus posted by the parent menu.
Arms the CascadeButtonGadget,
posts the associated submenu,
enables mouse traversal,
and, in a MenuBar, arms the MenuBar.
If the menu is already active, this event
disables keyboard traversal for the menu and returns
the menu to mouse traversal mode.
* **Btn1Up:** 

Calls the callbacks in`XmNcascadingCallback`,
posts the submenu attached to the CascadeButtonGadget and
enables keyboard traversal within the menu.
If the CascadeButtonGadget does not have a submenu attached,
this action calls the callbacks in`XmNactivateCallback`,
activates the CascadeButtonGadget, and unposts
all posted menus in the cascade.
* **KeyosfActivate:** 

Calls the callbacks in`XmNcascadingCallback`,
and
posts the submenu attached to the CascadeButtonGadget if keyboard
traversal is enabled in the menu.
If the CascadeButtonGadget does not have a submenu attached,
this action calls the callbacks in`XmNactivateCallback`,
activates the CascadeButtonGadget, and unposts
all posted menus in the cascade.
This action applies only to gadgets in MenuBars, PulldownMenus, and
PopupMenus. For a CascadeButtonGadget in an OptionMenu, if the parent
is a manager, this action passes the event to the parent.
* **KeyosfSelect:** 

Calls the callbacks in`XmNcascadingCallback`,
and
posts the submenu attached to the CascadeButtonGadget if keyboard
traversal is enabled in the menu.
If the CascadeButtonGadget does not have a submenu attached,
this action calls the callbacks in`XmNactivateCallback`,
activates the CascadeButtonGadget, and unposts all posted
menus in the cascade.
* **KeyosfHelp:** 

Unposts all menus in the menu hierarchy and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard focus to
the widget that had the focus before the menu system was entered.
Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help callbacks for this widget, this action calls the
help callbacks for the nearest ancestor that has them.
* **KeyosfCancel:** 

In a MenuBar, disarms the CascadeButtonGadget and the menu and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores
keyboard focus to the widget that had the focus before the menu was
entered.
For a CascadeButtonGadget in an OptionMenu, if the parent is a manager, this
action passes the event to the parent.

In a toplevel Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard
focus to the widget that had the focus before the MenuBar was
entered.
In other Pulldown MenuPanes, unposts the menu.

In a Popup MenuPane, unposts the menu and restores keyboard focus to the
widget from which the menu was posted.
* **Enter:** 

If keyboard traversal is enabled does nothing.
Otherwise, in a MenuBar,
unposts any MenuPanes associated with another MenuBar entry,
arms the CascadeButtonGadget, and posts the associated submenu.
In other menus,
arms the CascadeButtonGadget and posts the associated submenu after
the delay specified by`XmNmappingDelay`.
* **Leave:** 

If keyboard traversal is enabled does nothing.
Otherwise, in a MenuBar,
disarms the CascadeButtonGadget
if the submenu associated with the CascadeButtonGadget is not currently
posted or if there is no submenu associated with the CascadeButtonGadget.

In other menus, if the pointer moves anywhere except into a submenu
associated with the CascadeButtonGadget, the CascadeButtonGadget is
disarmed and its submenu is unposted.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Object;, &cdeman.RectObj;,
&cdeman.XmCascadeButtonHighlight;,
&cdeman.XmCreateCascadeButtonGadget;,
&cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreatePopupMenu;,
&cdeman.XmCreateOptionMenu;,
&cdeman.XmGadget;, &cdeman.XmLabelGadget;,
and &cdeman.XmRowColumn;.