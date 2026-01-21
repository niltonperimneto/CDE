dtksh Convenience Functionsconvenience functionsThedtkshutility includes a file of convenience
functions. This file is itself a shell script containing shell functions
that may be useful to a shell programmer. The shell functions perform operations
thatdtkshprogrammers frequently have to do for themselves.
These include functions for quickly creating certain kinds of dialogs (help,
error, warning, and so on), a function for easily creating a collection of
buttons, and functions that make it easier to configure the constraint resources
for a child of a form widget. It is not a requirement that shell script
writers use these convenience functions; they are supplied to make it easier
for developers to write shorter and more readable shell scripts.Before a shell script can access these functions, it must first include
the file containing the convenience functions. The convenience functions
are located in the file/usr/dt/scripts/DtFuncs.sh.Use the following notation to include them in a shell script:. /usr/dt/lib/dtksh/DtFuncs.dtshDtkshAddButtonsDtkshAddButtonsDtkshAddButtonsadds one or more buttons of the same
kind into a composite widget. It is most often used to add a collection
of buttons into a menupane or menubar.Usage:DtkshAddButtons parent widgetClass label1 callback1
                [label2 callback2 ...]
DtkshAddButtons [-w] parent widgetClass variable1 label1 callback1 \
                [variable2 label2 callback2 ...]The-woption indicates that the convenience function
should return the widget handle for each of the buttons it creates. The
widget handle is returned in the specified environment variable. ThewidgetClassparameter can be set to any of the following, but it
defaults toXmPushButtonGadgetif nothing is specified.XmPushButtonXmPushButtonGadgetXmToggleButtonXmToggleButtonGadgetXmCascadeButtonXmCascadeButtonGadgetExamples:DtkshAddButtons $MENU XmPushButtonGadget Open do_Open Save do_Save
                Quit exit
DtkshAddButtons -w $MENU XmPushButtonGadget B1 Open do_Open B2 Save
                do_SaveDtkshSetReturnKeyControlsDtkshSetReturnKeyControlsDtkshSetReturnKeyControlsconfigures a text widget
within a form widget so that the Return key does not activate the default
button within the form, but instead moves the focus to the next text widget
within the form. This is useful if you have a window that contains a series
of text widgets, and the default button should not be activated until the
user presses the Return key while the focus is in the last text widget.Usage:DtkshSetReturnKeyControls textWidget nextTextWidget formWidget
                          defaultButtonThetextWidgetparameter specifies
the widget to be configured to catch the Return key and force the focus to
move to the next text widget (as indicated by thenextTextWidgetparameter). TheformWidgetparameter specifies the form containing the default button and
should be the parent of the two text widgets. ThedefaultButtonparameter indicates which component is to be treated
as the default button within the form widget.Examples:DtkshSetReturnKeyControls $TEXT1 $TEXT2 $FORM $OK
DtkshSetReturnKeyControls $TEXT2 $TEXT3 $FORM $OKDtkshUnder, DtkshOver, DtkshRightOf, and DtkshLeftOfDtkshUnderDtkshOverDtkshRightOfDtkshLeftOfThese convenience functions simplify the specification of certain classes
of form constraints. They provide a way of attaching a component to one
edge of another component. They are used when constructing the resource
list for a widget. This behavior is accomplished using theATTACH_WIDGETconstraint.Usage:DtkshUnder widgetId [offset]
DtkshOver widgetId [offset]
DtkshRightOf widgetId [offset]
DtkshLeftOf widgetId [offset]ThewidgetIdparameter specifies the
widget to which the current component is to be attached. Theoffsetvalue is optional and defaults to 0 if not specified.Example:XtCreateManagedWidget BUTTON4 button4 XmPushButton $FORM \
        labelString:&ldquo;Exit&ldquo; \
        $(DtkshUnder $BUTTON2) \
        $(DtkshRightOf $BUTTON3)DtkshFloatRight, DtkshFloatLeft, DtkshFloatTop, and DtkshFloatBottomDtkshFloatRightDtkshFloatLeftDtkshFloatTopDtkshFloatBottomThese convenience functions simplify the specification of certain classes
of form constraints. They provide a way of positioning a component, independent
of the other components within the form. As the form grows or shrinks, the
component maintains its relative position within the form. The component
may still grow or shrink, depending upon the other form constraints specified
for the component. This behavior is accomplished using theATTACH_POSITIONconstraint.Usage:DtkshFloatRight [position]
DtkshFloatLeft [position]
DtkshFloatTop [position]
DtkshFloatBottom [position]The optionalpositionparameter specifies
the relative position to which the indicated edge of the component is positioned.
Thepositionvalue is optional and defaults
to 0 if one is not specified.Example:XtCreateManagedWidget BUTTON1 button1 XmPushButton $FORM \
        labelString:&ldquo;Ok&ldquo; \
        $(DtkshUnder $SEPARATOR) \
        $(DtkshFloatLeft 10) \
        $(DtkshFloatRight 40)DtkshAnchorRight, DtkshAnchorLeft, DtkshAnchorTop, and DtkshAnchorBottomDtkshAnchorRightDtkshAnchorLeftDtkshAnchorTopDtkshAnchorBottomThese convenience functions simplify the specification of certain classes
