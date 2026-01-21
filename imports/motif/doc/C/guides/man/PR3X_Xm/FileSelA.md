# XmFileSelectionBox
library call`XmFileSelectionBox`The FileSelectionBox widget classXmFileSelectionBoxwidget classFileSelectionBox&npzwc;#include &lt;Xm/FileSB.h>
## DESCRIPTION


FileSelectionBox traverses
through directories, views the files and subdirectories in them,
and then selects files.

A FileSelectionBox has five main areas:

A text input field for displaying and editing a directory mask used to
select the files to be displayed

An optional text input field for displaying and editing a filter mask
used to select the files to be displayed.

A scrollable list of filenames

A scrollable list of subdirectories

A text input field for displaying and editing a filename

A group of PushButtons,
labeled`OK`,`Filter`,`Cancel`, and`Help`.
The layout direction of the buttons depends on the`XmNlayoutDirection`resource.

####Additional children may be added to the FileSelectionBox after
creation.
FileSelectionBox inherits the layout functionality provided
by SelectionBox for any additional children.
To remove the list of filenames, the list of subdirectories, or both
from the FileSelectionBox after creation, unmanage the
appropriate widgets and their labels.
The list and label widgets are obtained through a call to the`XmFileSelectionBoxGetChild`function.
To remove either the directory list or the file list, unmanage the
parent of the appropriate list widget and unmanage the corresponding
label.

The user can specify resources in a resource file for the automatically
created widgets and gadgets of FileSelectionBox. The following list
identifies the names of these widgets (or gadgets) and the associated
FileSelectionBox areas:

* **FilterLabel** 

`FilterText`
* **Filter Text** 

`TextField`
* **Directory List** 

`DirList`
* **Directory List Label** 

`Dir`
* **DirL** 

`Label`
* **DirText** 

`TextField`


The directory mask is a string specifying the base directory to be
examined and a search pattern.
Ordinarily, the directory list displays the subdirectories of the base
directory, as well as the base directory itself and its parent
directory.
The file list ordinarily displays all files and/or subdirectories in the
base directory that match the search pattern.

Optionally, the search pattern mask and the base directory can be
displayed in two separate text fields. This option is controlled by
the`XmNpathMode`resource. Using this alternate
display does not change the meaning of resources that control the
content of these fields:`XmNdirectory`,`XmNdirMask`,`XmNpattern`.

A procedure specified by the`XmNqualifySearchDataProc`resource
extracts the base directory and search pattern from the directory mask.
If the directory specification is empty, the current working directory
is used.
If the search pattern is empty, a pattern that matches all files is
used.

An application can supply its own`XmNqualifySearchDataProc`as well
as its own procedures to search for subdirectories and files.
The default`XmNqualifySearchDataProc`works as follows:
The directory mask is a pathname that can contain zero or more`wildcard`characters in its directory portion, its file portion, or
both.
The directory components of the directory mask &mdash; up to, but not
including, the first component with a wildcard character &mdash; specify the
directory to be searched, relative to the current working directory.
The remaining components specify the search pattern.
If the directory mask is empty or if its first component contains a
wildcard character, the current working directory is searched.
If no component of the directory mask contains a wildcard character, the
entire directory mask is the directory specification, and all files in
that directory are matched.

The user can select a new directory to examine by scrolling through the
list of directories and selecting the desired directory or by editing
the directory mask.
Selecting a new directory from the directory list does not change the
search pattern.
A user can select a new search pattern by editing the directory mask
or, when the FileSelectionBox has the optional`XmNpathMode
XmPATH_MODE_RELATIVE`display, the filter text field.
Double clicking or pressing`KActivate`on a directory in the
directory list initiates a search for files and subdirectories in the
new directory, using the current search pattern.

The user can select a file by scrolling through the list of filenames
and selecting the desired file or by entering the filename directly into
the text edit area.
Selecting a file from the list causes that filename to appear in the
file selection text edit area.

The user may select a new file as many times as desired.
The application is not notified until the user takes one of the
following actions:

Selects the`OK`PushButton

Presses`KActivate`while the selection text edit area has the
keyboard focus

Double clicks or presses`KActivate`on an item in the file list

FileSelectionBox initiates a directory and file search when any of the
following occurs:

The FileSelectionBox is initialized

The function`XtSetValues`is used to change`XmNdirMask`,`XmNdirectory`,`XmNpattern`, or`XmNfileTypeMask`

The user activates the`Filter`PushButton

The user double clicks or presses`KActivate`on an item in the
directory list

The application calls`XmFileSelectionDoSearch`

The user presses`KActivate`while the directory mask text edit area
has the keyboard focus

When a file search is initiated, the FileSelectionBox takes the
following actions:

Constructs anXmFileSelectionBoxCallbackStructstructure with
values appropriate for the action that initiated the search

Calls the`XmNqualifySearchDataProc`with the callback structure as
the data input argument

Sets`XmNdirectoryValid`and`XmNlistUpdated`to False

Calls the`XmNdirSearchProc`with the qualified data returned by the`XmNqualifySearchDataProc`

If`XmNdirectoryValid`is True, the FileSelectionBox takes the
following additional actions:

Sets`XmNlistUpdated`to False

Calls the`XmNfileSearchProc`with the qualified data returned by
the`XmNqualifySearchDataProc`(and possibly modified by the`XmNdirSearchProc`)

If`XmNlistUpdated`is True and the file list is empty, displays the`XmNnoMatchString`in the file list and clears the selection text
and`XmNdirSpec`

If`XmNlistUpdated`is True and the file list is not empty, sets the
selection text and`XmNdirSpec`to the qualified`dir`returned
by the`XmNqualifySearchDataProc`(and possibly modified by the`XmNdirSearchProc`)

Sets the directory mask text and`XmNdirMask`to the qualified`mask`returned by the`XmNqualifySearchDataProc`(and possibly
modified by the`XmNdirSearchProc`)

Sets`XmNdirectory`to the qualified`dir`returned by the`XmNqualifySearchDataProc`(and possibly modified by the`XmNdirSearchProc`)

Sets`XmNpattern`to the qualified`pattern`returned by the`XmNqualifySearchDataProc`(and possibly modified by the`XmNdirSearchProc`)

FileSelectionBox uses the`XmQTactivatable`trait.
### Data Transfer Behavior


Child widgets of a FileSelectionBox support the data transfer operations
and targets associated with their widget classes.

In addition, if the source of a data transfer is the directory list and
if`XmNdirSearchProc`has its default value, the directory list
supports the`FILE`and`FILE_NAME`targets.

If the source of a data transfer is the file list and if`XmNfileSearchProc`has its default value, the file list supports
the`FILE`and`FILE_NAME`targets.

In either case, FileSelectionBox adds an`XmNconvertCallback`procedure to the appropriate list.
This procedure adds`FILE`and`FILE_NAME`to the`TARGETS`returned by the list.
It treats requests for conversion of a selection to`FILE`and`FILE_NAME`exactly like requests for conversion to`TEXT`.

If an application changes`XmNdirSearchProc`or`XmNfileSearchProc`and wants to support the`FILE`and`FILE_NAME`targets on the corresponding list, it must provide
support itself by adding a procedure to the list's`XmNconvertCallback`list.
### Descendants


FileSelectionBox automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`Apply``XmPushButtonGadget`Apply button`Cancel``XmPushButtonGadget`Cancel button`Dir``XmLabelGadget`title above list of directories`DirList``XmList`list of directories`DirListSW``XmScrolledWindow`ScrolledWindow parent of`DirList``FilterLabel``XmLabelGadget`title above filter box`FilterText``XmText`or`XmTextField`text within filter box`Help``XmPushButtonGadget`Help button`Items``XmLabelGadget`title above list of filenames`ItemsList``XmList`list of filenames`ItemsListSW``XmScrolledWindow`ScrolledWindow parent of`ItemsList``OK``XmPushButtonGadget`OK button`Selection``XmLabelGadget`title above selection box`Separator``XmSeparatorGadget`optional dividing line`Text``XmText`or`XmTextField`text within selection box
### Classes


FileSelectionBox inherits behavior,
resources, and traits from`Core`,`Composite`,`Constraint`,`XmManager`,`XmBulletinBoard`, and`XmSelectionBox`.

The class pointer is`xmFileSelectionBoxWidgetClass`.

