
# Login Manager Concepts





# Introducing Desktop Sessions
starting: Desktop sessionlogging into Desktop sessionDesktop: starting sessionsession: starting Desktopusing: home sessionhome session: definedcurrent session defined

A session is the collection of applications, settings and resources
that are present on your desktop.
A desktop session occurs between the time you log in and
the time you log out. The login screen, created by the Login Manager, is
your gateway to the desktop. It provides a place for you
to type your user ID and password.

The Options menu on the login screen lists your options.
In addition to running a desktop session, you can choose to run a failsafe session.
You can also select the language for your session.

After Login Manager authenticates your user ID and password, Session
Manager takes over.
Session management is a set of
conventions and protocols that allow a special session manager such as
the Common Desktop Environment Session Manager
to save and restore your session. You can log
into your system and be presented with the same set of running
applications, settings and resources as were present when you
logged off.
Session Manager saves and restores:

The appearance and behavior settings&emdash;for example, fonts, colors,
and mouse settings.

The window applications that were running&emdash;for example, your File
Manager and Text Editor windows. Certain types of applications cannot be
saved and restored by Session Manager. For example, if you start thevieditor from a command line in a Terminal window, Session Manager cannot
restore your editing session.

When you log into the desktop for the first time, a
default initial session is loaded. Afterward,
the Session Manager supports the notion of a current and a home session.
# Initial Session


When you log into the desktop for the first time,
the Session Manager will
generate your initial session using system default values.
By default, File Manager and an Introduction to Common Desktop Environment
are automatically started.
# Current Session


Ordinarily, the desktop saves session information when you log out and uses
that information to start your next session. If you start or stop applications
during your session, or use Style Manager to change the appearance and
behavior of your system, changes you make are reflected in your next
session.

Your running session is always considered thecurrent
session, whether restored upon login from a saved home
session, a saved current session or the system default
initial session. Based on your Style Manager Startup
settings, when you exit the session, Session
Manager automatically saves the current session. When you
next log into the desktop, your previously saved current session
is restored. This means that the desktop will be restored to same state
as it was when the you last logged out.
# Home Session


The desktop also provides ahome session. A home session is a
session that you explicitly save. It's like taking a snapshot of your
current session at some point in time. Once you've saved a home session,
you can specify that the desktop restores that session instead of
the current session the next time you log in.

See the following tasks:






# Other Ways to Log In
session: types other than Regularstarting: failsafe sessionstarting: Command Line Login mode sessionfailsafe session: startingCommand Line Login mode: starting session

In addition to the regular desktop session, there are additional types of sessions:

* **Failsafe** 

A failsafe session provides a Terminal window and Window
Manager. It is useful for executing several commands before
logging in to another desktop session.
(See.)
* **Command Line Login** 

Command Line Login lets you temporarily leave the
desktop to work in your system console.
(See.)
