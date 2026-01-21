# XmCascadeButton
library call`XmCascadeButton`The CascadeButton widget classXmCascadeButtonwidget classCascadeButton&npzwc;#include &lt;Xm/CascadeB.h>
## DESCRIPTION


CascadeButton links two menu panes or a MenuBar to a menu pane.

It is used in menu systems and must have a RowColumn parent
with its`XmNrowColumnType`resource set to`XmMENU_BAR`,`XmMENU_POPUP`or`XmMENU_PULLDOWN`.

It is the only widget that can have a Pulldown menu pane attached to it as
a submenu. The submenu is displayed when this widget is activated within
a MenuBar, a PopupMenu, or a PulldownMenu. Its visuals can
include a label or pixmap and a cascading indicator when it is in a
Popup or Pulldown menu pane; or it can include only a label or a pixmap
when it is in a MenuBar.
The positioning of the PulldownMenu with respect to the CascadeButton
depends on the`XmNlayoutDirection`resource of the MenuShell.

The default behavior associated with a CascadeButton depends on the type
of menu system in which it resides.
By default,`BSelect`controls the behavior of the CascadeButton.
In addition,`BMenu`controls the behavior of the CascadeButton if
it resides in a PopupMenu system.
The actual mouse button used is determined by its RowColumn parent.`BMenu`also performs the`BSelect`actions in all
types of menu systems.

A CascadeButton's visuals differ from most other button gadgets. When the
button
becomes armed, its visuals change from a 2-D to a 3-D look, and it displays
the submenu that has been attached
to it. If no submenu is attached, it simply changes its visuals.

When a CascadeButton within a Pulldown or Popup menu pane
is armed as the result of the user
moving the mouse pointer into the widget, it does not immediately display its
submenu. Instead, it waits a short amount of time to see if the arming
was temporary (that is, the user was simply passing through the widget), or
whether the user really wanted the submenu posted. This time delay is
configurable using`XmNmappingDelay`.

CascadeButton provides a single mechanism for
activating the widget from the keyboard. This mechanism is referred to as
a keyboard mnemonic.
If a mnemonic has been
specified for the widget, the user may activate the CascadeButton
by simply typing the mnemonic while the CascadeButton is visible.
If the CascadeButton is in a MenuBar and the MenuBar does not have the
focus, the`MAlt`modifier must be
pressed with the mnemonic.
Mnemonics are typically used to interact with a menu
using the keyboard interface.

If the Cascadebutton is in a Pulldown or Popup menu pane and
there is a submenu attached, the`XmNmarginBottom`,`XmNmarginLeft`,`XmNmarginRight`, and`XmNmarginTop`resources may
enlarge to accommodate`XmNcascadePixmap`.`XmNmarginWidth`defaults
to 6 if this
resource is in a MenuBar; otherwise, it takes Label's default, which
is 2.

CascadeButton uses the`XmQTmenuSystem`and`XmQTspecifyRenderTable`traits.
### Classes


CascadeButton inherits behavior, resources, and traits from`Core`,`XmPrimitive`, and`XmLabel`classes.

The class pointer is`xmCascadeButtonWidgetClass`.

The class name is`XmCascadeButton`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile,
remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmCascadeButton Resource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNcascadePixmapXmCPixmapPixmapdynamicCSGXmNcascadingCallbackXmCCallbackXtCallbackListNULLCXmNmappingDelayXmCMappingDelayint180 msCSGXmNsubMenuIdXmCMenuWidgetWidgetNULLCSG

* **`XmNactivateCallback`** 

Specifies the list of callbacks that is called
when the user activates the CascadeButton widget
and there is no submenu attached to pop up. The activation occurs
when a mouse button is released
or when the mnemonic associated with the widget is typed. The specific mouse
button depends on information in the RowColumn parent. The reason
sent by the callback is`XmCR_ACTIVATE`.
* **`XmNcascadePixmap`** 

