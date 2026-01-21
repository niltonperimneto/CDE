# dtfile_copy
user cmddtfile_copythe CDE File Manager copy utilitydtfile_copyoptions ...source_folder target_folder
## DESCRIPTION


The CDE File Manager copy utility is used to
recursively copy folders and their contents, including subfolders. The
utility's default action is to create a duplicate of the source_folder
in the target_folder's location. Thus objects which exist in the target
but not in the source are deleted, objects which exist in the source but
not in the target are copied, and objects which exist in the target and
in the source are replaced if they are different. The utility compares
both timestamp and size of two objects to determine if they are
identical.

The copy utility is invoked by the File Manager whenever a user requests
a folder be moved or copied. Its use is thus transparent to the user.
However, it can also be explicitly invoked from a shell window. The
utility has many options which can be used to modify its default
behavior.
## OPTIONS


The following options are available from the command line:
### -dontDoIt


Write a description of the actions that would be performed to a dialog window,
but do not modify any objects.
### -keepNew


If an object exists in the source and target folders, do not replace the
target object if it is newer than the source object.
### -keepOld


If an object exists in the source and target folders, rename the
existing target object by appending .old to the name before copying the
source.
### -dontDelete


If an object exists in the target folder but not the source, do not
delete the target object.
### -dontAdd


If an object exists in the source folder but not the target, do not copy
the source file.
### -dontReplace


If an object exists in the source and target folders, do not replace the
target object.
### -dontRecur


Process only the files in the source folder, do not process any
subfolders.
### -keepLinks


If the target object is a symbolic link to the source object, retain the link
instead of replacing the link by a copy of the source object.
### -keepCopies


If a source object is a symbolic link and the target object is a
a copy of the object that the source link points at (i.e., has same
size and timestamp), retain the target object instead of replacing
it by a symbolic link.
### -forceCopies


If an object exists in the source and target folders, copy the source
object even if the timestamps and sizes are equal.
### -linkFolders


If a folder exists in the source but not the target, create a symbolic link in
the target pointing to the source instead of copying the source folder.
### -linkFiles


If a file exists in the source but not the target, create a symbolic link in the
target pointing to the source instead of copying the source file.
### -copyFolders


If the source is a symbolic link to a folder, make a copy of the folder
that the source link points at, instead of just copying the link.
### -copyFiles


If the source is a symbolic link to a file, makes a copy of the file
that the source link points at, instead of just copying the link.
### -copyTop


If the target folder does not exist, create one.
### -move


Following a successful copy operation, remove the source folder.
### -confirmReplace


If an object exists in the source and target directories, display a
dialog giving a choice of actions before proceeding.
### -confirmErrors


If an error occurs processing an object, display a dialog describing the
error before proceeding.
### -popDown


Following a successful copy or move operation, automatically remove the
dtfile_copy dialog after the interval specified by the delay option.
### -delay


The time, in microseconds, that the dtfile_copy dialog is displayed after a
successful copy operation is completed.
### -slow


Pause for a preset time interval between each file operation.
## EXAMPLES

### dtfile /u/aUser/FolderA /u/aUser/FolderA.backup


The folder /u/aUser/FolderA.backup is made to be a duplicate of
/u/aUser/FolderA. The name of each oject processed is written to a dialog window
with an indication of the operation performed.
## RESOURCES
`Name             Class            Type             Default`dontDoIt         DontDoIt         XmRBoolean       False
keepNew          KeepNew          XmRBoolean       False
keepOld          KeepOld          XmRBoolean       False
dontDelete       DontDelete       XmRBoolean       False
dontAdd          DontAdd          XmRBoolean       False
dontReplace      DontReplace      XmRBoolean       False
dontRecur        DontRecur        XmRBoolean       False
keepLinks        KeepLinks        XmRBoolean       False
keepCopies       KeepCopies       XmRBoolean       False
forceCopies      ForceCopies      XmRBoolean       False
linkFolders      LinkFolders      XmRBoolean       False
linkFiles        LinkFiles        XmRBoolean       False
copyFolders      CopyFolders      XmRBoolean       False
copyFiles        CopyFiles        XmRBoolean       False
copyTop          CopyTop          XmRBoolean       False
move             move             XmRBoolean       False
confirmReplace   ConfirmReplace   XmRBoolean       False
confirmErrors    ConfirmErrors    XmRBoolean       False
popDown          PopDown          XmRBoolean       False
delay            Delay            XmRBoolean       False
toggle           Toggle           XmRBoolean       True
slow             Slow             XmRBoolean       False
### Dtfile*dontDoIt:


Write a description of the actions that would be performed to a dialog window,
but do not modify any objects.
### Dtfile*keepNew:


If an object exists in the source and target folders, do not replace the
target object if it is newer than the source object.
### Dtfile*keepOld:


If an object exists in the source and target folders, rename the
existing target object by appending .old to the name before copying the
source.
### Dtfile*dontDelete:


If an object exists in the target folder but not the source, do not
delete the target object.
### Dtfile*dontAdd:


If an object exists in the source folder but not the target, do not copy
the source file.
### Dtfile*dontReplace:


If an object exists in the source and target folders, do not replace the
target object.
### Dtfile*dontRecur:


Process only the files in the source folder, do not process any
subfolders.
### Dtfile*keepLinks:


If the target object is a symbolic link to the source object, retain the link
instead of replacing the link by a copy of the source object.
### Dtfile*keepCopies:


If a source object is a symbolic link and the target object is a
a copy of the object that the source link points at (i.e., has same
size and timestamp), retain the target object instead of replacing
it by a symbolic link.
### Dtfile*forceCopies:


If an object exists in the source and target folders, copy the source
object even if the timestamps and sizes are equal.
### Dtfile*linkFolders:


If a folder exists in the source but not the target, create a symbolic link in
the target pointing to the source instead of copying the source folder.
### Dtfile*linkFiles:


If a file exists in the source but not the target, create a symbolic link in the
target pointing to the source instead of copying the source file.
### Dtfile*copyFolders:


If the source is a symbolic link to a folder, make a copy of the folder
that the source link points at, instead of just copying the link.
### Dtfile*copyFiles:


If the source is a symbolic link to a file, makes a copy of the file
that the source link points at, instead of just copying the link.
### Dtfile*copyTop:


If the target folder does not exist, create one.
### Dtfile*move:


Following a successful copy operation, remove the source folder.
### Dtfile*confirmReplace:


If an object exists in the source and target directories, display a
dialog giving a choice of actions before proceeding.
### Dtfile*confirmErrors:


If an error occurs processing an object, display a dialog describing the
error before proceeding.
### Dtfile*popDown:


Following a successful copy or move operation, automatically remove the
dtfile_copy dialog after the interval specified by the delay option.
### Dtfile*delay:


The time, in microseconds, that the dtfile_copy dialog is displayed after a
successful copy operation is completed.
### Dtfile*slow:


Pause for a preset time interval between each file operation.