# XmBulletinBoard
library call`XmBulletinBoard`The BulletinBoard widget classXmBulletinBoardwidget classBulletinBoard&npzwc;#include &lt;Xm/BulletinB.h>
## DESCRIPTION


BulletinBoard is a composite widget that provides simple geometry
management for child widgets.
It does not force positioning on its children, but can be set to reject
geometry requests that result in overlapping children.
BulletinBoard is the base widget for most dialog widgets and is also used
as a general container widget.

Modal and modeless dialogs are implemented as collections of widgets that
include a DialogShell, a BulletinBoard (or subclass) child of the shell,
and various dialog components (buttons, labels, and so on) that are children
of BulletinBoard.
BulletinBoard defines callbacks useful for dialogs (focus, map, unmap),
which are available for application use.
If its parent is a DialogShell, BulletinBoard passes title and input mode
(based on dialog style)
information to the parent, which is responsible for appropriate communication
with the window manager.

The default value for`XmNinitialFocus`is the value of`XmNdefaultButton`.

BulletinBoard uses the`XmQTtakesDefault`trait, and
holds the`XmQTdialogShellSavvy`and`XmQTspecifyRenderTable`traits.
### Classes


BulletinBoard inherits behavior, resources, and traits
from the`Core`,`Composite`,`Constraint`,
and`XmManager`classes.

The class pointer is`xmBulletinBoardWidgetClass`.

The class name is`XmBulletinBoard`.
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

`XmBulletinBoard Resource Set``Name``Class``Type``Default``Access`XmNallowOverlapXmCAllowOverlapBooleanTrueCSGXmNautoUnmanageXmCAutoUnmanageBooleanTrueCGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNcancelButtonXmCWidgetWidgetNULLSGXmNdefaultButtonXmCWidgetWidgetNULLSGXmNdefaultPositionXmCDefaultPositionBooleanTrueCSGXmNdialogStyleXmCDialogStyleunsigned chardynamicCSGXmNdialogTitleXmCDialogTitleXmStringNULLCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTabledynamicCSGXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension10CSGXmNmarginWidthXmCMarginWidthDimension10CSGXmNnoResizeXmCNoResizeBooleanFalseCSGXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_ANYCSGXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_OUTCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNtextTranslationsXmCTranslationsXtTranslationsNULLCXmNunmapCallbackXmCCallbackXtCallbackListNULLC

* **`XmNallowOverlap`** 

Controls the policy for overlapping child widgets.
If this resource is True, BulletinBoard allows geometry requests that result
in overlapping children.
* **`XmNautoUnmanage`** 

Controls whether or not BulletinBoard is automatically unmanaged after a
button is activated.
If this resource is True on initialization and if the BulletinBoard's
parent is a DialogShell, BulletinBoard adds a callback to button
children (PushButtons, PushButtonGadgets, and DrawnButtons) that
unmanages the BulletinBoard when a button is activated.
If this resource is False on initialization or if the BulletinBoard's
parent is not a DialogShell, the
BulletinBoard is not automatically unmanaged.
For BulletinBoard subclasses with Apply or Help buttons, activating
those buttons does not automatically unmanage the BulletinBoard.
* **`XmNbuttonFontList`** 

Specifies the font list used for button descendants. See the`XmNbuttonRenderTable`resource.
* **`XmNbuttonRenderTable`** 

