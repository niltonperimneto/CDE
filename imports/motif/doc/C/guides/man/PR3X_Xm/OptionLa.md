# XmOptionLabelGadget
library call`XmOptionLabelGadget`A RowColumn function that obtains the widget ID for the LabelGadget in an OptionMenuXmOptionLabelGadgetRowColumn functionsXmOptionLabelGadget#include <Xm/RowColumn.h>Widget`XmOptionLabelGadget`Widgetoption_menu
## DESCRIPTION


`XmOptionLabelGadget`provides the application with the means for obtaining the
widget ID for the internally created LabelGadget.
Once the application has obtained the widget ID, it can
adjust the visuals for the LabelGadget, if desired.

* **`option_menu`** 

Specifies the OptionMenu widget ID


When an application creates an instance of the OptionMenu widget, the
widget creates two internal gadgets. One is a LabelGadget that is used
to display RowColumn's`XmNlabelString`resource. The other
is a CascadeButtonGadget that displays the current selection and provides
the means for posting the OptionMenu's submenu.

The user can specify resources in a resource file for the automatically
created widgets and gadgets of an OptionMenu. The following list
identifies the names of these widgets (or gadgets) and the associated
OptionMenu areas.

* **Option Menu Label Gadget** 

`OptionLabel`
* **Option Menu Cascade Button** 

`OptionButton`


For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the widget ID for the internal label.
## RELATED


&cdeman.XmCreateOptionMenu;, &cdeman.XmLabelGadget;,
&cdeman.XmOptionButtonGadget;, and &cdeman.XmRowColumn;.