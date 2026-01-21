# DtStdAppFontNames
file formats`DtStdAppFontNames`&str-XZ; Standard Application Font NamesThe &str-XZ; Standard Application Font Names are a set of generic X Window
System font names, usable by applications as their default
fonts, for the most common categories of type designs and styles.
These names, for at least six sizes of 13 typefaces,
must be provided on all &str-XZ;
systems, and they should be provided in any
X server product on which &str-XZ; applications are expected to run.
They are typically mapped to existing fonts on the system
using the font alias mechanism, although this method is not required.
## DESCRIPTION


&str-XZ; 1.0 does not come with a common set of fonts on all vendors'
systems, and both &str-XZ; itself and &str-XZ; applications must be able to run
on X servers and X terminals from non-&str-XZ; vendors if those vendors so
desire.
Therefore, there are a standard set of ``generic'' font names
and sizes that each &str-XZ; vendor makes available on their &str-XZ; systems and
that X server vendors may make available on their X servers and
terminals.
The names map to existing fonts on each vendor's system,
which may vary from vendor to vendor.

The &str-XZ; Standard Application Font Names described here allow
applications to use a single set of default font specifications in theirapp-defaultsfiles, without concern for the system
or X server
on which &str-XZ; is running.
Theseapp-defaultsapplication defaults are given as XLFD font name patterns that will
match the standard &str-XZ; font names on all &str-XZ; systems.
This allows
application developers both to reduce their concern with selecting
their default fonts from a varying set of fonts on different &str-XZ;
systems and to make use of the system default fonts.
### Background


Application fonts are the fonts used within an application, where a
wide variety of text designs, styles, weights and point sizes are useful.
These variations are used for emphasis, cross-references,
section headers, and so forth.
There are thousands of fonts available in the
market for use in applications, and different &str-XZ; systems will have
different fonts.
The standard names attempt to provide the minimum
variety in generic designs, styles and sizes that an application might
want to use as defaults.
(The &str-XZ; Standard Interface Font Names, described in
&cdeman.DtStdInterfaceFontNames;, provide a similar mechanism for the elements of the &str-XZ; desktop itself.)

Common application font names prevent applications
from needing differentapp-defaultsfiles on each &str-XZ; system.
The Standard Application Font Names
allow applications to use a singleapp-defaultsfile across all &str-XZ; systems.
In addition, any X server or X
terminal vendor may ensure that &str-XZ; applications can run on their
X server by mapping these standard application names to fonts of the
corresponding style on their individual X systems.
### Rationale


Two of the most common design variations in fonts used to display text
are the presence or absence of serifs and the choice between
proportional or regularly spaced (mono-spaced) characters.
Combining these two design variations yields four ``generic'' font
designs, or families:

serif proportionally-spaced

sans serif proportionally-spaced

serif mono-spaced

sans serif mono-spaced

Common examples of these four designs are:

Times Roman

Helvetica

Courier

Lucida Sans Typewriter

Each of these designs typically come, for text fonts,
in four styles (combinations of weight and slant):

plain

bold

italic

bold-italic

The four styles of each of the four design variations yield 16 generic
font variations.
These 16 generic fonts are among the most commonly
used in general desktop computing.
For example, taking the first three
real examples above (Times Roman, Helvetica, Courier), these 12 fonts,
along with the Symbol font, constitute the so-called ``Adobe 13'' that
is a de facto minimum set of fonts in the PostScript community in the
desktop computer marketplace.

In some cases, applications do not care about the exact font family or
name to be used, but do need to use a mono-spaced font, a sans serif
font or a serif font.
This &str-XZ; mechanism allows such applications to
be freed from the need to be concerned about the exact font names that
may or may not be present on a particular &str-XZ; system.
### The Standard Names for the Latin-1 Character Set


The 13 standard application font names are provided on all &str-XZ; systems
only for the
ISO 8859
(Latin-1) character set.
These represent 12
generic design and style variations (serif and sans serif
proportionally-spaced, and a mono-spaced font that is either serif or
sans serif), as well as a symbol font.
These standard names are
provided in addition to the ``real'' names of the fonts that the
standard names are mapped to for a particular &str-XZ; system.
An additional four standard font names, to allow both serif and sans serif
designs in a mono-spaced font, may also be provided by a &str-XZ; system.
### XLFD Field Values for the Standard Application Font Names


