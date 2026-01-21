
# Create Action Tasks

# Creating and Editing Actions





# Creating and Editing Data Types





# Icons for Actions and Data Types



# To Create an Action With Create Action
action:creating

Open the Desktop_Apps application group in Application Manager and
double-click Create Action.

This displays the Create Action main window.

In the Action Name text field, type the name that will label the action's icon.

The action name can include any characters except spaces.

Use the Action Icons controls to specify the icon for the application. Initially,
the default icon is shown.

To use a different existing icon, click Find Set to open the Find Set
dialog box. For more information, see.

To create new icons, click Edit Icon to run the Icon Editor.
See.

In the Command When Action Icon is Opened text field, type the
command to start the application.

Use the syntax$`n`for a file
argument. For example:emacs
bitmap $1
diff $1 $2
lp -oraw $1

If the command line includes a file argument ($`n`), then the action
icon will be a drop zone for files.

The command lines are not passed to a shell unless you explicitly specify
the use of a shell. For example, these lines use shell processing:/bin/sh -c `ps | lp'
/bin/sh -c `spell $1 | more'

In the Help Text for Action Icon text field, type the help information to be
displayed when the user gets on-item help on theaction icon.

The text will automatically wrap in the text field. However, these line
breaks are not preserved online. If you want to specify a "hard" line break,
use&bsol;n.

Use the Window Type button menu to select the kind of windowing support
required

* **Graphical (X-Window)** 

The application creates its own window.
* **Terminal (Auto-Close)** 

The application will be run in a terminal window that
closes automatically when the user exits the
application.
* **Terminal (Manual Close)** 

The application will be run in a terminal window that
remains open until the user explicitly closes it.
* **No Output** 

The application does not produce output to the display.


Proceed as follows:

If your application has data files and you want to create one or more
data types for them, see.

If you do not need to create a data type:

Save the action by choosing Save from the File menu.

Test the new action by double-clicking its icon in your home directory.
# To Create a Data Type with Create Action
data type:creating

Define the action for the application. See steps 1 through 6 of the
topic.

Click the Advanced button to expand the Create Action window.

If you want theaction iconto prompt for a file argument when the icon
is double-clicked, type the text of the prompt into the "When Action Opens,
Ask Users for" text field.

You must use this field if the application's command line has a required file
argument.

You must leave this field blank if the application's command line does not have a
file argument.

If the file argument in the application's command line is optional, you have
a choice.
If you supply the prompt text, the action icon will
prompt for the file
when double-clicked.
If you do not supply the prompt text, the action will be executed with a
null string as the file argument.

Specify the types of files that the action will accept as arguments:

If the action can accept any data type, click All Datatypes.

If the action can accept only the data type(s) you create for the
application, click Only Above List.

Initially, the Datatypes That use This Action list is empty. As you create
data types for the
application, they are added to the list.

Click Add to display the Add Datatype dialog box.

Optional: If you don't want to use the default data type name, type a new
name for the new data type into the Name of Datatype text field. The name
cannot include spaces.

Click the Edit button beside the Identifying Characteristics box to display
the Identifying Characteristics dialog box.

Use this dialog box to set the
characteristics used to differentiate the data type
from others. You can choose one criterion (for example, Name Pattern) or
combine criteria (for example, both Name and Permission Patterns).

Click either the File or Folder toggle button to specify whether the
data type presents a file or a folder.

If the data type depends on the name, click the Name Pattern check box
and type the name pattern. You can use*and?as wildcards.

* ***** 

Matches any sequence of characters.
* **?** 

Matches any single character.


If the data type depends on the permissions, click the
Permission Pattern check box and click the permissions required for
the data type.

* **On** 

The file must have the specified permission.
* **Off** 

The file must lack the specified permission.
* **Either** 

(Default) The permission does not matter.


If the data type depends on the contents, click the Contents check box
and supply the requested information-Pattern to search
for and Type of contents. Optionally, you can supply the byte location where
the search should start.

Click OK to close the File Characteristics dialog box.

The characteristics
will be displayed in the Identifying Characteristics box in the
Add Datatype dialog box.

The Permissions information in the Identifying Characteristics box uses this
coding:

* **d** 

Directory
* **r** 

Read permission
* **w** 

