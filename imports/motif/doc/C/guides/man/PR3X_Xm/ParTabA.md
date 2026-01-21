# XmParseTable
library callXmParseTableData type for a compound string parse tableXmParseTabledata typesXmParseTable&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmParseTableis the data type for an array of parse mappings
(objects of typeXmParseMapping).

A parse table is used by some routines that parse and unparse compound
strings.
The table is an ordered list of parse mappings.
A parsing routine that uses a parse table scans the input text and
searches the parse mappings, in order, for one containing a pattern that
matches the input text.
The matching parse mapping supplies a compound string to be included in
the compound string under construction.

An unparsing routine that uses a parse table searches the parse
mappings, in order, for one containing a compound string that matches
the input compound string.
The unparsing routine can then include the parse mapping's text pattern
in the output text under construction.
## RELATED


&cdeman.XmParseMapping;,
&cdeman.XmParseTableFree;, and
&cdeman.XmString;.