# XmScrolledWindow
library call`XmScrolledWindow`The ScrolledWindow widget classXmScrolledWindowwidget classScrolledWindow&npzwc;#include &lt;Xm/ScrolledW.h>
## DESCRIPTION


The ScrolledWindow widget combines one or two ScrollBar widgets and a
viewing area to implement a visible window onto some other (usually larger)
data display. The visible part of the window can be scrolled through the
larger display by the use of ScrollBars.

To use ScrolledWindow, an application first creates a ScrolledWindow
widget, any needed ScrollBar widgets, and a widget capable of displaying
any desired data as the work area of ScrolledWindow. ScrolledWindow
positions the work area widget and displays the ScrollBars if so
requested. When the user performs some action on the ScrollBar, the
application is notified through the normal ScrollBar callback
interface.

ScrolledWindow can be configured to operate automatically so
that it performs all scrolling and display actions with no need for application
program involvement. It can also be configured to provide a minimal support
framework in which the application is responsible for processing all user input
and making all visual changes to the displayed data in response to that input.

When ScrolledWindow is performing automatic scrolling it creates a
clipping window and automatically creates the scroll bars.
Conceptually, this window becomes the viewport through which
the user examines the larger underlying data area. The application simply
creates the desired data, then makes that data the work area of the
ScrolledWindow.
When the user moves the slider to change the displayed data, the
workspace is moved under the viewing area so that a new portion of the data
becomes visible.

Sometimes it is impractical for an application to create a
large data space and simply display it through a small clipping window. For
example, in a text editor, creating a single data area that consisted of a
large file would involve an undesirable amount of overhead.
The application needs to use a ScrolledWindow
(a small viewport onto some larger data), but needs to be notified
when the user scrolls the viewport so it can bring in more data from
storage and update the display area. For these cases, the ScrolledWindow can be
configured so that it provides only visual layout support. No clipping window
is created, and the application must maintain the data displayed in the
work area, as well as respond to user input on the ScrollBars.

The user can specify resources in a resource file for the automatically
created widgets that contain the horizontal and vertical scrollbars
and the clipping area of
the ScrolledWindow widget. The names of these widgets are`HorScrollBar`,`VertScrollBar`, and`ClipWindow`respectively,
and remain consistent whether created by`XmCreateScrolledList`,`XmCreateScrolledText`or`XmCreateScrolledWindow`.

ScrolledWindow uses the`XmQTnavigator`trait, and
holds the`XmQTscrollFrame`trait.
### Descendants


ScrolledWindow automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`VertScrollBar``XmScrollBar`vertical scroll bar`HorScrollBar``XmScrollBar`horizontal scroll bar`ClipWindow``XmClipWindow`clip window
### Classes


ScrolledWindow inherits behavior,
resources, and traits from`Core`,`Composite`,`Constraint`, and`XmManager`.

The class pointer is`xmScrolledWindowWidgetClass`.

The class name is`XmScrolledWindow`.
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

`XmScrolledWindow Resource Set``Name``Class``Type``Default``Access`XmNautoDragModelXmCAutoDragModelXtEnumXmAUTO_DRAG_ENABLEDCSGXmNclipWindowXmCClipWindowWidgetdynamicGXmNhorizontalScrollBarXmCHorizontalScrollBarWidgetdynamicCSGXmNscrollBarDisplayPolicyXmCScrollBarDisplayPolicyunsigned chardynamicCSGXmNscrollBarPlacementXmCScrollBarPlacementunsigned charXmBOTTOM_RIGHTCSGXmNscrolledWindowMarginHeightXmCScrolledWindowMarginHeightDimension0CSGXmNscrolledWindowMarginWidthXmCScrolledWindowMarginWidthDimension0CSGXmNscrollingPolicyXmCScrollingPolicyunsigned charXmAPPLICATION_DEFINEDCGXmNspacingXmCSpacingDimension4CSGXmNtraverseObscuredCallbackXmCCallbackXtCallbackListNULLCSGXmNverticalScrollBarXmCVerticalScrollBarWidgetdynamicCSGXmNvisualPolicyXmCVisualPolicyunsigned chardynamicGXmNworkWindowXmCWorkWindowWidgetNULLCSG

* **`XmNautoDragModel`** 

