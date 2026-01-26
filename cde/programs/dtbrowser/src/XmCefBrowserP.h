#ifndef _XmCefBrowserP_h
#define _XmCefBrowserP_h

#include "XmCefBrowser.h"
#include <Xm/PrimitiveP.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  String url;
  void *browser_handle; // Generic pointer to CefBrowser or Wrapper
} XmCefBrowserPart;

typedef struct _XmCefBrowserRec {
  CorePart core;
  XmPrimitivePart primitive;
  XmCefBrowserPart cef_browser;
} XmCefBrowserRec;

typedef struct {
  int empty;
} XmCefBrowserClassPart;

typedef struct _XmCefBrowserClassRec {
  CoreClassPart core_class;
  XmPrimitiveClassPart primitive_class;
  XmCefBrowserClassPart cef_browser_class;
} XmCefBrowserClassRec;

extern XmCefBrowserClassRec xmCefBrowserClassRec;

#ifdef __cplusplus
}
#endif

#endif /* _XmCefBrowserP_h */
