# WML
file formats`WML`The widget meta-language file format for creating uil compilerswidget meta-languageWML
## DESCRIPTION


The widget meta-language facility (WML) is used to generate the
components of the user interface language (UIL) compiler that
can change depending on the widget set. Using WML you can add support
in UIL for
new widgets to the Motif widget set or for a totally new widget set.
### File


WML files are ASCII files that you can modify with any standard
text editor. They are accessed in the`tools/wml`directory by WML.
By convention WML files have the suffix`&npzwc;.wml`.
The Motif widget set is described in the`motif.wml`file.
This is also the default WML file when using the WML facility.

When adding new widgets or changing widget characteristics, you should
start with a copy of the`motif.wml`file.
If you are creating a new widget set for use with UIL, you should
start from scratch.
In either case the`motif.wml`file is a good example of WML syntax, and you should familiarize
yourself with it before writing your own WML file.

WML files have a simple syntax, similar in structure to UIL.
It is made up of the following elements:

Comments

Data Type Definitions

Character Set Definitions

Enumeration Set Definitions

Control List Definitions

Class Definitions

Child Definitions

Resource Definitions

You can use space, tabs, or newlines anywhere in the syntax,
as long as you do not split up keywords or strings, except that
comments end at a newline.
The order of elements is not important to the syntax.

This description uses the following additional conventions
to describe the syntax of the widget meta-language:

* **[&ensp;&ensp;]** 

Indicates optional elements.
* **...** 

Indicates where an element of syntax can be repeated.
* **|** 

Indicates a choice among multiple items.

### Comments


You can include comments in the WML file.
Comments have the following syntax:[any.element]!any.comment

Comments begin with an exclamation point and extend to the
end of the line. A comment can begin on a line by itself or
follow any part of another element. A comment does not change
the meaning of any other element.
For example:!This is a comment
!  that spans two lines.
DataType    !This is a comment following code.
### Data Type Definitions


Data type definitions register all the resource data
types used in the file. You must register all the data types used
in your WML file.
Data type definitions have the following syntax:DataType
     any.datatype [{ InternalLiteral = internal.name |
          DocName = "`string`"; [...]}];
     [...]

A data type definition begins with the keyword`DataType`. Following
the`DataType`keyword is a list of data types that can be further modified with

* **`InternalLiteral`** 

This forces the value of the internal symbol table literal definition
of the data type name. This modifier is only used to get around
symbol table definitions hard coded into the UIL compiler.
It should rarely be used.
* **`DocName`** 

This gives an arbitrary string for use in the documentation.
This string is meant to supply a different name for the data type for
use in the documentation, or a single name for the data type if the
data type has aliases.


For example:DataType OddNumber {DocName="OddNumber";};
         NewString;
### Character Set Definitions


Character set definitions register the Motif Toolkit name and other
information for the character set names used in UIL.
Character set definitions have the following syntax:CharacterSet
     any.character.set
          { [ FontListElementTag | XmStringCharsetName ] = "`string`";
               [ Alias = "`string`" ...; |
               Direction = [ LeftToRight | RightToLeft ]; |
               ParseDirection = [ LeftToRight | RightToLeft ]; |
               CharacterSize = [ OneByte | TwoByte ]; ]
               [ ... ] };
     [ ... ]

A character set definition begins with the keyword`CharacterSet`.
Following the`CharacterSet`keyword is a list of character sets
that can be further modified with

* **`FontListElementTag`&ensp;|&ensp;`XmStringCharsetName`** 

Specifies the name of the character set, which will become the character
set component of a compound string segment created using this
character set.
This modifier is required.
* **`Alias`** 

Specifies one or more aliases for the character set name.
Each alias can be used within UIL to refer to the same character set.
* **`Direction`** 

Specifies the direction of a compound string segment created using
this character set.
The default is`LeftToRight`.
* **`ParseDirection`** 

Specifies the direction in which an input string is parsed when a
compound string segment is created using this character set.
The default is whatever`Direction`is specified.
* **`CharacterSize`** 

Specifies the number of bytes in each character of a compound string
segment created using this character set.
The default is`OneByte`.


