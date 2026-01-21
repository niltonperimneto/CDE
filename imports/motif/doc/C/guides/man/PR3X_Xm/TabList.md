# XmTabList
library callXmTabListData type for a tab listXmTabList&npzwc;#include &lt;Xm/Xm.h>
XmTabList
## DESCRIPTION


XmTabListis the data type for a tab list. A tab list consists
of tab stop list entries (XmTabs). Whenever a tab component is
encountered while anXmStringis being rendered, the origin of
the next X draw depends on the nextXmTab. If a tab stop would
cause text to overlap, the x position for the segment is reset to
follow immediately after the end of the previous segment.

Tab lists are specified in resource files with the following syntax:`resource_spec``:``tab`WHITESPACE [, WHITESPACE`tab`]*

The resource value string consists of one or more tabs separated by
commas. Each`tab`identifies the value of the tab, the unit
type, and whether the offset is relative or absolute. For example:`tab`:=`float`[ WHITESPACE`units`]`float`:= [`sign`] [[ DIGIT]*. ]DIGIT+`sign`:= +

where the presence or absence of`sign`indicates, respectively, a
relative offset or an absolute offset. Note that negative tab values
are not allowed.`units`indicates the unitType to use as
described in the`XmConvertUnits`reference page.

For example, the following specifies a tab list consisting of a one
inch absolute tab followed by a one inch relative tab:*tabList: 1in, +1in

For resources of type, dimension, or position, you can specify units
as described in the`XmNunitType`resource of the`XmGadget`,`XmManager`, or`XmPrimitive`reference page.
## RELATED


Refer to the &MotifProgGd; for more information about tabs and tab lists.
&cdeman.XmTabListCopy;,
&cdeman.XmTabListFree;,
&cdeman.XmTabListGetTab;,
&cdeman.XmTabListInsertTabs;,
&cdeman.XmTabListRemoveTabs;,
&cdeman.XmTabListReplacePositions;, and
&cdeman.XmTabListTabCount;.