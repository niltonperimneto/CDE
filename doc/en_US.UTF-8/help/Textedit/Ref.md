
# Text Editor Reference

# Text Editor Editing and Cursor Keys





# Text Editor Menus















# Text Editor Window And Dialog Boxes





















# General Reference







# See Also



# Text Editor Menu Accelerators
menus: accelerators

* **Close** 

Alt+F4
* **Copy** 

Control+C
* **Cut** 

Control+X
* **Delete** 

Delete key
* **Find/Change** 

Control+F
* **Overstrike** 

Insert key
* **Paste** 

Control+V
* **Print** 

Control+P
* **Select All** 

Control+/
* **Undo** 

Control+Z


If your keyboard does not have an Alt key, ask your system
administrator to identify the corresponding key.Note that not all vendors' keyboards supply these keys.
Please see your local vendor's documentation for alternate key bindings.
# Editing and Cursor Keys
editing keyscontrol keyAlt key
# Editing Keys


* **Key** 

Action
* **Backspace** 

Deletes the character before the cursor
* **Delete (Remove)** 

Deletes the character following the
insertion cursor
* **Control+Delete
(Control+Remove)** 

Deletes all the characters from the cursor to the end of the current line
* **Control+Backspace** 

Deletes the previous word
* **Shift+Backspace** 

Deletes characters from the cursor to the beginning of the line

# Cursor Keys
control key, used with arrow keysarrow keyscursor movement keyskeyboard navigationkeys: cursor movementkeys: arrow

* **Key** 

Cursor Movement
* **Up Arrow** 

Up one line
* **Down Arrow** 

Down one line
* **Left Arrow** 

Left one character
* **Right Arrow** 

Right one character
* **Control+Right&sigspace;Arrow&sigspace;** 

Right one word
* **Control+Left&sigspace;Arrow** 

Left one word
* **Control+Down&sigspace;Arrow&sigspace;** 

Beginning of the next paragraph
* **Control+Up&sigspace;Arrow** 

Beginning of the previous paragraph
* **Home** 

Beginning of the current line
* **End** 

End of the current line
* **Control+Home** 

Beginning of the document
* **Control+End** 

End of the document


Some key combinations may be different on the system you are using.
If so, ask your system administrator to identify the corresponding keys.

&sigspace;

&sigspace;

To use Emacs key bindings in Text Editor see.

&sigspace;
# See Also











# Unix Key Bindings
Unix keybindingskeybindings: UnixEmacs keybindings

Unix key bindings provide a set of extended Emacs keys,
such as Control+F (forward character) and Control+N (next line),
in Text Editor. To enable Unix key bindings (which are
set off by default), do the following:

Edit the.Xdefaultsfile in your home directory by
adding this line to the file:#include "/usr/dt/app-defaults/`lang`/UNIXbindings"

Replace`lang`with the value of your language environment
variable. If the.Xdefaultsfile does not exist,
create the file in your home directory.

Log out of your current session.

Log in and restart Text Editor.

&sigspace;

When you use Unix key bindings, Text Editor provides alternate menu
accelerators for these commands:

* **Command** 

Alternate Menu Accelerator
* **Undo (Control+Z)** 

Control+_
* **Paste (Control+V)** 

Shift+Insert
* **Find/Change (Control+F)&sigspace;** 

Control+S
* **Print (Control+P)** 

no alternate


&sigspace;
If you want to modify these menu accelerators, copy
the contents of the/usr/dt/app-defaults/`lang`/UNIXbindingsfile into your.Xdefaultsfile, and then make your
changes.

When Unix key bindings are enabled, the Delete (Remove) key deletes the previous
character rather than the character that follows the cursor.
# See Also







# Text Editor File Menu


menus:File

* **New** 

Clears the Text Editor window. If your document has unsaved
changes a dialog box is displayed that enables you to save your work before the window is cleared.
* **Open** 

Displays a dialog box for opening an existing
file.
* **Include** 

Displays a dialog box for
specifying a file to be inserted into the current
document.
* **Save** 

Saves the document to the current file. If
Wrap To Fit is set on the Save dialog is displayed. If the
current document has not been previously saved, the Save As dialog box is displayed.
* **Save As** 

Displays a dialog box for saving a document to a
new file.
* **Copy to File** 

Copies the information you are viewing or
editing to a separate file without changing your edit session. In some
instances, the Save As command is replaced by the Copy To File command.
* **Print** 

