/*
 * CDE - Common Desktop Environment
 *
 * Copyright (c) 1993-2012, The Open Group. All rights reserved.
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these libraries and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
 */
/* $XConsortium: DisplayAreaP.h /main/18 1996/08/13 11:35:51 cde-hp $ */
/************************************<+>*************************************
 ****************************************************************************
 **
 **   File:        DisplayAreaP.h
 **
 **   Project:     Cde Help System
 **
 **   Description: Defines the Display Area structures and defines.
 **
 ****************************************************************************
 *************************************<+>*************************************/
/*
 * (c) Copyright 1996 Digital Equipment Corporation.
 * (c) Copyright 1987, 1988, 1989, 1990, 1991, 1992,
                 1993, 1994, 1996 Hewlett-Packard Company.
 * (c) Copyright 1993, 1994, 1996 International Business Machines Corp.
 * (c) Copyright 1993, 1994, 1996 Sun Microsystems, Inc.
 * (c) Copyright 1993, 1994, 1996 Novell, Inc.
 * (c) Copyright 1996 FUJITSU LIMITED.
 * (c) Copyright 1996 Hitachi.
 */

#ifndef _DtHelpDisplayAreaP_h
#define _DtHelpDisplayAreaP_h

#include <DtI/DisplayAreaI.h>
#include <DtI/GraphicsP.h>
#include <X11/X.h>
#include <X11/Xlib.h>
#include <Xm/XmP.h>

/* Defines Stubbed from CanvasP.h for compilation compatibility */
typedef void *_DtCvPointer;
typedef void *_DtCvHandle;
typedef int _DtCvUnit;
typedef unsigned long _DtCvFlags;
struct _dtCvTopicInfo;
typedef struct _dtCvTopicInfo *_DtCvTopicPtr;

/* Forward declarations for types used in structs */
typedef struct _dtCvSegPts _DtCvSegPts;
typedef struct _dtCvSegment _DtCvSegment;

/* Types conflicting with CanvasP.h */
#ifndef _DtCanvasP_h
typedef struct _dtCvPointInfo {
  struct _dtCvSegment *segment;
  int num_pts;
  _DtCvSegPts **segs;
} _DtCvPointInfo;
#endif /* _DtCanvasP_h */

/* Types conflicting with CanvasP.h */
#ifndef _DtCanvasP_h
typedef enum {
  _DtCvBAD_TYPE = 0,
  _DtCvCANVAS_TYPE,
  _DtCvLINE_TYPE,
  _DtCvLINK_TYPE,
  _DtCvLOCALE_TYPE,
  _DtCvMARK_TYPE,
  _DtCvREGION_TYPE,
  _DtCvSTRING_TYPE,
  _DtCvTRAVERSAL_TYPE
} _DtCvElemType;

typedef int _DtCvValue;
typedef int _DtCvRenderType;
typedef int _DtCvStatus;
typedef int _DtCvTraversalCmd;

typedef struct {
  char *specification;
  char *description;
  int hyper_type;
  int win_hint;
  _DtCvUnit offset_x;
  _DtCvUnit offset_y;
} _DtCvLinkInfo;

typedef struct _dtCvMetrics {
  _DtCvUnit width;
  _DtCvUnit height;
  _DtCvUnit top_margin;
  _DtCvUnit side_margin;
  _DtCvUnit line_height;
  _DtCvUnit horiz_pad_hint;
} _dtCvMetrics;

typedef struct _dtCvSpaceMetrics {
  _DtCvUnit space_before;
  _DtCvUnit space_after;
  _DtCvUnit space_above;
  _DtCvUnit space_below;
} _DtCvSpaceMetrics;
#endif /* _DtCanvasP_h */

/********    Canvas Segment Types (Moved from CanvasSegP.h)    ********/

