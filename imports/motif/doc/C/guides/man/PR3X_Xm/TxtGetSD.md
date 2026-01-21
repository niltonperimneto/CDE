# XmTextGetSource
library call`XmTextGetSource`A Text function that accesses the source of the widgetXmTextGetSourceText functionsXmTextGetSource#include <Xm/Text.h>XmTextSource`XmTextGetSource`Widgetwidget
## DESCRIPTION


`XmTextGetSource`accesses the source of the Text widget. Text
widgets can share sources of text so that editing in one widget is
reflected in another. This function accesses the source of one widget
so that it can be made the source of another widget, using the function
&cdeman.XmTextSetSource;.

Setting a new text source destroys the old text source if no other Text
widgets are using that source.
To replace a text source but keep it for later use, create an unmanaged
Text widget and set its source to the text source you want to keep.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns anXmTextSourcevalue that represents the source of the Text
widget.
## RELATED


&cdeman.XmText;.