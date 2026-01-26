#include "XmCefBrowser.h"
#include "XmCefBrowserP.h"
#include <Xm/DrawP.h>
#include <Xm/PrimitiveP.h>
#include <cef/cef_base.h>

#include <stdio.h>
#include <string.h>

/* Defines */
#define DEFAULT_WIDTH 600
#define DEFAULT_HEIGHT 400

/* Static Method Declarations */
static void ClassInitialize(void);
static void Initialize(Widget request, Widget new_w, ArgList args,
                       Cardinal *num_args);
static void Realize(Widget w, XtValueMask *valueMask,
                    XSetWindowAttributes *attributes);
static void Destroy(Widget w);
static void Resize(Widget w);
static void CefBrowserExpose(Widget w, XEvent *event, Region region);
static Boolean SetValues(Widget current, Widget request, Widget new_w,
                         ArgList args, Cardinal *num_args);

/* Resources */
static XtResource resources[] = {{XmNurl, XmCUrl, XmRString, sizeof(String),
                                  XtOffsetOf(XmCefBrowserRec, cef_browser.url),
                                  XmRImmediate, (XtPointer) "about:blank"}};

/* Class Record */
XmCefBrowserClassRec xmCefBrowserClassRec = {
    /* Core Class */
    {
        (WidgetClass)&xmPrimitiveClassRec, /* superclass */
        "XmCefBrowser",                    /* class_name */
        sizeof(XmCefBrowserRec),           /* widget_size */
        ClassInitialize,                   /* class_initialize */
        NULL,                              /* class_part_initialize */
        FALSE,                             /* class_inited */
        Initialize,                        /* initialize */
        NULL,                              /* initialize_hook */
        Realize,                           /* realize */
        NULL,                              /* actions */
        0,                                 /* num_actions */
        resources,                         /* resources */
        XtNumber(resources),               /* num_resources */
        NULLQUARK,                         /* xrm_class */
        TRUE,                              /* compress_motion */
        XtExposeCompressMaximal,           /* compress_exposure */
        TRUE,                              /* compress_enterleave */
        FALSE,                             /* visible_interest */
        Destroy,                           /* destroy */
        Resize,                            /* resize */
        CefBrowserExpose,                  /* expose */
        SetValues,                         /* set_values */
        NULL,                              /* set_values_hook */
        XtInheritSetValuesAlmost,          /* set_values_almost */
        NULL,                              /* get_values_hook */
        NULL,                              /* accept_focus */
        XtVersion,                         /* version */
        NULL,                              /* callback_private */
        NULL,                              /* tm_table */
        NULL,                              /* query_geometry */
        NULL,                              /* display_accelerator */
        NULL                               /* extension */
    },
    /* XmPrimitive Class */
    {
        (XtWidgetProc)_XtInherit, /* border_highlight */
        (XtWidgetProc)_XtInherit, /* border_unhighlight */
        NULL,                     /* translations */
        NULL,                     /* arm_and_activate */
        NULL,                     /* syn_resources */
        0,                        /* num_syn_resources */
        NULL                      /* extension */
    },
    /* XmCefBrowser Class */
    {
        0 /* extension */
    }};

WidgetClass xmCefBrowserWidgetClass = (WidgetClass)&xmCefBrowserClassRec;

static void ClassInitialize(void) {}

static void Initialize(Widget request, Widget new_w, ArgList args,
                       Cardinal *num_args) {
  XmCefBrowserWidget bw = (XmCefBrowserWidget)new_w;

  /* Set default size if not specified */
  if (bw->core.width == 0)
    bw->core.width = DEFAULT_WIDTH;
  if (bw->core.height == 0)
    bw->core.height = DEFAULT_HEIGHT;

  bw->cef_browser.browser_handle = NULL;
  bw->cef_browser.url =
      XtNewString(((XmCefBrowserWidget)request)->cef_browser.url);
}

static void Realize(Widget w, XtValueMask *valueMask,
                    XSetWindowAttributes *attributes) {
  XmCefBrowserWidget bw = (XmCefBrowserWidget)w;
  cef_window_info_t window_info;

  /* Call superclass realize */
  XtCreateWindow(w, InputOutput, CopyFromParent, *valueMask, attributes);

  /* Initialize CEF Window Info */
  /* Implementation note: In a real app we'd call CefBrowserHost::CreateBrowser
   */
  /* Here we mock it via the shim */

  // Position and size
  int x = 0;
  int y = 0;
  int width = bw->core.width;
  int height = bw->core.height;

  // We pass the Window ID of this widget as parent
  cef_window_info_set_as_child(&window_info, (unsigned long)XtWindow(w), x, y,
                               width, height);

  fprintf(stderr, "[XmCefBrowser] Realized. Parent Window: 0x%lx\n",
          (unsigned long)XtWindow(w));
  fprintf(stderr, "[XmCefBrowser] Navigating to: %s\n", bw->cef_browser.url);

  // Simulate Browser Creation
  // In real code: CefBrowserHost::CreateBrowser(window_info, handler, url, ...)
}

static void Destroy(Widget w) {
  XmCefBrowserWidget bw = (XmCefBrowserWidget)w;
  XtFree(bw->cef_browser.url);
  // CefShutdown usually handled globally
}

static void Resize(Widget w) {
  XmCefBrowserWidget bw = (XmCefBrowserWidget)w;
  // Tell CEF to resize
  fprintf(stderr, "[XmCefBrowser] Resized to %dx%d\n", bw->core.width,
          bw->core.height);
  // In real code: browser->GetHost()->WasResized();
}

static void CefBrowserExpose(Widget w, XEvent *event, Region region) {
  // Redisplay if needed, though CEF handles its own painting in windowed mode
}

static Boolean SetValues(Widget current, Widget request, Widget new_w,
                         ArgList args, Cardinal *num_args) {
  XmCefBrowserWidget cur_w = (XmCefBrowserWidget)current;
  XmCefBrowserWidget new_bw = (XmCefBrowserWidget)new_w;
  Boolean redisplay = FALSE;

  if (strcmp(cur_w->cef_browser.url, new_bw->cef_browser.url) != 0) {
    XtFree(cur_w->cef_browser.url);
    new_bw->cef_browser.url = XtNewString(new_bw->cef_browser.url);

    fprintf(stderr, "[XmCefBrowser] URL changed to: %s\n",
            new_bw->cef_browser.url);
    // In real code: browser->GetMainFrame()->LoadURL(new_bw->cef_browser.url);
  }

  return redisplay;
}

Widget XmCreateCefBrowser(Widget parent, char *name, ArgList arglist,
                          Cardinal argcount) {
  return XtCreateWidget(name, xmCefBrowserWidgetClass, parent, arglist,
                        argcount);
}

void XmCefBrowserLoadUrl(Widget w, const char *url) {
  Arg args[1];
  XtSetArg(args[0], XmNurl, url);
  XtSetValues(w, args, 1);
}
