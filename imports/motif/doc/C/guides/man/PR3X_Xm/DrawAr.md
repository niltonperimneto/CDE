# XmDrawingArea
library call`XmDrawingArea`The DrawingArea widget classXmDrawingAreawidget classDrawingArea&npzwc;#include &lt;Xm/DrawingA.h>
## DESCRIPTION


DrawingArea is an empty widget that is easily adaptable
to a variety of purposes.
It does no drawing and defines no behavior except for invoking
callbacks.
Callbacks notify the application when graphics need to be drawn
(exposure events or widget resize) and when the widget receives input from
the keyboard or mouse.

Applications are responsible for defining appearance and behavior as needed
in response to DrawingArea callbacks.

DrawingArea is also a composite widget and subclass of`XmManager`that supports
minimal geometry management for multiple widget or gadget children.

DrawingArea uses the`XmNinitialFocus`resource of`XmManager`to define whether or not DrawingArea will receive focus when it is
traversed to, even if it has traversable children. If`XmNinitialFocus`is NULL, DrawingArea receives focus only if it
does not have any traversable children. If`XmNinitialFocus`is
not NULL, then DrawingArea receives focus when traversed to. In the latter
case, the application first needs to be able to realize that the DrawingArea
will receive focus, then, as appropriate, needs to either call the`XmProcessTraversal`function for the desired child, or to navigate
across the private DrawingArea graphics objects.

The following resources are not currently used by the DrawingArea
widget:`XmNshadowThickness`,`XmNtopShadowPixmap`,`XmNbottomShadowPixmap`,`XmNtopShadowColor`, and`XmNbottomShadowColor`
### Data Transfer Behavior


DrawingArea has no widget class conversion or destination procedure.
Subclasses and the`XmNconvertCallback`procedures are responsible
for any conversion of selections.
Subclasses and the`XmNdestinationCallback`procedures are
responsible for any data transfers to the widget.
### Classes


DrawingArea inherits behavior and resources from the`Core`,`Composite`,`Constraint`, and`XmManager`classes.

The class pointer is`xmDrawingAreaWidgetClass`.

The class name is`XmDrawingArea`.
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

`XmDrawingArea Resource Set``Name``Class``Type``Default``Access`XmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNdestinationCallbackXmCCallbackXtCallbackListNULLCXmNexposeCallbackXmCCallbackXtCallbackListNULLCXmNinputCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension10CSGXmNmarginWidthXmCMarginWidthDimension10CSGXmNresizeCallbackXmCCallbackXtCallbackListNULLCXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_ANYCSG

* **`XmNconvertCallback`** 

Specifies a list of callbacks called when the DrawingArea is asked to
convert a selection.
The type of the structure whose address is passed to these callbacks isXmConvertCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNdestinationCallback`** 

Specifies a list of callbacks called when the DrawingArea is the
destination of a transfer operation.
The type of the structure whose address is passed to these callbacks isXmDestinationCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNexposeCallback`** 

Specifies the list of callbacks that is
called when DrawingArea receives an exposure event.
The callback reason is`XmCR_EXPOSE`.
The callback structure also includes the exposure event.

The default bit gravity for Manager windows is`NorthWestGravity`.
This may cause the`XmNexposeCallback`procedures not to be invoked
when the DrawingArea window is made smaller.
* **`XmNinputCallback`** 

Specifies the list of callbacks that is
called when the DrawingArea receives a keyboard
or mouse event (key or button, up or down).
The callback reason is`XmCR_INPUT`.
The callback structure also includes the input event.
* **`XmNmarginHeight`** 

Specifies the minimum spacing in pixels between the top or bottom edge
of DrawingArea and any child widget.
* **`XmNmarginWidth`** 

Specifies the minimum spacing in pixels between the left or right edge
of DrawingArea and any child widget.
* **`XmNresizeCallback`** 

Specifies the list of callbacks that is
called when the DrawingArea is resized.
The callback reason is`XmCR_RESIZE`.
* **`XmNresizePolicy`** 

