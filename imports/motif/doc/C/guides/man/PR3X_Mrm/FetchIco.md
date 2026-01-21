# MrmFetchIconLiteral
library call`MrmFetchIconLiteral`Fetches an icon literal from a hierarchyMrmFetchIconLiteraluil functionsMrmFetchIconLiteral#include <Mrm/MrmPublic.h>Cardinal`MrmFetchIconLiteral`MrmHierarchyhierarchy_idStringindexScreen *screenDisplay *displayPixelfgpixPixelbgpixPixmap *pixmapMRM functionMrmFetchIconLiteralMrmFetchIconLiteraldefinition
## DESCRIPTION


The`MrmFetchIconLiteral`functionMrmFetchIconLiteraldescriptionfetches an icon literal from an MRM hierarchy and converts the
icon literal to an X pixmap.

* **`hierarchy_id`** 

Specifies the ID of the UID hierarchy that contains the
specified icon literal.
The`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.
* **`index`** 

Specifies the UIL name of the icon literal to fetch.
* **`screen`** 

Specifies the screen used for the pixmap.
The`screen`argument specifies a pointer to the
Xlib structureScreen,
which contains the information about that screen and is linked to theDisplaystructure.
For more information on theDisplayandScreenstructures, see the Xlib function`XOpenDisplay`and the associated screen information macros.
* **`display`** 

Specifies the display used for the pixmap.
The`display`argument specifies the connection to the X server.
For more information on theDisplaystructure, see the Xlib function`XOpenDisplay`.
* **`fgpix`** 

Specifies the foreground color for the pixmap.
* **`bgpix`** 

Specifies the background color for the pixmap.
* **`pixmap`** 

Returns the resulting X pixmap value.
The function allocates space for this pixmap.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmDestroyPixmap`.

## RETURN
MrmSUCCESSMrmBAD_HIERARCHYMrmNOT_FOUNDMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmNOT_FOUND`** 

The icon literal was not found in the hierarchy.
* **`MrmWRONG_TYPE`** 

The caller tried to fetch a literal of a type not supported by this
function.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmFetchBitmapLiteral;,
&cdeman.MrmOpenHierarchyPerDisplay;,
&cdeman.MrmFetchLiteral;,
&cdeman.MrmFetchColorLiteral;, andXOpenDisplay(3).