For example:CharacterSet
  iso_latin1
    { XmStringCharsetName = "ISO8859-1";
      Alias = "ISOLatin1"; };
  iso_hebrew_lr
    { XmStringCharsetName = "ISO8859-8";
      Alias = "iso_latin8_lr";
      Direction = RightToLeft;
      ParseDirection = LeftToRight; };
  ksc_korean
    { XmStringCharsetName = "KSC5601.1987-0";
      CharacterSize = TwoByte; };
### Enumeration Set Definitions


Enumeration set definitions register the named constants used in the
Motif Toolkit to specify some resource values.
Enumeration set definitions have the following syntax:EnumerationSet
     resource.name: resource.type
          { enum.value.name; [ ... ] };

An enumeration set definition begins with the keyword`EnumerationSet`.
For each enumeration set defined, the name and type of the resource are
listed.
The resource name is the Motif Toolkit resource name, with the beginning`XmN`removed and with the initial letter capitalized.
For example, the name of the Motif Toolkit resource`XmNrowColumnType`is`RowColumnType`.
The resource type is the data type for the resource; for most resources,
this is`integer`.
Following the resource name and type is a list of names of enumeration
values that can be used as settings for the resource.
These names are the same as those in the Motif Toolkit.

For example:EnumerationSet
  RowColumnType: integer
    { XmWORK_AREA; XmMENU_BAR; XmMENU_POPUP;
      XmMENU_PULLDOWN; XmMENU_OPTION; };

Enumeration sets also support Boolean values.
### Control List Definitions


Control list definitions assign a name to groups of controls.
You can use these control lists later in class definitions to simplify
the structure of your WML file.
Control list definitions have the following syntax:ControlList
     any.control.list [{ any.control; [...]}];

A control list definition starts with the`ControlList`keyword.
Following the`ControlList`keyword are any number of control list definitions. Control list
definitions are made up of a control list name followed by the
set of controls it represents. For example:ControlList
        Buttons {PushButton;
                 RadioButton;
                 CascadeButton;
                 NewCascadebutton;};

Each control specified in the control list must be defined as
a class in the file.
### Class Definitions


Class definitions describe a particular widget class including
its position in the class hierarchy, toolkit convenience function,
resources, and controls. There should be one class definition for
each widget or gadget in the widget set you want to support in UIL.
Class definitions have the following syntax:Class class.name: MetaClass | Widget | Gadget
     [{[
     SuperClass = class.name; |
     ParentClass = parent.class.name; |
     InternalLiteral = internal.name; |
     Alias =`alias`; |
     ConvenienceFunction = convenience.function; |
     WidgetClass = widget.class; |
     DocName = "`string`"; |
     DialogClass = True | False; |
     Resources { any.resource.name [{
               Default = new.default.value; |
               Exclude = True |
               False;
               [...]} ];
          [...]}; |
     Controls { any.control.name; [...]};
     Children { any.child.name; [...] };
     [...]
     ]}];

Class definitions start with the`Class`keyword.
For each class defined, the name of the class and whether the
class is a metaclass, widget, or gadget is listed.
Each class definition can be further modified with the
keywords described in the following list.

* **`SuperClass`** 

This indicates the name of the parent class.
Only the root of the hierarchy does not specify a SuperClass.
* **`ParentClass`** 

This indicates the name of the widget's automatically created
parent class if one exists. This allows resources for that
automatically created class to be used in instances of this class.
For example,`XmBulletinBoardDialog`creates both an`XmBulletinBoard`and an`XmDialogShell`. To access the
resources of the`XmDialogShell`parent class it must be
specified here.
* **`InternalLiteral`** 

This forces the value of the internal symbol table literal definition
of the class name. This modifier is only used to get around
symbol table definitions hard coded into the UIL compiler.
It should rarely be used.
* **`Alias`** 

This indicates alternate names for the class
for use in a UIL specification.
* **`ConvenienceFunction`** 

This indicates the name of the creation convenience function
for this class. All widget and gadget classes must have a`ConvenienceFunction.`
* **`WidgetClass`** 

