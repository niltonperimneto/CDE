# XmDropSite
library call`XmDropSite`The DropSite RegistryXmDropSite&npzwc;#include &lt;Xm/DragDrop.h>
## DESCRIPTION


A client registers a widget or gadget as a drop site using the`XmDropSiteRegister`function. In addition, this routine
defines the behavior and capabilities of a drop site by specifying
appropriate resources. For example, the`XmNimportTargets`and`XmNnumImportTargets`resources identify respectively the
selection target types and number of types supported by a drop
site. The visual animation effects associated with
a drop site are also described with DropSite resources.

Drop site animation effects that occur in response to the pointer
entering a valid drop site are called drag-under effects. A receiver
can select from several animation styles supplied by the toolkit or
can provide customized animation effects. Drag-under effects supplied
by the toolkit include border highlighting, shadow in/out drawing,
and pixmap representation.

When a preregister drag protocol style is used, the toolkit generates
drag-under visual effects based on the value of the`XmNanimationStyle`resource. In dynamic mode, if the drop site`XmNdragProc`resource is NULL, the toolkit also provides animation effects based on
the`XmNanimationStyle`resource. Otherwise, if the`XmNdragProc`routine is specified, the receiver can either assume
responsibility for animation effects (through the`XmNdragProc`routine)
or rely on the toolkit to provide animation.
An application can either handle all or none of the animation effects
for a particular drop site.
That is, an application should never do a partial job of animation
on a particular drop site.

Drop sites may overlap. The initial stacking order corresponds to the
order in which the drop sites were registered. When a drop site
overlaps another drop site, the drag-under effects of the
drop site underneath are clipped by the obscuring drop site(s).

The`XmDropSiteUpdate`routine sets resources for a widget
that is registered as a drop site.`XmDropSiteRetrieve`gets drop
site resource values previously specified for a registered widget. These
routines are used instead of`XtSetValues`and`XtGetValues`.
### Classes


XmDropSite does not inherit from any widget class.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. To reference a resource by name
or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix
and use the remaining letters (in either lowercase or uppercase, but include
any underscores between words). The codes in the access column
indicate if the given resource can be set at creation time (C),
set by using`XmDropSiteUpdate`(S), retrieved by using`XmDropSiteRetrieve`(G), or is not applicable (N/A).

`XmDropSite Resource Set``Name``Class``Type``Default``Access`XmNanimationMaskXmCAnimationMaskPixmapXmUNSPECIFIED_PIXMAPCSGXmNanimationPixmapXmCAnimationPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNanimationPixmapDepthXmCAnimationPixmapDepthint0CSGXmNanimationStyleXmCAnimationStyleunsigned charXmDRAG_UNDER_HIGHLIGHTCSGXmNdragProcXmCDragProcXtCallbackProcNULLCSGXmNdropProcXmCDropProcXtCallbackProcNULLCSGXmNdropRectanglesXmCDropRectanglesXRectangle *dynamicCSGXmNdropSiteActivityXmCDropSiteActivityunsigned charXmDROP_SITE_ACTIVECSGXmNdropSiteOperationsXmCDropSiteOperationsunsigned charXmDROP_MOVE | XmDROP_COPYCSGXmNdropSiteTypeXmCDropSiteTypeunsigned charXmDROP_SITE_SIMPLECGXmNimportTargetsXmCImportTargetsAtom *NULLCSGXmNnumDropRectanglesXmCNumDropRectanglesCardinal1CSGXmNnumImportTargetsXmCNumImportTargetsCardinal0CSG

* **`XmNanimationMask`** 

Specifies a mask to use with the pixmap specified
by`XmNanimationPixmap`when the animation style is`XmDRAG_UNDER_PIXMAP`.
* **`XmNanimationPixmap`** 

Specifies a pixmap for drag-under animation when the
animation style is`XmDRAG_UNDER_PIXMAP`. The pixmap
is drawn with its origin at the upper left corner of the
bounding box of the drop site. If the drop site window
is larger than the animation pixmap, the portion of the
window not covered by the pixmap will be tiled with the
window's background color.
* **`XmNanimationPixmapDepth`** 

Specifies the depth of the pixmap specified by the`XmNanimationPixmap`resource. When the depth is 1, the
colors are taken from the foreground and background of the
drop site widget. For any other value, drop site animation
occurs only if the`XmNanimationPixmapDepth`matches the
depth of the drop site window. Colors are derived from the
current colormap.
* **`XmNanimationStyle`** 

Specifies the drag-under animation style used when a drag enters
a valid drop site. The possible values are

* **`XmDRAG_UNDER_HIGHLIGHT`** 

The drop site uses highlighting effects.
* **`XmDRAG_UNDER_SHADOW_OUT`** 

The drop site uses an outset shadow.
* **`XmDRAG_UNDER_SHADOW_IN`** 

The drop site uses an inset shadow.
* **`XmDRAG_UNDER_PIXMAP`** 

The drop site uses the pixmap specified by`XmNanimationPixmap`to indicate that it can receive the drop.
* **`XmDRAG_UNDER_NONE`** 

The drop site does not use animation effects. A client
using a dynamic protocol, may provide drag-under
effects in its`XmNdragProc`routine.

* **`XmNdragProc`** 

Specifies the procedure that is invoked when the drop site
receives a crossing, motion, or operation changed message. This
procedure is called only when a dynamic protocol is used.
The type of structure whose address is passed to this
procedure isXmDragProcCallbackStruct.
The reason sent to the procedure is one of
the following:

`XmCR_DROP_SITE_ENTER_MESSAGE`

`XmCR_DROP_SITE_LEAVE_MESSAGE`

`XmCR_DRAG_MOTION`

`XmCR_OPERATION_CHANGED`

The drag procedure may change the values of some members of theXmDragProcCallbackStructpassed to it.
After the drag procedure returns, the toolkit uses the final values in
initializing some members of the callback structure passed to the
appropriate callbacks of the initiator (the DragContext's`XmNdropSiteEnterCallback`,`XmNdropSiteLeaveCallback`,`XmNdragMotionCallback`, or`XmNoperationChangedCallback`callbacks).
* **`XmNdropProc`** 

Specifies the procedure that is invoked when a drop
(excluding a cancel or interrupt action) occurs
on a drop site regardless of the status of the drop site. The
type of the structure whose address is passed to this procedure
isXmDropProcCallbackStruct. The reason
sent to the procedure is`XmCR_DROP_MESSAGE`.

The drop procedure may change the values of some members of theXmDropProcCallbackStructpassed to it.
After the drop procedure returns, the toolkit uses the final values in
initializing some members of theXmDropStartCallbackStructpassed
to the initiator's drop start callbacks (the DragContext's`XmNdropStartCallback`callbacks).
* **`XmNdropRectangles`** 

Specifies a list of rectangles that describe the shape of a
drop site. The locations of the rectangles are relative to the
origin of the enclosing object. When`XmNdropRectangles`is NULL,
the drop site is assumed to be the sensitive area of
the enclosing widget. If`XmNdropSiteType`is`XmDROP_SITE_COMPOSITE`, this resource cannot be specified by
the application.

Retrieving this resource returns allocated memory that needs to be
freed with the`XtFree`function.
* **`XmNdropSiteActivity`** 

Indicates whether a drop site is active or inactive. The values
are`XmDROP_SITE_ACTIVE`,`XmDROP_SITE_INACTIVE`, and`XmDROP_SITE_IGNORE`.
An
active drop site can receive a drop, whereas an inactive drop
site is dormant. An inactive drop site is treated as if it was
not a registered drop site and any drag-under visuals associated
with entering or leaving the drop site do not occur. However, it
is still used for clipping drag-under effects.
A value of`XmDROP_SITE_IGNORE`indicates that a drop site should
be ignored for all purposes.
* **`XmNdropSiteOperations`** 

