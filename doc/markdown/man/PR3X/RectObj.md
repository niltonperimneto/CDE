# RectObj
library call`RectObj`The RectObj widget classRectObjwidget classRectObj&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


RectObj is never instantiated.
Its sole purpose is as a supporting superclass for other
widget classes.
### Classes


RectObj inherits behavior and a resource from`Object`.

The class pointer is`rectObjClass`.

The class name is`RectObj`.
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

`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension1CSGXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

* **`XmNancestorSensitive`** 

Specifies whether the immediate parent of the
gadget receives input events.
Use the function`XtSetSensitive`if you are changing the
argument to preserve data integrity (see`XmNsensitive`).
The default is the bitwise AND of the parent's`XmNsensitive`and`XmNancestorSensitive`resources.
* **`XmNborderWidth`** 

Specifies the width of the border placed around the RectObj's rectangular
display area.
* **`XmNheight`** 

Specifies the inside height (excluding the border) of the RectObj's
rectangular display area.
* **`XmNsensitive`** 

Determines whether a RectObj receives input
events.
If a RectObj is sensitive, the parent dispatches to the
gadget all keyboard, mouse button, motion, window enter/leave,
and focus events.
Insensitive gadgets do not receive these events.
Use the function`XtSetSensitive`to change the sensitivity argument.
Using`XtSetSensitive`ensures that if a parent widget has`XmNsensitive`set to False, the ancestor-sensitive flag of all its children
is appropriately set.
* **`XmNwidth`** 

Specifies the inside width (excluding the border) of the RectObj's
rectangular display area.
* **`XmNx`** 

Specifies the x-coordinate of the upper left outside corner of the
RectObj's rectangular display area.
The value is relative to the upper left inside corner of the parent
window.
* **`XmNy`** 

Specifies the y-coordinate of the upper left outside corner of the
RectObj's rectangular display area.
The value is relative to the upper left inside corner of the parent
window.

### Inherited Resources


RectObj inherits behavior and a resource from`Object`. For
a description of this resource, refer to the`Object`reference page.

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Translations


There are no translations for RectObj.
## RELATED


&cdeman.Object;.