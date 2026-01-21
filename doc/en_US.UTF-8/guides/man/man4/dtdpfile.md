# dtdpfile
special filedtdpfileformat and location of desktop palette files#color 1 background RBG data
#color 2 background RBG data
#color 3 background RBG data
#color 4 background RBG data
#color 5 background RBG data
#color 6 background RBG data
#color 7 background RBG data
#color 7 background RBG data
## DESCRIPTION


A palette is a color scheme used to color a CDE
workspace and the CDE components.

A`palette`file provides the set of eight background colors that
make up a color palette.

A variety of palettes are provided with the CDE.
As part of the default palette set there are several palettes designed
specifically to work well on monochrome and gray scale displays.

TheStyle Manageris the desktop graphical user interface tool
which is used to add, delete, or modify a color palette.
The end user should never need to act on the palette file directly.

All palette files have a.dpsuffix.
The file contents are the
eight colors of the palette, specified as RGB values in hexadecimal.

The palette files provided with the system are located in the directory/usr/dt/palettes/C.

User specific palette files are located in`HOME`/.dt/palettes.

TheStyle Managerresource`paletteDirectories`can be used
to specify additional directories where palettes are located.

When a user creates a new palette, it is located in`HOME`/.dt/palettes.
System palettes that the user has modified are also located in this directory.
When a user deletes a palette, that palette is placed in the`HOME`/.dt/palettesdirectory with a`~`prefix.
If it was a user
specified palette, the original file is removed.
System palettes are never
removed.
If the user has a`~`copy of a system palette, that palette
will not appear in the list of available palettes in theStyle Manager.
## RETURN VALUE


None.
## EXAMPLES


The following is an example of a palette file named`Julie.dp`:#a200e500c600
#3ffc93008d77
#2d2d4e4e7600
#78008900a500
#63ea7e189500
#3ec670ec8a00
#3d157d008400
#61006d098e00
## APPLICATION USAGE


None.
## SEE ALSO


&cdeman.dtsession;, &cdeman.dtstyle;.