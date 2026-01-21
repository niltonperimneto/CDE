# XmSpinBoxValidatePosition
library call`XmSpinBoxValidatePosition`translate the current value of the specified XmSpinBox child
into a valid position#include <Xm/SpinBox.h>int`XmSpinBoxValidatePosition`Widgettextfieldint*position
## DESCRIPTION


The`XmSpinBoxValidatePosition`function
is a utility that can be used by applications wanting to
implement a policy for tracking user modifications to editableXmSpinBoxchildren of type`XmNUMERIC`. The specifics of when and how the user's
modifications take effect is left up to the application.
`text_field`The`text_field`argument specifies the
widget ID of the child of the`XmSpinBox`that is
being modified. The requirement on`text_field`is that it holds the`accessTextual`trait (already a requirement for
children of`XmSpinBox`). This way,`XmSpinBox`can extract the string out of the`text_field`widget (even if it is not an`XmTextField`).`position`The location pointed to by the position argument is assigned the result
of the translation done by`XmSpinBoxValidatePosition`.`XmSpinBoxValidatePosition`first checks to make sure this is an`XmNUMERIC``XmSpinBox`child. If it is not,`XSmpinBoxValidatePosition`sets position to the current
position and returns`XmCURRENT_VALUE`.


`XmSpinBoxValidatePosition`attempts to translate the input string to a floating point number. If
this translation fails,`XmSpinBoxValidatePosition`sets position to the current position and
returns`XmCURRENT_VALUE`.

`XmSpinBoxValidatePosition`converts the floating point number to an integer using the`XmNdecimalPoints`resource. Extra decimal places are truncated. The resulting integer is range
checked to make sure it falls within the valid range defined by`XmNminimumValue`and`XmNmaximumValue`inclusive. If the input falls outside this range,`XmSpinBoxValidatePosition`sets position to the nearest limit and returns either`XmMINIMUM_VALUE`or`XmMAXIMUM_VALUE`.

Finally,`XmSpinBoxValidatePosition`checks the integer to make sure it belongs to the series
defined by`XmNminimumValue ... XmNminumumValue + ((n - 1) * XmNincrementlValue)`. If
the integer does not belong to this series,`XmSpinBoxValidatePosition`sets position to the
nearest element which is less than or equal to the integer and returns`XmINCREMENT_VALUE`.

Otherwise,`XmSpinBoxValidatePosition`assigns the integer to position and returns`XmVALID_VALUE`.
## RETURN VALUE


The`XmSpinBoxValidatePosition`function returns the status of the validation.
The set of possible values returned is as follows:
`XmCURRENT_VALUE`

Cannot convert, returning current position_value.`XmMINIMUM_VALUE`

Less than min.`XmMAXIMUM_VALUE`

More than max.`XmINCREMENT_VALUE`

Not on increment.`XmVALID_VALUE`

Okay.

## EXAMPLES


This first example demonstrates how theXmSpinBoxValidatePositionfunction could be used from inside anXmNmodifyVerifyCallbackcallback installed on theXmSpinBoxor theXmSimpleSpinBox:/*
 * Install a callback on a spin box arrow press.
 */
  XtAddCallback(sb, XmNmodifyVerifyCallback, ModifyVerifyCB, NULL);
  XtAddCallback(simple_sb, XmNmodifyVerifyCallback, ModifyVerifyCB, NULL);

with the callback doing:void ModifyVerifyCB(widget, call_data, client_data) {
    XmSpinBoxCallbackStruct *cbs = (XmSpinBoxCallbackStruct*) call_data;
    int position;
    Widget textual = NULL;
    if (XtIsSubclass(w, xmSimpleSpinBoxWidgetClass))
    {
        Arg args[1];
        XtSetArg(args[0], XmNtextField, &textual);
        XtGetValues(w, args, 1);
    }
    else if (XtIsSubclass(w, xmSpinBoxWidgetClass))
      textual = cbs->widget;
    else
      textual = (Widget) NULL;

    ...

    if (XmSpinBoxValidatePosition(textual, &position) == XmCURRENT_VALUE)
      XBell(XtDisplay(w), 0);
    else
      cbs->position = position;
}This second example demonstrates how theXmSpinBoxValidatePositionfunction could be used from inside anXmNactivateCallbackcallback installed on
theTextFieldchild of theXmSpinBox:/*
 * Install a callback on a spin box arrow press.
 */
XtAddCallback(tf, XmNactivateCallback, ModifyVerifyChildCB, NULL);with the callback doing:void ModifyVerifyChildCB(widget, call_data, client_data) {
    int     position;
    Widget  textual = widget;
    Arg     args[1];

    if (XmSpinBoxValidatePosition (textual, &position) == XmCURRENT_VALUE)
      XBell(XtDisplay(widget), 0);

    /* Set the position constraint resource of the textfield */

    XtSetArg(args[0], XmNposition, position);
    XtSetValues(textual, args, 1);
}SEE ALSO&cdeman.XmSpinBox;,
&cdeman.XmCreateSpinBox;