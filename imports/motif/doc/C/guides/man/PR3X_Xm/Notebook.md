# XmNotebook
library call`XmNotebook`The Notebook widget classXmNotebookwidget classNotebook&npzwc;#include &lt;Xm/Notebook.h>
## DESCRIPTION


Notebook is a manager widget that
organizes its children into pages, tabs, status areas, and page scrollers to
simulate a real notebook. It stacks its page children so that all page
children occupy the same area like real book pages.
Notebook displays visuals that look like the binding of a book and the
edges of other pages around the page that is shown.
Tab children simulate
notebook tabs. Major tabs divide Notebook into several sections,
and minor tabs subdivide these sections. Status area children provide additional
information about pages such as page numbers. The page scroller child allows
the user to move from page to page. Notebook also provides tab scrollers
for scrolling major and minor tabs if it cannot display all tabs within its
edges.

The application creates pages, tabs, status areas, and page scroller
as children of the Notebook widget. Notebook creates tab scrollers
when the Notebook is created.

The`XmNnotebookChildType`constraint resource
of Notebook determines whether a child widget is a page, tab, status area,
or page scroller. Any Motif widget can be a page of the Notebook.
When the application creates a child of the Notebook widget without setting
the child type constraint, the child becomes a page by default, unless it has
the`XmQTactivatable`,`XmQTaccessTextual`, or`XmQTnavigator`trait.
Children with the`XmQTactivatable`,`XmQTaccessTextual`, or`XmQTnavigator`trait become
major tabs, status areas, and page scrollers, respectively.

Notebook uses the`XmQTaccessTextual`,`XmQTactivatable`,`XmQTjoinSide`, and`XmQTnavigator`traits, and installs the`XmQTscrollFrame`trait.

The application attaches a tab to a page by creating a tab child of the
Notebook and setting the`XmNpageNumber`constraint to the page number of
the targeted page. By the same method, a status area widget can be attached to
a page. The page scroller child, on the other hand, is associated with the
Notebook, not with a specific page. Therefore, there is only one valid page
scroller for each Notebook.
### Pages


Only one child of type`XmPAGE`is displayed at a time by Notebook.
Other page children are hidden off-screen. When Notebook displays a particular
page, it positions the previously-displayed page off-screen and puts the new
page in its place. The page is resized to fit into the dimensions that
Notebook has allocated to display pages.
### Page Numbers


Notebook uses the`XmNcurrentPageNumber`,`XmNfirstPageNumber`,
and`XmNlastPageNumber`resources to determine the current page and
available page number range. Only those pages whose page numbers are within
the range can be displayed. Other pages cannot be displayed until the range
between`XmNfirstPageNumber`and`XmNlastPageNumber`is
changed to include them or their page numbers are changed to a number within
the range.

If`XmNfirstPageNumber`and`XmNlastPageNumber`are not set explicitly
by the application, they are set to 1 by default; Notebook
sets`XmNlastPageNumber`to the largest page number assigned by the application thereafter by
default. However, once`XmNlastPageNumber`is set by the application,
Notebook no longer changes it
even when a page with a higher page number is managed.

The`XmNpageNumber`constraint resource is used for specifying the page
number of a page widget. It can be set to any integer. For tab and status area
children, the resource is used for linking the child widget to a page.
For the page scroller child, the resource has no meaning and is ignored by
the Notebook.

When a page without a page number is managed, Notebook assigns it
the smallest unallocated page number that is not less than
the first page number and greater than the last allocated page number. When
a tab or a status area without a page number is managed, the newly managed
widget is assigned
the page number of the most recently managed page, unless the page already has
the same type of child. If the page does have the same type of child,
Notebook assigns the newly managed widget
a page number one greater than the most recently managed page;
this new page number is now occupied.
Notebook may generate a
default page number greater than`XmNlastPageNumber`, making
those pages inaccessible to the user.
### Duplicate and Empty Pages


