# dtprintinfo
user cmddtprintinfothe CDE Print Viewerdtprintinfo-p <printer>-all-populate-help
## DESCRIPTION


The Print Viewer program provides a graphical interface that
displays the status of print queues and print jobs.
Additional information about print queues or print
jobs can be retrieved within the interface, individual print
queue labels and icons can be customized, and individual
print jobs can be canceled. Print jobs can be submitted by
dragging files, text, or attachments from other CDE programs
and dropping them on a print queue icon in a Print Viewer window.
### KEY SUPPORTED TASKS


Displaying configured print queues on the system.

Displaying print queue and print job properties.

Changing print queue labels and icons.

Submitting print jobs.

Canceling print jobs.
## NO OPTIONS


When the Print Viewer is started without any options,
it shows the LPDEST printer or the system default printer if
LPDEST is not set.
## OPTIONS

### -p<printer>


Display information only for the specified<printer>.
### -all


Display information for all printers.
### -populate


The-populateoption is available only for root. This
option creates default printer actions for all printers
in /etc/dt/appconfig/types/LANG. If LANG is not defined,
the actions are created in /etc/dt/appconfig/types/C.
The post-installation process uses the-populatefeature to create default printer actions in /etc/dt/appconfig/types/C.
### -help


Display command line option help.
### -session<session_file>


The Print Viewer creates a session file when you exit the desktop. When you
start the desktop again, the Print Viewer uses the session file specified
in the session_file parameter to restore the view settings, which printers are
shown and which printers are opened.
## RESOURCES


The Print Viewer does not defined any application specific resources.
## ENVIRONMENT

### LPDEST


This environment variable determines which print queue to display
when the Print Viewer is started without any options.
### XMICONSEARCHPATH


This environment variable determines which icon folders to
search when customizing a printer's icon set.
## BROADCAST MESSAGES


The Print Viewer responds to ReloadActions messages and sends
a ReloadActions message when you customize a printer's icon name
or icon set.
## WARNING MESSAGES


- Are you sure you want to cancel <job>? -

The Print Viewer displays a prompt before a print <job> is canceled.
## EXIT STATUS


The following exit values are returned:

* **0** 

Successful completion.
* **>0** 

An invocation error was detected.

## EXAMPLES

### dtprintinfo


Display the status of the LPDEST printer or the system default printer.
### dtprintinfo -p lp0


Display the status of the print queue calledlp0.
### dtprintinfo -all


Display the status of all print queues.
## RELATED INFORMATION

### CDE Print Specific Actions


Two print specific actions are created for each printer on the system.
These actions are defined in <printer>.dt files located in
/etc/dt/appconfig/types/$LANG.
### <printer>_Print


This action displays the status of the print queue specified by the<printer>parameter.
### <printer>_Print <file>


This action invokes the Print action for the<file>on the printer
specified by the<printer>parameter.
## FILES

### dtprintinfo


Executable file. This file is located in /usr/dt/bin.
### Dtprintinfo


Application defaults file. This file is located in /usr/dt/app-defaults/$LANG.
### print.dt


CDE Print action definition file. This file is located in
/usr/dt/appconfig/types/$LANG.
### <printer>.dt (Global actions)


The Print Viewer stores a <printer>.dt action file for each printer on the
system. These files are located in /etc/dt/appconfig/types/$LANG.
### <printer>.dt (customized actions)


The Print Viewer stores a <printer>.dt action file for each customized
printer. These files are located in the $HOME/.dt/types.
### dtprintinfo.cat


Message catalog. This file is located in /usr/dt/lib/nls/msg/$LANG.
### Printmgr.sdl, PM*.tif, PM*.pm


Help files. These files are located in /usr/dt/appconfig/help/$LANG and
/usr/dt/appconfig/help/$LANG/graphics.
## SEE ALSO


&cdeman.dtlp;,&cdeman.dtprintinfoaction;