Specifies the cascade pixmap displayed on one end of the widget when
a CascadeButton is used within a Popup or Pulldown menu pane and a submenu
is attached.
The Label class resources`XmNmarginBottom`,`XmNmarginLeft`,`XmNmarginRight`, and`XmNmarginTop`may be modified to ensure
that room is left for the cascade pixmap.
The default cascade pixmap is an arrow pointing to the side of the menu
where the submenu will appear.
The positioning of the cascade pixmap
to either the left of right of the widget, and the direction of the
arrow, depend on the`XmNlayoutDirection`resource of the MenuShell.
* **`XmNcascadingCallback`** 

Specifies the list of callbacks that is called
just prior to the mapping of the submenu associated with
CascadeButton. The reason sent by the callback is`XmCR_CASCADING`.
* **`XmNmappingDelay`** 

Specifies the amount of time, in milliseconds, between when a CascadeButton
becomes armed and when it maps its submenu. This delay is used only when
the widget is within a Popup or Pulldown menu pane.
The value must not be negative.
* **`XmNsubMenuId`** 

Specifies the widget ID for the Pulldown menu pane to be associated with
this CascadeButton. The specified menu pane is displayed
when the CascadeButton becomes armed. The menu pane must have been created
with the appropriate parentage depending on the type of menu used. See
&cdeman.XmCreateMenuBar;, &cdeman.XmCreatePulldownMenu;, and
&cdeman.XmCreatePopupMenu; for more information on the menu systems.

### Inherited Resources


CascadeButton inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmLabel Resource Set``Name``Class``Type``Default``Access`XmNacceleratorXmCAcceleratorStringNULLN/AXmNacceleratorTextXmCAcceleratorTextXmStringNULLN/AXmNalignmentXmCAlignmentunsigned chardynamicCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNlabelInsensitivePixmapXmCLabelInsensitivePixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelPixmapXmCLabelPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlabelTypeXmCLabelTypeunsigned charXmSTRINGCSGXmNmarginBottomXmCMarginBottomDimensiondynamicCSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginLeftXmCMarginLeftDimension0CSGXmNmarginRightXmCMarginRightDimensiondynamicCSGXmNmarginTopXmCMarginTopDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimensiondynamicCSGXmNmnemonicXmCMnemonicKeySymNULLCSGXmNmnemonicCharSetXmCMnemonicCharSetStringXmFONTLIST_DEFAULT_TAGCSGXmNrecomputeSizeXmCRecomputeSizeBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCSG

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension0CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleandynamicGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback or is NULL
if this callback was not triggered due to an`XEvent`

### Translations


`XmCascadeButton`includes translations from`XmPrimitive`.`XmCascadeButton`includes the menu traversal translations from`XmLabel`.

Note that altering translations in`#override`or`#augment`mode is undefined.

The following list describes the translations for a
CascadeButton in a MenuBar.
The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **EnterWindow`Normal`:** 

MenuBarEnter()
* **LeaveWindow`Normal`:** 

MenuBarLeave()
* **Btn2Down:** 

ProcessDrag()
* **`c<BtnDown>`:** 

MenuButtonTakeFocusUp()
* **`c<BtnUp>`:** 

MenuButtonTakeFocusUp()
* **`&ap;c`BtnDown:** 

MenuBarSelect()
* **`&ap;c`BtnUp:** 

DoSelect()
* **`:`KeyosfSelect:** 

KeySelect()
* **`:`KeyosfActivate:** 

KeySelect()
* **`:`KeyosfHelp:** 

Help()
* **`:`KeyosfCancel:** 

CleanupMenuBar()
* **`&ap;s`Key`Return`:** 

KeySelect()
* **`&ap;s`Key`space`:** 

KeySelect()


The following list describes the translations for a
CascadeButton in a PullDown or Popup MenuPane.
In a Popup menu system,Btn3also performs theBtn1actions.

* **EnterWindow:** 

