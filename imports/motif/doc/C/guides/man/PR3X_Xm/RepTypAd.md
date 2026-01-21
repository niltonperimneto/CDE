# XmRepTypeAddReverse
library call`XmRepTypeAddReverse`A representation type manager function
that installs the reverse converter for a previously registered representation typeXmRepTypeAddReverserepresentation type manager functionsXmRepTypeAddReverse#include <Xm/RepType.h>void`XmRepTypeAddReverse`XmRepTypeIdrep_type_id
## DESCRIPTION


`XmRepTypeAddReverse`installs the reverse converter
for a previously registered representation type. The reverse
converter takes a numerical representation type value and returns
its corresponding string value. Certain applications may require
this capability to obtain a string value to display on a screen
or to build a resource file.

The`values`argument of the`XmRepTypeRegister`function
can be used to register representation types with nonconsecutive
values or with duplicate names for the same value. If the list
of numerical values for a representation type contains duplicate values,
the reverse converter uses the first name in the`value_names`list that matches the specified numeric value. For example, if
a`value_names`array has`cancel`,`proceed`, and`abort`, and the corresponding`values`array contains 0, 1, and 0, the reverse converter
will return`cancel`instead of`abort`for an
input value of 0.

* **`rep_type_id`** 

Specifies the identification number of the representation type

## RELATED


&cdeman.XmRepTypeGetId; and
&cdeman.XmRepTypeRegister;.