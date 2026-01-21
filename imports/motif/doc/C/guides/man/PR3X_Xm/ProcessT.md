# XmProcessTraversal
library call`XmProcessTraversal`A function that determines which component receives keyboard events when a widget has the focusXmProcessTraversal#include <Xm/Xm.h>Boolean`XmProcessTraversal`WidgetwidgetXmTraversalDirectiondirection
## DESCRIPTION


`XmProcessTraversal`determines which component of a hierarchy
receives keyboard events when the hierarchy that contains the given
widget has keyboard focus.

`XmProcessTraversal`changes focus only when the keyboard focus
policy of the widget hierarchy is explicit.
If the`XmNkeyboardFocusPolicy`of the nearest shell ancestor of the
given widget is not`XmEXPLICIT`,`XmProcessTraversal`returns False without making any focus changes.

* **`widget`** 

Specifies the widget ID of the widget whose hierarchy is to be
traversed
* **direction** 

Specifies the direction of traversal

### DEFINITIONS


In order to be eligible to receive keyboard focus when the shell's`XmNkeyboardFocusPolicy`is`XmEXPLICIT`, a widget or gadget
must meet the following conditions:

The widget and its ancestors are not in the process of being destroyed.

The widget and its ancestors are`sensitive`.
A widget is sensitive when its`XmNsensitive`and`XmNancestorSensitive`resources are both True.

The`XmNtraversalOn`resource for the widget and its ancestors is
True.

The widget is viewable.
This means that the widget and its ancestors are managed, realized, and
(except for gadgets) mapped.
Furthermore, in general, some part of the widget's rectangular area must
be unobscured by the widget's ancestors. If an application unmaps a
widget that has its`XmNmappedWhenManaged`resource set to True, the
result is undefined.

In a ScrolledWindow with an`XmNscrollingPolicy`of`XmAUTOMATIC`, a widget that is obscured because it is not within
the clip window may be able to receive focus if some part of the widget
is within the work area and if an`XmNtraverseObscuredCallback`routine can make the widget at least partially visible by scrolling the
window.

In general only primitives, gadgets, and Drawing Area are eligible to
receive focus.
Most managers cannot receive focus even if they meet all these
conditions.

Thedirectionargument identifies the kind of traversal action to
take.
The descriptions of these actions below refer to traversable
non-tab-group widgets and traversable tab groups.

A traversable non-tab-group widget is a widget that is not a tab group
and that meets all the conditions for receiving focus described above.

A traversable tab group widget is a tab group widget that meets the same
conditions, except that a manager that is a tab group and meets the
other conditions is also eligible for traversal as long as it contains a
descendant that can receive focus.

A tab group is a widget whose`XmNnavigationType`is:

`XmTAB_GROUP`or`XmSTICKY_TAB_GROUP`, if the hierarchy (up to
the nearest shell ancestor) that contains the widget has no widget whose`XmNnavigationType`is`XmEXCLUSIVE_TAB_GROUP`

`XmEXCLUSIVE_TAB_GROUP`or`XmSTICKY_TAB_GROUP`, if the
hierarchy (up to the nearest shell ancestor) that contains the widget
has any widget whose`XmNnavigationType`is`XmEXCLUSIVE_TAB_GROUP`
### TRAVERSAL ACTIONS


The hierarchy to be traversed is that containing the`widget`argument.
This hierarchy is traversed only up to the nearest shell;`XmProcessTraversal`does not move focus from one shell to another.
If the shell containing`widget`does not currently have the focus,
any change that`XmProcessTraversal`makes to the element with focus
within that shell does not take effect until the next time the shell
receives focus.

`XmProcessTraversal`begins the traversal action from the widget in
the hierarchy that currently has keyboard focus or that last had focus
when the user traversed away from the shell hierarchy.

The value of thedirectionargument determines which of three
kinds of traversal action to take:

Traversal to a non-tab-group widget.
This kind of traversal is possible only when the widget that currently
has focus is not a tab group; otherwise,`XmProcessTraversal`returns False for these actions.

These actions do not move focus from one tab group to another.
The actions first determine the containing tab group.
This is the tab group containing the widget that currently has focus.
The actions traverse only to a non-tab-group widget within the
containing tab group.

