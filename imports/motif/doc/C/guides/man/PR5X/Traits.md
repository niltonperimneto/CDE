# Traits
file formats`Traits`Lists the traits used by the Motif Toolkit.Traits
## DESCRIPTION


A trait is a characteristic of a widget. A widget holding
a particular trait is announcing a particular ability to
other widgets.
The following table summarizes the standard Motif traits.

`Purpose of Each Trait``Trait Name``A Widget Holding This Trait Can Do The Following:``XmQTaccessTextual`Display one primary text parcel.`XmQTactivatable`Become a command button in a dialog box.`XmQTcareParentVisual`Borrow its parent's visual information.`XmQTcontainer`Manage container item children.`XmQTcontainerItem`Become a child of a container widget.`XmQTdialogShellSavvy`Become a child of a DialogShell.`XmQTjoinSide`Attach itself to one side of a suitable parent.`XmQTmenuSavvy`Become a menu child.`XmQTmenuSystem`Manage a menu system.`XmQTnavigator`Act as a navigator to a scrollable widget.`XmQTscrollFrame`Handle one or more navigator widgets.`XmQTspecifyRenderTable`Supply the names of its default render tables.`XmQTtakesDefault`Change its appearance to show that it is the default button.`XmQTtransfer`Transfer data to other widgets and/or receive data from other
widgets

Traits are not often used in Motif application programs.
However, traits are very important to widget writers.
For complete details on traits, see the&MotifWidGd;.

The following table lists the names of all widgets and gadgets
in the standard Motif widget set that hold a particular trait.
For example, the following table shows that the`XmQTcontainerItem`trait is held by theXmIconGadget.
As the table suggests, some traits are held by many of the
standard Motif widgets.

`Trait Installation in
Standard Widget Set``Trait Name``Is Installed on The Following Widgets:``XmQTaccessTextual``XmLabel`and all its subclasses;`XmLabelGadget`and all its subclasses;`XmText; XmTextField``XmQTactivatable``XmArrowButton; XmArrowButtonGadget; XmDrawnButton; XmPushButton;
XmPushButtonGadget``XmQTcareParentVisual`All the subclasses of`XmGadget`(but not`XmGadget`itself);`XmPrimitive`and all its subclasses`XmQTcontainer``XmContainer``XmQTcontainerItem`XmIconGadget`XmQTdialogShellSavvy``XmBulletinBoard``XmQTjoinSide`No widgets install this trait`XmQTmenuSavvy``XmLabel; XmDrawnButton; XmCascadeButton; XmPushButton;
XmToggleButton; XmLabelGadget; XmCascadeButtonGadget;
XmPushButtonGadget; XmToggleButtonGadget``XmQTmenuSystem``XmRowColumn``XmQTnavigator``XmScrollBar; XmSpinBox``XmQTscrollFrame``XmNotebook; XmScrolledWindow``XmQTspecifyRenderTable``XmBulletinBoard`and all its subclasses;`XmMenuShell; XmVendorShell``XmQTtakesDefault``XmPushButton; XmPushButtonGadget``XmQTtransfer``XmContainer; XmLabel`and all its subclasses;`XmLabelGadget`and all its subclasses;`XmList; XmScale; XmText; XmTextField`

