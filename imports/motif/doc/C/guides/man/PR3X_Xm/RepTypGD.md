# XmRepTypeGetRegistered
library call`XmRepTypeGetRegistered`A representation type manager function that
returns a copy of the registration listXmRepTypeGetRegisteredrepresentation type manager functionsXmRepTypeGetRegistered#include <Xm/RepType.h>XmRepTypeList`XmRepTypeGetRegistered`
## DESCRIPTION


`XmRepTypeGetRegistered`retrieves information about
all representation types that are registered with the
representation type manager. The registration list is
an array of structures, each of which contains information
for a representation type entry. The end of the registration
list is marked with a representation type entry whose`rep_type_name`field has a NULL pointer. This routine
allocates memory for the returned data. The application must free
this memory using`XtFree`.

The representation type entry structure contains the following
information:typedef struct
{
        Stringrep_type_name;
        String  *value_names;
        unsigned char   *values;
        unsigned charnum_values;
        Booleanreverse_installed;
        XmRepTypeIdrep_type_id;
} XmRepTypeEntryRec, *XmRepTypeList;

* **`rep_type_name`** 

The name of the representation type
* **`value_names`** 

An array of representation type value names
* **`values`** 

An array of representation type numerical values
* **`num_values`** 

The number of values associated with the representation type
* **`reverse_installed`** 

A flag that indicates whether or not the reverse converter is
installed
* **`rep_type_id`** 

The identification number of the representation type

## RETURN


Returns a pointer to the registration list of representation types.
## RELATED


&cdeman.XmRepTypeRegister; and
&cdeman.XmRepTypeGetRecord;.