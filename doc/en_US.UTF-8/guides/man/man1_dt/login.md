dtloginuser cmddtloginCDE login
servicedtlogin-configconfiguration_file-daemon-debugdebug_level-errorerror_log_file-quiet-nodaemon-resourcesresource_file-serverserver_entry-udpPortport_number-sessionsession_programDESCRIPTIONKey Supported TasksThedtloginclient supports the following key tasks:Launch of dtgreet login screen for explicitly
managed local and remote displays and XDMCP managed remote displays.Access to traditional terminal (character) login
from GUI login screen.System dependent user authentication and login.Launching the selected session.Thedtloginclient provides services similar to those
provided byinit(1m),getty(1m) andlogin(1) on character terminals: prompting
for login and password, authenticating the user, and running a "session."A "session" is defined by the lifetime of a particular process; in
the traditional character-based terminal world, it is the user's login shell
process. In the DT context, it is the DT Session Manager.If the DT Session Manager is not used, the typical substitute is either
a window manager with an exit option, or a terminal emulator running a shell,
where the lifetime of the terminal emulator is the lifetime of the shell process
that it is running; thus reducing the X session to an emulation of the character-based
terminal session.When the session is terminated,dtloginresets the
X server and (optionally) restarts the whole process.Thedtloginclient supports management of remote
displays using the X Display Manager Control Protocol (XDMCP), Version 1.0.Whendtloginreceives an Indirect query via XDMCP,
it can run a chooser process to perform an XDMCP BroadcastQuery (or an XDMCP
Query to specified hosts) on behalf of the display and offer a menu of possible
hosts that offer XDMCP display management. This feature is useful with X
terminals that do not offer a host menu themselves.Becausedtloginprovides the first interface that
users see, it is designed to be simple to use and easy to customize to the
needs of a particular site.Login WindowThe Login window allows the user to enter a user name and password, specify
a locale, and select a startup session. A user may also reset the login screen
or suspend the X server to access the character login prompt.Contents of Login window:login fieldEntry field to enter user name.password fieldEntry field to enter user password (no-echo).OKAuthenticate user and start a session.ClearClear login and password field.OptionsDisplay menu for language (locale), session, command line login
(suspend the X server), and login screen reset.HelpDisplay help message.Login Window -- Options MenuThe Options menu allows the user to specify a locale for the startup session,
select a startup session, suspend the X server to access the character login
prompt, or reset the login screen.Contents of Options Menu:LanguagesShow Language menu.SessionsShow Sessions menu.Command Line LoginDisplay character login prompt (local displays only). Enabled whendtloginis started by the system at boot time (seedtconfigman page). Disabled ifdtloginis started manually from a command line
"root" login session. This type of manual login is performed for
security reasons since it places the user in a running "root" session.Reset Login ScreenRestart X Server and return to login screen.Login Window -- Language MenuThe Language menu displays the languages (locales)
available in the CDE environment. Selecting a language
sets theLANGenvironment variable to the selected language and
restarts the login screen in that language.
Login screen
localization andLANGreturn to the default value upon conclusion of the session.The contents of this menu can vary depending upon the locales installed
on the system and can be overridden by using thelanguageListresource. The default locale of C can be
overridden using thelanguageresource.The system orlanguageListlocales specified are
displayed as menu items in the Languages menu. Alternate text to be displayed
may be specified for a given locale name by using thelanguageNameresource.Login Window -- Sessions MenuThe Sessions menu allows the user to select which
of the following sessions to start:CurrentStart the user's most recent session.HomeStart the user's home session.display-name- CurrentCreate a new display-specific session and start the first of the following
sessions that exists:display-specific Homegeneric Homenew user (initial) sessiondisplay-name- HomeCreate a new display-specific session and start the user's generic home
session if it exists. Otherwise, start a new user session.Fail-safe SessionStart a fail-safe session (Xfailsafe)Although a user's list of sessions is not known until after the user
logs in, the dialog presents all of the session choices. If a user selects
a session that does not exist, the Session Manager (dtsession)
takes the following
actions. If the user selects:HomeThe Session Manager
starts a new user session.CurrentThe Session Manager
starts the user's home session if it exists.
If it does not, it starts a new user session.If the user selects a display-specific session and one does not exist,
the Session Manager posts a warning dialog stating that a new session will
be created. The warning dialog contains three buttons:Cancel LoginCancels the login and returns the user to the login screen.`OK`If a display-specific Home session was selected,
the Session Manager
creates a new display-specific session and starts the user's generic
home session if it exists. If it does not exist, it starts a new user session.If a display-specific Current session was selected,
the Session Manager
creates a new display-specific