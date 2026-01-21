
# Application Manager Tasks

# Running Applications







# Making Applications Easier to Find on the Desktop







# Configuring the Application Manager







# Advanced Application Manager Information





# Other General Tasks


The Application Manager window is used very much like a File Manager
window. These topics in File Manager help might be useful.

To Select a Single File or Folder

To Drag and Drop a File or Folder

To Move a File or Folder

To Copy a File or Folder

To Change Basic Viewing Preferences

For additional information about using File Manager:File Manager Tasks.
# To Start an Application
starting an applicationapplications:startingApplication Manager:opening

Open Application Manager by clicking the Application Manager control in
the Front Panel.

Open the application group containing the application by double-clicking
its icon.

application groupApplication group icons are special folders that behave just like folder
icons in File Manager.

The application group contains icons representing the application
and things associated with the application, such as README files and
templates.

application iconsaction iconsThe icon that starts the application is called
theapplication iconoraction icon.

Double-click the application's action icon.

In some cases, you may also be able to start the application by
dropping a data file on the action icon.
# Starting Applications from Data Files
data files:starting applications fromOpen action

Some applications have been configured to open when you:

Double-click a data file icon in File Manager.

Or, select the data file in File Manager and choose Open from the
Selected menu.
# To Get Help on an Application Icon
action icons:help onapplication icons:help onon item help:on action icons

Choose Help from the icon's pop-up menu. To display the pop-up menu,
point to the icon and press mouse button 3.

Or, choose On Item from the Help menu and then click the icon.
# To Put an Application Icon in the Front Panel
Front Panel:copying application icon intoaction icons:copying to Front Panel

Display the application's action icon in Application Manager.

Display the subpanel to which you want to add the application.

Drag the action icon from Application Manager to the Install Ojbect control
in the subpanel.
# To Put an Application Group in the Front Panel
Front Panel:copying an application group intoapplication group:copying to Front Panel

Open Application Manager.

Display the subpanel to which you want to add the application group.

Drag an application group icon from Application Manager to the
Install Icon control in the subpanel.
# To Put an Application Icon on the Workspace Backdrop


Display the application's icon in Application Manager.

Drag the application's icon to the desktop backdrop.
# To Update Application Manager
Application Manager:updatingreloading applicationsapplications:reloading

Application Manager is created the first time you log in, and
is rebuilt each time you log in. Each time it is built, it searches
certain system and network locations for applications and places them in
Application Manager.

If your system administrator adds an application, you can update
your Application Manager:

Log out and back in.

Or, open the Desktop_Tools application group and
double-click Reload Applications.
# To Create a Personal Application Group
application group:personalApplication Manager:modifying

A personal application group is a group that you alone own.
You create the group yourself and have permission to modify it.

Create a directory:`HomeDirectory`/.dt/appmanager/`app_group_name`

whereapp_group_nameis the name of the application group as it
will appear in Application Manager.

Open the Desktop_Tools application group and double-click
Reload Applications.

You should now see your new application group at the top level of
Application Manager.

You can treat your personal application group like any other directory
to which you have write permission. You can move or copy files to it, create
subdirectories, and delete unwanted files.
# Related Topics



# To Add an Application to a Personal Application Group
Application Manager:modifyingapplications:adding

The following procedure:

Creates an action for your application.

An action is a desktop
construct that behaves like a macro--it automates tasks usually
done with command lines. The desktop uses actions to connect command
lines with icons.

Creates an action icon representing the application in the application group.

If necessary, create an application group to which you have
write permission. See.

Determine the command line you would use to start the application from
a terminal emulator window.

Create Action

Use the desktop application Create Action to create the action.
SeeTo Create an Action With Create Action.

Create Action requires you to enter an action name. The name you supply
will become the label on the action.

Create Action creates an action icon in your home folder. The icon
consists of an executable file with the same name as the action.

Copy or move the action icon created by Create Action from your
home folder to the personal application group.
# To Register Applications in Application Manager
applications:registeringregistering applicationsApplication Manager:modifying

When an application is registered in Application Manager, it has its own
application group. This application group is available to all users
on the system.

Registering applications is a system administration task. For
instructions, see theCommon Desktop Environment Advanced User's and System Administrator's
Guide.