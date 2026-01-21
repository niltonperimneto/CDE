
# Icon Editor Concepts







# Getting Started with Icon Editor
using: Icon EditorIcon Editor: usingediting: iconsicons: editing

Icon Editor enables you to create and edit images in two formats:

X pixmap (XPM format)-- Multicolor images that include static
and dynamic colors. Pixmap files are normally identified by a.pmfile-name extension.

X bitmap (XBM format)-- Black-and-white images. Bitmap files
are normally identified by a.bmfile-name extension.

You draw images by selecting a tool and a color, and then drawing in the work
area. As you draw, Icon Editor displays an actual-size copy of your icon
in both formats. Even if you are drawing an image for use on color
systems, you should be sure the black and white version is legible because
icons may revert to the bitmap format if there are not enough free colors
to display the full color version.
# Icon Design Recommendations


Try to use a common theme among related icons. For example, if you are
designing icons for an application, have purposeful similarities between
the application's icon and icons for related data files.

Be sure the black-and-white version of any color icon you design is acceptable.
If the icon is displayed on a monochrome or grayscale display (or if there
are not enough colors available), the icon is automatically displayed in
its black-and-white form.




# Color Usage
color: usage in iconsicons: color usage

Icons in the desktop use a palette of 22 colors:

Eight static grays

Eight static colors: red, blue, green, cyan, magenta, yellow, black,
and white

Six dynamic colors: foreground, background, top shadow, bottom shadow,
select, and transparent

This palette is rich enough to create attractive, easy-to-read icons
without taking too many color resources that may be needed by other
applications.
Icons provided with the desktop use mostly grays with color for accent.

The dynamic colors are useful for icons that change color as different
color palettes are selected in Style Manager.

The transparent color is useful for creating icons that have the
illusion of being nonrectangular, since it allows the color behind
the icon to show through.
# Icon Size Recommendations
icons: size recommendationssize: icons

Following are the recommended sizes -- in pixels, width&times;height -- for
creating new icons.

File Manager (Large):

High resolution: 32&times;32&newline;Medium resolution:32&times;32&newline;Low resolution: 32&times;32

File Manager (Small):

High resolution: 16&times;16&newline;Medium resolution:16&times;16&newline;Low resolution: 16&times;16

Application Manager (Large):

High resolution: 32&times;32&newline;Medium resolution:32&times;32&newline;Low resolution: 32&times;32

Application Manager (Small):

High resolution: 16&times;16&newline;Medium resolution:16&times;16&newline;Low resolution: 16&times;16

Front Panel:

High resolution: 48&times;48&newline;Medium resolution:48&times;48&newline;Low resolution: 32&times;32

Front Panel Subpanels:

High resolution: 32&times;32&newline;Medium resolution:32&times;32&newline;Low resolution: 16&times;16

Minimized Windows:

High resolution: 48&times;48&newline;Medium resolution:48&times;48&newline;Low resolution: 32&times;32

Desktop:

High resolution: 32&times;32&newline;Medium resolution:32&times;32&newline;Low resolution: 32&times;32

Backdrop images can be any size. The pattern is repeated to fill the
entire workspace.
# See Also



# Icon File-Name Conventions
icons: how files are foundimage files: see iconsicons: naming conventions.pmfile-name.bmfile-namebitmaps: how files are foundpixmaps: how files are foundconventions, image file naming

Each icon and backdrop image is stored as a separate file.
Typically, an
icon is specified with just the base part of its file name. For example,
an icon might be referenced with just the namemailwhen the file is
actually stored as:/usr/dt/appconfig/icons/`language`/mail.l.pm

The file-naming convention of adding suffixes helps group icons by size and
type. For desktop components, many icon names are in these general
formats:`basename`.`size`.`format``basename`.`format`

where`basename`is the image name used to reference the image,`size`is a single letter indicating the icon size, and`format`ispmfor X pixmaps orbmfor X bitmaps.

The valid icon sizes are:

* **Name** 

Size Suffix
* **Tiny** 

16&times;16`t`
* **Small** 

24&times;24`s`
* **Medium** 

32&times;32`m`
* **Large** 

48&times;48`l`


For example, suppose you specify an icon namedmailfor a file type
you've written. If you have a color display and have set the File Manager
preferences to use small icons, the assumed icon name ismail.s.pm(thesrepresents small, andpmis for`pixmap`, the color
icon format).
# Search Paths


The directory where an image is stored is determined by searching for the
file in a list of directories. This list of directories is defined
by a "search path" for icons.

If you are using a color display, the desktop searches for files ending
in.pmfirst. Otherwise, it looks for files ending in.bm.

If an image is specified with a complete path name, the search
path is not used.
# See Also


lists the search path specifications.
# Storing Icon Files
storing icon filesicon file: storing

By default, the desktop components look in these directories for icon files:

Personal icons:/`HomeDirectory`/.dt/icons

System-wide icons:/etc/dt/appconfig/icons/`language`

Built-in icons:/usr/dt/appconfig/icons/`language`

For English icons, useCfor`language`.
# Storing Backdrop Image Files
backdrop: storing backdrop image filesimage file: storing backdrop

Each backdrop listed in the Style Manager Backdrop dialog box represents
an image file. By default, Style Manager looks for backdrops in these
directories:

System-wide images:/etc/dt/appconfig/backdrops/`language`

Built-in images:/usr/dt/backdrops/`language`

If you create a new backdrop image that you want all system users to
be able to access,
place the backdrop image in/etc/dt/appconfig/backdrops/`language`.
To restrict access to a backdrop image to particular users, put the
backdrop image in a different directory and add that directory to
the*backdropDirectoriesresource for those users.