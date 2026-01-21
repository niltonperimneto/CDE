# XmFileSelectionDoSearch
library call`XmFileSelectionDoSearch`A FileSelectionBox function that initiates a directory searchXmFileSelectionDoSearchFileSelectionBox functionsXmFileSelectionDoSearch#include <Xm/FileSB.h>void`XmFileSelectionDoSearch`WidgetwidgetXmStringdirmask
## DESCRIPTION


`XmFileSelectionDoSearch`initiates a directory and file search in a
FileSelectionBox widget.
For a description of the actions that the FileSelectionBox takes when
doing a search, see &cdeman.XmFileSelectionBox;.

* **`widget`** 

Specifies the FileSelectionBox widget ID.
* **`dirmask`** 

Specifies the directory mask used in determining the directories and
files displayed in the FileSelectionBox lists.
This value is used as the`mask`member of the input dataXmFileSelectionBoxCallbackStructstructure passed to the
FileSelectionBox's`XmNqualifySearchDataProc`.
The`dir`and`pattern`members of that structure are NULL.


For a complete definition of FileSelectionBox and its associated resources, see
&cdeman.XmFileSelectionBox;.
## RELATED


&cdeman.XmFileSelectionBox;.