Displays a dialog box for selecting print options
and printing a document.
* **Close** 

Closes the Text Editor window and exits Text Editor.

# See Also







# Text Editor Edit Menu


menus:Edit

* **Undo** 

Reverses the last cut, paste, clear, delete, replace,
include, or format operation
* **Cut** 

Removes the selected text and stores it on the clipboard
* **Copy** 

Copies the selected text and stores it on the clipboard
* **Paste** 

Copies the text from the last cut or copy operation to the current
cursor position
* **Clear** 

Replaces the selected text with spaces
* **Delete** 

Deletes the selected text
* **Select All** 

Selects all text in your document
* **Find/Change** 

Opens a dialog box that enables you to search for words or
phrases in your document and make changes to occurrences
that are found
* **Check Spelling&sigspace;** 

Runs the spell checker on the document

# See Also







# Text Editor Format Menu


menus:Format

* **Settings** 

Displays a dialog box for
setting margins and paragraph alignment, and applying format settings to
your document
* **Paragraph** 

Applies the settings to the paragraph containing
the cursor
* **All** 

Applies the settings to the entire document

# See Also







# Text Editor Options Menu


menus: Options

* **Overstrike** 

Toggles the text entry mode that allows you
to type over existing characters.
* **Wrap To Fit** 

Toggles the text entry mode that causes lines
to automatically wrap at the edge of the window. If Wrap To Fit is
is not set on, you must press Return to end each line.
* **Status Line** 

Toggles the display of a status line at the
bottom of the window. The status line reports the line number where the
cursor is located, the total number of lines in the document,
application messages, and indicates when Overstrike mode is on.
It also provides a way to move the cursor to a specific line number.

# See Also













# Text Editor Help Menu


menus:Help

* **Overview** 

Explains how to start Text Editor.
* **Table of Contents &sigspace;** 

Provides an outline of Text Editor
help topics.
* **Tasks** 

Provides task instructions for most Text Editor
operations.
* **Reference** 

Summarizes Text Editor features, such as
menus, dialog boxes, resources, and command-line options.
* **On Item** 

Changes the cursor to a special pointer that
you can click on an item in a Text Editor window or dialog box. A
description of the item you've clicked is displayed.
* **Using Help** 

Provides help on using Help windows.
* **About Text Editor** 

Displays the version and copyright information for
Text Editor.

# See Also





# Text Editor Window


window, Text Editor

&sigspace;

* **Menu bar** 

Text Editor provides five menus: File, Edit,
Format, Options, and Help.
* **Document window&sigspace;** 

The area where you type the content of your
document.
* **Status line** 

Reports the line number where the
cursor is located, the total number of lines in the document,
application messages, and indicates when Overstrike mode is on.
It also provides a way to move the cursor to a specific line number.

# See Also







# Text Editor Open a File Dialog Box


* **Enter path or folder name&sigspace;** 

Identifies the path name of the current folder.
* **Filter** 

An asterisk (*) shows all files. You can enter
wildcard characters to display only those files that
match an extension. For example, *.doc lists
only those files with a .doc extension.
* **Files** 

Lists files located in the current folder.
* **Folders** 

Lists folders located in the current folder. You can open a file in the current folder, a subfolder, or
a different folder.
* **Enter a file name** 

Displays the file name selected in the Files list.
Press Return or click OK to open the file. Or,
you can type the name of the file you want to open.
* **OK** 

Opens the file specified in the Enter a file name field.
* **Update** 

Displays a new list of files after
changing the filter key or changing to a new folder.
* **Cancel** 

Cancels the Open operation.
* **Help** 

Describes text entry fields, selection
lists, and buttons in the dialog box.

# See Also







# Text Editor Save As Dialog Box


* **Enter path or folder name&sigspace;** 

Identifies the path name of the
current folder.
* **Filter** 

An asterisk (*) shows all files. You can enter
wildcard characters to display only those files that
match an extension. For example, *.doc lists
only those files with a .doc extension.
* **Files** 

Lists files located in the current folder.
* **Folders** 

Lists folders located in the current folder.
* **Enter file name** 

Field in which you type a new file name for your document.
* **OK** 

Saves the file using the name provided.
* **Update** 

Displays a new list of files after
changing the filter key or changing to a new folder.
* **Cancel** 

Cancels the Save As operation.


If you use Wrap To Fit, these additional choices appear in the
dialog box:

Add newline characters to the end of word-wrapped lines.

