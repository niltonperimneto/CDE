/* CanvasEnums.h - Extracted enums from CanvasP.h to avoid conflicts with
 * DisplayAreaP.h */
#ifndef _DtHelpCanvasEnums_h
#define _DtHelpCanvasEnums_h

/* Window hints */
#define _DtCvWindowHint_PopupWindow 1
#define _DtCvWindowHint_CurrentWindow 2
#define _DtCvWindowHint_NewWindow 3
#define _DtCvWindowHint_Original 4

/* Rendering flags */
#define _DtCvLINK_FLAG (0x01 << 0)
#define _DtCvLINK_BEGIN (0x01 << 1)
#define _DtCvLINK_END (0x01 << 2)
#define _DtCvLINK_POP_UP (0x01 << 3)
#define _DtCvLINK_NEW_WINDOW (0x01 << 4)
#define _DtCvSEARCH_FLAG (0x01 << 5)
#define _DtCvSEARCH_BEGIN (0x01 << 6)
#define _DtCvSEARCH_END (0x01 << 7)
#define _DtCvSEARCH_CURR (0x01 << 8)
#define _DtCvMARK_FLAG (0x01 << 9)
#define _DtCvMARK_BEGIN (0x01 << 10)
#define _DtCvMARK_END (0x01 << 11)
#define _DtCvMARK_ON (0x01 << 12)
#define _DtCvTRAVERSAL_FLAG (0x01 << 13)
#define _DtCvTRAVERSAL_BEGIN (0x01 << 14)
#define _DtCvTRAVERSAL_END (0x01 << 15)

/* App flags */
#define _DtCvAPP_FIELD_OFFSET (16)
#define _DtCvAPP_FIELD_MASK (0x0f << _DtCvAPP_FIELD_OFFSET)
#define _DtCvAPP_FIELD_S_MASK (0x03 << _DtCvAPP_FIELD_OFFSET)
#define _DtCvAPP_FLAG1 (0x01 << (_DtCvAPP_FIELD_OFFSET + 0))
#define _DtCvAPP_FLAG2 (0x01 << (_DtCvAPP_FIELD_OFFSET + 1))
#define _DtCvAPP_FLAG3 (0x01 << (_DtCvAPP_FIELD_OFFSET + 2))
#define _DtCvAPP_FLAG4 (0x01 << (_DtCvAPP_FIELD_OFFSET + 3))

#define _DtCvSELECTED_FLAG (0x01 << 20)
#define _DtCvEND_OF_LINE (0x01 << 21)

#define _DtCvACTIVATE_SELECTION (0x01 << 0)
#define _DtCvACTIVATE_MARK (0x01 << 1)
#define _DtCvDEACTIVATE (0x01 << 2)
#define _DtCvACTIVATE_MARK_ON (0x01 << 3)
#define _DtCvACTIVATE_MARK_OFF (0x01 << 4)

/* Element types */
typedef enum {
  _DtCvBAD_TYPE_E,
  _DtCvCANVAS_TYPE_E,
  _DtCvLINE_TYPE_E,
  _DtCvLINK_TYPE_E,
  _DtCvLOCALE_TYPE_E,
  _DtCvMARK_TYPE_E,
  _DtCvREGION_TYPE_E,
  _DtCvSTRING_TYPE_E,
  _DtCvTRAVERSAL_TYPE_E
} _fake_DtCvElemType;

/* Define constants to map to the enum above if needed, but SDLI uses them
   directly? SDLI uses e.g. SdlElementNone. Wait, SDLI doesn't use _DtCvElemType
   members directly in enum. It uses _dtCvValue members.
*/

/* Canvas Engine values */
/* We define these as macros or enum members. Enum is safer for debugger.
   But DisplayAreaP.h defines _DtCvValue as int.
   So we can define an enum with a different tag to provide the constants. */
enum _DtCvValue_Constants {
  _DtCvFALSE = 0,
  _DtCvSTATUS_OK = 0,
  _DtCvTRUE = 1,
  _DtCvSTATUS_BAD = 1,
  _DtCvSTATUS_ID_BAD,
  _DtCvSTATUS_NONE,
  _DtCvSTATUS_LINK,
  _DtCvSTATUS_MARK,
  _DtCvRENDER_PARTIAL,
  _DtCvRENDER_COMPLETE,
  _DtCvTRAVERSAL_OFF,
  _DtCvTRAVERSAL_ON,
  _DtCvTRAVERSAL_TOP,
  _DtCvTRAVERSAL_NEXT,
  _DtCvTRAVERSAL_PREV,
  _DtCvTRAVERSAL_BOTTOM,
  _DtCvTRAVERSAL_ID,
  _DtCvTRAVERSAL_MARK,
  _DtCvSELECTION_CLEAR,
  _DtCvSELECTION_START,
  _DtCvSELECTION_UPDATE,
  _DtCvSELECTION_END,
  _DtCvUSE_BOUNDARY,
  _DtCvUSE_BOUNDARY_MOVE,
  _DtCvIGNORE_BOUNDARY,
  /* Missing ones found via grep will be added here */
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
  _DtCvJUSTIFY_LEFT_CORNER,
  _DtCvJUSTIFY_LEFT,
  _DtCvJUSTIFY_LEFT_MARGIN,
  _DtCvJUSTIFY_CENTER,
  _DtCvJUSTIFY_RIGHT_MARGIN,
  _DtCvJUSTIFY_RIGHT,
  _DtCvJUSTIFY_RIGHT_CORNER,
  _DtCvJUSTIFY_NUM,
  _DtCvJUSTIFY_TOP,
  _DtCvJUSTIFY_BOTTOM,
  _DtCvWRAP,
  _DtCvWRAP_NONE,
  _DtCvWRAP_JOIN
};

/* Define 'False' and 'True' if not defined? X11/Xlib.h defines them.
   Standard headers included in FormatMarkdown.c ensure they are defined. */

#endif /* _DtHelpCanvasEnums_h */
