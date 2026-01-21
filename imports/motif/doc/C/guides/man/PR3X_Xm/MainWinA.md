# XmMainWindow
library call`XmMainWindow`The MainWindow widget classXmMainWindowwidget classMainWindow&npzwc;#include &lt;Xm/MainW.h>
## DESCRIPTION


MainWindow provides a standard layout for the primary window of an
application. This layout includes a MenuBar, a CommandWindow, a work
region, a MessageWindow, and ScrollBars. Any or all of these areas are optional. The
work region and ScrollBars in the MainWindow behave identically to the work
region and ScrollBars in the ScrolledWindow widget. The user can think of
the MainWindow as an extended ScrolledWindow with an optional MenuBar and
optional CommandWindow and MessageWindow.

In a fully loaded MainWindow, the MenuBar spans the top of the window
horizontally. The CommandWindow spans the MainWindow horizontally just below
the MenuBar, and the work region lies below the CommandWindow.
The MessageWindow is below the work region.
Any space remaining below the
MessageWindow is managed in a manner identical to ScrolledWindow.
The behavior of ScrolledWindow can be controlled by the ScrolledWindow
resources.
To create a MainWindow, first create the
work region elements, a MenuBar, a CommandWindow, a MessageWindow, a
horizontal
ScrollBar, and a vertical ScrollBar widget, and then
call`XmMainWindowSetAreas`with those widget IDs.

MainWindow
can also create three Separator widgets that provide a visual separation of
MainWindow's four components.
The user can specify resources in a resource file for the automatically
created gadgets that contain the MainWindow separators. The name of the
first separator gadget is`Separator1`; the second is`Separator2`;
and the third is`Separator3`.

MainWindow also provides the following three child types for layout at
creation time:

`XmMENU_BAR`

`XmCOMMAND_WINDOW`

`XmMESSAGE_WINDOW`

MainWindow can use these child types
at creation time instead of their associated resource values.
MainWindow uses the`XmQTmenuSystem`trait.
### Descendants


MainWindow automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`HorScrollBar``XmScrollBar`horizontal scroll bar`Separator1``XmSeparatorGadget`optional first separator`Separator2``XmSeparatorGadget`optional second separator`Separator3``XmSeparatorGadget`optional third separator`VertScrollBar``XmScrollBar`vertical scroll bar
### Classes


MainWindow inherits behavior and resources from`Core`,`Composite`,`Constraint`,`XmManager`,
and`XmScrolledWindow`.

The class pointer is`xmMainWindowWidgetClass`.

The class name is`XmMainWindow`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove
the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmMainWindow Resource Set``Name``Class``Type``Default``Access`XmNcommandWindowXmCCommandWindowWidgetNULLCSGXmNcommandWindowLocationXmCCommandWindowLocationunsigned charABOVE (SeeDesc.)CGXmNmainWindowMarginHeightXmCMainWindowMarginHeightDimension0CSGXmNmainWindowMarginWidthXmCMainWindowMarginWidthDimension0CSGXmNmenuBarXmCMenuBarWidgetNULLCSGXmNmessageWindowXmCMessageWindowWidgetNULLCSGXmNshowSeparatorXmCShowSeparatorBooleanFalseCSG

* **`XmNcommandWindow`** 

Specifies the widget to be laid out as the CommandWindow. This widget
must have been previously created and managed as a child of MainWindow.
* **`XmNcommandWindowLocation`** 

Controls the position of the command window.`XmCOMMAND_ABOVE_WORKSPACE`locates the command window between the menu bar and the work window.`XmCOMMAND_BELOW_WORKSPACE`locates the command window between the
work window and the message window.
* **`XmNmainWindowMarginHeight`** 

Specifies the margin height on the top and bottom of MainWindow. This
resource overrides any setting of the
ScrolledWindow resource`XmNscrolledWindowMarginHeight`.
* **`XmNmainWindowMarginWidth`** 

Specifies the margin width on the right and left sides of MainWindow. This
resource overrides any setting of the ScrolledWindow resource`XmNscrolledWindowMarginWidth`.
* **`XmNmenuBar`** 

Specifies the widget to be laid out as the MenuBar. This widget must
have been previously created and managed as a child of MainWindow.
* **`XmNmessageWindow`** 

Specifies the widget to be laid out as the MessageWindow. This widget
must have been previously created and managed as a child of MainWindow.
The MessageWindow is positioned at the bottom of the MainWindow.
If this value is NULL, no message window is included in the MainWindow.
* **`XmNshowSeparator`** 

Displays separators between the components of the MainWindow when set
to True. If set to False, no separators are displayed.

### Inherited Resources


MainWindow inherits behavior and resources from the
superclasses described in the following table.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmScrolledWindow Resource Set``Name``Class``Type``Default``Access`XmNautoDragModelXmCAutoDragModelXtEnumXmAUTO_DRAG_ENABLEDCSGXmNclipWindowXmCClipWindowWidgetdynamicGXmNhorizontalScrollBarXmCHorizontalScrollBarWidgetdynamicCSGXmNscrollBarDisplayPolicyXmCScrollBarDisplayPolicyunsigned chardynamicCSGXmNscrollBarPlacementXmCScrollBarPlacementunsigned charXmBOTTOM_RIGHTCSGXmNscrolledWindowMarginHeightXmCScrolledWindowMarginHeightDimension0N/AXmNscrolledWindowMarginWidthXmCScrolledWindowMarginWidthDimension0N/AXmNscrollingPolicyXmCScrollingPolicyunsigned charXmAPPLICATION_DEFINEDCGXmNspacingXmCSpacingDimension4CSGXmNtraverseObscuredCallbackXmCCallbackXtCallbackListNULLCSGXmNverticalScrollBarXmCVerticalScrollBarWidgetdynamicCSGXmNvisualPolicyXmCVisualPolicyunsigned chardynamicGXmNworkWindowXmCWorkWindowWidgetNULLCSG

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


MainWindow inherits translations from ScrolledWindow.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;, &cdeman.Core;,
&cdeman.XmCreateMainWindow;,
&cdeman.XmMainWindowSep1;, &cdeman.XmMainWindowSep2;,
&cdeman.XmMainWindowSep3;,
&cdeman.XmMainWindowSetAreas;,
&cdeman.XmManager;, and &cdeman.XmScrolledWindow;