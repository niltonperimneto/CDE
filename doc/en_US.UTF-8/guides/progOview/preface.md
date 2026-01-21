PrefaceTheCommon Desktop Environment: Programmer's Overviewprovides a high-level view of the Common Desktop Environment development
environment and the developer documentation set. Read this book first before
starting application design and development.In this manual, the terms(the) Common Desktop EnvironmentandthedesktopCommon Desktop Environmentdesktopare used interchangeably.Outside of the Preface, this manual omits theCommon Desktop
Environmentprefix when referring to a Common Desktop Environment
development or run-time environment manual. For example, theCommon
Desktop Environment: Programmer's Overviewis referred to as theProgrammer's Overview.Who Should Use This BookRead theProgrammer's Overviewif you are:An application developer who
wants to develop a new Common Desktop Environment application, or integrate
an existingMotifMotif&reg;orOPEN LOOKOPEN
LOOK&reg;application into the desktopA manager, architect, or project lead interested
in designing a project involving applications that will run on the Common
Desktop Environment desktopFor the remainder of this manual, Motif is referred to as Motif&reg;.How This Book Is OrganizedThe Programmer's Overview is divided into two parts. Part I contains
an architectural overview of the Common Desktop Environment, including high-
level information on both the run-time and development environments. Part
II contains information useful to know before developing an application,
and describes the development environment components.This section provides brief descriptions of the chapters and appendixes
contained in this manual.,presents an overview of the Common Desktop Environment
architecture.,discusses information you should know
about the environment before you start to develop an application.,presents information specific to developing a Common
Desktop Environment application, such as naming conventions and guidelines
to follow.,discusses issues pertaining to writing portable and
maintainable applications.,summarizes how to make your application
launch-integrated (that is, started by double-clicking an icon on the desktop).,provides overviews of all components and guidelines
that you should use so your application has the same look and feel as, and
interoperates well with, other Common Desktop Environment desktop applications.,provides overviews of the components to incorporate
into your application as needed for added functionality.,lists in alphabetical order all development
environment components and guidelines, with associated library, header files,
and documentation.Related BooksFor information onMotifrelated
documentationMotif, see:Motif Programmer's
Guide, Release 1.2, by Open Software Foundation, 11 Cambridge
Center, Cambridge, MA 02142, published by PTR Prentice Hall, Englewood Cliffs,
NJ 07632Motif Programmer's Reference,
Release 1.2, by Open Software Foundation, 11 Cambridge Center, Cambridge,
MA 02142, published by PTR Prentice Hall, Englewood Cliffs, NJ 07632Motif Reference Guide,
by Douglas A. Young, published by PTR Prentice Hall, Englewood Cliffs, NJ
07632Motif 1.2 Style Guide,
by Open Software Foundation, 11 Cambridge Center, Cambridge, MA 02142, published
by PTR Prentice Hall, Englewood Cliffs, NJ 07632TheCommon Desktop Environment: Style Guide and Certification
Checklistis an extension of theMotif 1.2 Style
Guideto the Common Desktop Environment.OSF Application Environment
Specification (AES) User Environment Volume, Revision C, by Open
Software Foundation, 11 Cambridge Center, Cambridge, MA 02142, published
by PTR Prentice Hall, Englewood Cliffs, NJ 07632Motif 1.2 IEEE Std 1295 standard, which you can order from: IEEE Service
Center, 445 Hoes Lane, P.O. Box 1331, Piscataway, NJ 08855For information on POSIXPOSIX,
see the IEEE Std 1003.1-1990 standard, which you can order from: IEEE Service
Center, 445 Hoes Lane, P.O. Box 1331, Piscataway, NJ 08855For information onXlib documentationXlib, see:Xlib Programming Manualfor Version 11 (Volume 1) by Adrian Nye, published
by O'Reilly & Associates, Sebastopol, CA, 95472Xlib Reference Manualfor Version 11 (Volume 2), published by O'Reilly & Associates, Sebastopol,
CA, 95472For information onXt documentationXt,
see:X Toolkit Intrinsics
Programming Manual, (Volume 4) by Adrian Nye and Tim O'Reilly,
published by O'Reilly and Associates, Sebastopol, CA 95472.X Toolkit Intrinsics Reference Manual,(Volume 5) edited by Tim O'Reilly, published by O'Reilly and
Associates, Sebastopol, CA 95472.For more information onToolTalk documentationToolTalk&reg;, see:The ToolTalk Service:
An Inter-Operability Solution, published by SunSoft Press and
PTR Prentice Hall, Englewood Cliffs, NJ 07632, ISBN 0-13-088717-XToolTalk and Open Protocols: Inter-Application
Communication, by Astrid Julienne and Brian Holtz, published by
SunSoft Press and PTR Prentice Hall, Englewood Cliffs, NJ 07632, ISBN 013-031055-7In addition to theProgrammer's Overview, thedocumentation setdevelopment environmentdevelopment environment documentation set consists of:Common Desktop Environment:
Style Guide and Certification ChecklistCommon Desktop Environment: Application
Builder User's GuideCommon Desktop Environment: Programmer's
GuideCommon Desktop Environment: Help System
Author's and Programmer's GuideCommon Desktop Environment: ToolTalk
Messaging OverviewCommon Desktop Environment: Internationalization
Programmer's GuideCommon Desktop Environment: Desktop KornShell
User's GuideCommon Desktop Environment: Information
System Author's and Programmer's GuideCommon Desktop Environment:
GlossaryOnline man pagesFor more information on these development environment books, see the
following section,.The run-time environment documentation set consists of:documentation setrun-timerun-timedocumentation setCommon Desktop Environment:
User's GuideCommon Desktop Environment: Advanced
User's and System Administrator's GuideOnline help volumesTheAdvanced User's and System Administrator's Guidecontains information to help you integrate an application into the desktop.Development Environment DocumentationThis section provides an overview of each manual&mdash;except for theProgrammer's Overview&mdash;in the developer documentation
set.Common Desktop Environment: Style Guide and Certification ChecklistTheCommon Desktop Environment: Style Guide and Certification
Checklistprovides application design style guidelines and the
list of requirements for Common Desktop Environment application-level certification.
These requirements consist of the Motif requirements with Common Desktop
Environment-specific additions.The checklist describes keys using a model keyboard mechanism. It assumes
that your application is being designed for a left-to-right language environment
in an English-language locale. Wherever keyboard input is specified, the
keys are indicated by the engravings on the Motif model keyboard. Mouse buttons
are described using a virtual button mechanism to better describe behavior
independent from the number of buttons on the mouse.This book provides information to assist the application designer in
developing consistent applications and behaviors within the applications.Common Desktop Environment: Application Builder User's GuideThe Common Desktop Environment Application Builder (also called App
Builder) is an interactive tool for developing Common Desktop Environment
applications. It provides features that facilitate both the construction
of an application graphical user interface (GUI) and the incorporation of
the desktop's many useful desktop services (such as Help, ToolTalk, Drag
and Drop). TheCommon Desktop Environment: Application Builder
User's Guideexplains how to create an interface by dragging and
dropping &ldquo;objects&rdquo; from a palette. It also explains how to
make connections between objects in the interface, how to use the application
framework editor to easily integrate desktop services, how to generate C
code, and how to add application code to the App Builder output to produce
a finished application.Common Desktop Environment: Programmer's GuideTheCommon Desktop Environment: Programmer's Guidehas two parts. Each part provides a detailed description of elements of the
Common Desktop Environment, a conceptual diagram, and a task-oriented description
of how to use each element, complete with code examples.Part I, &ldquo;Recommended Integration,&rdquo; provides an overview
of basic integration, and describes how to integrate new applications with
the Session Manager, fonts, and drag and drop. It also discusses displaying
error messages.Part II, &ldquo;Optional Integration,&rdquo; describes how to integrate
new applications with the Workspace Manager, Common Desktop Environment Motif
widgets, actions, data types, and Calendar.TheProgrammer's Guideprovides an introduction
to the application program interfaces (APIs) for the components referred
to in the descriptions of Parts I and II above, with cross-references to
the relevant man pages. Details are covered in the man pages.Common Desktop Environment: Help System Author's and Programmer's
GuideTheCommon Desktop Environment: Help System Author's and Programmer's
Guidedescribes how to develop online help for application software.
It covers how to create help topics and how to integrate online help into
a Motif application.The audience for this book includes:Authors who design, create,
and view online help informationDevelopers who want to create software applications
that provide a fully integrated help facilityThis book has four parts. Part I describes the collaborative role that
authors and developers undertake to design application help. Part II provides
information for authors organizing and writing online help. Part III describes
the Help System application programmer's toolkit. Part IV contains information
for both authors and programmers about preparing online help for different
language environments.Common Desktop Environment: ToolTalk Messaging OverviewTheCommon Desktop Environment: ToolTalk Messaging Overviewdescribes the ToolTalk components, commands, and error messages
offered as convenience routines to enable your application to conform to
Media Exchange and Desktop Services message set conventions. This manual
is for developers who create or maintain applications that use the ToolTalk
service to interoperate with other applications.TheToolTalk Messaging Overviewdoesnotdescribe general ToolTalk functionality. For detailed information
about the ToolTalk service, refer toThe ToolTalk Service: An Inter-Operability
Solution. For tips and techniques to help make using ToolTalk
easier, readToolTalk and Open Protocols: Inter-Application Communication. Both of these books are listed in.Common Desktop Environment: Internationalization Programmer's GuideTheCommon Desktop Environment: Internationalization Programmer's
Guideprovides information for internationalizing an application
so that it can be easily localized to support various languages and cultural
conventions in a consistent user interface.Specifically, this guide:Provides guidelines and hints
for developers on how to write applications for worldwide distributionProvides an overall view of internationalization
topics that span different layers within the desktopProvides pointers to reference and more detailed
documentation. In some cases, standard documentation is referenced.This guide is not intended to duplicate the existing reference or conceptual
documentation, but rather to provide guidelines and conventions on specific
internationalization topics. It focuses on internationalization topics and
not on any specific component or layer in an open software environment.Common Desktop Environment: Desktop KornShell User's GuideTheCommon Desktop Environment: Desktop KornShell User's Guidedescribes how to create Motif applications with Desktop KornShell
(dtksh) scripts. It contains several example scripts of
increasing complexity, in addition to the basic information a developer needs
to get started.This guide is intended for developers who find a shell-style scripting
environment suitable for a particular task. It assumes a knowledge of KornShell
programming, Motif, the Xt Intrinsics, and, to a lesser extent, Xlib.Common Desktop Environment: Information System Author's
and Programmer's GuideTheCommon Desktop Environment: Information System
Author's and Programmer's Guidedescribes how to
develop and integrate on-line documentation. This manual is addressed
to two audiences: the documentation author, who
develops the documentation content; and the programmer,
who integrates the documentation with the CDE documentation
browser,dtinfo.Common Desktop Environment: GlossaryTheCommon Desktop Environment: Glossaryprovides
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