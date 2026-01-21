/*
 * FormatMarkdown.c
 * Adapter for Rust-based Markdown Parser
 *
 * This file translates Markdown events into CDE Help System formatting calls.
 */

#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* X11 and Motif headers */
#include <X11/Intrinsic.h>
#include <X11/Xlib.h>
#include <X11/Xresource.h>
#include <Xm/Xm.h>

/*
 * Dependency Resolution Section
 * The CDE Help system headers have complex circular dependencies and
 * conditional compilation issues. We must order them carefully.
 */

/* 1. Definitions from DisplayAreaP and CanvasP
 * We keep _DtCanvasP_h defined (via build flag) to suppress conflicting structs
 * in CanvasP.h, relying on DisplayAreaP.h stubs. However, SDLI.h needs the
 * ENUMS from CanvasP.h, which DisplayAreaP.h defines conditionally or
 * partially. If DisplayAreaP.h defines them, we are good. If not, we manually
 * patch them.
 */
#include "DisplayAreaP.h"

/* 2. Definitions for SDLI.h
 * SDLI.h (SDL interface) relies on _DtCv* enums and _DtHelpFontHints.
 * We undefine its guard to ensure it is processed if it was skipped.
 */
#undef _DtHelpSDLI_h

/* Define canvas constants that might be missing from DisplayAreaP.h stubs */
#ifndef _DtCvOPTION_BAD
#define _DtCvOPTION_BAD 1
#endif
#ifndef _DtCvLITERAL
#define _DtCvLITERAL 2
#endif
#ifndef _DtCvDYNAMIC
#define _DtCvDYNAMIC 3
#endif

/* 3. Include headers defining types used by SDLI.h (_DtHelpFontHints) */
#include <DtI/FontAttrI.h>
#include <DtI/FontI.h>

/* 4. Include SDLI.h (Defines SDLAttribute) */
#include "SDLI.h"

/* 5. Generic Help System Headers */
/* DtHelpVolumeRec is used in some headers but typedef might be missing */
typedef struct _DtHelpVolumeRec *_DtHelpVolume;

#include "Access.h"
#include "AccessCCDFI.h"
#include "AccessI.h"
#include "AccessP.h"

/* Break circular dependency with forward declaration if needed */
/* FormatUtilI.h defines _FrmtUiInfo, used by others */
#include "FormatUtilI.h"

#include "CvtToArrayP.h"
/* Format headers depend on _FrmtUiInfo and SDLAttribute */
#include "FormatCCDFI.h"
#include "FormatI.h"
#include "FormatSDLI.h"

#include "HelpXlate.h"
#include "Lock.h"
#include "StringFuncsI.h"
#include "XInterfaceI.h"
#include "bufioI.h"

/* 6. Rust Integration Headers (Defines Events EV_*) */
#include "dthelp_rust.h"

#define BUFF_SIZE 4096

/* Context for the parser callback */
typedef struct {
  XtPointer varHandle;
  _DtHelpFontHints fontAttrs;
  int in_code_block;
} ParserContext;

/* Helper structure for FormatUtil calls */
static const _FrmtUiInfo defUiInfo = {NULL, NULL, NULL, NULL, NULL,
                                      NULL, 0,    0,    1,    False};