This is the default choice. It adds a newline character to the
end of each wrapped line and preserves line breaks
exactly as they appear in your document.

Do not add newlines. Only line breaks created by Return
will be preserved.

This option retains the "soft" line breaks inserted by Wrap To Fit.
If you reopen the document, the text conforms to the width of the
new window.
# See Also







# Text Editor Save Dialog Box


&sigspace;
If you use the Wrap To Fit option, the Save dialog box is displayed when
you save changes to your document. Line breaks inserted by Wrap To Fit
can be handled in two ways:

Add newline characters to the end of word-wrapped lines.

This is the default choice. It adds a newline character to the
end of each wrapped line and preserves line breaks
exactly as they appear in your document.

Do not add newlines. Only line breaks created by Return
will be preserved.

This option retains the "soft" line breaks inserted by Wrap To Fit.
If you reopen the document, the text conforms to the width of the
new window.

* **Yes** 

Saves the current file or displays the Save As
dialog box for saving your document
* **No** 

Proceeds with the operation without saving the file.
* **Cancel** 

Cancels the operation
* **Help** 

Describes the Save dialog box


The Save dialog is also displayed if you choose a menu command that
will result in the loss of your current editing changes.
# See Also







# Text Editor Include a File Dialog Box


* **Enter path or folder name&sigspace;** 

Identifies the path name of the current folder.
* **Filter** 

An asterisk (*) shows all files. You can enter
wildcard characters to display only those files that
match an extension. For example, *.doc lists
only those files with a .doc extension.
* **Files** 

Lists files located in the current folder.
* **Folders** 

Lists folders located in the current folder.

You can insert a file from the current folder, a subfolder, or
a different folder.
* **Enter a file name** 

Displays the file name selected in the Files list.
Click OK or press Return to include the file. Or,
you can type the name of the file you want to include.
* **OK** 

Inserts the file specified in the Enter file name field at the location of the cursor.
* **Update** 

Displays a new list of files after
changing the filter key or changing to a new folder.
* **Cancel** 

Cancels the Include operation.
* **Help** 

Describes text entry fields, selection
lists, and buttons in the dialog box.

# See Also







# Text Editor Spell Dialog Box




* **Misspelled Words** 

Lists the misspelled words found in the document.
* **Change To** 

Field in which to type the correctly spelled word.
* **Find** 

Finds the first occurrence of the misspelled
word. Find proceeds from the location of the insertion cursor.
* **Change** 

Replaces the highlighted word with the correct spelling.
* **Change All** 

Replaces all occurrences of the misspelled word.
* **Close** 

Closes the dialog box.


&sigspace;

The Check Spelling command is only available for English.
# See Also







# Text Editor Find/Change Dialog Box




* **Find** 

Field in which to type a word or phrase that you
want to find. Find is case sensitive.
* **Change To** 

Field in which to type the replacement text.
* **Find** 

Finds the first occurrence of the misspelled word.
* **Change** 

Replaces the highlighted word with the replacement text.
* **Change All** 

Replaces all occurrences of the search string.
* **Close** 

Closes the dialog box.

# See Also







# Text Editor Format Settings Dialog Box




&sigspace;

* **Left Margin** 

The number of characters to indent the printed text from
the left edge of the paper
* **Right Margin** 

The number of columns for the text
* **Left Align** 

Aligns text lines on the left margin
* **Right Align** 

Aligns text lines on the right margin
* **Justify** 

Aligns text in a block style with equal left and right margins
* **Center** 

Text lines are centered
* **Paragraph** 

Applies the settings to the paragraph
containing the cursor
* **All** 

Applies the settings to the entire document
* **Close** 

Closes the dialog box

# See Also







# Text Editor Copy To File Dialog Box


Other applications can use Text Editor as a tool to edit information
and may restrict how editing changes are saved. For example, in some
instances the Save As command may be replaced by Copy To File,
which enables you to create a copy of the information
you are viewing or editing without changing your edit session to the
new file.

* **Enter path or folder name&sigspace;** 

Identifies the path name of the
current folder.
* **Filter** 

An asterisk (*) shows all files. You can enter
wildcard characters to display only those files that
match an extension. For example, *.doc lists
only those files with a .doc extension.
* **Files** 

Lists files located in the current folder.
* **Folders** 

Lists folders located in the current folder.
* **Enter file name** 

Field in which you type a new file name for your document.
* **OK** 

Copies the information to the file name provided.
* **Update** 

Displays a new list of files after
changing the filter key or changing to a new folder.
* **Cancel** 

