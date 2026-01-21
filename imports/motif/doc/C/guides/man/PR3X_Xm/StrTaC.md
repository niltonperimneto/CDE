# XmStringTableProposeTablist
library call`XmStringTableProposeTablist`A convenience function that returns a tab listXmStringTableProposeTablist#include <Xm/Xm.h>XmTabList`XmStringTableProposeTablist`XmStringTablestringsCardinalnum_stringsWidgetwidgetfloatpad_valueXmOffsetModeloffset_model
## DESCRIPTION


`XmStringTableProposeTablist`takes anXmStringTablestructure containing tabbed compound strings, information on padding
between columns, and rendering information and returns a tab list
that, if used to render the strings in the table, would cause the
strings to line up in columns with no overlap and with the specified
amount of padding between the widest item in each column and the start
of the next column. Each tab in the tablist would have the same unit
type as`units`, an offset model of`offset_model`, and an
alignment type of`XmALIGNMENT_BEGINNING`.

* **`strings`** 

Specifies an array of compound strings.
* **`num_strings`** 

Specifies the number of compound strings in`strings`.
* **`widget`** 

Specifies the widget used for deriving any necessary information for
creating the rendition. In particular, the`XmNunitType`of`widget`will be used to specify the unit type
to be used in determining the amount of padding
separating columns and for the tabs in the proposed tab list. Also,`widget`'s render table will be used in interpreting rendition
tags within the strings.
* **`pad_value`** 

Specifies the value of the amount of padding to be used to separate
columns.
The units for this parameter are specified as
the`XmNunitType`set for the`widget`parameter.
Refer to the`XmNunitType`resource of the`XmGadget`,`XmManager`, or`XmPrimitive`reference page.
* **`offset_model`** 

Specifies the offset model to be used in creating the tabs. Can be`XmABSOLUTE`or`XmRELATIVE`.

## RETURN


Returns a newXmTabList.
The function allocates space to hold the returned tab list.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmTabListFree`.
## RELATED


&cdeman.XmTabList; and
&cdeman.XmTabListFree;.