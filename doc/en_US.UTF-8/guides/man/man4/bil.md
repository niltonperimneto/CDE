# BIL
special fileBILBuilder Interface Language for the CDE Application Builder`:bil-version`major minor`:project`project_name`(`project_attributes[`:connection``(`connection_attributes`)`]`)``:module``module_name``(``:element`element_name`(`element_attributes[children]`)`[`:connection``(`connection_attributes`)`]`)`
## DESCRIPTION


BIL, the Builder Interface Language, is a specialized representation syntax
designed to meet the intermediate storage requirements of the CDE
Application Builder.
It describes the overall attributes of the application (also known as a
"project"), the modules that it contains, the elements that
comprise modules, and the behavioral relationships between objects
("connections").

BIL is not a compiled language.
It is read and written as data by the CDE Application Builder and its
associated code generator.
Even so, the format of BIL is printable ASCII characters with a
"human readable" nature, making it possible for users to examine
the contents of a BIL-formatted file and to process a BIL file
using simple ASCII string-oriented tools and techniques.

Two distinct types of BIL files are recognized by the CDE Application
Builder.
One type is known as aproject file, and is used to describe overall attributes of the project (application)
as well as identify the modules that comprise it.
It will also contain any cross-module information, such as connections
that bind together objects in different modules.

A second type of BIL file exists for each module and is known as amodule file. It contains all module-specific information, including definition of
the elements that comprise the module and connections that exist within
it.

BIL is structured in an object-oriented fashion.
Objects are defined and named, and sets of parenthesis are used to
enclose declarations of all the object's attributes.
Object types recognized by BIL include project, module, element
and connection.

* **:bil-version major minor** 

Version information is useful to the CDE Application Builder and
code generators in determining how to handle BIL files that may
date from earlier releases.
* **:project project_name** 

The`:project`directive names the project (application).
It encloses a declaration ofproject_attributesas well as a list of any cross-moduleconnectionsthat may exist within the project.
* **:module module_name** 

The`:module`directive names a module.
It encloses a declaration of all`elements`that comprise the
module as well as a list of anyconnectionsthat may exist within the module.
* **:element element_name** 

The`:element`directive names an element.
It encloses a declaration of allelement_attributes,
including a list of anychildrenthat may be part of the
element.
* **:connection** 

The`:connection`directive names a connection and
specifies its attributes.
It can occur as part of aprojector a`module`.
* **:children** 

The`:children`directive appears as an attribute of any
element which contains ("parents") other
elements.
It encloses a list of all child elements,
referencing each by its element name.

Project, element and connection attributes take the form:`:`attribute_nameattribute_valueWhereattribute_nameis an attribute appropriate for the project,
element or connection, andattribute_valueis typed according to the
attribute.
Attribute types include integer, string, object name, and keyword, where
the keyword is one of a fixed list used by BIL to identify specific
data values (e.g. :true, :none).
Some attribute values are ordered N-tuples, in which case the
complete set is enclosed in parentheses.

Comments may be placed in a BIL file by starting a line with two slashes (//).
Any comments that occur in a BIL file prior to the`:bil-version`line will be preserved by the
CDE Application Builder
across successive updates to the file.
Comments that occur after the`:bil-version`line will not be retained.

## RETURN VALUE


None.
## EXAMPLES


The following is an example of a project file::bil-version    1 0
:project todotool
(
    :files      (main_window.bil task_dialog.bil)

:connection
(
    :from       main_window.add_item_btn
    :to task_dialog.task_dialog
    :when       :activate
    :action-type        :builtin
    :action     :show
    :arg-type   :void
)
)

A typical module file would contain::bil-version    1 0
:module task_dialog
(

:element        task_dialog
(
    :type       :dialog
    :x  649
    :y  499
    :width      354
    :height     57
    :bg-color   "white"
    :label      "Adding a New Task"
    :resizable  :true
    :has-footer :false
    :visible    :false
    :children   (
        ctrl_panel
        activate_panel)
)
:element        ctrl_panel
(
    :type       :container
    :container-type     :relative
    :x  0
    :y  0
    :width      354
    :height     57
    :visible    :true
    :border-frame       :none
    :north-attachment   (:obj task_dialog 0)
    :south-attachment   (:obj task_dialog 0)
    :east-attachment    (:obj task_dialog 0)
    :west-attachment    (:point 0 0)
    :children   (
        task)
)
:element        task
(
    :type       :text-field
    :text-type  :alphanumeric
    :x  28
    :y  6
    :width      -1
    :height     -1
    :border-frame       :none
    :label-type :string
    :label      "Task:"
    :label-position     :west
    :num-columns        32
    :max-length 80
    :read-only  :false
    :active     :true
    :visible    :true
    :north-attachment   (:point 0 6)
    :south-attachment   (:none 0 0)
    :east-attachment    (:none 0 0)
    :west-attachment    (:point 0 28)
)
:element        activate_panel
(
    :type       :container
    :container-type     :activate
    :x  -1
    :y  -1
    :width      -1
    :height     -1
    :visible    :true
    :border-frame       :etched-in
    :children   (
        ok_button
        cancel_button
        help_button)
)
:element        ok_button
(
    :type       :button
    :button-type        :push-button
    :x  -1
    :y  -1
    :width      -1
    :height     -1
    :border-frame       :none
    :label-type :string
    :label-alignment    :center
    :label      "OK"
    :active     :true
    :visible    :true
    :north-attachment   (:grid-line 5 0)
    :south-attachment   (:grid-line 95 0)
    :east-attachment    (:grid-line 30 0)
    :west-attachment    (:grid-line 10 0)
)
:element        cancel_button
(
    :type       :button
    :button-type        :push-button
    :x  -1
    :y  -1
    :width      -1
    :height     -1
    :border-frame       :none
    :label-type :string
    :label-alignment    :center
    :label      "Cancel"
    :active     :true
    :visible    :true
    :north-attachment   (:grid-line 5 0)
    :south-attachment   (:grid-line 95 0)
    :east-attachment    (:grid-line 60 0)
    :west-attachment    (:grid-line 40 0)
)
:element        help_button
(
    :type       :button
    :button-type        :push-button
    :x  -1
    :y  -1
    :width      -1
    :height     -1
    :border-frame       :none
    :label-type :string
    :label-alignment    :center
    :label      "Help"
    :active     :true
    :visible    :true
    :north-attachment   (:grid-line 5 0)
    :south-attachment   (:grid-line 95 0)
    :east-attachment    (:grid-line 90 0)
    :west-attachment    (:grid-line 70 0)
)
:connection
(
    :from       ok_button
    :to task_dialog
    :when       :activate
    :action-type        :builtin
    :action     :hide
    :arg-type   :void
)
:connection
(
    :from       cancel_button
    :to task_dialog
    :when       :activate
    :action-type        :builtin
    :action     :hide
    :arg-type   :void
)
:connection
(
    :from       ok_button
    :when       :create
    :action-type        :call-function
    :action     confirm_taskCB
    :arg-type   :void
)
:connection
(
    :from       cancel_button
    :when       :create
    :action-type        :call-function
    :action     cancel_taskCB
    :arg-type   :void
)
)
## APPLICATION USAGE


Applications and application developers typically do not work
directly with BIL files, instead using them indirectly through
the CDE Application Builder.
It may, however, be useful for them to understand the layout and
scope of BIL to satisfy particular application development needs
(e.g. to add specialized comments to BIL files or to examine them
to examine useful information (such as help text built-into a
application)).
## SEE ALSO


&cdeman.dtbuilder; &cdeman.dtcodegen;