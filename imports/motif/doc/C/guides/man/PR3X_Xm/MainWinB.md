# XmMainWindowSep1
library call`XmMainWindowSep1`A MainWindow function that returns the widget ID of the first SeparatorXmMainWindowSep1MainWindow functionsXmMainWindowSep1#include <Xm/MainW.h>Widget`XmMainWindowSep1`Widgetwidget
## DESCRIPTION


`XmMainWindowSep1`returns the widget ID of the first Separator
in the MainWindow.
The first Separator is located between the MenuBar and the Command widget.
This Separator is visible only when`XmNshowSeparator`is True.

`NOTE:``XmMainWindowSep1`is obsolete and exists for compatibility
with previous releases. Use`XtNameToWidget`instead. Pass
a MainWindow variable as the first argument to`XtNameToWidget`and pass`Separator1`as the second argument.

* **`widget`** 

Specifies the MainWindow widget ID


For a complete definition of MainWindow and its associated resources, see
&cdeman.XmMainWindow;.
## RETURN


Returns the widget ID of the first Separator.
## RELATED


&cdeman.XmMainWindow;.