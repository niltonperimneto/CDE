# XmTextDisableRedisplay
library call`XmTextDisableRedisplay`A Text function that temporarily
prevents visual update of the Text widgetXmTextDisableRedisplayText functionsXmTextDisableRedisplay#include <Xm/Text.h>void`XmTextDisableRedisplay`Widgetwidget
## DESCRIPTION


`XmTextDisableRedisplay`prevents redisplay of the specified
Text widget even though its visual attributes have been
modified. The visual appearance of the widget remains
unchanged until`XmTextEnableRedisplay`is called,
although the insertion cursor is not displayed.
This allows an application to make multiple changes to
the widget without causing intermediate visual updates.

* **`widget`** 

Specifies the Text widget ID

## RELATED


&cdeman.XmTextEnableRedisplay;.