Controls the policy for resizing DrawingArea widgets.
Possible values include`XmRESIZE_NONE`(fixed size),`XmRESIZE_ANY`(shrink or grow as needed), and`XmRESIZE_GROW`(grow only).

### Inherited Resources


DrawingArea inherits behavior and resources from the following
superclasses. For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to the`XmNexposeCallback`,`XmNinputCallback`, and`XmNresizeCallback`procedures:typedef struct
{
        int`reason`;
        XEvent`* event`;
        Window`window`;
} XmDrawingAreaCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
This is NULL for the`XmNresizeCallback`.
* **`window`** 

Is set to the widget window.


A pointer to the following structure is passed to the`XmNconvertCallback`procedures:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Atom`selection`;
        Atom`target`;
        XtPointer`source_data`;
        XtPointer`location_data`;
        int`flags`;
        XtPointer`parm`;
        int`parm_format`;
        unsigned long`parm_length`;
        int`status`;
        XtPointer`value`;
        Atom`type`;
        int`format`;
        unsigned long`length`;
} XmConvertCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL.
* **`selection`** 

Indicates the selection for which conversion is being requested.
Possible values are`CLIPBOARD`,`PRIMARY`,`SECONDARY`,
and_MOTIF_DROP.
* **`target`** 

Indicates the conversion target.
* **`source_data`** 

Contains information about the selection source.
When the selection is_MOTIF_DROP,`source_data`is the
DragContext.
Otherwise, it is NULL.
* **`location_data`** 

Contains information about the location of data to be converted.
If the value is NULL, the data to be transferred consists of the
widget's current selection.
* **`flags`** 

Indicates the status of the conversion. Following are the possible
values:

* **`XmCONVERTING_NONE`** 

This flag is currently unused.
* **`XmCONVERTING_PARTIAL`** 

The target widget was able to be converted, but some data was lost.
* **`XmCONVERTING_SAME`** 

The conversion target is the source of the data to be transferred.
* **`XmCONVERTING_TRANSACT`** 

This flag is currently unused.

* **`parm`** 

Contains parameter data for this target.
If no parameter data exists, the value is NULL.

When`selection`is`CLIPBOARD`and`target`is_MOTIF_CLIPBOARD_TARGETSor_MOTIF_DEFERRED_CLIPBOARD_TARGETS, the value is the requested
operation (`XmCOPY`,`XmMOVE`, or`XmLINK`).
* **`parm_format`** 

Specifies whether the data in`parm`should be viewed
as a list of`char`,`short`, or`long`quantities.
Possible values are 0 (when`parm`is NULL),
8 (when the data in`parm`should be viewed as a list of`char`s),
16 (when the data in`parm`should be viewed as a list of`short`s),
or 32 (when the data in`parm`should be viewed as a list of`long`s).
Note that`parm_format`symbolizes a data type, not the number of bits
in each list element.
For example, on some machines, a`parm_format`of 32 means that
the data in`parm`should be viewed as a list of 64-bit quantities,
not 32-bit quantities.
* **`parm_length`** 

Specifies the number of elements of data in`parm`, where each
element has the size specified by`parm_format`.
When`parm`is NULL, the value is 0.
* **`status`** 

An IN/OUT member that specifies the status of the conversion.
The initial value is`XmCONVERT_DEFAULT`.
The callback procedure can set this member to one of the following
values:

* **`XmCONVERT_DEFAULT`** 

This value means that the widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it
overwrites the data provided by the callback procedures in the`value`member.
* **`XmCONVERT_MERGE`** 

This value means that the widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it appends
its data to the data provided by the callback procedures in the`value`member.
This value is intended for use with targets that result in lists of
data, such as`TARGETS`.
* **`XmCONVERT_DONE`** 

This value means that the callback procedure has successfully finished
the conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.
* **`XmCONVERT_REFUSE`** 

This value means that the callback procedure has terminated the
conversion process without completing the requested conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.

* **`value`** 

An IN/OUT parameter that contains any data that the callback procedure
produces as a result of the conversion.
The initial value is NULL.
If the callback procedure sets this member, it must ensure that the`type`,`format`, and`length`members correspond
to the data in`value`.
The callback procedure is responsible for allocating, but not for
freeing, memory when it sets this member.
* **`type`** 

