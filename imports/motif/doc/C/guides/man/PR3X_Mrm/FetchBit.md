# MrmFetchBitmapLiteral
library call`MrmFetchBitmapLiteral`Fetches a bitmap literal from a hierarchyMrmFetchBitmapLiteraluil functionsMrmFetchBitmapLiteral#include <Mrm/MrmPublic.h>Cardinal`MrmFetchBitmapLiteral`MrmHierarchyhierarchy_idStringindexScreen *screenDisplay *displayPixmap *pixmap_returnDimension *widthDimension *heightMRM functionMrmFetchBitmapLiteralMrmFetchBitmapLiteraldefinition
## DESCRIPTION
MrmFetchBitmapLiteraldescription

The`MrmFetchBitmapLiteral`function fetches a bitmap literal
from an MRM hierarchy, and converts the bitmap literal to an X pixmap
of depth 1. The function returns this pixmap and its width and height.

* **`hierarchy_id`** 

Specifies the ID of the UID hierarchy that contains the
specified icon literal. The value of`hierarchy_id`was returned in
a previous call to`MrmOpenHierarchyPerDisplay`.
* **`index`** 

Specifies the UIL name of the bitmap literal to fetch.
* **`screen`** 

Specifies the screen used for the pixmap.
The`screen`argument specifies a pointer to the
Xlib structureScreenwhich contains the information about that
screen and is linked to theDisplaystructure.
For more information on theDisplayandScreenstructures,
see the Xlib function`XOpenDisplay`and the associated screen
information macros.
* **`display`** 

Specifies the display used for the pixmap.
The`display`argument specifies the connection to the X server.
For more information on theDisplaystructure, see the Xlib function`XOpenDisplay`.
* **`pixmap_return`** 

Returns the resulting X pixmap value.
The function allocates space for this pixmap.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmDestroyPixmap`.
* **`width`** 

Specifies a pointer to the width of the pixmap.
* **`height`** 

Specifies a pointer to the height of the pixmap.

## RETURN
MrmSUCCESSMrmBAD_HIERARCHYMrmNOT_FOUNDMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmNOT_FOUND`** 

The bitmap literal was not found in the hierarchy.
* **`MrmWRONG_TYPE`** 

The caller tried to fetch a literal of a type not supported by this
function.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmFetchIconLiteral;,
&cdeman.MrmFetchLiteral;, andXOpenDisplay(3).