The standard names are available using the X Window System XLFD font
naming scheme.
There are three aspects to the standard names:

Theunderlying fonton each system,
or X server platform,
to which a standard name is mapped,
typically will be different on each system.

Thestandard nameitself, a full XLFD name mapped to the underlying font,
may be different on each system in some of the XLFD fields.
However, most of the fields are the same from system to system,
allowing thepatterns(described next) to be the same.

The font resource`pattern`containing the * wildcards, used inapp-defaultsfiles,
which will match the full XLFD name of the standard name,
is the same across all systems, for a given use in anapp-defaultsfile.

Each &str-XZ; or X server vendor implementing this specification
must provide full XLFD names for the standard names, mapped to
system-dependent underlying fonts,
so that the XLFD patterns used in &str-XZ; applicationapp-defaultsfiles will always match one of the full XLFD names provided.

The Standard Application Font Names are identified by the presence of
the following XLFD field name values:

`FOUNDRY`is`dt`

`FAMILY_NAME`is`application`

`WEIGHT_NAME`is`medium`or`bold`

`SLANT`is`r`or`i`

`SETWIDTH`is`normal`

`ADD_STYLE`is`sans`for sans serif,`serif`for serif

`SPACING`is`p`or`m`

`CHARSET_REGISTRY`is`iso8859`

`CHARSET_ENCODING`is`1`

Although`sans`and`serif`are not required by the XLFD font
convention, they are always part of the standard &str-XZ; font names.
### Point Sizes


The complete set of point sizes available for each of the standard
application font names is determined by the set of fonts included in a
system, whether bitmapped only or both bitmapped and scalable outline.
The minimum set of sizes required and available on
all &str-XZ; systems corresponds to the standard sizes of bitmapped fonts
that make up the default mapping for X11R5:
8, 10, 12, 14, 18 and 24.

For example, the entire set of six sizes of the plain monospaced font, on
any &str-XZ; system, is represented by:-dt-application-medium-r-normal-*-*-80-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-100-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-120-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-140-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-180-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-240-*-*-m-*-iso8859-1

These patterns will match the corresponding standard font name on any
&str-XZ; system, even though the`PIXEL_SIZE`and`AVERAGE_WIDTH`numeric
fields may be different on various systems, and the matched fonts may
be either serif or sans serif, depending on the implementation
of the set of standard names.
The`RESOLUTION`fields in the XLFD names of the underlying fonts, when
those fonts are bitmapped fonts, must match the resolution
of the monitor on which the fonts are displayed for the
point sizes to be accurate.
To provide expected point size
behavior for applications, systems should ensure that the`RESOLUTION_X`and`RESOLUTION_Y`fields of the underlying
fonts vary no more than 20% from the real monitor
resolution of the displays on which the fonts will be used.

Applications requesting point sizes different from the six in the minimum
set may obtain either ``scaled bitmapped'' fonts of the requested
design, or scaled outline versions of the requested design.
This behavior requires that the X server in question
support the scaling of fonts and that the standard names
are mapped to underlying fonts that can be scaled using
this support.
### Example XLFD Patterns for the Standard Names


Using the specified field values for these standard names,
subsets of the standard names can be represented with various XLFD patterns.
The XLFD pattern-dt-application-*

logically matches the full set of &str-Zx; Standard Application Font Names.
(Note that no specific X server behavior is implied).
The pattern-dt-application-bold-*-*-*-*-*-*-*-p-*-*-*-

matches the bold, proportionally-spaced &str-XZ; fonts, both serif and sans
serif.
And the pattern-dt-application-*-*-*-*-*-*-*-*-m-*-*-*-

matches the monospaced fonts (including both serif and sans serif).

