XmSpinBoxlibrary callXmSpinBoxThe SpinBox widget classXmSpinBoxwidget classSpinBox#include <Xm/SpinB.h>DESCRIPTIONSpinBox allows the user to select a value from a ring of related but
mutually exclusive choices which are displayed in sequence. The SpinBox always
has an increment arrow, a decrement arrow, and one or more other children.
The choices are displayed, one at a time, in a traversable text child (XmTextorXmTextField. The user clicksBtn1on an arrow to display the next (or previous) item in the ring
of choices. By pressing and holdingBtn1on an arrow, the
user continuously cycles through the choices.The traversable children in a SpinBox can be of typeXmNUMERICorXmSTRING,
as defined by theXmNspinBoxChildTypeconstraint resource. The ring of choices for numeric children is defined
by minimum, maximum, incremental, and decimal point values. The ring of choices
for string children is defined in an array of compound strings.The application programmer can include multiple traversable children
in the SpinBox. For example, a SpinBox might consist of a pair of arrows
and month, day, and year text fields. The arrows only spin the child that
currently has focus.Arrow size is specified by the SpinBox resourceXmNarrowSize. This value sets both width and height of each
arrow in pixels.The programmer can display SpinBox arrows in one of several layouts,
as specified by theXmNarrowLayoutresource:XmARROWS_BEGINNINGPlaces a pair of left and right arrows before the children.XmARROWS_ENDPlaces a pair of left and right arrows after the children.XmARROWS_SPLITPlaces one arrow on each side of the children.XmARROWS_FLAT_BEGINNINGPlaces a pair of arrows side by side before the`XmSpinBox`children.XmARROWS_FLAT_BEGINNINGPlaces a pair of arrows side by side after the`XmSpinBox`children.Positions forXmARROWS_BEGINNINGandXmARROWS_ENDare dependent on
theVendorShellresourceXmNlayoutDirection. When layout direction is left-to-right, beginning
arrows are positioned to the left of the children. When layout direction
is right-to-left, beginning arrows are positioned to the right.The actions of the arrows are determined by theVendorShellresourceXmNlayoutDirection.
For left-to-right layouts, the right arrow is the increment arrow and the
left arrow is the decrement arrow. For right-to-left layouts, the right
arrow is the decrement arrow and the left arrow is the increment arrow.For a numeric type child, the increment arrow increases the displayed
value by the incremental value up to the maximum. The decrement arrow decreases
the displayed value by the given incremental value down to the minimum.The increment arrow for a string type child moves toward the last entry
of the array of compound strings (by increasing the SpinBox constraint resourceXmNposition). The decrement arrow
moves toward the first entry of the compound string array.The programmer can also control the sensitivity of each arrow in the
SpinBox. Sensitive arrows spin choices; insensitive arrows do not spin choices.
Arrow sensitivity is set for the SpinBox widget by using theXmNdefaultArrowSensitivityresource, but it
can be modified on a per child basis by using theXmNarrowSensitivityconstraint resource.SpinBox provides two callbacks to application programmers. (In addition,
the callbacks of the SpinBox's children may be invoked.) Each of these callbacks
receives a pointer toXmSpinBoxCallbackStruct.
TheXmNmodifyVerifyCallbackprocedures
are calledbeforea new choice is displayed.
TheXmNvalueChangedCallbackprocedures
are calledaftera new choice is displayed.XmNmodifyVerifyCallbacktells
the application what the new position will be in the ring of choices. This
callback can be used to make the SpinBox stop at the upper and lower limits
or go to a different, nonconsecutive choice. The application allows the
change in position by leaving thedoitmember
set to True. The application can spin to a position other than the next
consecutive position by leavingdoitset
to True and by changing thepositionmember
to the desired position. Whendoitis set
to False by an application, there is no change in the choice displayed.After a new choice is displayed, theXmNvalueChangedCallbackprocedure is called. The application
can use this procedure to perform tasks when specific values are reached
or when boundaries are crossed. For example, if the user spins from January
back to December, the application could change to the previous year. If
the user spins from December to January, the application could change to
the next year.SpinBox dimensions can be set using the Core resourcesXmNheightandXmNwidth. If dimensions are not specified, the SpinBox size
is determined by the sizes of its arrows and children. The SpinBox will
attempt to grow so that the arrows and all children are visible.SpinBox uses theXmQTaccessTextualtrait and holds theXmQTnavigatortrait.ClassesSpinBox inherits behavior, resources, and traits from theCore,Composite,Constraint, andXmManagerclasses.The class pointer isxmSpinBoxWidgetClass.The class name isXmSpinBox.New ResourcesThe following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a resource
by name or by class in a.Xdefaultsfile, remove theXmNorXmCprefix and use the remaining letters. To specify one of the defined values
for a resource in a.Xdefaultsfile, remove theXmprefix and use the remaining letters (in
either lowercase or uppercase, but include any underscores between words).
The codes in the access column indicate whether the given resource can be
set at creation time (C), set by usingXtSetValues(S),
retrieved by usingXtGetValues(G), or is not applicable
(N/A).XmSpinBox Resource
SetNameClassTypeDefaultAccessXmNarrowLayoutXmCArrowLayoutunsigned charXmARROWS_BEGINNINGCSGXmNarrowOrientationXmCArrowOrientationunsigned charXmARROWS_VERTICALCSGXmNarrowSizeXmCArrowSizeDimension16CSGXmNdefaultArrowSensitivityXmCDefaultArrowSensitivityunsigned charXmARROWS_SENSITIVECSGXmNdetailShadowThicknessXmCDetailShadowThicknessDimension2CSGXmNinitialDelayXmCInitialDelayunsigned int250 msCSGXmNmarginHeightXmCMarginHeightDimensiondynamicCSGXmNmarginWidthXmCMarginWidthDimensiondynamicCSGXmNmodifyVerifyCallbackXmCCallbackXtCallbackListNULLCXmNrepeatDelayXmCRepeatDelayunsigned int200 msCSGXmNspacingXmCSpacingDimensiondynamicCSGXmNvalueChangedCallbackXmCCallbackXtCallbackListNULLCXmNarrowLayoutSpecifies placement of the two arrows in the widget. Possible layouts
are as follows:XmARROWS_BEGINNINGPlaces left and right arrows beside each other, before the child(ren).
Positioning for this layout is dependent on the VendorShell resourceXmNlayoutDirection.XmARROWS_ENDPlaces left and right arrows beside each other, after the child(ren).
Positioning for this layout is dependent on the VendorShell resourceXmNlayoutDirection.XmARROWS_FLAT_BEGINNINGPlaces a pair of arrows side by side before the`XmSpinBox`children. Positioning for this layout is dependent on the VendorShell resourceXmNlayoutDirection.XmARROWS_FLAT_ENDPlaces a pair of arrows side by side after the`XmSpinBox`children. Positioning for this layout is dependent on the VendorShell resourceXmNlayoutDirection.XmARROWS_SPLITPlaces a left arrow on the left side and a right arrow on the right
side of the child(ren).XmNarrowSizeSpecifies both the width and height of the arrow in pixels.XmNdefaultArrowSensitivitySpecifies the default sensitivity of the arrows in the widget. Insensitive
arrows change color, cannot be depressed, and perform no action. (This resource
may be overridden by the constraint resourceXmNarrowSensitivityfor individual traversable text children
of the SpinBox.) Possible default sensitivity values are as follows:XmARROWS_SENSITIVEBoth arrows are sensitive.XmARROWS_DECREMENT_SENSITIVEOnly the decrement arrow (as determined byXmNlayoutDirection) is sensitive. The increment arrow is insensitive.XmARROWS_INCREMENT_SENSITIVEOnly the increment arrow (as determined byXmNlayoutDirection) is sensitive. The decrement arrow is insensitive.XmARROWS_INSENSITIVEBoth arrows are insensitive.XmNdetailShadowThicknessSpecifies the thickness of the inside arrow shadows. The default thickness
is 2 pixels.XmNinitialDelaySpecifies how long, in milliseconds, the mouse button must be held
down before automatic spinning begins. In other words, when the user selects
the increment or decrement arrow and keeps it depressed, this delay occurs
before the choices start spinning. IfXmNinitialDelayis 0, thenXmNrepeatDelayis used as the initial delay.XmNmarginHeightSpecifies the amount of blank space between the top edge of the SpinBox
widget and the first item in each column, and the bottom edge of the SpinBox
widget and the last item in each column.XmNmarginWidthSpecifies the amount of blank space between the left edge of the SpinBox
widget and the first item in each row, and the right edge of the SpinBox widget
and the last item in each row.XmNmodifyVerifyCallbackThis callback is called before the SpinBox position changes (see the
Constraint resourceXmNposition).
The application can use this callback to set the next position, change SpinBox
resources, or cancel the impending action. For example, this callback can
be used to stop the spinning just before wrapping at the upper and lower
position boundaries. If thedoitmember
is set to False, nothing happens. Otherwise the position changes. Reasons
sent by the callback areXmCR_SPIN_NEXT,XmCR_SPIN_PRIOR,XmCR_SPIN_FIRST, orXmCR_SPIN_LAST.XmNrepeatDelayWhen the user selects and keeps an arrow button depressed by pressing
and holdingBtn1, spinning begins. After the time specified
inXmNinitialDelayelapses, the
SpinBox position changes automatically until the arrow button is released.
TheXmNrepeatDelayresource specifies
the delay in milliseconds between each automatic change. IfXmNrepeatDelayis set to 0 (zero), automatic
spinning is turned off andXmNinitialDelayis ignored.XmNspacingSpecifies the horizontal and vertical spacing between items contained
within the SpinBox widget.XmNvalueChangedCallbackThis is calledn+1 times fornSpinBox position changes (see the Constraint resourceXmNposition). Reasons sent by the
callback areXmCR_OK,XmCR_SPIN_NEXT,XmCR_SPIN_PRIOR,XmCR_SPIN_FIRST, orXmCR_SPIN_LAST.
Other members are detailed in the callback structure description.XmSpinBox Constraint
Resource SetNameClassTypeDefaultAccessXmNarrowSensitivityXmCArrowSensitivityunsigned charXmARROWS_DEFAULT_SENSITIVITYCSGXmNdecimalPointsXmCDecimalPointsshort0CSGXmNincrementValueXmCIncrementValueint1CSGXmNmaximumValueXmCMaximumValueint10CSGXmNminimumValueXmCMinimumValueint0CSGXmNnumValuesXmCNumValuesint0CSGXmNpositionXmCPositionint0CSGXmNpositionTypeXmCPositionTypecharXmPOSITION_VALUECGXmNspinBoxChildTypeXmSpinBoxChildTypeunsigned charXmSTRINGCGXmNvaluesXmCValuesXmStringTableNULLCSGXmNarrowSensitivitySpecifies the sensitivity of the arrows for a SpinBox child. By using
this resource in the definition of a SpinBox child, the application programmer
can override the default SpinBox sensitivity (set byXmNdefaultArrowSensitivity) for a particular child. This allows
each traversable child to have a different arrow sensitivity. The arrow
sensitivity values are as follows:XmARROWS_SENSITIVEBoth arrows are sensitive.XmARROWS_DECREMENT_SENSITIVEOnly the decrement arrow (as determined byXmNlayoutDirection) is sensitive.XmARROWS_INCREMENT_SENSITIVEOnly the increment arrow (as determined byXmNlayoutDirection) is sensitive.XmARROWS_INSENSITIVEBoth arrows are insensitive.XmARROWS_DEFAULT_SENSITIVITYUse the sensitivity specified in theXmNdefaultArrowSensitivityresource.XmNdecimalPointsSpecifies the number of decimal places used when displaying the value
of a SpinBox numeric type child. If the number of decimal places specified
is greater than the number of digits in a displayed value, the value is padded
with 0 (zeros). For example, when`XmNinitialValue`is 1
andXmNmaximumValueis 1000 andXmNdecimalPointsis 3, the range of values displayed
in the SpinBox is 0.001 to 1.000. This is used only whenXmNspinBoxChildTypeisXmNUMERIC.XmNincrementValueSpecifies the amount by which to increment or decrement a SpinBox numeric
type child. This is used only whenXmNspinBoxChildTypeisXmNUMERIC.XmNmaximumValueSpecifies the highest possible value for a numeric SpinBox. This is
used only whenXmNspinBoxChildTypeisXmNUMERIC.XmNminimumValueSpecifies the lowest possible value for a numeric SpinBox. This is
used only whenXmNspinBoxChildTypeisXmNUMERIC.XmNnumValuesSpecifies the number of strings inXmNvalues. The application must change this value when strings are added
or removed fromXmNvalues. This
is used only whenXmNspinBoxChildTypeisXmSTRING.XmNpositionSpecifies the position of the currently displayed item. The interpritation
of`XmNposition`is dependent upon the value of the`XmNpositionType`resource.When`XmNpositionType`is`XmPOSITION_INDEX`the`XmNposition`value is interpreted as follows:
For`XmSpinBox`children of type`XmNUMERIC`,
the`XmNposition`resource is interpreted as an index into
an array of items. The minimum allowable value for`XmNposition`is 0. The maximum allowable value for`XmNposition`is(XmNmaximumValue-XmNminimumValue)/XmNincrementValue.
The value display by the`XmSpinBox`child isXmNminimumValue+(XmNposition*XmNincrementValue). For`XmSpinBox`children of type`XmSTRING`, the`XmNposition`resource is interpreted as an index into an array of`XmNnumValues`items. The minimum allowable value for`XmNposition`is 0. The maximum allowable value for`XmNposition`isXmNnumValues - 1. The value displayed by the`XmSpinBox`is the`XmNposition`'th value in the`XmNvalues`array.When`XmNpositionType`is`XmPOSITION_VALUE`the`XmNposition`value is interpreted as follows:For`XmSpinBox`children of type`XmNUMERIC`, the`XmNposition`resource is interpreted as
the actual value to be displayed. The minimum allowable value for`XmNposition`is`XmNminimumValue`. The maximum allowable
value for`XmNposition`is`XmNmaximumValue`.
The value displayed by the`XmSpinBox`child is`XmNposition`. For`XmSpinBox`children of type`XmSTRING`, the interpretation is the same for`XmPOSITION_VALUE`as for`XmPOSITION_INDEX`.Position values falling outside the specified range are invalid. When
an application assigns a value to`XmNposition`which is
less than the minimum,`XmNposition`is set to the minimum
and an error message is displayed. When an application assigns a value
to`XmNposition`which is greater than the maximum,`XmNposition`is set to the maximum and an error message is displayed.XmNpositionTypeSpecifies how values the`XmNposition`resource are
to be interpreted. Valid values include`XmPOSITION_INDEX`and`XmPOSITION_VALUE`.XmNspinBoxChildTypeSpecifies the type of data displayed in the child:XmNUMERICThe SpinBox choice range is defined by numeric minimum, maximum, and
incremental values.XmSTRINGThe SpinBox choices are alphanumeric.XmNvaluesSpecifies the array ofXmStrings
to be displayed in a SpinBox string type child. The application must changeXmNnumValueswhen strings are added to or removed
fromXmNvalues. This is used only
whenXmNspinBoxChildTypeisXmSTRING.Inherited ResourcesSpinBox inherits behavior and resources from the superclasses described
in the following tables. For a complete description of each resource, refer
to the reference page for that superclass.XmManager Resource
SetNameClassTypeDefaultAccessXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetdynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSGComposite Resource
SetNameClassTypeDefaultAccessXmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0GCore Resource SetNameClassTypeDefaultAccessXmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSGCallbackA pointer to the following structure is passed to each callback:typedef struct
{
        intreason;
        XEvent* event;
        Widgetwidget;
        Booleandoit;
        intposition;
        XmStringvalue;
        Booleancrossed_boundary;
} XmSpinBoxCallbackStruct;reasonIndicates why the callback was invoked. Reasons may be the following:XmCR_OKSpinning has stopped because the SpinBox arrow has been disarmed.XmCR_OKis either the last or only call.XmCR_SPIN_NEXTThe increment arrow has been armed and position is increasing. Further
callbacks will come. For a numeric type child, the values displayed are
approaching the maximum. For a string SpinBox, the values displayed are
approaching the last entry in the array ofXmStrings.XmCR_SPIN_PRIORThe decrement arrow has been armed and position is decreasing. Further
callbacks will come. For a numeric type child, the values displayed are
approaching the minimum. For a string type child, the values displayed
are approaching the first entry in the array ofXmStrings.XmCR_SPIN_FIRSTThe begin data (osfBeginData) key sequence has been
pressed. The SpinBox is at its first position, displaying the lowest value
or the first entry in the array ofXmStrings.XmCR_SPIN_LASTThe end data (osfEndData) key sequence has been pressed.
The SpinBox is at its last position, displaying the highest value or the
last entry in the array ofXmStrings.eventPoints to the`XEvent`that triggered this callback.widgetSpecifies the child widget affected by this callback.doitWhen the callback isXmNmodifyVerifyCallback,doitindicates whether or
not an action will be performed before the SpinBox position changes. If
the callback leavesdoitset to True (the
default), the spinning action is performed. If the callback setsdoitto False, the spinning action is not performed.
When the callback isXmNvalueChangedCallback,doitis ignored.positionSpecifies the next value of the SpinBox position (same asXmNposition). This is an output field for
theXmNmodifyVerifyCallback, which
may change the next position as dictated by the needs of an application.valueSpecifies the newXmStringvalue
in the text child widget. The user program must copy this string if it
is to be used outside the callback routine.crossed_boundarySpecifies whether or not the SpinBox has crossed the upper or lower
boundary (the last or first compound string, or the maximum or minimum value).
Thecrossed_boundaryvalue is True if the
SpinBox has just crossed a boundary, and False if it has not.TranslationsTheXmSpinBoxtranslations are as follows:The following key names are listed in the X standard key event translation
table syntax. This format is the one used by Motif to specify the widget actions
corresponding to a given key. A brief overview of the format is provided under &cdeman.VirtualBindings;. For a complete description
of the format, please refer to the X Toolkit Instrinsics Documentation.Btn1Down:SpinBArm()Btn1Up:SpinBDisarm():KeyosfUp:SpinBPrior():KeyosfDown:SpinBNext():KeyosfLeft:SpinBLeft():KeyosfRight:SpinBRight():KeyosfBeginData:SpinBFirst():KeyosfEndData:SpinBLast()AcceleratorsTheXmNacceleratorsresource
of a SpinBox are added to each traversable text child. The defaultXmNacceleratorsare defined in the following
list. The bindings forKeyosfUpandKeyosfDowncannot be changed.KeyosfUp:SpinBPrior()KeyosfDown:SpinBNext()KeyUposfUp:SpinBDisarm()KeyUposfDown:SpinBDisarm()KeyosfLeft:SpinBLeft()KeyosfRight:SpinBRight()KeyUposfLeft:SpinBDisarm()KeyUposfRight:SpinBDisarm()KeyosfBeginData:SpinBFirst()KeyosfEndData:SpinBLast()Action RoutinesTheXmSpinBoxaction routines are as follows:SpinBArm():Visually arms the SpinBox by drawing the armed arrow so that it appears
to be depressed. This action is initiated when the user pressesBtn1while the pointer is within the boundaries of either the increment
or decrement arrow. The arrow remains visually armed as long asBtn1remains depressed.If the time period specified byXmNrepeatDelayis not greater than zero milliseconds, nothing else happens
whileBtn1remains depressed.If the time period specified byXmNrepeatDelayis greater than zero milliseconds, and the arrow is disarmed
before the time period specified byXmNinitialDelayhas elapsed, nothing else happens in this action.If the time period specified byXmNrepeatDelayis greater than zero milliseconds, and the arrow is still
armed after the time period specified byXmNinitialDelayhas elapsed, the following occurs:Thereasonmember of the
SpinBox callback structure,XmSpinBoxCallbackStruct, is set toXmCR_SPIN_NEXTif the increment arrow is armed, or toXmCR_SPIN_PRIORif the decrement arrow is armed.Thepositionmember is set
to the next position.Thedoitmember is set to
True.XmNmodifyVerifyCallback,
if it exists, is invoked. The application may change the value ofpositionanddoit.
If the application setsdoitto False,
nothing else happens until theXmNrepeatDelayperiod has elapsed, or untilBtn1is released.Ifdoitremains set to True, the following
occurs:The value ofXmNpositionis changed to the value ofpositionin the
SpinBox callback structure.The text corresponding to the new position is displayed in
the traversable text child that currently has focus.Thereasonmember of the
SpinBox callback structure is set toXmCR_SPIN_NEXTif the increment arrow is armed, orXmCR_SPIN_PRIORif the decrement arrow is armed.Thepositionmember is set
to the current (new) value ofXmNposition.XmNvalueChangedCallback,
if it exists, is called. SpinBox ignores any changes topositionordoitmembers made byXmNvalueChangedCallback.These events are repeated each time theXmNrepeatDelayperiod elapses and the arrow remains armed.SpinBDisarm():Visually disarms the SpinBox by drawing the previously armed arrow
so that it no longer appears to be depressed.If the time period specified byXmNrepeatDelayis not greater than zero milliseconds, or the time period specified
byXmNinitialDelayhas not elapsed,
the following then occurs:Thereasonmember of the
SpinBox callback structure,XmSpinBoxCallbackStruct, is set toXmCR_SPIN_NEXTif the increment arrow is armed, or toXmCR_SPIN_PRIORif the decrement arrow is armed.Thepositionmember is set
to the next position.Thedoitmember is set to
True.TheXmNmodifyVerifyCallback,
if there is one, is invoked. The application may change the value ofpositionanddoit.
If the application setsdoitto False, nothing
else happens until theXmNrepeatDelayperiod has elapsed, or untilBtn1is released.Ifdoitremains set to True, the following
occurs:The value ofXmNpositionis changed to the value ofpositionin the
SpinBox callback structure.The text corresponding to the new position is displayed in
the traversable text child that currently has focus.Thereasonmember of the
SpinBox callback structure is set toXmCR_SPIN_NEXTif the increment arrow is armed, orXmCR_SPIN_PRIORif the decrement arrow is armed.Thepositionmember is set
to the current (new) value ofXmNposition.XmNvalueChangedCallback,
if it exists, is called. SpinBox ignores any changes topositionordoitmembers made by
anXmNvalueChangedCallback.If anXmNvalueChangedCallbackprocedure is issued after the button has been armed, regardless of the value
ofXmNrepeatDelayor whether theXmNinitialDelayhas expired:Thereasonmember of the
SpinBox callback structure is set toXmCR_OK.Thepositionmember is set
to the current value ofXmNposition.XmNvalueChangedCallback,
if it exists, is called.SpinBFirst():The following occurs:Thereasonmember of the
SpinBox callback structure,XmSpinBoxCallbackStruct, is set toXmCR_SPIN_FIRST.Thepositionmember is set
to the first (0) position.Thedoitmember is set to
True.XmNmodifyVerifyCallback,
if it exists, is invoked. The application may change the value ofpositionanddoit.
If the application setsdoitto False, nothing
else happens until theXmNrepeatDelayperiod has elapsed, or untilBtn1is released.Ifdoitremains set to True, the following
occurs:The value ofXmNpositionis changed to the value ofpositionin the
SpinBox callback structure.The text corresponding to the new position is displayed in
the traversable text child that currently has focus.Thereasonmember of the
SpinBox callback structure is set toXmCR_SPIN_FIRST.Thepositionmember is set
to the current (new) value ofXmNposition.XmNvalueChangedCallback,
if it exists, is called.Thereasonmember of the
SpinBox callback structure is set toXmCR_OK.Thepositionmember is set
to the current (new)XmNpositionvalue.TheXmNvalueChangedCallbackis called again. SpinBox ignores any changes topositionordoitmembers made byXmNvalueChangedCallback.SpinBLast():The following occurs:Thereasonmember of the
SpinBox callback structure,XmSpinBoxCallbackStruct, is set toXmCR_SPIN_LAST.Thepositionmember is set
to the last position.Thedoitmember is set to
True.XmNmodifyVerifyCallback,
if it exists, is invoked. The application may change the value ofpositionanddoit.
If the application setsdoitto False, nothing
else happens until theXmNrepeatDelayperiod has elapsed, or untilBtn1is released.Ifdoitremains set to True, the following
occurs:The value ofXmNpositionis changed to the value ofpositionin the
SpinBox callback structure.The text corresponding to the new position is displayed in
the traversable text child that currently has focus.Thereasonmember of the
SpinBox callback structure is set toXmCR_SPIN_LAST.Thepositionmember is set
to the current (new) valueXmNposition.XmNvalueChangedCallback,
if it exists, is called.Thereasonmember of the
SpinBox callback structure is set toXmCR_OK.Thepositionmember is set
to the current (new) ofXmNposition.XmNvalueChangedCallbackis called again. SpinBox ignores any changes to thepositionordoitmembers made byXmNvalueChangedCallback.SpinBLeft():If the VendorShell resourceXmNlayoutDirectionis left-to-right, theSpinBPrioraction is
invoked. Otherwise, theSpinBNextaction is invoked.SpinBNext():Visually arms the SpinBox by drawing the increment arrow so that it
appears to be depressed. The following occurs:Thereasonmember of the
SpinBox callback structure,XmSpinBoxCallbackStruct, is set toXmCR_SPIN_NEXT.Thepositionmember is set
to the next position.Thedoitmember is set to
True.XmNmodifyVerifyCallback,
if it exists, is invoked. The application may change the value ofpositionanddoit.
If the application setsdoitto False, nothing
else happens until theXmNrepeatDelayperiod has elapsed, or untilBtn1is released.Ifdoitremains set to True, the following
occurs:The value ofXmNpositionis changed to the value ofpositionin the
SpinBox callback structure.The text corresponding to the new position is displayed in
the traversable text child that currently has focus.Thereasonmember of the
SpinBox callback structure is set toXmCR_SPIN_NEXT.Thepositionmember is set
to the current (new) value ofXmNposition.XmNvalueChangedCallback,
if it exists, is called.Thereasonmember of the
SpinBox callback structure is set toXmCR_OK.Thepositionmember is set
to the current (new)XmNposition.TheXmNvalueChangedCallbackis called again. SpinBox ignores any changes topositionordoitmembers made byXmNvalueChangedCallback.SpinBPrior():Visually arms the SpinBox by drawing the decrement arrow so that it
appears to be depressed. The following occurs:Thereasonmember of the
SpinBox callback structure,XmSpinBoxCallbackStruct, is set toXmCR_SPIN_PRIOR.Thepositionmember is set
to the next position.Thedoitmember is set to
True.XmNmodifyVerifyCallback,
if it exists, is invoked. The application may change the value ofpositionanddoit.
If the application setsdoitto False, nothing
else happens until theXmNrepeatDelayperiod has elapsed, or untilBtn1is released.Ifdoitremains set to True, the following
occurs:The value ofXmNpositionis changed to the value ofpositionin the
SpinBox callback structure.The text corresponding to the new position is displayed in
the traversable text child that currently has focus.Thereasonmember of the
SpinBox callback structure is set toXmCR_SPIN_PRIOR.Thepositionmember is set
to the current (new) value ofXmNposition.XmNvalueChangedCallback,
if it exists, is called.Thereasonmember of the
SpinBox callback structure is set toXmCR_OK.Thepositionmember is set
to the current (new) value ofXmNposition.XmNvalueChangedCallbackis called again. SpinBox ignores any changes toposition