/* Format Option Enum - Suppress if CanvasSegP.h is present */
#ifndef _DtCanvasSegP_h
typedef enum _dtCvFrmtOption {
  _DtCvOPTION_BAD,
  _DtCvLITERAL,
  _DtCvDYNAMIC,
  _DtCvBORDER_NONE,
  _DtCvBORDER_FULL,
  _DtCvBORDER_HORZ,
  _DtCvBORDER_VERT,
  _DtCvBORDER_TOP,
  _DtCvBORDER_BOTTOM,
  _DtCvBORDER_LEFT,
  _DtCvBORDER_RIGHT,
  _DtCvBORDER_TOP_LEFT,
  _DtCvBORDER_TOP_RIGHT,
  _DtCvBORDER_BOTTOM_LEFT,
  _DtCvBORDER_BOTTOM_RIGHT,
  _DtCvJUSTIFY_LEFT_CORNER,
  _DtCvJUSTIFY_LEFT,
  _DtCvJUSTIFY_LEFT_MARGIN,
  _DtCvJUSTIFY_CENTER,
  _DtCvJUSTIFY_RIGHT_MARGIN,
  _DtCvJUSTIFY_RIGHT,
  _DtCvJUSTIFY_RIGHT_CORNER,
  _DtCvJUSTIFY_NUM,
  _DtCvJUSTIFY_CHAR,
  _DtCvINHERIT,
  _DtCvJUSTIFY_TOP,
  _DtCvJUSTIFY_BOTTOM,
  _DtCvWRAP,
  _DtCvWRAP_NONE,
  _DtCvWRAP_JOIN
} _DtCvFrmtOption;
#endif /* _DtCanvasSegP_h */

/* Extend SegP guard to other segment types */
#ifndef _DtCanvasSegP_h
/* Forward declarations */
struct _dtCvSegment;

typedef struct _dtCvLine {
  _DtCvUnit width;
  _DtCvPointer data;
} _DtCvLine;

typedef struct _dtCvContainer {
  char *id;
  char *justify_char;
  _DtCvFrmtOption type;
  _DtCvFrmtOption border;
  _DtCvFrmtOption justify;
  _DtCvFrmtOption vjustify;
  _DtCvFrmtOption orient;
  _DtCvFrmtOption vorient;
  _DtCvFrmtOption flow;
  int percent;
  _DtCvUnit leading;
  _DtCvUnit fmargin;
  _DtCvUnit lmargin;
  _DtCvUnit rmargin;
  _DtCvUnit tmargin;
  _DtCvUnit bmargin;
  _DtCvLine bdr_info;
  struct _dtCvSegment *seg_list;
} _DtCvContainer;

typedef struct _DtCvString {
  void *string;
  _DtCvPointer font;
} _DtCvString;

typedef struct _dtCvRegion {
  _DtCvPointer info;
  _DtCvUnit width;
  _DtCvUnit height;
  _DtCvUnit ascent;
} _DtCvRegion;

typedef struct _dtCvTable {
  int num_cols;
  char **col_w;
  _DtCvFrmtOption *col_justify;
  char *justify_chars;
  char **cell_ids;
  struct _dtCvSegment **cells;
} _DtCvTable;

typedef union _dtCvSegHandles {
  _DtCvContainer container;
  _DtCvString string;
  _DtCvRegion region;
  _DtCvTable table;
  _DtCvLine rule;
  char *marker;
} _DtCvSegHandles;

struct _dtCvSegment {
  unsigned long type;
  int link_idx;
  _DtCvSegHandles handle;
  struct _dtCvSegment *next_seg;
  struct _dtCvSegment *next_disp;
  _DtCvPointer client_use;
  _DtCvPointer internal_use;
};

struct _dtCvSegPts {
  struct _dtCvSegment *segment;
  int offset;
  int len;
};

/* Forward declare LinkDb struct */
struct _dtCvLinkDb;

/* Actual definition of _DtCvTopicInfo */
typedef struct _dtCvTopicInfo {
  char *id_str;
  struct _dtCvSegment *seg_list;
  _DtCvPointInfo **mark_list;
  /* struct _dtCvLinkDb *link_data; */ /* Use void* for now */
  void *link_data;
} _DtCvTopicInfo;

typedef struct _dtCvStringClientData {
  unsigned int vcc;
  unsigned int vclen;

  unsigned int hilite_type;

  char *bg_color;
  char *fg_color;
  unsigned long bg_pixel;
  unsigned long fg_pixel;
} _DtCvStringClientData;

typedef struct _dtCvRegionClientData {
  _DtCvPointer GraphicHandle;
} _DtCvRegionClientData;

