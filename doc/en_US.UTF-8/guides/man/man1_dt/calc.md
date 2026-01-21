dtcalcuser cmddtcalcThe CDE
Calculatordtcalc-a accuracy-m mode-b numeric_base-notation
display_notation-trig trigonometric_type-no_menu_bar-session session_file-?DESCRIPTIONThedtcalcutility is a calculator for use within
the CDE. It provides an easy-to-use interface designed to give access to common
arithmetic and financial calculations.The calculator is designed to operate in much the same way as many
hand-held calculators. It provides three modes of operation: scientific, financial,
and logical. The default operation is scientific, but with the easy-to-use
GUI, changing to the modes of operation is easy. When the operation mode is
changed, a number of the keys change for the new operations.OPTIONSThedtcalcutility defines a number of command-line
options that allow the user to configure how the calculator displays itself.
Command-line options have a higher precedence than resources. By using command-line
options a user can override anything specified in a resource file.-a<accuracy>This is the initial number of digits displayed after the numeric point.
This value must be in the range 0 to 9. The default value is 2.-m<mode>This determines which mode the calculator will display itself in. The
possible values for <mode> are: scientific,
financial, or logical. Scientific is the default mode. Some of the calculator
keys change operations when the calculator's mode is changed.-b<numeric_base>This determines which numeric base the calculator will use when it does
calculations. There are four bases the calculator supports: binary (base 2),
octal (base 8), decimal (base 10), or hexadecimal (base 16). The possible
values for <numeric_base> are: binary, octal, decimal,
or hexadecimal. The default is decimal.-notation<display_notation>This determines how the answers are to be displayed on the calculator.
The possible values for <display_notation> are: scientific,
engineering, or fixed. The default is fixed.-trig<trigonometric_type>This determines how answers are presented when the calculator is in
scientific mode. The possible values for <trigonometric_type> are: degrees, radians, or gradients. The default is degrees.-no_menu_barThis option makes the calculator come up with no menubar.-session<session_file>The dtcalc utility runs with the session file specified in thesession_fileparameter. Session files are generated as a dtcalc
session shuts down.-?This prints out the usage message.RESOURCESThe calculator supports a number of resources which make it much more
configurable. Following is the list of supported resources and their default
values.Client Resource SetNameClassTypeDefaultpostMenuBarPostMenuBarBooleanTrueaccuracyAccuracyint2baseBasestringdecimaldisplayNotationDisplayNotationstringfixedmodeModestringscientifictrigTypeTrigTypestringdegreesDtcalc*postMenuBar:Specifies whether the menu bar should appear or not.Dtcalc*accuracy:Specifies whether the menu bar should appear or not.Dtcalc*base:This resource allows the user to change the default for the numeric
base the calculator uses when it does its calculations. The default is "decimal"
which is base 10. Possible values are:binary(orbin): do calculations in base 2.octal(oroct): do calculations in base 8.decimal(ordec): do calculations in base 10.hexadecimal(orhex): do calculations in base 16.Dtcalc*display:This resource allows the user to change the default for the way answers
are displayed on the calculator. The default is "fixed". Possible values are:fixed(orfix): display in fixed mode.scientific(orsci): display in scientific mode.engineering(oreng): display in engineering
mode.FILES/usr/dt/bin/dtcalcThis is the executable for the CDE Calculator./usr/dt/app-defaults/<LANG>/DtcalcThis file includes the application defaults for the CDE Calculator.