
# Mailer Messages


This section describes possible causes and solutions for
Mailer error messages.error messagesmessages, error
# Mailer is confused about the state of this mailbox.


Close the mailbox by choosing OK and then restart Mailer.

Something has caused your mailbox to be in an unknown state in the
system. This could occur if you did a save in another instance of
Mailer or Mail while this Mailer was running.
# Possible Solution:


Try closing Mailer and restarting it.

If that doesn't work, contact your system administrator.
# attachment nameis an executable attachment.


Do you want to run it?

The selected attachment is an executable file. You can either run
the executable or cancel the operation.
# Possible Solution:


If you weren't expecting to run the executable, you can
save the attachment to a file and run it later.
# You are already using the forwarding facility


for something other than Vacation. While Vacation is running,Vacation will be appended to this other forwarding activity.Is it still OK to start Vacation?

When the Vacation option is activated in Mailer, it uses a forwarding
facility to save your messages and reply to people who send messages to you.
If you use this forwarding facility for something else, like forwarding
your mail to another user account,
the Vacation option is appended to the .forward file and
both features take effect.
# Possible Solution:


Remove the forwarding facility in the .forward file if you want
only the Vacation option to take effect.

Turn Vacation off if you want only the forwarding facility
to take effect.
# You are already running the vacation program


in your .forward file. Consult documentation on how to stop itand remove it from your .forward file. Try this command afterfixing that problem.

You tried to start the Vacation facility when you already had it running.
# Possible Solution:


Only run one instance of Vacation.
# Cannot open .vacation.msg file -- No write permission.


The file that contains your vacation information message,
usually located in your home directory,
is currently a read-only file.
# Possible Solution:


Change file permissions so you have write permission, then
try to create and save a vacation message.
# vacation.msg file exists. Replace with new text?


You currently have vacation message text in your .vacation.msg file.
Do you want to overwrite it?
# Possible Solution:


You can overwrite the existing text with the new message or you
can choose to cancel, saving the existing vacation message.
# Unable to create a Compose window.


When you selected New Message from the Compose menu, Mailer
was unable to create a Compose window.
# Possible Solution:


Try again, and if it doesn't work, restart Mailer.
# The template does not exist.


The template you have selected for use is not in the
location specified in the Mailer Templates Options dialog box.
# Possible Solution:


Make sure the Template Options dialog box
specifies the correct location (full path name) of the template.
# The template appears to be corrupt.


The template file is in the specified location but something is
wrong with it.
# Possible Solution:


Make sure the template file is not corrupt. Replace it with
a backup file if possible and try again to insert the template.
# There is not enough memory to load the template.


The template you are attempting to use in Mailer exceeds the amount
of memory you have available.
# Possible Solution:


Close some applications on the desktop and try again to load
the template.
# ToolTalk is not initialized.


Mailer cannot run without ToolTalk. Try starting/usr/dt/bin/dtsession, or contact your system administrator.

While attempting to perform a function, ToolTalk could not be
initialized.
# Possible Solution:


Check with your system administrator about whether
your system is working correctly with ToolTalk.
# Mailer has not been properly installed and


cannot run because the execution group is incorrectly set.

On some systems, Mailer must be run with the group id (execution group)
set to "mail". Your execution group has not been set so that
Mailer can run.
# Possible Solution:


Check with your system administrator to set the group permission
correctly on the Mailer executable.
# You do not have permission to viewfile name.


You do not have permission to view the specified mailbox.
The file permissions may be set so you don't have read permission.
# Possible Solution:


If you own the file, change the permissions so you can read it.

If you don't own the file, ask the file's owner to change the
permissions so you can read it.
# The mailbox is a directory and cannot be opened.


The selected mailbox name is a directory (folder) and cannot be opened.
# Possible Solution:


Indicate a mailbox name that is a file.
# An attachment needs to be selected before issuing


the Save As command to save to a file.

You selected Save As without first selecting an attachment.
# Possible Solution:


Select an attachment, then select Save As.
# The mailbox is locked. You can manually unlock the


mailbox and try again or contact your system administrator.

Someone else may currently be accessing this mailbox or you may have this
mailbox open elsewhere.
# Possible Solution:


If you would still like to access the mailbox, choose OK so that
Mailer will request the lock on this mailbox to be released. If the
request is successful, you will have access to the mailbox.

Choose Cancel to end the attempt to open this mailbox.
# Destroy the messages you have marked for


deletion in this mailbox?

When you try to close a mailbox, if you have deleted messages,
you are asked if you want to destroy the messages marked for deletion.
# Possible Solution:


