# DtEditorDisableRedisplay
library call`DtEditorDisableRedisplay`temporarily prevent visual update of a DtEditor widget#include <Dt/Editor.h>void`DtEditorDisableRedisplay`Widgetwidget
## DESCRIPTION


The`DtEditorDisableRedisplay`function prevents redisplay of a DtEditor widget even
though its visual attributes have been modified.
The visual appearance of the widget remains unchanged until
&cdeman.DtEditorEnableRedisplay; is called.
This allows an application to make multiple
changes to an editor widget without causing intermediate
visual updates.

The`widget`argument specifies the DtEditor widget ID.