typedef union _dtCvClientData {
  _DtCvRegionClientData region;
  _DtCvStringClientData string;
} _DtCvClientData;

/********    End Canvas Segment Types    ********/
#endif /* _DtCanvasSegP_h */

/* Types conflicting with CanvasP.h */
#ifndef _DtCanvasP_h
typedef struct {
  void *get_metrics;
  void *render_elem;
  void *get_width;
  void *get_font_metrics;
  void *build_selection;
  void *exec_cmd_filter;
} _DtCvVirtualInfo;
#endif /* _DtCanvasP_h */

/* Legacy Canvas Defines for Compatibility */
#ifndef _DtCvFALSE
#define _DtCvFALSE 0
#define _DtCvTRUE 1
#endif

#define _DtCvSTATUS_OK 0
#define _DtCvSTATUS_BAD 1

#define _DtCvRENDER_PARTIAL 0
#define _DtCvRENDER_COMPLETE 1

#define _DtCvUSE_BOUNDARY 0
#define _DtCvUSE_BOUNDARY_MOVE 1
#define _DtCvIGNORE_BOUNDARY 2

/* Link Types */
#define _DtCvLinkType_Execute 3
#define _DtCvLinkType_ManPage 4
#define _DtCvLinkType_AppDefine 5
#define _DtCvLinkType_SameVolume 6
#define _DtCvLinkType_CrossLink 7
#define _DtCvLinkType_TextFile 8

/* Window Hints */
#define _DtCvWindowHint_PopupWindow 1
#define _DtCvWindowHint_CurrentWindow 2
#define _DtCvWindowHint_NewWindow 3
#define _DtCvWindowHint_Original 4

/* Traversal Defines */
#define _DtCvTRAVERSAL_OFF 0
#define _DtCvTRAVERSAL_ON 1
#define _DtCvTRAVERSAL_ID 6

/*
 * Traversal flags
 */
#define _DT_HELP_SHADOW_TRAVERSAL (1 << 0)
#define _DT_HELP_NOT_INITIALIZED (1 << 1)
#define _DT_HELP_TRAVERSAL_DRAWN (1 << 2)
#define _DT_HELP_DRAW_TOC_IND (1 << 3)
#define _DT_HELP_CLEAR_TOC_IND (1 << 4)
#define _DT_HELP_TOC_ON (1 << 5)

/*
 * enum states for selection
 */
enum _DtHelpSelectState {
  _DtHelpNothingDoing,
  _DtHelpCopyOrLink,
  _DtHelpSelectingText
};

/*
 * Whether the display area has the focus or not
 */
#define _DT_HELP_FOCUS_FLAG 0x04

typedef struct {
  int used;
  int num_pixels;
  Pixmap pix;
  Pixmap mask;
  Dimension width;
  Dimension height;
  Pixel *pixels;
} DtHelpGraphicStruct;

typedef struct {
  _DtCvPointer font_ptr;
  int spc_idx;
} DtHelpSpecialChars;

typedef struct _dtHelpDAFontMetrics {
  _DtCvUnit ascent;        /* Maximum ascent               */
  _DtCvUnit descent;       /* Maximum descent              */
  _DtCvUnit average_width; /* Average width of a character */
  _DtCvUnit super;         /* Offset from baseline for super scripts */
  _DtCvUnit sub;           /* Offset from baseline for sub scripts   */
} _DtHelpDAFontMetrics;

typedef struct {
  short inited;
  _DtHelpDAFontMetrics fm;
} DtHelpDAFSMetrics;

typedef struct _DtHelpDAfontInfo {
  char **exact_fonts;         /* the list of fonts specified by the
                                 toss element rather than hints.   */
  XrmDatabase def_font_db;    /* The default font resource db      */
  XrmDatabase font_idx_db;    /* Which font index goes with which
                                 set of font resources	     */
  XFontStruct **font_structs; /* The font structures opened        */
  XFontSet *font_sets;        /* The font sets opened              */
  DtHelpDAFSMetrics *fs_metrics;

  XrmQuark lang_charset; /* the char set for current lang     */
  int *exact_idx;        /* The indexes for the exact fonts   */
  int max_structs;       /* the max number of font_structs    */
  int max_sets;          /* The max number of font_sets	     */
  int struct_cnt;        /* the cur number of font_structs    */
  int set_cnt;           /* The cur number of font_sets	     */
  long def_idx;          /* The default index                 */
} DtHelpDAFontInfo;