The class name is`XmFileSelectionBox`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmFileSelectionBox Resource Set``Name``Class``Type``Default``Access`XmNdirectoryXmCDirectoryXmStringdynamicCSGXmNdirectoryValidXmCDirectoryValidBooleandynamicSGXmNdirListItemsXmCDirListItemsXmStringTabledynamicSGXmNdirListItemCountXmCDirListItemCountintdynamicSGXmNdirListLabelStringXmCDirListLabelStringXmStringdynamicCSGXmNdirMaskXmCDirMaskXmStringdynamicCSGXmNdirSearchProcXmCDirSearchProcXmSearchProcdefault procedureCSGXmNdirSpecXmCDirSpecXmStringdynamicCSGXmNdirTextLabelStringXmCDirTextLabelStringXmStringNULLCXmNfileFilterStyleXmCFileFilterStyleXtEnumXmFILTER_NONECXmNfileListItemsXmCItemsXmStringTabledynamicSGXmNfileListItemCountXmCItemCountintdynamicSGXmNfileListLabelStringXmCFileListLabelStringXmStringdynamicCSGXmNfileSearchProcXmCFileSearchProcXmSearchProcdefault procedureCSGXmNfileTypeMaskXmCFileTypeMaskunsigned charXmFILE_REGULARCSGXmNfilterLabelStringXmCFilterLabelStringXmStringdynamicCSGXmNlistUpdatedXmCListUpdatedBooleandynamicSGXmNnoMatchStringXmCNoMatchStringXmString"&numsp;[&numsp;&numsp;&numsp;&numsp;]&numsp;"CSGXmNpathModeXmCPathModeXtEnumXmPATH_MODE_FULLCXmNpatternXmCPatternXmStringdynamicCSGXmNqualifySearchDataProcXmCQualifySearchDataProcXmQualifyProcdefault procedureCSG

* **`XmNdirectory`** 

Specifies the base directory used in combination with`XmNpattern`in determining the files and directories to be displayed.
The default value is determined by the`XmNqualifySearchDataProc`and depends on the initial values of`XmNdirMask`,`XmNdirectory`, and`XmNpattern`.
If the default is NULL or empty, the current working directory is used.
* **`XmNdirectoryValid`** 

Specifies an attribute that is set only by the directory search
procedure.
The value is set to True if the directory passed to the directory search
procedure can actually be searched.
If this value is False the file search procedure is not called, and`XmNdirMask`,`XmNdirectory`, and`XmNpattern`are not
changed.
* **`XmNdirListItems`** 

Specifies the items in the directory list.`XtGetValues`for this resource returns the list items themselves,
not a copy of the list items.
The application must not free the returned items.
* **`XmNdirListItemCount`** 

Specifies the number of items in the directory list.
The value must not be negative.
* **`XmNdirListLabelString`** 

Specifies the label string of the directory list.
The default for this resource depends on the locale.
In the C locale the default is`Directories`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNdirMask`** 

Specifies the directory mask used
in determining the files and directories to be displayed.
The default value is determined by the`XmNqualifySearchDataProc`and depends on the initial values of`XmNdirMask`,`XmNdirectory`, and`XmNpattern`.
* **`XmNdirSearchProc`** 

Specifies a directory search procedure to replace the default
directory search procedure.
FileSelectionBox's default directory search procedure fulfills the needs
of most applications.
Because it is impossible to cover the requirements of all applications,
you can replace the default search procedure.

The directory search procedure is called with two arguments:
the FileSelectionBox widget and a pointer to anXmFileSelectionBoxCallbackStructstructure.
The callback structure is generated by the`XmNqualifySearchDataProc`and contains all information required to
conduct a directory search, including the directory mask and a qualified
base directory and search pattern.
Once called, it is up to the search routine to generate a new list of
directories and update the FileSelectionBox widget by using`XtSetValues`.

The search procedure must set`XmNdirectoryValid`and`XmNlistUpdated`.
If it generates a new list of directories, it must also set`XmNdirListItems`and`XmNdirListItemCount`.

If the search procedure cannot search the specified directory, it must
warn the user and set`XmNdirectoryValid`and`XmNlistUpdated`to False, unless it prompts and subsequently obtains a valid directory.
If the directory is valid but is the same as the current`XmNdirectory`, the search procedure must set`XmNdirectoryValid`to True, but it may elect not to generate a new
list of directories.
In this case, it must set`XmNlistUpdated`to False.

If the search procedure generates a new list of directories, it must set`XmNdirListItems`to the new list of directories and`XmNdirListItemCount`to the number of items in the list.
If there are no directories, it sets`XmNdirListItems`to NULL and`XmNdirListItemCount`to 0 (zero).
In either case, it must set`XmNdirectoryValid`and`XmNlistUpdated`to True.

