# XmCreateBulletinBoardDialog
library call`XmCreateBulletinBoardDialog`The BulletinBoard BulletinBoardDialog convenience creation functionXmCreateBulletinBoard\\%Dialogcreation functionsXmCreateBulletinBoard\\%Dialog#include <Xm/BulletinB.h>Widget`XmCreateBulletinBoardDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateBulletinBoardDialog`is a convenience
creation function that creates a DialogShell and an unmanaged BulletinBoard
child of the DialogShell.
A BulletinBoardDialog is used for interactions not supported by the
standard dialog set.
This function does not automatically create
any labels, buttons, or other dialog components. Such components should be
added by the application after the BulletinBoardDialog is created.

Use`XtManageChild`to pop up the BulletinBoardDialog (passing the
BulletinBoard as the widget parameter); use`XtUnmanageChild`to pop
it down.

`XmCreateBulletinBoardDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of BulletinBoard and its associated resources, see
&cdeman.XmBulletinBoard;.
## RETURN


Returns the BulletinBoard widget ID.
## RELATED


&cdeman.XmBulletinBoard;.