# XmResolvePartOffsets
library call`XmResolvePartOffsets`A function that allows writing of upward-compatible applications and widgetsXmResolvePartOffsets#include <Xm/Xm.h>void`XmResolvePartOffsets`WidgetClasswidget_classXmOffsetPtr* offset
## DESCRIPTION


The use of offset records requires one extra global variable per widget
class.
The variable consists of a pointer to an array of offsets into the
widget record for each part of the widget structure.
The`XmResolvePartOffsets`function
allocates the offset records needed by an application to guarantee
upward-compatible access to widget instance records by
applications and widgets.
These offset records are used by the widget to access all of the
widget's variables.
A widget needs to take the steps described in the following paragraphs.

Instead of creating a resource list, the widget creates an offset
resource list.
To accomplish this, use theXmPartResourcestructure and the`XmPartOffset`macro.
TheXmPartResourcedata structure looks just like a
resource list, but instead of having
one integer for its offset, it has two shorts.
This structure is put into the class record as if it were a normal resource
list. Instead of using`XtOffset`for the offset, the widget uses`XmPartOffset`.XmPartResource resources[] = {
  { BarNxyz, BarCXyz, XmRBoolean,
    sizeof(Boolean), XmPartOffset(Bar,xyz),
    XmRImmediate, (XtPointer)False }
};

Instead of putting the widget size in the class record, the widget puts the
widget part size in the same field.

Instead of putting`XtVersion`in the class record, the widget puts`XtVersionDontCheck`in the class record.

The widget defines a variable, of typeXmOffsetPtr, to point to
the offset record.
This can be part of the widget's class record or a separate global
variable.

In class initialization, the widget calls`XmResolvePartOffsets`,
passing it a pointer to contain the address of the offset
record and the class record.
This does several things:

Adds the superclass (which, by definition, has already been initialized)
size field to the part size field

Allocates an array based upon the number of superclasses

Fills in the offsets of all the widget parts with the appropriate
values, determined by examining the size fields of all superclass
records

Uses the part offset array to modify the offset entries in the resource
list to be real offsets, in place

The widget defines a constant that will be the index to its part
structure in the offsets array.
The value should be 1 greater than
the index of the widget's superclass.
Constants defined for all`Xm`widgets can be found inXmP.h.&npzwc;#define BarIndex (XmBulletinBIndex + 1)

Instead of accessing fields directly, the widget must always go through
the offset table.
The`XmField`macro helps you access these fields.
Because the`XmPartOffset`and`XmField`macros concatenate things together, you must
ensure that there is no space
after the part argument.
For example, the following macros do not work because of the space
after the part (Label) argument:XmField(w, offset, Label, text, char *)
XmPartOffset(Label, text)

Therefore, you must not have any spaces after the part (Label)
argument, as illustrated here:XmField(w, offset, Label, text, char *)

You can define macros for each field to make this easier.
Assume an integer field`xyz`:&npzwc;#define BarXyz(w) (*(int *)(((char *) w) + &bsol;
        offset[BarIndex] + XtOffset(BarPart,xyz)))

The parameters for`XmResolvePartOffsets`are

* **`widget_class`** 

Specifies the widget class pointer for the created widget
* **`offset`** 

Returns the offset record

## RELATED INFORMATION


&cdeman.XmResolveAllPartOffsets;.