
# dtterm Tasks

















# To Start dtterm


There are several ways you can start a &ProductName;dttermterminal
emulator:

From the Front Panel

From File Manager

From a command in an existing terminal window

From Application Manager

From New in thedttermWindow pulldown menu
# To Start dtterm from the Front Panel
starting: terminal emulatorclosing: terminal emulatorTerminal: buttonbutton: Terminal

The terminal control is located in the Personal Applications subpanel.

Click the terminal control. The busy light blinks indicating that
the terminal is being activated.

Thedttermwindow appears after a short time.
# To Start dtterm from File Manager


Choose Open Terminal from the File menu.

This opensdttermwith the same current directory
as the File Manager view.
# To Start a Terminal Emulator Other Than dtterm


To use a terminal emulator other thandtterm,
start it from a command line in an
existing terminal emulator window.

At the command line prompt, type the name of the terminal emulator and
any options you want. For example, to startxterm, enter:xterm  [`options`] &

* **`options`** 

represents optional items to customize the terminal emulator.
* **&** 

specifies that the terminal emulator runs in the background
so you can continue working in your
original window while the terminal emulator is
running.


The terminal emulator starts in the current workspace
unless directed otherwise by options.
# To Start dtterm from the Window Menu


Choose New from the Window menu in an existingdttermwindow.
A duplicatedttermwindow appears.
# Examples


The following command starts adttermwindow in the workspaceProject
Notes:dtterm -xrm '*workspaceList: "Project Notes"' &

The following command starts adttermwindow on a screen on the system
named "lgmcd":dtterm -display lgmcd:0.1 &
# See Also


Refer to thedtterm (1X)man page for details
about the options available fordtterm.
# To Close dtterm
closing:dttermwindow menu: button

Typeexitat the command line and press Return.

Or, Choose Exit from the Window menu.

Or, choose Close from the window
menu pulldown menu (accessed from the button at the upper left corner of
the window manager frame).

Typingexitat the command line is the preferred method of
closingdtterm. Closing it through the menus doesn't terminate
any background
processes you started fromdtterm, which can sometimes cause
problems.
# To Copy and Paste Text
cutting: textpasting: texttext: cutting and pasting
# To Copy Text


Using mouse button 1,
drag the pointer over the text you want to copy. The text appears
highlighted.

Release mouse button 1 after all the text you want
to copy is highlighted.

The text is not
removed from its original position.
# To Paste Text


Position the cursor where you want to insert the text.

Click mouse button 2.

A copy of the current selection is pasted at the location you
indicated. You can paste additional copies by repeating the above steps.
# To Resize the dtterm Window
resize window

Choose Window Size from Options.

Choose one of the three sizes:

80x24

132x24

normal

In some cases, depending on the screen size and font size you are using,
you may not be able
to resize thedttermwindow to 132 columns. If this occurs,dttermresizes to
the maximum number of columns allowable under the circumstances.

You can also resizedttermusing the Window Manager menu.
# See Also



# To Resize the Window Contents
resizefunction

When you change the size of a terminal emulator window, applications running in the
window may not know about the resizing. Use this procedure to resize the
application's output.

At the command-line prompt type:eval `resize`
# See Also



# To Start Applications in a dtterm Window


Enter the command to start the application at the command line prompt.

The general syntax for starting an application is:`application`[`options`] &

* **`application`** 

The application name.
* **`options`** 

a list of optional information to be passed to the
application.
* **&** 

specifies that the application runs in the background so
you can continue to use the terminal emulator window
while the application is running.

# Example


To start a digital clock from the command line:xclock -digital &
# See Also


Refer to the man page or other documentation for each application to
find the command and options to use for that application.
# To Log On to a Remote System

# Using rlogin


Use therlogincommand in an existing terminal emulator window to log
in to a remote host.
Once the window is acting as a terminal to the remote host, you can run
applications there, redirecting the display back to your system if you
desire.
# Example


The following command logs onto a system namedthere, runs
the clientxload, and redirects the display back to your original
system. Assume your system is namedhere.rlogin there
xload -display here:0
# Using remsh


Theremshcommand starts a shell on a remote host, performs some
client (often starting a terminal emulator on that host), and redirects the
display back to your original system if desired. (Systems that don't supportremshusually sypport the equivalentrshcommand.)
The syntax for theremshcommand is:remsh`remote`-n`client`-display`system:display[.screen]`

where:

* **`remote`** 

The remote host name
* **`client`** 

The program you want to run on the remote host
* **`system:display[.screen]`** 

The host and display on which the results
are to be displayed.

# Example


The following command runsxloadon the remote host namedthere, and directs output back to your system,here.remsh there -n /usr/bin/X11/xload -display here:0.0 &

Theremshcommand is often used when customizing a menu to access other
hosts.
# To Configure dtterm







# To Set dtterm Resources


A resource is a variable whose value affects some attribute ofdtterm. Examples of resources are foreground color, background
color, height, and width. Resources are found in a resource database.
Examples ofdttermresources are:Dtterm*saveLines:  4s
Dtterm*scrollBar: True

App-default files for the desktop applications are located in the/usr/dt/app-defaults/languagedirectory.

Resources are loaded at session start-up by Session Manager. For information
on how Session Manager loads the resources into the RESOURCE_MANAGER,
see "Loading the Session Resources" in theCDE Advanced User's & System
Administrator's Guide.
# To Set System-Wide Resources


Add the resources to the file/etc/dt/config/language/sys.resources.
(You may have to create the file.)

For example, if in/etc/dt/config/C/sys.resourcesyou specify:AnApplication*resource: value

then the resourceAnApplication*resourceis set in each user's
RESOURCE_MANAGER property at the next login.
# To Set Personal Resources


Add the resources to the fileHomeDirectory/.Xdefaults.

Double-click Reload Resources in the Desktop_Tools application group.
# To Specify Scrollbars


Specify ascrollBarresource for
the terminal emulator.
If the value ofscrollBaris True,dttermwill
have scrollbars. If the value is False, it will not have scrollbars.

Log out, then log back in for the current session. (For home session,
set home session, log out, then log back in.)
# Examples


To set scrollbars in alldttermwindows:Dtterm*scrollBar:     True

To set scrollbars only fordttermwindows named
"localTerminal":localTerminal*scrollBar:   True
# To Set Terminal Control Characters


Edit thettyModesresource using one of the methods described inSetting dtterm Resources.

The syntax for this resource is:ttyModes:`^C``name`

where`name`is the control, and`C`is the character. For example,
the default value ofttyModesis:ttyModes:	erase ^H intr ^C kill ^U start ^Q stop ^S swtch ^@

Becausedttermonlyemulatesa terminal, your
control characters might not be what you are used to on a physical
terminal. ThettyModesresource enables you to set control characters
for your terminal emulator.

By default, Login Manager sets the following control characters.

* **Control name** 

Character (Effect)
* **erase** 

^H (Backspace erases characters)
* **intr** 

^C (Interrupt - cancel the current operation and redisplay the
command line prompt)
* **kill** 

^U (Clear the input line)
* **start** 

^Q (Start subprocess output)
* **swtch** 

^@ (Switch between layers in a shell)


The "^" character stands for theCTRLkey, so to interrupt an
operation in progress you pressCTRLC. When settingttyModes, use the^key instead of theCTRLkey.