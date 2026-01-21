dtmailuser cmddtmailthe desktop
mailerdtmail-h-c-fmailfile-afile1...fileNDESCRIPTIONTheDtmailprogram is a mailer for use on the desktop.
It provides an easy-to-use interface for viewing, filing, composing and
sending electronic mail containers and mail messages.The Mailer provides a GUI-based interface for manipulating electronic
mail messages that can have attachments. Use the interface to compose a
message, view a message or a container holding messages, load new mail, copy
or move messages from one container to another, delete messages, reply to
messages, add and delete attachments to a message when composing, and view
contents of attachments in a message. The Mailer also provides a mail-pervasive
desktop environment by providing a public Tooltalk API. Other clients can
use the Tooltalk API to compose and send messages.OPTIONS-a file1 ... fileNBring up a Compose window with file1 through fileN
as attachments.-cBring up an empty Compose window-f filenameThis specifies the mail file to be loaded in at start up time. Ordinarily,
the mailfile pointed to by the environment variable MAIL is read in as the
user's inbox. Use of this option overrides the use of the MAIL variable.-hDisplay help for command line optionsRESOURCESThe Mailer provides the following resources:Dtmail*Message_List*doubleClickIntervalThe double click time out (in milliseconds) for the scrolling message
header list. Default is 400 milliseconds.Dtmail*Message_List*backgroundColor to use for the scrolling message header list background. Default
is system dependent.Dtmail*Message_List*foregroundColor to use for the scrolling message header list foreground. Default
is system dependent.Dtmail*Message_List*fontListThe list of fonts to use in the scrolling message header list. The
list must contain two fonts. The first must be tagged "plain" and is the
font used to render the header text. The second must be tagged "attach" and
is used to render the attachment indicator. Default is to use system dependent
fixed width fonts.Dtmail*Work_Area*Text*backgroundColor to use for the View and Compose window text background. Default
is system dependent.Dtmail*Work_Area*Text*foregroundColor to use for the View and Compose window text foreground. Default
is system dependent.Dtmail*Work_Area*Text*fontListThe list of fonts to use in the View and Compose windows. Font tag
"plain". Default is to use a system dependent variable width font.MAIL VARIABLESIn addition to the variables recognized bymailx(1),dtmailrecognizes those listed below. They can be set by editing
your.mailrcfile; however, since most of the variables
are accessible through the Mailer Options menu, we strongly recommend that
you modify them there to reduce the chance of error. Unless otherwise noted,
the default for the following variables isoff.additionalfieldsA list of header fields to access via theFormatmenu. This variable can be accessed through theCustom Fields:,Header Field:, andDefault Value:portions of the Compose Window
category in the Mail Options dialog.bellThe number of times to ring the bell when new mail arrives. This variable
can be accessed through theSignalNew Mailportion in the Message
Header List category of the Mail Options dialog. The default is 0.composeintervalThe interval in seconds for checkpointing to dead.letter. Default is
every 600 seconds (10 minutes).dontlogmessagesThis variable controls whether or not theLog Messageitem is selected in the File Menu in the Compose window.
The default is to log messages. This variable can be accessed through theLog all sent messagesitem in the Message Filing category of the Mail Options dialog.expertSet expert mode in which minimal confirmations are requested. This variable
can be accessed through theShow confirmation noticescheck box in the Advanced category of
the Mail Options dialog.filemenu2A list of files from which to initialize theMove,
andCopy Tomenus. These can be absolute
pathnames or pathnames relative to the directory specified in thefoldervariable. This variable can be accessed through theMove MenuandCopy To Menu:scrolling list in
the Message Filing category of the Mail Options dialog.filemenusizeSpecifies the maximum number of entries in theMove,
andCopy Tomenus. The default is 10.flashThe number of times to flash the window or icon when new mail arrives.
This variable can be accessed through the Signal New Mail portion of the
Message Header List category of the Mail Options Dialog. The default is 0.folderThe directory for saving mail files. This variable can be accessed
through theStart Looking Initem in the Mail Filing category of the Mail Options Dialog.headerlinesThe number of lines to display at a time in the scrolling header list.
This variable can be accessed through theDisplayitem in the Message View category of the Mail Options dialog. The default
is 15.hideattachmentsHide the attachments pane in the Compose Message window by default.
This variable can be accessed through theShow Attachment Listitem in the Compose Window category
of the Mail Options dialog. The default is to show the attachment pane.indentprefixWhen indentprefix is set, the string that it is set to is used to mark
indented lines from included messages. The default indentprefix is "> ". This
variable can be accessed through theIndent Stringitem in the Compose Window category of
the Mail Options dialog.keepdeletedDon't purge the mailbox of deleted messages when closing (exiting) dtmail.
Default is to ask the user if they would like to purge the mailbox on exit.
This variable can be set in theDestroy Deleted Messagesportion in the Message Header List
category of the Mail Options dialog. SeequietdeletequietdeleteDon't ask for confirmation when purging the mailbox of deleted messages
when exiting dtmail. This variable can be set in theDestroy Deleted Messagesportion in the Message Header
List category of the Mail Options dialog. SeekeepdeletedrecordThe mail file in which to record outgoing messages. You can control
recording of outgoing mail on a per message basis by theLog Messageitem in the Compose window's File menu. Thedontlogmessagesvariable controls whether or not this item is selected by default.
Therecordvariable may be set through theMailbox for Sent Messages:item in the Message Filing category of the Mail Options
Dialog. Ifrecordis not set and the user chooses to
log a message then the message will be saved in ~/sent.mail.retrieveintervalThe interval in seconds to check for new mail. This variable can be
accessed through theCheck for New Mail Every:item in the Message Header List category of the Mail Options Dialog. The default
is 300 seconds.saveintervalThe interval (in seconds) at which to checkpoint the state of the mail
box to disk. Default is 1800 seconds (30 minutes). This variable can be set
using theUpdate Mailbox Stateitem in the Advanced category of the Mail Options dialog.showmsgnumShow message numbers in the scrolling list of message headers. This
variable can be set using theDisplay message numbersitem in the Message Header List
category of the Mail Options dialog. Default is to not show message numbers.showtoShow the "To" field of mail messages in the Header Window
if the mail is from the same user that is reading mail (eg. you). This variable
is accessed using theShow To: recipient when mail isitem in the Message Header List category of the Mail Options
dialog.strictmimeUse strict MIME character encoding for outgoing mail. In this case all
lines longer than 72 characters are broken with a newline (and a trailing
"=" is appended to the line), and all trailing spaces are encoded (appearing
as "=20"). Any time character encoding takes place all "=" must be protected
and are therefore are encoded as "=3d". Note that 8 bit characters are always
encoded, even if strictmime is not specified. If you typically send mail to
users of non MIME compliant readers you may want to consider not specifying
strictmime. This variable can be set by selecting theUse strict MIME character encodingitem in the Advanced category of the Mail Options dialog.
The default is to use more relaxed character encoding (ie don't break long
lines and don't protect trailing spaces).templatesA list ofname:pathpairs to access via theTemplatesitem in theFormatmenu of the Compose
window.nameappears in the menu;pathis the file included when name is selected. This variable
can be accessed in the Template category of the Mail Options dialog.toolcolsDefault width of the primary windows (in columns). This variable can
be accessed through the item in the Message View category of the Mail Options
dialog.MAIL COMMANDSIn addition to the commands recognized bymailx(1)
in the.mailrcfile,dtmailalso recognizes
the following commands.ignore [header-field...]Suppress displaying of the specified header fields. Examples of header
fields to ignore are Status and Received. The fields are also ignored when
the message is printed. This variable can be accessed through the Abbreviated
Header item in the Message View category of the Mail Options dialog.PRINTINGYou can print messages using command invocation by selecting the
message or messages to be printed and then activating thePrint...command in theMessagepulldown menu in thedtmailmenu bar or thePrint...command in theMailer - Messagespopup menu that is displayed onBMenu Downevents in the message list.In addition, you can use thePrintcommand button
located at the bottom of the message headers list to print the currently
selected messages. In this case, the print job is started using the print
setup context from the last print command without displaying any of the print
setup dialogs.Alternatively, you can print messages using Drag and Drop invocation.Messages containing attachments are printed with summary lines in place
of the attachment. You must print attachments individually in separate
print job invocations.You can choose to print multiple messages either as a single print
job or as separate print jobs. If you print multiple messages in
a single print job, you can choose to separate the messages using
a blank line or a page break.To print a mailboxes, use CDE Drag and Drop to drag the icon for the
mailbox from the File Manager to the printer icon in
the desktop.When you invoke printing, whether by command invocation or by
drag and drop,dtmaildisplays a Print Setup window that
allows you to set a number of generic and printer-specific printing
options. For example, you can send the output to a file
or a printer. In the case of printed output, you can specify how many copies
you want. You can also access another window to set options specific to
the printer/spooler you are using. For example, you can select paper size,
orientation, a banner page title, one- or two-sided printing, and email notification
on completion of the print job.ENVIRONMENT VARIABLESThe following are environment variables taken from the execution environment
and are not alterable withindtmail.HOME= directoryThe user's home directory.MAIL= filenameThe name of the initial mailbox file to read (in lieu of the standard
system mailbox). The default is system dependent.
See FILE section.MAILRC= filenameThe name of the start-up file.
Default is
$HOME/.mailrc.FILES/var/mail/* (Sun)/var/spool/mail/* (IBM)/usr/mail* (HP)System mailboxes/etc/mail/mailx.rcSystem setup file that is read in before ~/.mailrc.~/.mailrcStart-up file formailanddtmail./usr/dt/bin/dtmailExecutable for the desktop Mailer./usr/dt/app-defaults/<LANG>/DtmailApplication defaults for the desktop Mailer.