A non-tab-group widget is eligible for this kind of traversal if the
widget is traversable and has no tab group ancestors up to the
containing tab group.
If the tab group contains no traversable non-tab-group widgets,`XmProcessTraversal`returns False.

Following are the possible values of thedirectionargument.
Note that when actions wrap, wrapping occurs in the traversal
direction. The following describes what happens in a left
to right environment:

`XmTRAVERSE_RIGHT`&mdash;If the`XmNnavigationType`of the
containing tab group is not`XmEXCLUSIVE_TAB_GROUP`, focus moves to
the next traversable non-tab-group widget to the right of the widget
that currently has focus.
In a left to right environment, at the right side of the tab group
this action wraps to the
non-tab-group widget at the left side and next toward the bottom.
At the rightmost widget in the bottom row of the tab group this action
wraps to the
non-tab-group widget at the leftmost widget in the upper row.

In a right to left environment, at the right side of the tab group,
this action wraps to the non-tab-group widget at the left side and
next toward the top. At the rightmost widget in the upper row of the
tab group this action wraps to the non-tab-group widget at the
leftmost widget in the bottom row.

If the`XmNnavigationType`of the containing tab group is`XmEXCLUSIVE_TAB_GROUP`, focus moves to the next traversable
non-tab-group widget in the tab group, proceeding in the order in which
the widgets appear in their parents'`XmNchildren`lists.
After the last widget in the tab group, this action wraps to the first
non-tab-group widget.

`XmTRAVERSE_LEFT`&mdash;If the`XmNnavigationType`of the
containing tab group is not`XmEXCLUSIVE_TAB_GROUP`, focus moves to
the next traversable non-tab-group widget to the left of the widget that
currently has focus.
In a left to right environment, at the left side of the tab group this
action wraps to the non-tab-group
widget at the right side and next toward the top.
At the leftmost widget in the upper row of the tab group this action
wraps to the non-tab-group widget at the rightmost widget in the
bottom row.

In a right to left environment, at the left side of the tab group this
action wraps to the non-tab-group widget at the right side and next
toward the bottom. At the leftmost widget in the bottom row of the
tab group this
action wraps to the non-tab-group widget at the rightmost widget in
the upper row.

If the`XmNnavigationType`of the containing tab group is`XmEXCLUSIVE_TAB_GROUP`, focus moves to the previous traversable
non-tab-group widget in the tab group, proceeding in the reverse order
in which the widgets appear in their parents'`XmNchildren`lists.
After the first widget in the tab group, this action wraps to the last
non-tab-group widget.

`XmTRAVERSE_DOWN`&mdash;If the`XmNnavigationType`of the
containing tab group is not`XmEXCLUSIVE_TAB_GROUP`, focus moves to
the next traversable non-tab-group widget below the widget that
currently has focus.
In a left to right environment, at the bottom of the tab group this
action wraps to the non-tab-group
widget at the top and next toward the right.
At the bottom widget in the rightmost column of the tab group this action
wraps to the
non-tab-group widget at the top widget in the leftmost column.

In a right to left environment, at the bottom of the tab group this
action wraps to the non-tab-group widget at the top and next toward
the left. At the bottom widget of the leftmost widget of the tab
group this action wraps
to the non-tab-group widget at the top widget of the rightmost column.

If the`XmNnavigationType`of the containing tab group is`XmEXCLUSIVE_TAB_GROUP`, focus moves to the next traversable
non-tab-group widget in the tab group, proceeding in the order in which
the widgets appear in their parents'`XmNchildren`lists.
After the last widget in the tab group, this action wraps to the first
non-tab-group widget.

`XmTRAVERSE_UP`&mdash;If the`XmNnavigationType`of the containing
tab group is not`XmEXCLUSIVE_TAB_GROUP`, focus moves to the next
traversable non-tab-group widget above the widget that currently has
focus.
In a left to right environment, at the top of the tab group this
action wraps to the non-tab-group
widget at the bottom and next toward the left.
At the top widget of the leftmost column of the tab group this action
wraps to the
non-tab-group widget at the bottom widget of the rightmost column.