The search procedure ordinarily should not change the callback structure.
But if the original directory is not valid, the search procedure may
obtain a new directory from the user.
In this case, it should set the`dir`member of the callback structure
to the new directory,
call the`XmNqualifySearchDataProc`with the
callback struct as the input argument, and copy the qualified data
returned by the`XmNqualifySearchDataProc`into the callback struct.
* **`XmNdirSpec`** 

Specifies the full file path specification.
This is the`XmNtextString`resource in SelectionBox, renamed for
FileSelectionBox.
The default value is determined by the FileSelectionBox after conducting
the initial directory and file search.
* **`XmNdirTextLabelString`** 

Uses the specifiedXmStringas the label above the TextField
directory. The resource takes effect when the`XmNpathMode`resource has a value of`XmPATH_MODE_RELATIVE`. It is ignored
when the`XmNpathMode`resource has a value of`XmPATH_MODE_FULL`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNfileFilterStyle`** 

Specifies whether or not the "hidden" files (those whose names begin
with &period; (period) in POSIX systems) will be listed in the file and
directory
scrolling lists (where the default directory search procedure is used).
The possible values are:

* **`XmFILTER_NONE`** 

Does not filter hidden files.
* **`XmFILTER_HIDDEN_FILES`** 

Restricts the list of possible file names, such as those beginning
with &period; (period).

* **`XmNfileListItems`** 

Specifies the items in the file list.
This is the`XmNlistItems`resource in SelectionBox, renamed for
FileSelectionBox.`XtGetValues`for this resource returns the list items themselves,
not a copy of the list items.
The application must not free the returned items.
* **`XmNfileListItemCount`** 

Specifies the number of items in the file list.
This is the`XmNlistItemCount`resource in SelectionBox, renamed for
FileSelectionBox.
The value must not be negative.
* **`XmNfileListLabelString`** 

Specifies the label string of the file list.
This is the`XmNlistLabelString`resource in SelectionBox, renamed
for FileSelectionBox.
The default for this resource depends on the locale.
In the C locale the default is`Files`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNfileSearchProc`** 

Specifies a file search procedure to replace the default file search
procedure.
FileSelectionBox's default file search procedure fulfills the needs of
most applications.
Because it is impossible to cover the requirements of all applications,
you can replace the default search procedure.

The file search procedure is called with two arguments:
the FileSelectionBox widget and a pointer to anXmFileSelectionBoxCallbackStructstructure.
The callback structure is generated by the`XmNqualifySearchDataProc`(and possibly modified by the`XmNdirSearchProc`).
It contains all information required to conduct a file search, including
the directory mask and a qualified base directory and search pattern.
Once this procedure is called,
it is up to the search routine to generate a new list of
files and update the FileSelectionBox widget by using`XtSetValues`.

The search procedure must set`XmNlistUpdated`.
If it generates a new list of files, it must also set`XmNfileListItems`and`XmNfileListItemCount`.

It is recommended that the search procedure always generate a new list of
files.
If the`mask`member of the callback structure is the same as the`mask`member of the callback struct in the preceding call to the
search procedure, the procedure may elect not to generate a new list of
files.
In this case it must set`XmNlistUpdated`to False.

If the search procedure generates a new list of files, it must set`XmNfileListItems`to the new list of files and`XmNfileListItemCount`to the number of items in the list.
If there are no files, it sets`XmNfileListItems`to NULL and`XmNfileListItemCount`to 0 (zero).
In either case it must set`XmNlistUpdated`to True.

In constructing the list of files, the search procedure should include
only files of the types specified by the widget's`XmNfileTypeMask`.

Setting`XmNdirSpec`is optional, but recommended.
Set this attribute to the full file specification of the directory
searched.
The directory specification is displayed below the directory and file
lists.
* **`XmNfileTypeMask`** 

Specifies the type of files listed in the file list.
The possible values are

* **`XmFILE_REGULAR`** 

Restricts the file list to contain only regular
files.
* **`XmFILE_DIRECTORY`** 

Restricts the file list to contain only
directories.
* **`XmFILE_ANY_TYPE`** 

Allows the list to contain all file types
including directories.

* **`XmNfilterLabelString`** 

