# dticon
user cmddticonthe Common Desktop Environment Icon Editordticonoptions
## DESCRIPTION


The Icon Editor provides the facilities that enable the user to create
new icons or modify existing ones.

The Icon Editor uses lines and geometric shape drawing tools in order
to create new icons or modify existing ones. It support XPM and XBM
formats. The Icon Editor, also is capable of grabbing images from the
display and use them as bases for new icons.

The Icon Editor, also, supports drag and drop, allowing the user to
drag an icon file from the file manager onto the Icon Editor window or
icon.
### Key Supported Tasks


- Modify size of window.

- Modify location of window.

- Iconify window.

- Edit a new or existing icon.

- Save the current icon

- Undo the last operation.

- Cut, copy, and paste areas within the Icon Editor drawing tablet.

- Grab images from the display.

- Rotate selected area.

- Scale selected area.

- Flip selected area right or left.

- Resize icon.

- Add a hot spot to the icon.

- Delete a hot spot from icon.

- Clear icon.

- Overlay the icon drawing area with a visible grid.

- Select icon format, XPM or XBM.

- Alter the magnification factor for the drawing tablet.

The Icon Editor accepts all of the standard X Toolkit/Widget command
line options. Additional application specific options are listed
below.
## OPTIONS


The following options are available from the command line:

`-f`file

This option takes a bitmap or pixmap file name.
If the file ends in .pm or .xpm suffix, it will try reading it as an XPM
file first. If it fails, it will try reading it as an XBM file.
If the file ends in .bm or .xbm suffix, it will try reading it as an XBM
file first. If it fails, it will try reading it as an XPM file.
If the file doesn't match any of these suffixes, it will try reading it
as an XPM file first. If it fails it will try reading it as an XBM file.

`-x`widthxheight

This option specifies an initial geometry for the icon image. If a
file is specified using the -f option, the size of that icon
supersedes the geometry specified.

`-session`file

This option takes the name of a session file as an additional
parameter. The Icon Editor is invoked with the specified session file
name. The session file is a file that was previously saved by the Icon
Editor through a session shutdown. This option causes all other
command line options to be ignored, as all settings are taken from the
specified session file.
## RETURN


Exit values are:

* **0** 

Successful completion.
* **>0** 

Error condition occurred.

## EXAMPLES


* **dticon -x 24x32** 

Starts the Icon Editor with an icon template
of width 24 and height 32.

## dticon


[Using Tooltalk to integrate dticon in your applications]
Tooltalk makes it possible to use dticon from within your application.
dticon supports the following Tooltalk messages: start, edit, quit.
An application can construct a Tooltalk message, which will include a
buffer containing the icon the application wishes to edit. The format
of the message appears in dticon's ptype file( dticon.ptype). The buffer
in the edit message is constructed to contain the icon contents of an XPM
or an XBM icon file. In the case of an XBM icon containing transparent regions
dticon generates two icon files, an XBM file containing the base icon(
the icon without the transparent regions) and a mask file that indicates
the transparent regions in the icon. The buffer in this case must include
both icon files double buffered, the base icon first and then the mask, both
in the same buffer.

* **NOTE:** 

An application may pass a buffer to dticon containing an XPM icon and
get back a double buffer containing an XBM icon and its mask, and vise versa.

## Suggested


- The Common Desktop Environment: ToolTalk Messaging Overview
- The Common Desktop Environment: Programmer's Guide
## RESOURCES


* **-useFileFilter** 

This resource instructs the Icon Editor to use the file filter feature
in OPEN and SAVE dialogs.

## FILES


Files used by this component.

* **/.../dt/app-defaults/C/Dticon** 

App-defaults file.
