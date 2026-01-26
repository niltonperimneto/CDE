#ifndef _XmCefBrowser_h
#define _XmCefBrowser_h

#include <Xm/Primitive.h>
#include <Xm/Xm.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Class and Widget pointers */
typedef struct _XmCefBrowserRec *XmCefBrowserWidget;
typedef struct _XmCefBrowserClassRec *XmCefBrowserWidgetClass;

extern WidgetClass xmCefBrowserWidgetClass;

/* Resources */
#define XmNurl "url"
#define XmCUrl "Url"

/* Convenience creation function */
extern Widget XmCreateCefBrowser(Widget parent, char *name, ArgList arglist,
                                 Cardinal argcount);

/* Public Functions */
extern void XmCefBrowserLoadUrl(Widget w, const char *url);

#ifdef __cplusplus
}
#endif

#endif /* _XmCefBrowser_h */
