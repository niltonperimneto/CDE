# XmStringComponentCreate
library call`XmStringComponentCreate`A compound string function that creates arbitrary componentsXmStringComponentCreate#include <Xm/Xm.h>XmString`XmStringComponentCreate`XmStringComponentTypec_typeunsigned intlengthXtPointervalue
## DESCRIPTION


`XmStringComponentCreate`creates a newXmStringcomponent
of type`c_type`, containing`value`. If`value`is invalid
for the particular component type, this function fails and
returns NULL.

* **`c_type`** 

Specifies the type of component to be created.
* **`length`** 

Specifies the length in bytes of`value`.
Note that this must be precisely the length of the`value`string,notincluding any trailing null characters.
* **`value`** 

Specifies the value to be used in the creation of the component.


Refer to the &cdeman.XmStringComponentType; reference page for a list of
the possibleXmStringcomponent types.
## RETURN


If`value`is invalid for`c_type`, fails
and returns NULL. Otherwise, this function returns a new compound string.
When the application no longer needs the returned compound string,
the application should call`XmStringFree`.
## RELATED


&cdeman.XmString;,`XmStringGetNextTriple`,XmStringComponentType, and
&cdeman.XmStringFree;.