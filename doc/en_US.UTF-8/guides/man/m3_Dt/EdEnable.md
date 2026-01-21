# DtEditorEnableRedisplay
library call`DtEditorEnableRedisplay`force the visual update of a DtEditor widget#include <Dt/Editor.h>void`DtEditorEnableRedisplay`Widgetwidget
## DESCRIPTION


The`DtEditorEnableRedisplay`function forces a DtEditor widget to update its visual
display if any visual attributes have been set or modified
since the
&cdeman.DtEditorDisableRedisplay; function was called for the specified widget.
Any subsequent changes that affect the widget's visual
appearance will cause the widget to update its display.
These functions allow an application to make multiple
changes to an editor widget without causing intermediate
visual updates.

The`widget`argument specifies the DtEditor widget ID.