Click Cancel if you do not want to permanently delete the messages.

Mailer closes after saving changes to the mailbox.
# Unable to overwritefile name. Check file permissions


and retry.

The selected file is not writeable.
# Possible Solution:


Check file permissions and change them if necessary
before trying the operation again.
# Unable to openattachment name.


The specified attachment cannot be opened. You may not have read
permission for the file.
# Possible Solution:


Check the attachment file permissions and change them if necessary
before trying the operation again.
# Unable to allocate memory.


There is not enough memory for the selected operation.
# Possible Solution:


Try closing some applications and trying again.
# file namealready exists. Replace?


You have tried to save an attachment to a file name that already
exists in your file system.
# Possible Solution:


If you do not want to overwrite the existing file, cancel,
and type a different file name.
# Unable to replacefile name.


The file name you specified cannot be overwritten.
This may be because the directory is not writeable or
because you are trying to write to a read-only file system.
# Possible Solution:


Check the directory permissions and add write permission if
possible.

Specify another (writeable) location to save the message.
# Unable to createfile name.


The mailbox name you specified cannot be created.
This may be because the directory is not writeable or
because you are trying to write to a read-only file system.
# Possible Solution:


Check the directory permissions and add write
permission if possible.

Specify another (writeable) location to save the message.
# The name already exists. Overwrite?


You have indicated an attachment that already exists in your file system.
# Possible Solution:


If you do not want to overwrite the existing attachment, cancel,
and type a different name for the attachment.
# file namealready exists. Overwrite?


You have indicated a mailbox that already exists in your file system.
# Possible Solution:


If you do not want to overwrite the existing mailbox, cancel,
and type a different name for the mailbox.
# Unable to write tofile name.


You tried a Save As Text operation, but you don't have write permission
for the file.
# Possible Solution:


Check the file permissions and add write
permission if possible.

Specify another (writeable) file name to save the message.
# The Compose window contains text or


attachments that will be lost if the window is closed.Close the Compose window?

You tried to close the Compose window while there was text in it.
# Possible Solution:


Choose OK to close the Compose window and any text you
typed will not be saved.

Choose Cancel to return to the Compose window.
# There is not enough memory to load the


existing .vacation.msg file.

Your current .vacation.msg file is too large.
# Possible Solution:


Close some applications and try again.
# The existing .vacation.msg file appears to be corrupt.


The file is in the correct location but something is wrong with it.
# Possible Solution:


Check the file (usually in your home directory) to make sure
it is not damaged. If it is, replace it with a new file.
# You have an attachment open that may have


unsaved changes. Sending this message will break theconnection to the open attachment. Any unsaved changeswill not be part of the message. You can use Save As tosave changes after the connection is broken, but the changeswill not be part of the attachment.

You tried to send a message without saving changes in some open
attachments. This warning makes sure that is what you intended.
# Possible Solution:


Save the changes to the attachments before sending.
# Select only one attachment and then choose rename.


You chose Rename when you had more than one attachment selected.
# Possible Solution:


Select only one attachment before choosing rename.
# Some of the addresses in the message are


incorrect, and do not refer to any known users in the system.Please make sure all of the addresses are valid and try again.

You may have typed an incorrect user name or a name that the system
doesn't recognize.
# Possible Solution:


Check all of the user names for accuracy, make corrections if
necessary, and resend the message.
# Mailer does not have enough memory available to


send this message. Try closing other applications and resend this message.

Mailer ran out of memory.
# Possible Solution:


Try closing some other applications to free up some memory.
# An error occurred while trying to send your message.


Check to make sure the message was received. If not, you mayhave to resend this message.

There was an error from the mail transport (sendmail).
# Possible Solution:


Contact the intended recipient by some means other than mail
and ask them if they received your message. If not, you may have to
resend.
# Exit Mail?


It appears that mail is installed incorrectly. You will have to
exit mail and make some changes.
# Possible Solution:


You will need to exit and change your permissions for Mailer.
Your system administrator should help you by becoming root
and changing permissions in /usr/dt/bin/dtmail.
# Fatal Error.


Mailer has come across a fatal (unrecoverable) error.
# Possible Solution:


Check with your system administrator.
# Recoverable error.


Mailer has encountered a recoverable error.
# Possible Solution:


Follow the instructions in the error dialog.

Or

Check with your system administrator.
# Insufficient Memory for Requested Operation

# Possible Solutions:


Be sure that you are not running too many applications.

Close some applications if possible in order to free up some system memory.

Increase swap space. If you do not know how to increase swap
space, contact your system administrator.

SeeTo Open and Close an Application Window