The following table lists the traits installed on each widget.
For example, the following table indicates that the`XmArrowButton`widget holds both the`XmQTactivatable`and`XmQTcareParentVisual`traits.`Trait Use by
Widget``Widget Name``Installs These Traits``=``XmArrowButton``XmQTactivatable, XmQTcareParentVisual``XmArrowButtonGadget``XmQTactivatable, XmQTcareParentVisual``XmBulletinBoard``XmQTdialogShellSavvy, XmQTspecifyRenderTable``XmCascadeButton``XmQTaccessTextual, XmQTcareParentVisual, XmQTmenuSavvy,
XmQTtransfer``XmCascadeButtonGadget``XmQTaccessTextual, XmQTcareParentVisual, XmQTmenuSavvy,
XmQTtransfer
XmComboBox``XmCommand``XmQTspecifyRenderTable``XmContainer``XmQTcontainer, XmQTtransfer``XmDialogShell`None`XmDisplay`None`XmDragContext`None`XmDragIcon`None`XmDrawingArea`None`XmDrawnButton``XmQTaccessTextual, XmQTactivatable, XmQTcareParentVisual,
XmQTmenuSavvy, XmQTtransfer``XmDropTransfer`None`XmFileSelectionBox``XmQTspecifyRenderTable``XmForm``XmQTspecifyRenderTable``XmFrame`None`XmGadget`NoneXmIconGadget`XmQTcareParentVisual, XmQTcontainerItem``XmLabel``XmQTaccessTextual, XmQTcareParentVisual, XmQTmenuSavvy,
XmQTtransfer``XmLabelGadget``XmQTaccessTextual, XmQTcareParentVisual, XmQTmenuSavvy,
XmQTtransfer``XmList``XmQTcareParentVisual, XmQTtransfer``XmMainWindow``XmManager``XmMenuShell``XmQTspecifyRenderTable``XmMessageBox``XmQTspecifyRenderTable``XmNotebook``XmQTscrollFrame``XmPanedWindow``XmPrimitive``XmQTcareParentVisual``XmPushButton``XmQTaccessTextual, XmQTactivatable, XmQTcareParentVisual,
XmQTmenuSavvy, XmQTtakesDefault, XmQTtransfer``XmPushButtonGadget``XmQTaccessTextual, XmQTactivatable, XmQTcareParentVisual,
XmQTmenuSavvy, XmQTtakesDefault, XmQTtransfer``XmRowColumn``XmQTmenuSystem``XmScale``XmQTtransfer``XmScreen``XmScrollbar``XmQTcareParentVisual, XmQTnavigator``XmScrolledWindow``XmQTscrollFrame``XmSelectionBox``XmQTspecifyRenderTable``XmSeparator``XmQTcareParentVisual``XmSeparatorGadget``XmQTcareParentVisual``XmSpinBox``XmQTnavigator``XmText``XmQTaccessTextual, XmQTcareParentVisual, XmQTtransfer``XmTextField``XmQTaccessTextual, XmQTcareParentVisual,
XmQTtransfer``XmToggleButton``XmQTaccessTextual, XmQTcareParentVisual,
XmQTtransfer``XmToggleButtonGadget``XmQTaccessTextual, XmQTcareParentVisual, XmQTtransfer``VendorShell``XmQTspecifyRenderTable`

The following table summarizes how the standard Motif widgets
access traits. There are two general ways for a widget to
access the traits of another widget.

One way is for a widget to ask another widget if it holds
a particular trait.
For example,`XmBulletinBoard`asks each of its children widgets
if they hold the`XmQTtakesDefault`trait.`XmBulletinBoard`calls none of the trait methods
of`XmQTtakesDefault`.