Indicates whether automatic drag is enabled (`XmAUTO_DRAG_ENABLED`or disabled (`XmAUTO_DRAG_DISABLED`). By default it is enabled.
* **`XmNclipWindow`** 

Specifies the widget ID of the clipping area. This
is automatically created by
ScrolledWindow when the`XmNvisualPolicy`resource is set to`XmCONSTANT`and
can only be read by the application. Any attempt to set this resource to a
new value causes a warning message to be printed by the scrolled
window. If the`XmNvisualPolicy`resource is set to`XmVARIABLE`, this
resource is set to NULL, and no clipping window is created.
* **`XmNhorizontalScrollBar`** 

Specifies the widget ID of the horizontal ScrollBar.
This is automatically created by ScrolledWindow when the`XmNscrollingPolicy`is initialized to`XmAUTOMATIC`; otherwise,
the default is NULL.
* **`XmNscrollBarDisplayPolicy`** 

Controls the automatic placement of the ScrollBars. If
it is set to`XmAS_NEEDED`and if`XmNscrollingPolicy`is
set to`XmAUTOMATIC`, ScrollBars are displayed only if the
workspace exceeds the clip area in one or both dimensions. A resource value
of`XmSTATIC`causes the ScrolledWindow
to display the ScrollBars whenever
they are managed, regardless of the relationship between the clip window
and the work area. This resource must be`XmSTATIC`when`XmNscrollingPolicy`is`XmAPPLICATION_DEFINED`.
The default is`XmAS_NEEDED`when`XmNscrollingPolicy`is`XmAUTOMATIC`, and`XmSTATIC`otherwise.
* **`XmNscrollBarPlacement`** 

Specifies the positioning of the ScrollBars
in relation to the work window. The
values are

* **`XmTOP_LEFT`** 

The horizontal ScrollBar is placed above the
work window; the vertical ScrollBar to
is placed the left.
* **`XmBOTTOM_LEFT`** 

The horizontal ScrollBar is placed below the
work window; the vertical ScrollBar to
is placed the left.
* **`XmTOP_RIGHT`** 

The horizontal ScrollBar is placed above the
work window; the vertical ScrollBar to
is placed the right.
* **`XmBOTTOM_RIGHT`** 

The horizontal ScrollBar is placed below the
work window; the vertical ScrollBar to
is placed the right.


The default value
depends on the value of the`XmNlayoutDirection`resource of the widget.
* **`XmNscrolledWindowMarginHeight`** 

Specifies the margin height on the top and bottom of the
ScrolledWindow.
In order to use the autoscroll drag feature of the Motif drag and
drop facility, a user must be able to hold a drag icon over the margin
of a scrolled window. Though drag and drop will work with the default
margin size of zero, a user may find it difficult to position the icon
precisely enough to use the feature easily. The application programmer
should ensure that the window margins are set to an adequate size, if the
use of the autoscroll drag feature is desired.
* **`XmNscrolledWindowMarginWidth`** 

Specifies the margin width on the right and left sides of the
ScrolledWindow.
Please refer to the warning concerning the default margin size for the`XmNscrolledWindowMarginHeight`resource, above.
* **`XmNscrollingPolicy`** 

Performs automatic scrolling of the
work area with no application interaction. If the value of this resource
is`XmAUTOMATIC`, ScrolledWindow automatically creates the
ScrollBars; attaches callbacks to the
ScrollBars; sets the visual policy to`XmCONSTANT`;
and automatically moves the
work area through the clip window in response to any user interaction with
the ScrollBars. An application can also add its own callbacks to the
ScrollBars. This allows the application to be notified of a scroll event
without having to perform any layout procedures.

`NOTE`: Since the
ScrolledWindow adds callbacks to the ScrollBars, an application should not
perform an`XtRemoveAllCallbacks`on any of the ScrollBar widgets.

When`XmNscrollingPolicy`is set to`XmAPPLICATION_DEFINED`, the application
is responsible for all aspects of scrolling. The ScrollBars must be created
by the application, and it is responsible for performing any visual changes
in the work area in response to user input.

This resource must be set to the desired policy at the time the
ScrolledWindow is created. It cannot be changed through`SetValues`.
* **`XmNspacing`** 

Specifies the distance that separates the ScrollBars from the
work window.
* **`XmNtraverseObscuredCallback`** 

Specifies a list of callbacks that is called when traversing to
a widget or gadget that is obscured due to its position in the
work area relative to the location of the ScrolledWindow viewport.
This resource is valid only when`XmNscrollingPolicy`is`XmAUTOMATIC`. If this resource is NULL, an obscured widget
cannot be traversed to. The callback reason is`XmCR_OBSCURED_TRAVERSAL`.
* **`XmNverticalScrollBar`** 

Specifies the widget ID of the vertical ScrollBar.
This is automatically created by ScrolledWindow when the`XmNscrollingPolicy`is initialized to`XmAUTOMATIC`; otherwise,
the default is NULL.
* **`XmNvisualPolicy`** 

Enlarges the ScrolledWindow to match the size of the work area. It can
also be used as a static viewport onto
a larger data space. If the visual policy
is`XmVARIABLE`, the ScrolledWindow forces the ScrollBar display
policy to`XmSTATIC`and
allows the work area to grow or shrink
at any time and adjusts its layout to accommodate the new size. When
the policy is`XmCONSTANT`, the work area grows or shrinks
as requested, but a clipping window forces the size of the visible
portion to remain constant. The only time the viewing area can grow is
in response
to a resize from the ScrolledWindow's parent.
The default is`XmCONSTANT`when`XmNscrollingPolicy`is`XmAUTOMATIC`, and`XmVARIABLE`otherwise.

`NOTE`: This resource must be set to the desired policy at the time the
ScrolledWindow is created. It cannot be changed through`SetValues`.
* **`XmNworkWindow`** 

Specifies the widget ID of the viewing area.
`XmScrolledWindow Constraint Resource Set``Name``Class``Type``Default``Access`XmNscrolledWindowChildTypeXmCScrolledWindowChildTypeunsigned charRESOURCE_DEFAULTCSG

* **`XmNscrolledWindowChildType`** 

Specifies what the child is. ScrolledWindow supports a number of child
types. The possible values are:

* **`XmWORK_AREA`** 

Indicates a work area child. This specifies that both ScrollBars are
limited to moving the child inside the clipping window. If the
scrolling policy is`XmAUTOMATIC`, the work area child can move in both directions.
* **`XmHOR_SCROLLBAR`** 

Indicates a horizontal child widget; the child must have the`XmQTnavigator`trait installed.
For example, the`XmScrollBar`widget has the`XmQTnavigator`trait installed.
* **`XmVERT_SCROLLBAR`** 

Indicates a vertical child widget; the child must have the`XmQTnavigator`trait installed.
* **`XmSCROLL_HOR`** 

Indicates that only the horizontal ScrollBar moves the child. This
value is only meaningful if the scrolling policy is`XmAUTOMATIC`.
* **`XmSCROLL_VERT`** 

Indicates that only the vertical ScrollBar moves the child. This
value is only meaningful if the scrolling policy is`XmAUTOMATIC`.
* **`XmNO_SCROLL`** 

Indicates that the child does not move with the ScrollBars. This
value is only meaningful if the scrolling policy is`XmAUTOMATIC`.


### Inherited Resources


ScrolledWindow inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


The application must use the ScrollBar callbacks to be notified of user
input.

ScrolledWindow defines a callback structure
for use with`XmNtraverseObscuredCallback`callbacks. The`XmNtraverseObscuredCallback`resource provides a mechanism
for traversal to obscured widgets (or gadgets) due to their
position in the work area of a ScrolledWindow. The`XmNtraverseObscuredCallback`routine has responsibility
for adjusting the position of the work area such that the
specified traversal destination widget is positioned within
the viewport of the ScrolledWindow. A NULL`XmNtraverseObscuredCallback`resource causes obscured widgets within the ScrolledWindow
to be nontraversable.

Traversal to an obscured widget or gadget requires these
conditions to be met: the widget or gadget can be obscured
only due to its position in the work area of a ScrolledWindow
relative to the viewport; the viewport of the associated ScrolledWindow is
fully visible, or can be made so by virtue of ancestral`XmNtraverseObscuredCallback`routines; and the`XmNtraverseObscuredCallback`resource must be non-NULL.

When ScrolledWindow widgets are nested, the`XmNtraverseObscuredCallback`routine for each ScrolledWindow
that obscures the traversal destination is called in ascending order
within the given hierarchy.

A pointer to the following structure is passed to callbacks
for`XmNtraverseObscuredCallback`.typedef struct
{
        int`reason`;
        XEvent *`event`:
        Widget`traversal_destination`;
        XmTraversalDirectiondirection;
} XmTraverseObscuredCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`traversal_destination`** 

Specifies the widget or gadget to traverse to, which will
be a descendant of the work window.
* **direction** 

Specifies the direction of traversal. See
the description of thedirectionparameter in
the`XmProcessTraversal`reference page for an explanation
of the valid values.

### Translations


`XmScrolledWindow`includes the translations from`XmManager`.
### Additional Behavior


This widget has the following additional behavior:

* **KeyosfPageUp:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window up the height of the viewport.
The distance scrolled may be reduced to provide some overlap.
The actual distance scrolled depends on the`XmNpageIncrement`resource of the vertical ScrollBar.
* **KeyosfPageDown:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window down the height of the viewport.
The distance scrolled may be reduced to provide some overlap.
The actual distance scrolled depends on the`XmNpageIncrement`resource of the vertical ScrollBar.
* **KeyosfPageLeft:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window left the width of the viewport.
The distance scrolled may be reduced to provide some overlap.
The actual distance scrolled depends on the`XmNpageIncrement`resource of the horizontal ScrollBar.
* **KeyosfPageRight:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window right the width of the viewport.
The distance scrolled may be reduced to provide some overlap.
The actual distance scrolled depends on the`XmNpageIncrement`resource of the horizontal ScrollBar.
* **KeyosfBeginLine:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window horizontally to the edge corresponding to the
horizontal ScrollBar's minimum value.
* **KeyosfEndLine:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window horizontally to the edge corresponding to the
horizontal ScrollBar's maximum value.
* **KeyosfBeginData:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window vertically to the edge corresponding to the
vertical ScrollBar's minimum value.
* **KeyosfEndData:** 

If`XmNscrollingPolicy`is`XmAUTOMATIC`,
scrolls the window vertically to the edge corresponding to the
vertical ScrollBar's maximum value.


Certain applications will want to replace the page bindings with
ones that are specific to the content of the scrolled area.
### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmCreateScrolledWindow;,
&cdeman.XmManager;,
&cdeman.XmProcessTraversal;,
&cdeman.XmScrollBar;,
&cdeman.XmScrollVisible;, and
&cdeman.XmScrolledWindowSetAreas;.