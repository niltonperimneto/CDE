dticonfilespecial filedticonfileicon conventions for &str-XZ; iconsThe conventions used within the &str-XZ; for icon
sizes, naming, location, and usage are specified.DESCRIPTIONBoth X Pixmap and X Bitmap icon file formats are used within the &str-XZ;.
X Pixmap (XPM) icons are multi-color images based on the XPM format developed
by Arnaud Le Hors. A pixmap file is an ASCII file that can be read and modified
by hand, in addition to using color pixmap editors (like the &str-XZ; Icon
Editor). X Bitmap (XBM) icons are monochrome (two-color) images based on the
official X11 Bitmap File Format. Bitmap files are ASCII files; however, the
data is simply a binary representation of the bitmap, and as such is not easily
read or modified by hand. The &str-XZ; icon editor can create and modify XBM
icons. The icon editor has the ability to write out any icon in either the
XPM or XBM file format. For a more detailed description of these file formats
and the icon editor, see the &cdeman.dtbmfile;, &cdeman.dtpmfile; and &cdeman.dticon; man pages.File Naming&str-XZ; icon file names are typically in one of the following forms:basename.formatbasename.size.formatThebasenameis the logical name of the icon. The
basenames for icons that are installed with &str-XZ; begin with eitherDtorFp.Dtis the default
prefix for all &str-XZ; icons. TheFpprefix is used for
icons that appear in the front panel when an icon other than the defaultDticon is desired. In this case, the basename is the same.If an additional icon is needed for the clienticonImage(iconified client window icon), a third prefix,Ic, is
used.Theformatispmfor a pixmap file andbmfor a bitmap file. Size is a single
letter:lfor large,mfor medium,sfor small andtfor tiny. Many of the logical
icons are provided in multiple sizes for both color and monochrome. This allows &str-XZ;
to use the optimal color and size combination for the specific task and configuration
the user is running. Many bitmap icons have a mask associated with the icon.
These are namedbasename.size_m.format.
All icons are named so that the longest filename
associated with that icon is 14 bytes or less; this allows it to be used on
a short filename system. The longest filename can be described asDtxxxxx_m.l.pm, wherexxxxxis the logical icon name.ExampleThis is an example of icon files that might be associated with the iconfoo. The single logical iconfoocontains tiny,
small, medium and large bitmap icons (with mask) and pixmap icons.Dtfoo.t.pm
Dtfoo.t.bm
Dtfoo.t_m.bm
Dtfoo.s.pm
Dtfoo.s.bm
Dtfoo.s_m.bm
Dtfoo.m.pm
Dtfoo.m.bm
Dtfoo.m_m.bm
Dtfoo.l.pm
Dtfoo.l.bm
Dtfoo.l_m.bmIcon SizesIcons of the following sizes and with the following suffixes are supported:Large48&times;48 icon with.lsuffixMedium32&times;32 icon with.msuffixSmall24&times;24 icon with.ssuffixTiny16&times;16 icon with.tsuffixThe icon sizes used varies for different components, and is dependent
on the display hardware.Table Of Icon Sizes Used