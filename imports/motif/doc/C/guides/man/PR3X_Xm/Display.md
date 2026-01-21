# XmDisplay
library call`XmDisplay`The Display widget classXmDisplaywidget classXmDisplay#include <Xm/Display.h>
## DESCRIPTION


The XmDisplay object is used by the Motif widgets to store information
that is specific to a display. It also allows the toolkit to access
certain information on widget hierarchies that would otherwise be
unavailable. Each client has one XmDisplay object for each display
it accesses.

An XmDisplay object is automatically created when the application
creates the first shell on a display (usually accomplished by a call to`XtAppInitialize`or`XtAppCreateShell`).
It is not necessary to create an XmDisplay object by any other means.
An application can use the function`XmGetXmDisplay`to obtain the
widget ID of the XmDisplay object for a given display.

An application cannot supply initial values for XmDisplay resources as
arguments to a call to any function that creates widgets.
The application or user can supply initial values in a resource file.
After creating the first shell on the display, the application can use`XmGetXmDisplay`to obtain the widget ID of the XmDisplay object and
then call`XtSetValues`to set the XmDisplay resources.

XmDisplay resources specify the drag protocol style for a client
participating in drag and drop transactions. The two basic
protocol types are preregister and dynamic. When a preregister protocol
is used, the toolkit handles any communication between the initiator
and receiver clients and displays the appropriate drag-over and
drag-under visual effects. A client registers its drop sites in
advance and this information is stored in a property for each
top-level window. When the drag pointer enters a top-level window,
the drop site information is read by the initiator. A dynamic
protocol allows the source and destination clients to dynamically
communicate drag and drop state information between each other, and
to update their respective visuals accordingly. The toolkit provides
drop site information as the pointer passes over any given drop site.
In this mode, a receiver can supply a procedure to generate its
own drag-under effects.
### Classes


Display inherits behavior and resources from`Core`,`Composite`,`Shell`,`WMShell`,`VendorShell`,`TopLevelShell`, and`ApplicationShell`classes.

The class pointer is`xmDisplayClass`.

The class name is`XmDisplay`.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. The programmer can also set the resource
values for the inherited classes to set attributes for this widget.
To reference a resource by name or by class in a.Xdefaultsfile,
remove the`XmN`or`XmC`prefix and use the remaining letters.
To specify one of the defined values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use the remaining letters (in
either lowercase or uppercase, but include any underscores between
words). The codes in the access column indicate if the given resource
can be set at creation time (C), set by using XtSetValues
(S), retrieved by using XtGetValues (G), or is not
applicable (N/A).

`XmDisplay Resource Set``Name``Class``Type``Default``Access`XmNdefaultButtonEmphasisXmCDefaultButtonEmphasisXtEnumXmEXTERNAL_HIGHLIGHTCXmNdefaultVirtualBindingsXmCDefaultVirtualBindingsStringdynamicCXmNdragInitiatorProtocolStyleXmCDragInitiatorProtocolStyleunsigned charXmDRAG_PREFER_RECEIVERCGXmNdragReceiverProtocolStyleXmCDragReceiverProtocolStyleunsigned charXmDRAG_PREFER_DYNAMICCGXmNdragStartCallbackXmCCallbackXtCallbackListNULLCXmNenableBtn1TransferXmCEnableBtn1TransferXtEnumXmOFFCXmNenableButtonTabXmCEnableButtonTabBooleanFalseCXmNenableDragIconXmCEnableDragIconBooleanFalseCXmNenableEtchedInMenuXmCEnableEtchedInMenuBooleanFalseCXmNenableToggleColorXmCEnableToggleColorBooleanFalseCXmNenableToggleVisualXmCEnableToggleVisualBooleanFalseCXmNenableUnselectableDragXmCEnableUnselectableDragBooleanTrueCXmNenableWarpXmCEnableWarpXtEnumTrueCSGXmNmotifVersionXmCMotifVersionintXmVERSIONCSGXmNnoFontCallbackXmCCallbackXtCallbackListNULLCXmNnoRenditionCallbackXmCCallbackXtCallbackListNULLCXmNuserDataXmCUserDataXtPointerNULLCSG

* **`XmNdefaultButtonEmphasis`** 

Specifies whether to change the look of the PushButton widget and
gadget that have the`XmNshowAsDefault`resource set. When the
PushButton is the default, it has an etched out button which is
enclosed with another etched in border. The`XmNdefaultButtonEmphasis`has the follow possible values, which
affect the location cursor:

* **`XmINTERNAL_HIGHLIGHT`** 

Causes the location cursor to appear in between the two etched borders
to minimize the space required.
* **`XmEXTERNAL_HIGHLIGHT`** 

Causes the PushButton to draw the location cursor outside the second border.

* **`XmNdefaultVirtualBindings`** 

Specifies the default virtual bindings for the display.
Following is an example of a specification for the`defaultVirtualBindings`resource in a resource file:*defaultVirtualBindings: &bsol;
        osfBackSpace:       &lt;Key>BackSpace       &bsol;n&bsol;
        osfInsert:       &lt;Key>InsertChar      &bsol;n&bsol;
        osfDelete:       &lt;Key>DeleteChar      &bsol;n&bsol;
        ...
        osfLeft:       &lt;Key>left, Ctrl&lt;Key>H
* **`XmNdragInitiatorProtocolStyle`** 

Specifies the drag and drop protocol requirements or preference when
the client is an initiator. The possible values are

* **`XmDRAG_PREREGISTER`** 

As an initiator, this client does not use the dynamic protocol and
can only arrange visual effects with receivers who provide
preregistered information.
* **`XmDRAG_DYNAMIC`** 

As an initiator, this client does not make use of any preregistered
drop site information made available by other clients, and can only
arrange visual effects with receivers who use the dynamic protocol.
* **`XmDRAG_NONE`** 

Specifies that drag and drop is disabled for this client.
* **`XmDRAG_DROP_ONLY`** 

As an initiator, this client does not use either the preregistered
drop site information or the dynamic protocol. It supports dragging,
and any time the cursor is over a client that supports drag and
drop, valid feedback is provided. There are no other visual effects.
* **`XmDRAG_PREFER_DYNAMIC`** 

As an initiator, this client can support both the preregister and
dynamic protocols, but prefers to use dynamic protocols whenever
possible in order to provide high-quality drag-under feedback.
* **`XmDRAG_PREFER_PREREGISTER`** 

As an initiator, this client can support both the preregister and
dynamic protocols, but prefers to use the preregister protocol
whenever possible in order to accommodate performance needs or to
provide consistent drag-over feedback.
* **`XmDRAG_PREFER_RECEIVER`** 

Indicates that this client can support both preregister and dynamic
protocols, but will defer to the preference of the receiver client.
This value is valid only for the`XmNdragInitiatorProtocolStyle`resource, and is its default value.

* **`XmNdragReceiverProtocolStyle`** 

Specifies the drag and drop protocol requirements or preference
when this client is a receiver. The values are

* **`XmDRAG_PREREGISTER`** 

As a receiver, this client preregisters drop site information and
does not use the dynamic protocol. It can only arrange visual
effects with initiators who make use of the preregistered information.
* **`XmDRAG_DYNAMIC`** 

As a receiver, this client uses the dynamic protocol and does
not preregister drop site information. It can only arrange visual
effects with initiators who use the dynamic protocol.
* **`XmDRAG_NONE`** 

Specifies that drag and drop is disabled for this client.
* **`XmDRAG_DROP_ONLY`** 

As a receiver, this client neither uses the dynamic protocol
nor preregisters drop site information. It supports
dropping, and when dragging over this client, valid feedback
is always provided, but there are no other visual effects.
* **`XmDRAG_PREFER_DYNAMIC`** 

As a receiver, this client can support both the preregister and
dynamic protocols, but prefers to use the dynamic protocol whenever
possible in order to provide high-quality drag-under feedback.
* **`XmDRAG_PREFER_PREREGISTER`** 

As a receiver, this client can support both the preregister and
dynamic protocols, but prefers to use the preregister protocol
whenever possible in order to accommodate performance
needs.


The default value of this resource is dependent on the capabilities of
the display. If the display supports the shape extension, allowing the
dynamic protocol to use arbitrarily sized drag cursors, the default of
this resource is`XmDRAG_PREFER_DYNAMIC`, otherwise the default is`XmDRAG_PREFER_PREREGISTER`.

The actual protocol used between an initiator and a receiver
is based on the protocol style of the receiver and initiator. The
decision matrix is described in the following table.`Drag Initiator
Protocol Style``Drag Receiver Protocol Style``Preregister``Prefer Preregister``Prefer Dynamic``Dynamic``Preregister`PreregisterPreregisterPreregisterDrop Only`Prefer Preregister`PreregisterPreregisterPreregisterDynamic`Prefer Receiver`PreregisterPreregisterDynamicDynamic`Prefer Dynamic`PreregisterDynamicDynamicDynamic`Dynamic`Drop OnlyDynamicDynamicDynamic

The value`XmDRAG_NONE`does not appear in the
matrix. When specified for either the initiator or receiver side,`XmDRAG_NONE`implies that drag and drop transactions are not
supported. A value of`XmDRAG_DROP_ONLY`(Drop Only) results
when an initiator and receiver cannot compromise protocol styles,
that is, one client requires dynamic mode while the other
can only support preregister mode, or if either explicitly has
specified`XmDRAG_DROP_ONLY`.
* **`XmNdragStartCallback`** 

Specifies the list of callbacks that are invoked when the`XmDragStart`function is called. The type of structure whose
address is passed to this callback isXmDragStartCallbackStruct.
The callback reason
is`XmCR_DRAG_START`.
* **`XmNenableBtn1Transfer`** 

Specifies if selection and transfer actions are integrated on Btn1
and extend actions are activated on Btn2. This resource
can take the following values:

* **`XmOFF`** 

Disables integration and selection activation on Btn1.
* **`XmBUTTON2_TRANSFER`** 

Enables integration and selection activation on Btn1 and transfer on Btn2.
* **`XmBUTTON2_ADJUST`** 

Enables integration and selection activation on Btn1 and adjust on Btn2.


This
resource affects the actions of Text, TextField, List, and Container.
* **`XmNenableButtonTab`** 

Specifies if the action for theTabkey (`KNextField`and`KPrevField`actions) is to be modified.
A value of True modifies the key to move as an arrow key until the
boundary of a tab group is reached. Then, at the boundary of the
tab group,`KNextField`and`KPrevField`will move to the next
or previous tab group, respectively. A value of False does not cause
modification.
* **`XmNenableDragIcon`** 

Specifies which set of icons are to be used for system default cursors
during drag and drop operations. A value of False specifies that
earlier versions of Motif release icons are used, a value of True
specifies that
alternate icons are used. This resource affects both the 16x16 and
the 32x32 icons that the system defaults for each of the Screen
objects associated with this display.
* **`XmNenableEtchedInMenu`** 

Specifies the shadowing of the button widgets and gadgets in menus
when the control is activated. A value of True causes the selected
menu to be drawn with the shadow etched in; this shadow style is
consistent with the selected appearance of other button widgets
outside of menus. A value of False causes
the selected menu to be draw with the shadow etched out. This
resource affects the actions of PushButton, ToggleButton, and
CascadeButton widgets and gadgets when they are children of Menu.When this resource is set, the background of a button in a menu
uses theXmNselectColor(derived from theXmNselectPixel) when armed
as a default. APushButtonuses theXmNarmColorif it is defined. AToggleButtonuses theXmNselectColorifXmNindicatorOnisFalseandXmNfillOnSelectisTrue.
* **`XmNenableToggleColor`** 

Specifies how to determine the default value of the`XmNselectColor`resource of ToggleButton and ToggleButtonGadget.
A value of True causes the default value of`XmNselectColor`to be set to the value of`XmNhighlightColor`.
A value of False causes the default value of`XmNselectColor`to be set to the value of`XmNbackground`.
This resource only affects the appearance of ToggleButton
widgets and gadgets that are in`XmONE_OF_MANY`or`XmONE_OF_MANY_ROUND`mode. In addition,`XmNenableToggleColor`only influences the default value of`XmNselectColor`. That is,
if the user or application sets a value for`XmNselectColor`,
then`XmNenableToggleColor`is ignored.
* **`XmNenableToggleVisual`** 

Specifies the visual appearance of the ToggleButton widget and/or
gadget. This resource affects the default value of the ToggleButton[Gadget]`XmNindicatorType`and`XmNindicatorOn`resources. When the
ToggleButton is in a RadioBox, a value of True causes the`XmONE_OF_MANY_ROUND`(a shadowed circle) to be the default.
Otherwise, when this resource is True, the ToggleButton`XmNindicatorOn`resource causes a default of`XmN_OF_MANY`,
which will be a shadowed square with a check mark (check box).

A value of False causes the following:

* **`XmONE_OF_MANY`** 

Is a shadowed diamond.
* **`XmN_OF_MANY`** 

Is a shadowed square.

* **`XmNenableUnselectableDrag`** 

Specifies whether or not it is possible to drag from Label and Scale.
A value of True enables the drag; a value of False disables it.
* **`XmNenableWarp`** 

Specifies if an application is allowed to warp the pointer
from the user. A value of True enables warping, a value of False does not.
* **`XmNmotifVersion`** 

Specifies the current version of Motif that the current implementation
is supposed to
behave like. By default, this resource gets its value from release
values inXm.h.
* **`XmNnoFontCallback`** 

This callback is called whenever a rendition attempts to load a font
or fontset and fails. This can happen on creation if the font is
specified as`XmLOAD_IMMEDIATE`or when an attempt is made to
render anXmStringusing a font specified as`XmLOAD_DEFERRED`. An application can have this callback attempt
to remedy this problem by calling`XmRenditionUpdate`on the input
rendition to provide a font for the widget to use. This may be done by
either providing an an alternative font name to be loaded using the`XmNfontName`and`XmNfontType`resources or with an already
loaded font using the`XmNfont`resource. The callback reason is`XmCR_NO_FONT`. This callback uses theXmDisplayCallbackStructstructure.
* **`XmNnoRenditionCallback`** 

This callback is called whenever an attempt is made to render a
segment with a`RENDITION`tag which does not match any renditions
in a given render table. The callback reason is`XmCR_NO_RENDITION`. This callback uses theXmDisplayCallbackStructstructure.

An application can have this callback attempt to remedy this problem
by creating a new
rendition with the given tag and adding it to`render_table`.

The`XmNnoRenditionCallback`should deallocate the render table passed in
in the`render_table`field of the callback structure. Note that
the table
will automatically be deallocated if the`XmRenderTableAddRenditions`function is
called on it. The callback should NOT deallocate the modified render
table that is passed back to Motif in the`render_table`field. If the
application wishes to manipulate this render table further, it should
make a copy with the`XmRenderTableCopy`function before returning
from the callback.
* **`XmNuserData`** 

Specifies a client data pointer for applications. An internally
unused resource.

### Inherited Resources


All of the superclass resources inherited by XmDisplay are
designated N/A (not applicable).
### Callback Information


A pointer to the following structure is passed to the`XmNdragStartCallback`callback:typedef struct
{
        int`reason`;
        XEvent  *`event`;
        Widget`timeStamp`;
        Boolean`doit`;
}XmDragStartCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`widget`** 

Indicates the ID of the widget from which the drag was initiated.
* **`doit`** 

Is an IN/OUT member that allows the callback to determine whether to
continue with the drag or cancel. Setting`doit`to
False will cancel the drag. The default value is NULL.


A pointer to the following structure is passed to the`XmNnoFontCallback`and`XmNnoRenditionCallback`callbacks:typedef struct
{
        int`reason`;
        XEvent *`event`;
        XmRendition`rendition`;
        char *`font_name`;
        XmRenderTable`render_table`;
        XmStringTagtag;
}XmDisplayCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback. It can be NULL.
* **`rendition`** 

Specifies the rendition with the missing font.
* **`font_name`** 

Specifies the name of the font or font set which could not be loaded.
* **`render_table`** 

Specifies the render table with the missing rendition.
* **tag** 

Specifies the tag of the missing rendition.


The following table describes the reasons for which the individual
callback structure fields are valid.`Reason``Valid Fields`XmCR_NO_FONTrendition, font_nameXmCR_NO_RENDITIONrender_table, tag
## RELATED


&cdeman.ApplicationShell;,
&cdeman.Composite;,
&cdeman.Core;,
&cdeman.TopLevelShell;,
&cdeman.VendorShell;,
&cdeman.WMShell;,
&cdeman.XmGetXmDisplay;, and
&cdeman.XmScreen;.