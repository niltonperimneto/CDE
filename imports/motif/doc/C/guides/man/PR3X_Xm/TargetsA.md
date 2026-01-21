# XmTargetsAreCompatible
library call`XmTargetsAreCompatible`A function that tests
whether the target types match between a drop site and source objectXmTargetsAreCompatibleDrag and Drop functionsXmTargetsAreCompatible#include <Xm/DragDrop.h>Boolean`XmTargetsAreCompatible`Display *displayAtom *export_targetsCardinalnum_export_targetsAtom *import_targetsCardinalnum_import_targets
## DESCRIPTION


`XmTargetsAreCompatible`determines whether the import targets of
the destination match any of the export targets of a source.
If there is at least one target in common, the function returns True.

* **`display`** 

Specifies the display connection.
* **`export_targets`** 

Specifies the list of target atoms associated with the source object.
This resource identifies the selection targets the source
can convert to.
* **`num_export_targets`** 

Specifies the number of entries in the list of export targets.
* **`import_targets`** 

Specifies the list of targets to be checked against the`XmNexportTargets`of the source associated with the
specified DragContext
* **`num_import_targets`** 

Specifies the number of entries in the`import_targets`list.

## RETURN


Returns a Boolean value that indicates whether the destination
targets are compatible with the source targets. If there is at
least one target in common, the routine returns True; otherwise,
returns False.
## RELATED


&cdeman.XmDragContext; and
&cdeman.XmDropSite;.