Since an application can create or change page numbers, it is possible
to have duplicate page numbers and empty pages. When two pages with the
same page number are managed, only the more recently managed page can be
displayed. Inserting a page with an existing page number
does not cause a warning. The old page widget cannot be displayed
until the new page widget is removed from the Notebook or until the page number
of the old page widget is changed to some other number.

An empty page is a page
slot where no page is inserted. Empty pages occur when a tab or status area
is associated with a page number that has no matching page widget.
Empty pages display the blank Notebook
background unless the application provides visual information to this
empty area while processing`XmNpageChangedCallback`.
### Notebook Visuals


Notebook draws lines around two sides of the top page
to simulate the edges of other pages that are behind the top page.
The`XmNbackPagePlacement`and`XmNorientation`resources determine
which two sides have the lines drawn around them. By default, they are drawn
on the bottom and right sides of the top page. The application can set
resources to control how many lines are drawn and how wide the area that they
are drawn in is. Applications can also choose from three styles of binding
visual that simulates the binding of a Notebook. Solid or spiral bindings
can be drawn by Notebook, or the application can supply a pixmap that is
tiled into the binding.
### Tabs


A major or minor tab is a Motif widget with the`XmQTactivatable`trait.
If a widget without the trait is created
for a tab, Notebook does not provide the page activation callback.
As a result, even though the tab is displayed, it cannot automatically move the
associated page to the top.

Major tabs divide the Notebook pages into sections. Minor tabs
subdivide these sections.
Only minor tabs associated with the current
section are displayed, where a section consists of the group of pages
between the current major tab and the next major tab, including the
current major tab but not including the page containing the next major tab.
The exception to this is when there is no preceding major tab, in
which case the section starts from the`XmNfirstPageNumber`value.
A user in one major tab section does not see
the minor tabs in other sections.
However, all tabs are used in computing
the size of the Notebook.

Unlike regular notebook tabs, tabs in the
Notebook are not attached to a physical page (a widget). They are, instead,
attached to a logical page (a page number). Therefore, it is possible to have
a tab with an empty page. When a page with a tab is removed from the Notebook,
the tab is not removed because it is still bound to a logical page.
Destroying or unmanaging
a page widget only erases the page and leaves an empty page.
It does not tear the page out of the Notebook. To remove the tab, the
application must explicitly destroy or unmanage it.

Notebook supports the`XmQTjoinSide`trait. A widget that has the`XmQTjoinSide`trait can be added to the Notebook as a Major or Minor
tab and will appear to be attached to its associated page with no margins or
shadows between them.
### Status Areas


A status area is any widget that is used for describing
the associated page. For example, the Label widget as a status area child
can hold a simple string or a pixmap that describes a page. A status area
widget is also attached to a page by the page number constraint resource.
Therefore, it is possible to have multiple status area widgets for one page.
Only the most recently managed status area widget for that page can be
displayed. All others for that page are not unmanaged, but their sizes are
used for computing the size of the Notebook. If no status area widget
is provided, the Notebook displays its blank background in the status area's
reserved space. Notebook does not create any default status area widget.
### Page Scrollers


The page scroller of the Notebook is any widget that the application
creates for scrolling pages. If the application does not create one when
the Notebook is realized, Notebook creates a SpinBox widget as the
default page scroller.
If the application creates a new page scroller, the default page scroller is
destroyed. If the application creates multiple page scrollers, only the
most recently managed one can be displayed and used. All others are
unmanaged.

The default SpinBox page scroller grays out one of the arrow visuals
if the current page is a boundary page.
If the current page is the first page, the previous arrow of the SpinBox
is grayed. If the current page is the last page, the next arrow of the
SpinBox is grayed.
### Tab Scrollers