/* Callback implementation */
static void markdown_callback(void *ctx, int event_type, const char *text) {
  ParserContext *context = (ParserContext *)ctx;

  switch (event_type) {
  case EV_TEXT:
    if (text) {
      /* Process text using the CE engine */
      __DtHelpCeProcessString(context->varHandle, NULL, _DtCvLITERAL, "",
                              (char *)text, strlen(text), 0, False,
                              &context->fontAttrs);
    }
    break;

  case EV_HEADER_START:
    /* Force new paragraph for header */
    __DtHelpCeProcessString(context->varHandle, NULL, _DtCvLITERAL, "\n", "\n",
                            1, 0, False, &context->fontAttrs);
    break;

  case EV_HEADER_END:
    __DtHelpCeProcessString(context->varHandle, NULL, _DtCvLITERAL, "\n", "\n",
                            1, 0, False, &context->fontAttrs);
    break;

  case EV_PARA_START:
    __DtHelpCeProcessString(context->varHandle, NULL, _DtCvLITERAL, "\n", "\n",
                            1, 0, False, &context->fontAttrs);
    break;

  case EV_PARA_END:
    __DtHelpCeProcessString(context->varHandle, NULL, _DtCvLITERAL, "\n", "\n",
                            1, 0, False, &context->fontAttrs);
    break;

  case EV_BOLD_START:
    /* Check actual enum values in FontAttrI.h or similar if these fail */
    /* Assuming standard names based on usage patterns */
    context->fontAttrs.weight = _DtHelpFontWeightBold;
    break;

  case EV_BOLD_END:
    context->fontAttrs.weight = _DtHelpFontWeightMedium; /* Default */
    break;

  case EV_ITALIC_START:
    context->fontAttrs.slant = _DtHelpFontSlantItalic;
    break;

  case EV_ITALIC_END:
    context->fontAttrs.slant = _DtHelpFontSlantRoman;
    break;

  case EV_CODE_START:
    context->in_code_block = 1;
    context->fontAttrs.spacing = _DtHelpFontSpacingMono;
    break;

  case EV_CODE_END:
    context->in_code_block = 0;
    context->fontAttrs.spacing = _DtHelpFontSpacingProp;
    break;

  case EV_LINK_START:
    /* Links handling would go here */
    break;

  case EV_LINK_END:
    break;
  }
}

int _DtHelpFormatMarkdownFile(XtPointer client_data, char *filename,
                              XtPointer *ret_handle) {
  int myFile;
  off_t file_size;
  char *buffer;
  _DtCvTopicPtr topic = NULL;
  DtHelpDispAreaStruct *pDAS = (DtHelpDispAreaStruct *)client_data;
  _FrmtUiInfo myUiInfo = defUiInfo;
  ParserContext context;
  int result = -1;

  if (filename == NULL || ret_handle == NULL) {
    errno = EINVAL;
    return -1;
  }

  /* Read file content */
  myFile = open(filename, O_RDONLY);
  if (myFile == -1)
    return -1;

  file_size = lseek(myFile, 0, SEEK_END);
  lseek(myFile, 0, SEEK_SET);

  buffer = (char *)malloc(file_size + 1);
  if (!buffer) {
    close(myFile);
    return -1;
  }

  if (read(myFile, buffer, file_size) != file_size) {
    free(buffer);
    close(myFile);
    return -1;
  }
  buffer[file_size] = '\0';
  close(myFile);

  /* Setup CE */
  memset(&context, 0, sizeof(context));
  _DtHelpCeCopyDefFontAttrList(&context.fontAttrs);
  _DtHelpCeGetLcCtype(NULL, &(context.fontAttrs.language),
                      &(context.fontAttrs.char_set));

  myUiInfo.load_font = _DtHelpDAResolveFont;
  myUiInfo.client_data = (_DtCvPointer)pDAS;
  myUiInfo.line_width = pDAS->lineThickness;
  myUiInfo.line_height = pDAS->lineHeight;
  myUiInfo.leading = pDAS->leading;
  myUiInfo.avg_char =
      (int)(pDAS->charWidth / 10 + ((pDAS->charWidth % 10) ? 1 : 0));
  myUiInfo.nl_to_space = pDAS->nl_to_space;

  context.varHandle = __DtHelpCeSetUpVars(&myUiInfo);

  if (context.varHandle) {
    /* Parse Markdown! */
    dthelp_parse_markdown(buffer, &context, markdown_callback);

    /* Get segments */
    result = __DtHelpCeGetParagraphList(context.varHandle, True, _DtCvLITERAL,
                                        &topic);

    free(context.varHandle);
  }

  free(context.fontAttrs.language);
  free(context.fontAttrs.char_set);
  free(buffer);

  *ret_handle = (XtPointer)topic;
  return result;
}
