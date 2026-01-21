Running Localized SessionsYou can customize the desktop user interface in many different languages.
Various elements such as screens, default languages, fonts, input (keyboard)
methods, and icons can be changed. In addition, the menus, online help, and
error messages can be localized and are available in multiple languages.language-specificsessionssessionlanguage-specificDefault Language at LoginThe messages and menus in the initial login window are displayed in
the default language. If the default language has not been set up, the messages
and menus are displayed using a generic &ldquo;C&rdquo; locale environment.
You can change the language from the Options menu in the login screen.Logging In to a Language-Specific Sessionlanguage-specificloginlogging into language-specific sessionlogin, language-specificLogging in to a specific language through the desktop is easy. However,
specific hardware requirements such as keyboards and printers may be required
to make the localized session more usable. These requirements vary by language,
character set, and country. Software and fonts can further increase the effective
localization of your system. To log in to a language-specific session:Use the Options menu in the login screen to select
a language.The list of languages includes all the supported languages.Log in normally with your name and password.Creating or Editing a Language-Specific Filelanguage-specificdataYou can create, edit, and print language-specific files. You can also
give files language-specific names. However, for system administration files
that are shared across a network, the file names should just contain ASCII
characters. Different systems on the network might be using different locales.If you have logged in to the desktop in a specific language, all applications
will be invoked using that language. However, you can still invoke an application
with another language.If you want to create a file with a different language, invoke a new
instance of Text Editor specifying your desired language.To Create or Edit a Language-Specific Filecreatingfile with language-specific characterseditingfile with language-specific charactersfileswith language-specific characters[fileslanguage-specificcharacters in fileTo create or edit a language-specific file, you can
either start the Text Editor with the language directly
or you can set
theLANGenvironment variable prior to starting the Editor.To start the Text Editor with the language directly, invoke thedtpadcommand with the language
specified for the-xnllanguageoption. For example:/usr/dt/bin/dtpad -xnllanguageJapanese_localenameTo setLANGbefore invoking
the Editor, perform the following steps:In a Terminal window, set the LANG environment
variable to the desired language. For example, to set the locale to Japanese,
you can type:Terminalsetting language throughLANG environment variableLANG=Japanese_localenamewhereJapanese_localenamespecifies the Japanese character set. Refer to your specific
platform to determine the value forJapanese_localename.In the same window, invoke the Text Editor (dtpad) under the desired language by typing:Text Editorstarting with specific languagestartingText Editorwith specific languagelanguage-specificText Editor/usr/dt/bin/dtpad &You can now enter Japanese characters if the locale specific files have
been installed. You can also use the Text Editor session to edit a previously
created Japanese file.Seefor an example of specifying a font set.Using a Language-Specific Terminal EmulatorThe following example usesdttermand
will start a Japanese terminal emulator. It assumes that the default
language is not Japanese, that you are using the Korn shell, and that
the locale specific files have been installed.terminal emulatorlanguage-specificlanguage-specificterminal emulatorstartingterminal emulator with specific languageFrom a command line in a Korn shell Terminal window type:LANG=Japanese_localenamedttermwhereJapanese_localenamespecifies the Japanese character set. Refer to your specific
platform to determine the value forJapanese_localename.Specifying Fontsfontsinternationalizinginternationalizationand fontsThe user usually changes fonts using Style Manager, which in turn restarts
Workspace Manager, resetting the desktop fonts. You can also customize fonts
at the command line or in resource files. In an internationalized environment,
the user must specify fonts that are independent of the code set. This is
necessary because the specification can be used under various locales with
different code sets than the character set (charset)
of the font. Therefore, all font lists should be specified with
a font set.Font SpecificationAfont specificationwithin a fontlist can be
either an X Logical Function Description (XLFD) name or an alias for the
XLFD name. For example, the following are valid font specifications for
a 14-point font:font specificationXLFD-dt-interface system-medium-r-normal-serif-*-*-*-*-p-*-iso8859-1Or,-*-r-*-14-*iso8859-1Font Set SpecificationThefont set specificationwithin a fontlist is a list of XLFD names
or their aliases (sometimes called abase name list).
The names are separated by semi-colons, and any blank space before or after
the semi-colon is ignored. Pattern- matching (wildcard) characters can be
specified to help shorten XLFD names.font set specificationbase name font listA font set specification is determined by the locale that is running.
For example, the Japanese locale defines three fonts (character sets) necessary
to display all of its characters. The following example identifies the set
of Mincho fonts needed.Sample name list with character
set:-dt-interface system-medium-r-normal-serif-*-*-*-*-p-*-14;
-dt-mincho-medium-r-normal--14-*-*-m-*-jisx0201.1976-0;
-dt-mincho-medium-r-normal--28-*-*-*-m-*-jisx0208.1983-0:Sample single pattern name without character set:-dt-*-medium-*-24-*-m-*:The preceding two cases can be used with a Japanese locale as long as
there are fonts that match the base name list.Changing FontsYou can change the fonts ofdttermby using either of the following methods:Specifying fonts from the command
lineSpecifying fonts within a resource fileTo Specify Fonts from the Command LineTo change the fonts for the menu from the command line, type:fontsspecifying from the command linedtterm -xrm '*fontList:fontset'wherefontsetis a font set specification. A font
set specification can be specified by a full X Logical Font Description (XLFD)
name list, a simple XLFD pattern, or an alias name. Note that a font set
specification is determined by the locale that is running.ExamplesTo use a larger font except for the menu font, type:dtterm -xrm '*fontList:-dt-interface user-medium-r-normal-l*-*-*-*:'To use a smaller font except for the menu font, type:dtterm -xrm '*fontList:-dt-interface user-medium-r-normal-s*-*-*-*:'These specifications will work for any locale.fontsspecifying from the command line <$endrange>To Specify Fonts within a Resource FileWhile it is possible to set fonts by editing application resource files in the/usr/dt/app-defaults/languagedirectory, this practice is not recommended. Such files are automatically
overwritten at each new installation. Rather, you should set fonts by adding
the resources to your personal`HomeDirectory`/.Xdefaultsfile.Choosing Your Input Method and KeyboardEach locale has a single default input method associated with it. If
the user does nothing, this default is selected. Because there may be many
input methods installed at any one time, the following sections explain how
various input methods are selected on behalf of the user.In addition to using resources to set the input method and the input method
style for preediting, you can use the Style Manager'sIntl'(Internationalization) control to set these values interactively.
For details, refer to theCDE Advanced User's
and System Administrator's Guide.Using Input Method ModifiersWhen there is more than one input
method for a locale, use theXmNinputMethodresource to identify the one you would like used. This is done by specifying
amodifier. The modifier must be of the following
form, wheremodifieris the name used to
uniquely identify the input method:input method for localesinputMethod :@im=modifierThemodifierstring specified in theXmNinputMethodresource is used to choose which input method
is used.Alternatively, set the XMODIFIERS environment variable. The syntax
is the same as for theXmNinputMethodresource, but values
are not. Values for XMODIFIERS are vendor specific.XMODIFIERS environment variable<Filename | Command>XmNinputMethod <Default Para Font> resourceSpecifying the Input Method StyleThe input method style determines how pre-editing will occur. It is
controlled by the`XmNpreeditType`resource. The syntax, possible values, and default value type of the`XmNpreeditType`resource are:`XmNpreeditType`Syntaxvalue[,value,....]Possible valuesOverTheSpot, OffTheSpot, OnTheSpot, Root, NoneDefault valueOnTheSpot, OverTheSpot, OffTheSpot, RootThe string list, separated by a comma, specifies the priority order
for this resource. The first value supported by the input method is used.For more information, see theInternationalization Programmer's
Guide.input method for locales <$endrange>