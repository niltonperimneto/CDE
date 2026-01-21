# XmDirection
library callXmDirectionData type for the direction of widget componentsXmDirectiondata typesXmDirection&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmDirectionis the data type specifying the direction in which
the system displays subwidgets, children of
widgets, or other visual components that are to be laid out.
This data type also affects traversal order within tab groups.

XmDirectionis implemented as an unsigned char bit mask.
The horizontal and vertical directions can be specified independent of
each other.XmDirectionalso specifies the precedence of the
horizontal and vertical directions relative to each other. For
example, a value of`XmRIGHT_TO_LEFT_TOP_TO_BOTTOM`lays out a
component horizontally from right to left first, then vertically
top to bottom.

XmDirectionprovides the following masks, each of which
corresponds to a particular bit inXmDirection:

`XmRIGHT_TO_LEFT_MASK`

`XmLEFT_TO_RIGHT_MASK`

`XmTOP_TO_BOTTOM_MASK`

`XmBOTTOM_TO_TOP_MASK`

`XmPRECEDENCE_HORIZ_MASK`

`XmPRECEDENCE_VERT_MASK`

In addition to the preceding single bit masks,XmDirectionalso provides the following multiple bit masks. These multiple
bit masks are particularly useful as arguments to`XmDirectionMatchPartial`:

`XmHORIZONTAL_MASK`

`XmPRECEDENCE_MASK`

`XmVERTICAL_MASK`

Motif also provides the following enumerated constants for
specifying various combinations of directions:

* **`XmRIGHT_TO_LEFT_TOP_TO_BOTTOM`** 

Specifies that the components are laid out from right to left first, then
top to bottom.
* **`XmLEFT_TO_RIGHT_TOP_TO_BOTTOM`** 

Specifies that the components are laid out from left to right first, then
top to bottom.
* **`XmRIGHT_TO_LEFT_BOTTOM_TO_TOP`** 

Specifies that the components are laid out from right to left first, then
bottom to top.
* **`XmLEFT_TO_RIGHT_BOTTOM_TO_TOP`** 

Specifies that the components are laid out from left to right first, then
bottom to top.
* **`XmTOP_TO_BOTTOM_RIGHT_TO_LEFT`** 

Specifies that the components are laid out from top to bottom first, then
right to left.
* **`XmTOP_TO_BOTTOM_LEFT_TO_RIGHT`** 

Specifies that the components are laid out from top to bottom first, then
left to right.
* **`XmBOTTOM_TO_TOP_RIGHT_TO_LEFT`** 

Specifies that the components are laid out from bottom to top first, then
right to left.
* **`XmBOTTOM_TO_TOP_LEFT_TO_RIGHT`** 

Specifies that the components are laid out from bottom to top first, then
left to right.
* **`XmTOP_TO_BOTTOM`** 

Specifies that the components are laid out from top to bottom.
If horizontal direction is important, do not use this constant.
* **`XmBOTTOM_TO_TOP`** 

Specifies that the components are laid out from bottom to top.
If horizontal direction is important, do not use this constant.
* **`XmDEFAULT_DIRECTION`** 

Specifies that the components are laid out according to the default
direction. (This constant is primarily for widget writers.)
* **`XmLEFT_TO_RIGHT`** 

Specifies that the components are laid out from left to right.
If vertical direction is important, do not use this constant.
* **`XmRIGHT_TO_LEFT`** 

Specifies that the components are laid out from right to left.
If vertical direction is important, do not use this constant.

## RELATED


&cdeman.XmDirectionMatch;, &cdeman.XmDirectionMatchPartial;,
&cdeman.XmDirectionToStringDirection;, &cdeman.XmString;,
&cdeman.XmStringDirection;, and
&cdeman.XmStringDirectionToDirection;.