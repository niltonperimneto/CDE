
# Login Manager Tasks

# Starting and Ending a Desktop Session







# Alternative Ways to Log In







# To Customize Session Startup and Logout






To Determine How Your Next Session Starts

To Save a Session in Progress

To Set Your Logout Confirmation Preference






# To Log In to a Desktop Session
logging in to a desktop session

Type your user ID and
press Return or click OK.

Type your password and press Return
or click OK.

If Login Manager does not recognize your user ID or password,
try again.

When you login, Session Manager starts a session:

If this is the first time you have logged in, you start a new session.

If you have logged in before, your previous session is restored.
# To Log Out of a Desktop Session
logging out of desktop sessionsession:endingFront Panel:exit controlWorkspace menu:logout command

Click the exit control in the Front Panel.

Or,choose Log out from the Workspace menu (displayed by pressing
mouse button 3).

When you log out of a regular desktop session, Session Manager
saves information about your current session so that information can be
restored the next time you log in.
# To Use a Session in a Different Language
language: differentsession:in different languages

Choose Language from the Options menu on the login screen.

Choose the language group that includes the language you need.

From the menu of languages in the group, choose the language
you need.
When you choose the language, the login screen will be
redisplayed in the selected language.

Log in.

The default language for your system is set by your system administrator.
The Options menu enables you to access other languages.
Choosing a language in the Language menu sets the LANG environment variable
for your session. The default language is restored at the end of the session.
# To Log In to and Out of a Failsafe Session
logging into and out of failsafe sessionfailsafe session: logging in and outterminal emulator:in failsafe session

A failsafe session is a simple session that
optionally starts a Window Manager and
a single Terminal window.
It is useful when you need access to a single Terminal window to
execute several commands before logging in to a desktop session.
# To Log In


On the login screen, Choose Session from the Options menu.

Choose Failsafe Session from the Session submenu.

Log in.
# To Log Out


Type theexitcommand in the Terminal window.
# To Enter Command Line Login Mode
X server: stopping temporarilyusing: Command Line Login modeCommand Line Login mode

On the login screen, choose Command Line Login from the Options menu.
The login screen disappears and is replaced by a console prompt.

Supply your user ID and password as prompted.

Command Line Login mode is not a desktop session. When your system is
in Command Line Login mode, the desktop is suspended. You log in using your
operating system mechanism rather than Login Manager. There are no
windows because the X server is not running.

Certain types of configurations (for example, Xterminals) do
not provide a Command Line Login mode option.
# To Leave Command Line Login Mode
using: Command Line Login modeCommand Line Login mode

Typeexitfrom the command-line prompt.
# To Save a Home Session
saving: home sessionhome session: saving

Click the Style Manager control in the Front Panel.

Click the Startup control in Style Manager.

Click Set Home session in the Startup dialog box.

Click OK in the confirmation dialog box that appears.

Click OK.

This saves the current state of your session.
# See Also


To Determine How Your Next Session Starts

To Save a Session in Progress

To Set Your Logout Confirmation Preference




# To Automatically Start the Home Session at Login
starting: home session at loginhome session: automatically starting at login

Click the Style Manager control in the Front Panel.

Click the Startup control in Style Manager. The Startup dialog box is displayed.

Select Return to Home session.

Click OK.

When you choose Return to Home session, Session Manager does`not`save
your current session at logout.
# See Also


.

.
# To Choose Which Session to Start
choosing: between current and home sessioncurrent and home session: choosing betweenhome and current session: choosing between

On the Login screen, click the Options button.sessiondetermining at login

Click Session. The Session menu lists the available sessions:CurrentStarts your most recent session.HomeStarts your home session (if you set one).display-name- CurrentStarts the current session for your display, if one exists.
Otherwise, a new current session for your display
will be created and the first one of the following sessions
that exists will be started: a home session for your display,
the generic home session, or a new user session.display-name- HomeStarts the home session for your display, if one exists.
Otherwise, a new session specific to your display will be
created and your generic home session (if it
exists) or a new user session will be started.Failsafe SessionStarts a failsafe session.Click on the session you want to start.
# See Also