Specifies the label string for the text entry field for the directory
mask.
The default for this resource depends on the locale.
In the C locale the default is`Filter`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNlistUpdated`** 

Specifies an attribute that is set only by the directory and file search
procedures.
This resource is set to True if the
search procedure updated the directory or file list.
* **`XmNnoMatchString`** 

Specifies a string to be displayed in the file list if the list of files
is empty.
* **`XmNpattern`** 

Specifies the search pattern used in combination with`XmNdirectory`in determining the files and directories to be displayed.
The default value is determined by`XmNqualifySearchDataProc`and depends on the initial values of`XmNdirMask`,`XmNdirectory`, and`XmNpattern`.
If the default is NULL or empty, a pattern that matches all files is
used.
* **`XmNpathMode`** 

Specifies whether or not an additional text field will be used to
display and edit the filter. The possible values are

* **`XmPATH_MODE_FULL`** 

Specifies that no additional text field will be used to display
the filter. There will just be a single text field to display`XmNdirMask`.
* **`XmPATH_MODE_RELATIVE`** 

Specifies that there will be two text field displays, one to display
the`XmNdirectory`and one to display the`XmNpattern`. In
this instance, the`XmNfilterLabelString`resource applies to the
text field for`XmNpattern`and`XmNdirTextLabelString`applies to the text
field for`XmNdirectory`.

* **`XmNqualifySearchDataProc`** 

Specifies a search data qualification procedure to replace the default
data qualification procedure.
FileSelectionBox's default data qualification procedure fulfills the
needs of most applications.
Because it is impossible to cover the requirements of all applications,
you can replace the default procedure.

The data qualification procedure is called to generate a qualified
directory mask, base directory, and search pattern for use by the
directory and file search procedures.
It is called with three arguments:
the FileSelectionBox widget and pointers to twoXmFileSelectionBoxCallbackStructstructures.
The first callback structure contains the input data.
The second callback structure contains the output data, to be filled in by
the data qualification procedure.

If the input`dir`and`pattern`members are not NULL, the
procedure must copy them to the corresponding members of the output
callback structure.

If the input`dir`is NULL, the procedure constructs the
output`dir`as follows:
If the input`mask`member is NULL, the procedure uses the
widget's`XmNdirectory`as the output`dir`; otherwise, it
extracts the output`dir`from the input`mask`.
If the resulting output`dir`is empty, the procedure uses
the current working directory instead.

If the input`pattern`is NULL, the procedure constructs
the output`pattern`as follows:
If the input`mask`member is NULL, the procedure uses the
widget's`XmNpattern`as the output`pattern`; otherwise, it
extracts the output`pattern`from the input`mask`.
If the resulting output`pattern`is empty, the procedure
uses a pattern that matches all files instead.

The data qualification procedure constructs the output`mask`from
the output`dir`and`pattern`.
The procedure must ensure that the output`dir`,`pattern`, and`mask`are fully qualified.

If the input`value`member is not NULL, the procedure must copy it
to the output`value`member; otherwise, the procedure must copy the
widget's`XmNdirSpec`to the output`value`.

The data qualification procedure must calculate the lengths of the
output`value`,`mask`,`dir`, and`pattern`members and
must fill in the corresponding length members of the output callback
struct.

The data qualification procedure must copy the input`reason`and`event`members to the corresponding output members.


The values of the`XmNdirSearchProc`and`XmNfileSearchProc`are procedure pointers of typeXmSearchProc, defined as
follows:void (* XmSearchProc) (`w, search_data`)
        Widget`w`;
        XtPointer`search_data`;

* **`w`** 

The FileSelectionBox widget
* **`search_data`** 

Pointer to anXmFileSelectionBoxCallbackStructcontaining
information for conducting a search


The value of the`XmNqualifySearchDataProc`resource
is a procedure pointer of typeXmQualifyProc, defined
as follows:void (* XmQualifyProc) (`w, input_data, output_data`)
        Widget`w`;
        XtPointer`input_data`;
        XtPointer`output_data`;

* **`w`** 

The FileSelectionBox widget
* **`input_data`** 

Pointer to anXmFileSelectionBoxCallbackStructcontaining
input data to be qualified
* **`output_data`** 

Pointer to anXmFileSelectionBoxCallbackStructcontaining
output data to be filled in by the qualification procedure

### Inherited Resources


FileSelectionBox inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmSelectionBox Resource Set``Name``Class``Type``Default``Access`XmNapplyCallbackXmCCallbackXtCallbackListNULLCXmNapplyLabelStringXmCApplyLabelStringXmStringdynamicCSGXmNcancelCallbackXmCCallbackXtCallbackListNULLCXmNcancelLabelStringXmCCancelLabelStringXmStringdynamicCSGXmNchildPlacementXmCChildPlacementunsigned charXmPLACE_ABOVE_SELECTIONCSGXmNdialogTypeXmCDialogTypeunsigned charXmDIALOG_FILE_SELECTIONGXmNhelpLabelStringXmCHelpLabelStringXmStringdynamicCSGXmNlistItemCountXmCItemCountintdynamicCSGXmNlistItemsXmCItemsXmStringTabledynamicCSGXmNlistLabelStringXmCListLabelStringXmStringdynamicCSGXmNlistVisibleItemCountXmCVisibleItemCountintdynamicCSGXmNminimizeButtonsXmCMinimizeButtonsBooleanFalseCSGXmNmustMatchXmCMustMatchBooleanFalseCSGXmNnoMatchCallbackXmCCallbackXtCallbackListNULLCXmNokCallbackXmCCallbackXtCallbackListNULLCXmNokLabelStringXmCOkLabelStringXmStringdynamicCSGXmNselectionLabelStringXmCSelectionLabelStringXmStringdynamicCSGXmNtextAcceleratorsXmCTextAcceleratorsXtAcceleratorsdefaultCXmNtextColumnsXmCColumnsshortdynamicCSGXmNtextStringXmCTextStringXmStringdynamicCSG