Specifies the set of valid operations associated with a drop site.
This resource is a bit mask that is formed by combining one or
more of the following values using a bitwise operation such as
inclusive OR (|):`XmDROP_COPY`,`XmDROP_LINK`, and`XmDROP_MOVE`.
The value`XmDROP_NOOP`for this resource indicates that no
operations are valid.
* **`XmNdropSiteType`** 

Specifies the type of the drop site. The possible values are

* **`XmDROP_SITE_SIMPLE`** 

The widget does not have any additional children that
are registered as drop sites.
* **`XmDROP_SITE_COMPOSITE`** 

The widget will have children that are registered as drop
sites.

* **`XmNimportTargets`** 

Specifies the list of target atoms that this drop site accepts.
* **`XmNnumDropRectangles`** 

Specifies the number of rectangles in the`XmNdropRectangles`list. If the drop site type is`XmDROP_SITE_COMPOSITE`, this resource
can not be specified by the application.
* **`XmNnumImportTargets`** 

Specifies the number of atoms in the target atom list.

### Callback Information


A pointer to the following structure is passed to the`XmNdragProc`routine when the drop site receives
crossing, motion, or operation changed messages:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        Widget`dragContext`;
        Positionx;
        Positiony;
        unsigned char`dropSiteStatus`;
        unsigned char`operation`;
        unsigned char`operations`;
        Boolean`animate`;
} XmDragProcCallbackStruct, *XmDragProcCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the timestamp of the logical event.
* **`dragContext`** 

Specifies the ID of the DragContext widget associated with
the transaction.
* **x** 

Indicates the x-coordinate of the pointer relative
to the drop site.
* **y** 

Indicates the y-coordinate of the pointer relative
to the drop site.
* **`dropSiteStatus`** 

An IN/OUT member that indicates whether or not a drop site is valid.

When`reason`is`XmCR_DROP_SITE_ENTER_MESSAGE`or`XmCR_OPERATION_CHANGED`,
or`reason`is`XmCR_DRAG_MOTION`or`XmCR_DROP_SITE_LEAVE_MESSAGE`and
the pointer is not in the same drop site as on the previous invocation
of the drag procedure,
the toolkit initializes`dropSiteStatus`to`XmDROP_SITE_VALID`if the DragContext's`XmNexportTargets`and the DropSite's`XmNimportTargets`are compatible and if the initial value of the`operation`member is not`XmDROP_NOOP`.
Otherwise, the toolkit initializes`dropSiteStatus`to`XmDROP_SITE_INVALID`.

When the`reason`is`XmCR_DRAG_MOTION`or`XmCR_DROP_SITE_LEAVE_MESSAGE`and the pointer is within the same
drop site as on the previous invocation of the drag procedure, the
toolkit initializes`dropSiteStatus`to the value of`dropSiteStatus`at the time the previous invocation of the drag
procedure returns.

The drag procedure may change the value of this member.
After the drag procedure returns, the toolkit uses the final value in
initializing the`dropSiteStatus`member of the callback struct
passed to the appropriate callbacks of the initiator.
* **`operation`** 

An IN/OUT member that identifies an operation.

The toolkit initializes`operation`by selecting an operation
from the bitwise AND of the initial value of the`operations`member
and the value of the DropSite's`XmNdropSiteOperations`resource.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If the toolkit finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.

The drag procedure may change the value of this member.
After the drag procedure returns, the toolkit uses the final value in
initializing the`operation`member of the callback struct
passed to the appropriate callbacks of the initiator.
* **`operations`** 

An IN/OUT member that indicates the set of operations supported for the
source data.

If the user does not select an operation (by pressing a modifier key),
the toolkit initializes`operations`to the value of the
DragContext's`XmNdragOperations`resource.
If the user does select an operation, the toolkit initializes`operations`to the bitwise AND of the corresponding operation and
the value of the DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

The drag procedure may change the value of this member.
After the drag procedure returns, the toolkit uses the final value in
initializing the`operations`member of the callback struct
passed to the appropriate callbacks of the initiator.
* **`animate`** 

