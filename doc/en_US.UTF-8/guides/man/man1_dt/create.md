# dtcreate
user cmddtcreateThe CDE Action and Datatype creation client.dtcreate<filename>
## DESCRIPTION


The CreateAction client is used to create actions and datatypes to
integrate an application into the CDE.
CreateAction provides the most commonly needed functions including the
creation of actions for an application, creation of the associated
datatypes for the application's data files, and the ability to specify
the Open and Print actions for those datatypes. The output from this
tool is a actions and datatypes definition file placed within the
users $HOME/.dt/ types directory and an action file that is placed in
the users $HOME directory. CreateAction also provides the ability to
edit a action and datatypes definition file that was created using
this tool.
### KEY SUPPORTED TASKS


Creating of actions to represent applications, commands, shell scripts, etc.

Creating of datatypes to represent an application's data files.

Creating of the Open and Print actions for the datatypes.

Creating of the actions and datatypes definition file within the
user's /HOME/.dt/types directory.

Creating the action file within the user's home directory. This file
provides the ability for the icon to be visible within the CDE's managers such as the File Manager.

Modifying of actions and datatypes definition files that were created
using this tool.

Rereading of the action and datatype database so that the actions and
datatypes are available for immediate use.
## OPTIONS


* **<filename>** 

Loads the<filename>action and datatype definition file into CreateAction for modification.

## RETURN


Exit values are:

* **0** 

Successful completion.
* **>0** 

Error condition occurred.

## EXAMPLES

### dtcreate MyAction.dt


dtcreate will load the MyAction.dt action and datatype definition file
and display using dtcreate's graphic user interface.