`XmBulletinBoard Resource Set``Name``Class``Type``Default``Access`XmNallowOverlapXmCAllowOverlapBooleanTrueCSGXmNautoUnmanageXmCAutoUnmanageBooleanFalseCGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNcancelButtonXmCWidgetWidgetCancel buttonSGXmNdefaultButtonXmCWidgetWidgetOK buttonSGXmNdefaultPositionXmCDefaultPositionBooleanTrueCSGXmNdialogStyleXmCDialogStyleunsigned chardynamicCSGXmNdialogTitleXmCDialogTitleXmStringNULLCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTabledynamicCSGXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension10CSGXmNmarginWidthXmCMarginWidthDimension10CSGXmNnoResizeXmCNoResizeBooleanFalseCSGXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_ANYCSGXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_OUTCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNtextTranslationsXmCTranslationsXtTranslationsNULLCXmNunmapCallbackXmCCallbackXtCallbackListNULLC

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetdynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicN/AXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        XmString`value`;
        int`length`;
        XmString`mask`;
        int`mask_length`;
        XmString`dir`;
        int`dir_length`;
        XmString`pattern`;
        int`pattern_length`;
} XmFileSelectionBoxCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`value`** 

Specifies the current value of`XmNdirSpec`
* **`length`** 

Specifies the number of bytes in`value`This member is obsolete and exists for compatibility with
earlier releases.
* **`mask`** 

Specifies the current value of`XmNdirMask`
* **`mask_length`** 

Specifies the number of bytes in`mask`This member is obsolete and exists for compatibility with
earlier releases.
* **`dir`** 

Specifies the current base directory
* **`dir_length`** 

Specifies the number of bytes in`dir`This member is obsolete and exists for compatibility with
earlier releases.
* **`pattern`** 

Specifies the current search pattern
* **`pattern_length`** 

Specifies the number of bytes in`pattern`This member is obsolete and exists for compatibility with
earlier releases.

### Translations


XmFileSelectionBox inherits translations from XmSelectionBox.
### Accelerators


The`XmNtextAccelerators`from XmSelectionBox are added to the
selection and directory mask (filter) Text descendants of
XmFileSelectionBox.
### Action Routines


The XmFileSelectionBox action routines are

* **SelectionBoxUpOrDown(`Previous|Next|First|Last`):** 

If neither the selection text nor the directory mask (filter) text has
the focus, this action does nothing.

If the selection text has the focus, the term`list`in the
following description refers to the file list, and the termtextrefers to the selection text.
If the directory mask text has the focus,`list`refers to the
directory list, andtextrefers to the directory mask text.

When called with an argument of`Previous`, or 0 (zero) for
compatibility, this action
selects the previous item in the
list and replaces the text with that item.

When called with an argument of`Next`, or 1 for
compatibility, this action
selects the next item in the
list and replaces the text with that item.

When called with an argument of`First`, or 2 for
compatibility, this action
selects the first item in the
list and replaces the text with that item.

When called with an argument of`Last`, or 3 for
compatibility, this action
selects the last item in the
list and replaces the text with that item.
* **SelectionBoxRestore():** 

If neither the selection text nor the directory mask (filter) text has
the focus, this action does nothing.

