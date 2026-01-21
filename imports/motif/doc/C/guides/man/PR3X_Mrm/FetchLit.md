# MrmFetchLiteral
library call`MrmFetchLiteral`Fetches a literal from a UID fileMrmFetchLiteraluil functionsMrmFetchLiteral#include <Mrm/MrmPublic.h>Cardinal`MrmFetchLiteral`MrmHierarchyhierarchy_idStringindexDisplay *displayXtPointer *valueMrmCode *typeMRM functionMrmFetchLiteralMrmFetchLiteraldefinition
## DESCRIPTION


The`MrmFetchLiteral`functionMrmFetchLiteraldescriptionreads and returns the value and type of a literal (named value)
that is stored as
a public resource in a single UID file.
This function returns a pointer to the value of the literal.
For example, an integer is always returned as a pointer to an integer,
and a string is always returned as a pointer to a string.

Applications should not use`MrmFetchLiteral`for fetching icon or color literals.
If this is attempted,`MrmFetchLiteral`returns an error.

* **`hierarchy_id`** 

Specifies the ID of the UID hierarchy that contains the
specified literal.
The value of`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.
* **`index`** 

Specifies the UIL name of the literal (pixmap) to fetch.
You must define this name in UIL as an exported value.
* **`display`** 

Specifies the display used for the pixmap.
The`display`argument specifies the connection to the X server.
For more information on theDisplaystructure, see the Xlib function`XOpenDisplay`.
* **`value`** 

Returns the ID of the named literal's value.
The function allocates space for the returned value.
The application is responsible for managing the allocated space
by calling the appropriate deallocation function. For example,
if the returned ID symbolizes a pixmap, then the application can
recover the allocated space by calling`XmDestroyPixmap`.
* **`type`** 

Returns the named literal's data type.
Types are defined in the include fileMrm/MrmPublic.h.

## RETURN
MrmSUCCESSMrmBAD_HIERARCHYMrmNOT_FOUNDMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmNOT_FOUND`** 

The literal was not found in the UIL file.
* **`MrmWRONG_TYPE`** 

The caller tried to fetch a literal of a type not supported by this
function.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmFetchBitmapLiteral;,
&cdeman.MrmOpenHierarchyPerDisplay;,
&cdeman.MrmFetchIconLiteral;,
&cdeman.MrmFetchColorLiteral;, andXOpenDisplay(3).