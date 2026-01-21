The window is mapped withXMapWindowor related routines.All its ancestors must be mapped.  This
condition is always satisfied for the children of the root
window, the top-level windows of each application.The window must not be obscured by visible
sibling windows or their
ancestors&emsp;this depends on the stacking order.  When first
mapped, a window appears on top of its siblings, which will
be on top of all windows if its parent is the root window.