Tab scrollers are created by the Notebook for scrolling major tabs and minor
tabs. When Notebook is initialized, it creates four ArrowButtonGadgets
for scrolling to the next major tab, the previous major tab, the next
minor tab, and the previous minor tab.
The application cannot replace these tab scrollers.
The application can change all resources
of these widgets except the position and the arrow direction. Tab scrollers
are only visible and enabled when there is not enough space to display all the
major or minor tabs appropriate to the page. Tab scrollers are also
grayed out when scrolling is inappropriate. The following lists the
tab scrollers that are created:`Child Widgets that XmNotebook Creates``Child``Name``Widget Class`Page ScrollerPageScrollerXmSpinBoxNext Major Tab ScrollerMajorTabScrollerNextXmArrowButtonGadgetPrevious Major Tab ScrollerMajorTabScrollerPreviousXmArrowButtonGadgetNext Minor Tab ScrollerMinorTabScrollerNextXmArrowButtonGadgetPrevious Minor Tab ScrollerMinorTabScrollerPreviousXmArrowButtonGadget

When the user selects the page scroller, a major tab, or a minor tab, the value
of`XmNcurrentPageNumber`is changed to the selected page number and`XmNpageChangedCallback`is invoked. After the application returns from
the callback, the Notebook displays the last page child whose page number is
equal to the current page number. It also displays the last matched status
area child. All other pages and status areas are automatically hidden. Major
tabs and minor tabs that can fit into the Notebook's edges are displayed and
positioned appropriately. All other tabs are also hidden. The application
can also cause a page change by calling`XtSetValues`on`XmNcurrentPageNumber`and then calling`XtCallCallbacks`on`XmNpageChangedCallback`.
### Orientation


The Notebook has eight different visual configurations, depending on
the value of`XmNbackPagePlacement`and`XmNorientation`. These two
resources determine the placement of back pages, the binding, major tabs,
minor tabs, the status area, and the page scroller. The location of the binding
is determined by`XmNorientation`.
Major tabs are always placed on the back page side opposite to the binding;
Minor tabs are placed on the
back page display area that is visually connected to the binding.
Both Major and Minor tabs are ordered so that the page numbers they access
increase as they get closer to the corner where the back pages meet.
The status area and the page scroller are always located on the bottom
of the Notebook, inside the frame. The page scroller is always placed adjacent
to a back page side. The following table shows the possible configurations
and the locations of each Notebook component within the configuration.
The default back page value
and the default orientation
are based upon`XmNlayoutDirection`.`Notebook Configurations``XmNbackPagePlacement``XmNorientation``Major Tabs``Status Area``Binding``Minor Tabs``Page Scroller`XmBOTTOM_RIGHTXmHORIZONTALRIGHTBOTTOM LEFTLEFTBOTTOMBOTTOM RIGHTXmBOTTOM_RIGHTXmVERTICALBOTTOMBOTTOM LEFTTOPRIGHTBOTTOM RIGHTXmBOTTOM_LEFTXmHORIZONTALLEFTBOTTOM RIGHTRIGHTBOTTOMBOTTOM LEFTXmBOTTOM_LEFTXmVERTICALBOTTOMBOTTOM RIGHTTOPLEFTBOTTOM LEFTXmTOP_RIGHTXmHORIZONTALRIGHTBOTTOM LEFTLEFTTOPBOTTOM RIGHTXmTOP_RIGHTXmVERTICALTOPBOTTOM LEFTBOTTOMRIGHTBOTTOM RIGHTXmTOP_LEFTXmHORIZONTALLEFTBOTTOM RIGHTRIGHTTOPBOTTOM LEFTXmTOP_LEFTXmVERTICALTOPBOTTOM RIGHTBOTTOMLEFTBOTTOM LEFT

There are three tab groups for tab group traversal
inside the Notebook: major tabs, minor tabs,
and the page scroller.
The application can also create additional types of tab groups
within the Notebook; for example, each page added by the application is
treated as a separate tab group by the traversal actions.
### Classes


