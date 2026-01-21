PrefaceWho Should Use This BookUse this book if you are a programmer interested in integrating an existing
application into the Common Desktop Environment (CDE), or in developing a
new application that uses the features and functionality of CDE. This book
describes the CDE development environment, and assumes that you are familiar
with Motif&reg;, X, UNIX&reg;, or C programming.Before You Read This BookThe Common Desktop Environment:Programmer's Guideis a collection of programming information. The manuals listed in the sectionshould be read before you begin integration
of any applications to CDE.TheCommon Desktop Environment: Programmer's Overviewprovides a description of CDE and introduces the programming environment.How This Book Is OrganizedTheCommon Desktop Environment: Programmer's Guidehas two parts. Each part provides a detailed description of each element
of the Common Desktop Environment, a conceptual diagram, and a task-oriented
description of how to use each element, complete with code examples.Part 1 &ndash;introduces how to register your application
and printing levels.describes the steps involved with the
basic integration of an existing application into CDE.Part 2 &ndash;introduces
how to integrate existing applications into the Common Desktop Environment.describes how to use generic standard font descriptions
to ensure that you get the closest matching font for your application on
any CDE-compliant system.describes a common model for presenting
information and error messages.describes the ICCM session management
protocol and provides examples of how to integrate your application with
Session Manager.describes the drag-and-drop user model,
the new drag-and-drop application program interface (API), and how to use
drag and drop.Part3 &ndash;describes how to integrate new applications
with the Session Manager and with drag and drop. It also explains how locales
affect the Login Manager, Window Manager, and the terminal emulator.describes how to integrate your application
with the Workspace Manager in specialized ways.describes how to use the custom widgets
that are provided as part of CDE.describes how to create actions within
your application.describes the data-typing functions and
how to use the data-typing database.introduces the Calendar API, including
functions, data structures, calendar attributes, and entry attributes. It
also describes how to use the Calendar API.Chapter 11 explains how to integrate
your application with thedtinfoon-line documentation browser; this chapter
also summarizes the DtInfo database engine API, which you may use to write
your own browser.Chapter 12 explains the CDE printing widgets and APIs.Glossaryis a list of words and
phrases found in this book and their definitions.Related BooksBefore beginning integration of your application into CDE, you should
become familiar with the other books in the documentation set. Seefor a list of the companion books.The run-time environment documentation set consists of:documentation setrun-timerun-timedocumentation setCommon Desktop Environment:
User's GuideCommon Desktop Environment: Advanced
User's and System Administrator's GuideOnline help volumesTheAdvanced User's and System Administrator's Guidecontains information to help you integrate an application into the desktop.For more information about the Calendaring and Scheduling API, contact
the X.400 API Association for the latest copy of theXAPIA Specification. The address is X.400 API Association, 800 El Camino Real, Mountain
View, California, 94043.Development Environment DocumentationThis section provides an overview of each manual&mdash;except for theProgrammer's Guide&mdash;in the developer documentation set. In
addition to theProgrammer's Guide, the development
environment documentation set consists of:documentation
setdevelopment environmentCommon Desktop Environment:
Style Guide and Certification ChecklistCommon Desktop Environment: Application
Builder User's GuideCommon Desktop Environment: Programmer's
OverviewCommon Desktop Environment: Help System
Author's and Programmer's GuideCommon Desktop Environment: ToolTalk
Messaging OverviewCommon Desktop Environment: Internationalization
Programmer's GuideCommon Desktop Environment: Desktop Korn
Shell User's GuideCommon Desktop Environment:
GlossaryOnline man pagesCommon Desktop Environment: Programmer's OverviewTheCommon Desktop Environment: Programmer's Overviewhas two parts. Part 1 contains an architectural overview of the Common Desktop
Environment, including high-level information on both the run-time and development
environments. Part 2 contains information useful to know before developing
an application, and describes the development environment components.TheCommon Desktop Environment: Programmer's Overviewprovides a high-level view of the Common Desktop Environment development
environment and the developer documentation set. Read this book first before
starting application design and development.Common Desktop Environment: Style Guide and Certification ChecklistTheCommon Desktop Environment: Style Guide and Certification
Checklistprovides application design style guidelines and the
list of requirements for Common Desktop Environment application-level certification.
These requirements consist of the Motif Version 1.2 requirements with Common
Desktop Environment-specific additions.The checklist describes keys using a model keyboard mechanism. It assumes
that your application is being designed for a left-to-right language environment
in an English-language locale. Wherever keyboard input is specified, the
keys are indicated by the engravings on the Motif model keyboard. Mouse buttons
are represented using a virtual button mechanism to specify behavior independent
of the number of buttons on the mouse.This book provides information to assist the application designer in
developing consistent applications and behaviors within the applications.Common Desktop Environment: Application Builder User's GuideThe Common Desktop Environment Application Builder (also calledApp Builder) is an interactive tool for developing Common Desktop
Environment applications. AppBuilder provides features that facilitate both
the construction of an application graphical user interface (GUI) and the
incorporation of the desktop's many useful desktop services (such as Help,
ToolTalk, and Drag and Drop). TheCommon Desktop Environment: Application
Builder User's Guideexplains how to create an interface by dragging
and dropping &ldquo;objects&rdquo; from a palette. The guide also explains
how to make connections between objects in the interface, use the application
framework editor to easily integrate desktop services, generate C code, and
add application code to the App Builder output to produce a finished application.Common Desktop Environment: Help System Author's and Programmer's
GuideTheCommon Desktop Environment: Help System Author's and Programmer's
Guidedescribes how to develop online help for application software.
It covers how to create help topics and integrate online help into a Motif
application.The audience for this book includes:Authors who design, create,
and view online help informationDevelopers who want to create software applications
that provide a fully integrated help facilityThis book has four parts. Part1 describes the collaborative role that
authors and developers undertake to design application help. Part 2 provides
information for authors organizing and writing online help. Part 3 describes
the Help System application programmer's toolkit. Part 4 contains information
for both authors and programmers about preparing online help for different
language environments.Common Desktop Environment: ToolTalk Messaging OverviewTheCommon Desktop Environment: ToolTalk Messaging Overviewdescribes the ToolTalk components, commands, and error messages
offered as convenience routines to enable your application to conform to
Media Exchange and Desktop Services message set conventions. This manual
is for developers who create or maintain applications that use the ToolTalk&reg;
service to interoperate with other applications.TheToolTalk Messaging Overviewdoesnotdescribe general ToolTalk functionality. For detailed information
about the ToolTalk service, refer toThe ToolTalk Service: An Inter-Operability
Solution. For tips and techniques to help make using ToolTalk
easier, readToolTalk and Open Protocols: Inter-Application Communication.Common Desktop Environment: Internationalization Programmer's GuideTheCommon Desktop Environment: Internationalization Programmer's
Guideprovides information for internationalizing an application
so that it can be easily localized to support various languages and cultural
conventions in a consistent user interface.Specifically, this guide:Provides guidelines and hints
for developers on how to write applications for worldwide distribution.Provides an overall view of internationalization
topics that span different layers within the desktop.Provides pointers to references and more detailed
documentation. In some cases, standard documentation is referenced.This guide is not intended to duplicate the existing reference or conceptual
documentation, but rather to provide guidelines and conventions on specific
internationalization topics. It focuses on internationalization topics and
not on any specific component or layer in an open software environment.Common Desktop Environment: Desktop Korn Shell User's GuideTheCommon Desktop Environment: Desktop Korn Shell User's
Guidedescribes how to create Motif applications with Desktop
Korn Shell (dtksh) scripts. It contains several example
scripts of increasing complexity, in addition to the basic information a
developer needs to get started.This guide is intended for developers who find a shell-style scripting
environment suitable for a particular task. It assumes a knowledge of Korn
Shell programming, Motif, the Xt Intrinsics, and, to a lesser extent, Xlib.Common Desktop Environment: GlossaryTheCommon Desktop Environment: Glossaryprovides
a comprehensive list of terms used in the Common Desktop Environment. The
Glossary is the source and reference base for all users of the desktop. Because
the audience for this glossary consists of many different types of users&mdash;from
end users to developers to translators&mdash;the format for a glossary definition
may include information about the audience, where the term originated, and
the Common Desktop Environment component that uses the term in its graphical
user interface.What DocBook SGML Markup MeansThis book is written in the Structured Generalized Markup
Language (SGML) using the DocBook Document Type Definition (DTD).
The following table describes the DocBook markup used for
various semantic elements.DocBook SGML MarkupMarkup AppearanceSemantic Element(s)ExampleAaBbCc123The names of commands.Use thelscommand to list files.AaBbCc123The names of command options.Usels&minus;ato list all files.AaBbCc123Command-line placeholder:
replace with a real name or value.To delete a file, typermfilename.AaBbCc123The names of files and
directories.Edit your.loginfile.AaBbCc123Book titles, new words or terms, or
words to be emphasized.Read Chapter 6 inUser's
Guide.
These are calledclassoptions.
Youmustbe root to do this.