The full set of &str-XZ; Standard Application Font Names can be represented
with the following patterns:-dt-application-bold-i-normal-serif-*-*-*-*-p-*-iso8859-1
-dt-application-bold-r-normal-serif-*-*-*-*-p-*-iso8859-1
-dt-application-medium-i-normal-serif-*-*-*-*-p-*-iso8859-1
-dt-application-medium-r-normal-serif-*-*-*-*-p-*-iso8859-1
-dt-application-bold-i-normal-sans-*-*-*-*-p-*-iso8859-1
-dt-application-bold-r-normal-sans-*-*-*-*-p-*-iso8859-1
-dt-application-medium-i-normal-sans-*-*-*-*-p-*-iso8859-1
-dt-application-medium-r-normal-sans-*-*-*-*-p-*-iso8859-1
-dt-application-bold-i-normal-*-*-*-*-*-m-*-iso8859-1
-dt-application-bold-r-normal-*-*-*-*-*-m-*-iso8859-1
-dt-application-medium-i-normal-*-*-*-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-*-*-*-m-*-iso8859-1
-dt-application-medium-r-normal-*-*-*-*-*-p-*-dtsymbol-1

Each of these 13 standard names comes in at least six point sizes.
### Implementation of Font Names


Each &str-XZ; system vendor and X server vendor
provides mappings of
their own fonts to XLFD names meeting this standard, so that &str-XZ;
applications will work on their system.
The actual XLFD names will
vary from system to system, just as the fonts they are mapped to,
since they contain some of the same values as the XLFD name of the
underlying font.
What does not vary is the behavior:
the common
patterns in which only specified fields are used will match each
system's standard names.
This is guaranteed by the field specifications given earlier.

The following requirements are placed on
each &str-XZ; or X server vendor's
implementation of the Standard Application Font Names:

The names must be fully specified XLFD names, without wild cards.

The`WEIGHT_NAME`,`SLANT`,`SETWIDTH_NAME`,`SPACING`,`CHARSET_REGISTRY`and`CHARSET_ENCODING`fields must contain valid values as defined previously
and must match those in the underlying font.

The`ADD_STYLE_NAME`field must contain either the`serif`or`sans`designation, whichever matches the underlying font.
### Default &str-XZ; Mappings for Latin-1 Locales


The default mapping of these standard application font names for the
ISO 8859
locales is to the following standard X11R5 bitmapped fonts
(the six minimum sizes are not shown explicitly in these patterns):-adobe-times-bold-i-normal--*-*-*-*-p-*-iso8859-1
-adobe-times-bold-r-normal--*-*-*-*-p-*-iso8859-1
-adobe-times-medium-i-normal--*-*-*-*-p-*-iso8859-1
-adobe-times-medium-r-normal--*-*-*-*-p-*-iso8859-1
-adobe-helvetica-bold-o-normal--*-*-*-*-p-*-iso8859-1
-adobe-helvetica-bold-r-normal--*-*-*-*-p-*-iso8859-1
-adobe-helvetica-medium-o-normal--*-*-*-*-p-*-iso8859-1
-adobe-helvetica-medium-r-normal--*-*-*-*-p-*-iso8859-1
-adobe-courier-bold-o-normal--*-*-*-*-m-*-iso8859-1
-adobe-courier-bold-r-normal--*-*-*-*-m-*-iso8859-1
-adobe-courier-medium-o-normal--*-*-*-*-m-*-iso8859-1
-adobe-courier-medium-r-normal--*-*-*-*-m-*-iso8859-1
-adobe-symbol-medium-r-normal--*-*-*-*-p-*-adobe-fontspecific

A system may provide a different
mapping of these standard names as long as all 13 names map to fonts of
the appropriate design and style and the required six point sizes are
available.
The system documentation must document the
system-specific default mapping for the standard names.
### Font Names in app-defaults Files


An application can use a singleapp-defaultsfile to specify font resources and use it across all &str-XZ; systems.
Since most of the fields (`FOUNDRY`,`FAMILY_NAME`,`WEIGHT_NAME`,`SLANT`,`SETWIDTH_NAME`,`ADD_STYLE_NAME`,`POINT_SIZE`,`SPACING`,`CHARSET_REGISTRY`and`CHARSET_ENCODING`) of the standard names are the same across different systems,
these values can be used in the resource specification in theapp-defaultsfile.
However, other fields (`PIXEL_SIZE`,`RESOLUTION_X`,`RESOLUTION_Y`and`AVERAGE_WIDTH`) may vary across
systems, and so must be wild-carded in the resource specification.
For example:appOne*headFont: -dt-application-bold-r-normal-sans-*-140-*-*-p-*-iso8859-1
appOne*linkFont: -dt-application-bold-i-normal-sans-*-100-*-*-p-*-iso8859-1