Notebook inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`, and`XmManager`classes.

The class pointer is`xmNotebookWidgetClass`.

The class name is`XmNotebook`.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. The programmer can also set the resource
values for the inherited classes to set attributes for this widget.
To reference a resource by name or by class in a.Xdefaultsfile,
remove the`XmN`or`XmC`prefix and use the remaining letters.
To specify one of the defined values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use the remaining letters (in
either lowercase or uppercase, but include any underscores between
words). The codes in the access column indicate if the given resource
can be set at creation time (C), set by using`XtSetValues`(S), retrieved by using`XtGetValues`(G), or is not
applicable (N/A).

`XmNotebook Resource Set``Name``Class``Type``Default``Access`XmNbackPageBackgroundXmCBackPageBackgroundPixeldynamicCSGXmNbackPageForegroundXmCBackPageForegroundPixeldynamicCSGXmNbackPageNumberXmCBackPageNumberCardinal2CSGXmNbackPagePlacementXmCBackPagePlacementunsigned chardynamicCSGXmNbackPageSizeXmCBackPageSizeDimension8CSGXmNbindingPixmapXmCBindingPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbindingTypeXmCBindingTypeunsigned charXmSPIRALCSGXmNbindingWidthXmCBindingWidthDimension25CSGXmNcurrentPageNumberXmCCurrentPageNumberintdynamicCSGXmNfirstPageNumberXmCFirstPageNumberint1CSGXmNframeBackgroundXmCFrameBackgroundPixeldynamicCSGXmNframeShadowThicknessXmCShadowThicknessDimension0CSGXmNinnerMarginHeightXmCInnerMarginHeightDimension0CSGXmNinnerMarginWidthXmCInnerMarginWidthDimension0CSGXmNlastPageNumberXmCLastPageNumberintdynamicCSGXmNminorTabSpacingXmCMinorTabSpacingDimension3CSGXmNmajorTabSpacingXmCMajorTabSpacingDimension3CSGXmNorientationXmCOrientationunsigned charXmHORIZONTALCSGXmNpageChangedCallbackXmCCallbackXtCallbackListNULLC

* **`XmNbackPageBackground`** 

Specifies the background color for drawing back pages.
The default is a lower-intensity version of`XmNframeBackground`.
* **`XmNbackPageForeground`** 

Specifies the forground color for drawing back pages. The default is
taken from the application's default foreground color.
* **`XmNbackPageNumber`** 

Specifies the number of lines to draw for back pages. The minimum value is
1, and the maximum value is (`XmNbackPageSize`/ 2).
* **`XmNbackPagePlacement`** 

Specifies where to place the back pages.
The default is dependent on the`XmNlayoutDirection`resource
of the Notebook's instance parents.
It can have one of the following
values:

* **`XmBOTTOM_RIGHT`** 

Displays back pages on the Notebook's bottom and
right sides.
* **`XmBOTTOM_LEFT`** 

Displays back pages on the Notebook's bottom and
left sides.
* **`XmTOP_RIGHT`** 

Displays back pages on the Notebook's top and right
sides.
* **`XmTOP_LEFT`** 

Displays back pages on the Notebook's top and left sides.

* **`XmNbackPageSize`** 

Specifies the thickness of the back page rendering.
* **`XmNbindingPixmap`** 

Specifies the pixmap or bitmap for stippling or tiling the binding when`XmNbindingType`is`XmPIXMAP`or`XmPIXMAP_OVERLAP_ONLY`.
* **`XmNbindingType`** 

Specifies the binding type. It can have one of the following values:

* **`XmNONE`** 

Displays no binding.
* **`XmSOLID`** 

Displays a solid binding in the foreground color of the
Notebook within the binding area specified by`XmNbindingWidth`.
* **`XmSPIRAL`** 

Displays a spiral binding in the foreground color of the
Notebook within the area specified by`XmNbindingWidth`and within the area outside of the frame equal to
the area specified by`XmNbindingWidth`.
* **`XmPIXMAP`** 

Displays the binding with the pixmap or bitmap specified by`XmNbindingPixmap`as a stipple or tile. It uses the foreground color of
the Notebook for stippling. The binding width is decided by the larger value
of`XmNbindingWidth`and the width of the pixmap or bitmap.
* **`XmPIXMAP_OVERLAP_ONLY`** 

Displays the binding with the pixmap or bitmap
specified by`XmNbindingPixmap`as a stipple or tile. It uses the
foreground color of the Notebook for stippling. The binding is displayed only
within the binding area specified by`XmNbindingWidth`.

* **`XmNbindingWidth`** 

Specifies the width of the Notebook binding. If`XmNbindingType`is`XmPIXMAP`and the width of the pixmap specified in`XmNbindingPixmap`is greater than`XmNbindingWidth`, then this
resource is ignored and the
width of the pixmap is used as the width of the Notebook binding
instead.
* **`XmNcurrentPageNumber`** 

Specifies the page number of the currently displayed page. Initially, it is set
to`XmNfirstPageNumber`. If it is set to less than`XmNfirstPageNumber`,
then it is set to`XmNfirstPageNumber`.
If it is set to`XmNlastPageNumber`,
then it is set to`XmNlastPageNumber`.
* **`XmNfirstPageNumber`** 

Specifies the page number for the first page of the Notebook.
The Notebook does not scroll to any page numbers below this value.
* **`XmNframeBackground`** 

Specifies the background color for drawing the Notebook's frame.
* **`XmNframeShadowThickness`** 

Specifies the shadow thickness around the Notebook's frame.
* **`XmNinnerMarginHeight`** 

Specifies the margin on the top and bottom sides of the page, status
area, and page scroller widgets.
* **`XmNinnerMarginWidth`** 

Specifies the margin on the left and right sides of the page, status
area, and page scroller widgets.
* **`XmNlastPageNumber`** 

Specifies the page number for the last page of the Notebook.
The Notebook does not scroll to any page numbers above this value.
The default page number is the largest page number of managed page,
major tab, or minor tab widgets.
If this is set to a value that is less than`XmNfirstPageNumber`, the
behavior of the Notebook is undefined.
* **`XmNmajorTabSpacing`** 

Specifies the spacing distance between major tabs.
If`XmNframeShadowThickness`is greater than`XmNmajorTabSpacing`, then
this resource is ignored and the size of`XmNframeShadowThickness`is
used as the spacing distance between major tabs.
* **`XmNminorTabSpacing`** 

Specifies the spacing distance between minor tabs. If`XmNframeShadowThickness`is greater than`XmNminorTabSpacing`, then
this resource is ignored and the size of`XmNframeShadowThickness`is
used as the spacing distance between minor tabs.
* **`XmNorientation`** 

Specifies the orientation of the Notebook. It can have one of the following
values:

* **`XmHORIZONTAL`** 

Places the binding beside the pages, in the left or
right side of the frame.
* **`XmVERTICAL`** 

Places the binding above or below the pages, in the top
or the bottom of the frame.

* **`XmNpageChangedCallback`** 

Specifies the list of callbacks to call whenever the`XmNcurrentPageNumber`, representing the current page number, is
changed. This includes the point when the widget is realized and the
page number is initialized.
The callback structure isXmNotebookCallbackStruct.
The reason is`XmCR_MAJOR_TAB`,`XmCR_MINOR_TAB`,`XmCR_PAGE_SCROLLER_INCREMENT`,`XmCR_PAGE_SCROLLER_DECREMENT`, or`XmCR_NONE`, depending
upon what action caused the Notebook to display a new page.


`XmNotebook Constraint Resource Set``Name``Class``Type``Default``Access`XmNnotebookChildTypeXmCNotebookChildTypeunsigned chardynamicCGXmNpageNumberXmCPageNumberintdynamicCSGXmNresizableXmCResizableBooleanTrueCSG

* **`XmNnotebookChildType`** 

Specifies the child type of the Notebook. It can be one of the following types:

* **`XmPAGE`** 

The child is a page of the Notebook. This is the default
when the child does not have
the`XmQTactivatable`,`XmQTaccessTextual`, or`XmQTnavigator`trait.
* **`XmMAJOR_TAB`** 

The child is a major tab. This is the default when
the child has the`XmQTactivatable`trait.
* **`XmMINOR_TAB`** 

The child is a minor tab.
* **`XmSTATUS_AREA`** 

The child is a status area. This is the default when
the child has the`XmQTaccessTextual`trait and does not have
the`XmQTactivatable`trait.
* **`XmPAGE_SCROLLER`** 

The child is the page scroller. The default page
scroller is destroyed, if it exists. Any previously created page
scrollers are unmanaged. This is the default when the child
has the`XmQTnavigator`trait and does have the`XmQTactivatable`trait or the`XmQTaccessTextual`trait.

* **`XmNpageNumber`** 

Specifies the page number associated with the widget.
If the widget is a page, the number specifies the page number of the widget.
If the widget is not a page, the number specifies the page number of the
associated page. If none is supplied by the application, Notebook
generates the smallest unallocated page number when the child is managed.
This resource is ignored for the page scroller.
* **`XmNresizable`** 

Specifies whether this child can request a resize.

### Inherited Resources


Notebook inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback


A pointer to the following structure is passed to callbacks for`XmNpageChangedCallback`.typedef struct
{
        int`reason`;
        XEvent`*event`;
        int`page_number`;
        Widget`page_widget`;
        int`prev_page_number`;
        Widget`prev_page_widget`;
} XmNotebookCallbackStruct;

* **`reason`** 

Specifies the reason for the callback.
* **`event`** 

Points to the`XEvent`that triggered the callback. It can be NULL.
* **`page_number`** 

Indicates the page number to be displayed.
* **`page_widget`** 

Indicates the page widget that has the new page number.
It is NULL if no page widget with the page number is found.
* **`prev_page_number`** 

Indicates the page number of the currently displayed page. If the
callback procedure is being called at widget initialization, this page
number will be returned as`XmUNSPECIFIED_PAGE_NUMBER`.
* **`prev_page_widget`** 

Indicates the currently displayed page widget. If the callback
procedure is being called at widget initialization, NULL will be
returned.

### Translations


Notebook inherits translations from Manager.
### Accelerators


Notebook accelerators are added to all major tab and minor tab
children of XmNotebook.
Notebook accelerators are listed below.
These accelerators might not directly correspond to a
translation table.

* **osfBeginLine`:`** 

TraverseTab(`Home`)
* **osfEndLine`:`** 

TraverseTab(`End`)
* **osfLeft`:`** 

TraverseTab(`Previous`)
* **osfRight`:`** 

TraverseTab(`Next`)
* **osfUp`:`** 

TraverseTab(`Previous`)
* **osfDown`:`** 

TraverseTab(`Next`)

### Action Routines


Notebook action routines are described below:

* **TraverseTab(`Home|End|Next|Previous`)** 

Moves the focus on major or minor tabs.

### Additional Behavior


The Notebook widget has the additional behavior described below:

* **Tab** 

Notebook intercepts tab group traversal when traversal is entering or
leaving major or minor tabs. It does this to support major tabs and minor
tabs as two separate tab groups when they are actually treated as one
by traversal. If a minor tab has keyboard focus and a user or program
action specifies`XmTRAVERSE_PREV_TAB_GROUP`, keyboard focus will
go to a major tab. If a major tab has keyboard focus and a user or program
action specifies`XmTRAVERSE_NEXT_TAB_GROUP`, keyboard focus will
go to a minor tab.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see
&cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmCreateNotebook;,
&cdeman.XmManager;, and
&cdeman.XmNotebookGetPageInfo;.