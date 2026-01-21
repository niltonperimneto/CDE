# XmFontList
library callXmFontListData type for a font listXmFontListdata typesXmFontList&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmFontListis the data type for a font list. A font list
consists of font list entries. Each entry contains a font or a
font set (a group of fonts) and is identified with a tag,
which is optional. If this tag is
NULL, the tag
is set to`XmFONTLIST_DEFAULT_TAG`.

The value of`XmFONTLIST_DEFAULT_TAG`is`XmFONTLIST_DEFAULT_TAG_STRING`.

When a compound string is displayed, the font list element tag of the
compound string segment is matched with a font list entry tag in
the font list and the matching font list entry is used to display
the compound string. A font list entry is chosen as follows:

The first font list entry whose tag matches the tag of the compound
string segment is used.

If no match has been found and if the tag of the compound string
segment is`XmFONTLIST_DEFAULT_TAG`, the first font list
entry whose tag matches the tag that would result from creating
an entry with`XmSTRING_DEFAULT_CHARSET`is used.
For example, if creating an entry with`XmSTRING_DEFAULT_CHARSET`would result in the tag`ISO8859-1`, the compound string segment
tag`XmFONTLIST_DEFAULT_TAG`matches the font list entry tag`ISO8859-1`.

If no match has been found and if the tag of the compound string
segment matches the tag that would result from creating a segment
with`XmSTRING_DEFAULT_CHARSET`, the first font list entry
whose tag is`XmFONTLIST_DEFAULT_TAG`is used.

If no match has been found, the first entry in the font list is
used.

The font list interface consists of the routines listed
in`Related Information`.

Font lists are specified in resource files with the following syntax:`resource_spec``:``font_entry`[,`font_entry`]+

The resource value string consists of one or more font list entries
separated by commas. Each`font_entry`identifies a font or
font set and an optional font list entry tag. A tag specified for a
single font follows the font name and is separated
by = (equals sign); otherwise,
in a font set the tag is separated by a colon. The colon is required
whether a tag is specified or not. A font entry uses the following
syntax to specify a single font:`font_name`[ '='tag]

For example, the following entry specifies a 10 point Times Italic font
without a font list entry tag;*fontList:  -Adobe-Times-Medium-I-Normal--10*

A font entry containing a font set is similar, except a semicolon
separates multiple font names and the specification ends with
a colon followed by an optional tag:`font_name`[ ';'`font_name`]+ ':' [tag]

A`font_name`is an X Logical Font Description (XLFD) string
andtagis any set of characters from ISO646IRV except
space, comma, colon, equal sign and semicolon. Following is an example of a font
set entry. It consists of three fonts (except for charsets),
and an explicit font list entry tag.*fontList: -Adobe-Courier-Bold-R-Normal--25-180-100-100-M-150;\
-JIS-Fixed-Medium-R-Normal--26-180-100-100-C-240;\
-JIS-Fixed-Medium-R-Normal--26-180-100-100-C-120:MY_TAG

Note that theXmRenderTableis another data type that can be
used for font lists. Refer to the &cdeman.XmRenderTable; for details.
## RELATED


&cdeman.XmFontListAdd;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListCopy;,
&cdeman.XmFontListCreate;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryGetFont;,
&cdeman.XmFontListEntryGetTag;,
&cdeman.XmFontListEntryLoad;,
&cdeman.XmFontListFree;,
&cdeman.XmFontListFreeFontContext;,
&cdeman.XmFontListGetNextFont;,
&cdeman.XmFontListInitFontContext;,
&cdeman.XmFontListNextEntry;,
&cdeman.XmFontListRemoveEntry;,
&cdeman.XmRenderTable;,
and
&cdeman.XmString;.