might be used to specify some of AppOne's default font resource needs.Other Character Sets in the Common LocalesThe standard application font names defined above can be used for locales other than the
ISO 8859 character set with the following exceptions and differences. For the
following locales, CDE guarantees that systems provide fonts with the following
XLFD attribute values, and that they be accessible using these names.
For full information on how vendors ship the fonts, and make such names
usable with the appropriate font base name lists required for correct CDE
support for internationalization, see the guidelines in the CDE
Internationalization Programming Guide document.Locales using ISO 8859-2, -3, -4, -5 (Cyrillic), -6 (Arabic), -7 (Greek), -8 (Hebrew):The same values forFOUNDRY,FAMILY_NAME,WEIGHT_NAME,SLANT,SET_WIDTH,ADD_STYLEandSPACINGas are used in this definition for the ISO 8859 locale.Japanese locales:Two values for theFAMILY_NAMEattribute (GothicandMincho) and two values for theWEIGHT_NAMEattribute (mediumandbold) as well asSLANT(r),ADD_STYLE(*) andSPACING(m).Chinese (Taiwan) locales:Two values for theFAMILY_NAMEattribute (SungandKai) and two values for theWEIGHT_NAMEattribute (mediumandbold) as well asSLANT(r),ADD_STYLE(*) andSPACING(m).Chinese (PRC) locales:Two values for theFAMILY_NAMEattribute (SongandKai) and two values for theWEIGHT_NAMEattribute (mediumandbold) as well asSLANT(r),ADD_STYLE(*) andSPACING(m).Korean locales:Two values for theFAMILY_NAMEattribute (TotumandPathang) and two values for theWEIGHT_NAMEattribute (mediumandbold) as well asSLANT(r),ADD_STYLE(*) andSPACING(m). Note that these names
are unofficial, tentative romanizations of the
two common font families in use in Korea;Totumcorresponds to fonts typically
shipped as Gothic, Kodig or Dotum andPathangcorresponds to fonts typically
shipped as Myungjo or Myeongjo. The official roman names for these fonts
are under review and may be changed in the future by the Korean
government, and thus may change for CDE.In addition, to facilitateapp-defaultsfiles that work under a variety of locales,
CDE systems must provide an additional set of Standard Application Font Names where theFAMILY_NAMEisapplication.
These font names are identified by the
presence of the following XLFD field values:FOUNDRYisdtFAMILY_NAMEisapplicationWEIGHT_NAMEismediumorboldSLANTis as appropriate (see above)SET_WIDTHisnormalADD_STYLEis*SPACINGis as appropriate (see above)CHARSET_REGISTRYis as appropriateCHARSET_ENCODINGis as appropriateThis set of names is mapped to an underlying font representing one of theFAMILY_NAMESas listed above.For example, the XLFD names-dt-mincho-medium-r-normal--*-*-*-*-m-*-jisx0208.1983-0
-dt-application-medium-r-normal--*-*-*-*-m-*-jisx0208.1983-0may both be mapped to-ibm_aix-mincho-medium-r-normal--*-*-*-*-m-*-jisx0208.1983-0This scheme allows application writers to create a XmFontList in a resource file as follows,
without regard for a particular locale:*FontList:-dt-application-medium-r-normal-*-0-14-75-75-m-0-*-*:and be assured that the font will be reasonable. Notice that specifying
fonts this way not only disregards the selection of the AsianFAMILY_NAME,
but also the LatinseriforsansADD_STYLEfield. The font selected
will be the default.
## SEE ALSO


&cdeman.dtstyle;, &cdeman.dtterm;, &cdeman.DtStdInterfaceFontNames;
## NOTES


There is no requirement on a &str-XZ; system to implement these
standard names in a particular way.
Several mechanisms are possible:
duplicate font files with altered naming attributes, X11R5 font
aliases, or vendor-specific mechanisms.
The only requirement is that
an XLFD pattern, written with attributes taken from the set that define
the standard names, can be successfully used to open a font with the
Xlib function`XLoadFont`; and, specifically, the Xlib function`XListFonts`need NOT return the same XLFD names for the pattern on
different &str-XZ; systems.

&str-XZ; applications should, of course, be written to behave in a
reasonable manner if these standard font names are not available on a
particular X server.
This is typically done in an X application by
defaulting to the`fixed`and`variable`fonts.