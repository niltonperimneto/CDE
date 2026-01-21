DtStdInterfaceFontNamesfile formats`DtStdInterfaceFontNames`&str-XZ; Standard Interface Font NamesDESCRIPTIONThe &str-XZ; Standard
Interface Font Names are a set of generic X Window
System font names, needed by the &str-XZ; GUI itself, that are used for user
interface elements such as button labels, window titles and text fields.
These names, for seven sizes of two typefaces, must exist on all &str-XZ;
systems, and they should be provided in any
X server product on which &str-XZ; applications are expected to run.
Seven sizes of a third typeface are recommended.
They are typically mapped to existing fonts on the system
using the font alias mechanism, although this method is not
required.&str-XZ; 1.0 does not come with a common set of fonts on all systems,
and it must be able to run on X servers and X terminals from non-&str-XZ;
vendors if those vendors so desire. Therefore, there are a standard set of
``generic'' font names and sizes that each &str-XZ; vendor makes available
on their &str-XZ; systems and that X server vendors may make available
on their X servers and terminals. The names map to existing fonts on
each vendor's system and may vary from vendor to vendor.The &str-XZ; Standard Interface Font Names described here allow clients
making up the &str-XZ; desktop, such as &cdeman.dtterm; and a single
set of default fonts in theirapp-defaultsfiles, without
concern for the system or X server on which &str-XZ; is running. (The &str-XZ;
Standard Application Font Names, described in &cdeman.DtStdAppFontNames;,
provide a similar mechanism for applications running on the &str-XZ; desktop.)BackgroundInterface fonts are designed by user interface experts for the narrow
purpose of making the menus, labels and fields of a graphical user interface
highly readable. They are usually finely hand-tuned bitmapped fonts, intended
for use on visual displays only and not on printers, and many of the glyphs
have been specially modified for this purpose. Interface fonts can be contrasted
with application fonts, which are the fonts used within an application running
on the &str-XZ; desktop. Interface fonts come in a restricted set of styles
and are used for short strings of text, whereas application fonts usually
come in a variety of designs, styles and weights and are used for emphasis,
cross-references, section headers, and so forth.RationaleCommon font names are required to prevent &str-XZ; clients such as &cdeman.dtterm; from needing differentapp-defaultsfiles on each system. In addition, any X server or X terminal vendor
may ensure that the &str-XZ; desktop can run on their X server by mapping
these standard names to fonts of the corresponding style on their individual
X systems.Interface fonts are needed because of user interface and cognitive research
that has examined the readability of various fonts on the display screens
in use today and found that many fine adjustments (for example, for centering,
baseline, height and alignment) must be made to characters in a font to make
them clear, distinguishable and consistent when used for the interface objects
of a GUI. And by using hand-tuned interface fonts for the GUI objects, the
desktop can achieve a very clean, crisp visual appearance.Interface fonts are broken into 2 categories: system and user. Cognitive
research has shown that this distinction is important for the usability and
readability of GUIs. System fonts are those used when the system is presenting
information to the user (for example, in buttons). User fonts are those used
for text that a user enters into the system (for example, for a text field
or terminal emulator).XLFD Field Values for the Standard Interface Font NamesThese standard names are available using the X Window System XLFD font
naming scheme. There are three aspects to the standard names:Theunderlying fonton each system, or
X server platform, to which a standard name is mapped, typically will
be different on each system.Thestandard nameitself,
a full XLFD name mapped to the underlying font, may be different on each system
in some of the XLFD fields. However, most of the fields are the same from
system to system, allowing thepatterns(described next)
to be the same.The font resourcepatterncontaining the * wildcards, used inapp-defaultsfiles, which will match the full XLFD name of the standard name, is the same
across all systems, for a given use in anapp-defaultsfile.Each &str-XZ; or X server vendor implementing this specification
must provide full XLFD names for the standard names, mapped to system-dependent
underlying fonts, so that the XLFD patterns used in &str-XZ; applicationapp-defaultsfiles will always match one of the full XLFD names
provided.The Standard Interface Font Names are identified by the presence of
the following XLFD field name values:FOUNDRYisdtFAMILY_NAMEis eitherinterface systemorinterface user(there is a single space between the two words in each family name)In addition, the other fields of the XLFD names defining the standard
names are constrained as follows:WEIGHT_NAMEis eithermediumorboldSLANTis alwaysrSETWIDTH_NAMEis alwaysnormalSPACINGisporm(it must bemforinterface userfonts, and should bepforinterface systemfonts, althoughmis acceptable)ADD_STYLE_NAMEcontains both a nominal size value in the rangexxstoxxl(see below), as well as eithersansfor sans
serif fonts orseriffor serif, if appropriate for the
underlying fontThe numeric fields (PIXEL_SIZE,POINT_SIZE,RESOLUTION_X,RESOLUTION_Y, andAVERAGE_WIDTH) must contain the same values as the underlying font.CHARSET_REGISTRYandCHARSET_ENCODINGare not specified;
the standard names may be implemented for any &str-XZ; locale.Although thesansandserifvalues
in theADD_STYLE_NAMEfield are
not required by the XLFD font convention, they are always part of the &str-XZ;
Standard Font Names when the underlying fonts are characterized as serif or
sans serif. However, this document imposes no restriction on whether the interface
fonts are serif or sans serif. The relevant attribute must be coded in theADD_STYLE_NAMEfield. Thus, for example, the
standard names for Japanese fonts, which are not characterized as being serif
or sans serif, would not include this designation in theADD_STYLE_NAMEfield.Restricted Set of Styles AvailableUnlike the Standard Application Font Names, only a limited set of styles
is available in the Standard Interface Font Names. The styles available represent
the minimum set currently considered necessary for the desktop GUI needs:a medium weight of an interface system font, preferably proportionally
spaced (but mono-spaced is acceptable if appropriate for the locale)a medium weight of an interface user font, always
mono-spaceda bold weight of an interface user font, always
mono-spaced (the standard font names for this generic typeface are recommended
if available for the targeted fonts and locale, but are not required)Named Set of Point Sizes AvailableIn addition, the set of seven point sizes for each of the three styles
that are part of this document are ``named'' point sizes, using string values
in theADD_STYLE_NAMEfield. Thus,
XLFD patterns matching these names match a size based on the named size, not
on a numeric size, even though the latter does exist in the XLFD name. These
named sizes are used because the exact size of an interface font is less important
than its nominal size, and implementation differences for the hand-tuned interface
fonts do not allow common numeric point sizes to be assured across systems.
The seven nominal sizes are as follows:xxsextra extra smallxsextra smallssmallmmediumllargexlextra largexxlextra extra largeThe goal of these named sizes is to provide enough fonts so that both
the variety of display monitor sizes and resolutions that &str-XZ; will run
on, and the range of user preferences for comfortably reading button labels,
window titles and so forth, can be accommodated in the GUI. Thus, both the
smallest size,xxs, and the largest size,xxl, are meant to be reasonable sizes for displaying and viewing the &str-XZ;
desktop on common displays and X terminals; they are not meant to imply either
hard-to-read fine print or headline-sized display type.These named size values must occur first in theADD_STYLE_NAMEfield, before any use of the valuesseriforsans(one of which is always required
when the underlying font can be so characterized) and before any other additional
stylistic attribute that might be appropriate. This is important when specifying
wild-carded patterns in a resource specification for these fonts, since whether
the underlying font these names are mapped to is serif or sans serif is not
specified by &str-XZ;, and the match must work for all XLFD names provided
by &str-XZ; system vendors or other X server vendors.Example XLFD Patterns for the Standard NamesUsing these values, the XLFD pattern-dt-interface*-*logically matches the full set of &str-Zx; Standard Interface Font Names.
(Note that no specific X server behavior is implied).The full set of 21 &str-XZ; Standard Interface Font Names can also be
represented, in a more meaningful way, as follows:-dt-interface system-medium-r-normal-*-*-*-*-*-*-*-iso8859-1
-dt-interface user-medium-r-normal-*-*-*-*-*-m-*-iso8859-1
-dt-interface user-bold-r-normal-*-*-*-*-*-m-*-iso8859-1The full set of patterns, usable inapp-defaultsfiles, for all seven sizes for the system font, for example, is:-dt-interface system-medium-r-normal-xxs*-*-*-*-*-*-*-iso8859-1
-dt-interface system-medium-r-normal-xs*-*-*-*-*-*-*-iso8859-1
-dt-interface system-medium-r-normal-s*-*-*-*-*-*-*-iso8859-1
-dt-interface system-medium-r-normal-m*-*-*-*-*-*-*-iso8859-1
-dt-interface system-medium-r-normal-l*-*-*-*-*-*-*-iso8859-1
-dt-interface system-medium-r-normal-xl*-*-*-*-*-*-*-iso8859-1
-dt-interface system-medium-r-normal-xxl*-*-*-*-*-*-*-iso8859-1These patterns could be used in a resource file and will match the full &str-XZ;
Standard Interface Names for Latin-1 locales on all &str-XZ;, or complying
X server, systems.Note in these wild-carded XLFD names that theADD_STYLE_NAMEfield has a pattern, such asxxs*,
and that the pattern is partly a string (xxs) and partly
the pattern-matching character*. The full XLFD name this
pattern matches--the XLFD name implementing the Standard Interface name--will
often containsansorserifin the field,
after thexxsand a space, and so the*is essential to match thatsansorserifstring (and any additional style attribute string that might be in the underlying
name). Note also that theSPACINGfield is wild-carded in the pattern for the system font, since eitherpormmay appear in the standard name being
matched.Implementation of Font NamesEach &str-XZ; system vendor and X server vendor provides mappings
of its own fonts to XLFD names as described by this document. The actual XLFD
names will vary from system to system, just as the fonts they are mapped to,
since they contain some of the same values as the XLFD name of the underlying
font. What does not vary is the behavior: the common patterns in which only
specified fields are used will match each system's standard names. This is
guaranteed by the field specifications given earlier.There is no precise specification of how the named sizesxxstoxxlare mapped to sizes of underlying fonts
in each system or X server product, although each size must be equal
to or larger than the previous size. Nonetheless, some guidelines are appropriate.Interface fonts have been developed because of human factors research
on visual clarity of text on displays, and this has been done in the context
of the display technology typically available today, mostly in the 100 dots
per inch (DPI) range. That, and the use of standard point sizes (10, 12, 14,
18) in the graphics arts, have resulted in the development in the industry
of hand-tuned bitmapped fonts for a set of ``pixel heights'' that are likely
to be used for these standard names. However, making the &str-XZ; desktop
usable with a range of point sizes effectively means, in addition to legibility
for the user, that the various &str-XZ; applications fit ``appropriately''
on the screen using those point sizes. This means, for example, that two application
windows can appear side by side on a typical display or that a certain number
of buttons can appear across the screen.Thus, these guidelines are expressed not only in pixel sizes, to reflect
current usage, but also in percentage of monitor height. This allows them
to remain appropriate as technological evolution improves display resolution
and monitor size (for example, wall-mounted monitors). The ideal set of sizes
would form a linear progression from the smallest (xxs)
to the largest (xxl), although this is not achievable.
The basic guideline is that thexxsfont should be, in
pixels, no less than 0.9% of the height of the display resolution, in pixels;
thexxlfont should be no more than 2.6% of the height.As an approximate example that does not represent any existing mapping
of fonts to a display, this table shows how the named sizes might map to real
bitmapped fonts of a given pixel size, and how large those sizes are in percentage
and point size terms: