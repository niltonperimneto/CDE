# XmStringComponentType
library callXmStringComponentTypeData type for compound string componentsXmStringComponentTypedata typesXmStringComponentType&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmStringComponentTypeis the data type specifying compound string
component types. A compound string component identifies some
part of a compound string, and can have a value and length. A compound
string component can be one of the
following types. These component types are grouped
according to their length and value types.

The following components have values of NULL and lengths of 0 (zero).

* **`XmSTRING_COMPONENT_SEPARATOR`** 

This component usually maps to a newline or carriage return in
displayed text.
* **`XmSTRING_COMPONENT_TAB`** 

This component may be thought of as a text component containing only a
single tab.
* **`XmSTRING_COMPONENT_LAYOUT_POP`** 

The layout direction is kept on a stack, with the current direction
kept on top of the stack. When this component is read, the most
recently read layout direction is popped off the stack and replaced by
the direction immediately before it.
* **`XmSTRING_COMPONENT_END`** 

This component marks the end of a compound string. No other components
should follow. If an application does not place an`XmSTRING_COMPONENT_END`component at the end of
anXmString, Motif automatically does it for the application.


The following component has a value ofXmDirectionand the
length of that direction.

* **`XmSTRING_COMPONENT_LAYOUT_PUSH`** 

The layout direction is kept on a stack, with the current direction
kept on top of the stack. This component replaces the current layout
direction, and causes another to be pushed onto the top of this stack.


The following component has a value ofXmStringDirectionand the
length of that direction.

* **`XmSTRING_COMPONENT_DIRECTION`** 

This component sets the string direction by overriding the previous
string direction.


The following components have values of typechar *or some
equivalent type, and the lengths of these types.

* **`XmSTRING_COMPONENT_LOCALE_TEXT`** 

This component contains the multibyte text of a compound string.
* **`XmSTRING_COMPONENT_WIDECHAR_TEXT`** 

This component contains the widechar text of a compound string.
* **`XmSTRING_COMPONENT_TEXT`** 

This component contains the charset text of a compound string. Note
that a compound string cannot contain both charset and locale
(multibyte or widechar) text.
* **`XmSTRING_COMPONENT_RENDITION_BEGIN`** 

This component marks the beginning of a new rendition. All text
following this component will be rendered using this rendition as the
primary one. If there is already a rendition in effect, it is kept in
memory and used to fill in any unspecified values in the primary
rendition. Renditions are kept until a corresponding`XmSTRING_COMPONENT_RENDITION_END`component is encountered.
* **`XmSTRING_COMPONENT_RENDITION_END`** 

This component signifies that the specified rendition will no longer
be used to render text, and will not be available to fill in
unspecified values of newer renditions.
* **`XmSTRING_COMPONENT_UNKNOWN`** 

This component type signifies that the component contents belong to an
unknown component type.
* **`XmSTRING_COMPONENT_LOCALE`** 

Use this component to specify the locale in which an internationalized
application is to execute.
The only valid character string for this component
is`_MOTIF_DEFAULT_LOCALE`.
* **`XmSTRING_COMPONENT_TAG`** 

For charset text, this is the tag of the font to be used to display
the text. This tag is sometimes referred to as the charset tag or the
fontlist tag.
* **`XmSTRING_COMPONENT_CHARSET`** 

This component is obsolete and remains for compatibility with previous
releases. It has been replaced by`XmSTRING_COMPONENT_TAG`.
* **`XmSTRING_COMPONENT_FONTLIST_ELEMENT_TAG`** 

This component is obsolete and remains for compatibility with previous
releases. It has been replaced by`XmSTRING_COMPONENT_TAG`.


Some compound string components depend on values defined in other
components. The`XmSTRING_COMPONENT_TAB`component definition, for
example, depends on information in the`XmSTRING_COMPONENT_RENDITION_BEGIN`. To account for these
dependencies, a typical compound string will have its member
components in the following order:[
  [`XmSTRING_COMPONENT_LAYOUT_PUSH`]
  [`XmSTRING_COMPONENT_RENDITION_BEGIN`]*
  [`XmSTRING_COMPONENT_TAG`|`XmSTRING_COMPONENT_LOCALE`]
  [`XmSTRING_COMPONENT_TAB`]*
  [`XmSTRING_COMPONENT_DIRECTION`]
  [`XmSTRING_COMPONENT_TEXT`|`XmSTRING_COMPONENT_LOCALE_TEXT`|`XmSTRING_COMPONENT_WIDECHAR_TEXT`]
  [`XmSTRING_COMPONENT_RENDITION_END`]*
  [`XmSTRING_COMPONENT_LAYOUT_POP`]
  [`XmSTRING_COMPONENT_SEPARATOR`]
]*`XmSTRING_COMPONENT_END`