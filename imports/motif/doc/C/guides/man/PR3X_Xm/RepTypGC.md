# XmRepTypeGetRecord
library call`XmRepTypeGetRecord`A representation type manager function that
returns information about a representation typeXmRepTypeGetRecordrepresentation type manager functionsXmRepTypeGetRecord#include <Xm/RepType.h>XmRepTypeEntry`XmRepTypeGetRecord`XmRepTypeIdrep_type_id
## DESCRIPTION


`XmRepTypeGetRecord`retrieves information about
a particular representation type that is registered with
the representation type manager. This routine allocates
memory for the returned data. The application must free
this memory using`XtFree`.

* **`rep_type_id`** 

The identification number of the representation type


The representation type entry structure contains the following
information:typedef struct
{
        Stringrep_type_name;
        String  *value_names;
        unsigned char   *values;
        unsigned charnum_values;
        Booleanreverse_installed;
        XmRepTypeIdrep_type_id;
} XmRepTypeEntryRec, *XmRepTypeEntry;

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


Returns a pointer to the representation type entry structure that
describes the representation type.
## RELATED


&cdeman.XmRepTypeGetId;,
&cdeman.XmRepTypeGetRegistered;, and
&cdeman.XmRepTypeRegister;.