This indicates the associated widget class of gadget type classes.
Presently, nothing is done with this value.
* **`DocName`** 

This defines an arbitrary string for use in the documentation.
Presently, nothing is done with this value.
* **`DialogClass`** 

This indicates whether the class is a dialog class.
Presently, nothing is done with this value.
* **`Resources`** 

This lists the resources of the widget class. This keyword
can be further modified with

* **`Default`** 

This specifies a new default value for this resource. Resource
default values are usually set in the resource definition. If
an inherited resource's default value is changed by the class,
the new default value should be noted here.
* **`Exclude`** 

This specifies whether an inherited resource should be excluded from the
resource list of the class.`Exclude`is False by default.

* **`Children`** 

This lists the names of the automatically created children of this
class, so that those children can be accessed in the UIL file.
* **`Controls`** 

This lists the controls that the widget class allows. The controls can
be other classes or a control list from the control list definition.


The following example uses the examples from the data type definitions and
control list definitions above.Class
     TopLevelWidget: MetaClass
          {
          Resources
               {
               XtbNfirstResource;
               XtbNsecondResource;
               };
          };
     NewWidget: Widget
          {
          SuperClass = TopLevelWidget;
          ConvenienceFunction =
              XtbCreateNewWidget;
          Resources
               {
               XtbNnewResource;
               XtbNfirstResource
                  {Default="XtbNEW_VALUE";};
               XtbNsecondResource
                  {Exclude=True;};
               };
          Controls
               {
               NewWidget;
               Buttons;
               };
          };
### Child Definitions


Child definitions register the classes of automatically created
children. Automatically created children are referenced elsewhere in
auilfile using the`Children`keyword within a class definition.
Child definitions have the following syntax:

`Child`child.name`:`class.name`;`[...]

Wherechild.nameis the name of the automatically created child
andclass.nameis the name of the class of that child.
### Resource Definitions


Resource definitions describe a particular resource including
its type, and default value.
There should be a resource definition for
each new resource referenced in the class definitions.
Resource definitions have the following syntax:Resource
     resource.name: Argument | Reason | Constraint | SubResource
          [{[
          Type =`type`;
          [ResourceLiteral = resource.literal; ]
          [InternalLiteral = internal.name; ]
          [Alias =`alias`; ]
          [Related =`related`; ]
          [Default =`default`; ]
          [DocName = doc.name; ]
          [...]}]
     [...]

Resource definitions start with the`Resource`keyword.
For each resource definition,
the name of the resource and whether the resource is an argument, reason,
constraint or subresource is listed.

* **`Argument`** 

Indicates a standard resource
* **`Reason`** 

Indicates a callback resource
* **`Constraint`** 

Indicates a constraint resource
* **`SubResource`** 

Presently, nothing is done with this value


The resource definition can be further modified with the following
keywords:

* **`Type`** 

This indicates the data type of the resource. It must be listed
in the data type definition.
* **`ResourceLiteral`** 

This indicates the keyword used in the UIL file to reference the
resource. In Motif, the resource name is the same as the`ResourceLiteral`.
* **`InternalLiteral`** 

This forces the value of the internal symbol table literal definition
of the resource name. This modifier is only used to get around
symbol table definitions hard coded into the UIL compiler.
It should rarely be used.
* **`Alias`** 

This indicates alternate names for the resource
for use in a UIL specification.
* **`Related`** 

This is a special purpose field that allows resources that
act as a counter for the current resources to be related to the resource.
UIL automatically sets the value of this related resource to the number of items
in the compiled instance of typeresource.name.
* **`Default`** 

This indicates the default value of the resource.
* **`DocName`** 

This defines an arbitrary string for use in the documentation.
Presently, nothing is done with this value.


The following example uses the examples from the data type definitions,
control list definitions and class definitions above.Resource
     XtbNfirstResource: Argument
          { Type = OddNumber;
            Default = "XtbOLD_VALUE";};
     XtbNsecondResource: Argument
          { Type = NewString;
            Default = "XtbNEW_STRING"; };
     XtbNnewResource: Argument
          { Type = OddNumber;
            Default = "XtbODD_NUMBER"; };