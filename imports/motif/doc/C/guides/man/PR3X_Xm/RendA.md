# XmRendition
library callXmRenditionThe Rendition registryXmRendition&npzwc;#include &lt;Xm/Xm.h>
XmRendition
## DESCRIPTION


XmRenditionis a pseudo widget used for the rendering ofXmStrings.XmRenditionhas two parts:XmStringTagand rendering information. TheXmStringTagpart
can be matched with anXmStringTagassociated with a`LOCALE`,`CHARSET`, or`RENDITION[BEGIN|END]`component
withinXmString.
The rendering information contains
information about the font or fontset, colors, tabs, and lines to be
used in rendering a text component.

If a resource in a rendition is unspecified, usually by setting it to`XmAS_IS`or`XmUNSPECIFIED_PIXEL`,
then the value to be used for that resource is the value of the
immediately preceding rendition inXmString. If that value
is unspecified, then the preceding value is used, and so on. If
no renditions specify a value for a resource, then a default value
will be used.
### Classes


XmRenditiondoes not inherit from any widget class.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. To reference a resource by name
or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix
and use the remaining letters (in either lowercase or uppercase, but include
any underscores between words). The codes in the access column
indicate if the given resource can be set at creation time (C),
set by using`XmRenditionUpdate`(S), retrieved by using`XmRenditionRetrieve`(G), or is not applicable (N/A).

`XmRendition Resource Set``Name``Class``Type``Default``Access`XmNrenditionBackgroundXmCRenditionBackgroundPixelXmUNSPECIFIED_PIXELCSGXmNfontXmCFontXtPointerXmAS_ISCSGXmNfontNameXmCFontNameStringXmAS_ISCSGXmNfontTypeXmCFontTypeXmFontTypeXmAS_ISCSGXmNrenditionForegroundXmCRenditionForegroundPixelXmUNSPECIFIED_PIXELCSGXmNloadModelXmCLoadModelunsigned charXmAS_ISCSGXmNstrikethruTypeXmCStrikethruTypeunsigned charXmAS_ISCSGXmNtabListXmCTabListXmTabListXmAS_ISCSGXmNtagXmCTagXmStringTag""GXmNunderlineTypeXmCUnderlineTypeunsigned charXmAS_ISCSG

* **`XmNrenditionBackground`** 

Specifies the background drawing color. A value of`XmUNSPECIFIED_PIXEL`indicates that the background is not
specified for this rendition.
* **`XmNfont`** 

Specifies the actual font or fontset to be used by this rendition. The
value of this resource, if set to other than`XmAS_IS`, will be
used regardless of the settings of the other font resources. Setting
this resource will force`XmNloadModel`to be`XmLOAD_IMMEDIATE`. If this resource is not initially set, then
it will be set subsequently by the rendition whenever the font or
fontset specified by`XmNfontName`is loaded. If both`XmNfontName`and`XmNfont`are specified in a resource file,
the`XmNfont`specification will take precedence.
* **`XmNfontName`** 

Specifies an X Logical Font Description (XLFD) string, which is
interpreted either as a font name or as a base font name list. A base
font name list is a comma-separated and NULL-terminated string. A
value of`XmAS_IS`indicates that the font is not specified for
this rendition. If both`XmNfontName`and`XmNfont`are
specified in a resource file, the`XmNfont`specification will
take precedence.
* **`XmNfontType`** 

Specifies whether the`XmNfontName`resource refers to a font name or to a base font name
list. Valid values are`XmFONT_IS_FONT`and`XmFONT_IS_FONTSET`.
* **`XmNrenditionForeground`** 

Specifies the foreground drawing color. A value of`XmUNSPECIFIED_PIXEL`indicates that the foreground is not
specified for this rendition.
* **`XmNloadModel`** 

Specifies whether the font or fontset specified by`XmNfontName`is to be loaded when the rendition is created (`XmLOAD_IMMEDIATE`)
or only when the font is required to render anXmStringsegment
(`XmLOAD_DEFERRED`).
Note that specifying`XmLOAD_IMMEDIATE`for`XmNloadModel`is
valid only if`XmNfontName`is specified, in which case the
specified font will be loaded on creation, or if`XmNfont`is
specified, in which case the font is already loaded.`XmLOAD_DEFERRED`is only valid when`XmNfontName`is
specified.
* **`XmNstrikethruType`** 

Specifies the type of line to be used to strike through a text
segment. Valid values are`XmNO_LINE`,`XmSINGLE_LINE`,`XmDOUBLE_LINE`,`XmSINGLE_DASHED_LINE`, and`XmDOUBLE_DASHED_LINE`. A value of`XmAS_IS`indicates that
the strike-through type is not specified for this rendition.
* **`XmNtabList`** 

Specifies the tab list to be used in rendering compound strings
containing tab components.
* **`XmNtag`** 

Specifies the tag string to be used in matching the renditions with
other renditions or withXmStringTagcomponents in`XmStrings`. This resource must always be specified. That is,
NULL is not a legal value but the empty string is.
This resource is automatically set to the value of thetagparameter in
the`XmRenditionCreate`call.
* **`XmNunderlineType`** 

Specifies the type of line to be used to underline a text segment.
Valid values are`XmNO_LINE`,`XmSINGLE_LINE`,`XmDOUBLE_LINE`,`XmSINGLE_DASHED_LINE`, and`XmDOUBLE_DASHED_LINE`. A value of`XmAS_IS`indicates that
the underline type is not specified for this rendition.

## RELATED


&cdeman.XmRenditionCreate;,
&cdeman.XmRenditionFree;,
&cdeman.XmRenditionRetrieve;,
&cdeman.XmRenditionUpdate;, and
&cdeman.XmString;.