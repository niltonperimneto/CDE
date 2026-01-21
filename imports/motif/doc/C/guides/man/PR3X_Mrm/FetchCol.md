# MrmFetchColorLiteral
library call`MrmFetchColorLiteral`Fetches a named color literal from a UID fileMrmFetchColorLiteraluil functionsMrmFetchColorLiteraluid file#include <Mrm/MrmPublic.h>Cardinal`MrmFetchColorLiteral`MrmHierarchyhierarchy_idStringindexDisplay *displayColormapcolormap_idPixel *pixelMRM functionMrmFetchColorLiteralMrmFetchColorLiteraldefinition
## DESCRIPTION


The`MrmFetchColorLiteral`functionMrmFetchColorLiteraldescriptionfetches a named color literal from a UID file, and converts the
color literal to a pixel color value.

* **`hierarchy_id`** 

Specifies the ID of the UID hierarchy that contains the
specified literal.
The value of`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.
* **`index`** 

Specifies the UIL name of the color literal to fetch.
You must define this name in UIL as an exported value.
* **`display`** 

Specifies the display used for the pixmap.
The`display`argument specifies the connection to the X server.
For more information on theDisplaystructure, see the Xlib function`XOpenDisplay`.
* **`colormap_id`** 

Specifies the ID of the color map.
If`colormap_id`is NULL, the default color map is used.
* **`pixel`** 

Returns the ID of the color literal.

## RETURN
MrmSUCCESSMrmBAD_HIERARCHYMrmNOT_FOUNDMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmNOT_FOUND`** 

The color literal was not found in the UIL file.
* **`MrmWRONG_TYPE`** 

The caller tried to fetch a literal of a type not supported by this
function.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmFetchBitmapLiteral;,
&cdeman.MrmOpenHierarchyPerDisplay;,
&cdeman.MrmFetchIconLiteral;,
&cdeman.MrmFetchLiteral;, andXOpenDisplay(3).