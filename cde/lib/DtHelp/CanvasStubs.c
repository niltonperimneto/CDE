/* CanvasStubs.c - Stubs for legacy Canvas and Help Engine functions */

#include "DtI/DisplayAreaP.h"

/* Canvas Stubs */
void _DtCanvasRender(_DtCvHandle canvas_handle, _DtCvUnit x1, _DtCvUnit y1,
                     _DtCvUnit x2, _DtCvUnit y2, _DtCvRenderType flag,
                     _DtCvValue pg_break, _DtCvUnit *max_y, _DtCvUnit *next_y) {
}

_DtCvStatus _DtCanvasMoveTraversal(_DtCvHandle canvas_handle,
                                   _DtCvTraversalCmd cmd, _DtCvValue wrap,
                                   _DtCvValue render, _DtCvPointer rid,
                                   _DtCvUnit *ret_x, _DtCvUnit *ret_y,
                                   _DtCvUnit *ret_baseline,
                                   _DtCvUnit *ret_height) {
  return _DtCvSTATUS_OK;
}

_DtCvStatus _DtCanvasGetPosLink(_DtCvHandle canvas_handle, _DtCvUnit x,
                                _DtCvUnit y, _DtCvUnit width, _DtCvUnit height,
                                _DtCvLinkInfo *ret_info) {
  return _DtCvSTATUS_BAD;
}

_DtCvStatus _DtCanvasResize(_DtCvHandle canvas_handle, _DtCvValue force,
                            _DtCvUnit *ret_width, _DtCvUnit *ret_height) {
  return _DtCvSTATUS_OK;
}

_DtCvStatus _DtCanvasSetTopic(_DtCvHandle canvas_handle,
                              _DtCvTopicPtr topic_handle, _DtCvValue honor_size,
                              _DtCvUnit *ret_width, _DtCvUnit *ret_height,
                              _DtCvUnit *ret_y) {
  return _DtCvSTATUS_OK;
}

_DtCvHandle _DtCanvasCreate(_DtCvVirtualInfo virt_info,
                            _DtCvPointer client_data) {
  return (_DtCvHandle)0;
}

/* Obsolete.c calls this */
void _DtCanvasDestroy(_DtCvHandle canvas_handle) {}

/* Help Engine (Ce) Stubs */
void *_DtHelpCeOpenSdlVolume(char *file, void *client_data) { return 0; }
void *_DtHelpCeOpenCcdfVolume(char *file, void *client_data) { return 0; }
void _DtHelpCeCloseSdlVolume(void *vol) {}
void _DtHelpCeCloseCcdfVolume(void *vol) {}
int _DtHelpCeRereadCcdfVolume(void *vol) { return 0; }
int _DtHelpCeRereadSdlVolume(void *vol) { return 0; }
char **_DtHelpCeGetCcdfKeywordList(void *vol) { return 0; }
char **_DtHelpCeGetSdlKeywordList(void *vol) { return 0; }
int _DtHelpCeReadBuf(void *vol, int offset, char *buf, int len) { return 0; }
char *_DtHelpCeGetCcdfTopicAbbrev(void *vol, char *id) { return 0; }
char *_DtHelpCeFrmtSDLTitleToAscii(char *title) { return 0; }
char *_DtHelpCeGetCcdfTopTopic(void *vol) { return 0; }
char *_DtHelpCeGetSdlHomeTopicId(void *vol) { return 0; }
char *_DtHelpCeFindCcdfId(void *vol, char *id) { return 0; }
char *_DtHelpCeFindSdlId(void *vol, char *id) { return 0; }
char *_DtHelpCeMapCcdfTargetToId(void *vol, char *target) { return 0; }
char *_DtHelpCeMapIdToSdlTopicId(void *vol, char *id) { return 0; }
char *_DtHelpCeGetCcdfVolLocale(void *vol) { return 0; }
char *_DtHelpCeGetSdlVolumeLocale(void *vol) { return 0; }
time_t _DtHelpCeGetCcdfDocStamp(void *vol) { return 0; }
time_t _DtHelpCeGetSdlDocStamp(void *vol) { return 0; }
void _DtHelpCeGetCcdfTopicChildren(void *vol, char *id, char ***children) {}
void _DtHelpCeGetSdlTopicChildren(void *vol, char *id, char ***children) {}
char *_DtHelpCeGetCcdfVolumeTitle(void *vol) { return 0; }
char *_DtHelpCeFrmtSDLVolTitleToAscii(char *title) { return 0; }
char *_DtHelpCeGetCcdfVolumeAbstract(void *vol) { return 0; }
char *_DtHelpCeGetSdlVolumeAsciiAbstract(void *vol) { return 0; }
int _DtHelpGetNxtToken(char *buf, char **token) { return 0; }
void _DtLinkDbDestroy(void *db) {}

/* Format Stubs */
void _DtHelpFormatVolumeTitle(void *vol, char *title) {}
void _DtHelpFormatTopicTitle(void *vol, char *title) {}
void _DtHelpFormatIndexEntries(void *vol, void *entries) {}
void _DtHelpFormatAsciiString(void *client_data, char *string) {}
void _DtHelpFormatAsciiStringDynamic(void *client_data, char *string) {}
void _DtHelpFormatManPage(void *client_data, char *man_page) {}
void _DtHelpFormatAsciiFile(void *client_data, char *file) {}
void _DtHelpFormatTopic(void *client_data, void *topic) {}
void _DtHelpFormatToc(void *client_data, void *toc) {}

/* Misc Stubs */
void _DtHelpDATocMarker(XtPointer client_data, Boolean flag) {}
void _DtHelpDADestroyRegion(void *region) {}
void _DtHelpLoadMultiInfo(void *client_data, void *info) {}
int dthelp_engine_get_height(void *engine) { return 0; }
char *_DtHelpGetAsciiVolumeTitle(void *vol) { return 0; }
