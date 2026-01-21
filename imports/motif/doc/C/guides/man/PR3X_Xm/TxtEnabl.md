# XmTextEnableRedisplay
library call`XmTextEnableRedisplay`A Text function that forces the
visual update of a Text widgetXmTextEnableRedisplayText functionsXmTextEnableRedisplay#include <Xm/Text.h>void`XmTextEnableRedisplay`Widgetwidget
## DESCRIPTION


`XmTextEnableRedisplay`is used in conjunction with`XmTextDisableRedisplay`, which suppresses visual update
of the Text widget. When`XmTextEnableRedisplay`is
called, it determines if any visual attributes have been set
or modified for the specified widget since`XmTextDisableRedisplay`was called. If so, it forces the widget to update its visual display for
all of the intervening changes. Any subsequent changes that affect
visual appearance cause the widget to update its visual display.
This function also causes the insertion cursor, which is not shown
while redisplay is disabled, to be restored.

* **`widget`** 

Specifies the Text widget ID

## RELATED


&cdeman.XmTextDisableRedisplay;.