dtpdmuser cmddtpdmPrint Dialog Manager to provide
printer-specific GUIsdtpdm-vdisplayvdpy-windowvwid-pdisplaypdpy-pcontextpcidDESCRIPTIONdtpdmis the Print Dialog Manager (PDM) that is
invoked by the Print Dialog Manager Daemon (dtpdmd) to
provide printer-specific GUIs on behalf of a printing application. It is
a process separate from the print server.At an application&rsquo;s requestdtpdmposts to
the user&rsquo;s display a set of printer-specific dialogs that enable the
user to configure a variety of printer options.Thedtpdmprogram provides a setup dialog to X printing
applications that allows the user to set printer specific and job specific
options. The setup dialog appears to be part of the application, but it is
actually managed by thedtpdmon behalf of the application.
It is capable of providing dialogs in all locales for which there exist applicable
message catalogs.dtpdmpresents a dialog containing
the printer name and description plus anXmNotebookwidget. This widget contains two tabs: one for the Printer
Setup Box and one for the Job Setup Box. Each of these boxes provides
controls that allow for configuration of various printing options.Thedtpdmdialog also contains three pushbuttons
labelledOK,Cancel, andHelp. When the user pressesOK,dtpdmdismisses the dialog and sets the newly configured printing options
in the current print context (viaXpSetAttributes).
When the user pressesCancel,dtpdmdismisses the dialog and makes no changes to the print context.Printer Setup BoxThe Printer Setup box presents options specific to the currently selected
printer. The options presented may vary in other PDM implementations. The
printer setup options presented bydtpdmare as follows:Printer InformationProvides information about the X Printer. The information consists of the
printer model and the document
format used to generate documents sent to this X Printer.Page OrientationSpecifies how the output will be oriented on the page. The orientation options
presented in the menu depend on the printer, but up to four orientations
are possible: portrait, landscape, reverse portrait, and reverse landscape.
An icon adjacent to theOptionsmenu presents a graphical
illustration showing the current selection.Printed SidesSpecifies single or double-sided printing. The actual choices available depend
on the printer, but up to three choices are possible: simplex, duplex, and
tumble. An icon adjacent to theOptionsmenu presents
a graphical illustration showing the current selection.TraySpecifies the printer tray from which the media will be drawn. TheAuto-selecttray option is available
for all printers. Selecting this
option indicates no preference as to which tray to use. Remaining Tray selections
are dependent on the printer.Page SizeSpecifies the media size for printing. The entries in this list box depend on
whether theLoaded in PrinterorAll Sizesradio button is selected.Loaded in PrinterProvides a list of the media sizes currently available on the printer. If the
currentTrayoption isAuto-select,
the user will see all media sizes available in all of the printer&rsquo;s
trays. If a specific tray is selected, only the media size loaded in that
tray will be shown. Information on which media size is available in which
tray is provided by the system administrator via theinput-trays-mediumattribute.
If the system administrator does not provide this information,
theLoaded in Printerradio button will be inactive.All SizesProvides a list of all supported media sizes available for the printer. If the
user selects this button, theTrayoption is set toAuto-select. This button is provided for the following situations:If the system administrator has not specified
which sizes are loaded in the printerIf a desired media size is not loaded and the printer
prompts for the requested sizeIf the output never reaches an actual printer (for
example, when printing to a file)Job Setup BoxThe Job Setup box presents options specific to the spooler controlling
the printer. The options presented may vary in other PDM implementations,
depending on the spooler. The job setup options presented bydtpdmare as follows:Send Mail When DoneInstructsdtpdmto send an email message to the user
when the job is completed.Banner Page TitleSpecifies the text the user wants to appear on the banner page of the
output.Print Command OptionsSpecifies command line options and arguments that the user wants to
pass to the spooler.dtpdmperforms no parsing of this
field. All parsing and argument validation is performed by the underlying
spooler.OPTIONSThe following options are available:-vdisplayvdpySpecifies the display connection to the Video X-Server.-windowvwidSpecifies the window id on the Video X-Server to which the PDM&rsquo;s
dialogs should be posted as transient windows.-pdisplaypdpySpecifies the display connection to the Print X-Server.-pcontextpcidSpecifies the print context id on the Print X-Server. The PDM uses
this id to gain access to the print context being used by the requesting
application.ENVIRONMENT VARIABLESThe Dt Print Dialog Manager uses the environment variableLANGto identify
the location of its localized message file.RESOURCESNone.ACTIONS/MESSAGESNone.RETURN VALUESThe return values (defined as integer constants inDt/dtpdmd.h) are as follows:PDM_EXIT_OKThe PDM is telling the PDMD that the user selectedOKto dismiss the PDM.PDM_EXIT_CANCELThe PDM is telling the PDMD that the user selectedCANCELto dismiss the PDM.DM_EXIT_VXAUTHThe PDM is telling the PDMD that it did not have proper authority to
make a display connection on the Video X-Server.PDM_EXIT_PXAUTHThe PDM is telling the PDMD that it did not have proper authority to
make a display connection to the Print X-Server.PDM_EXIT_ERRORThe PDM is telling the PDMD that it encountered an error.all other valuesThe PDMD treats all unknown return values the same asPDM_EXIT_ERROR. Such return values
are likely from uncontrollable exit conditions
often found in other libraries (for example, untrapped XIO errors from libX).ERRORS/WARNINGSNone.FILESNone.EXAMPLESNone.SEE ALSOdtpdmd1