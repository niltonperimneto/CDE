# dsdm
user cmddsdmDrop Site Database Manager - Motif/OPENLOOK drag and drop
proxy agentdsdm
## DESCRIPTION


Thedsdmmanages a database of all drop sites on the display. When a drag
operation is started, thedsdmis queried for a list of the drop sites.
This drop site information is used by the dragging application to
identify drop sites and provide
user feedback during the drag operation.

Thedsdmprovides a drag-and-drop
gateway between Motif and OPENLOOK applications.
When an object is dropped on a drop site,
thedsdmgets a drop message from
the drop sender application. Thedsdmthen translates
the drop message, and sends the translated message
to the drop receiver application.
## WARNINGS


Thedsdmmust be running before applications are started for the drag-and-drop
gateway feature to work.