Specifies the render table used for BulletinBoard's button descendants.
If this value is NULL at initialization, the parent hierarchy of the widget
is searched for an ancestor that
holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found, the render table is initialized to the`XmBUTTON_RENDER_TABLE`value of the ancestor widget. If no such
ancestor is
found, the default is implementation dependent. Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNcancelButton`** 

Specifies the widget ID of the`Cancel`button. BulletinBoard's
subclasses, which define a`Cancel`button,
set this resource. BulletinBoard does not directly provide any behavior for
that button.
* **`XmNdefaultButton`** 

Specifies the widget ID of the default button.
Some BulletinBoard subclasses, which define a default button,
set this resource. BulletinBoard defines translations and installs
accelerators that activate that button when`KActivate`is pressed
and the keyboard focus is not in another button.
Controls the positioning of a DialogShell managing a BulletinBoard.
If the BulletinBoard is not being managed by a DialogShell, then
this resource has no effect. If`XmNdefaultPosition`is True,
the DialogShell will center itself at the center of its own parent.
For example, if the parent of the DialogShell is an ApplicationShell,
then the center of the DialogShell will be at the same coordinates
as the center of the ApplicationShell.
If the DialogShell becomes unmapped (but stays managed) and then remapped,
this resource has no influence on the DialogShell's position.
If this resource is False, the DialogShell does not automatically
center itself.
* **`XmNdialogStyle`** 

Indicates the dialog style associated with the BulletinBoard.
If the parent of the BulletinBoard is a DialogShell, the parent's`XmNmwmInputMode`is set according to the value of this resource.
This resource can be set only if the BulletinBoard is unmanaged.
Possible values for this resource include the following:

* **`XmDIALOG_SYSTEM_MODAL`** 

Used for dialogs that
must be responded to before
any other interaction in any application.
* **`XmDIALOG_PRIMARY_APPLICATION_MODAL`** 

Used for dialogs that must
be responded to before some other interactions in
ancestors of the widget.
* **`XmDIALOG_APPLICATION_MODAL`** 

Used for dialogs that must be
responded to before some other interactions in
ancestors of the widget. This value is the same as`XmDIALOG_PRIMARY_APPLICATION_MODAL`, and remains for compatibility.
* **`XmDIALOG_FULL_APPLICATION_MODAL`** 

Used for dialogs that must be
responded to before some other interactions in
the same application.
* **`XmDIALOG_MODELESS`** 

Used for dialogs that do not interrupt interaction
of any application.
This is the default when the parent of the BulletinBoard is a DialogShell.
* **`XmDIALOG_WORK_AREA`** 

Used for BulletinBoard widgets whose
parents are not DialogShells.`XmNdialogStyle`is forced to have this value when the parent of the
BulletinBoard is not a DialogShell.


Posting a modal dialog in response to a button down or key down event
(via translation actions or callbacks) can cause the corresponding
button up or key up event to be lost. For example, posting a modal
dialog from an`XmNincrementCallback`of`XmScrollBar`will cause
the loss of the button up event, causing the`XmScrollBar`to
auto-increment indefinitely.
* **`XmNdialogTitle`** 

Specifies the dialog title. If this resource is not NULL, and the parent
of the BulletinBoard is a subclass of WMShell, BulletinBoard sets the`XmNtitle`and`XmNtitleEncoding`of its parent.
If the only character set in`XmNdialogTitle`is ISO8859-1,`XmNtitle`is set to the string of the title, and`XmNtitleEncoding`is set to`STRING`.
If`XmNdialogTitle`contains character sets other than ISO8859-1,`XmNtitle`is set to the string of the title converted to a compound
text string, and`XmNtitleEncoding`is set to`COMPOUND_TEXT`.
The direction of the title is based on the`XmNlayoutDirection`resource
of the widget.
* **`XmNfocusCallback`** 

Specifies the list of callbacks that is called
when the BulletinBoard widget or one of its
descendants accepts the input focus.
The callback reason is`XmCR_FOCUS`.
* **`XmNlabelFontList`** 

Specifies the font list used for label descendants. See the`XmNlabelRenderTable`resource.
* **`XmNlabelRenderTable`** 

Specifies the render table used for BulletinBoard's label descendants.
If this value is NULL at initialization, the parent hierarchy of the widget
is searched for an ancestor that
holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found, the render table is initialized to the`XmLABEL_RENDER_TABLE`value of the ancestor widget. If no such
ancestor is
found, the default is implementation dependent. Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNmapCallback`** 

Specifies the list of callbacks that is called
only when the parent of the BulletinBoard is a DialogShell. In this case,
this callback list is invoked when the BulletinBoard widget is mapped.
The callback reason is`XmCR_MAP`.
DialogShells are usually mapped when the DialogShell is managed.
* **`XmNmarginHeight`** 

Specifies the minimum spacing in pixels between the top or bottom edge
of BulletinBoard and any child widget.
* **`XmNmarginWidth`** 

