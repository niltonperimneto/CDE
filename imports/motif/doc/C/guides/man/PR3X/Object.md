# Object
library call`Object`The Object widget classObjectwidget classObject&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Object is never instantiated.
Its sole purpose is as a supporting superclass for other
widget classes.
### Classes


The class pointer is`objectClass`.

The class name is`Object`.
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

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC

* **`XmNdestroyCallback`** 

Specifies a list of callbacks that is called when the gadget is destroyed.

### Translations


There are no translation for Object.