Write permission
* **x** 

Executable
* **!** 

NOT
* **&** 

AND


Type the help text for the data type into the Help Text text field

Use the Datatype Icons controls to specify the icon for the application.

Initially, the default icons are shown.

To use a different existing icon, click Find Set to open the Find Set
dialog box. For more information, see.

To create new icons, click Edit Icon to run the Icon Editor.
See.

Optional: If the application supplies a print command for printing data
files from the
command line, type the command into the Print Command text field, using
the syntax$`n`for a file argument.

Click OK to close the Add Datatype dialog box and add the data type
to the data types list in the Create Action window.
# To Specify Icons for Actions and Data Types


The Create Action Main window and Add Datatype dialog box contain buttons
for specifying the icon to be used by actions and data types.

To use an existing icon, click Find Set. See.

To create a new icon using Icon Editor, click Edit Icon. See.
# To Use the Find Set Dialog Box


The Find Set dialog box is displayed when you click Find Set in the Create
Action main window or in the Create Datatype dialog box.

The Find Set dialog box lets you specify:

An icon located in a folder in the Icons Folders list.
The Icon Folders list includes all the folders on the icon search path.

An icon that is part of a registration package that will be integrated
with the desktop usingdtappintegrate.
# To Specify an Icon in an Icon Folder


In the Icon Folders list, double-click the folder path containing
the icon.

The Icon Files list will show all the icon files in that folder.

In the Icon Files list, click the icon you want to use.

Click OK.
# To Specify an Icon in a Registration Package


If you are a system administrator or programmer creating a registration
package, the icon image files are initially located in a directory in the
registration package:`app_root`/dt/appconfig/icons/`language`

After registration, the icon files will be in anicon folder.
Therefore, the action and data type definitions
should use the base file name.

Use this procedure to specify a base name for an icon that is not currently
in an icon folder:

In the Enter Icon Filename text box, type thebase nameof the icon file.

Click OK.

An error dialog box is displayed, informing you that the icon file wasn't found
in an icon folder.

In the error dialog box, click Name OK. This closes the dialog box and
the Find Set dialog box.

Ignore the absence of an icon image in the Action Icons or Datatype icons
field. The icon image will be found when the application has been registered.
# Creating a New Icon Image


Click Edit Icon in the Create Action or Edit Datatype window.
This runs Icon Editor.

Use Icon Editor to draw a new icon. See:

Icon Editor Tasks.



Choose Save from the File menu to save the icon file.

Save the icon file into the`HomeFolder`/.dt/iconsfolder.
See.

If you are a system administrator or applications programmer creating a
registration package, the icon file should be saved to
the`approot`/dt/appconfig/icons/`language`directory.
# Icon Sizes and Names


Desktop icons use this naming convention:

* **Size (Pixels)** 

Name
* **32 by 32** 

`basename`.m.pmor`basename`.m.bm
* **16 by 16** 

`basename`.s.pmor`basename`.s.bm


For action icons, use the action name as the basename.

For data type icons, use the data type name as the base name.
# To Edit an Existing Action
actions:editing

You can use Create Action to edit an existing action if:

The action was created with Create Action.

And, the the file containing the action definition has not been
edited manually (using a text editor).

Open the Desktop_Apps application group in Application Manager and
double-click Create Action.

This displays the main Create Action window.

Choose Open from the File menu. This displays the Open dialog box.

In the Files list, select the file containing the action definition.
It has the name`action_name`.dt.

Choose OK.
# To Edit an Existing Data Type
data type:editing

You can use Create Action to edit an existing data type if:

The data type was created with Create Action.

And, the file containin the data type
has not been edited manually (using a text editor).

Open the Desktop_Apps application group in Application Manager and
double-click Create Action.

This displays the main Create Action window.

Choose Open from the File menu. This displays the Open dialog box.

In the Files list, double-click the file containing the data type definition.

The name of the file is`action_name`.dt, whereaction_nameis
the name of the action that was created at the same time you created
the data type.

Click Advanced.

In the Data Types That Use This Action list, select the data type to be
edited.

Click Edit to open the Edit Datatype dialog box.

Make changes in the Edit Datatype dialog box. When you are done, click OK.

To save the edited definition, choose Save from the File menu.