In a right to left environment, at the top of the tab group this
action wraps to the non-tab-group widget at the bottom and next toward
the right. At the top widget of the right most column of the tab
group this action
wraps to the non-tab-group widget at the bottom widget of the leftmost
column.

If the`XmNnavigationType`of the containing tab group is`XmEXCLUSIVE_TAB_GROUP`, focus moves to the previous traversable
non-tab-group widget in the tab group, proceeding in the reverse order
in which the widgets appear in their parents'`XmNchildren`lists.
After the first widget in the tab group, this action wraps to the last
non-tab-group widget.

`XmTRAVERSE_NEXT`&mdash;Focus moves to the next traversable
non-tab-group widget in the tab group, proceeding in the order in which
the widgets appear in their parents'`XmNchildren`lists.
After the last widget in the tab group, this action wraps to the first
non-tab-group widget.

`XmTRAVERSE_PREV`&mdash;Focus moves to the previous traversable
non-tab-group widget in the tab group, proceeding in the reverse order
in which the widgets appear in their parents'`XmNchildren`lists.
After the first widget in the tab group, this action wraps to the last
non-tab-group widget.

`XmTRAVERSE_HOME`&mdash;If the`XmNnavigationType`of the containing
tab group is not`XmEXCLUSIVE_TAB_GROUP`, focus moves to the first
traversable non-tab-group widget at the initial focus of the tab
group.

If the`XmNnavigationType`of the containing tab group is`XmEXCLUSIVE_TAB_GROUP`, focus moves to the first traversable
non-tab-group widget in the tab group, according to the order in which
the widgets appear in their parents'`XmNchildren`lists.

Traversal to a tab group.
These actions first determine the current widget hierarchy and the
containing tab group.
The current widget hierarchy is the widget hierarchy whose root is the
nearest shell ancestor of the widget that currently has focus.
The containing tab group is is the tab group containing the widget that
currently has focus.
If the current widget hierarchy contains no traversable tab groups,`XmProcessTraversal`returns False.

Following are the possible values of thedirectionargument. If
any tab group in the current widget hierarchy has an`XmNnavigationType`of`XmEXCLUSIVE_TAB_GROUP`, traversal of
tab groups in the hierarchy proceeds to widgets in the order in which
their`XmNnavigationType`resources were specified as`XmEXCLUSIVE_TAB_GROUP`or`XmSTICKY_TAB_GROUP`.:

`XmTRAVERSE_NEXT_TAB_GROUP`&mdash;Finds the hierarchy that contains`widget`,
finds the active tab group (if any), and makes the next tab group the
active tab group in the hierarchy.

`XmTRAVERSE_PREV_TAB_GROUP`&mdash;Finds the hierarchy that contains`widget`,
finds the active tab group (if any), and makes the previous tab group the
active tab group in the hierarchy.

Traversal to any widget.
In this case the`widget`argument is the widget to which`XmProcessTraversal`tries to give focus.
If the widget is not traversable,`XmProcessTraversal`returns
False.

Following are the possible values of thedirectionargument:

`XmTRAVERSE_CURRENT`&mdash;Finds the hierarchy and the tab group that
contain`widget`.
If this tab group is not the active tab group, this action
makes it the active tab group.
If`widget`is an item in the active tab group, this action
makes it the active item.
If`widget`is the active tab group, this action
makes the first traversable
item in the tab group the active item.
### CAUTIONS


Using`XmProcessTraversal`to traverse to MenuBars,
Pulldown menu panes, or Popup menu panes is not supported.

`XmProcessTraversal`cannot be called recursively.
In particular, an application cannot call this routine from an`XmNfocusCallback`or`XmNlosingFocusCallback`procedure.
## RETURN


Returns True if the traversal action succeeded.
Returns False if the`XmNkeyboardFocusPolicy`of the nearest shell
ancestor of`widget`is not`XmEXPLICIT`, if the traversal
action finds no traversable widget to receive focus, or if the call to
the routine has invalid arguments.
## RELATED


&cdeman.XmGetVisibility; and
&cdeman.XmIsTraversable;.