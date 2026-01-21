# XmText
library call`XmText`The Text widget classXmTextwidget classText#include <Xm/Text.h>
## DESCRIPTION


Text provides a single-line and multiline text editor for customizing both user and
programmatic interfaces. It can be used for single-line string entry,
forms entry with verification procedures, and full-window editing.
It provides an application with a consistent editing system for
textual data. The screen's textual data adjusts to the
application writer's needs.

Text provides separate callback lists to verify
movement of the insert cursor, modification of the text, and
changes in input focus. Each of
these callbacks provides the verification function with the
widget instance, the event that caused the callback, and a
data structure specific to the verification type. From this
information, the function can verify if the application considers
this to be a legitimate state change and can signal the widget
whether to continue with the action.

The user interface tailors a new set of translations. The default
translations provide key bindings for insert cursor movement, deletion,
insertion, and selection of text.

Text allows the user to select regions of text.
Selection is based on the model specified in theInter-Client
Communication Conventions Manual(ICCCM). Text supports primary
and secondary selection.

In some Asian languages, texts are drawn vertically. Also, some characters
are displayed with 90-degree clockwise rotation, and other characters are mapped
to vertical glyphs that differ from the normal horizaontal
glyphs.