An IN/OUT parameter that indicates the type of the data in the`value`member.
The initial value is`INTEGER`.
* **`format`** 

An IN/OUT parameter that specifies whether the data in`value`should
be viewed as a list of`char`,`short`, or`long`quantities.
The initial value is 8.
The callback procedure can set this member to 8 (for a list of`char`),
16 (for a list of`short`), or 32 (for a list of`long`).
* **`length`** 

An IN/OUT member that specifies the number of elements of data in`value`, where each element has the size symbolized by`format`.
The initial value is 0.


A pointer to the following callback structure is passed to the`XmNdestinationCallback`procedures:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Atom`selection`;
        XtEnum`operation`;
        int`flags`;
        XtPointer`transfer_id`;
        XtPointer`destination_data`;
        XtPointer`location_data`;
        Time`time`;
} XmDestinationCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL.
* **`selection`** 

Indicates the selection for which data transfer is being requested.
Possible values are`CLIPBOARD`,`PRIMARY`,`SECONDARY`, and_MOTIF_DROP.
* **`operation`** 

Indicates the type of transfer operation requested.

When the selection is`PRIMARY`, possible values are`XmMOVE`,`XmCOPY`, and`XmLINK`.

When the selection is`SECONDARY`or`CLIPBOARD`, possible
values are`XmCOPY`and`XmLINK`.

When the selection is_MOTIF_DROP, possible values are`XmMOVE`,`XmCOPY`,`XmLINK`, and`XmOTHER`.
A value of`XmOTHER`means that the callback procedure must get
further information from theXmDropProcCallbackStructin the`destination_data`member.
* **`flags`** 

Indicates whether or not the destination widget is also the source of
the data to be transferred.
Following are the possible values:

* **`XmCONVERTING_NONE`** 

The destination widget is not the source of the data to be transferred.
* **`XmCONVERTING_SAME`** 

The destination widget is the source of the data to be transferred.

* **`transfer_id`** 

Serves as a unique ID to identify the transfer transaction.
* **`destination_data`** 

Contains information about the destination.
When the selection is_MOTIF_DROP, the callback procedures are
called by the drop site's`XmNdropProc`, and`destination_data`is a pointer to theXmDropProcCallbackStructpassed to the`XmNdropProc`procedure.
When the selection is`SECONDARY`,`destination_data`is an Atom
representing a target recommmended by the selection owner for use in
converting the selection.
Otherwise,`destination_data`is NULL.
* **`location_data`** 

Contains information about the location where data is to be transferred.
The value is always NULL when the selection is`SECONDARY`or`CLIPBOARD`.
If the value is NULL, the data is to be inserted at the widget's cursor
position.`location_data`is only valid for the duration of a
transfer. Once`XmTransferDone`procedures start to be called,`location_data`will no longer be stable.
* **`time`** 

Indicates the time when the transfer operation began.

### Translations


XmDrawingArea inherits translations from XmManager.
Before calling the XmManager actions, all events in the inherited
translations exceptBtnMotion,EnterWindow,LeaveWindow,FocusIn, andFocusOutalso call theDrawingAreaInput()action.

XmDrawingArea has the following additional translations.
The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **BtnDown:** 

DrawingAreaInput()
* **BtnUp:** 

DrawingAreaInput()
* **KeyDown:** 

DrawingAreaInput()
ManagerGadgetKeyInput()
* **KeyUp:** 

DrawingAreaInput()

### Action Routines


The`XmDrawingArea`action routines are

* **DrawingAreaInput():** 

Unless the event takes place in a gadget, calls the callbacks for`XmNinputCallback`
* **ManagerGadgetKeyInput():** 

Causes the current gadget to process a keyboard event

### Additional Behavior


The XmDrawingArea widget has the following additional behavior:

* **Expose:** 

Calls the callbacks for`XmNexposeCallback`
* **Widget&ensp;Resize:** 

Calls the callbacks for`XmNresizeCallback`

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;, &cdeman.Core;,
&cdeman.XmCreateDrawingArea;,
and &cdeman.XmManager;.