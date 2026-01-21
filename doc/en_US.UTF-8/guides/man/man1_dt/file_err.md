# dtfile_error
user cmddtfile_errorthe CDE File Manager error-dialog
scriptdtfile_error error_message
## DESCRIPTION


This script can be used by applications to display an error dialog when
it would be difficult or impossible to do in the context of the
executing program. For example, it can be used when exec fails in a
child process or if an error is detected before an application's main
window can be realized. It can also be used from a shell script to
display an error dialog.

This script is used by File Manager to display an error dialog when
exec fails within a child process.
## EXAMPLES

### dtfile_error You did something wrong


Executed from a command line, this displays an error dialog. The dialog
consists of the message text, "You did something wrong", and an OK
button. Clicking on OK dismisses the dialog.
### execl(dtfile_error, dtfile_error, s, NULL);


Executed from within a program, this displays an error dialog. The
dialog consists of the message text in the string s and an OK button.
Clicking on OK dismisses the dialog.