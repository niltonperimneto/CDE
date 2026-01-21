# MrmCloseHierarchy
library call`MrmCloseHierarchy`Closes a UID hierarchyMrmCloseHierarchyuil functionsMrmCloseHierarchyuid hierarchy#include <Mrm/MrmPublic.h>Cardinal`MrmCloseHierarchy`MrmHierarchyhierarchy_idMRM functionMrmCloseHierarchyMrmCloseHierarchydefinition
## DESCRIPTION


The`MrmCloseHierarchy`functionMrmCloseHierarchydescriptioncloses a UID hierarchy previously opened by`MrmOpenHierarchyPerDisplay`.
All files associated with the hierarchy are closed by the
Motif Resource Manager (MRM) and all associated memory is returned.

* **`hierarchy_id`** 

Specifies the ID of a previously opened UID hierarchy.
The`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.

## RETURN
MrmSUCCESSMrmBAD_HIERARCHYMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmOpenHierarchyPerDisplay;.