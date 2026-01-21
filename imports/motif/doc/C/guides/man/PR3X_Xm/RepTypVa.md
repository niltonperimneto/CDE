# XmRepTypeValidValue
library call`XmRepTypeValidValue`A representation type manager function that
tests the validity of a numerical value of a  representation type resourceXmRepTypeValidValuerepresentation type manager functionsXmRepTypeValidValue#include <Xm/RepType.h>Boolean`XmRepTypeValidValue`XmRepTypeIdrep_type_idunsigned chartest_valueWidgetenable_default_warning
## DESCRIPTION


`XmRepTypeValidValue`tests the validity of a numerical
value for a given representation type resource. The function
generates a default warning message if the value is invalid
and the`enable_default_warning`argument is non-NULL.

* **`rep_type_id`** 

Specifies the identification number of the representation
type.
* **`test_value`** 

Specifies the numerical value to test.
* **`enable_default_warning`** 

Specifies the ID of the widget that contains a default warning
message. If this parameter is NULL,
no default warning message is generated and
the application must provide its own error handling.

## RETURN


Returns True if the specified value is valid; otherwise, returns False.
## RELATED


&cdeman.XmRepTypeGetId; and
&cdeman.XmRepTypeRegister;.