/*
 * SelectionScroll structure
 */
typedef struct {
  int horizontal_reason;
  int vertical_reason;
} SelectionScrollStruct;

/*
 * DisplayArea structure
 */
typedef struct DtHelpEngine DtHelpEngine;

typedef struct _dtHelpDispAreaStruct DtHelpDispAreaStruct;
struct _dtHelpDispAreaStruct {
  Widget dispWid;       /* The text and graphic area. */
  Widget vertScrollWid; /* The vertical scroll bar    */
  Widget horzScrollWid; /* The horizontal scroll bar  */
  Boolean vertIsMapped;
  Boolean horzIsMapped;
  short neededFlags; /* _DtHelpAS_NEEDED flags        */
  short nl_to_space; /* are newlines in multibyte     */
                     /* strings turned into spaces?   */

  Dimension formWidth;     /* Pixel width of the parent  area  */
  Dimension formHeight;    /* Pixel height of the parent area  */
  Dimension dispWidth;     /* Pixel width of the display area  */
  Dimension dispHeight;    /* Pixel height of the display area */
  Dimension dispUseHeight; /* Pixel height of the display area
                              minus the decor margin.          */
  Dimension dispUseWidth;  /* Pixel width of the display area
                              minus the decor margin.          */
  Dimension marginWidth;   /* Pixel padding at the left and
                              right of the display area.       */
  Dimension marginHeight;  /* Pixel padding at the top and
                              bottom of the display area.      */

  short decorThickness; /* the shadow thickness plus highlight
                           thickness of the display area    */

  void (*hyperCall)(DtHelpDispAreaStruct *, XtPointer,
                    DtHelpHyperTextStruct *); /* The hypertext callback */
  void (*resizeCall)(XtPointer);              /* The resize callback    */
  int (*exec_filter)(void *, const char *,
                     char **); /* The execution filter callback    */
  XtPointer clientData;        /* The client's data for the callback */

  Pixel traversalColor;  /* The client's traversal color */
  Pixel foregroundColor; /* The client's foreground color     */
  Pixel backgroundColor; /* The client's foreground color     */
  Pixel searchColor;     /* The client's search hilite color  */
  GC pixmapGC;
  GC normalGC;
  GC invertGC;
  Pixmap def_pix;           /* the default 'missing pixmap'      */
  Dimension def_pix_width;  /* the width of the default pixmap   */
  Dimension def_pix_height; /* the height of the default pixmap  */
  _DtGrContext *context;    /* image converter context */

  Colormap colormap; /* The colormap to use		*/
  Visual *visual;    /* The visual to use		*/

  DtHelpDAFontInfo font_info; /* The font information		*/

  int depth; /* The depth of the window	*/

  int fontAscent;
  int lineHeight;
  int leading;
  long charWidth;    /* The average size of a character   */
  int moveThreshold; /* The number of pixels that must
                        be moved before a copy-paste
                        action occurs.                    */
  int underLine;
  int lineThickness;  /* For traversal box and underline   */
  int firstVisible;   /* The absolute number of the first
                         line visible in the window.      */
  int nextNonVisible; /* The absolute number of the first
                         line non visible, next to the last
                         visible window.  */
  int visibleCount;   /* The number of lines viewable     */
  int maxYpos;        /* Maximum Y positioning	    */

  int virtualX; /* The virtual x of the window      */
  int maxX;     /* The max virtual x of a line      */

  int max_spc;          /* The maximum special characters   */
  int cur_spc;          /* The current unused structure     */
  int timerX;           /* Used for button clicks/selections */
  int timerY;           /* Used for button clicks/selections */
  int scr_timer_x;      /* Used for button clicks/selections */
  int scr_timer_y;      /* Used for button clicks/selections */
  int vert_init_scr;    /* The initial vert scrolling timeout*/
  int vert_rep_scr;     /* The repeat vert scrolling timeout */
  int horz_init_scr;    /* The initial horz scrolling timeout*/
  int horz_rep_scr;     /* The repeat horz scrolling timeout */
  _DtCvUnit toc_width;  /* The width  of the toc indicator   */
  _DtCvUnit toc_height; /* The height of the toc indicator   */
  _DtCvUnit toc_y;      /* The y coordinate of the toc       */
  _DtCvUnit toc_base;   /* The baseline coordinate of the toc*/