If the selection text has the focus, this action
replaces the selection text with
the selected item in the file list.
If no item in the file list is selected, it clears the selection text.

If the directory mask text has the focus, this action
replaces the directory mask
text with a new directory mask constructed from the`XmNdirectory`and`XmNpattern`resources.

### Additional Behavior


The FileSelectionBox widget has the following additional behavior:

* **KeyosfCancel:** 

Calls the activate callbacks for the cancel button if it is sensitive.
If no cancel button exists and the parent of the FileSelectionBox is a manager,
it passes the event to the parent.
* **KeyosfActivate&ensp;in&ensp;Selection&ensp;Text:** 

Calls the selection text widget's`XmNactivateCallback`callbacks.
If`XmNmustMatch`is True and the selection text does not match an
item in the file list, it calls the`XmNnoMatchCallback`callbacks with
reason`XmCR_NO_MATCH`.
Otherwise, it calls the`XmNokCallback`callbacks with reason`XmCR_OK`.
* **KeyosfActivate&ensp;in&ensp;Directory&ensp;Mask&ensp;Text:** 

Calls the directory mask text widget's`XmNactivateCallback`callbacks,
initiates a directory and file search, and
calls the`XmNapplyCallback`callbacks with reason`XmCR_APPLY`.
* **Btn1Down`(2+)`&ensp;or&ensp;KeyosfActivate&ensp;in&ensp;Directory&ensp;List:** 

Calls the directory list widget's`XmNdefaultActionCallback`callbacks,
initiates a directory and file search,
and calls the`XmNapplyCallback`callbacks with reason`XmCR_APPLY`.
* **Btn1Down`(2+)`&ensp;or&ensp;KeyosfActivate&ensp;in&ensp;File&ensp;List:** 

Calls the file list widget's`XmNdefaultActionCallback`callbacks and
calls the`XmNokCallback`callbacks with reason`XmCR_OK`.
* **KeyosfSelect&ensp;in&ensp;Directory&ensp;List:** 

Generates a new directory mask, using the selected list item as the
directory and the pattern extracted from the current directory mask text
as the search pattern.
If the search pattern is empty, it uses a pattern that matches all files in
the directory.
Replaces the directory mask text with the new directory mask.
* **KeyosfSelect&ensp;in&ensp;File&ensp;List:** 

Replaces the selection text with the selected list item.
* **Btn2Downin File List:** 

Drags the content of one or more selected list items using the drag
and drop facility. If`<Btn2Down`is pressed on an unselected item,
drags only that item, excluding any other selected items.

This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures of the file
list, possibly multiple times, for the_MOTIF_DROPselection.
* **Btn2Downin Directory List:** 

Drags the content of one or more selected list items using the drag
and drop facility. If`<Btn2Down`is pressed on an unselected item,
it drags only that item, excluding any other selected items.

This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures of the
directory list, possibly multiple times, for the_MOTIF_DROPselection.
* **Apply&ensp;Button&ensp;Activated:** 

Initiates a directory and file search.
Calls the`XmNapplyCallback`callbacks with reason`XmCR_APPLY`.
* **OK&ensp;Button&ensp;Activated:** 

If`XmNmustMatch`is True and the selection text does not match an
item in the file list, calls the`XmNnoMatchCallback`callbacks with
reason`XmCR_NO_MATCH`.
Otherwise, calls the`XmNokCallback`callbacks with reason`XmCR_OK`.
* **Cancel&ensp;Button&ensp;Activated:** 

Calls the`XmNcancelCallback`callbacks with reason`XmCR_CANCEL`.
* **Help&ensp;Button&ensp;Activated:** 

Calls the`XmNhelpCallback`callbacks with reason`XmCR_HELP`.
* **KeyosfActivate:** 

If no button, list widget, or text widget has the keyboard focus,
if`XmNmustMatch`is True and the selection text does not match an
item in the file list, it calls the`XmNnoMatchCallback`callbacks with
reason`XmCR_NO_MATCH`.
Otherwise, it calls the`XmNokCallback`callbacks with reason`XmCR_OK`.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmBulletinBoard;,
&cdeman.XmCreateFileSelectionBox;,
&cdeman.XmCreateFileSelectionDialog;,
&cdeman.XmFileSelectionBoxGetChild;,
&cdeman.XmFileSelectionDoSearch;,
&cdeman.XmManager;, and
&cdeman.XmSelectionBox;.