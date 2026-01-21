# XmOptionButtonGadget
library call`XmOptionButtonGadget`A RowColumn function that obtains the widget ID for the CascadeButtonGadget in an OptionMenuXmOptionButtonGadgetRowColumn functionsXmOptionButtonGadget#include <Xm/RowColumn.h>Widget`XmOptionButtonGadget`Widgetoption_menu
## DESCRIPTION


`XmOptionButtonGadget`provides the application with the means for
obtaining the widget ID for the internally created CascadeButtonGadget. Once
the application has obtained the widget ID, it can
adjust the visuals for the CascadeButtonGadget, if desired.

When an application creates an instance of the OptionMenu widget, the
widget creates two internal gadgets. One is a LabelGadget that is
used to display RowColumn's`XmNlabelString`resource.
The other is a CascadeButtonGadget that displays the current selection
and provides the means for posting the OptionMenu's submenu.

The user can specify resources in a resource file for the automatically
created widgets and gadgets of an OptionMenu. The following list
identifies the names of these widgets (or gadgets) and the associated
OptionMenu areas.

* **Option Menu Label Gadget** 

`OptionLabel`
* **Option Menu Cascade Button** 

`OptionButton`


* **`option_menu`** 

Specifies the OptionMenu widget ID


For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the widget ID for the internal button.
## RELATED


&cdeman.XmCreateOptionMenu;, &cdeman.XmCascadeButtonGadget;,
&cdeman.XmOptionLabelGadget;,
and &cdeman.XmRowColumn;.