An OUT member that indicates whether the toolkit or
the receiver client provides drag-under effects for a valid
drop site. If`animate`is set to True, the toolkit provides
drop site animation per the`XmNanimationStyle`resource value; if it is set to False,
the receiver generates drag-under animation effects.


A pointer to the following structure is passed to the`XmNdropProc`routine when the drop site receives a drop message:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        Widget`dragContext`;
        Positionx;
        Positiony;
        unsigned char`dropSiteStatus`;
        unsigned char`operation`;
        unsigned char`operations`;
        unsigned char`dropAction`;
} XmDropProcCallbackStruct, *XmDropProcCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Specifies the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the timestamp of the logical event.
* **`dragContext`** 

Specifies the ID of the DragContext widget associated with the
transaction.
* **x** 

Indicates the x-coordinate of the pointer relative to the
drop site.
* **y** 

Indicates the y-coordinate of the pointer relative to the
drop site.
* **`dropSiteStatus`** 

An IN/OUT member that indicates whether or not a drop site is valid.

The toolkit initializes`dropSiteStatus`to`XmDROP_SITE_VALID`if the DragContext's`XmNexportTargets`and the DropSite's`XmNimportTargets`are compatible and if the initial value of the`operation`member is not`XmDROP_NOOP`.
Otherwise, the toolkit initializes`dropSiteStatus`to`XmDROP_SITE_INVALID`.

The drop procedure may change the value of this member.
After the drop procedure returns, the toolkit uses the final value in
initializing the`dropSiteStatus`member of theXmDropStartCallbackStructpassed to the initiator's drop start
callbacks (the DragContext's`XmNdropStartCallback`callbacks).
* **`operation`** 

An IN/OUT member that identifies an operation.

The toolkit initializes`operation`by selecting an operation
from the bitwise AND of the initial value of the`operations`member
and the value of the DropSite's`XmNdropSiteOperations`resource.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If it finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.

The drop procedure may change the value of this member.
After the drop procedure returns, the toolkit uses the final value in
initializing the`operation`member of theXmDropStartCallbackStructpassed to the initiator's drop start
callbacks (the DragContext's`XmNdropStartCallback`callbacks).
* **`operations`** 

An IN/OUT member that indicates the set of operations supported for the
source data.

If the user does not select an operation (by pressing a modifier key),
the toolkit initializes`operations`to the value of the
DragContext's`XmNdragOperations`resource.
If the user does select an operation, the toolkit initializes`operations`to the bitwise AND of the corresponding operation and
the value of the DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

The drop procedure may change the value of this member.
After the drop procedure returns, the toolkit uses the final value in
initializing the`operations`member of theXmDropStartCallbackStructpassed to the initiator's drop start
callbacks (the DragContext's`XmNdropStartCallback`callbacks).
* **`dropAction`** 

An IN/OUT member that identifies the action associated with the drop.
The possible values are

* **`XmDROP`** 

A drop was attempted. If the drop site is valid, drop transfer
handling proceeds.
* **`XmDROP_HELP`** 

The user has requested help on the drop site.


The drop procedure may change the value of this member.
After the drop procedure returns, the toolkit uses the final value in
initializing the`dropAction`member of theXmDropStartCallbackStructpassed to the initiator's drop start
callbacks (the DragContext's`XmNdropStartCallback`callbacks).

## RELATED INFORMATION


&cdeman.XmDragContext;,
&cdeman.XmDragIcon;,
&cdeman.XmDropSiteConfigureStackingOrder;,
&cdeman.XmDropSiteEndUpdate;,
&cdeman.XmDropSiteQueryStackingOrder;,
&cdeman.XmDropSiteRegister;,
&cdeman.XmDropSiteStartUpdate;,
&cdeman.XmDropSiteUpdate;,
&cdeman.XmDropSiteUnregister;,
&cdeman.XmDropTransfer;, and
&cdeman.XmTargetsAreCompatible;.