dtcmuser cmddtcmThe CDE
Calendar Manager.dtcm-vview-ccalendar-pprinterDESCRIPTIONdtcmis the CDE appointment and resource scheduling
tool. Use it to:- Display day, week, month, and year views of your calendar- Schedule single or repeating calendar entries- Browse and edit another user's calendar- Schedule reminders to give you notice of events- Restrict access to your calendar- Print high-quality hardcopy- View and Schedule entries for a group of calendars- Change the time zone context- Announce appointments via electronic mail- Schedule appointments received in electronic mailOPTIONSdtcmaccepts all of the standard X Toolkit command
line options as well as the following:-c calendarSpecifies the calendar to display. The default value is equivalent
to $USER@$HOST.-p printerSpecifies the default printer. The default is the system's default
printer.-v viewSpecifies the initial view. Values can be "day", "week", "month" or
"year".CONCEPTSAcalendaris a persistent entity existing somewhere
on the network, which contains scheduling data for auser.
Acalendar entryis an event recorded within the context
of a calendar. Calendar entries supported by dtcm areappointmentandto-do. Calendar entries can besingle, orrepeatentries. A calendar entry
may have one or moreremindersassociated with it. A
reminder causes dtcm to notify you by issuing analarmwhen the system time enters your specifiedreminder notice periodbefore the event. When viewing your calendar, you have the notion
of thecurrent date, which is the date you last selected
by selecting the Day View, or by clicking in any of the other views. The
current date does not change until you select another date, even if you have
navigated out of a view that would contain the current date.dtcm presents anetwork calendar model. To operate
on a calendar belonging to some user on the network, dtcm establishes a session
with acalendar server. The calendar server is (conceptually
at least) a separate process running on the host where the calendar is located.
It manages all the calendars for the host on which it is running, and can
service multiple connected applications simultaneously. The calendar server
responds to authentication, session control and calendar transactions initiated
by dtcm. In addition dtcm responds to events occurring at the server, such
as updates caused by other connected applications instances.USAGEdtcm has a single main window, containing a graphical calendar. You
can choose to view day, week, month or year in the main window. Navigating
between the views is achieved by selecting the view from theViewmenu, or by clicking one of the navigation buttons displayed in
the current view.MenuBarFileMenuPrint Current ViewPrints hard-copy based on the currently displayed calendar view.Print...Brings up a Print Setup window that allows you to select the report
type to use for the currently selected view, as well as various printing
options. You can specify a from/to date range, the printer name, and the
number of copies. You can set other more generic options, such as margins
and footers, by clicking on theMore...button.Options...Brings up default options for all of dtcm's global attributes.ExitTerminates the dtcm application.EditMenuAppointment...Brings up the appointment editor, described below.ToDo...Brings up the to-do editor, described below.Properties...Brings up the properties dialog for appointments and to-do entries.
This item is for properties of calendar data entities. For properties of
the dtcm application, use theProperties...entry in
theFilemenu.ViewMenuDayChanges the current view in the main window to Day View.WeekChanges the current view in the main window to Week View.MonthChanges the current view in the main window to Month View.YearChanges the current view in the main window to Year View.Appointment List...Brings up theAppointment Listdialog.ToDo List...Brings up theToDo Listdialog.Find...Brings up theFinddislog, which you can use to
locate calendar entries by specifying some search criteria.Go to Date...Brings up theGo to Datedialog, which allows you
to change the view to a specified date. This is a convenient way to get to
dates that are distant from the current date.BrowseMenuShow Other Calendar...Brings up a dialog to let you connect to a different calendar than the
one currently displayed, in the main window. You will still be displaying
a single calendar.Compare Calendars...Brings up theCompare Calendarsdialog, described
below.Menu Editor...Brings up a dialog that allows you to add frequently used calendars
to theBrowsemenu for this and future sessions with
dtcm.<user>@<host>...This sequence of entries consists of your own calendar, followed by
an optional list of calendars that you can add to the menu using theMenu Editor...option described above. Your own calendar always
appears first. The other options are listed in alphabetical order.SecondaryWindowsIn addition to the main window, dtcm has several secondary windows,
which give you access to the scheduling and browsing features of dtcm.TheAppointment Editorallows scheduling of appointments.
An appointment is the most common type of calendar entry. It is useful for
scheduling time-slots in your calendar, and can be exported to other users
either by direct entry to their calendars, or through electronic mail. To
invoke the appointment editor, select it from theSchedulemenu in the main window, or double-click anywhere in the graphical calendar
view.TheTo Do Editorallows you to maintain a list
of to-do items for your personal use. To-do entries are not visible to other
dtcm users who are browsing your calendar; they are private to you. To-do
entries differ from appointments in that they do not necessarily appear as
scheduled events in your calendar views. If they have aDue Dateassociated with them, you will see that on your calendar view.
The main purpose of to-do entries is to allow you to maintain a list of work
items, without necessarily allocating calendar time for them. Invoke the
to-do editor from by selecting it form theSchedulemenu
in the main window.TheGroup Appointment Editorallows you to schedule
an appointment on multiple calendars at once. Invoke the group appointment
editor by clickingSchedulein theCompare
Calendarswindow. You may optionally announce the appointment
over electronic mail.TheCompare Calendarswindow allows you to connect
to several calendars simultaneously, and get a graphical overview of busy
and available time in the resultant "virtual calendar". Invoke the compare
calendars window by selecting it from theBrowsemenu
in the main window.ThePrint Setup Boxwindow allows you to select
the report type to use for the currently selected view. In addition to selecting
the view information, you can set a number of generic and printer-specific
printing options. For example, you can send the output to a file or a printer.
In the case of printed output, you can specify how many copies you want.
You can also access another window to set options specific to the printer/spooler
you are using. For example, you can select paper size, one- or two-sided
printing, and email notification on completion of the print job.TheOptionswindow, accessible from theFilemenu, gives you access to the dtcm options that you can configure.
There are several categories of options: Editor Defaults; Display Settings;
Access List and Permissions; Printer Settings; Date Format. Set the options
to suit your requirements, and save them by clickingApply.RESOURCESdtcm supports a number of application resources to allow you to configure
its behaviour. The application class name for dtcm isDtcm. To set application resources, you can copy the system default
version of this file from /usr/dt/app-defaults/<LANG>/Dtcm to a personal
version, typically ~/app-defaults/Dtcm, and edit it with your changes.
Following is the list of supported resources and their default values.Application ResourcesNameClassTypeDefaultlabelFontLabelFontXmRFontList(varies)viewFontViewFontXmRFontList(varies)bold FontBoldFontXmRFontList(varies)iconFontIconFontXmRFontList-dt-application-bold-r-normal-sansapplicationFontFamilyApplicationFontFamilyXmRXmStringapplicationdtcm*labelFontSpecifies the font to use for the labels in the calendar's views.dtcm*viewFontSpecifies the font to use for the text of the appointments in the calendar's views.dtcm*boldFontSpecifies the font to use for the time ranges in the week view.dtcm*iconFontSpecifies the icon font.dtcm*applicationFontFamilySpecifies the font family name to use for the text of the appointments in
the calendar's views. A font is used with this family name and an appropriate
size to match the system font size chosen viadtstyle.dtcm*viewFontanddtcm*boldFonthave a higher precedent thandtcm*applicationFontFamily.FILES/usr/dt/bin/dtcmThis is the executable for dtcm./usr/dt/app-defaults/<LANG>/DtcmThis is the system-default application defaults file for dtcm./usr/dt/bin/rpc/cmsdThis is the calendar daemon (server) that manages calendars on a machine./var/spool/calendar/callog.<user>This is the persistent calendar database for a user on this machine.