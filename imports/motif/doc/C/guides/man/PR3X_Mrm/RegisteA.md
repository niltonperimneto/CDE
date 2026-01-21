# MrmRegisterClass
library call`MrmRegisterClass`Saves the information needed for MRM to access the widget creation function for user-defined widgetsMrmRegisterClassuil functionsMrmRegisterClass#include <Mrm/MrmPublic.h>Cardinal`MrmRegisterClass`MrmTypeclass_codeStringclass_nameStringcreate_nameWidget (*create_proc) ()WidgetClassclass_recordMRM functionMrmRegisterClassMrmRegisterClassdefinition
## DESCRIPTION


The`MrmRegisterClass`functionMrmRegisterClassdescriptionallows MRM to access user-defined widget classes.
This function registers the necessary information for
MRM to create widgets of this class.
You must call`MrmRegisterClass`prior to fetching any user-defined class widget.

`MrmRegisterClass`saves the information needed to access the widget creation
function and to do type conversion of argument lists by using the
information in MRM databases.

* **`class_code`** 

This argument is ignored; it is present for compatibility with previous
releases.
* **`class_name`** 

This argument is ignored; it is present for compatibility with previous
releases.
* **`create_name`** 

Specifies the case-sensitive name of the low-level widget creation
function for the class.
An example from the Motif Toolkit is`XmCreateLabel`.
Arguments are`parent_widget`,`name`,`override_arglist`,
and`override_argcount`.

For user-defined widgets,`create_name`is the creation procedure in the UIL that defines this widget.
* **`create_proc`** 

Specifies the address of the creation function that you named in`create_name`.
* **`class_record`** 

Specifies a pointer to the class record.

## RETURN
MrmSUCCESSMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmFAILURE`** 

The function failed.
