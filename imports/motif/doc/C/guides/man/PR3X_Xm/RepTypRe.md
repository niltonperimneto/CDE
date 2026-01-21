# XmRepTypeRegister
library call`XmRepTypeRegister`A representation type manager function
that registers a representation type resourceXmRepTypeRegisterrepresentation type manager functionsXmRepTypeRegister#include <Xm/RepType.h>XmRepTypeId`XmRepTypeRegister`Stringrep_typeString *value_namesunsigned char *valuesunsigned charnum_values
## DESCRIPTION


`XmRepTypeRegister`registers a representation
type resource with the representation type manager. All
features of the representation type management facility become
available for the specified representation type. The function
installs a forward type converter to convert string values to
numerical representation type values.

When the`values`argument is NULL, consecutive numerical
values are assumed. The order of the strings in the`value_names`array determines the numerical values for the resource. For example,
the first value name is 0 (zero); the second value name is 1; and so on.

If it is non-NULL, the`values`argument can be used to assign values
to representation types that have nonconsecutive values or have
duplicate names for the same value. Representation types registered
in this manner will consume additional storage and will be slightly
slower than representation types with consecutive values.

A representation type can only be registered once; if the
same representation type name is registered more than once, the
behavior is undefined.

The function`XmRepTypeAddReverse`installs a reverse converter
for a registered representation type. The reverse converter takes
a representation type numerical value and returns the corresponding
string value. If the list of numerical values for a representation
type contains duplicate values, the reverse converter uses the first
name in the`value_names`list that matches the specified numeric
value. For example, if a`value_names`array has`cancel`,`proceed`, and`abort`, and the corresponding`values`array
contains 0, 1, and 0, the reverse converter will return`cancel`instead of`abort`for an input value of 0.

* **`rep_type`** 

Specifies the representation type name.
* **`value_names`** 

Specifies a pointer to an array of value names associated with the
representation type. A value name is specified in lowercase characters
without an`Xm`prefix. Words within a name are separated with
underscores.
* **`values`** 

Specifies a pointer to an array of values associated with the
representation type. A value in this array is associated with
the value name in the corresponding position of the`value_names`array.
* **`num_values`** 

Specifies the number of entries in the`value_names`and`values`arrays.

## RETURN


Returns the identification number for the specified representation
type.
## RELATED


&cdeman.XmRepTypeAddReverse;,
&cdeman.XmRepTypeGetId;,
&cdeman.XmRepTypeGetNameList;,
&cdeman.XmRepTypeGetRecord;,
&cdeman.XmRepTypeGetRegistered;, and
&cdeman.XmRepTypeValidValue;.