of form constraints. They provide a way of attaching a component to one
of the edges of a form widget in such a way that, as the form grows or shrinks,
the component's position does not change. However, depending upon the other
form constraints set on this component, it may still grow or shrink in size.
This behavior is accomplished using theATTACH_FORMconstraint.Usage:DtkshAnchorRight [offset]
DtkshAnchorLeft [offset]
DtkshAnchorTop [offset]
DtkshAnchorBottom [offset]The optionaloffsetparameter specifies
how far from the edge of the form widget the component should be positioned.
If an offset is not specified, then 0 is used.Example:XtCreateManagedWidget BUTTON1 button1 XmPushButton $FORM \
        labelString:&ldquo;Ok&ldquo; \
        $(DtkshUnder $SEPARATOR) \
        $(DtkshAnchorLeft 10) \
        $(DtkshAnchorBottom 10)DtkshSpanWidth and DtkshSpanHeightDtkshSpanWidthDtkshSpanHeightThese convenience functions simplify the specification of certain classes
of form constraints. They provide a way of configuring a component so that
it spans either the full height or width of the form widget. This behavior
is accomplished by attaching two edges of the component (top and bottom forDtSpanHeight, and left and right forDtSpanWidth) to the form widget. The component typically resizes whenever
the form widget is resized. TheATTACH_FORMconstraint
is used for all attachments.Usage:DtkshSpanWidth [leftOffset rightOffset]
DtkshSpanHeight [topOffset bottomOffset]The optionaloffsetparameters specify
how far from the edges of the form widget the component should be positioned.
If an offset is not specified, then 0 is used.Example:XtCreateManagedWidget SEP sep XmSeparator $FORM \
                      $(DtkshSpanWidth 1 1)DtkshDisplayInformationDialog, DtkshDisplayQuestionDialog, DtDisplayWarningDialog, DtkshDisplayWorkingDialog, and DtkshDisplayErrorDialogDtkshDisplayInformationDialogDtkshDisplayQuestionDialogDtDisplayWarningDialogDtkshDisplayWorkingDialogDtkshDisplayErrorDialogThese convenience functions create a single instance of each of the
Motif feedback dialogs. If an instance of the requested type of dialog already
exists, then it is reused. The parent of the dialog is obtained from the
environment variable$TOPLEVEL, which should be set
by the calling shell script, and then should not be changed. The handle
for the requested dialog is returned in one of the following environment
variables:_DTKSH_ERROR_DIALOG_HANDLE_DTKSH_QUESTION_DIALOG_HANDLE_DTKSH_WORKING_DIALOG_HANDLE_DTKSH_WARNING_DIALOG_HANDLE_DTKSH_INFORMATION_DIALOG_HANDLEIf you are attaching your own callbacks to the dialog buttons, do not
destroy the dialog when you are done with it. Unmanage the dialog, so that
it can be used again at a later time. If it is necessary to destroy the
dialog, then be sure to clear the associated environment variable so the
convenience function does not attempt to reuse the dialog.Usage:DtkshDisplay<name>Dialog title message [okCallback closeCallback
                        helpCallback dialogStyle]The Ok button is always managed, and by default unmanages the dialog.
The Cancel and Help buttons are only managed when a callback is supplied
for them. ThedialogStyleparameter accepts
any of the standard resource settings supported by the associated bulletin
board resource.Example:DtkshDisplayErrorDialog &ldquo;Read Error&ldquo; &ldquo;Unable to read the file&ldquo;
                 &ldquo;OkCallback&ldquo; \
                 &ldquo;CancelCallback&ldquo; &ldquo;&ldquo; DIALOG_PRIMARY_APPLICATION_MODALDtkshDisplayQuickHelpDialog and DtkshDisplayHelpDialogDtkshDisplayQuickHelpDialogDtkshDisplayHelpDialogThese convenience functions create a single instance of each of the
help dialogs. If an instance of the requested type of help dialog already
exists, then it is reused. The parent of the dialog is obtained from the
environment variable$TOPLEVEL, which should be set
by the calling shell script, and then should not be changed. The handle
for the requested dialog is returned in one of the following environment
variables:_DTKSH_HELP_DIALOG_HANDLE_DTKSH_QUICK_HELP_DIALOG_HANDLEIf it is necessary to destroy a help dialog, then be sure to clear
the associated environment variable so that the convenience function does
not attempt to reuse the dialog.Usage:DtkshDisplay*HelpDialog title helpType helpInformation [locationId]The meaning of the parameters is dependent upon the value specified
for thehelpTypeparameter. Their meanings
are:helpType=HELP_TYPE_TOPIChelpInformation= help volume namelocationId= help
topic location IDhelpType=HELP_TYPE_STRINGhelpInformation= help stringlocationId= <not
used>helpType=HELP_TYPE_DYNAMIC_STRINGhelpInformation= help stringlocationId= <not
used>helpType=HELP_TYPE_MAN_PAGEhelpInformation= manual page namelocationId= <not
used>helpType=HELP_TYPE_FILEhelpInformation= help file namelocationId=
<not used>Example:DtkshDisplayHelpDialog &ldquo;Help On Dtksh&ldquo; HELP_TYPE_FILE
                       &ldquo;helpFileName&ldquo;