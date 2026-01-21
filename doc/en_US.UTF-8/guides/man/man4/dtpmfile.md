# dtpmfile
special filedtpmfileformat of X Pixmap (XPM) format desktop icon files/* XPM */
    static char *icon_name[] = {
    "width height num_colors chars_per_pixel [x_hot y_hot]",
    "char {key color}+",
    ...
    "`pixels`",
    ...
    };
## DESCRIPTION


Pixmap icons are multi-color images based on the XPM (X PixMap) format.
A pixmap file is an ASCII file, which can
be read and modified by hand, in addition to the use of color pixmap
editors (like the Icon Editor).
The XPM format was designed specifically
for small "icon" images.
XPM files can be included directly in C source
code, so the file data is in the form of a pixmap structure variable.
A pixmap file can be broken down into three main components: generic
pixmap information, pixmap colors, and the actual pixmap data.

The generic pixmap file contains the following information:

pixmap name

pixmap width

pixmap height

number of colors used in pixmap

number of characters per pixel

x hot-spot and y hot-spot (optional)

Each color used in the pixmap is defined by a string containing the
following information:

character(s) used to represent this color in pixmap data

symbolic name for this color

mono visual to use for this color (that is, black/white)

color visual by name ("red") or RGB value ("#9696c8c8afaf")

gray scale visuals (most CDE icons do not include gray scale data)

The data itself is simply strings of symbols representing colors.
There is one string per column of the pixmap, and this data will
resemble the actual icon.
## RETURN VALUE


None.
## EXAMPLES


The following is a pixmap that can be used to represent an executable
file. The icon has a 3-D border around it and contains a lightning bolt.static char ** execute = {
"22 22 7 1",
"R s iconGray1 m white c #e1e1e1e1e1e1",
"B s iconGray3 m white c #afafafafafaf",
"~ s iconColor6 m white c yellow",
"Y s iconColor1 m black c black",
"X s iconGray2 m white c #c8c8c8c8c8c8",
"+ s iconGray7 m black c #4b4b4b4b4b4b",
"@ s iconGray5 m black c #7d7d7d7d7d7d",
"RRRRRRRRRRRRRRRRRRRBRR",
"RRRRRRRRRRRRRRRR~~BRRY",
"RRBBBBBBBBBBBBX~~YBBYY",
"RRBBBBBBBBBBBB~~YBBBYY",
"RRBBBBBBBBBB~~~YBBBBYY",
"RRBBBBBBBBX~~~YBBBBBYY",
"RRBBBBBBBB~~~YBBBBBBYY",
"RRBBBBBBX~~~~~~~~XYBYY",
"RRBBBBX~~~~~~~~XYYBBYY",
"RRBBBBBYYYY~~~XYBBBBYY",
"RRBBBBBBX~~~XYYBBBBBYY",
"RRBBBBBX~~XYYBBBBBBBYY",
"RRBBBX~~~YYYBBBBBBBBYY",
"RRBB~~~~~~~~~~~XYBBBYY",
"RRX~~~~~~~~~~X+YBBBBYY",
"RRBYYYY~~~~X+YBBBBBBYY",
"RRBBBBB~~B@YBBBBBBBBYY",
"RRBBBB~X@YYBBBBBBBBBYY",
"RRBBB~B+YBBBBBBBBBBBYY",
"RRBB~+YBBBBBBBBBBBBBYY",
"RRY~YYYYYYYYYYYYYYYYYY",
"RYYYYYYYYYYYYYYYYYYYYY"};
## APPLICATION USAGE


For a description of icon location, usage, design, etc. within the
CDE, refer to
&cdeman.dticonfile;.
## SEE ALSO


`X11`Window`System`documentation,
&cdeman.dticonfile;, &cdeman.dticon;,
&cdeman.dtbmfile;.