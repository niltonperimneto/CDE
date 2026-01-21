# dtcm_editor
user cmddtcm_editorThe CDE Environment standalone appointment editordtcm_editorfilename
## DESCRIPTION


Thedtcm_editorprogram a stand alone editor for files that
representdtcmappointments. These files are created by either
mailing appointments from withindtcm, or by dragging and dropping
appointments from adtcmeditor dialog into some kind of
permanent store (adtmailattachment pane, ordtfile).
Thedtcm_editorallows modifying these appointment files,
and writing them back out.
## OPTIONS


The only available option is to start thedtcm_editorwith
a filename. This will cause the editor to operate on that file,
and write out to that file. If thedtcm_editoris started
without a file name, it is assumed to have been invoked thru
tooltalk, and acts as a media alliance editor.
## RESOURCES


dtcm_editorsupports a number of application resources to
allow you to configure
its behaviour. The application class name fordtcm_editorisDtcm.
To set application resources, you can copy the system default version of
this file from/usr/dt/app-defaults/<LANG>/Dtcmto a personal version,
typically~/app-defaults/Dtcm, and edit it with your changes.
## FILES


* **/usr/dt/bin/dtcm_editor** 

This is the executable fordtcm_editor.
* **/usr/dt/app-defaults/<LANG>/Dtcm** 

This is the system-default application defaults file fordtcm_editor.