Another kind of access is when one widget calls another widget's
trait method(s).
For example,`XmBulletinBoard`calls the`getRenderTable`trait method of the`XmQTspecifyRenderTable`trait.`Trait Access By
Widget``Widget``Accesses These Traits:``Calls These Trait Methods:``XmArrowButton`NoneNone`XmArrowButtonGadget`NoneNone`XmBulletinBoard``XmQTtakesDefault`None`XmBulletinBoard``XmQTspecifyRenderTable``getRenderTable``XmCascadeButton``XmQTmenuSystem`Many`XmCascadeButton``XmQTspecifyRenderTable``getRenderTable``XmCascadeButtonGadget``XmQTmenuSystem`Many`XmCascadeButtonGadget``XmQTspecifyRenderTable``getRenderTable``XmComboBox``XmQTaccessTextual``getValue, setValue``XmCommand`NoneNone`XmContainer``XmQTcontainerItem``getValues, setValues``XmContainer``XmQTscrollFrame``getInfo``XmDialogShell``XmQTdialogShellSavvy``callMapUnmapCB``XmDisplay`NoneNone`XmDragContext`NoneNone`XmDragIcon`NoneNone`XmDrawingArea`NoneNone`XmDrawnButton``XmQTmenuSystem`Many`XmDrawnButton``XmQTspecifyRenderTable``getRenderTable``XmDropTransfer`NoneNone`XmFileSelectionBox``XmQTactivatable`None`XmForm`NoneNone`XmFrame`NoneNone`XmGadget`NoneNoneXmIconGadget`XmQTcontainer``getValues`XmIconGadget`XmQTspecifyRenderTable``getRenderTable``XmLabel``XmQTmenuSystem``various methods``XmLabel``XmQTspecifyRenderTable``getRenderTable``XmLabelGadget``XmQTmenuSystem``various methods``XmLabelGadget``XmQTspecifyRenderTable``getRenderTable``XmList``XmQTnavigator``getValues``XmList``XmQTscrollFrame``getInfo, init``XmList``XmQTspecifyRenderTable``getRenderTable``XmMainWindow``XmQTmenuSystem``various methods``XmManager`NoneNone`XmMenuShell``XmQTmenuSystem``various methods``XmMenuShell``XmQTspecifyRenderTable``getRenderTable``XmMessageBox``XmQTactivatable`None`XmNotebook``XmQTscrollFrame``init, addNavigator, removeNavigator``XmNotebook``XmQTnavigator``getValue``XmNotebook``XmQTactivatable``changeCB``XmNotebook``XmQTaccessTextual`None`XmPanedWindow`NoneNone`XmPrimitive`NoneNone`XmPushButton``XmQTmenuSystem``various methods``XmPushButton``XmQTspecifyRenderTable``getRenderTable``XmPushButtonGadget``XmQTmenuSystem``various methods``XmPushButtonGadget``XmQTspecifyRenderTable``getRenderTable``XmRowColumn``XmQTmenuSavvy``getAccelerator, getMnemonic, getActivateCBName``XmRowColumn``XmQTmenuSystem``various methods``XmScale``XmQTspecifyRenderTable``getRenderTable``XmScreen`NoneNone`XmScrollbar`NoneNone`XmScrolledWindow``XmQTnavigator``getValue``XmScrolledWindow``XmQTscrollFrame``init, addNavigator``XmSelectionBox``XmQTaccessTextual``setValue;``XmSelectionBox``XmQTactivatable`None`XmSeparator`NoneNone`XmSeparatorGadget`NoneNone`XmSpinBox``XmQTaccessTextual``setValue``XmText``XmQTaccessTextual``getValue, setValue``XmText``XmQTnavigator``getValue``XmText``XmQTscrollFrame``getInfo, init``XmText``XmQTspecifyRenderTable``getRenderTable``XmTextField``XmQTspecifyRenderTable``getRenderTable``XmToggleButton``XmQTmenuSystem``various methods``XmToggleButton``XmQTspecifyRenderTable``getRenderTable``XmToggleButtonGadget``XmQTmenuSystem``various methods``XmToggleButtonGadget``XmQTspecifyRenderTable``getRenderTable``VendorShell``XmQTspecifyRenderTable``getRenderTable`
## RELATED


The following reference pages are documented in the&MotifWidGd;:
&cdeman.XmeTraitSet;,
&cdeman.XmeTraitGet;,
&cdeman.XmQTaccessTextual;,
&cdeman.XmQTactivatable;,
&cdeman.XmQTcareParentVisual;,
&cdeman.XmQTcontainer;,
&cdeman.XmQTcontainerItem;,
&cdeman.XmQTdialogShellSavvy;,
&cdeman.XmQTjoinSide;,
&cdeman.XmQTmenuSavvy;,
&cdeman.XmQTmenuSystem;,
&cdeman.XmQTnavigator;,
&cdeman.XmQTscrollFrame;,
&cdeman.XmQTspecifyRenderTable;,
and
&cdeman.XmQTtakesDefault;.