Specifies the minimum spacing in pixels between the left or right edge
of BulletinBoard and any child widget.
* **`XmNnoResize`** 

Controls whether or not resize controls are included in the window
manager frame around the BulletinBoard's parent.
If this resource is
set to True,mwmdoes not include resize controls in the
window manager frame containing the parent of the BulletinBoard if the
parent is a subclass of VendorShell.
If this resource is set to False,
the window manager frame does include resize controls.
Other controls provided bymwmcan be included or excluded through
themwmresources provided by VendorShell.
* **`XmNresizePolicy`** 

Controls the policy for resizing BulletinBoard widgets.
Possible values include

* **`XmRESIZE_NONE`** 

Fixed size
* **`XmRESIZE_ANY`** 

Shrink or grow as needed
* **`XmRESIZE_GROW`** 

Grow only

* **`XmNshadowType`** 

Describes the shadow
drawing style for BulletinBoard. This resource can have the
following values:

* **`XmSHADOW_IN`** 

Draws the BulletinBoard shadow
so that it appears inset.
This means that the bottom shadow visuals and top shadow visuals
are reversed.
* **`XmSHADOW_OUT`** 

Draws the BulletinBoard shadow
so that it appears outset.
* **`XmSHADOW_ETCHED_IN`** 

Draws the BulletinBoard shadow
using a double line giving the
effect of a line etched into the window, similar to the Separator widget.
* **`XmSHADOW_ETCHED_OUT`** 

Draws the BulletinBoard shadow using a double
line giving the
effect of a line coming out of the window, similar to the Separator widget.

* **`XmNtextFontList`** 

Specifies the font list used for text descendants. See the`XmNtextRenderTable`resource.
* **`XmNtextRenderTable`** 

Specifies the render table used for BulletinBoard's text descendants.
If this value is NULL at initialization, the parent hierarchy of the widget
is searched for an ancestor that
holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found, the render table is initialized to the`XmTEXT_RENDER_TABLE`value of the ancestor widget. If no such
ancestor is
found, the default is implementation dependent. Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNtextTranslations`** 

It adds translations to any Text widget or Text widget subclass that
is added as a child of BulletinBoard.
* **`XmNunmapCallback`** 

Specifies the list of callbacks that is called only when the parent of
the BulletinBoard is a DialogShell.
In this case, this callback list is invoked when the BulletinBoard
widget is unmapped.
The callback reason is`XmCR_UNMAP`.
DialogShells are usually unmapped when the DialogShell is unmanaged.

### Inherited Resources


BulletinBoard inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetdynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicN/AXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback

### Translations


`XmBulletinBoard`includes the translations from`XmManager`.
### Additional Behavior


The`XmBulletinBoard`widget has the following additional behavior:

* **KeyosfCancel:** 

Calls the activate callbacks for the cancel button if it is sensitive.
If no cancel button exists and if the parent of the BulletinBoard is
a manager, passes the event to the parent.
* **KeyosfActivate:** 

Calls the activate callbacks for the button with the keyboard focus.
If no button has the keyboard focus, calls the activate callbacks
for the default button if it is sensitive.
In a List widget or single-line Text widget,
the List or Text action associated withKeyosfActivateis called before the BulletinBoard actions associated withKeyosfActivate.

In a multiline Text widget, anyKeyosfActivateevent exceptKeyEntercalls
the Text action associated withKeyosfActivate,
then the BulletinBoard actions associated withKeyosfActivate.
If no button has the focus, no default button exists, and the parent of the
BulletinBoard is a manager, passes the event to the parent.
* **FocusIn:** 

Calls the callbacks for`XmNfocusCallback`.
When the focus policy is`XmPOINTER`, the callbacks are
called when the pointer
enters the window.
When the focus policy is`XmEXPLICIT`, the callbacks are called
when the user traverses to the widget.
* **Map:** 

Calls the callbacks for`XmNmapCallback`.
* **Unmap:** 

Calls the callbacks for`XmNunmapCallback`.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;,
&cdeman.Core;, &cdeman.XmCreateBulletinBoard;,
&cdeman.XmCreateBulletinBoardDialog;,
&cdeman.XmDialogShell;, and &cdeman.XmManager;.