Cancels the Save As operation.


If you use Wrap To Fit, these additional choices appear in the
dialog box:

Add newline characters to the end of word-wrapped lines.

This is the default choice. It adds a newline character to the
end of each wrapped line and preserves line breaks
exactly as they appear in your document.

Do not add newlines. Only line breaks created by Return
will be preserved.

This option retains the "soft" line breaks inserted by Wrap To Fit.
If you reopen the document, the text conforms to the width of the
new window.
# See Also





# Text Editor File Exists Dialog Box


When saving a document, if you enter the name of an existing file
this dialog box is displayed.

To overwrite the original file, click OK.

To enter a different file name, click Cancel and choose
Save As from the File menu.
# See Also





# Text Editor Command-line Syntax and Options
Text Editor: starting in Terminal window

The command-line syntax for starting Text Editor is:dtpad [`options`]

Where`options`is one or more of the following:

* **-server** 

Specifies that Text Editor should be started in server
mode with no initial window displayed. Subsequent invocations of Text
Editor that run in the default requestor mode cause the server to create
a separate edit window for each request.
* **-standAlone** 

This option forces Text Editor to run in standalone
mode in which it handles its own editing independently of the Text Editor
server. This can be useful if one desires an instance of
Text Editor with an environment different from that of other Text Editor
windows. An example would be running an instance under a different locale,
or with different color resources. This is equivalent to setting
thestandAloneresource to True.
* **-exitOnLastClose** 

Specifies that the Text Editor server process
should terminate when the last edit window for the display is closed. This
option is applicable only with the-serveroption. If this option is not
specified, the Text Editor server process remains active indefinitely, even
when all edit windows have been closed.
* **-noBlocking** 

Specifies that the Text Editor
requestor terminate as soon as the Text Editor server signifies that it can
access a file in the referenced folder. In the absence of this option the
Text Editor requestor process will block, and will exit only when it
receives notification from the Text Editor server that the window has been closed.
* **-statusLine** 

This option causes Text Editor to display a status
line at the bottom of the edit window. The status line reports the line
number where the cursor is located, the total number of lines in the document,
application messages, and alerts the user when Overstrike mode is on.
It also provides a way to move the cursor to a specific line number.
* **-wrapToFit** 

Specifies that Wrap To Fit is set on at start up.

# See Also






Refer to thedtpad(1)man page for a
complete list and description of Text Editor command-line options.
# Text Editor Resources
application resourcesresources

The following resources control the appearance and behavior of
Text Editor.

Dtpad*server: [ true | false ]

Specifies that Text Editor should be started in server mode to
process all subsequent edit requests for the display. Setting this resource
to True is equivalent to specifying the-servercommand-line option.

Dtpad*standAlone: [ true | false ]

Specifies whether the current Text Editor process should run in
standalone mode, where it handles its own editing, or in requestor mode,
where the actual editing is handled by a single, separate Text Editor
server process. Setting this resource to True is
equivalent to specifying the-standAlonecommand-line option.

Dtpad*blocking: [ true | false ]

Specifies that when Text Editor is run in the default
requestor mode, where the actual editing is handled by a separate Text
Editor server process, the requestor process should not exit until the
window associated with the edit request is closed. Setting
this resource to False is equivalent to specifying the-noBlockingcommand-line option.

Dtpad*exitOnLastClose: [ true | false ]

Specifies whether the Text Editor server process should terminate when
the last active edit window is closed. If this resource is set to False, then the
Text Editor server will continue to run, ready to receive a message to edit
a file. If this resource is set to True, the Text Editor server will terminate when the last active
edit window is closed.

Dtpad*statusLine: [ true | false ]

Specifies whether Text Editor should display the status line at
the bottom of the edit window. Setting this resource to True is equivalent
to specifying the-statusLinecommand-line option.

Dtpad*wrapToFit: [ true | false ]

Specifies whether Text Editor should have Wrap To Fit enabled
(True) or
disabled (False) when the editor is started. Setting this resource to True
is equivalent to specifying the-wrapToFitcommand-line
option.
# See Also






Refer to thedtpad(1)man page for a
complete list and description of Text Editor resources.
# Text Editor Fileset
Text Editor, executableapplication defaultsresources

Text Editor's executable file and application defaults file
are:

* **Executable file** 

/usr/dt/bin/dtpad
* **Application defaults file** 

/usr/dt/app-defaults/`lang`/Dtpad

# See Also