DelayedArm()
* **LeaveWindow:** 

CheckDisarm()
* **Btn2Down:** 

ProcessDrag()
* **`c<BtnDown>`:** 

MenuButtonTakeFocus()
* **`c<BtnUp>`:** 

MenuButtonTakeFocusUp()
* **`&ap;c`BtnDown:** 

StartDrag()
* **`&ap;c`BtnUp:** 

DoSelect()
* **`:`KeyosfSelect:** 

KeySelect()
* **`:`KeyosfActivate:** 

KeySelect()
* **`:`KeyosfHelp:** 

Help()
* **`:`KeyosfCancel:** 

CleanupMenuBar()
* **`&ap;s`Key`Return`:** 

KeySelect()
* **`&ap;s`Key`space`:** 

KeySelect()

### Action Routines


The XmCascadeButton action routines are

* **CleanupMenuBar():** 

In a MenuBar, disarms the CascadeButton and the menu and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores
keyboard focus to the widget that had the focus before the menu was
entered.

In a toplevel Pulldown MenuPane from a MenuBar, unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard
focus to the widget that had the focus before the MenuBar was
entered.
In other Pulldown MenuPanes, unposts the menu.

In a Popup MenuPane, unposts the menu and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard focus to the
widget from which the menu was posted.
* **DoSelect():** 

Calls the callbacks in`XmNcascadingCallback`,
posts the submenu attached to the CascadeButton and
enables keyboard traversal within the menu.
If the CascadeButton does not have a submenu attached,
this action calls the callbacks in`XmNactivateCallback`,
activates the CascadeButton, and unposts all posted menus in the cascade.
* **Help():** 

Unposts all menus in the menu hierarchy and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard focus to
the widget that had the focus before the menu system was entered.
Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help callbacks for this widget, this action calls the
help callbacks for the nearest ancestor that has them.
* **KeySelect():** 

Calls the callbacks in`XmNcascadingCallback`,
and posts the submenu attached to the CascadeButton if keyboard
traversal is enabled in the menu.
If the CascadeButton does not have a submenu attached,
this action calls the callbacks in`XmNactivateCallback`,
activates the CascadeButton, and unposts all posted menus in the cascade.
* **MenuBarSelect():** 

Unposts any menus posted by the parent menu.
Arms both the CascadeButton and the MenuBar,
posts the associated submenu, and enables mouse traversal.
If the menu is already active, this event
disables keyboard traversal for the menu and returns
the menu to mouse traversal mode.
* **StartDrag():** 

Arms the CascadeButton, posts the associated submenu, and enables mouse
traversal.
If the menu is already active, this event
disables keyboard traversal for the menu and returns
the menu to mouse traversal mode.

### Additional Behavior


Posting a submenu calls the`XmNcascadingCallback`callbacks.
This widget has the following additional behavior:

* **EnterWindow:** 

If keyboard traversal is enabled, does nothing.
Otherwise, in a MenuBar that is armed,
unposts any MenuPanes associated with another MenuBar entry,
arms the CascadeButton, and posts the associated submenu.
In other menus,
arms the CascadeButton and posts the associated submenu after
the delay specified by`XmNmappingDelay`.
* **LeaveWindow:** 

If keyboard traversal is enabled does nothing.
Otherwise, in a MenuBar that is armed,
disarms the CascadeButton
if the submenu associated with the CascadeButton is not currently
posted or if there is no submenu associated with the CascadeButton.

In other menus,
if the pointer moves anywhere except into a submenu associated with the
CascadeButton, the CascadeButton is disarmed and its submenu is unposted.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;, &cdeman.XmCascadeButtonHighlight;,
&cdeman.XmCreateCascadeButton;,
&cdeman.XmCreateMenuBar;, &cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreatePopupMenu;,
&cdeman.XmLabel;, &cdeman.XmPrimitive;, and &cdeman.XmRowColumn;.