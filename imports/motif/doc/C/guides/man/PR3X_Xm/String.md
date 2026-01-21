# XmString
library callXmStringData type for a compound stringXmStringdata typesXmString&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmStringis the data type for a compound string.
Compound strings consist of a sequence of components, including, but
not limited to, the following:

`XmSTRING_COMPONENT_SEPARATOR`

`XmSTRING_COMPONENT_TAB`

`XmSTRING_COMPONENT_LAYOUT_POP`

`XmSTRING_COMPONENT_DIRECTION`

`XmSTRING_COMPONENT_LAYOUT_PUSH`

`XmSTRING_COMPONENT_CHARSET`

`XmSTRING_COMPONENT_FONTLIST_ELEMENT_TAG`

`XmSTRING_COMPONENT_LOCALE`

`XmSTRING_COMPONENT_LOCALE_TEXT`

`XmSTRING_COMPONENT_TAG`

`XmSTRING_COMPONENT_TEXT`

`XmSTRING_COMPONENT_END`

`XmSTRING_COMPONENT_RENDITION_BEGIN`

`XmSTRING_COMPONENT_RENDITION_END`

`XmSTRING_COMPONENT_UNKNOWN`

`XmSTRING_COMPONENT_WIDECHAR_TEXT`

and also a rendition tags table, text, and text component.
When a
compound string is displayed, the rendition tags and the
direction are used to determine how to display the text.

Calling`XtGetValues`for a resource whose type isXmStringyields a
copy of the compound string resource value.
The application is responsible for using`XmStringFree`to free the
memory allocated for the copy.

Please see theXmStringComponentTypereference page for more
detail about compound string components, and for a description of the
order in which the components should appear in a compound string.
Refer to theXmRenderTablereference page for a description
of the algorithm that associates the rendition tags used for
displaying a compound string text component with a rendition in a
render table.
## RELATED


&cdeman.XmParseMapping;,
&cdeman.XmParseMappingCreate;,
&cdeman.XmParseMappingFree;,
&cdeman.XmParseMappingGetValues;,
&cdeman.XmParseMappingSetValues;,
&cdeman.XmParseTable;,
&cdeman.XmParseTableFree;,
&cdeman.XmStringBaseline;,
&cdeman.XmStringByteCompare;,
&cdeman.XmStringByteStreamLength;,
&cdeman.XmStringCompare;,
&cdeman.XmStringComponentCreate;,
&cdeman.XmStringComponentType;,
&cdeman.XmStringConcat;,
&cdeman.XmStringConcatAndFree;,
&cdeman.XmStringCopy;,
&cdeman.XmStringCreate;,
&cdeman.XmStringCreateLocalized;,
&cdeman.XmStringCreateLtoR;,
&cdeman.XmStringCreateSimple;,
&cdeman.XmStringDirection;,
&cdeman.XmStringDirectionCreate;,
&cdeman.XmStringDirectionToDirection;,
&cdeman.XmStringDraw;,
&cdeman.XmStringDrawImage;,
&cdeman.XmStringDrawUnderline;,
&cdeman.XmStringEmpty;,
&cdeman.XmStringExtent;,
&cdeman.XmStringFree;,
&cdeman.XmStringFreeContext;,
&cdeman.XmStringGenerate;,
&cdeman.XmStringGetLtoR;,
&cdeman.XmStringGetNextComponent;,
&cdeman.XmStringGetNextSegment;,
&cdeman.XmStringGetNextTriple;,
&cdeman.XmStringHasSubstring;,
&cdeman.XmStringHeight;,
&cdeman.XmStringInitContext;,
&cdeman.XmStringIsVoid;,
&cdeman.XmStringLength;,
&cdeman.XmStringLineCount;,
&cdeman.XmStringNConcat;,
&cdeman.XmStringNCopy;,
&cdeman.XmStringParseText;,
&cdeman.XmStringPeekNextComponent;,
&cdeman.XmStringPeekNextTriple;,
&cdeman.XmStringPutRendition;,
&cdeman.XmStringSegmentCreate;,
&cdeman.XmStringSeparatorCreate;,
&cdeman.XmStringTable;,
&cdeman.XmStringTableParseStringArray;,
&cdeman.XmStringTableProposeTablist;,
&cdeman.XmStringTableToXmString;,
&cdeman.XmStringTableUnparse;,
&cdeman.XmStringToXmStringTable;,
&cdeman.XmStringUnparse;,
&cdeman.XmStringWidth;,
&cdeman.XmCvtXmStringToByteStream;,
&cdeman.XmCvtXmStringToCT;,
&cdeman.XmCvtCTToXmString;, and
&cdeman.XmCvtByteStreamToXmString;.