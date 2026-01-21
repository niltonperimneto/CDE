# dthffile
special filedthffiledefine the format of CDE Help Family filesCDE`Help``Family`file format description.
## DESCRIPTION


The Family file is based on the X11 syntax for application resources.
Family files are used in conjunction with
&cdeman.dthelpgen; to generate a browser Help volume
that allows users to navigate through all of the available Help information.

The Family file,`<filename>.hf`, contains the following fields: character set,
title, abstract, volume list and an optional image name.

The`charSet`field specifies the standard CDE locale and code set value to be used in
parsing the title and abstract fields of the Family file.
The default
value is`en_US.iso8859-1`.
See theCDEHelpSystemAuthor'sandProgrammer'sfor a complete list of supported locales and code sets.
The Family title and abstract must not contain any HelpTag
markup as this file is not processed by the HelpTag
software.

The`title`field specifies the title of the family, as displayed in the top-level of
the browser Help volume.

The`abstract`field specifies the abstract for the Family.
Used to describe the family, it
gives information about what type of volumes are in this Family or
why these volumes are related.
For long abstracts, the
backslash character can be used to continue to the next line.

The`volumes`field specifies which volumes belong to the Family.
The
volumes are listed in the order they appear on this line.
A volume may
be listed in more than one Family.
This is a list of volume
names separated by spaces.
The ending.sdlextension must be specified if the search path contains a`%H`rather than an`%H.sdl`. See
&cdeman.DtHelpDialog; for more information about CDE Help search path syntax and use.

The`[bitmap]`field specifies the image that pictorially describes the Family.
This image is displayed in the top-level browser volume just under the
Family title, and to the left of the abstract text.

The`icon`file is optional.
If one is provide, the
path used to specify the location of the file should be a complete
pathname.
If an icon file is not provided, the`*.bitmap`resource in the Family file should not be provided.

`Supported``Graphics``Formats:`

* **`*.xwd`** 

X11 XWD file
* **`*.tif`** 

TIFF 5.0 image file
* **`*.pm`** 

X11 X Pixmap file
* **`*.bm`** 

X11 Bitmap file

### Registering A Product Family


To register a product family, create and install a Help
Family file (`product.hf`) with the rest of the product's Help files.
Register the Family file by running thedtappintegratescript, which
creates a symbolic link to the`product.hf`file.
For most products, the
appropriate place for the link is/etc/dt/appconfig/help/language/. See
&cdeman.dtappintegrate; for more information on registering Help within the desktop.
## EXAMPLES


* **`*.charSet:`** 

iso8859-1
* **`*.title:`** 

Developer's Information About The Help System
* **`*.abstract:`** 

Abstract giving a short description about the Help system.
* **`*.volumes:`** 

helpkit.sdl helpdemo.sdl
* **`*.bitmap:`** 

/usr/local/bitmap/helpGraphic.bm

## SEE ALSO


&cdeman.dthelpgen;.