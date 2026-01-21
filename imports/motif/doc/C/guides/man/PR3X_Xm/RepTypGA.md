# XmRepTypeGetId
library call`XmRepTypeGetId`A representation type manager function that
retrieves the identification number of a representation typeXmRepTypeGetIdrepresentation type manager functionsXmRepTypeGetId#include <Xm/RepType.h>XmRepTypeId`XmRepTypeGetId`Stringrep_type
## DESCRIPTION


`XmRepTypeGetId`searches the registration list
for the specified representation type and returns the
associated identification number.

* **`rep_type`** 

Specifies the representation type for which an identification
number is requested

## RETURN


Returns the identification number of the specified
representation type. If the representation type is not
registered, the function returns`XmREP_TYPE_INVALID`.
## RELATED


&cdeman.XmRepTypeGetRegistered; and
&cdeman.XmRepTypeRegister;.