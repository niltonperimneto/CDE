# MrmOpenHierarchyPerDisplay
library call`MrmOpenHierarchyPerDisplay`Allocates a hierarchy ID and opens all the UID files in the hierarchyMrmOpenHierarchyPer\\%Displayuil functionsMrmOpenHierarchyPer\\%Display#include <Mrm/MrmPublic.h>Cardinal`MrmOpenHierarchyPerDisplay`Display *displayMrmCountnum_filesStringfile_names_list[]MrmOsOpenParamPtr *ancillary_structures_listMrmHierarchy *hierarchy_id
## DESCRIPTION


`MrmOpenHierarchyPerDisplay`allows you to specify the list
of UID files that MRM searches in subsequent fetch operations.
All subsequent fetch operations return the first occurrence of the
named item encountered while traversing the UID hierarchy from
the first list element (UID file specification) to the last list
element.
This function also
allocates a hierarchy ID and opens all the UID files in the
hierarchy.
It initializes the optimized search lists in the hierarchy.
If`MrmOpenHierarchyPerDisplay`encounters any errors during its execution, any files that were opened
are closed.

The application must call`XtAppInitialize`before calling`MrmOpenHierarchyPerDisplay`.

* **`display`** 

Specifies the connection to the X server and the value to pass
to`XtResolvePathname`. For more information on theDisplaystructure, see the Xlib function`XOpenDisplay`.
* **`num_files`** 

Specifies the number of files in the name list.
* **`file_names_list`** 

Specifies an array of character strings that identify
the UID files.
* **`ancillary_structures_list`** 

A list of operating-system-dependent ancillary structures corresponding
to items such as filenames, clobber flags, and so forth.
This argument should be NULL for most operations.
If you need to reference this structure,
see the definition ofMrmOsOpenParamPtrin theMrmPublic.hheader file for more information.
* **`hierarchy_id`** 

Returns the search hierarchy ID.
The search hierarchy ID identifies the list of UID files that
MRM searches (in order) when performing subsequent
fetch calls.


Each UID file string in`file_names_list`can specify either a full
pathname or a filename.
If a UID file string has a leading / (slash), it specifies a full
pathname, and MRM opens the file as specified.
Otherwise, the UID file string specifies a filename.
In this case MRM looks for the file along a search path specified by the`UIDPATH`environment variable or by a default search path, which
varies depending on whether or not the`XAPPLRESDIR`environment
variable is set.

The`UIDPATH`environment variable specifies a search path and
naming conventions associated with UID files.
It can contain the substitution field`%U`, where the UID file string from
the`file_names_list`argument to`MrmOpenHierarchyPerDisplay`is
substituted for`%U`.
It can also contain the substitution fields accepted by`XtResolvePathname`.
The substitution field`%T`is always mapped to`uid`.
The entire path is searched first with`%S`mapped to.uid.
If no file is found, it is searched again with`%S`mapped to NULL.
For example, the following`UIDPATH`value and`MrmOpenHierarchyPerDisplay`call cause MRM to open two separate UID
files:UIDPATH=/uidlib/%L/%U.uid:/uidlib/%U/%L
  static char *uid_files[] = {"/usr/users/me/test.uid", "test2"};
  MrmHierarchy  *Hierarchy_id;
  MrmOpenHierarchyPerDisplay((MrmCount)2,uid_files, NULL, Hierarchy_id)

MRM opens the first file,/usr/users/me/test.uid, as specified in
the`file_names_list`argument to`MrmOpenHierarchyPerDisplay`,
because the UID file string in the`file_names_list`argument
specifies a full pathname.
MRM looks for the second file,test2,
first as/uidlib/%L/test2.uidand second as/uidlib/test2/%L,
where the display's language string is substituted for`%L`.

After`MrmOpenHierarchyPerDisplay`opens the UID hierarchy, you should not delete or modify the
UID files until you close the UID hierarchy by calling`MrmCloseHierarchy`.

If`UIDPATH`is not set, but the environment variable`XAPPLRESDIR`is set, MRM searches the following pathnames:

%U%S

$XAPPLRESDIR/%L/uid/%N/%U%S

$XAPPLRESDIR/%l/uid/%N/%U%S

$XAPPLRESDIR/uid/%N/%U%S

$XAPPLRESDIR/%L/uid/%U%S

$XAPPLRESDIR/%l/uid/%U%S

$XAPPLRESDIR/uid/%U%S

$HOME/uid/%U%S

$HOME/%U%S

/usr/lib/X11/%L/uid/%N/%U%S

/usr/lib/X11/%l/uid/%N/%U%S

/usr/lib/X11/uid/%N/%U%S

/usr/lib/X11/%L/uid/%U%S

/usr/lib/X11/%l/uid/%U%S

/usr/lib/X11/uid/%U%S

/usr/include/X11/uid/%U%S

If neither`UIDPATH`nor`XAPPLRESDIR`is set, MRM searches the
following pathnames:

%U%S

$HOME/%L/uid/%N/%U%S

$HOME/%l/uid/%N/%U%S

$HOME/uid/%N/%U%S

$HOME/%L/uid/%U%S

$HOME/%l/uid/%U%S

$HOME/uid/%U%S

$HOME/%U%S

/usr/lib/X11/%L/uid/%N/%U%S

/usr/lib/X11/%l/uid/%N/%U%S

/usr/lib/X11/uid/%N/%U%S

/usr/lib/X11/%L/uid/%U%S

/usr/lib/X11/%l/uid/%U%S

/usr/lib/X11/uid/%U%S

/usr/include/X11/uid/%U%S

These paths are defaults that vendors may change.
For example, a vendor may use different directories for/usr/lib/X11and/usr/include/X11.

The following substitutions are used in these paths:

* **`%U`** 

The UID file string, from the`file_names_list`argument.
* **`%N`** 

The class name of the application.
* **`%L`** 

The display's language string.
This string is influenced by`XtSetLanguageProc`.
The default string is determined by
callingsetlocale(`LC_ALL, NULL`).
* **`%l`** 

The language component of the display's language string.
* **`%S`** 

The suffix to the filename.
The entire path is first searched with a suffix of.uid.
If
no file is found, it is searched again with a NULL suffix.

## RETURN


This function returns one of the following status return constants:MrmSUCCESSMrmNOT_FOUNDMrmFAILURE

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmNOT_FOUND`** 

File not found.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmCloseHierarchy;.