  Time anchor_time;      /* Indicates the primary selection
                            time.                              */
  Boolean primary;       /* Indicates if this widget has the
                            primary selection                  */
  Boolean text_selected; /* Indicates if the selection has
                            occurred     		      */
  enum _DtHelpSelectState select_state;
  /* Indicates the state of the current
     selection.                         */
  short toc_flag;             /* Indicates if the traversal indicator
                                 is always on.		      */
  _DtCvPointer toc_indicator; /* The indicator used in the toc */

  XtIntervalId scr_timer_id;
  SelectionScrollStruct scr_timer_data;
  DtHelpSpecialChars *spc_chars; /* Structure containing the spc chars */
  _DtCvHandle canvas;
  _DtCvTopicPtr lst_topic;
  wchar_t *cant_begin_chars; /* characters that cannot    */
                             /* begin a line of text      */
  wchar_t *cant_end_chars;   /* characters that cannot    */
                             /* end a line of text        */

  short dtinfo;   /* Indicates if being used by dtinfo */
  Pixmap stipple; /* stippled pixmap */

  /* callback to be called whenever you manipulate the display */
  /* area's vertical scrollbar directly using XtSetValues, */
  /* because the application (dtinfo) has callbacks on the */
  /* scrollbar, these do not get called when we adjust it with */
  /* XtSetValues() */
  void (*vScrollNotify)(void *, unsigned int);

  /* dtinfo requires this for doing link previews */
  void (*armCallback)(void *);

  _DtCvValue honor_size;           /* Layout parameter for _DtCvSetTopic */
  _DtCvRenderType render_type;     /* Render type performed on expose */
  unsigned short media_resolution; /* used for scaling images */

  DtHelpEngine *rust_engine; /* Rust rendering engine instance */
};

/* Legacy Function Prototypes from CanvasP.h */
#ifndef _DtCanvasP_h
extern void _DtCanvasRender(_DtCvHandle canvas_handle, _DtCvUnit x1,
                            _DtCvUnit y1, _DtCvUnit x2, _DtCvUnit y2,
                            _DtCvRenderType flag, _DtCvValue pg_break,
                            _DtCvUnit *max_y, _DtCvUnit *next_y);

extern void _DtHelpDATocMarker(XtPointer client_data, Boolean flag);

extern _DtCvStatus
_DtCanvasMoveTraversal(_DtCvHandle canvas_handle, _DtCvTraversalCmd cmd,
                       _DtCvValue wrap, _DtCvValue render, _DtCvPointer rid,
                       _DtCvUnit *ret_x, _DtCvUnit *ret_y,
                       _DtCvUnit *ret_baseline, _DtCvUnit *ret_height);

extern _DtCvStatus _DtCanvasGetPosLink(_DtCvHandle canvas_handle, _DtCvUnit x,
                                       _DtCvUnit y, _DtCvUnit width,
                                       _DtCvUnit height,
                                       _DtCvLinkInfo *ret_info);

extern _DtCvStatus _DtCanvasResize(_DtCvHandle canvas_handle, _DtCvValue force,
                                   _DtCvUnit *ret_width, _DtCvUnit *ret_height);

extern _DtCvStatus _DtCanvasSetTopic(_DtCvHandle canvas_handle,
                                     _DtCvTopicPtr topic_handle,
                                     _DtCvValue honor_size,
                                     _DtCvUnit *ret_width,
                                     _DtCvUnit *ret_height, _DtCvUnit *ret_y);

extern void _DtCanvasLoadMetrics(_DtCvHandle canvas_handle);

extern _DtCvHandle _DtCanvasCreate(_DtCvVirtualInfo virt_info,
                                   _DtCvPointer client_data);

extern void _DtCanvasDestroy(_DtCvHandle canvas_handle);
#endif /* _DtCanvasP_h */

#endif /* _DtHelpDisplayAreaP_h */
