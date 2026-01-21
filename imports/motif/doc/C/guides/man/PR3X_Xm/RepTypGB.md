# XmRepTypeGetNameList
library call`XmRepTypeGetNameList`A representation type manager function that
generates a list of values for a representation typeXmRepTypeGetNameListrepresentation type manager functionsXmRepTypeGetNameList#include <Xm/RepType.h>String *`XmRepTypeGetNameList`XmRepTypeIdrep_type_idBooleanuse_uppercase_format
## DESCRIPTION


`XmRepTypeGetNameList`generates a NULL-terminated list of
the value names associated with the specified representation type.
Each value name is a NULL-terminated string. This routine allocates
memory for the returned data. The application must free
this memory using`XtFree`.

* **`rep_type_id`** 

Specifies the identification number of the representation type.
* **`use_uppercase_format`** 

Specifies a Boolean value that controls the format of the name
list. If the value is True, each value name is in uppercase characters prefixed
by`Xm`; if it is False, the names are in lowercase characters.

## RETURN


Returns a pointer to an array of the value names.
## RELATED


&cdeman.XmRepTypeGetId;,
&cdeman.XmRepTypeGetRegistered;, and
&cdeman.XmRepTypeRegister;.