.

.
# To Set Personal Environment Variables
setting: personal environment variablespersonal environment variables: settingenvironment variables: setting personalvariables, environment: setting personal

Personal environment variables can be set in the script fileHomeDirectory/.dtprofile.

Edit`HomeDirectory`/.dtprofile

Add lines to the file to set the environment variable

The desktop will accept either sh or ksh syntax for the commands in this
file. The commands should only be those that set environment
variables, not any that perform terminal I/O, ex. "tset" or "stty".

The files/etc/profileandHomeDirectory/.profileare
not read by the desktop as they may contain terminal I/O based commands
inappropriate for a graphical interface.

The desktop automatically
sets the following environment variables for each user:

* **DISPLAY** 

set to the value of the first field in the Xservers file
* **EDITOR** 

set to the desktop default editor
* **ENV** 

set to "HomeDirectory/.kshrc"
* **HOME** 

set to the user's home directory (from /etc/passwd)
* **KBD_LANG** 

set to the value of $LANG for some languages (see Xsession)
* **LANG** 

set to the display's current NLS language (if any)
* **LC_ALL, LC_MESSAGES** 

set to the value of $LANG
* **LOGNAME** 

set to the user name
* **MAIL** 

set to "/var/mail/$USER"
* **PATH** 

set to the value of the Dtlogin "userPath" resource
* **USER** 

set to the user name
* **SHELL** 

set to the user's default shell (from /etc/passwd)
* **TERM** 

set to xterm
* **TZ** 

set to the value of the Dtlogin "timeZone" resource

# See Also


.

Dtlogin(1X) man page for details on setting environment variables.
# To Use Your Existing .profile or .login File
using: shell environment file with.dtprofileshell environment file, using with.dtprofileenvironment file, shell, using with.dtprofile

If you have an existing shell environment file (.profileor.login),
this procedure lets you continue to use that file. This will avoid duplicate
variable assigments between.dtprofileand the shell environment file.
With minor editing, you can adaptHomeDirectory/.profile (or .login)for use both with and without the desktop.

Edit the.profile or .loginto provide two sections:

One section contains commands that do not apply to the desktop (for
example, commands that require terminal I/O, or variables for which
you want to replicate the desktop default values).
Enclose them with an "if" statement that checks for the setting of
the "DT" environment variable.

The other section contains variables that apply, whether or not the desktop
is run.

Modify.dtprofileto source in the shell
environment file by uncommenting the following line at the
bottom on the.dtprofilefile.DTSOURCEPROFILE=true

Log in again

The following sample shows how to separate it into a non-desktop
section and a section for variables that apply to both environments.
# example for sh/ksh
#
  #  commands and environment variables used when logging in without 
  #  the desktop
  #
  if [ ! "$DT" ]; then
    stty ...
    tset ...
    DISPLAY=mydisplay:0
    MAIL=/var/mail/$USER
    EDITOR=/bin/vi
	        ...
  fi	    

  #
  # environment variables common to both desktop and non-desktop 
  #
  PATH=$HOME/bin:$PATH
       ...
# example for csh
#
  #  commands and environment variables used when logging in without 
  #  the desktop
  #
  if (! ${?DT}) then
     stty ...
     tset ...
     setenv DISPLAY mydisplay:0
     setenv MAIL /var/mail/$USER
     setenv EDITOR /bin/vi
         ...
  fi
  #
  # environment variables common to both desktop and non-desktop
  #
  setenv PATH $HOME/bin:$PATH

Errors in .dtprofile or .profile (.login) may prevent a successful
login. If so, log in via the failsafe session and correct the error.

If a terminal emulator is started with the-lsoption,.loginor.profilewill be read automatically.