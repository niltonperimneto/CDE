# XmCreateQuestionDialog
library call`XmCreateQuestionDialog`The MessageBox QuestionDialog convenience creation functionXmCreateQuestionDialogcreation functionsXmCreateQuestionDialog#include <Xm/MessageB.h>Widget`XmCreateQuestionDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateQuestionDialog`is a convenience creation function that
creates a DialogShell and an unmanaged MessageBox child of the DialogShell.
A QuestionDialog is used to get the answer to a question from the user.
It includes a symbol, a message, and
three buttons. The default symbol is a question mark. The default button
labels are`OK`,`Cancel`, and`Help`.

Use`XtManageChild`to pop up the QuestionDialog (passing the MessageBox
as the widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreateQuestionDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of MessageBox and its associated resources, see
&cdeman.XmMessageBox;.
## RETURN


Returns the MessageBox widget ID.
## RELATED


&cdeman.XmMessageBox;.