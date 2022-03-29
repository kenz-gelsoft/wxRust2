#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::{c_int, c_long};

use crate::manual::*;

//  SKIP: wxTextEntryDialogStyle

pub const wxEL_ALLOW_NEW: c_int = 0x0100;
pub const wxEL_ALLOW_EDIT: c_int = 0x0200;
pub const wxEL_ALLOW_DELETE: c_int = 0x0400;
pub const wxEL_NO_REORDER: c_int = 0x0800;
pub const wxEL_DEFAULT_STYLE: c_int = (wxEL_ALLOW_NEW | wxEL_ALLOW_EDIT | wxEL_ALLOW_DELETE);

//  SKIP: wxPG_IT_CHILDREN
//  ENUM: wxPG_ITERATOR_FLAGS
pub const wxPG_ITERATE_PROPERTIES: c_long = (wxPG_PROP_PROPERTY|wxPG_PROP_MISC_PARENT|wxPG_PROP_AGGREGATE| 
                                       wxPG_PROP_COLLAPSED|((wxPG_PROP_MISC_PARENT|wxPG_PROP_CATEGORY)<<16));
pub const wxPG_ITERATE_HIDDEN: c_long = (wxPG_PROP_HIDDEN|wxPG_IT_CHILDREN(wxPG_PROP_COLLAPSED));
pub const wxPG_ITERATE_FIXED_CHILDREN: c_long = (wxPG_IT_CHILDREN(wxPG_PROP_AGGREGATE)|wxPG_ITERATE_PROPERTIES);
pub const wxPG_ITERATE_CATEGORIES: c_long = (wxPG_PROP_CATEGORY|wxPG_IT_CHILDREN(wxPG_PROP_CATEGORY)|wxPG_PROP_COLLAPSED);
pub const wxPG_ITERATE_ALL_PARENTS: c_long = (wxPG_PROP_MISC_PARENT|wxPG_PROP_AGGREGATE|wxPG_PROP_CATEGORY);
pub const wxPG_ITERATE_ALL_PARENTS_RECURSIVELY: c_long = (wxPG_ITERATE_ALL_PARENTS|wxPG_IT_CHILDREN(wxPG_ITERATE_ALL_PARENTS));
pub const wxPG_ITERATOR_FLAGS_ALL: c_long = (wxPG_PROP_PROPERTY|wxPG_PROP_MISC_PARENT|wxPG_PROP_AGGREGATE| 
                                      wxPG_PROP_HIDDEN|wxPG_PROP_CATEGORY|wxPG_PROP_COLLAPSED);
pub const wxPG_ITERATOR_MASK_OP_ITEM: c_long = wxPG_ITERATOR_FLAGS_ALL;
pub const wxPG_ITERATOR_MASK_OP_PARENT: c_long = wxPG_ITERATOR_FLAGS_ALL;
pub const wxPG_ITERATE_VISIBLE: c_long = (wxPG_ITERATE_PROPERTIES|wxPG_PROP_CATEGORY|wxPG_IT_CHILDREN(wxPG_PROP_AGGREGATE));
pub const wxPG_ITERATE_ALL: c_long = (wxPG_ITERATE_VISIBLE|wxPG_ITERATE_HIDDEN);
pub const wxPG_ITERATE_NORMAL: c_long = (wxPG_ITERATE_PROPERTIES|wxPG_ITERATE_HIDDEN);
pub const wxPG_ITERATE_DEFAULT: c_long = wxPG_ITERATE_NORMAL;

//  ENUM: Style
pub const Style_None: c_int = 0x00;
pub const Style_WithThousandsSep: c_int = 0x01;
pub const Style_NoTrailingZeroes: c_int = 0x02;

//  ENUM: @25
pub const wxID_HTML_PANEL: c_int = wxID_HIGHEST + 10;
pub const wxID_HTML_BACK: c_int = wxID_HIGHEST + 10 + 1;
pub const wxID_HTML_FORWARD: c_int = wxID_HIGHEST + 10 + 2;
pub const wxID_HTML_UPNODE: c_int = wxID_HIGHEST + 10 + 3;
pub const wxID_HTML_UP: c_int = wxID_HIGHEST + 10 + 4;
pub const wxID_HTML_DOWN: c_int = wxID_HIGHEST + 10 + 5;
pub const wxID_HTML_PRINT: c_int = wxID_HIGHEST + 10 + 6;
pub const wxID_HTML_OPENFILE: c_int = wxID_HIGHEST + 10 + 7;
pub const wxID_HTML_OPTIONS: c_int = wxID_HIGHEST + 10 + 8;
pub const wxID_HTML_BOOKMARKSLIST: c_int = wxID_HIGHEST + 10 + 9;
pub const wxID_HTML_BOOKMARKSADD: c_int = wxID_HIGHEST + 10 + 10;
pub const wxID_HTML_BOOKMARKSREMOVE: c_int = wxID_HIGHEST + 10 + 11;
pub const wxID_HTML_TREECTRL: c_int = wxID_HIGHEST + 10 + 12;
pub const wxID_HTML_INDEXPAGE: c_int = wxID_HIGHEST + 10 + 13;
pub const wxID_HTML_INDEXLIST: c_int = wxID_HIGHEST + 10 + 14;
pub const wxID_HTML_INDEXTEXT: c_int = wxID_HIGHEST + 10 + 15;
pub const wxID_HTML_INDEXBUTTON: c_int = wxID_HIGHEST + 10 + 16;
pub const wxID_HTML_INDEXBUTTONALL: c_int = wxID_HIGHEST + 10 + 17;
pub const wxID_HTML_NOTEBOOK: c_int = wxID_HIGHEST + 10 + 18;
pub const wxID_HTML_SEARCHPAGE: c_int = wxID_HIGHEST + 10 + 19;
pub const wxID_HTML_SEARCHTEXT: c_int = wxID_HIGHEST + 10 + 20;
pub const wxID_HTML_SEARCHLIST: c_int = wxID_HIGHEST + 10 + 21;
pub const wxID_HTML_SEARCHBUTTON: c_int = wxID_HIGHEST + 10 + 22;
pub const wxID_HTML_SEARCHCHOICE: c_int = wxID_HIGHEST + 10 + 23;
pub const wxID_HTML_COUNTINFO: c_int = wxID_HIGHEST + 10 + 24;

//  ENUM: wxURIHostType
pub const wxURI_REGNAME: c_int = 0;
pub const wxURI_IPV4ADDRESS: c_int = 0 + 1;
pub const wxURI_IPV6ADDRESS: c_int = 0 + 2;
pub const wxURI_IPVFUTURE: c_int = 0 + 3;

pub const wxPB_USE_TEXTCTRL: c_int = 0x0002;
pub const wxPB_SMALL: c_int = 0x8000;

pub const wxGA_HORIZONTAL: c_int = wxHORIZONTAL;
pub const wxGA_VERTICAL: c_int = wxVERTICAL;
pub const wxGA_SMOOTH: c_int = 0x0020;

// NODEF: wxDROP_ICON
//  ENUM: @12
pub const wxDrag_CopyOnly: c_int = 0;
pub const wxDrag_AllowMove: c_int = 1;
pub const wxDrag_DefaultMove: c_int = 3;
//  ENUM: wxDragResult
pub const wxDragError: c_int = 0;
pub const wxDragNone: c_int = 0 + 1;
pub const wxDragCopy: c_int = 0 + 2;
pub const wxDragMove: c_int = 0 + 3;
pub const wxDragLink: c_int = 0 + 4;
pub const wxDragCancel: c_int = 0 + 5;

pub const wxIMAGELIST_DRAW_NORMAL: c_int = 0x0001;
pub const wxIMAGELIST_DRAW_TRANSPARENT: c_int = 0x0002;
pub const wxIMAGELIST_DRAW_SELECTED: c_int = 0x0004;
pub const wxIMAGELIST_DRAW_FOCUSED: c_int = 0x0008;
//  ENUM: @32
pub const wxIMAGE_LIST_NORMAL: c_int = 0;
pub const wxIMAGE_LIST_SMALL: c_int = 0 + 1;
pub const wxIMAGE_LIST_STATE: c_int = 0 + 2;

pub const wxSIZE_AUTO_WIDTH: c_int = 0x0001;
pub const wxSIZE_AUTO_HEIGHT: c_int = 0x0002;
pub const wxSIZE_AUTO: c_int = (wxSIZE_AUTO_WIDTH|wxSIZE_AUTO_HEIGHT);
pub const wxSIZE_USE_EXISTING: c_int = 0x0000;
pub const wxSIZE_ALLOW_MINUS_ONE: c_int = 0x0004;
pub const wxSIZE_NO_ADJUSTMENTS: c_int = 0x0008;
pub const wxSIZE_FORCE: c_int = 0x0010;
pub const wxSIZE_FORCE_EVENT: c_int = 0x0020;
pub const wxVSCROLL: c_long = 0x80000000;
pub const wxHSCROLL: c_long = 0x40000000;
pub const wxCAPTION: c_long = 0x20000000;
pub const wxDOUBLE_BORDER: c_long = wxBORDER_DOUBLE;
pub const wxSUNKEN_BORDER: c_long = wxBORDER_SUNKEN;
pub const wxRAISED_BORDER: c_long = wxBORDER_RAISED;
pub const wxBORDER: c_long = wxBORDER_SIMPLE;
pub const wxSIMPLE_BORDER: c_long = wxBORDER_SIMPLE;
pub const wxSTATIC_BORDER: c_long = wxBORDER_STATIC;
pub const wxNO_BORDER: c_long = wxBORDER_NONE;
pub const wxALWAYS_SHOW_SB: c_long = 0x00800000;
pub const wxCLIP_CHILDREN: c_long = 0x00400000;
pub const wxCLIP_SIBLINGS: c_long = 0x20000000;
pub const wxTRANSPARENT_WINDOW: c_long = 0x00100000;
pub const wxTAB_TRAVERSAL: c_long = 0x00080000;
pub const wxWANTS_CHARS: c_long = 0x00040000;
pub const wxRETAINED: c_long = 0x00000000;
pub const wxBACKINGSTORE: c_long = wxRETAINED;
pub const wxPOPUP_WINDOW: c_long = 0x00020000;
pub const wxFULL_REPAINT_ON_RESIZE: c_long = 0x00010000;
pub const wxNO_FULL_REPAINT_ON_RESIZE: c_int = 0;
pub const wxWINDOW_STYLE_MASK: c_long = (wxVSCROLL|wxHSCROLL|wxBORDER_MASK|wxALWAYS_SHOW_SB|wxCLIP_CHILDREN| wxCLIP_SIBLINGS|wxTRANSPARENT_WINDOW|wxTAB_TRAVERSAL|wxWANTS_CHARS| wxRETAINED|wxPOPUP_WINDOW|wxFULL_REPAINT_ON_RESIZE);
pub const wxWS_EX_VALIDATE_RECURSIVELY: c_int = 0x00000001;
pub const wxWS_EX_BLOCK_EVENTS: c_int = 0x00000002;
pub const wxWS_EX_TRANSIENT: c_int = 0x00000004;
pub const wxWS_EX_THEMED_BACKGROUND: c_int = 0x00000008;
pub const wxWS_EX_PROCESS_IDLE: c_int = 0x00000010;
pub const wxWS_EX_PROCESS_UI_UPDATES: c_int = 0x00000020;
pub const wxFRAME_EX_METAL: c_int = 0x00000040;
pub const wxDIALOG_EX_METAL: c_int = 0x00000040;
pub const wxWS_EX_CONTEXTHELP: c_int = 0x00000080;
pub const wxFRAME_EX_CONTEXTHELP: c_int = wxWS_EX_CONTEXTHELP;
pub const wxDIALOG_EX_CONTEXTHELP: c_int = wxWS_EX_CONTEXTHELP;
pub const wxFRAME_DRAWER: c_int = 0x0020;
pub const wxFRAME_NO_WINDOW_MENU: c_int = 0x0100;
pub const wxMB_DOCKABLE: c_int = 0x0001;
pub const wxMENU_TEAROFF: c_int = 0x0001;
pub const wxCOLOURED: c_int = 0x0800;
pub const wxFIXED_LENGTH: c_int = 0x0400;
pub const wxLB_SORT: c_int = 0x0010;
pub const wxLB_SINGLE: c_int = 0x0020;
pub const wxLB_MULTIPLE: c_int = 0x0040;
pub const wxLB_EXTENDED: c_int = 0x0080;
pub const wxLB_NEEDED_SB: c_int = 0x0000;
pub const wxLB_OWNERDRAW: c_int = 0x0100;
pub const wxLB_ALWAYS_SB: c_int = 0x0200;
pub const wxLB_NO_SB: c_int = 0x0400;
pub const wxLB_HSCROLL: c_long = wxHSCROLL;
pub const wxLB_INT_HEIGHT: c_int = 0x0800;
pub const wxCB_SIMPLE: c_int = 0x0004;
pub const wxCB_SORT: c_int = 0x0008;
pub const wxCB_READONLY: c_int = 0x0010;
pub const wxCB_DROPDOWN: c_int = 0x0020;
pub const wxRA_LEFTTORIGHT: c_int = 0x0001;
pub const wxRA_TOPTOBOTTOM: c_int = 0x0002;
pub const wxRA_SPECIFY_COLS: c_int = wxHORIZONTAL;
pub const wxRA_SPECIFY_ROWS: c_int = wxVERTICAL;
pub const wxRA_HORIZONTAL: c_int = wxHORIZONTAL;
pub const wxRA_VERTICAL: c_int = wxVERTICAL;
pub const wxRB_GROUP: c_int = 0x0004;
pub const wxRB_SINGLE: c_int = 0x0008;
pub const wxSB_HORIZONTAL: c_int = wxHORIZONTAL;
pub const wxSB_VERTICAL: c_int = wxVERTICAL;
pub const wxSP_HORIZONTAL: c_int = wxHORIZONTAL /*  4 */;
pub const wxSP_VERTICAL: c_int = wxVERTICAL   /*  8 */;
pub const wxSP_ARROW_KEYS: c_int = 0x4000;
pub const wxSP_WRAP: c_int = 0x8000;
pub const wxTC_RIGHTJUSTIFY: c_int = 0x0010;
pub const wxTC_FIXEDWIDTH: c_int = 0x0020;
pub const wxTC_TOP: c_int = 0x0000    /*  default */;
pub const wxTC_LEFT: c_int = 0x0020;
pub const wxTC_RIGHT: c_int = 0x0040;
pub const wxTC_BOTTOM: c_int = 0x0080;
pub const wxTC_MULTILINE: c_int = 0x0200    /* == wxNB_MULTILINE */;
pub const wxTC_OWNERDRAW: c_int = 0x0400;
pub const wxBI_EXPAND: c_int = wxEXPAND;
pub const wxLI_HORIZONTAL: c_int = wxHORIZONTAL;
pub const wxLI_VERTICAL: c_int = wxVERTICAL;
pub const wxYES: c_int = 0x00000002;
pub const wxOK: c_long = 0x00000004;
pub const wxNO: c_int = 0x00000008;
pub const wxYES_NO: c_int = (wxYES | wxNO);
pub const wxCANCEL: c_long = 0x00000010;
pub const wxAPPLY: c_int = 0x00000020;
pub const wxCLOSE: c_int = 0x00000040;
pub const wxOK_DEFAULT: c_int = 0x00000000  /* has no effect (default) */;
pub const wxYES_DEFAULT: c_int = 0x00000000  /* has no effect (default) */;
pub const wxNO_DEFAULT: c_int = 0x00000080  /* only valid with wxYES_NO */;
pub const wxCANCEL_DEFAULT: c_long = 0x80000000  /* only valid with wxCANCEL */;
pub const wxICON_EXCLAMATION: c_int = 0x00000100;
pub const wxICON_HAND: c_int = 0x00000200;
pub const wxICON_WARNING: c_int = wxICON_EXCLAMATION;
pub const wxICON_ERROR: c_int = wxICON_HAND;
pub const wxICON_QUESTION: c_int = 0x00000400;
pub const wxICON_INFORMATION: c_int = 0x00000800;
pub const wxICON_STOP: c_int = wxICON_HAND;
pub const wxICON_ASTERISK: c_int = wxICON_INFORMATION;
pub const wxHELP: c_int = 0x00001000;
pub const wxFORWARD: c_int = 0x00002000;
pub const wxBACKWARD: c_int = 0x00004000;
pub const wxRESET: c_int = 0x00008000;
pub const wxMORE: c_int = 0x00010000;
pub const wxSETUP: c_int = 0x00020000;
pub const wxICON_NONE: c_int = 0x00040000;
pub const wxICON_AUTH_NEEDED: c_int = 0x00080000;
pub const wxICON_MASK: c_int = (wxICON_EXCLAMATION|wxICON_HAND|wxICON_QUESTION|wxICON_INFORMATION|wxICON_NONE);
pub const wxNOT_FOUND: c_int = (-1);
pub const wxPRINT_QUALITY_HIGH: c_int = -1;
pub const wxPRINT_QUALITY_MEDIUM: c_int = -2;
pub const wxPRINT_QUALITY_LOW: c_int = -3;
pub const wxPRINT_QUALITY_DRAFT: c_int = -4;
pub const wxSTAY_ON_TOP: c_int = 0x8000;
pub const wxICONIZE: c_int = 0x4000;
pub const wxMINIMIZE: c_int = wxICONIZE;
pub const wxMAXIMIZE: c_int = 0x2000;
pub const wxCLOSE_BOX: c_long = 0x1000;
pub const wxSYSTEM_MENU: c_long = 0x0800;
pub const wxMINIMIZE_BOX: c_long = 0x0400;
pub const wxMAXIMIZE_BOX: c_long = 0x0200;
pub const wxTINY_CAPTION: c_int = 0x0080;
pub const wxRESIZE_BORDER: c_long = 0x0040;
//  SKIP: wxINT8_MIN
//  SKIP: wxINT8_MAX
//  SKIP: wxUINT8_MAX
//  SKIP: wxINT16_MIN
//  SKIP: wxINT16_MAX
//  SKIP: wxUINT16_MAX
//  SKIP: wxINT32_MIN
//  SKIP: wxINT32_MAX
//  SKIP: wxUINT32_MAX
//  SKIP: wxINT64_MIN
//  SKIP: wxINT64_MAX
//  SKIP: wxUINT64_MAX
// NODEF: wxINT32_SWAP_ALWAYS
// NODEF: wxUINT32_SWAP_ALWAYS
// NODEF: wxINT16_SWAP_ALWAYS
// NODEF: wxUINT16_SWAP_ALWAYS
// NODEF: wxINT32_SWAP_ON_BE
// NODEF: wxUINT32_SWAP_ON_BE
// NODEF: wxINT16_SWAP_ON_BE
// NODEF: wxUINT16_SWAP_ON_BE
// NODEF: wxINT32_SWAP_ON_LE
// NODEF: wxUINT32_SWAP_ON_LE
// NODEF: wxINT16_SWAP_ON_LE
// NODEF: wxUINT16_SWAP_ON_LE
// NODEF: wxDECLARE_NO_ASSIGN_CLASS
// NODEF: wxDECLARE_NO_COPY_CLASS
// NODEF: wxDECLARE_NO_COPY_TEMPLATE_CLASS
// NODEF: wxDECLARE_NO_COPY_TEMPLATE_CLASS_2
// NODEF: wxDEPRECATED
// NODEF: wxDEPRECATED_BUT_USED_INTERNALLY
// NODEF: wxDEPRECATED_INLINE
// NODEF: wxDEPRECATED_ACCESSOR
// NODEF: wxDEPRECATED_BUT_USED_INTERNALLY_INLINE
// NODEF: wxEXPLICIT
// NODEF: wxSUPPRESS_GCC_PRIVATE_DTOR_WARNING
//  ENUM: wxGeometryCentre
pub const wxCENTRE: c_long = 0x0001;
pub const wxCENTER: c_long = wxCENTRE;
//  ENUM: wxOrientation
pub const wxHORIZONTAL: c_int = 0x0004;
pub const wxVERTICAL: c_int = 0x0008;
pub const wxBOTH: c_int = wxVERTICAL | wxHORIZONTAL;
pub const wxORIENTATION_MASK: c_int = wxBOTH;
//  ENUM: wxDirection
pub const wxLEFT: c_int = 0x0010;
pub const wxRIGHT: c_int = 0x0020;
pub const wxUP: c_int = 0x0040;
pub const wxDOWN: c_int = 0x0080;
pub const wxTOP: c_int = wxUP;
pub const wxBOTTOM: c_int = wxDOWN;
pub const wxNORTH: c_int = wxUP;
pub const wxSOUTH: c_int = wxDOWN;
pub const wxWEST: c_int = wxLEFT;
pub const wxEAST: c_int = wxRIGHT;
pub const wxALL: c_int = (wxUP | wxDOWN | wxRIGHT | wxLEFT);
pub const wxDIRECTION_MASK: c_int = wxALL;
//  ENUM: wxAlignment
pub const wxALIGN_INVALID: c_int = -1;
pub const wxALIGN_NOT: c_int = 0x0000;
pub const wxALIGN_CENTER_HORIZONTAL: c_int = 0x0100;
pub const wxALIGN_CENTRE_HORIZONTAL: c_int = wxALIGN_CENTER_HORIZONTAL;
pub const wxALIGN_LEFT: c_int = wxALIGN_NOT;
pub const wxALIGN_TOP: c_int = wxALIGN_NOT;
pub const wxALIGN_RIGHT: c_int = 0x0200;
pub const wxALIGN_BOTTOM: c_int = 0x0400;
pub const wxALIGN_CENTER_VERTICAL: c_int = 0x0800;
pub const wxALIGN_CENTRE_VERTICAL: c_int = wxALIGN_CENTER_VERTICAL;
pub const wxALIGN_CENTER: c_int = (wxALIGN_CENTER_HORIZONTAL | wxALIGN_CENTER_VERTICAL);
pub const wxALIGN_CENTRE: c_int = wxALIGN_CENTER;
pub const wxALIGN_MASK: c_int = 0x0f00;
//  ENUM: wxSizerFlagBits
pub const wxFIXED_MINSIZE: c_int = 0x8000;
pub const wxRESERVE_SPACE_EVEN_IF_HIDDEN: c_int = 0x0002;
pub const wxSIZER_FLAG_BITS_MASK: c_int = 0x8002;
//  ENUM: wxStretch
pub const wxSTRETCH_NOT: c_int = 0x0000;
pub const wxSHRINK: c_int = 0x1000;
pub const wxGROW: c_int = 0x2000;
pub const wxEXPAND: c_int = wxGROW;
pub const wxSHAPED: c_int = 0x4000;
pub const wxTILE: c_int = wxSHAPED | wxFIXED_MINSIZE;
pub const wxSTRETCH_MASK: c_int = 0x7000;
//  ENUM: wxBorder
pub const wxBORDER_DEFAULT: c_long = 0;
pub const wxBORDER_NONE: c_long = 0x00200000;
pub const wxBORDER_STATIC: c_long = 0x01000000;
pub const wxBORDER_SIMPLE: c_long = 0x02000000;
pub const wxBORDER_RAISED: c_long = 0x04000000;
pub const wxBORDER_SUNKEN: c_long = 0x08000000;
pub const wxBORDER_DOUBLE: c_long = 0x10000000;
pub const wxBORDER_THEME: c_long = wxBORDER_DOUBLE;
pub const wxBORDER_MASK: c_long = 0x1f200000;
//  ENUM: wxBackgroundStyle
pub const wxBG_STYLE_ERASE: c_int = 0;
pub const wxBG_STYLE_SYSTEM: c_int = 0 + 1;
pub const wxBG_STYLE_PAINT: c_int = 0 + 2;
pub const wxBG_STYLE_COLOUR: c_int = 0 + 3;
pub const wxBG_STYLE_TRANSPARENT: c_int = 0 + 4;
//  ENUM: wxStandardID
pub const wxID_AUTO_LOWEST: c_int = 0;
pub const wxID_AUTO_HIGHEST: c_int = 0 + 1;
pub const wxID_NONE: c_int = -3;
pub const wxID_SEPARATOR: c_int = -2;
pub const wxID_ANY: c_int = -1;
pub const wxID_LOWEST: c_int = 4999;
pub const wxID_OPEN: c_int = 4999 + 1;
pub const wxID_CLOSE: c_int = 4999 + 2;
pub const wxID_NEW: c_int = 4999 + 3;
pub const wxID_SAVE: c_int = 4999 + 4;
pub const wxID_SAVEAS: c_int = 4999 + 5;
pub const wxID_REVERT: c_int = 4999 + 6;
pub const wxID_EXIT: c_int = 4999 + 7;
pub const wxID_UNDO: c_int = 4999 + 8;
pub const wxID_REDO: c_int = 4999 + 9;
pub const wxID_HELP: c_int = 4999 + 10;
pub const wxID_PRINT: c_int = 4999 + 11;
pub const wxID_PRINT_SETUP: c_int = 4999 + 12;
pub const wxID_PAGE_SETUP: c_int = 4999 + 13;
pub const wxID_PREVIEW: c_int = 4999 + 14;
pub const wxID_ABOUT: c_int = 4999 + 15;
pub const wxID_HELP_CONTENTS: c_int = 4999 + 16;
pub const wxID_HELP_INDEX: c_int = 4999 + 17;
pub const wxID_HELP_SEARCH: c_int = 4999 + 18;
pub const wxID_HELP_COMMANDS: c_int = 4999 + 19;
pub const wxID_HELP_PROCEDURES: c_int = 4999 + 20;
pub const wxID_HELP_CONTEXT: c_int = 4999 + 21;
pub const wxID_CLOSE_ALL: c_int = 4999 + 22;
pub const wxID_PREFERENCES: c_int = 4999 + 23;
pub const wxID_EDIT: c_int = 5030;
pub const wxID_CUT: c_int = 5030 + 1;
pub const wxID_COPY: c_int = 5030 + 2;
pub const wxID_PASTE: c_int = 5030 + 3;
pub const wxID_CLEAR: c_int = 5030 + 4;
pub const wxID_FIND: c_int = 5030 + 5;
pub const wxID_DUPLICATE: c_int = 5030 + 6;
pub const wxID_SELECTALL: c_int = 5030 + 7;
pub const wxID_DELETE: c_int = 5030 + 8;
pub const wxID_REPLACE: c_int = 5030 + 9;
pub const wxID_REPLACE_ALL: c_int = 5030 + 10;
pub const wxID_PROPERTIES: c_int = 5030 + 11;
pub const wxID_VIEW_DETAILS: c_int = 5030 + 12;
pub const wxID_VIEW_LARGEICONS: c_int = 5030 + 13;
pub const wxID_VIEW_SMALLICONS: c_int = 5030 + 14;
pub const wxID_VIEW_LIST: c_int = 5030 + 15;
pub const wxID_VIEW_SORTDATE: c_int = 5030 + 16;
pub const wxID_VIEW_SORTNAME: c_int = 5030 + 17;
pub const wxID_VIEW_SORTSIZE: c_int = 5030 + 18;
pub const wxID_VIEW_SORTTYPE: c_int = 5030 + 19;
pub const wxID_FILE: c_int = 5050;
pub const wxID_FILE1: c_int = 5050 + 1;
pub const wxID_FILE2: c_int = 5050 + 2;
pub const wxID_FILE3: c_int = 5050 + 3;
pub const wxID_FILE4: c_int = 5050 + 4;
pub const wxID_FILE5: c_int = 5050 + 5;
pub const wxID_FILE6: c_int = 5050 + 6;
pub const wxID_FILE7: c_int = 5050 + 7;
pub const wxID_FILE8: c_int = 5050 + 8;
pub const wxID_FILE9: c_int = 5050 + 9;
pub const wxID_OK: c_int = 5100;
pub const wxID_CANCEL: c_int = 5100 + 1;
pub const wxID_APPLY: c_int = 5100 + 2;
pub const wxID_YES: c_int = 5100 + 3;
pub const wxID_NO: c_int = 5100 + 4;
pub const wxID_STATIC: c_int = 5100 + 5;
pub const wxID_FORWARD: c_int = 5100 + 6;
pub const wxID_BACKWARD: c_int = 5100 + 7;
pub const wxID_DEFAULT: c_int = 5100 + 8;
pub const wxID_MORE: c_int = 5100 + 9;
pub const wxID_SETUP: c_int = 5100 + 10;
pub const wxID_RESET: c_int = 5100 + 11;
pub const wxID_CONTEXT_HELP: c_int = 5100 + 12;
pub const wxID_YESTOALL: c_int = 5100 + 13;
pub const wxID_NOTOALL: c_int = 5100 + 14;
pub const wxID_ABORT: c_int = 5100 + 15;
pub const wxID_RETRY: c_int = 5100 + 16;
pub const wxID_IGNORE: c_int = 5100 + 17;
pub const wxID_ADD: c_int = 5100 + 18;
pub const wxID_REMOVE: c_int = 5100 + 19;
pub const wxID_UP: c_int = 5100 + 20;
pub const wxID_DOWN: c_int = 5100 + 21;
pub const wxID_HOME: c_int = 5100 + 22;
pub const wxID_REFRESH: c_int = 5100 + 23;
pub const wxID_STOP: c_int = 5100 + 24;
pub const wxID_INDEX: c_int = 5100 + 25;
pub const wxID_BOLD: c_int = 5100 + 26;
pub const wxID_ITALIC: c_int = 5100 + 27;
pub const wxID_JUSTIFY_CENTER: c_int = 5100 + 28;
pub const wxID_JUSTIFY_FILL: c_int = 5100 + 29;
pub const wxID_JUSTIFY_RIGHT: c_int = 5100 + 30;
pub const wxID_JUSTIFY_LEFT: c_int = 5100 + 31;
pub const wxID_UNDERLINE: c_int = 5100 + 32;
pub const wxID_INDENT: c_int = 5100 + 33;
pub const wxID_UNINDENT: c_int = 5100 + 34;
pub const wxID_ZOOM_100: c_int = 5100 + 35;
pub const wxID_ZOOM_FIT: c_int = 5100 + 36;
pub const wxID_ZOOM_IN: c_int = 5100 + 37;
pub const wxID_ZOOM_OUT: c_int = 5100 + 38;
pub const wxID_UNDELETE: c_int = 5100 + 39;
pub const wxID_REVERT_TO_SAVED: c_int = 5100 + 40;
pub const wxID_CDROM: c_int = 5100 + 41;
pub const wxID_CONVERT: c_int = 5100 + 42;
pub const wxID_EXECUTE: c_int = 5100 + 43;
pub const wxID_FLOPPY: c_int = 5100 + 44;
pub const wxID_HARDDISK: c_int = 5100 + 45;
pub const wxID_BOTTOM: c_int = 5100 + 46;
pub const wxID_FIRST: c_int = 5100 + 47;
pub const wxID_LAST: c_int = 5100 + 48;
pub const wxID_TOP: c_int = 5100 + 49;
pub const wxID_INFO: c_int = 5100 + 50;
pub const wxID_JUMP_TO: c_int = 5100 + 51;
pub const wxID_NETWORK: c_int = 5100 + 52;
pub const wxID_SELECT_COLOR: c_int = 5100 + 53;
pub const wxID_SELECT_FONT: c_int = 5100 + 54;
pub const wxID_SORT_ASCENDING: c_int = 5100 + 55;
pub const wxID_SORT_DESCENDING: c_int = 5100 + 56;
pub const wxID_SPELL_CHECK: c_int = 5100 + 57;
pub const wxID_STRIKETHROUGH: c_int = 5100 + 58;
pub const wxID_SYSTEM_MENU: c_int = 5200;
pub const wxID_CLOSE_FRAME: c_int = 5200 + 1;
pub const wxID_MOVE_FRAME: c_int = 5200 + 2;
pub const wxID_RESIZE_FRAME: c_int = 5200 + 3;
pub const wxID_MAXIMIZE_FRAME: c_int = 5200 + 4;
pub const wxID_ICONIZE_FRAME: c_int = 5200 + 5;
pub const wxID_RESTORE_FRAME: c_int = 5200 + 6;
pub const wxID_MDI_WINDOW_FIRST: c_int = 5230;
pub const wxID_MDI_WINDOW_CASCADE: c_int = wxID_MDI_WINDOW_FIRST;
pub const wxID_MDI_WINDOW_TILE_HORZ: c_int = wxID_MDI_WINDOW_FIRST + 1;
pub const wxID_MDI_WINDOW_TILE_VERT: c_int = wxID_MDI_WINDOW_FIRST + 2;
pub const wxID_MDI_WINDOW_ARRANGE_ICONS: c_int = wxID_MDI_WINDOW_FIRST + 3;
pub const wxID_MDI_WINDOW_PREV: c_int = wxID_MDI_WINDOW_FIRST + 4;
pub const wxID_MDI_WINDOW_NEXT: c_int = wxID_MDI_WINDOW_FIRST + 5;
pub const wxID_MDI_WINDOW_LAST: c_int = wxID_MDI_WINDOW_NEXT;
pub const wxID_FILEDLGG: c_int = 5900;
pub const wxID_FILECTRL: c_int = 5950;
pub const wxID_HIGHEST: c_int = 5999;
//  ENUM: wxItemKind
pub const wxITEM_SEPARATOR: c_int = -1;
pub const wxITEM_NORMAL: c_int = -1 + 1;
pub const wxITEM_CHECK: c_int = -1 + 2;
pub const wxITEM_RADIO: c_int = -1 + 3;
pub const wxITEM_DROPDOWN: c_int = -1 + 4;
pub const wxITEM_MAX: c_int = -1 + 5;
//  ENUM: wxHitTest
pub const wxHT_NOWHERE: c_int = 0;
pub const wxHT_SCROLLBAR_FIRST: c_int = wxHT_NOWHERE;
pub const wxHT_SCROLLBAR_ARROW_LINE_1: c_int = wxHT_NOWHERE + 1;
pub const wxHT_SCROLLBAR_ARROW_LINE_2: c_int = wxHT_NOWHERE + 2;
pub const wxHT_SCROLLBAR_ARROW_PAGE_1: c_int = wxHT_NOWHERE + 3;
pub const wxHT_SCROLLBAR_ARROW_PAGE_2: c_int = wxHT_NOWHERE + 4;
pub const wxHT_SCROLLBAR_THUMB: c_int = wxHT_NOWHERE + 5;
pub const wxHT_SCROLLBAR_BAR_1: c_int = wxHT_NOWHERE + 6;
pub const wxHT_SCROLLBAR_BAR_2: c_int = wxHT_NOWHERE + 7;
pub const wxHT_SCROLLBAR_LAST: c_int = wxHT_NOWHERE + 8;
pub const wxHT_WINDOW_OUTSIDE: c_int = wxHT_NOWHERE + 9;
pub const wxHT_WINDOW_INSIDE: c_int = wxHT_NOWHERE + 10;
pub const wxHT_WINDOW_VERT_SCROLLBAR: c_int = wxHT_NOWHERE + 11;
pub const wxHT_WINDOW_HORZ_SCROLLBAR: c_int = wxHT_NOWHERE + 12;
pub const wxHT_WINDOW_CORNER: c_int = wxHT_NOWHERE + 13;
pub const wxHT_MAX: c_int = wxHT_NOWHERE + 14;
//  ENUM: wxDataFormatId
pub const wxDF_INVALID: c_int =          0;
pub const wxDF_TEXT: c_int =             1;
pub const wxDF_BITMAP: c_int =           2;
pub const wxDF_METAFILE: c_int =         3;
pub const wxDF_SYLK: c_int =             4;
pub const wxDF_DIF: c_int =              5;
pub const wxDF_TIFF: c_int =             6;
pub const wxDF_OEMTEXT: c_int =          7;
pub const wxDF_DIB: c_int =              8;
pub const wxDF_PALETTE: c_int =          9;
pub const wxDF_PENDATA: c_int =          10;
pub const wxDF_RIFF: c_int =             11;
pub const wxDF_WAVE: c_int =             12;
pub const wxDF_UNICODETEXT: c_int =      13;
pub const wxDF_ENHMETAFILE: c_int =      14;
pub const wxDF_FILENAME: c_int =         15;
pub const wxDF_LOCALE: c_int =           16;
pub const wxDF_PRIVATE: c_int =          20;
pub const wxDF_HTML: c_int =             30;
pub const wxDF_MAX: c_int =             30 + 1;
//  ENUM: wxKeyCode
pub const WXK_NONE: c_int =    0;
pub const WXK_CONTROL_A: c_int = 1;
pub const WXK_CONTROL_B: c_int = 1 + 1;
pub const WXK_CONTROL_C: c_int = 1 + 2;
pub const WXK_CONTROL_D: c_int = 1 + 3;
pub const WXK_CONTROL_E: c_int = 1 + 4;
pub const WXK_CONTROL_F: c_int = 1 + 5;
pub const WXK_CONTROL_G: c_int = 1 + 6;
pub const WXK_CONTROL_H: c_int = 1 + 7;
pub const WXK_CONTROL_I: c_int = 1 + 8;
pub const WXK_CONTROL_J: c_int = 1 + 9;
pub const WXK_CONTROL_K: c_int = 1 + 10;
pub const WXK_CONTROL_L: c_int = 1 + 11;
pub const WXK_CONTROL_M: c_int = 1 + 12;
pub const WXK_CONTROL_N: c_int = 1 + 13;
pub const WXK_CONTROL_O: c_int = 1 + 14;
pub const WXK_CONTROL_P: c_int = 1 + 15;
pub const WXK_CONTROL_Q: c_int = 1 + 16;
pub const WXK_CONTROL_R: c_int = 1 + 17;
pub const WXK_CONTROL_S: c_int = 1 + 18;
pub const WXK_CONTROL_T: c_int = 1 + 19;
pub const WXK_CONTROL_U: c_int = 1 + 20;
pub const WXK_CONTROL_V: c_int = 1 + 21;
pub const WXK_CONTROL_W: c_int = 1 + 22;
pub const WXK_CONTROL_X: c_int = 1 + 23;
pub const WXK_CONTROL_Y: c_int = 1 + 24;
pub const WXK_CONTROL_Z: c_int = 1 + 25;
pub const WXK_BACK: c_int =    8;
pub const WXK_TAB: c_int =    9;
pub const WXK_RETURN: c_int =    13;
pub const WXK_ESCAPE: c_int =    27;
pub const WXK_SPACE: c_int =    32;
pub const WXK_DELETE: c_int =    127;
pub const WXK_START: c_int = 300;
pub const WXK_LBUTTON: c_int = 300 + 1;
pub const WXK_RBUTTON: c_int = 300 + 2;
pub const WXK_CANCEL: c_int = 300 + 3;
pub const WXK_MBUTTON: c_int = 300 + 4;
pub const WXK_CLEAR: c_int = 300 + 5;
pub const WXK_SHIFT: c_int = 300 + 6;
pub const WXK_ALT: c_int = 300 + 7;
pub const WXK_CONTROL: c_int = 300 + 8;
pub const WXK_RAW_CONTROL: c_int = 300 + 9;
pub const WXK_MENU: c_int = 300 + 10;
pub const WXK_PAUSE: c_int = 300 + 11;
pub const WXK_CAPITAL: c_int = 300 + 12;
pub const WXK_END: c_int = 300 + 13;
pub const WXK_HOME: c_int = 300 + 14;
pub const WXK_LEFT: c_int = 300 + 15;
pub const WXK_UP: c_int = 300 + 16;
pub const WXK_RIGHT: c_int = 300 + 17;
pub const WXK_DOWN: c_int = 300 + 18;
pub const WXK_SELECT: c_int = 300 + 19;
pub const WXK_PRINT: c_int = 300 + 20;
pub const WXK_EXECUTE: c_int = 300 + 21;
pub const WXK_SNAPSHOT: c_int = 300 + 22;
pub const WXK_INSERT: c_int = 300 + 23;
pub const WXK_HELP: c_int = 300 + 24;
pub const WXK_NUMPAD0: c_int = 300 + 25;
pub const WXK_NUMPAD1: c_int = 300 + 26;
pub const WXK_NUMPAD2: c_int = 300 + 27;
pub const WXK_NUMPAD3: c_int = 300 + 28;
pub const WXK_NUMPAD4: c_int = 300 + 29;
pub const WXK_NUMPAD5: c_int = 300 + 30;
pub const WXK_NUMPAD6: c_int = 300 + 31;
pub const WXK_NUMPAD7: c_int = 300 + 32;
pub const WXK_NUMPAD8: c_int = 300 + 33;
pub const WXK_NUMPAD9: c_int = 300 + 34;
pub const WXK_MULTIPLY: c_int = 300 + 35;
pub const WXK_ADD: c_int = 300 + 36;
pub const WXK_SEPARATOR: c_int = 300 + 37;
pub const WXK_SUBTRACT: c_int = 300 + 38;
pub const WXK_DECIMAL: c_int = 300 + 39;
pub const WXK_DIVIDE: c_int = 300 + 40;
pub const WXK_F1: c_int = 300 + 41;
pub const WXK_F2: c_int = 300 + 42;
pub const WXK_F3: c_int = 300 + 43;
pub const WXK_F4: c_int = 300 + 44;
pub const WXK_F5: c_int = 300 + 45;
pub const WXK_F6: c_int = 300 + 46;
pub const WXK_F7: c_int = 300 + 47;
pub const WXK_F8: c_int = 300 + 48;
pub const WXK_F9: c_int = 300 + 49;
pub const WXK_F10: c_int = 300 + 50;
pub const WXK_F11: c_int = 300 + 51;
pub const WXK_F12: c_int = 300 + 52;
pub const WXK_F13: c_int = 300 + 53;
pub const WXK_F14: c_int = 300 + 54;
pub const WXK_F15: c_int = 300 + 55;
pub const WXK_F16: c_int = 300 + 56;
pub const WXK_F17: c_int = 300 + 57;
pub const WXK_F18: c_int = 300 + 58;
pub const WXK_F19: c_int = 300 + 59;
pub const WXK_F20: c_int = 300 + 60;
pub const WXK_F21: c_int = 300 + 61;
pub const WXK_F22: c_int = 300 + 62;
pub const WXK_F23: c_int = 300 + 63;
pub const WXK_F24: c_int = 300 + 64;
pub const WXK_NUMLOCK: c_int = 300 + 65;
pub const WXK_SCROLL: c_int = 300 + 66;
pub const WXK_PAGEUP: c_int = 300 + 67;
pub const WXK_PAGEDOWN: c_int = 300 + 68;
pub const WXK_NUMPAD_SPACE: c_int = 300 + 69;
pub const WXK_NUMPAD_TAB: c_int = 300 + 70;
pub const WXK_NUMPAD_ENTER: c_int = 300 + 71;
pub const WXK_NUMPAD_F1: c_int = 300 + 72;
pub const WXK_NUMPAD_F2: c_int = 300 + 73;
pub const WXK_NUMPAD_F3: c_int = 300 + 74;
pub const WXK_NUMPAD_F4: c_int = 300 + 75;
pub const WXK_NUMPAD_HOME: c_int = 300 + 76;
pub const WXK_NUMPAD_LEFT: c_int = 300 + 77;
pub const WXK_NUMPAD_UP: c_int = 300 + 78;
pub const WXK_NUMPAD_RIGHT: c_int = 300 + 79;
pub const WXK_NUMPAD_DOWN: c_int = 300 + 80;
pub const WXK_NUMPAD_PAGEUP: c_int = 300 + 81;
pub const WXK_NUMPAD_PAGEDOWN: c_int = 300 + 82;
pub const WXK_NUMPAD_END: c_int = 300 + 83;
pub const WXK_NUMPAD_BEGIN: c_int = 300 + 84;
pub const WXK_NUMPAD_INSERT: c_int = 300 + 85;
pub const WXK_NUMPAD_DELETE: c_int = 300 + 86;
pub const WXK_NUMPAD_EQUAL: c_int = 300 + 87;
pub const WXK_NUMPAD_MULTIPLY: c_int = 300 + 88;
pub const WXK_NUMPAD_ADD: c_int = 300 + 89;
pub const WXK_NUMPAD_SEPARATOR: c_int = 300 + 90;
pub const WXK_NUMPAD_SUBTRACT: c_int = 300 + 91;
pub const WXK_NUMPAD_DECIMAL: c_int = 300 + 92;
pub const WXK_NUMPAD_DIVIDE: c_int = 300 + 93;
pub const WXK_WINDOWS_LEFT: c_int = 300 + 94;
pub const WXK_WINDOWS_RIGHT: c_int = 300 + 95;
pub const WXK_WINDOWS_MENU: c_int = 300 + 96;
pub const WXK_COMMAND: c_int = 300 + 97;
pub const WXK_SPECIAL1: c_int = 193;
pub const WXK_SPECIAL2: c_int = 193 + 1;
pub const WXK_SPECIAL3: c_int = 193 + 2;
pub const WXK_SPECIAL4: c_int = 193 + 3;
pub const WXK_SPECIAL5: c_int = 193 + 4;
pub const WXK_SPECIAL6: c_int = 193 + 5;
pub const WXK_SPECIAL7: c_int = 193 + 6;
pub const WXK_SPECIAL8: c_int = 193 + 7;
pub const WXK_SPECIAL9: c_int = 193 + 8;
pub const WXK_SPECIAL10: c_int = 193 + 9;
pub const WXK_SPECIAL11: c_int = 193 + 10;
pub const WXK_SPECIAL12: c_int = 193 + 11;
pub const WXK_SPECIAL13: c_int = 193 + 12;
pub const WXK_SPECIAL14: c_int = 193 + 13;
pub const WXK_SPECIAL15: c_int = 193 + 14;
pub const WXK_SPECIAL16: c_int = 193 + 15;
pub const WXK_SPECIAL17: c_int = 193 + 16;
pub const WXK_SPECIAL18: c_int = 193 + 17;
pub const WXK_SPECIAL19: c_int = 193 + 18;
pub const WXK_SPECIAL20: c_int = 193 + 19;
//  ENUM: wxKeyModifier
pub const wxMOD_NONE: c_int = 0x0000;
pub const wxMOD_ALT: c_int = 0x0001;
pub const wxMOD_CONTROL: c_int = 0x0002;
pub const wxMOD_ALTGR: c_int = wxMOD_ALT | wxMOD_CONTROL;
pub const wxMOD_SHIFT: c_int = 0x0004;
pub const wxMOD_META: c_int = 0x0008;
pub const wxMOD_WIN: c_int = wxMOD_META;
pub const wxMOD_RAW_CONTROL: c_int = wxMOD_META + 1;
pub const wxMOD_CMD: c_int = wxMOD_CONTROL;
pub const wxMOD_ALL: c_int = 0xffff;
//  ENUM: wxPaperSize
pub const wxPAPER_10X11: c_int = 0;
pub const wxPAPER_10X14: c_int = 0 + 1;
pub const wxPAPER_11X17: c_int = 0 + 2;
pub const wxPAPER_12X11: c_int = 0 + 3;
pub const wxPAPER_15X11: c_int = 0 + 4;
pub const wxPAPER_9X11: c_int = 0 + 5;
pub const wxPAPER_A2: c_int = 0 + 6;
pub const wxPAPER_A3: c_int = 0 + 7;
pub const wxPAPER_A3_EXTRA: c_int = 0 + 8;
pub const wxPAPER_A3_EXTRA_TRANSVERSE: c_int = 0 + 9;
pub const wxPAPER_A3_ROTATED: c_int = 0 + 10;
pub const wxPAPER_A3_TRANSVERSE: c_int = 0 + 11;
pub const wxPAPER_A4: c_int = 0 + 12;
pub const wxPAPER_A4SMALL: c_int = 0 + 13;
pub const wxPAPER_A4_EXTRA: c_int = 0 + 14;
pub const wxPAPER_A4_PLUS: c_int = 0 + 15;
pub const wxPAPER_A4_ROTATED: c_int = 0 + 16;
pub const wxPAPER_A4_TRANSVERSE: c_int = 0 + 17;
pub const wxPAPER_A5: c_int = 0 + 18;
pub const wxPAPER_A5_EXTRA: c_int = 0 + 19;
pub const wxPAPER_A5_ROTATED: c_int = 0 + 20;
pub const wxPAPER_A5_TRANSVERSE: c_int = 0 + 21;
pub const wxPAPER_A6: c_int = 0 + 22;
pub const wxPAPER_A6_ROTATED: c_int = 0 + 23;
pub const wxPAPER_A_PLUS: c_int = 0 + 24;
pub const wxPAPER_B4: c_int = 0 + 25;
pub const wxPAPER_B4_JIS_ROTATED: c_int = 0 + 26;
pub const wxPAPER_B5: c_int = 0 + 27;
pub const wxPAPER_B5_EXTRA: c_int = 0 + 28;
pub const wxPAPER_B5_JIS_ROTATED: c_int = 0 + 29;
pub const wxPAPER_B5_TRANSVERSE: c_int = 0 + 30;
pub const wxPAPER_B6_JIS: c_int = 0 + 31;
pub const wxPAPER_B6_JIS_ROTATED: c_int = 0 + 32;
pub const wxPAPER_B_PLUS: c_int = 0 + 33;
pub const wxPAPER_CSHEET: c_int = 0 + 34;
pub const wxPAPER_DBL_JAPANESE_POSTCARD: c_int = 0 + 35;
pub const wxPAPER_DBL_JAPANESE_POSTCARD_ROTATED: c_int = 0 + 36;
pub const wxPAPER_DSHEET: c_int = 0 + 37;
pub const wxPAPER_ENV_10: c_int = 0 + 38;
pub const wxPAPER_ENV_11: c_int = 0 + 39;
pub const wxPAPER_ENV_12: c_int = 0 + 40;
pub const wxPAPER_ENV_14: c_int = 0 + 41;
pub const wxPAPER_ENV_9: c_int = 0 + 42;
pub const wxPAPER_ENV_B4: c_int = 0 + 43;
pub const wxPAPER_ENV_B5: c_int = 0 + 44;
pub const wxPAPER_ENV_B6: c_int = 0 + 45;
pub const wxPAPER_ENV_C3: c_int = 0 + 46;
pub const wxPAPER_ENV_C4: c_int = 0 + 47;
pub const wxPAPER_ENV_C5: c_int = 0 + 48;
pub const wxPAPER_ENV_C6: c_int = 0 + 49;
pub const wxPAPER_ENV_C65: c_int = 0 + 50;
pub const wxPAPER_ENV_DL: c_int = 0 + 51;
pub const wxPAPER_ENV_INVITE: c_int = 0 + 52;
pub const wxPAPER_ENV_ITALY: c_int = 0 + 53;
pub const wxPAPER_ENV_MONARCH: c_int = 0 + 54;
pub const wxPAPER_ENV_PERSONAL: c_int = 0 + 55;
pub const wxPAPER_ESHEET: c_int = 0 + 56;
pub const wxPAPER_EXECUTIVE: c_int = 0 + 57;
pub const wxPAPER_FANFOLD_LGL_GERMAN: c_int = 0 + 58;
pub const wxPAPER_FANFOLD_STD_GERMAN: c_int = 0 + 59;
pub const wxPAPER_FANFOLD_US: c_int = 0 + 60;
pub const wxPAPER_FOLIO: c_int = 0 + 61;
pub const wxPAPER_ISO_B4: c_int = 0 + 62;
pub const wxPAPER_JAPANESE_POSTCARD: c_int = 0 + 63;
pub const wxPAPER_JAPANESE_POSTCARD_ROTATED: c_int = 0 + 64;
pub const wxPAPER_JENV_CHOU3: c_int = 0 + 65;
pub const wxPAPER_JENV_CHOU3_ROTATED: c_int = 0 + 66;
pub const wxPAPER_JENV_CHOU4: c_int = 0 + 67;
pub const wxPAPER_JENV_CHOU4_ROTATED: c_int = 0 + 68;
pub const wxPAPER_JENV_KAKU2: c_int = 0 + 69;
pub const wxPAPER_JENV_KAKU2_ROTATED: c_int = 0 + 70;
pub const wxPAPER_JENV_KAKU3: c_int = 0 + 71;
pub const wxPAPER_JENV_KAKU3_ROTATED: c_int = 0 + 72;
pub const wxPAPER_JENV_YOU4: c_int = 0 + 73;
pub const wxPAPER_JENV_YOU4_ROTATED: c_int = 0 + 74;
pub const wxPAPER_LEDGER: c_int = 0 + 75;
pub const wxPAPER_LEGAL: c_int = 0 + 76;
pub const wxPAPER_LEGAL_EXTRA: c_int = 0 + 77;
pub const wxPAPER_LETTER: c_int = 0 + 78;
pub const wxPAPER_LETTERSMALL: c_int = 0 + 79;
pub const wxPAPER_LETTER_EXTRA: c_int = 0 + 80;
pub const wxPAPER_LETTER_EXTRA_TRANSVERSE: c_int = 0 + 81;
pub const wxPAPER_LETTER_PLUS: c_int = 0 + 82;
pub const wxPAPER_LETTER_ROTATED: c_int = 0 + 83;
pub const wxPAPER_LETTER_TRANSVERSE: c_int = 0 + 84;
pub const wxPAPER_NONE: c_int = 0 + 85;
pub const wxPAPER_NOTE: c_int = 0 + 86;
pub const wxPAPER_P16K: c_int = 0 + 87;
pub const wxPAPER_P16K_ROTATED: c_int = 0 + 88;
pub const wxPAPER_P32K: c_int = 0 + 89;
pub const wxPAPER_P32KBIG: c_int = 0 + 90;
pub const wxPAPER_P32KBIG_ROTATED: c_int = 0 + 91;
pub const wxPAPER_P32K_ROTATED: c_int = 0 + 92;
pub const wxPAPER_PENV_1: c_int = 0 + 93;
pub const wxPAPER_PENV_10: c_int = 0 + 94;
pub const wxPAPER_PENV_10_ROTATED: c_int = 0 + 95;
pub const wxPAPER_PENV_1_ROTATED: c_int = 0 + 96;
pub const wxPAPER_PENV_2: c_int = 0 + 97;
pub const wxPAPER_PENV_2_ROTATED: c_int = 0 + 98;
pub const wxPAPER_PENV_3: c_int = 0 + 99;
pub const wxPAPER_PENV_3_ROTATED: c_int = 0 + 100;
pub const wxPAPER_PENV_4: c_int = 0 + 101;
pub const wxPAPER_PENV_4_ROTATED: c_int = 0 + 102;
pub const wxPAPER_PENV_5: c_int = 0 + 103;
pub const wxPAPER_PENV_5_ROTATED: c_int = 0 + 104;
pub const wxPAPER_PENV_6: c_int = 0 + 105;
pub const wxPAPER_PENV_6_ROTATED: c_int = 0 + 106;
pub const wxPAPER_PENV_7: c_int = 0 + 107;
pub const wxPAPER_PENV_7_ROTATED: c_int = 0 + 108;
pub const wxPAPER_PENV_8: c_int = 0 + 109;
pub const wxPAPER_PENV_8_ROTATED: c_int = 0 + 110;
pub const wxPAPER_PENV_9: c_int = 0 + 111;
pub const wxPAPER_PENV_9_ROTATED: c_int = 0 + 112;
pub const wxPAPER_QUARTO: c_int = 0 + 113;
pub const wxPAPER_STATEMENT: c_int = 0 + 114;
pub const wxPAPER_TABLOID: c_int = 0 + 115;
pub const wxPAPER_TABLOID_EXTRA: c_int = 0 + 116;
//  ENUM: wxPrintOrientation
pub const wxPORTRAIT: c_int = 0;
pub const wxLANDSCAPE: c_int = 0 + 1;
//  ENUM: wxDuplexMode
pub const wxDUPLEX_SIMPLEX: c_int = 0;
pub const wxDUPLEX_HORIZONTAL: c_int = 0 + 1;
pub const wxDUPLEX_VERTICAL: c_int = 0 + 2;
//  ENUM: wxPrintMode
pub const wxPRINT_MODE_NONE: c_int =    0;
pub const wxPRINT_MODE_PREVIEW: c_int = 1;
pub const wxPRINT_MODE_FILE: c_int =    2;
pub const wxPRINT_MODE_PRINTER: c_int = 3;
pub const wxPRINT_MODE_STREAM: c_int =  4;
//  ENUM: wxUpdateUI
pub const wxUPDATE_UI_NONE: c_int = 0;
pub const wxUPDATE_UI_RECURSE: c_int = 0 + 1;
pub const wxUPDATE_UI_FROMIDLE: c_int = 0 + 2;

//  SKIP: wxPG_LABEL
pub const wxPG_LABEL_STRING: &str = "@!";
//  SKIP: wxPG_NULL_BITMAP
//  SKIP: wxPG_COLOUR_BLACK
//  SKIP: wxPG_COLOUR
//  SKIP: wxPG_DEFAULT_IMAGE_SIZE
//  SKIP: wxPG_INVALID_VALUE
pub const wxPG_DONT_RECURSE: c_int = 0x00000000;
//  SKIP: wxPG_BASE_OCT
//  SKIP: wxPG_BASE_DEC
//  SKIP: wxPG_BASE_HEX
//  SKIP: wxPG_BASE_HEXL
//  SKIP: wxPG_PREFIX_NONE
//  SKIP: wxPG_PREFIX_0x
//  SKIP: wxPG_PREFIX_DOLLAR_SIGN
//  ENUM: wxPG_GETPROPERTYVALUES_FLAGS
pub const wxPG_KEEP_STRUCTURE: c_int = 0x00000010;
pub const wxPG_RECURSE: c_int = 0x00000020;
pub const wxPG_INC_ATTRIBUTES: c_int = 0x00000040;
pub const wxPG_RECURSE_STARTS: c_int = 0x00000080;
pub const wxPG_FORCE: c_int = 0x00000100;
pub const wxPG_SORT_TOP_LEVEL_ONLY: c_int = 0x00000200;
//  ENUM: wxPG_MISC_ARG_FLAGS
pub const wxPG_FULL_VALUE: c_int = 0x00000001;
pub const wxPG_REPORT_ERROR: c_int = 0x00000002;
pub const wxPG_PROPERTY_SPECIFIC: c_int = 0x00000004;
pub const wxPG_EDITABLE_VALUE: c_int = 0x00000008;
pub const wxPG_COMPOSITE_FRAGMENT: c_int = 0x00000010;
pub const wxPG_UNEDITABLE_COMPOSITE_FRAGMENT: c_int = 0x00000020;
pub const wxPG_VALUE_IS_CURRENT: c_int = 0x00000040;
pub const wxPG_PROGRAMMATIC_VALUE: c_int = 0x00000080;
//  ENUM: wxPG_SETVALUE_FLAGS
pub const wxPG_SETVAL_REFRESH_EDITOR: c_int = 0x0001;
pub const wxPG_SETVAL_AGGREGATED: c_int = 0x0002;
pub const wxPG_SETVAL_FROM_PARENT: c_int = 0x0004;
pub const wxPG_SETVAL_BY_USER: c_int = 0x0008;

//  ENUM: wxRichTextStyleType
pub const wxRICHTEXT_STYLE_ALL: c_int = 0;
pub const wxRICHTEXT_STYLE_PARAGRAPH: c_int = 0 + 1;
pub const wxRICHTEXT_STYLE_CHARACTER: c_int = 0 + 2;
pub const wxRICHTEXT_STYLE_LIST: c_int = 0 + 3;
pub const wxRICHTEXT_STYLE_BOX: c_int = 0 + 4;

//  ENUM: @4
pub const wxCAL_SUNDAY_FIRST: c_int = 0x0000;
pub const wxCAL_MONDAY_FIRST: c_int = 0x0001;
pub const wxCAL_SHOW_HOLIDAYS: c_int = 0x0002;
pub const wxCAL_NO_YEAR_CHANGE: c_int = 0x0004;
pub const wxCAL_NO_MONTH_CHANGE: c_int = 0x000c;
pub const wxCAL_SEQUENTIAL_MONTH_SELECTION: c_int = 0x0010;
pub const wxCAL_SHOW_SURROUNDING_WEEKS: c_int = 0x0020;
pub const wxCAL_SHOW_WEEK_NUMBERS: c_int = 0x0040;
//  ENUM: wxCalendarDateBorder
pub const wxCAL_BORDER_NONE: c_int = 0;
pub const wxCAL_BORDER_SQUARE: c_int = 0 + 1;
pub const wxCAL_BORDER_ROUND: c_int = 0 + 2;
//  ENUM: wxCalendarHitTestResult
pub const wxCAL_HITTEST_NOWHERE: c_int = 0;
pub const wxCAL_HITTEST_HEADER: c_int = 0 + 1;
pub const wxCAL_HITTEST_DAY: c_int = 0 + 2;
pub const wxCAL_HITTEST_INCMONTH: c_int = 0 + 3;
pub const wxCAL_HITTEST_DECMONTH: c_int = 0 + 4;
pub const wxCAL_HITTEST_SURROUNDING_WEEK: c_int = 0 + 5;
pub const wxCAL_HITTEST_WEEK: c_int = 0 + 6;

//  ENUM: @39
pub const Timeout_Auto: c_int = -1;
pub const Timeout_Never: c_int = 0;

// NODEF: WX_APPEND_ARRAY
// NODEF: WX_CLEAR_ARRAY
// NODEF: WX_DECLARE_OBJARRAY
// NODEF: WX_DECLARE_EXPORTED_OBJARRAY
// NODEF: WX_DECLARE_USER_EXPORTED_OBJARRAY
// NODEF: WX_DEFINE_ARRAY
// NODEF: WX_DEFINE_EXPORTED_ARRAY
// NODEF: WX_DEFINE_USER_EXPORTED_ARRAY
// NODEF: WX_DEFINE_OBJARRAY
// NODEF: WX_DEFINE_EXPORTED_OBJARRAY
// NODEF: WX_DEFINE_USER_EXPORTED_OBJARRAY
// NODEF: WX_DEFINE_SORTED_ARRAY
// NODEF: WX_DEFINE_SORTED_EXPORTED_ARRAY
// NODEF: WX_DEFINE_SORTED_USER_EXPORTED_ARRAY
// NODEF: WX_PREPEND_ARRAY

//  ENUM: wxSignal
pub const wxSIGNONE: c_int = 0;
pub const wxSIGHUP: c_int = 0 + 1;
pub const wxSIGINT: c_int = 0 + 2;
pub const wxSIGQUIT: c_int = 0 + 3;
pub const wxSIGILL: c_int = 0 + 4;
pub const wxSIGTRAP: c_int = 0 + 5;
pub const wxSIGABRT: c_int = 0 + 6;
pub const wxSIGEMT: c_int = 0 + 7;
pub const wxSIGFPE: c_int = 0 + 8;
pub const wxSIGKILL: c_int = 0 + 9;
pub const wxSIGBUS: c_int = 0 + 10;
pub const wxSIGSEGV: c_int = 0 + 11;
pub const wxSIGSYS: c_int = 0 + 12;
pub const wxSIGPIPE: c_int = 0 + 13;
pub const wxSIGALRM: c_int = 0 + 14;
pub const wxSIGTERM: c_int = 0 + 15;
//  ENUM: wxKillError
pub const wxKILL_OK: c_int = 0;
pub const wxKILL_BAD_SIGNAL: c_int = 0 + 1;
pub const wxKILL_ACCESS_DENIED: c_int = 0 + 2;
pub const wxKILL_NO_PROCESS: c_int = 0 + 3;
pub const wxKILL_ERROR: c_int = 0 + 4;
//  ENUM: wxKillFlags
pub const wxKILL_NOCHILDREN: c_int = 0;
pub const wxKILL_CHILDREN: c_int = 1;
//  ENUM: wxShutdownFlags
pub const wxSHUTDOWN_FORCE: c_int = 1;
pub const wxSHUTDOWN_POWEROFF: c_int = 2;
pub const wxSHUTDOWN_REBOOT: c_int = 4;
pub const wxSHUTDOWN_LOGOFF: c_int = 8;
//  ENUM: @54
pub const wxStrip_Mnemonics: c_int = 1;
pub const wxStrip_Accel: c_int = 2;
pub const wxStrip_All: c_int = wxStrip_Mnemonics | wxStrip_Accel;
//  ENUM: @55
pub const wxEXEC_ASYNC: c_int = 0;
pub const wxEXEC_SYNC: c_int = 1;
pub const wxEXEC_SHOW_CONSOLE: c_int = 2;
pub const wxEXEC_MAKE_GROUP_LEADER: c_int = 4;
pub const wxEXEC_NODISABLE: c_int = 8;
pub const wxEXEC_NOEVENTS: c_int = 16;
pub const wxEXEC_HIDE_CONSOLE: c_int = 32;
pub const wxEXEC_BLOCK: c_int = wxEXEC_SYNC | wxEXEC_NOEVENTS;

pub const wxFLP_OPEN: c_int = 0x0400;
pub const wxFLP_SAVE: c_int = 0x0800;
pub const wxFLP_OVERWRITE_PROMPT: c_int = 0x1000;
pub const wxFLP_FILE_MUST_EXIST: c_int = 0x2000;
pub const wxFLP_CHANGE_DIR: c_int = 0x4000;
pub const wxFLP_SMALL: c_int = wxPB_SMALL;
pub const wxFLP_USE_TEXTCTRL: c_int = (wxPB_USE_TEXTCTRL);
pub const wxFLP_DEFAULT_STYLE: c_int = (wxFLP_OPEN|wxFLP_FILE_MUST_EXIST);
pub const wxDIRP_DIR_MUST_EXIST: c_int = 0x0008;
pub const wxDIRP_CHANGE_DIR: c_int = 0x0010;
pub const wxDIRP_SMALL: c_int = wxPB_SMALL;
pub const wxDIRP_USE_TEXTCTRL: c_int = (wxPB_USE_TEXTCTRL);
pub const wxDIRP_DEFAULT_STYLE: c_int = (wxDIRP_DIR_MUST_EXIST);

//  ENUM: @45
pub const wxRICHTEXT_FIELD_STYLE_COMPOSITE: c_int = 0x01;
pub const wxRICHTEXT_FIELD_STYLE_RECTANGLE: c_int = 0x02;
pub const wxRICHTEXT_FIELD_STYLE_NO_BORDER: c_int = 0x04;
pub const wxRICHTEXT_FIELD_STYLE_START_TAG: c_int = 0x08;
pub const wxRICHTEXT_FIELD_STYLE_END_TAG: c_int = 0x10;

pub const wxST_NO_AUTORESIZE: c_int = 0x0001;
pub const wxST_ELLIPSIZE_START: c_int = 0x0004;
pub const wxST_ELLIPSIZE_MIDDLE: c_int = 0x0008;
pub const wxST_ELLIPSIZE_END: c_int = 0x0010;

//  ENUM: @3
pub const NO_IMAGE: c_int = -1;

//  ENUM: wxRasterOperationMode
pub const wxCLEAR: c_int = 0;
pub const wxXOR: c_int = 0 + 1;
pub const wxINVERT: c_int = 0 + 2;
pub const wxOR_REVERSE: c_int = 0 + 3;
pub const wxAND_REVERSE: c_int = 0 + 4;
pub const wxCOPY: c_int = 0 + 5;
pub const wxAND: c_int = 0 + 6;
pub const wxAND_INVERT: c_int = 0 + 7;
pub const wxNO_OP: c_int = 0 + 8;
pub const wxNOR: c_int = 0 + 9;
pub const wxEQUIV: c_int = 0 + 10;
pub const wxSRC_INVERT: c_int = 0 + 11;
pub const wxOR_INVERT: c_int = 0 + 12;
pub const wxNAND: c_int = 0 + 13;
pub const wxOR: c_int = 0 + 14;
pub const wxSET: c_int = 0 + 15;
//  ENUM: wxFloodFillStyle
pub const wxFLOOD_SURFACE: c_int = 1;
pub const wxFLOOD_BORDER: c_int = 1 + 1;
//  ENUM: wxMappingMode
pub const wxMM_TEXT: c_int = 1;
pub const wxMM_METRIC: c_int = 1 + 1;
pub const wxMM_LOMETRIC: c_int = 1 + 2;
pub const wxMM_TWIPS: c_int = 1 + 3;
pub const wxMM_POINTS: c_int = 1 + 4;

//  ENUM: Context
pub const Context_Current: c_int = 0;
pub const Context_Exception: c_int = 0 + 1;

pub const wxPG_PROP_PASSWORD: c_long = wxPG_PROP_CLASS_SPECIFIC_2;
pub const wxPG_PROP_STATIC_CHOICES: c_long = wxPG_PROP_CLASS_SPECIFIC_1;
pub const wxPG_PROP_SHOW_FULL_FILENAME: c_long = wxPG_PROP_CLASS_SPECIFIC_1;
//  SKIP: wxPG_PROP_NO_ESCAPE
pub const wxPG_PROP_USE_CHECKBOX: c_long = wxPG_PROP_CLASS_SPECIFIC_1;
pub const wxPG_PROP_USE_DCC: c_long = wxPG_PROP_CLASS_SPECIFIC_2;
pub const wxAEDIALOG_STYLE: c_long = (wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER | wxOK | wxCANCEL | wxCENTRE);
//  ENUM: wxPGNumericValidationConstants
pub const wxPG_PROPERTY_VALIDATION_ERROR_MESSAGE: c_int = 0;
pub const wxPG_PROPERTY_VALIDATION_SATURATE: c_int = 1;
pub const wxPG_PROPERTY_VALIDATION_WRAP: c_int = 2;

//  ENUM: wxMessageOutputFlags
pub const wxMSGOUT_PREFER_STDERR: c_int = 0;
pub const wxMSGOUT_PREFER_MSGBOX: c_int = 1;

pub const wxSOUND_SYNC: c_int = 0;
pub const wxSOUND_ASYNC: c_int = 1;
pub const wxSOUND_LOOP: c_int = 2;

pub const wxHF_TOOLBAR: c_int = 0x0001;
pub const wxHF_CONTENTS: c_int = 0x0002;
pub const wxHF_INDEX: c_int = 0x0004;
pub const wxHF_SEARCH: c_int = 0x0008;
pub const wxHF_BOOKMARKS: c_int = 0x0010;
pub const wxHF_OPEN_FILES: c_int = 0x0020;
pub const wxHF_PRINT: c_int = 0x0040;
pub const wxHF_FLAT_TOOLBAR: c_int = 0x0080;
pub const wxHF_MERGE_BOOKS: c_int = 0x0100;
pub const wxHF_ICONS_BOOK: c_int = 0x0200;
pub const wxHF_ICONS_BOOK_CHAPTER: c_int = 0x0400;
pub const wxHF_ICONS_FOLDER: c_int = 0x0000;
pub const wxHF_DEFAULT_STYLE: c_int = (wxHF_TOOLBAR | wxHF_CONTENTS | wxHF_INDEX | wxHF_SEARCH | wxHF_BOOKMARKS | wxHF_PRINT);

//  ENUM: wxWebViewZoom
pub const wxWEBVIEW_ZOOM_TINY: c_int = 0;
pub const wxWEBVIEW_ZOOM_SMALL: c_int = 0 + 1;
pub const wxWEBVIEW_ZOOM_MEDIUM: c_int = 0 + 2;
pub const wxWEBVIEW_ZOOM_LARGE: c_int = 0 + 3;
pub const wxWEBVIEW_ZOOM_LARGEST: c_int = 0 + 4;
//  ENUM: wxWebViewZoomType
pub const wxWEBVIEW_ZOOM_TYPE_LAYOUT: c_int = 0;
pub const wxWEBVIEW_ZOOM_TYPE_TEXT: c_int = 0 + 1;
//  ENUM: wxWebViewNavigationError
pub const wxWEBVIEW_NAV_ERR_CONNECTION: c_int = 0;
pub const wxWEBVIEW_NAV_ERR_CERTIFICATE: c_int = 0 + 1;
pub const wxWEBVIEW_NAV_ERR_AUTH: c_int = 0 + 2;
pub const wxWEBVIEW_NAV_ERR_SECURITY: c_int = 0 + 3;
pub const wxWEBVIEW_NAV_ERR_NOT_FOUND: c_int = 0 + 4;
pub const wxWEBVIEW_NAV_ERR_REQUEST: c_int = 0 + 5;
pub const wxWEBVIEW_NAV_ERR_USER_CANCELLED: c_int = 0 + 6;
pub const wxWEBVIEW_NAV_ERR_OTHER: c_int = 0 + 7;
//  ENUM: wxWebViewReloadFlags
pub const wxWEBVIEW_RELOAD_DEFAULT: c_int = 0;
pub const wxWEBVIEW_RELOAD_NO_CACHE: c_int = 0 + 1;
//  ENUM: wxWebViewFindFlags
pub const wxWEBVIEW_FIND_WRAP: c_int =             0x0001;
pub const wxWEBVIEW_FIND_ENTIRE_WORD: c_int =      0x0002;
pub const wxWEBVIEW_FIND_MATCH_CASE: c_int =       0x0004;
pub const wxWEBVIEW_FIND_HIGHLIGHT_RESULT: c_int = 0x0008;
pub const wxWEBVIEW_FIND_BACKWARDS: c_int =        0x0010;
pub const wxWEBVIEW_FIND_DEFAULT: c_int =          0;

pub const wxIMAGE_OPTION_PNG_FORMAT: &str = "PngFormat";
pub const wxIMAGE_OPTION_PNG_BITDEPTH: &str = "PngBitDepth";
pub const wxIMAGE_OPTION_PNG_FILTER: &str = "PngF";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_LEVEL: &str = "PngZL";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_MEM_LEVEL: &str = "PngZM";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_STRATEGY: &str = "PngZS";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_BUFFER_SIZE: &str = "PngZB";

//  ENUM: wxZlibCompressionLevels
pub const wxZ_DEFAULT_COMPRESSION: c_int = -1;
pub const wxZ_NO_COMPRESSION: c_int = 0;
pub const wxZ_BEST_SPEED: c_int = 1;
pub const wxZ_BEST_COMPRESSION: c_int = 9;
//  ENUM: wxZLibFlags
pub const wxZLIB_NO_HEADER: c_int = 0;
pub const wxZLIB_ZLIB: c_int = 1;
pub const wxZLIB_GZIP: c_int = 2;
pub const wxZLIB_AUTO: c_int = 3;

//  ENUM: Direction
pub const Get: c_int = 0x01;
pub const Set: c_int = 0x02;
pub const Both: c_int = 0x03;

//  ENUM: wxBrushStyle
pub const wxBRUSHSTYLE_INVALID: c_int = -1;
pub const wxBRUSHSTYLE_SOLID: c_int = wxSOLID;
pub const wxBRUSHSTYLE_TRANSPARENT: c_int = wxTRANSPARENT;
pub const wxBRUSHSTYLE_STIPPLE_MASK_OPAQUE: c_int = wxSTIPPLE_MASK_OPAQUE;
pub const wxBRUSHSTYLE_STIPPLE_MASK: c_int = wxSTIPPLE_MASK;
pub const wxBRUSHSTYLE_STIPPLE: c_int = wxSTIPPLE;
pub const wxBRUSHSTYLE_BDIAGONAL_HATCH: c_int = wxSTIPPLE + 1;
pub const wxBRUSHSTYLE_CROSSDIAG_HATCH: c_int = wxSTIPPLE + 2;
pub const wxBRUSHSTYLE_FDIAGONAL_HATCH: c_int = wxSTIPPLE + 3;
pub const wxBRUSHSTYLE_CROSS_HATCH: c_int = wxSTIPPLE + 4;
pub const wxBRUSHSTYLE_HORIZONTAL_HATCH: c_int = wxSTIPPLE + 5;
pub const wxBRUSHSTYLE_VERTICAL_HATCH: c_int = wxSTIPPLE + 6;
pub const wxBRUSHSTYLE_FIRST_HATCH: c_int = wxSTIPPLE + 7;
pub const wxBRUSHSTYLE_LAST_HATCH: c_int = wxSTIPPLE + 8;

//  ENUM: wxMessageQueueError
pub const wxMSGQUEUE_NO_ERROR: c_int = 0;
pub const wxMSGQUEUE_TIMEOUT: c_int = 0 + 1;
pub const wxMSGQUEUE_MISC_ERROR: c_int = 0 + 2;

//  ENUM: wxHtmlSelectionState
pub const wxHTML_SEL_OUT: c_int = 0;
pub const wxHTML_SEL_IN: c_int = 0 + 1;
pub const wxHTML_SEL_CHANGING: c_int = 0 + 2;
//  ENUM: @26
pub const wxHTML_FIND_EXACT: c_int = 1;
pub const wxHTML_FIND_NEAREST_BEFORE: c_int = 2;
pub const wxHTML_FIND_NEAREST_AFTER: c_int = 4;
//  ENUM: wxHtmlScriptMode
pub const wxHTML_SCRIPT_NORMAL: c_int = 0;
pub const wxHTML_SCRIPT_SUB: c_int = 0 + 1;
pub const wxHTML_SCRIPT_SUP: c_int = 0 + 2;

//  ENUM: @10
pub const wxDP_DEFAULT: c_int = 0;
pub const wxDP_SPIN: c_int = 1;
pub const wxDP_DROPDOWN: c_int = 2;
pub const wxDP_SHOWCENTURY: c_int = 4;
pub const wxDP_ALLOWNONE: c_int = 8;

//  ENUM: @22
pub const wxCOL_WIDTH_DEFAULT: c_int = -1;
pub const wxCOL_WIDTH_AUTOSIZE: c_int = -2;
//  ENUM: @23
pub const wxCOL_RESIZABLE: c_int = 1;
pub const wxCOL_SORTABLE: c_int = 2;
pub const wxCOL_REORDERABLE: c_int = 4;
pub const wxCOL_HIDDEN: c_int = 8;
pub const wxCOL_DEFAULT_FLAGS: c_int = wxCOL_RESIZABLE | wxCOL_REORDERABLE;

// NODEF: wxCHECK_GCC_VERSION
// NODEF: wxCHECK_SUNCC_VERSION
// NODEF: wxCHECK_VISUALC_VERSION
// NODEF: wxCHECK_W32API_VERSION

//  ENUM: wxRibbonButtonBarButtonState
pub const wxRIBBON_BUTTONBAR_BUTTON_SMALL: c_int = 0 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_MEDIUM: c_int = 1 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_LARGE: c_int = 2 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_SIZE_MASK: c_int = 3 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED: c_int = 1 << 3;
pub const wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED: c_int = 1 << 4;
pub const wxRIBBON_BUTTONBAR_BUTTON_HOVER_MASK: c_int = wxRIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED | wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED;
pub const wxRIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE: c_int = 1 << 5;
pub const wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE: c_int = 1 << 6;
pub const wxRIBBON_BUTTONBAR_BUTTON_ACTIVE_MASK: c_int = wxRIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE | wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE;
pub const wxRIBBON_BUTTONBAR_BUTTON_DISABLED: c_int = 1 << 7;
pub const wxRIBBON_BUTTONBAR_BUTTON_TOGGLED: c_int = 1 << 8;
pub const wxRIBBON_BUTTONBAR_BUTTON_STATE_MASK: c_int = 0x1F8;

//  ENUM: wxMessageQueueError

//  ENUM: wxOperatingSystemId
pub const wxOS_UNKNOWN: c_int = 0;
pub const wxOS_MAC_OS: c_int = 1 << 0;
pub const wxOS_MAC_OSX_DARWIN: c_int = 1 << 1;
pub const wxOS_MAC: c_int = wxOS_MAC_OS|wxOS_MAC_OSX_DARWIN;
pub const wxOS_WINDOWS_9X: c_int = 1 << 2;
pub const wxOS_WINDOWS_NT: c_int = 1 << 3;
pub const wxOS_WINDOWS_MICRO: c_int = 1 << 4;
pub const wxOS_WINDOWS_CE: c_int = 1 << 5;
pub const wxOS_WINDOWS: c_int = wxOS_WINDOWS_9X       |
                    wxOS_WINDOWS_NT      |
                    wxOS_WINDOWS_MICRO   |
                    wxOS_WINDOWS_CE;
pub const wxOS_UNIX_LINUX: c_int = 1 << 6;
pub const wxOS_UNIX_FREEBSD: c_int = 1 << 7;
pub const wxOS_UNIX_OPENBSD: c_int = 1 << 8;
pub const wxOS_UNIX_NETBSD: c_int = 1 << 9;
pub const wxOS_UNIX_SOLARIS: c_int = 1 << 10;
pub const wxOS_UNIX_AIX: c_int = 1 << 11;
pub const wxOS_UNIX_HPUX: c_int = 1 << 12;
pub const wxOS_UNIX: c_int = wxOS_UNIX_LINUX     |
                wxOS_UNIX_FREEBSD   |
                wxOS_UNIX_OPENBSD   |
                wxOS_UNIX_NETBSD    |
                wxOS_UNIX_SOLARIS   |
                wxOS_UNIX_AIX       |
                wxOS_UNIX_HPUX;
pub const wxOS_DOS: c_int = 1 << 15;
pub const wxOS_OS2: c_int = 1 << 16;
//  ENUM: wxPortId
pub const wxPORT_UNKNOWN: c_int = 0;
pub const wxPORT_BASE: c_int = 1 << 0;
pub const wxPORT_MSW: c_int = 1 << 1;
pub const wxPORT_MOTIF: c_int = 1 << 2;
pub const wxPORT_GTK: c_int = 1 << 3;
pub const wxPORT_DFB: c_int = 1 << 4;
pub const wxPORT_X11: c_int = 1 << 5;
pub const wxPORT_OS2: c_int = 1 << 6;
pub const wxPORT_MAC: c_int = 1 << 7;
pub const wxPORT_COCOA: c_int = 1 << 8;
pub const wxPORT_WINCE: c_int = 1 << 9;
//  ENUM: wxArchitecture
pub const wxARCH_INVALID: c_int = -1;
pub const wxARCH_32: c_int = -1 + 1;
pub const wxARCH_64: c_int = -1 + 2;
pub const wxARCH_MAX: c_int = -1 + 3;
//  ENUM: wxEndianness
pub const wxENDIAN_INVALID: c_int = -1;
pub const wxENDIAN_BIG: c_int = -1 + 1;
pub const wxENDIAN_LITTLE: c_int = -1 + 2;
pub const wxENDIAN_PDP: c_int = -1 + 3;
pub const wxENDIAN_MAX: c_int = -1 + 4;

pub const wxDEFAULT_DELIMITERS: &str = " \t\r\n";
//  ENUM: wxStringTokenizerMode
pub const wxTOKEN_INVALID: c_int = -1;
pub const wxTOKEN_DEFAULT: c_int = -1 + 1;
pub const wxTOKEN_RET_EMPTY: c_int = -1 + 2;
pub const wxTOKEN_RET_EMPTY_ALL: c_int = -1 + 3;
pub const wxTOKEN_RET_DELIMS: c_int = -1 + 4;
pub const wxTOKEN_STRTOK: c_int = -1 + 5;

//  ENUM: wxShowEffect
pub const wxSHOW_EFFECT_NONE: c_int = 0;
pub const wxSHOW_EFFECT_ROLL_TO_LEFT: c_int = 0 + 1;
pub const wxSHOW_EFFECT_ROLL_TO_RIGHT: c_int = 0 + 2;
pub const wxSHOW_EFFECT_ROLL_TO_TOP: c_int = 0 + 3;
pub const wxSHOW_EFFECT_ROLL_TO_BOTTOM: c_int = 0 + 4;
pub const wxSHOW_EFFECT_SLIDE_TO_LEFT: c_int = 0 + 5;
pub const wxSHOW_EFFECT_SLIDE_TO_RIGHT: c_int = 0 + 6;
pub const wxSHOW_EFFECT_SLIDE_TO_TOP: c_int = 0 + 7;
pub const wxSHOW_EFFECT_SLIDE_TO_BOTTOM: c_int = 0 + 8;
pub const wxSHOW_EFFECT_BLEND: c_int = 0 + 9;
pub const wxSHOW_EFFECT_EXPAND: c_int = 0 + 10;
pub const wxSHOW_EFFECT_MAX: c_int = 0 + 11;
//  ENUM: @56
pub const wxSEND_EVENT_POST: c_int = 1;
//  ENUM: wxWindowVariant
pub const wxWINDOW_VARIANT_NORMAL: c_int = 0;
pub const wxWINDOW_VARIANT_SMALL: c_int = 0 + 1;
pub const wxWINDOW_VARIANT_MINI: c_int = 0 + 2;
pub const wxWINDOW_VARIANT_LARGE: c_int = 0 + 3;
pub const wxWINDOW_VARIANT_MAX: c_int = 0 + 4;

pub const wxAC_NO_AUTORESIZE: c_int = (0x0010);
pub const wxAC_DEFAULT_STYLE: c_long = (wxBORDER_NONE);
//  ENUM: wxAnimationType
pub const wxANIMATION_TYPE_INVALID: c_int = 0;
pub const wxANIMATION_TYPE_GIF: c_int = 0 + 1;
pub const wxANIMATION_TYPE_ANI: c_int = 0 + 2;
pub const wxANIMATION_TYPE_ANY: c_int = 0 + 3;

pub const wxFRAME_NO_TASKBAR: c_int = 0x0002;
pub const wxFRAME_TOOL_WINDOW: c_int = 0x0004;
pub const wxFRAME_FLOAT_ON_PARENT: c_int = 0x0008;

//  ENUM: OpenMode
pub const read: c_int = 0;
pub const write: c_int = 0 + 1;
pub const read_write: c_int = 0 + 2;
pub const write_append: c_int = 0 + 3;
pub const write_excl: c_int = 0 + 4;
//  ENUM: @16
pub const fd_invalid: c_int = -1;
pub const fd_stdin: c_int = -1 + 1;
pub const fd_stdout: c_int = -1 + 2;
pub const fd_stderr: c_int = -1 + 3;

pub const wxNB_DEFAULT: c_int = wxBK_DEFAULT;
pub const wxNB_TOP: c_int = wxBK_TOP;
pub const wxNB_BOTTOM: c_int = wxBK_BOTTOM;
pub const wxNB_LEFT: c_int = wxBK_LEFT;
pub const wxNB_RIGHT: c_int = wxBK_RIGHT;
pub const wxNB_FIXEDWIDTH: c_int = 0x0100;
pub const wxNB_MULTILINE: c_int = 0x0200;
pub const wxNB_NOPAGETHEME: c_int = 0x0400;
pub const wxNB_FLAT: c_int = 0x0800;
//  ENUM: @38
pub const wxNB_HITTEST_NOWHERE: c_int = wxBK_HITTEST_NOWHERE;
pub const wxNB_HITTEST_ONICON: c_int = wxBK_HITTEST_ONICON;
pub const wxNB_HITTEST_ONLABEL: c_int = wxBK_HITTEST_ONLABEL;
pub const wxNB_HITTEST_ONITEM: c_int = wxBK_HITTEST_ONITEM;
pub const wxNB_HITTEST_ONPAGE: c_int = wxBK_HITTEST_ONPAGE;

//  ENUM: NumericType
pub const Signed: c_int = 0;
pub const Unsigned: c_int = 0 + 1;
pub const Float: c_int = 0 + 2;

//  ENUM: wxAuiToolBarStyle
pub const wxAUI_TB_TEXT: c_int = 1 << 0;
pub const wxAUI_TB_NO_TOOLTIPS: c_int = 1 << 1;
pub const wxAUI_TB_NO_AUTORESIZE: c_int = 1 << 2;
pub const wxAUI_TB_GRIPPER: c_int = 1 << 3;
pub const wxAUI_TB_OVERFLOW: c_int = 1 << 4;
pub const wxAUI_TB_VERTICAL: c_int = 1 << 5;
pub const wxAUI_TB_HORZ_LAYOUT: c_int = 1 << 6;
pub const wxAUI_TB_HORIZONTAL: c_int = 1 << 7;
pub const wxAUI_TB_PLAIN_BACKGROUND: c_int = 1 << 8;
pub const wxAUI_TB_HORZ_TEXT: c_int = (wxAUI_TB_HORZ_LAYOUT | wxAUI_TB_TEXT);
pub const wxAUI_ORIENTATION_MASK: c_int = (wxAUI_TB_VERTICAL | wxAUI_TB_HORIZONTAL);
pub const wxAUI_TB_DEFAULT_STYLE: c_int = 0;
//  ENUM: wxAuiToolBarArtSetting
pub const wxAUI_TBART_SEPARATOR_SIZE: c_int = 0;
pub const wxAUI_TBART_GRIPPER_SIZE: c_int = 1;
//  SKIP: wxAUI_TBART_OVERFLOW_SIZE
//  ENUM: wxAuiToolBarToolTextOrientation
pub const wxAUI_TBTOOL_TEXT_LEFT: c_int = 0;
pub const wxAUI_TBTOOL_TEXT_RIGHT: c_int = 1;
pub const wxAUI_TBTOOL_TEXT_TOP: c_int = 2;
pub const wxAUI_TBTOOL_TEXT_BOTTOM: c_int = 3;
//  ENUM: wxAuiPaneDockArtSetting
pub const wxAUI_DOCKART_SASH_SIZE: c_int = 0;
pub const wxAUI_DOCKART_CAPTION_SIZE: c_int = 1;
pub const wxAUI_DOCKART_GRIPPER_SIZE: c_int = 2;
pub const wxAUI_DOCKART_PANE_BORDER_SIZE: c_int = 3;
pub const wxAUI_DOCKART_PANE_BUTTON_SIZE: c_int = 4;
pub const wxAUI_DOCKART_BACKGROUND_COLOUR: c_int = 5;
pub const wxAUI_DOCKART_SASH_COLOUR: c_int = 6;
pub const wxAUI_DOCKART_ACTIVE_CAPTION_COLOUR: c_int = 7;
pub const wxAUI_DOCKART_ACTIVE_CAPTION_GRADIENT_COLOUR: c_int = 8;
pub const wxAUI_DOCKART_INACTIVE_CAPTION_COLOUR: c_int = 9;
pub const wxAUI_DOCKART_INACTIVE_CAPTION_GRADIENT_COLOUR: c_int = 10;
pub const wxAUI_DOCKART_ACTIVE_CAPTION_TEXT_COLOUR: c_int = 11;
pub const wxAUI_DOCKART_INACTIVE_CAPTION_TEXT_COLOUR: c_int = 12;
pub const wxAUI_DOCKART_BORDER_COLOUR: c_int = 13;
pub const wxAUI_DOCKART_GRIPPER_COLOUR: c_int = 14;
pub const wxAUI_DOCKART_CAPTION_FONT: c_int = 15;
pub const wxAUI_DOCKART_GRADIENT_TYPE: c_int = 16;

pub const wxIMAGE_OPTION_TIFF_BITSPERSAMPLE: &str = "BitsPerSample";
pub const wxIMAGE_OPTION_TIFF_SAMPLESPERPIXEL: &str = "SamplesPerPixel";
pub const wxIMAGE_OPTION_TIFF_COMPRESSION: &str = "Compression";
pub const wxIMAGE_OPTION_TIFF_PHOTOMETRIC: &str = "Photometric";
pub const wxIMAGE_OPTION_TIFF_IMAGEDESCRIPTOR: &str = "ImageDescriptor";

//  SKIP: wxInvalidDateTime

//  ENUM: wxRibbonBarOption
pub const wxRIBBON_BAR_SHOW_PAGE_LABELS: c_int = 0;
pub const wxRIBBON_BAR_SHOW_PAGE_ICONS: c_int = 0 + 1;
pub const wxRIBBON_BAR_FLOW_HORIZONTAL: c_int = 0 + 2;
pub const wxRIBBON_BAR_FLOW_VERTICAL: c_int = 0 + 3;
pub const wxRIBBON_BAR_SHOW_PANEL_EXT_BUTTONS: c_int = 0 + 4;
pub const wxRIBBON_BAR_SHOW_PANEL_MINIMISE_BUTTONS: c_int = 0 + 5;
pub const wxRIBBON_BAR_ALWAYS_SHOW_TABS: c_int = 0 + 6;
pub const wxRIBBON_BAR_SHOW_TOGGLE_BUTTON: c_int = 0 + 7;
pub const wxRIBBON_BAR_SHOW_HELP_BUTTON: c_int = 0 + 8;
pub const wxRIBBON_BAR_DEFAULT_STYLE: c_int = 0 + 9;
pub const wxRIBBON_BAR_FOLDBAR_STYLE: c_int = 0 + 10;
//  ENUM: wxRibbonDisplayMode
pub const wxRIBBON_BAR_PINNED: c_int = 0;
pub const wxRIBBON_BAR_MINIMIZED: c_int = 0 + 1;
pub const wxRIBBON_BAR_EXPANDED: c_int = 0 + 2;

//  ENUM: wxMouseButton
pub const wxMOUSE_BTN_ANY: c_int = -1;
pub const wxMOUSE_BTN_NONE: c_int = 0;
pub const wxMOUSE_BTN_LEFT: c_int = 1;
pub const wxMOUSE_BTN_MIDDLE: c_int = 2;
pub const wxMOUSE_BTN_RIGHT: c_int = 3;
pub const wxMOUSE_BTN_AUX1: c_int = 4;
pub const wxMOUSE_BTN_AUX2: c_int = 5;
pub const wxMOUSE_BTN_MAX: c_int = 5 + 1;

// NODEF: wxDYNLIB_FUNCTION
//  ENUM: wxDynamicLibraryCategory
pub const wxDL_LIBRARY: c_int = 0;
pub const wxDL_MODULE: c_int = 0 + 1;
//  ENUM: wxPluginCategory
pub const wxDL_PLUGIN_GUI: c_int = 0;
pub const wxDL_PLUGIN_BASE: c_int = 0 + 1;

pub const wxFRAME_SHAPED: c_int = 0x0010;

//  ENUM: wxIPCFormat
pub const wxIPC_INVALID: c_int =     0;
pub const wxIPC_TEXT: c_int =        1;
pub const wxIPC_BITMAP: c_int =      2;
pub const wxIPC_METAFILE: c_int =    3;
pub const wxIPC_SYLK: c_int =        4;
pub const wxIPC_DIF: c_int =         5;
pub const wxIPC_TIFF: c_int =        6;
pub const wxIPC_OEMTEXT: c_int =     7;
pub const wxIPC_DIB: c_int =         8;
pub const wxIPC_PALETTE: c_int =     9;
pub const wxIPC_PENDATA: c_int =     10;
pub const wxIPC_RIFF: c_int =        11;
pub const wxIPC_WAVE: c_int =        12;
pub const wxIPC_UTF16TEXT: c_int =   13;
pub const wxIPC_ENHMETAFILE: c_int = 14;
pub const wxIPC_FILENAME: c_int =    15;
pub const wxIPC_LOCALE: c_int =      16;
pub const wxIPC_UTF8TEXT: c_int =    17;
pub const wxIPC_UTF32TEXT: c_int =   18;
pub const wxIPC_UNICODETEXT: c_int = wxIPC_UTF16TEXT;
pub const wxIPC_PRIVATE: c_int =     20;

//  ENUM: wxPathFormat
pub const wxPATH_NATIVE: c_int = 0;
pub const wxPATH_UNIX: c_int = 0 + 1;
pub const wxPATH_BEOS: c_int = wxPATH_UNIX;
pub const wxPATH_MAC: c_int = wxPATH_UNIX + 1;
pub const wxPATH_DOS: c_int = wxPATH_UNIX + 2;
pub const wxPATH_WIN: c_int = wxPATH_DOS;
pub const wxPATH_OS2: c_int = wxPATH_DOS;
pub const wxPATH_VMS: c_int = wxPATH_DOS + 1;
pub const wxPATH_MAX: c_int = wxPATH_DOS + 2;
//  ENUM: wxSizeConvention
pub const wxSIZE_CONV_TRADITIONAL: c_int = 0;
pub const wxSIZE_CONV_IEC: c_int = 0 + 1;
pub const wxSIZE_CONV_SI: c_int = 0 + 2;
//  ENUM: wxPathNormalize
pub const wxPATH_NORM_ENV_VARS: c_int = 0x0001;
pub const wxPATH_NORM_DOTS: c_int = 0x0002;
pub const wxPATH_NORM_TILDE: c_int = 0x0004;
pub const wxPATH_NORM_CASE: c_int = 0x0008;
pub const wxPATH_NORM_ABSOLUTE: c_int = 0x0010;
pub const wxPATH_NORM_LONG: c_int =     0x0020;
pub const wxPATH_NORM_SHORTCUT: c_int = 0x0040;
pub const wxPATH_NORM_ALL: c_int = 0x00ff & !wxPATH_NORM_CASE;
//  ENUM: @19
pub const wxPATH_RMDIR_FULL: c_int = 1;
pub const wxPATH_RMDIR_RECURSIVE: c_int = 2;
//  ENUM: @20
pub const wxFILE_EXISTS_REGULAR: c_int = 0x0001;
pub const wxFILE_EXISTS_DIR: c_int = 0x0002;
pub const wxFILE_EXISTS_SYMLINK: c_int = 0x1004;
pub const wxFILE_EXISTS_DEVICE: c_int = 0x0008;
pub const wxFILE_EXISTS_FIFO: c_int = 0x0016;
pub const wxFILE_EXISTS_SOCKET: c_int = 0x0032;
//  SKIP: wxFILE_EXISTS_NO_FOLLOW

pub const wxFNTP_FONTDESC_AS_LABEL: c_int = 0x0008;
pub const wxFNTP_USEFONT_FOR_LABEL: c_int = 0x0010;
pub const wxFONTBTN_DEFAULT_STYLE: c_int = (wxFNTP_FONTDESC_AS_LABEL | wxFNTP_USEFONT_FOR_LABEL);
pub const wxFNTP_USE_TEXTCTRL: c_int = (wxPB_USE_TEXTCTRL);
pub const wxFNTP_DEFAULT_STYLE: c_int = (wxFNTP_FONTDESC_AS_LABEL|wxFNTP_USEFONT_FOR_LABEL);

//  ENUM: wxAntialiasMode
pub const wxANTIALIAS_NONE: c_int = 0;
pub const wxANTIALIAS_DEFAULT: c_int = 0 + 1;
//  ENUM: wxInterpolationQuality
pub const wxINTERPOLATION_DEFAULT: c_int = 0;
pub const wxINTERPOLATION_NONE: c_int = 0 + 1;
pub const wxINTERPOLATION_FAST: c_int = 0 + 2;
pub const wxINTERPOLATION_GOOD: c_int = 0 + 3;
pub const wxINTERPOLATION_BEST: c_int = 0 + 4;
//  ENUM: wxCompositionMode
pub const wxCOMPOSITION_INVALID: c_int = -1;
pub const wxCOMPOSITION_CLEAR: c_int = -1 + 1;
pub const wxCOMPOSITION_SOURCE: c_int = -1 + 2;
pub const wxCOMPOSITION_OVER: c_int = -1 + 3;
pub const wxCOMPOSITION_IN: c_int = -1 + 4;
pub const wxCOMPOSITION_OUT: c_int = -1 + 5;
pub const wxCOMPOSITION_ATOP: c_int = -1 + 6;
pub const wxCOMPOSITION_DEST: c_int = -1 + 7;
pub const wxCOMPOSITION_DEST_OVER: c_int = -1 + 8;
pub const wxCOMPOSITION_DEST_IN: c_int = -1 + 9;
pub const wxCOMPOSITION_DEST_OUT: c_int = -1 + 10;
pub const wxCOMPOSITION_DEST_ATOP: c_int = -1 + 11;
pub const wxCOMPOSITION_XOR: c_int = -1 + 12;
pub const wxCOMPOSITION_ADD: c_int = -1 + 13;

pub const wxCHOICE_WIDTH: c_int = 150;
pub const wxCHOICE_HEIGHT: c_int = 200;
pub const wxCHOICEDLG_STYLE: c_long = (wxDEFAULT_DIALOG_STYLE | wxOK | wxCANCEL | wxCENTRE | wxRESIZE_BORDER);

pub const wxSTC_INVALID_POSITION: c_int = -1;
pub const wxSTC_START: c_int = 2000;
pub const wxSTC_OPTIONAL_START: c_int = 3000;
pub const wxSTC_LEXER_START: c_int = 4000;
pub const wxSTC_WS_INVISIBLE: c_int = 0;
pub const wxSTC_WS_VISIBLEALWAYS: c_int = 1;
pub const wxSTC_WS_VISIBLEAFTERINDENT: c_int = 2;
pub const wxSTC_EOL_CRLF: c_int = 0;
pub const wxSTC_EOL_CR: c_int = 1;
pub const wxSTC_EOL_LF: c_int = 2;
pub const wxSTC_CP_UTF8: c_int = 65001;
pub const wxSTC_MARKER_MAX: c_int = 31;
pub const wxSTC_MARK_CIRCLE: c_int = 0;
pub const wxSTC_MARK_ROUNDRECT: c_int = 1;
pub const wxSTC_MARK_ARROW: c_int = 2;
pub const wxSTC_MARK_SMALLRECT: c_int = 3;
pub const wxSTC_MARK_SHORTARROW: c_int = 4;
pub const wxSTC_MARK_EMPTY: c_int = 5;
pub const wxSTC_MARK_ARROWDOWN: c_int = 6;
pub const wxSTC_MARK_MINUS: c_int = 7;
pub const wxSTC_MARK_PLUS: c_int = 8;
pub const wxSTC_MARK_VLINE: c_int = 9;
pub const wxSTC_MARK_LCORNER: c_int = 10;
pub const wxSTC_MARK_TCORNER: c_int = 11;
pub const wxSTC_MARK_BOXPLUS: c_int = 12;
pub const wxSTC_MARK_BOXPLUSCONNECTED: c_int = 13;
pub const wxSTC_MARK_BOXMINUS: c_int = 14;
pub const wxSTC_MARK_BOXMINUSCONNECTED: c_int = 15;
pub const wxSTC_MARK_LCORNERCURVE: c_int = 16;
pub const wxSTC_MARK_TCORNERCURVE: c_int = 17;
pub const wxSTC_MARK_CIRCLEPLUS: c_int = 18;
pub const wxSTC_MARK_CIRCLEPLUSCONNECTED: c_int = 19;
pub const wxSTC_MARK_CIRCLEMINUS: c_int = 20;
pub const wxSTC_MARK_CIRCLEMINUSCONNECTED: c_int = 21;
pub const wxSTC_MARK_BACKGROUND: c_int = 22;
pub const wxSTC_MARK_DOTDOTDOT: c_int = 23;
pub const wxSTC_MARK_ARROWS: c_int = 24;
pub const wxSTC_MARK_PIXMAP: c_int = 25;
pub const wxSTC_MARK_FULLRECT: c_int = 26;
pub const wxSTC_MARK_LEFTRECT: c_int = 27;
pub const wxSTC_MARK_AVAILABLE: c_int = 28;
pub const wxSTC_MARK_UNDERLINE: c_int = 29;
pub const wxSTC_MARK_RGBAIMAGE: c_int = 30;
pub const wxSTC_MARK_CHARACTER: c_int = 10000;
pub const wxSTC_MARKNUM_FOLDEREND: c_int = 25;
pub const wxSTC_MARKNUM_FOLDEROPENMID: c_int = 26;
pub const wxSTC_MARKNUM_FOLDERMIDTAIL: c_int = 27;
pub const wxSTC_MARKNUM_FOLDERTAIL: c_int = 28;
pub const wxSTC_MARKNUM_FOLDERSUB: c_int = 29;
pub const wxSTC_MARKNUM_FOLDER: c_int = 30;
pub const wxSTC_MARKNUM_FOLDEROPEN: c_int = 31;
pub const wxSTC_MASK_FOLDERS: c_long = 0xFE000000;
pub const wxSTC_MARGIN_SYMBOL: c_int = 0;
pub const wxSTC_MARGIN_NUMBER: c_int = 1;
pub const wxSTC_MARGIN_BACK: c_int = 2;
pub const wxSTC_MARGIN_FORE: c_int = 3;
pub const wxSTC_MARGIN_TEXT: c_int = 4;
pub const wxSTC_MARGIN_RTEXT: c_int = 5;
pub const wxSTC_STYLE_DEFAULT: c_int = 32;
pub const wxSTC_STYLE_LINENUMBER: c_int = 33;
pub const wxSTC_STYLE_BRACELIGHT: c_int = 34;
pub const wxSTC_STYLE_BRACEBAD: c_int = 35;
pub const wxSTC_STYLE_CONTROLCHAR: c_int = 36;
pub const wxSTC_STYLE_INDENTGUIDE: c_int = 37;
pub const wxSTC_STYLE_CALLTIP: c_int = 38;
pub const wxSTC_STYLE_LASTPREDEFINED: c_int = 39;
pub const wxSTC_STYLE_MAX: c_int = 255;
pub const wxSTC_CHARSET_ANSI: c_int = 0;
pub const wxSTC_CHARSET_DEFAULT: c_int = 1;
pub const wxSTC_CHARSET_BALTIC: c_int = 186;
pub const wxSTC_CHARSET_CHINESEBIG5: c_int = 136;
pub const wxSTC_CHARSET_EASTEUROPE: c_int = 238;
pub const wxSTC_CHARSET_GB2312: c_int = 134;
pub const wxSTC_CHARSET_GREEK: c_int = 161;
pub const wxSTC_CHARSET_HANGUL: c_int = 129;
pub const wxSTC_CHARSET_MAC: c_int = 77;
pub const wxSTC_CHARSET_OEM: c_int = 255;
pub const wxSTC_CHARSET_RUSSIAN: c_int = 204;
pub const wxSTC_CHARSET_CYRILLIC: c_int = 1251;
pub const wxSTC_CHARSET_SHIFTJIS: c_int = 128;
pub const wxSTC_CHARSET_SYMBOL: c_int = 2;
pub const wxSTC_CHARSET_TURKISH: c_int = 162;
pub const wxSTC_CHARSET_JOHAB: c_int = 130;
pub const wxSTC_CHARSET_HEBREW: c_int = 177;
pub const wxSTC_CHARSET_ARABIC: c_int = 178;
pub const wxSTC_CHARSET_VIETNAMESE: c_int = 163;
pub const wxSTC_CHARSET_THAI: c_int = 222;
pub const wxSTC_CHARSET_8859_15: c_int = 1000;
pub const wxSTC_CASE_MIXED: c_int = 0;
pub const wxSTC_CASE_UPPER: c_int = 1;
pub const wxSTC_CASE_LOWER: c_int = 2;
pub const wxSTC_FONT_SIZE_MULTIPLIER: c_int = 100;
pub const wxSTC_WEIGHT_NORMAL: c_int = 400;
pub const wxSTC_WEIGHT_SEMIBOLD: c_int = 600;
pub const wxSTC_WEIGHT_BOLD: c_int = 700;
pub const wxSTC_INDIC_PLAIN: c_int = 0;
pub const wxSTC_INDIC_SQUIGGLE: c_int = 1;
pub const wxSTC_INDIC_TT: c_int = 2;
pub const wxSTC_INDIC_DIAGONAL: c_int = 3;
pub const wxSTC_INDIC_STRIKE: c_int = 4;
pub const wxSTC_INDIC_HIDDEN: c_int = 5;
pub const wxSTC_INDIC_BOX: c_int = 6;
pub const wxSTC_INDIC_ROUNDBOX: c_int = 7;
pub const wxSTC_INDIC_STRAIGHTBOX: c_int = 8;
pub const wxSTC_INDIC_DASH: c_int = 9;
pub const wxSTC_INDIC_DOTS: c_int = 10;
pub const wxSTC_INDIC_SQUIGGLELOW: c_int = 11;
pub const wxSTC_INDIC_DOTBOX: c_int = 12;
pub const wxSTC_INDIC_MAX: c_int = 31;
pub const wxSTC_INDIC_CONTAINER: c_int = 8;
pub const wxSTC_INDIC0_MASK: c_int = 0x20;
pub const wxSTC_INDIC1_MASK: c_int = 0x40;
pub const wxSTC_INDIC2_MASK: c_int = 0x80;
pub const wxSTC_INDICS_MASK: c_int = 0xE0;
pub const wxSTC_IV_NONE: c_int = 0;
pub const wxSTC_IV_REAL: c_int = 1;
pub const wxSTC_IV_LOOKFORWARD: c_int = 2;
pub const wxSTC_IV_LOOKBOTH: c_int = 3;
pub const wxSTC_PRINT_NORMAL: c_int = 0;
pub const wxSTC_PRINT_INVERTLIGHT: c_int = 1;
pub const wxSTC_PRINT_BLACKONWHITE: c_int = 2;
pub const wxSTC_PRINT_COLOURONWHITE: c_int = 3;
pub const wxSTC_PRINT_COLOURONWHITEDEFAULTBG: c_int = 4;
pub const wxSTC_FIND_WHOLEWORD: c_int = 2;
pub const wxSTC_FIND_MATCHCASE: c_int = 4;
pub const wxSTC_FIND_WORDSTART: c_int = 0x00100000;
pub const wxSTC_FIND_REGEXP: c_int = 0x00200000;
pub const wxSTC_FIND_POSIX: c_int = 0x00400000;
pub const wxSTC_FOLDLEVELBASE: c_int = 0x400;
pub const wxSTC_FOLDLEVELWHITEFLAG: c_int = 0x1000;
pub const wxSTC_FOLDLEVELHEADERFLAG: c_int = 0x2000;
pub const wxSTC_FOLDLEVELNUMBERMASK: c_int = 0x0FFF;
pub const wxSTC_FOLDFLAG_LINEBEFORE_EXPANDED: c_int = 0x0002;
pub const wxSTC_FOLDFLAG_LINEBEFORE_CONTRACTED: c_int = 0x0004;
pub const wxSTC_FOLDFLAG_LINEAFTER_EXPANDED: c_int = 0x0008;
pub const wxSTC_FOLDFLAG_LINEAFTER_CONTRACTED: c_int = 0x0010;
pub const wxSTC_FOLDFLAG_LEVELNUMBERS: c_int = 0x0040;
pub const wxSTC_TIME_FOREVER: c_int = 10000000;
pub const wxSTC_WRAP_NONE: c_int = 0;
pub const wxSTC_WRAP_WORD: c_int = 1;
pub const wxSTC_WRAP_CHAR: c_int = 2;
pub const wxSTC_WRAPVISUALFLAG_NONE: c_int = 0x0000;
pub const wxSTC_WRAPVISUALFLAG_END: c_int = 0x0001;
pub const wxSTC_WRAPVISUALFLAG_START: c_int = 0x0002;
pub const wxSTC_WRAPVISUALFLAG_MARGIN: c_int = 0x0004;
pub const wxSTC_WRAPVISUALFLAGLOC_DEFAULT: c_int = 0x0000;
pub const wxSTC_WRAPVISUALFLAGLOC_END_BY_TEXT: c_int = 0x0001;
pub const wxSTC_WRAPVISUALFLAGLOC_START_BY_TEXT: c_int = 0x0002;
pub const wxSTC_WRAPINDENT_FIXED: c_int = 0;
pub const wxSTC_WRAPINDENT_SAME: c_int = 1;
pub const wxSTC_WRAPINDENT_INDENT: c_int = 2;
pub const wxSTC_CACHE_NONE: c_int = 0;
pub const wxSTC_CACHE_CARET: c_int = 1;
pub const wxSTC_CACHE_PAGE: c_int = 2;
pub const wxSTC_CACHE_DOCUMENT: c_int = 3;
pub const wxSTC_EFF_QUALITY_MASK: c_int = 0xF;
pub const wxSTC_EFF_QUALITY_DEFAULT: c_int = 0;
pub const wxSTC_EFF_QUALITY_NON_ANTIALIASED: c_int = 1;
pub const wxSTC_EFF_QUALITY_ANTIALIASED: c_int = 2;
pub const wxSTC_EFF_QUALITY_LCD_OPTIMIZED: c_int = 3;
pub const wxSTC_MULTIPASTE_ONCE: c_int = 0;
pub const wxSTC_MULTIPASTE_EACH: c_int = 1;
pub const wxSTC_EDGE_NONE: c_int = 0;
pub const wxSTC_EDGE_LINE: c_int = 1;
pub const wxSTC_EDGE_BACKGROUND: c_int = 2;
pub const wxSTC_STATUS_OK: c_int = 0;
pub const wxSTC_STATUS_FAILURE: c_int = 1;
pub const wxSTC_STATUS_BADALLOC: c_int = 2;
pub const wxSTC_CURSORNORMAL: c_int = -1;
pub const wxSTC_CURSORARROW: c_int = 2;
pub const wxSTC_CURSORWAIT: c_int = 4;
pub const wxSTC_CURSORREVERSEARROW: c_int = 7;
pub const wxSTC_VISIBLE_SLOP: c_int = 0x01;
pub const wxSTC_VISIBLE_STRICT: c_int = 0x04;
pub const wxSTC_CARET_SLOP: c_int = 0x01;
pub const wxSTC_CARET_STRICT: c_int = 0x04;
pub const wxSTC_CARET_JUMPS: c_int = 0x10;
pub const wxSTC_CARET_EVEN: c_int = 0x08;
pub const wxSTC_SEL_STREAM: c_int = 0;
pub const wxSTC_SEL_RECTANGLE: c_int = 1;
pub const wxSTC_SEL_LINES: c_int = 2;
pub const wxSTC_SEL_THIN: c_int = 3;
pub const wxSTC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE: c_int = 0;
pub const wxSTC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE: c_int = 1;
pub const wxSTC_CARETSTICKY_OFF: c_int = 0;
pub const wxSTC_CARETSTICKY_ON: c_int = 1;
pub const wxSTC_CARETSTICKY_WHITESPACE: c_int = 2;
pub const wxSTC_ALPHA_TRANSPARENT: c_int = 0;
pub const wxSTC_ALPHA_OPAQUE: c_int = 255;
pub const wxSTC_ALPHA_NOALPHA: c_int = 256;
pub const wxSTC_CARETSTYLE_INVISIBLE: c_int = 0;
pub const wxSTC_CARETSTYLE_LINE: c_int = 1;
pub const wxSTC_CARETSTYLE_BLOCK: c_int = 2;
pub const wxSTC_MARGINOPTION_NONE: c_int = 0;
pub const wxSTC_MARGINOPTION_SUBLINESELECT: c_int = 1;
pub const wxSTC_ANNOTATION_HIDDEN: c_int = 0;
pub const wxSTC_ANNOTATION_STANDARD: c_int = 1;
pub const wxSTC_ANNOTATION_BOXED: c_int = 2;
pub const wxSTC_UNDO_MAY_COALESCE: c_int = 1;
pub const wxSTC_SCVS_NONE: c_int = 0;
pub const wxSTC_SCVS_RECTANGULARSELECTION: c_int = 1;
pub const wxSTC_SCVS_USERACCESSIBLE: c_int = 2;
pub const wxSTC_TECHNOLOGY_DEFAULT: c_int = 0;
pub const wxSTC_TECHNOLOGY_DIRECTWRITE: c_int = 1;
pub const wxSTC_KEYWORDSET_MAX: c_int = 8;
pub const wxSTC_TYPE_BOOLEAN: c_int = 0;
pub const wxSTC_TYPE_INTEGER: c_int = 1;
pub const wxSTC_TYPE_STRING: c_int = 2;
pub const wxSTC_MOD_INSERTTEXT: c_int = 0x1;
pub const wxSTC_MOD_DELETETEXT: c_int = 0x2;
pub const wxSTC_MOD_CHANGESTYLE: c_int = 0x4;
pub const wxSTC_MOD_CHANGEFOLD: c_int = 0x8;
pub const wxSTC_PERFORMED_USER: c_int = 0x10;
pub const wxSTC_PERFORMED_UNDO: c_int = 0x20;
pub const wxSTC_PERFORMED_REDO: c_int = 0x40;
pub const wxSTC_MULTISTEPUNDOREDO: c_int = 0x80;
pub const wxSTC_LASTSTEPINUNDOREDO: c_int = 0x100;
pub const wxSTC_MOD_CHANGEMARKER: c_int = 0x200;
pub const wxSTC_MOD_BEFOREINSERT: c_int = 0x400;
pub const wxSTC_MOD_BEFOREDELETE: c_int = 0x800;
pub const wxSTC_MULTILINEUNDOREDO: c_int = 0x1000;
pub const wxSTC_STARTACTION: c_int = 0x2000;
pub const wxSTC_MOD_CHANGEINDICATOR: c_int = 0x4000;
pub const wxSTC_MOD_CHANGELINESTATE: c_int = 0x8000;
pub const wxSTC_MOD_CHANGEMARGIN: c_int = 0x10000;
pub const wxSTC_MOD_CHANGEANNOTATION: c_int = 0x20000;
pub const wxSTC_MOD_CONTAINER: c_int = 0x40000;
pub const wxSTC_MOD_LEXERSTATE: c_int = 0x80000;
pub const wxSTC_MODEVENTMASKALL: c_int = 0xFFFFF;
pub const wxSTC_UPDATE_CONTENT: c_int = 0x1;
pub const wxSTC_UPDATE_SELECTION: c_int = 0x2;
pub const wxSTC_UPDATE_V_SCROLL: c_int = 0x4;
pub const wxSTC_UPDATE_H_SCROLL: c_int = 0x8;
pub const wxSTC_KEY_DOWN: c_int = 300;
pub const wxSTC_KEY_UP: c_int = 301;
pub const wxSTC_KEY_LEFT: c_int = 302;
pub const wxSTC_KEY_RIGHT: c_int = 303;
pub const wxSTC_KEY_HOME: c_int = 304;
pub const wxSTC_KEY_END: c_int = 305;
pub const wxSTC_KEY_PRIOR: c_int = 306;
pub const wxSTC_KEY_NEXT: c_int = 307;
pub const wxSTC_KEY_DELETE: c_int = 308;
pub const wxSTC_KEY_INSERT: c_int = 309;
pub const wxSTC_KEY_ESCAPE: c_int = 7;
pub const wxSTC_KEY_BACK: c_int = 8;
pub const wxSTC_KEY_TAB: c_int = 9;
pub const wxSTC_KEY_RETURN: c_int = 13;
pub const wxSTC_KEY_ADD: c_int = 310;
pub const wxSTC_KEY_SUBTRACT: c_int = 311;
pub const wxSTC_KEY_DIVIDE: c_int = 312;
pub const wxSTC_KEY_WIN: c_int = 313;
pub const wxSTC_KEY_RWIN: c_int = 314;
pub const wxSTC_KEY_MENU: c_int = 315;
pub const wxSTC_SCMOD_NORM: c_int = 0;
pub const wxSTC_SCMOD_SHIFT: c_int = 1;
pub const wxSTC_SCMOD_CTRL: c_int = 2;
pub const wxSTC_SCMOD_ALT: c_int = 4;
pub const wxSTC_SCMOD_SUPER: c_int = 8;
pub const wxSTC_SCMOD_META: c_int = 16;
pub const wxSTC_LEX_CONTAINER: c_int = 0;
pub const wxSTC_LEX_NULL: c_int = 1;
pub const wxSTC_LEX_PYTHON: c_int = 2;
pub const wxSTC_LEX_CPP: c_int = 3;
pub const wxSTC_LEX_HTML: c_int = 4;
pub const wxSTC_LEX_XML: c_int = 5;
pub const wxSTC_LEX_PERL: c_int = 6;
pub const wxSTC_LEX_SQL: c_int = 7;
pub const wxSTC_LEX_VB: c_int = 8;
pub const wxSTC_LEX_PROPERTIES: c_int = 9;
pub const wxSTC_LEX_ERRORLIST: c_int = 10;
pub const wxSTC_LEX_MAKEFILE: c_int = 11;
pub const wxSTC_LEX_BATCH: c_int = 12;
pub const wxSTC_LEX_XCODE: c_int = 13;
pub const wxSTC_LEX_LATEX: c_int = 14;
pub const wxSTC_LEX_LUA: c_int = 15;
pub const wxSTC_LEX_DIFF: c_int = 16;
pub const wxSTC_LEX_CONF: c_int = 17;
pub const wxSTC_LEX_PASCAL: c_int = 18;
pub const wxSTC_LEX_AVE: c_int = 19;
pub const wxSTC_LEX_ADA: c_int = 20;
pub const wxSTC_LEX_LISP: c_int = 21;
pub const wxSTC_LEX_RUBY: c_int = 22;
pub const wxSTC_LEX_EIFFEL: c_int = 23;
pub const wxSTC_LEX_EIFFELKW: c_int = 24;
pub const wxSTC_LEX_TCL: c_int = 25;
pub const wxSTC_LEX_NNCRONTAB: c_int = 26;
pub const wxSTC_LEX_BULLANT: c_int = 27;
pub const wxSTC_LEX_VBSCRIPT: c_int = 28;
pub const wxSTC_LEX_BAAN: c_int = 31;
pub const wxSTC_LEX_MATLAB: c_int = 32;
pub const wxSTC_LEX_SCRIPTOL: c_int = 33;
pub const wxSTC_LEX_ASM: c_int = 34;
pub const wxSTC_LEX_CPPNOCASE: c_int = 35;
pub const wxSTC_LEX_FORTRAN: c_int = 36;
pub const wxSTC_LEX_F77: c_int = 37;
pub const wxSTC_LEX_CSS: c_int = 38;
pub const wxSTC_LEX_POV: c_int = 39;
pub const wxSTC_LEX_LOUT: c_int = 40;
pub const wxSTC_LEX_ESCRIPT: c_int = 41;
pub const wxSTC_LEX_PS: c_int = 42;
pub const wxSTC_LEX_NSIS: c_int = 43;
pub const wxSTC_LEX_MMIXAL: c_int = 44;
pub const wxSTC_LEX_CLW: c_int = 45;
pub const wxSTC_LEX_CLWNOCASE: c_int = 46;
pub const wxSTC_LEX_LOT: c_int = 47;
pub const wxSTC_LEX_YAML: c_int = 48;
pub const wxSTC_LEX_TEX: c_int = 49;
pub const wxSTC_LEX_METAPOST: c_int = 50;
pub const wxSTC_LEX_POWERBASIC: c_int = 51;
pub const wxSTC_LEX_FORTH: c_int = 52;
pub const wxSTC_LEX_ERLANG: c_int = 53;
pub const wxSTC_LEX_OCTAVE: c_int = 54;
pub const wxSTC_LEX_MSSQL: c_int = 55;
pub const wxSTC_LEX_VERILOG: c_int = 56;
pub const wxSTC_LEX_KIX: c_int = 57;
pub const wxSTC_LEX_GUI4CLI: c_int = 58;
pub const wxSTC_LEX_SPECMAN: c_int = 59;
pub const wxSTC_LEX_AU3: c_int = 60;
pub const wxSTC_LEX_APDL: c_int = 61;
pub const wxSTC_LEX_BASH: c_int = 62;
pub const wxSTC_LEX_ASN1: c_int = 63;
pub const wxSTC_LEX_VHDL: c_int = 64;
pub const wxSTC_LEX_CAML: c_int = 65;
pub const wxSTC_LEX_BLITZBASIC: c_int = 66;
pub const wxSTC_LEX_PUREBASIC: c_int = 67;
pub const wxSTC_LEX_HASKELL: c_int = 68;
pub const wxSTC_LEX_PHPSCRIPT: c_int = 69;
pub const wxSTC_LEX_TADS3: c_int = 70;
pub const wxSTC_LEX_REBOL: c_int = 71;
pub const wxSTC_LEX_SMALLTALK: c_int = 72;
pub const wxSTC_LEX_FLAGSHIP: c_int = 73;
pub const wxSTC_LEX_CSOUND: c_int = 74;
pub const wxSTC_LEX_FREEBASIC: c_int = 75;
pub const wxSTC_LEX_INNOSETUP: c_int = 76;
pub const wxSTC_LEX_OPAL: c_int = 77;
pub const wxSTC_LEX_SPICE: c_int = 78;
pub const wxSTC_LEX_D: c_int = 79;
pub const wxSTC_LEX_CMAKE: c_int = 80;
pub const wxSTC_LEX_GAP: c_int = 81;
pub const wxSTC_LEX_PLM: c_int = 82;
pub const wxSTC_LEX_PROGRESS: c_int = 83;
pub const wxSTC_LEX_ABAQUS: c_int = 84;
pub const wxSTC_LEX_ASYMPTOTE: c_int = 85;
pub const wxSTC_LEX_R: c_int = 86;
pub const wxSTC_LEX_MAGIK: c_int = 87;
pub const wxSTC_LEX_POWERSHELL: c_int = 88;
pub const wxSTC_LEX_MYSQL: c_int = 89;
pub const wxSTC_LEX_PO: c_int = 90;
pub const wxSTC_LEX_TAL: c_int = 91;
pub const wxSTC_LEX_COBOL: c_int = 92;
pub const wxSTC_LEX_TACL: c_int = 93;
pub const wxSTC_LEX_SORCUS: c_int = 94;
pub const wxSTC_LEX_POWERPRO: c_int = 95;
pub const wxSTC_LEX_NIMROD: c_int = 96;
pub const wxSTC_LEX_SML: c_int = 97;
pub const wxSTC_LEX_MARKDOWN: c_int = 98;
pub const wxSTC_LEX_TXT2TAGS: c_int = 99;
pub const wxSTC_LEX_A68K: c_int = 100;
pub const wxSTC_LEX_MODULA: c_int = 101;
pub const wxSTC_LEX_COFFEESCRIPT: c_int = 102;
pub const wxSTC_LEX_TCMD: c_int = 103;
pub const wxSTC_LEX_AVS: c_int = 104;
pub const wxSTC_LEX_ECL: c_int = 105;
pub const wxSTC_LEX_OSCRIPT: c_int = 106;
pub const wxSTC_LEX_VISUALPROLOG: c_int = 107;
pub const wxSTC_LEX_AUTOMATIC: c_int = 1000;
pub const wxSTC_P_DEFAULT: c_int = 0;
pub const wxSTC_P_COMMENTLINE: c_int = 1;
pub const wxSTC_P_NUMBER: c_int = 2;
pub const wxSTC_P_STRING: c_int = 3;
pub const wxSTC_P_CHARACTER: c_int = 4;
pub const wxSTC_P_WORD: c_int = 5;
pub const wxSTC_P_TRIPLE: c_int = 6;
pub const wxSTC_P_TRIPLEDOUBLE: c_int = 7;
pub const wxSTC_P_CLASSNAME: c_int = 8;
pub const wxSTC_P_DEFNAME: c_int = 9;
pub const wxSTC_P_OPERATOR: c_int = 10;
pub const wxSTC_P_IDENTIFIER: c_int = 11;
pub const wxSTC_P_COMMENTBLOCK: c_int = 12;
pub const wxSTC_P_STRINGEOL: c_int = 13;
pub const wxSTC_P_WORD2: c_int = 14;
pub const wxSTC_P_DECORATOR: c_int = 15;
pub const wxSTC_C_DEFAULT: c_int = 0;
pub const wxSTC_C_COMMENT: c_int = 1;
pub const wxSTC_C_COMMENTLINE: c_int = 2;
pub const wxSTC_C_COMMENTDOC: c_int = 3;
pub const wxSTC_C_NUMBER: c_int = 4;
pub const wxSTC_C_WORD: c_int = 5;
pub const wxSTC_C_STRING: c_int = 6;
pub const wxSTC_C_CHARACTER: c_int = 7;
pub const wxSTC_C_UUID: c_int = 8;
pub const wxSTC_C_PREPROCESSOR: c_int = 9;
pub const wxSTC_C_OPERATOR: c_int = 10;
pub const wxSTC_C_IDENTIFIER: c_int = 11;
pub const wxSTC_C_STRINGEOL: c_int = 12;
pub const wxSTC_C_VERBATIM: c_int = 13;
pub const wxSTC_C_REGEX: c_int = 14;
pub const wxSTC_C_COMMENTLINEDOC: c_int = 15;
pub const wxSTC_C_WORD2: c_int = 16;
pub const wxSTC_C_COMMENTDOCKEYWORD: c_int = 17;
pub const wxSTC_C_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const wxSTC_C_GLOBALCLASS: c_int = 19;
pub const wxSTC_C_STRINGRAW: c_int = 20;
pub const wxSTC_C_TRIPLEVERBATIM: c_int = 21;
pub const wxSTC_C_HASHQUOTEDSTRING: c_int = 22;
pub const wxSTC_C_PREPROCESSORCOMMENT: c_int = 23;
pub const wxSTC_D_DEFAULT: c_int = 0;
pub const wxSTC_D_COMMENT: c_int = 1;
pub const wxSTC_D_COMMENTLINE: c_int = 2;
pub const wxSTC_D_COMMENTDOC: c_int = 3;
pub const wxSTC_D_COMMENTNESTED: c_int = 4;
pub const wxSTC_D_NUMBER: c_int = 5;
pub const wxSTC_D_WORD: c_int = 6;
pub const wxSTC_D_WORD2: c_int = 7;
pub const wxSTC_D_WORD3: c_int = 8;
pub const wxSTC_D_TYPEDEF: c_int = 9;
pub const wxSTC_D_STRING: c_int = 10;
pub const wxSTC_D_STRINGEOL: c_int = 11;
pub const wxSTC_D_CHARACTER: c_int = 12;
pub const wxSTC_D_OPERATOR: c_int = 13;
pub const wxSTC_D_IDENTIFIER: c_int = 14;
pub const wxSTC_D_COMMENTLINEDOC: c_int = 15;
pub const wxSTC_D_COMMENTDOCKEYWORD: c_int = 16;
pub const wxSTC_D_COMMENTDOCKEYWORDERROR: c_int = 17;
pub const wxSTC_D_STRINGB: c_int = 18;
pub const wxSTC_D_STRINGR: c_int = 19;
pub const wxSTC_D_WORD5: c_int = 20;
pub const wxSTC_D_WORD6: c_int = 21;
pub const wxSTC_D_WORD7: c_int = 22;
pub const wxSTC_TCL_DEFAULT: c_int = 0;
pub const wxSTC_TCL_COMMENT: c_int = 1;
pub const wxSTC_TCL_COMMENTLINE: c_int = 2;
pub const wxSTC_TCL_NUMBER: c_int = 3;
pub const wxSTC_TCL_WORD_IN_QUOTE: c_int = 4;
pub const wxSTC_TCL_IN_QUOTE: c_int = 5;
pub const wxSTC_TCL_OPERATOR: c_int = 6;
pub const wxSTC_TCL_IDENTIFIER: c_int = 7;
pub const wxSTC_TCL_SUBSTITUTION: c_int = 8;
pub const wxSTC_TCL_SUB_BRACE: c_int = 9;
pub const wxSTC_TCL_MODIFIER: c_int = 10;
pub const wxSTC_TCL_EXPAND: c_int = 11;
pub const wxSTC_TCL_WORD: c_int = 12;
pub const wxSTC_TCL_WORD2: c_int = 13;
pub const wxSTC_TCL_WORD3: c_int = 14;
pub const wxSTC_TCL_WORD4: c_int = 15;
pub const wxSTC_TCL_WORD5: c_int = 16;
pub const wxSTC_TCL_WORD6: c_int = 17;
pub const wxSTC_TCL_WORD7: c_int = 18;
pub const wxSTC_TCL_WORD8: c_int = 19;
pub const wxSTC_TCL_COMMENT_BOX: c_int = 20;
pub const wxSTC_TCL_BLOCK_COMMENT: c_int = 21;
pub const wxSTC_H_DEFAULT: c_int = 0;
pub const wxSTC_H_TAG: c_int = 1;
pub const wxSTC_H_TAGUNKNOWN: c_int = 2;
pub const wxSTC_H_ATTRIBUTE: c_int = 3;
pub const wxSTC_H_ATTRIBUTEUNKNOWN: c_int = 4;
pub const wxSTC_H_NUMBER: c_int = 5;
pub const wxSTC_H_DOUBLESTRING: c_int = 6;
pub const wxSTC_H_SINGLESTRING: c_int = 7;
pub const wxSTC_H_OTHER: c_int = 8;
pub const wxSTC_H_COMMENT: c_int = 9;
pub const wxSTC_H_ENTITY: c_int = 10;
pub const wxSTC_H_TAGEND: c_int = 11;
pub const wxSTC_H_XMLSTART: c_int = 12;
pub const wxSTC_H_XMLEND: c_int = 13;
pub const wxSTC_H_SCRIPT: c_int = 14;
pub const wxSTC_H_ASP: c_int = 15;
pub const wxSTC_H_ASPAT: c_int = 16;
pub const wxSTC_H_CDATA: c_int = 17;
pub const wxSTC_H_QUESTION: c_int = 18;
pub const wxSTC_H_VALUE: c_int = 19;
pub const wxSTC_H_XCCOMMENT: c_int = 20;
pub const wxSTC_H_SGML_DEFAULT: c_int = 21;
pub const wxSTC_H_SGML_COMMAND: c_int = 22;
pub const wxSTC_H_SGML_1ST_PARAM: c_int = 23;
pub const wxSTC_H_SGML_DOUBLESTRING: c_int = 24;
pub const wxSTC_H_SGML_SIMPLESTRING: c_int = 25;
pub const wxSTC_H_SGML_ERROR: c_int = 26;
pub const wxSTC_H_SGML_SPECIAL: c_int = 27;
pub const wxSTC_H_SGML_ENTITY: c_int = 28;
pub const wxSTC_H_SGML_COMMENT: c_int = 29;
pub const wxSTC_H_SGML_1ST_PARAM_COMMENT: c_int = 30;
pub const wxSTC_H_SGML_BLOCK_DEFAULT: c_int = 31;
pub const wxSTC_HJ_START: c_int = 40;
pub const wxSTC_HJ_DEFAULT: c_int = 41;
pub const wxSTC_HJ_COMMENT: c_int = 42;
pub const wxSTC_HJ_COMMENTLINE: c_int = 43;
pub const wxSTC_HJ_COMMENTDOC: c_int = 44;
pub const wxSTC_HJ_NUMBER: c_int = 45;
pub const wxSTC_HJ_WORD: c_int = 46;
pub const wxSTC_HJ_KEYWORD: c_int = 47;
pub const wxSTC_HJ_DOUBLESTRING: c_int = 48;
pub const wxSTC_HJ_SINGLESTRING: c_int = 49;
pub const wxSTC_HJ_SYMBOLS: c_int = 50;
pub const wxSTC_HJ_STRINGEOL: c_int = 51;
pub const wxSTC_HJ_REGEX: c_int = 52;
pub const wxSTC_HJA_START: c_int = 55;
pub const wxSTC_HJA_DEFAULT: c_int = 56;
pub const wxSTC_HJA_COMMENT: c_int = 57;
pub const wxSTC_HJA_COMMENTLINE: c_int = 58;
pub const wxSTC_HJA_COMMENTDOC: c_int = 59;
pub const wxSTC_HJA_NUMBER: c_int = 60;
pub const wxSTC_HJA_WORD: c_int = 61;
pub const wxSTC_HJA_KEYWORD: c_int = 62;
pub const wxSTC_HJA_DOUBLESTRING: c_int = 63;
pub const wxSTC_HJA_SINGLESTRING: c_int = 64;
pub const wxSTC_HJA_SYMBOLS: c_int = 65;
pub const wxSTC_HJA_STRINGEOL: c_int = 66;
pub const wxSTC_HJA_REGEX: c_int = 67;
pub const wxSTC_HB_START: c_int = 70;
pub const wxSTC_HB_DEFAULT: c_int = 71;
pub const wxSTC_HB_COMMENTLINE: c_int = 72;
pub const wxSTC_HB_NUMBER: c_int = 73;
pub const wxSTC_HB_WORD: c_int = 74;
pub const wxSTC_HB_STRING: c_int = 75;
pub const wxSTC_HB_IDENTIFIER: c_int = 76;
pub const wxSTC_HB_STRINGEOL: c_int = 77;
pub const wxSTC_HBA_START: c_int = 80;
pub const wxSTC_HBA_DEFAULT: c_int = 81;
pub const wxSTC_HBA_COMMENTLINE: c_int = 82;
pub const wxSTC_HBA_NUMBER: c_int = 83;
pub const wxSTC_HBA_WORD: c_int = 84;
pub const wxSTC_HBA_STRING: c_int = 85;
pub const wxSTC_HBA_IDENTIFIER: c_int = 86;
pub const wxSTC_HBA_STRINGEOL: c_int = 87;
pub const wxSTC_HP_START: c_int = 90;
pub const wxSTC_HP_DEFAULT: c_int = 91;
pub const wxSTC_HP_COMMENTLINE: c_int = 92;
pub const wxSTC_HP_NUMBER: c_int = 93;
pub const wxSTC_HP_STRING: c_int = 94;
pub const wxSTC_HP_CHARACTER: c_int = 95;
pub const wxSTC_HP_WORD: c_int = 96;
pub const wxSTC_HP_TRIPLE: c_int = 97;
pub const wxSTC_HP_TRIPLEDOUBLE: c_int = 98;
pub const wxSTC_HP_CLASSNAME: c_int = 99;
pub const wxSTC_HP_DEFNAME: c_int = 100;
pub const wxSTC_HP_OPERATOR: c_int = 101;
pub const wxSTC_HP_IDENTIFIER: c_int = 102;
pub const wxSTC_HPHP_COMPLEX_VARIABLE: c_int = 104;
pub const wxSTC_HPA_START: c_int = 105;
pub const wxSTC_HPA_DEFAULT: c_int = 106;
pub const wxSTC_HPA_COMMENTLINE: c_int = 107;
pub const wxSTC_HPA_NUMBER: c_int = 108;
pub const wxSTC_HPA_STRING: c_int = 109;
pub const wxSTC_HPA_CHARACTER: c_int = 110;
pub const wxSTC_HPA_WORD: c_int = 111;
pub const wxSTC_HPA_TRIPLE: c_int = 112;
pub const wxSTC_HPA_TRIPLEDOUBLE: c_int = 113;
pub const wxSTC_HPA_CLASSNAME: c_int = 114;
pub const wxSTC_HPA_DEFNAME: c_int = 115;
pub const wxSTC_HPA_OPERATOR: c_int = 116;
pub const wxSTC_HPA_IDENTIFIER: c_int = 117;
pub const wxSTC_HPHP_DEFAULT: c_int = 118;
pub const wxSTC_HPHP_HSTRING: c_int = 119;
pub const wxSTC_HPHP_SIMPLESTRING: c_int = 120;
pub const wxSTC_HPHP_WORD: c_int = 121;
pub const wxSTC_HPHP_NUMBER: c_int = 122;
pub const wxSTC_HPHP_VARIABLE: c_int = 123;
pub const wxSTC_HPHP_COMMENT: c_int = 124;
pub const wxSTC_HPHP_COMMENTLINE: c_int = 125;
pub const wxSTC_HPHP_HSTRING_VARIABLE: c_int = 126;
pub const wxSTC_HPHP_OPERATOR: c_int = 127;
pub const wxSTC_PL_DEFAULT: c_int = 0;
pub const wxSTC_PL_ERROR: c_int = 1;
pub const wxSTC_PL_COMMENTLINE: c_int = 2;
pub const wxSTC_PL_POD: c_int = 3;
pub const wxSTC_PL_NUMBER: c_int = 4;
pub const wxSTC_PL_WORD: c_int = 5;
pub const wxSTC_PL_STRING: c_int = 6;
pub const wxSTC_PL_CHARACTER: c_int = 7;
pub const wxSTC_PL_PUNCTUATION: c_int = 8;
pub const wxSTC_PL_PREPROCESSOR: c_int = 9;
pub const wxSTC_PL_OPERATOR: c_int = 10;
pub const wxSTC_PL_IDENTIFIER: c_int = 11;
pub const wxSTC_PL_SCALAR: c_int = 12;
pub const wxSTC_PL_ARRAY: c_int = 13;
pub const wxSTC_PL_HASH: c_int = 14;
pub const wxSTC_PL_SYMBOLTABLE: c_int = 15;
pub const wxSTC_PL_VARIABLE_INDEXER: c_int = 16;
pub const wxSTC_PL_REGEX: c_int = 17;
pub const wxSTC_PL_REGSUBST: c_int = 18;
pub const wxSTC_PL_LONGQUOTE: c_int = 19;
pub const wxSTC_PL_BACKTICKS: c_int = 20;
pub const wxSTC_PL_DATASECTION: c_int = 21;
pub const wxSTC_PL_HERE_DELIM: c_int = 22;
pub const wxSTC_PL_HERE_Q: c_int = 23;
pub const wxSTC_PL_HERE_QQ: c_int = 24;
pub const wxSTC_PL_HERE_QX: c_int = 25;
pub const wxSTC_PL_STRING_Q: c_int = 26;
pub const wxSTC_PL_STRING_QQ: c_int = 27;
pub const wxSTC_PL_STRING_QX: c_int = 28;
pub const wxSTC_PL_STRING_QR: c_int = 29;
pub const wxSTC_PL_STRING_QW: c_int = 30;
pub const wxSTC_PL_POD_VERB: c_int = 31;
pub const wxSTC_PL_SUB_PROTOTYPE: c_int = 40;
pub const wxSTC_PL_FORMAT_IDENT: c_int = 41;
pub const wxSTC_PL_FORMAT: c_int = 42;
pub const wxSTC_PL_STRING_VAR: c_int = 43;
pub const wxSTC_PL_XLAT: c_int = 44;
pub const wxSTC_PL_REGEX_VAR: c_int = 54;
pub const wxSTC_PL_REGSUBST_VAR: c_int = 55;
pub const wxSTC_PL_BACKTICKS_VAR: c_int = 57;
pub const wxSTC_PL_HERE_QQ_VAR: c_int = 61;
pub const wxSTC_PL_HERE_QX_VAR: c_int = 62;
pub const wxSTC_PL_STRING_QQ_VAR: c_int = 64;
pub const wxSTC_PL_STRING_QX_VAR: c_int = 65;
pub const wxSTC_PL_STRING_QR_VAR: c_int = 66;
pub const wxSTC_RB_DEFAULT: c_int = 0;
pub const wxSTC_RB_ERROR: c_int = 1;
pub const wxSTC_RB_COMMENTLINE: c_int = 2;
pub const wxSTC_RB_POD: c_int = 3;
pub const wxSTC_RB_NUMBER: c_int = 4;
pub const wxSTC_RB_WORD: c_int = 5;
pub const wxSTC_RB_STRING: c_int = 6;
pub const wxSTC_RB_CHARACTER: c_int = 7;
pub const wxSTC_RB_CLASSNAME: c_int = 8;
pub const wxSTC_RB_DEFNAME: c_int = 9;
pub const wxSTC_RB_OPERATOR: c_int = 10;
pub const wxSTC_RB_IDENTIFIER: c_int = 11;
pub const wxSTC_RB_REGEX: c_int = 12;
pub const wxSTC_RB_GLOBAL: c_int = 13;
pub const wxSTC_RB_SYMBOL: c_int = 14;
pub const wxSTC_RB_MODULE_NAME: c_int = 15;
pub const wxSTC_RB_INSTANCE_VAR: c_int = 16;
pub const wxSTC_RB_CLASS_VAR: c_int = 17;
pub const wxSTC_RB_BACKTICKS: c_int = 18;
pub const wxSTC_RB_DATASECTION: c_int = 19;
pub const wxSTC_RB_HERE_DELIM: c_int = 20;
pub const wxSTC_RB_HERE_Q: c_int = 21;
pub const wxSTC_RB_HERE_QQ: c_int = 22;
pub const wxSTC_RB_HERE_QX: c_int = 23;
pub const wxSTC_RB_STRING_Q: c_int = 24;
pub const wxSTC_RB_STRING_QQ: c_int = 25;
pub const wxSTC_RB_STRING_QX: c_int = 26;
pub const wxSTC_RB_STRING_QR: c_int = 27;
pub const wxSTC_RB_STRING_QW: c_int = 28;
pub const wxSTC_RB_WORD_DEMOTED: c_int = 29;
pub const wxSTC_RB_STDIN: c_int = 30;
pub const wxSTC_RB_STDOUT: c_int = 31;
pub const wxSTC_RB_STDERR: c_int = 40;
pub const wxSTC_RB_UPPER_BOUND: c_int = 41;
pub const wxSTC_B_DEFAULT: c_int = 0;
pub const wxSTC_B_COMMENT: c_int = 1;
pub const wxSTC_B_NUMBER: c_int = 2;
pub const wxSTC_B_KEYWORD: c_int = 3;
pub const wxSTC_B_STRING: c_int = 4;
pub const wxSTC_B_PREPROCESSOR: c_int = 5;
pub const wxSTC_B_OPERATOR: c_int = 6;
pub const wxSTC_B_IDENTIFIER: c_int = 7;
pub const wxSTC_B_DATE: c_int = 8;
pub const wxSTC_B_STRINGEOL: c_int = 9;
pub const wxSTC_B_KEYWORD2: c_int = 10;
pub const wxSTC_B_KEYWORD3: c_int = 11;
pub const wxSTC_B_KEYWORD4: c_int = 12;
pub const wxSTC_B_CONSTANT: c_int = 13;
pub const wxSTC_B_ASM: c_int = 14;
pub const wxSTC_B_LABEL: c_int = 15;
pub const wxSTC_B_ERROR: c_int = 16;
pub const wxSTC_B_HEXNUMBER: c_int = 17;
pub const wxSTC_B_BINNUMBER: c_int = 18;
pub const wxSTC_PROPS_DEFAULT: c_int = 0;
pub const wxSTC_PROPS_COMMENT: c_int = 1;
pub const wxSTC_PROPS_SECTION: c_int = 2;
pub const wxSTC_PROPS_ASSIGNMENT: c_int = 3;
pub const wxSTC_PROPS_DEFVAL: c_int = 4;
pub const wxSTC_PROPS_KEY: c_int = 5;
pub const wxSTC_L_DEFAULT: c_int = 0;
pub const wxSTC_L_COMMAND: c_int = 1;
pub const wxSTC_L_TAG: c_int = 2;
pub const wxSTC_L_MATH: c_int = 3;
pub const wxSTC_L_COMMENT: c_int = 4;
pub const wxSTC_L_TAG2: c_int = 5;
pub const wxSTC_L_MATH2: c_int = 6;
pub const wxSTC_L_COMMENT2: c_int = 7;
pub const wxSTC_L_VERBATIM: c_int = 8;
pub const wxSTC_L_SHORTCMD: c_int = 9;
pub const wxSTC_L_SPECIAL: c_int = 10;
pub const wxSTC_L_CMDOPT: c_int = 11;
pub const wxSTC_L_ERROR: c_int = 12;
pub const wxSTC_LUA_DEFAULT: c_int = 0;
pub const wxSTC_LUA_COMMENT: c_int = 1;
pub const wxSTC_LUA_COMMENTLINE: c_int = 2;
pub const wxSTC_LUA_COMMENTDOC: c_int = 3;
pub const wxSTC_LUA_NUMBER: c_int = 4;
pub const wxSTC_LUA_WORD: c_int = 5;
pub const wxSTC_LUA_STRING: c_int = 6;
pub const wxSTC_LUA_CHARACTER: c_int = 7;
pub const wxSTC_LUA_LITERALSTRING: c_int = 8;
pub const wxSTC_LUA_PREPROCESSOR: c_int = 9;
pub const wxSTC_LUA_OPERATOR: c_int = 10;
pub const wxSTC_LUA_IDENTIFIER: c_int = 11;
pub const wxSTC_LUA_STRINGEOL: c_int = 12;
pub const wxSTC_LUA_WORD2: c_int = 13;
pub const wxSTC_LUA_WORD3: c_int = 14;
pub const wxSTC_LUA_WORD4: c_int = 15;
pub const wxSTC_LUA_WORD5: c_int = 16;
pub const wxSTC_LUA_WORD6: c_int = 17;
pub const wxSTC_LUA_WORD7: c_int = 18;
pub const wxSTC_LUA_WORD8: c_int = 19;
pub const wxSTC_LUA_LABEL: c_int = 20;
pub const wxSTC_ERR_DEFAULT: c_int = 0;
pub const wxSTC_ERR_PYTHON: c_int = 1;
pub const wxSTC_ERR_GCC: c_int = 2;
pub const wxSTC_ERR_MS: c_int = 3;
pub const wxSTC_ERR_CMD: c_int = 4;
pub const wxSTC_ERR_BORLAND: c_int = 5;
pub const wxSTC_ERR_PERL: c_int = 6;
pub const wxSTC_ERR_NET: c_int = 7;
pub const wxSTC_ERR_LUA: c_int = 8;
pub const wxSTC_ERR_CTAG: c_int = 9;
pub const wxSTC_ERR_DIFF_CHANGED: c_int = 10;
pub const wxSTC_ERR_DIFF_ADDITION: c_int = 11;
pub const wxSTC_ERR_DIFF_DELETION: c_int = 12;
pub const wxSTC_ERR_DIFF_MESSAGE: c_int = 13;
pub const wxSTC_ERR_PHP: c_int = 14;
pub const wxSTC_ERR_ELF: c_int = 15;
pub const wxSTC_ERR_IFC: c_int = 16;
pub const wxSTC_ERR_IFORT: c_int = 17;
pub const wxSTC_ERR_ABSF: c_int = 18;
pub const wxSTC_ERR_TIDY: c_int = 19;
pub const wxSTC_ERR_JAVA_STACK: c_int = 20;
pub const wxSTC_ERR_VALUE: c_int = 21;
pub const wxSTC_BAT_DEFAULT: c_int = 0;
pub const wxSTC_BAT_COMMENT: c_int = 1;
pub const wxSTC_BAT_WORD: c_int = 2;
pub const wxSTC_BAT_LABEL: c_int = 3;
pub const wxSTC_BAT_HIDE: c_int = 4;
pub const wxSTC_BAT_COMMAND: c_int = 5;
pub const wxSTC_BAT_IDENTIFIER: c_int = 6;
pub const wxSTC_BAT_OPERATOR: c_int = 7;
pub const wxSTC_TCMD_DEFAULT: c_int = 0;
pub const wxSTC_TCMD_COMMENT: c_int = 1;
pub const wxSTC_TCMD_WORD: c_int = 2;
pub const wxSTC_TCMD_LABEL: c_int = 3;
pub const wxSTC_TCMD_HIDE: c_int = 4;
pub const wxSTC_TCMD_COMMAND: c_int = 5;
pub const wxSTC_TCMD_IDENTIFIER: c_int = 6;
pub const wxSTC_TCMD_OPERATOR: c_int = 7;
pub const wxSTC_TCMD_ENVIRONMENT: c_int = 8;
pub const wxSTC_TCMD_EXPANSION: c_int = 9;
pub const wxSTC_TCMD_CLABEL: c_int = 10;
pub const wxSTC_MAKE_DEFAULT: c_int = 0;
pub const wxSTC_MAKE_COMMENT: c_int = 1;
pub const wxSTC_MAKE_PREPROCESSOR: c_int = 2;
pub const wxSTC_MAKE_IDENTIFIER: c_int = 3;
pub const wxSTC_MAKE_OPERATOR: c_int = 4;
pub const wxSTC_MAKE_TARGET: c_int = 5;
pub const wxSTC_MAKE_IDEOL: c_int = 9;
pub const wxSTC_DIFF_DEFAULT: c_int = 0;
pub const wxSTC_DIFF_COMMENT: c_int = 1;
pub const wxSTC_DIFF_COMMAND: c_int = 2;
pub const wxSTC_DIFF_HEADER: c_int = 3;
pub const wxSTC_DIFF_POSITION: c_int = 4;
pub const wxSTC_DIFF_DELETED: c_int = 5;
pub const wxSTC_DIFF_ADDED: c_int = 6;
pub const wxSTC_DIFF_CHANGED: c_int = 7;
pub const wxSTC_CONF_DEFAULT: c_int = 0;
pub const wxSTC_CONF_COMMENT: c_int = 1;
pub const wxSTC_CONF_NUMBER: c_int = 2;
pub const wxSTC_CONF_IDENTIFIER: c_int = 3;
pub const wxSTC_CONF_EXTENSION: c_int = 4;
pub const wxSTC_CONF_PARAMETER: c_int = 5;
pub const wxSTC_CONF_STRING: c_int = 6;
pub const wxSTC_CONF_OPERATOR: c_int = 7;
pub const wxSTC_CONF_IP: c_int = 8;
pub const wxSTC_CONF_DIRECTIVE: c_int = 9;
pub const wxSTC_AVE_DEFAULT: c_int = 0;
pub const wxSTC_AVE_COMMENT: c_int = 1;
pub const wxSTC_AVE_NUMBER: c_int = 2;
pub const wxSTC_AVE_WORD: c_int = 3;
pub const wxSTC_AVE_STRING: c_int = 6;
pub const wxSTC_AVE_ENUM: c_int = 7;
pub const wxSTC_AVE_STRINGEOL: c_int = 8;
pub const wxSTC_AVE_IDENTIFIER: c_int = 9;
pub const wxSTC_AVE_OPERATOR: c_int = 10;
pub const wxSTC_AVE_WORD1: c_int = 11;
pub const wxSTC_AVE_WORD2: c_int = 12;
pub const wxSTC_AVE_WORD3: c_int = 13;
pub const wxSTC_AVE_WORD4: c_int = 14;
pub const wxSTC_AVE_WORD5: c_int = 15;
pub const wxSTC_AVE_WORD6: c_int = 16;
pub const wxSTC_ADA_DEFAULT: c_int = 0;
pub const wxSTC_ADA_WORD: c_int = 1;
pub const wxSTC_ADA_IDENTIFIER: c_int = 2;
pub const wxSTC_ADA_NUMBER: c_int = 3;
pub const wxSTC_ADA_DELIMITER: c_int = 4;
pub const wxSTC_ADA_CHARACTER: c_int = 5;
pub const wxSTC_ADA_CHARACTEREOL: c_int = 6;
pub const wxSTC_ADA_STRING: c_int = 7;
pub const wxSTC_ADA_STRINGEOL: c_int = 8;
pub const wxSTC_ADA_LABEL: c_int = 9;
pub const wxSTC_ADA_COMMENTLINE: c_int = 10;
pub const wxSTC_ADA_ILLEGAL: c_int = 11;
pub const wxSTC_BAAN_DEFAULT: c_int = 0;
pub const wxSTC_BAAN_COMMENT: c_int = 1;
pub const wxSTC_BAAN_COMMENTDOC: c_int = 2;
pub const wxSTC_BAAN_NUMBER: c_int = 3;
pub const wxSTC_BAAN_WORD: c_int = 4;
pub const wxSTC_BAAN_STRING: c_int = 5;
pub const wxSTC_BAAN_PREPROCESSOR: c_int = 6;
pub const wxSTC_BAAN_OPERATOR: c_int = 7;
pub const wxSTC_BAAN_IDENTIFIER: c_int = 8;
pub const wxSTC_BAAN_STRINGEOL: c_int = 9;
pub const wxSTC_BAAN_WORD2: c_int = 10;
pub const wxSTC_LISP_DEFAULT: c_int = 0;
pub const wxSTC_LISP_COMMENT: c_int = 1;
pub const wxSTC_LISP_NUMBER: c_int = 2;
pub const wxSTC_LISP_KEYWORD: c_int = 3;
pub const wxSTC_LISP_KEYWORD_KW: c_int = 4;
pub const wxSTC_LISP_SYMBOL: c_int = 5;
pub const wxSTC_LISP_STRING: c_int = 6;
pub const wxSTC_LISP_STRINGEOL: c_int = 8;
pub const wxSTC_LISP_IDENTIFIER: c_int = 9;
pub const wxSTC_LISP_OPERATOR: c_int = 10;
pub const wxSTC_LISP_SPECIAL: c_int = 11;
pub const wxSTC_LISP_MULTI_COMMENT: c_int = 12;
pub const wxSTC_EIFFEL_DEFAULT: c_int = 0;
pub const wxSTC_EIFFEL_COMMENTLINE: c_int = 1;
pub const wxSTC_EIFFEL_NUMBER: c_int = 2;
pub const wxSTC_EIFFEL_WORD: c_int = 3;
pub const wxSTC_EIFFEL_STRING: c_int = 4;
pub const wxSTC_EIFFEL_CHARACTER: c_int = 5;
pub const wxSTC_EIFFEL_OPERATOR: c_int = 6;
pub const wxSTC_EIFFEL_IDENTIFIER: c_int = 7;
pub const wxSTC_EIFFEL_STRINGEOL: c_int = 8;
pub const wxSTC_NNCRONTAB_DEFAULT: c_int = 0;
pub const wxSTC_NNCRONTAB_COMMENT: c_int = 1;
pub const wxSTC_NNCRONTAB_TASK: c_int = 2;
pub const wxSTC_NNCRONTAB_SECTION: c_int = 3;
pub const wxSTC_NNCRONTAB_KEYWORD: c_int = 4;
pub const wxSTC_NNCRONTAB_MODIFIER: c_int = 5;
pub const wxSTC_NNCRONTAB_ASTERISK: c_int = 6;
pub const wxSTC_NNCRONTAB_NUMBER: c_int = 7;
pub const wxSTC_NNCRONTAB_STRING: c_int = 8;
pub const wxSTC_NNCRONTAB_ENVIRONMENT: c_int = 9;
pub const wxSTC_NNCRONTAB_IDENTIFIER: c_int = 10;
pub const wxSTC_FORTH_DEFAULT: c_int = 0;
pub const wxSTC_FORTH_COMMENT: c_int = 1;
pub const wxSTC_FORTH_COMMENT_ML: c_int = 2;
pub const wxSTC_FORTH_IDENTIFIER: c_int = 3;
pub const wxSTC_FORTH_CONTROL: c_int = 4;
pub const wxSTC_FORTH_KEYWORD: c_int = 5;
pub const wxSTC_FORTH_DEFWORD: c_int = 6;
pub const wxSTC_FORTH_PREWORD1: c_int = 7;
pub const wxSTC_FORTH_PREWORD2: c_int = 8;
pub const wxSTC_FORTH_NUMBER: c_int = 9;
pub const wxSTC_FORTH_STRING: c_int = 10;
pub const wxSTC_FORTH_LOCALE: c_int = 11;
pub const wxSTC_MATLAB_DEFAULT: c_int = 0;
pub const wxSTC_MATLAB_COMMENT: c_int = 1;
pub const wxSTC_MATLAB_COMMAND: c_int = 2;
pub const wxSTC_MATLAB_NUMBER: c_int = 3;
pub const wxSTC_MATLAB_KEYWORD: c_int = 4;
pub const wxSTC_MATLAB_STRING: c_int = 5;
pub const wxSTC_MATLAB_OPERATOR: c_int = 6;
pub const wxSTC_MATLAB_IDENTIFIER: c_int = 7;
pub const wxSTC_MATLAB_DOUBLEQUOTESTRING: c_int = 8;
pub const wxSTC_SCRIPTOL_DEFAULT: c_int = 0;
pub const wxSTC_SCRIPTOL_WHITE: c_int = 1;
pub const wxSTC_SCRIPTOL_COMMENTLINE: c_int = 2;
pub const wxSTC_SCRIPTOL_PERSISTENT: c_int = 3;
pub const wxSTC_SCRIPTOL_CSTYLE: c_int = 4;
pub const wxSTC_SCRIPTOL_COMMENTBLOCK: c_int = 5;
pub const wxSTC_SCRIPTOL_NUMBER: c_int = 6;
pub const wxSTC_SCRIPTOL_STRING: c_int = 7;
pub const wxSTC_SCRIPTOL_CHARACTER: c_int = 8;
pub const wxSTC_SCRIPTOL_STRINGEOL: c_int = 9;
pub const wxSTC_SCRIPTOL_KEYWORD: c_int = 10;
pub const wxSTC_SCRIPTOL_OPERATOR: c_int = 11;
pub const wxSTC_SCRIPTOL_IDENTIFIER: c_int = 12;
pub const wxSTC_SCRIPTOL_TRIPLE: c_int = 13;
pub const wxSTC_SCRIPTOL_CLASSNAME: c_int = 14;
pub const wxSTC_SCRIPTOL_PREPROCESSOR: c_int = 15;
pub const wxSTC_ASM_DEFAULT: c_int = 0;
pub const wxSTC_ASM_COMMENT: c_int = 1;
pub const wxSTC_ASM_NUMBER: c_int = 2;
pub const wxSTC_ASM_STRING: c_int = 3;
pub const wxSTC_ASM_OPERATOR: c_int = 4;
pub const wxSTC_ASM_IDENTIFIER: c_int = 5;
pub const wxSTC_ASM_CPUINSTRUCTION: c_int = 6;
pub const wxSTC_ASM_MATHINSTRUCTION: c_int = 7;
pub const wxSTC_ASM_REGISTER: c_int = 8;
pub const wxSTC_ASM_DIRECTIVE: c_int = 9;
pub const wxSTC_ASM_DIRECTIVEOPERAND: c_int = 10;
pub const wxSTC_ASM_COMMENTBLOCK: c_int = 11;
pub const wxSTC_ASM_CHARACTER: c_int = 12;
pub const wxSTC_ASM_STRINGEOL: c_int = 13;
pub const wxSTC_ASM_EXTINSTRUCTION: c_int = 14;
pub const wxSTC_ASM_COMMENTDIRECTIVE: c_int = 15;
pub const wxSTC_F_DEFAULT: c_int = 0;
pub const wxSTC_F_COMMENT: c_int = 1;
pub const wxSTC_F_NUMBER: c_int = 2;
pub const wxSTC_F_STRING1: c_int = 3;
pub const wxSTC_F_STRING2: c_int = 4;
pub const wxSTC_F_STRINGEOL: c_int = 5;
pub const wxSTC_F_OPERATOR: c_int = 6;
pub const wxSTC_F_IDENTIFIER: c_int = 7;
pub const wxSTC_F_WORD: c_int = 8;
pub const wxSTC_F_WORD2: c_int = 9;
pub const wxSTC_F_WORD3: c_int = 10;
pub const wxSTC_F_PREPROCESSOR: c_int = 11;
pub const wxSTC_F_OPERATOR2: c_int = 12;
pub const wxSTC_F_LABEL: c_int = 13;
pub const wxSTC_F_CONTINUATION: c_int = 14;
pub const wxSTC_CSS_DEFAULT: c_int = 0;
pub const wxSTC_CSS_TAG: c_int = 1;
pub const wxSTC_CSS_CLASS: c_int = 2;
pub const wxSTC_CSS_PSEUDOCLASS: c_int = 3;
pub const wxSTC_CSS_UNKNOWN_PSEUDOCLASS: c_int = 4;
pub const wxSTC_CSS_OPERATOR: c_int = 5;
pub const wxSTC_CSS_IDENTIFIER: c_int = 6;
pub const wxSTC_CSS_UNKNOWN_IDENTIFIER: c_int = 7;
pub const wxSTC_CSS_VALUE: c_int = 8;
pub const wxSTC_CSS_COMMENT: c_int = 9;
pub const wxSTC_CSS_ID: c_int = 10;
pub const wxSTC_CSS_IMPORTANT: c_int = 11;
pub const wxSTC_CSS_DIRECTIVE: c_int = 12;
pub const wxSTC_CSS_DOUBLESTRING: c_int = 13;
pub const wxSTC_CSS_SINGLESTRING: c_int = 14;
pub const wxSTC_CSS_IDENTIFIER2: c_int = 15;
pub const wxSTC_CSS_ATTRIBUTE: c_int = 16;
pub const wxSTC_CSS_IDENTIFIER3: c_int = 17;
pub const wxSTC_CSS_PSEUDOELEMENT: c_int = 18;
pub const wxSTC_CSS_EXTENDED_IDENTIFIER: c_int = 19;
pub const wxSTC_CSS_EXTENDED_PSEUDOCLASS: c_int = 20;
pub const wxSTC_CSS_EXTENDED_PSEUDOELEMENT: c_int = 21;
pub const wxSTC_CSS_MEDIA: c_int = 22;
pub const wxSTC_CSS_VARIABLE: c_int = 23;
pub const wxSTC_POV_DEFAULT: c_int = 0;
pub const wxSTC_POV_COMMENT: c_int = 1;
pub const wxSTC_POV_COMMENTLINE: c_int = 2;
pub const wxSTC_POV_NUMBER: c_int = 3;
pub const wxSTC_POV_OPERATOR: c_int = 4;
pub const wxSTC_POV_IDENTIFIER: c_int = 5;
pub const wxSTC_POV_STRING: c_int = 6;
pub const wxSTC_POV_STRINGEOL: c_int = 7;
pub const wxSTC_POV_DIRECTIVE: c_int = 8;
pub const wxSTC_POV_BADDIRECTIVE: c_int = 9;
pub const wxSTC_POV_WORD2: c_int = 10;
pub const wxSTC_POV_WORD3: c_int = 11;
pub const wxSTC_POV_WORD4: c_int = 12;
pub const wxSTC_POV_WORD5: c_int = 13;
pub const wxSTC_POV_WORD6: c_int = 14;
pub const wxSTC_POV_WORD7: c_int = 15;
pub const wxSTC_POV_WORD8: c_int = 16;
pub const wxSTC_LOUT_DEFAULT: c_int = 0;
pub const wxSTC_LOUT_COMMENT: c_int = 1;
pub const wxSTC_LOUT_NUMBER: c_int = 2;
pub const wxSTC_LOUT_WORD: c_int = 3;
pub const wxSTC_LOUT_WORD2: c_int = 4;
pub const wxSTC_LOUT_WORD3: c_int = 5;
pub const wxSTC_LOUT_WORD4: c_int = 6;
pub const wxSTC_LOUT_STRING: c_int = 7;
pub const wxSTC_LOUT_OPERATOR: c_int = 8;
pub const wxSTC_LOUT_IDENTIFIER: c_int = 9;
pub const wxSTC_LOUT_STRINGEOL: c_int = 10;
pub const wxSTC_ESCRIPT_DEFAULT: c_int = 0;
pub const wxSTC_ESCRIPT_COMMENT: c_int = 1;
pub const wxSTC_ESCRIPT_COMMENTLINE: c_int = 2;
pub const wxSTC_ESCRIPT_COMMENTDOC: c_int = 3;
pub const wxSTC_ESCRIPT_NUMBER: c_int = 4;
pub const wxSTC_ESCRIPT_WORD: c_int = 5;
pub const wxSTC_ESCRIPT_STRING: c_int = 6;
pub const wxSTC_ESCRIPT_OPERATOR: c_int = 7;
pub const wxSTC_ESCRIPT_IDENTIFIER: c_int = 8;
pub const wxSTC_ESCRIPT_BRACE: c_int = 9;
pub const wxSTC_ESCRIPT_WORD2: c_int = 10;
pub const wxSTC_ESCRIPT_WORD3: c_int = 11;
pub const wxSTC_PS_DEFAULT: c_int = 0;
pub const wxSTC_PS_COMMENT: c_int = 1;
pub const wxSTC_PS_DSC_COMMENT: c_int = 2;
pub const wxSTC_PS_DSC_VALUE: c_int = 3;
pub const wxSTC_PS_NUMBER: c_int = 4;
pub const wxSTC_PS_NAME: c_int = 5;
pub const wxSTC_PS_KEYWORD: c_int = 6;
pub const wxSTC_PS_LITERAL: c_int = 7;
pub const wxSTC_PS_IMMEVAL: c_int = 8;
pub const wxSTC_PS_PAREN_ARRAY: c_int = 9;
pub const wxSTC_PS_PAREN_DICT: c_int = 10;
pub const wxSTC_PS_PAREN_PROC: c_int = 11;
pub const wxSTC_PS_TEXT: c_int = 12;
pub const wxSTC_PS_HEXSTRING: c_int = 13;
pub const wxSTC_PS_BASE85STRING: c_int = 14;
pub const wxSTC_PS_BADSTRINGCHAR: c_int = 15;
pub const wxSTC_NSIS_DEFAULT: c_int = 0;
pub const wxSTC_NSIS_COMMENT: c_int = 1;
pub const wxSTC_NSIS_STRINGDQ: c_int = 2;
pub const wxSTC_NSIS_STRINGLQ: c_int = 3;
pub const wxSTC_NSIS_STRINGRQ: c_int = 4;
pub const wxSTC_NSIS_FUNCTION: c_int = 5;
pub const wxSTC_NSIS_VARIABLE: c_int = 6;
pub const wxSTC_NSIS_LABEL: c_int = 7;
pub const wxSTC_NSIS_USERDEFINED: c_int = 8;
pub const wxSTC_NSIS_SECTIONDEF: c_int = 9;
pub const wxSTC_NSIS_SUBSECTIONDEF: c_int = 10;
pub const wxSTC_NSIS_IFDEFINEDEF: c_int = 11;
pub const wxSTC_NSIS_MACRODEF: c_int = 12;
pub const wxSTC_NSIS_STRINGVAR: c_int = 13;
pub const wxSTC_NSIS_NUMBER: c_int = 14;
pub const wxSTC_NSIS_SECTIONGROUP: c_int = 15;
pub const wxSTC_NSIS_PAGEEX: c_int = 16;
pub const wxSTC_NSIS_FUNCTIONDEF: c_int = 17;
pub const wxSTC_NSIS_COMMENTBOX: c_int = 18;
pub const wxSTC_MMIXAL_LEADWS: c_int = 0;
pub const wxSTC_MMIXAL_COMMENT: c_int = 1;
pub const wxSTC_MMIXAL_LABEL: c_int = 2;
pub const wxSTC_MMIXAL_OPCODE: c_int = 3;
pub const wxSTC_MMIXAL_OPCODE_PRE: c_int = 4;
pub const wxSTC_MMIXAL_OPCODE_VALID: c_int = 5;
pub const wxSTC_MMIXAL_OPCODE_UNKNOWN: c_int = 6;
pub const wxSTC_MMIXAL_OPCODE_POST: c_int = 7;
pub const wxSTC_MMIXAL_OPERANDS: c_int = 8;
pub const wxSTC_MMIXAL_NUMBER: c_int = 9;
pub const wxSTC_MMIXAL_REF: c_int = 10;
pub const wxSTC_MMIXAL_CHAR: c_int = 11;
pub const wxSTC_MMIXAL_STRING: c_int = 12;
pub const wxSTC_MMIXAL_REGISTER: c_int = 13;
pub const wxSTC_MMIXAL_HEX: c_int = 14;
pub const wxSTC_MMIXAL_OPERATOR: c_int = 15;
pub const wxSTC_MMIXAL_SYMBOL: c_int = 16;
pub const wxSTC_MMIXAL_INCLUDE: c_int = 17;
pub const wxSTC_CLW_DEFAULT: c_int = 0;
pub const wxSTC_CLW_LABEL: c_int = 1;
pub const wxSTC_CLW_COMMENT: c_int = 2;
pub const wxSTC_CLW_STRING: c_int = 3;
pub const wxSTC_CLW_USER_IDENTIFIER: c_int = 4;
pub const wxSTC_CLW_INTEGER_CONSTANT: c_int = 5;
pub const wxSTC_CLW_REAL_CONSTANT: c_int = 6;
pub const wxSTC_CLW_PICTURE_STRING: c_int = 7;
pub const wxSTC_CLW_KEYWORD: c_int = 8;
pub const wxSTC_CLW_COMPILER_DIRECTIVE: c_int = 9;
pub const wxSTC_CLW_RUNTIME_EXPRESSIONS: c_int = 10;
pub const wxSTC_CLW_BUILTIN_PROCEDURES_FUNCTION: c_int = 11;
pub const wxSTC_CLW_STRUCTURE_DATA_TYPE: c_int = 12;
pub const wxSTC_CLW_ATTRIBUTE: c_int = 13;
pub const wxSTC_CLW_STANDARD_EQUATE: c_int = 14;
pub const wxSTC_CLW_ERROR: c_int = 15;
pub const wxSTC_CLW_DEPRECATED: c_int = 16;
pub const wxSTC_LOT_DEFAULT: c_int = 0;
pub const wxSTC_LOT_HEADER: c_int = 1;
pub const wxSTC_LOT_BREAK: c_int = 2;
pub const wxSTC_LOT_SET: c_int = 3;
pub const wxSTC_LOT_PASS: c_int = 4;
pub const wxSTC_LOT_FAIL: c_int = 5;
pub const wxSTC_LOT_ABORT: c_int = 6;
pub const wxSTC_YAML_DEFAULT: c_int = 0;
pub const wxSTC_YAML_COMMENT: c_int = 1;
pub const wxSTC_YAML_IDENTIFIER: c_int = 2;
pub const wxSTC_YAML_KEYWORD: c_int = 3;
pub const wxSTC_YAML_NUMBER: c_int = 4;
pub const wxSTC_YAML_REFERENCE: c_int = 5;
pub const wxSTC_YAML_DOCUMENT: c_int = 6;
pub const wxSTC_YAML_TEXT: c_int = 7;
pub const wxSTC_YAML_ERROR: c_int = 8;
pub const wxSTC_YAML_OPERATOR: c_int = 9;
pub const wxSTC_TEX_DEFAULT: c_int = 0;
pub const wxSTC_TEX_SPECIAL: c_int = 1;
pub const wxSTC_TEX_GROUP: c_int = 2;
pub const wxSTC_TEX_SYMBOL: c_int = 3;
pub const wxSTC_TEX_COMMAND: c_int = 4;
pub const wxSTC_TEX_TEXT: c_int = 5;
pub const wxSTC_METAPOST_DEFAULT: c_int = 0;
pub const wxSTC_METAPOST_SPECIAL: c_int = 1;
pub const wxSTC_METAPOST_GROUP: c_int = 2;
pub const wxSTC_METAPOST_SYMBOL: c_int = 3;
pub const wxSTC_METAPOST_COMMAND: c_int = 4;
pub const wxSTC_METAPOST_TEXT: c_int = 5;
pub const wxSTC_METAPOST_EXTRA: c_int = 6;
pub const wxSTC_ERLANG_DEFAULT: c_int = 0;
pub const wxSTC_ERLANG_COMMENT: c_int = 1;
pub const wxSTC_ERLANG_VARIABLE: c_int = 2;
pub const wxSTC_ERLANG_NUMBER: c_int = 3;
pub const wxSTC_ERLANG_KEYWORD: c_int = 4;
pub const wxSTC_ERLANG_STRING: c_int = 5;
pub const wxSTC_ERLANG_OPERATOR: c_int = 6;
pub const wxSTC_ERLANG_ATOM: c_int = 7;
pub const wxSTC_ERLANG_FUNCTION_NAME: c_int = 8;
pub const wxSTC_ERLANG_CHARACTER: c_int = 9;
pub const wxSTC_ERLANG_MACRO: c_int = 10;
pub const wxSTC_ERLANG_RECORD: c_int = 11;
pub const wxSTC_ERLANG_PREPROC: c_int = 12;
pub const wxSTC_ERLANG_NODE_NAME: c_int = 13;
pub const wxSTC_ERLANG_COMMENT_FUNCTION: c_int = 14;
pub const wxSTC_ERLANG_COMMENT_MODULE: c_int = 15;
pub const wxSTC_ERLANG_COMMENT_DOC: c_int = 16;
pub const wxSTC_ERLANG_COMMENT_DOC_MACRO: c_int = 17;
pub const wxSTC_ERLANG_ATOM_QUOTED: c_int = 18;
pub const wxSTC_ERLANG_MACRO_QUOTED: c_int = 19;
pub const wxSTC_ERLANG_RECORD_QUOTED: c_int = 20;
pub const wxSTC_ERLANG_NODE_NAME_QUOTED: c_int = 21;
pub const wxSTC_ERLANG_BIFS: c_int = 22;
pub const wxSTC_ERLANG_MODULES: c_int = 23;
pub const wxSTC_ERLANG_MODULES_ATT: c_int = 24;
pub const wxSTC_ERLANG_UNKNOWN: c_int = 31;
pub const wxSTC_MSSQL_DEFAULT: c_int = 0;
pub const wxSTC_MSSQL_COMMENT: c_int = 1;
pub const wxSTC_MSSQL_LINE_COMMENT: c_int = 2;
pub const wxSTC_MSSQL_NUMBER: c_int = 3;
pub const wxSTC_MSSQL_STRING: c_int = 4;
pub const wxSTC_MSSQL_OPERATOR: c_int = 5;
pub const wxSTC_MSSQL_IDENTIFIER: c_int = 6;
pub const wxSTC_MSSQL_VARIABLE: c_int = 7;
pub const wxSTC_MSSQL_COLUMN_NAME: c_int = 8;
pub const wxSTC_MSSQL_STATEMENT: c_int = 9;
pub const wxSTC_MSSQL_DATATYPE: c_int = 10;
pub const wxSTC_MSSQL_SYSTABLE: c_int = 11;
pub const wxSTC_MSSQL_GLOBAL_VARIABLE: c_int = 12;
pub const wxSTC_MSSQL_FUNCTION: c_int = 13;
pub const wxSTC_MSSQL_STORED_PROCEDURE: c_int = 14;
pub const wxSTC_MSSQL_DEFAULT_PREF_DATATYPE: c_int = 15;
pub const wxSTC_MSSQL_COLUMN_NAME_2: c_int = 16;
pub const wxSTC_V_DEFAULT: c_int = 0;
pub const wxSTC_V_COMMENT: c_int = 1;
pub const wxSTC_V_COMMENTLINE: c_int = 2;
pub const wxSTC_V_COMMENTLINEBANG: c_int = 3;
pub const wxSTC_V_NUMBER: c_int = 4;
pub const wxSTC_V_WORD: c_int = 5;
pub const wxSTC_V_STRING: c_int = 6;
pub const wxSTC_V_WORD2: c_int = 7;
pub const wxSTC_V_WORD3: c_int = 8;
pub const wxSTC_V_PREPROCESSOR: c_int = 9;
pub const wxSTC_V_OPERATOR: c_int = 10;
pub const wxSTC_V_IDENTIFIER: c_int = 11;
pub const wxSTC_V_STRINGEOL: c_int = 12;
pub const wxSTC_V_USER: c_int = 19;
pub const wxSTC_KIX_DEFAULT: c_int = 0;
pub const wxSTC_KIX_COMMENT: c_int = 1;
pub const wxSTC_KIX_STRING1: c_int = 2;
pub const wxSTC_KIX_STRING2: c_int = 3;
pub const wxSTC_KIX_NUMBER: c_int = 4;
pub const wxSTC_KIX_VAR: c_int = 5;
pub const wxSTC_KIX_MACRO: c_int = 6;
pub const wxSTC_KIX_KEYWORD: c_int = 7;
pub const wxSTC_KIX_FUNCTIONS: c_int = 8;
pub const wxSTC_KIX_OPERATOR: c_int = 9;
pub const wxSTC_KIX_IDENTIFIER: c_int = 31;
pub const wxSTC_GC_DEFAULT: c_int = 0;
pub const wxSTC_GC_COMMENTLINE: c_int = 1;
pub const wxSTC_GC_COMMENTBLOCK: c_int = 2;
pub const wxSTC_GC_GLOBAL: c_int = 3;
pub const wxSTC_GC_EVENT: c_int = 4;
pub const wxSTC_GC_ATTRIBUTE: c_int = 5;
pub const wxSTC_GC_CONTROL: c_int = 6;
pub const wxSTC_GC_COMMAND: c_int = 7;
pub const wxSTC_GC_STRING: c_int = 8;
pub const wxSTC_GC_OPERATOR: c_int = 9;
pub const wxSTC_SN_DEFAULT: c_int = 0;
pub const wxSTC_SN_CODE: c_int = 1;
pub const wxSTC_SN_COMMENTLINE: c_int = 2;
pub const wxSTC_SN_COMMENTLINEBANG: c_int = 3;
pub const wxSTC_SN_NUMBER: c_int = 4;
pub const wxSTC_SN_WORD: c_int = 5;
pub const wxSTC_SN_STRING: c_int = 6;
pub const wxSTC_SN_WORD2: c_int = 7;
pub const wxSTC_SN_WORD3: c_int = 8;
pub const wxSTC_SN_PREPROCESSOR: c_int = 9;
pub const wxSTC_SN_OPERATOR: c_int = 10;
pub const wxSTC_SN_IDENTIFIER: c_int = 11;
pub const wxSTC_SN_STRINGEOL: c_int = 12;
pub const wxSTC_SN_REGEXTAG: c_int = 13;
pub const wxSTC_SN_SIGNAL: c_int = 14;
pub const wxSTC_SN_USER: c_int = 19;
pub const wxSTC_AU3_DEFAULT: c_int = 0;
pub const wxSTC_AU3_COMMENT: c_int = 1;
pub const wxSTC_AU3_COMMENTBLOCK: c_int = 2;
pub const wxSTC_AU3_NUMBER: c_int = 3;
pub const wxSTC_AU3_FUNCTION: c_int = 4;
pub const wxSTC_AU3_KEYWORD: c_int = 5;
pub const wxSTC_AU3_MACRO: c_int = 6;
pub const wxSTC_AU3_STRING: c_int = 7;
pub const wxSTC_AU3_OPERATOR: c_int = 8;
pub const wxSTC_AU3_VARIABLE: c_int = 9;
pub const wxSTC_AU3_SENT: c_int = 10;
pub const wxSTC_AU3_PREPROCESSOR: c_int = 11;
pub const wxSTC_AU3_SPECIAL: c_int = 12;
pub const wxSTC_AU3_EXPAND: c_int = 13;
pub const wxSTC_AU3_COMOBJ: c_int = 14;
pub const wxSTC_AU3_UDF: c_int = 15;
pub const wxSTC_APDL_DEFAULT: c_int = 0;
pub const wxSTC_APDL_COMMENT: c_int = 1;
pub const wxSTC_APDL_COMMENTBLOCK: c_int = 2;
pub const wxSTC_APDL_NUMBER: c_int = 3;
pub const wxSTC_APDL_STRING: c_int = 4;
pub const wxSTC_APDL_OPERATOR: c_int = 5;
pub const wxSTC_APDL_WORD: c_int = 6;
pub const wxSTC_APDL_PROCESSOR: c_int = 7;
pub const wxSTC_APDL_COMMAND: c_int = 8;
pub const wxSTC_APDL_SLASHCOMMAND: c_int = 9;
pub const wxSTC_APDL_STARCOMMAND: c_int = 10;
pub const wxSTC_APDL_ARGUMENT: c_int = 11;
pub const wxSTC_APDL_FUNCTION: c_int = 12;
pub const wxSTC_SH_DEFAULT: c_int = 0;
pub const wxSTC_SH_ERROR: c_int = 1;
pub const wxSTC_SH_COMMENTLINE: c_int = 2;
pub const wxSTC_SH_NUMBER: c_int = 3;
pub const wxSTC_SH_WORD: c_int = 4;
pub const wxSTC_SH_STRING: c_int = 5;
pub const wxSTC_SH_CHARACTER: c_int = 6;
pub const wxSTC_SH_OPERATOR: c_int = 7;
pub const wxSTC_SH_IDENTIFIER: c_int = 8;
pub const wxSTC_SH_SCALAR: c_int = 9;
pub const wxSTC_SH_PARAM: c_int = 10;
pub const wxSTC_SH_BACKTICKS: c_int = 11;
pub const wxSTC_SH_HERE_DELIM: c_int = 12;
pub const wxSTC_SH_HERE_Q: c_int = 13;
pub const wxSTC_ASN1_DEFAULT: c_int = 0;
pub const wxSTC_ASN1_COMMENT: c_int = 1;
pub const wxSTC_ASN1_IDENTIFIER: c_int = 2;
pub const wxSTC_ASN1_STRING: c_int = 3;
pub const wxSTC_ASN1_OID: c_int = 4;
pub const wxSTC_ASN1_SCALAR: c_int = 5;
pub const wxSTC_ASN1_KEYWORD: c_int = 6;
pub const wxSTC_ASN1_ATTRIBUTE: c_int = 7;
pub const wxSTC_ASN1_DESCRIPTOR: c_int = 8;
pub const wxSTC_ASN1_TYPE: c_int = 9;
pub const wxSTC_ASN1_OPERATOR: c_int = 10;
pub const wxSTC_VHDL_DEFAULT: c_int = 0;
pub const wxSTC_VHDL_COMMENT: c_int = 1;
pub const wxSTC_VHDL_COMMENTLINEBANG: c_int = 2;
pub const wxSTC_VHDL_NUMBER: c_int = 3;
pub const wxSTC_VHDL_STRING: c_int = 4;
pub const wxSTC_VHDL_OPERATOR: c_int = 5;
pub const wxSTC_VHDL_IDENTIFIER: c_int = 6;
pub const wxSTC_VHDL_STRINGEOL: c_int = 7;
pub const wxSTC_VHDL_KEYWORD: c_int = 8;
pub const wxSTC_VHDL_STDOPERATOR: c_int = 9;
pub const wxSTC_VHDL_ATTRIBUTE: c_int = 10;
pub const wxSTC_VHDL_STDFUNCTION: c_int = 11;
pub const wxSTC_VHDL_STDPACKAGE: c_int = 12;
pub const wxSTC_VHDL_STDTYPE: c_int = 13;
pub const wxSTC_VHDL_USERWORD: c_int = 14;
pub const wxSTC_CAML_DEFAULT: c_int = 0;
pub const wxSTC_CAML_IDENTIFIER: c_int = 1;
pub const wxSTC_CAML_TAGNAME: c_int = 2;
pub const wxSTC_CAML_KEYWORD: c_int = 3;
pub const wxSTC_CAML_KEYWORD2: c_int = 4;
pub const wxSTC_CAML_KEYWORD3: c_int = 5;
pub const wxSTC_CAML_LINENUM: c_int = 6;
pub const wxSTC_CAML_OPERATOR: c_int = 7;
pub const wxSTC_CAML_NUMBER: c_int = 8;
pub const wxSTC_CAML_CHAR: c_int = 9;
pub const wxSTC_CAML_WHITE: c_int = 10;
pub const wxSTC_CAML_STRING: c_int = 11;
pub const wxSTC_CAML_COMMENT: c_int = 12;
pub const wxSTC_CAML_COMMENT1: c_int = 13;
pub const wxSTC_CAML_COMMENT2: c_int = 14;
pub const wxSTC_CAML_COMMENT3: c_int = 15;
pub const wxSTC_HA_DEFAULT: c_int = 0;
pub const wxSTC_HA_IDENTIFIER: c_int = 1;
pub const wxSTC_HA_KEYWORD: c_int = 2;
pub const wxSTC_HA_NUMBER: c_int = 3;
pub const wxSTC_HA_STRING: c_int = 4;
pub const wxSTC_HA_CHARACTER: c_int = 5;
pub const wxSTC_HA_CLASS: c_int = 6;
pub const wxSTC_HA_MODULE: c_int = 7;
pub const wxSTC_HA_CAPITAL: c_int = 8;
pub const wxSTC_HA_DATA: c_int = 9;
pub const wxSTC_HA_IMPORT: c_int = 10;
pub const wxSTC_HA_OPERATOR: c_int = 11;
pub const wxSTC_HA_INSTANCE: c_int = 12;
pub const wxSTC_HA_COMMENTLINE: c_int = 13;
pub const wxSTC_HA_COMMENTBLOCK: c_int = 14;
pub const wxSTC_HA_COMMENTBLOCK2: c_int = 15;
pub const wxSTC_HA_COMMENTBLOCK3: c_int = 16;
pub const wxSTC_T3_DEFAULT: c_int = 0;
pub const wxSTC_T3_X_DEFAULT: c_int = 1;
pub const wxSTC_T3_PREPROCESSOR: c_int = 2;
pub const wxSTC_T3_BLOCK_COMMENT: c_int = 3;
pub const wxSTC_T3_LINE_COMMENT: c_int = 4;
pub const wxSTC_T3_OPERATOR: c_int = 5;
pub const wxSTC_T3_KEYWORD: c_int = 6;
pub const wxSTC_T3_NUMBER: c_int = 7;
pub const wxSTC_T3_IDENTIFIER: c_int = 8;
pub const wxSTC_T3_S_STRING: c_int = 9;
pub const wxSTC_T3_D_STRING: c_int = 10;
pub const wxSTC_T3_X_STRING: c_int = 11;
pub const wxSTC_T3_LIB_DIRECTIVE: c_int = 12;
pub const wxSTC_T3_MSG_PARAM: c_int = 13;
pub const wxSTC_T3_HTML_TAG: c_int = 14;
pub const wxSTC_T3_HTML_DEFAULT: c_int = 15;
pub const wxSTC_T3_HTML_STRING: c_int = 16;
pub const wxSTC_T3_USER1: c_int = 17;
pub const wxSTC_T3_USER2: c_int = 18;
pub const wxSTC_T3_USER3: c_int = 19;
pub const wxSTC_T3_BRACE: c_int = 20;
pub const wxSTC_REBOL_DEFAULT: c_int = 0;
pub const wxSTC_REBOL_COMMENTLINE: c_int = 1;
pub const wxSTC_REBOL_COMMENTBLOCK: c_int = 2;
pub const wxSTC_REBOL_PREFACE: c_int = 3;
pub const wxSTC_REBOL_OPERATOR: c_int = 4;
pub const wxSTC_REBOL_CHARACTER: c_int = 5;
pub const wxSTC_REBOL_QUOTEDSTRING: c_int = 6;
pub const wxSTC_REBOL_BRACEDSTRING: c_int = 7;
pub const wxSTC_REBOL_NUMBER: c_int = 8;
pub const wxSTC_REBOL_PAIR: c_int = 9;
pub const wxSTC_REBOL_TUPLE: c_int = 10;
pub const wxSTC_REBOL_BINARY: c_int = 11;
pub const wxSTC_REBOL_MONEY: c_int = 12;
pub const wxSTC_REBOL_ISSUE: c_int = 13;
pub const wxSTC_REBOL_TAG: c_int = 14;
pub const wxSTC_REBOL_FILE: c_int = 15;
pub const wxSTC_REBOL_EMAIL: c_int = 16;
pub const wxSTC_REBOL_URL: c_int = 17;
pub const wxSTC_REBOL_DATE: c_int = 18;
pub const wxSTC_REBOL_TIME: c_int = 19;
pub const wxSTC_REBOL_IDENTIFIER: c_int = 20;
pub const wxSTC_REBOL_WORD: c_int = 21;
pub const wxSTC_REBOL_WORD2: c_int = 22;
pub const wxSTC_REBOL_WORD3: c_int = 23;
pub const wxSTC_REBOL_WORD4: c_int = 24;
pub const wxSTC_REBOL_WORD5: c_int = 25;
pub const wxSTC_REBOL_WORD6: c_int = 26;
pub const wxSTC_REBOL_WORD7: c_int = 27;
pub const wxSTC_REBOL_WORD8: c_int = 28;
pub const wxSTC_SQL_DEFAULT: c_int = 0;
pub const wxSTC_SQL_COMMENT: c_int = 1;
pub const wxSTC_SQL_COMMENTLINE: c_int = 2;
pub const wxSTC_SQL_COMMENTDOC: c_int = 3;
pub const wxSTC_SQL_NUMBER: c_int = 4;
pub const wxSTC_SQL_WORD: c_int = 5;
pub const wxSTC_SQL_STRING: c_int = 6;
pub const wxSTC_SQL_CHARACTER: c_int = 7;
pub const wxSTC_SQL_SQLPLUS: c_int = 8;
pub const wxSTC_SQL_SQLPLUS_PROMPT: c_int = 9;
pub const wxSTC_SQL_OPERATOR: c_int = 10;
pub const wxSTC_SQL_IDENTIFIER: c_int = 11;
pub const wxSTC_SQL_SQLPLUS_COMMENT: c_int = 13;
pub const wxSTC_SQL_COMMENTLINEDOC: c_int = 15;
pub const wxSTC_SQL_WORD2: c_int = 16;
pub const wxSTC_SQL_COMMENTDOCKEYWORD: c_int = 17;
pub const wxSTC_SQL_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const wxSTC_SQL_USER1: c_int = 19;
pub const wxSTC_SQL_USER2: c_int = 20;
pub const wxSTC_SQL_USER3: c_int = 21;
pub const wxSTC_SQL_USER4: c_int = 22;
pub const wxSTC_SQL_QUOTEDIDENTIFIER: c_int = 23;
pub const wxSTC_ST_DEFAULT: c_int = 0;
pub const wxSTC_ST_STRING: c_int = 1;
pub const wxSTC_ST_NUMBER: c_int = 2;
pub const wxSTC_ST_COMMENT: c_int = 3;
pub const wxSTC_ST_SYMBOL: c_int = 4;
pub const wxSTC_ST_BINARY: c_int = 5;
pub const wxSTC_ST_BOOL: c_int = 6;
pub const wxSTC_ST_SELF: c_int = 7;
pub const wxSTC_ST_SUPER: c_int = 8;
pub const wxSTC_ST_NIL: c_int = 9;
pub const wxSTC_ST_GLOBAL: c_int = 10;
pub const wxSTC_ST_RETURN: c_int = 11;
pub const wxSTC_ST_SPECIAL: c_int = 12;
pub const wxSTC_ST_KWSEND: c_int = 13;
pub const wxSTC_ST_ASSIGN: c_int = 14;
pub const wxSTC_ST_CHARACTER: c_int = 15;
pub const wxSTC_ST_SPEC_SEL: c_int = 16;
pub const wxSTC_FS_DEFAULT: c_int = 0;
pub const wxSTC_FS_COMMENT: c_int = 1;
pub const wxSTC_FS_COMMENTLINE: c_int = 2;
pub const wxSTC_FS_COMMENTDOC: c_int = 3;
pub const wxSTC_FS_COMMENTLINEDOC: c_int = 4;
pub const wxSTC_FS_COMMENTDOCKEYWORD: c_int = 5;
pub const wxSTC_FS_COMMENTDOCKEYWORDERROR: c_int = 6;
pub const wxSTC_FS_KEYWORD: c_int = 7;
pub const wxSTC_FS_KEYWORD2: c_int = 8;
pub const wxSTC_FS_KEYWORD3: c_int = 9;
pub const wxSTC_FS_KEYWORD4: c_int = 10;
pub const wxSTC_FS_NUMBER: c_int = 11;
pub const wxSTC_FS_STRING: c_int = 12;
pub const wxSTC_FS_PREPROCESSOR: c_int = 13;
pub const wxSTC_FS_OPERATOR: c_int = 14;
pub const wxSTC_FS_IDENTIFIER: c_int = 15;
pub const wxSTC_FS_DATE: c_int = 16;
pub const wxSTC_FS_STRINGEOL: c_int = 17;
pub const wxSTC_FS_CONSTANT: c_int = 18;
pub const wxSTC_FS_WORDOPERATOR: c_int = 19;
pub const wxSTC_FS_DISABLEDCODE: c_int = 20;
pub const wxSTC_FS_DEFAULT_C: c_int = 21;
pub const wxSTC_FS_COMMENTDOC_C: c_int = 22;
pub const wxSTC_FS_COMMENTLINEDOC_C: c_int = 23;
pub const wxSTC_FS_KEYWORD_C: c_int = 24;
pub const wxSTC_FS_KEYWORD2_C: c_int = 25;
pub const wxSTC_FS_NUMBER_C: c_int = 26;
pub const wxSTC_FS_STRING_C: c_int = 27;
pub const wxSTC_FS_PREPROCESSOR_C: c_int = 28;
pub const wxSTC_FS_OPERATOR_C: c_int = 29;
pub const wxSTC_FS_IDENTIFIER_C: c_int = 30;
pub const wxSTC_FS_STRINGEOL_C: c_int = 31;
pub const wxSTC_CSOUND_DEFAULT: c_int = 0;
pub const wxSTC_CSOUND_COMMENT: c_int = 1;
pub const wxSTC_CSOUND_NUMBER: c_int = 2;
pub const wxSTC_CSOUND_OPERATOR: c_int = 3;
pub const wxSTC_CSOUND_INSTR: c_int = 4;
pub const wxSTC_CSOUND_IDENTIFIER: c_int = 5;
pub const wxSTC_CSOUND_OPCODE: c_int = 6;
pub const wxSTC_CSOUND_HEADERSTMT: c_int = 7;
pub const wxSTC_CSOUND_USERKEYWORD: c_int = 8;
pub const wxSTC_CSOUND_COMMENTBLOCK: c_int = 9;
pub const wxSTC_CSOUND_PARAM: c_int = 10;
pub const wxSTC_CSOUND_ARATE_VAR: c_int = 11;
pub const wxSTC_CSOUND_KRATE_VAR: c_int = 12;
pub const wxSTC_CSOUND_IRATE_VAR: c_int = 13;
pub const wxSTC_CSOUND_GLOBAL_VAR: c_int = 14;
pub const wxSTC_CSOUND_STRINGEOL: c_int = 15;
pub const wxSTC_INNO_DEFAULT: c_int = 0;
pub const wxSTC_INNO_COMMENT: c_int = 1;
pub const wxSTC_INNO_KEYWORD: c_int = 2;
pub const wxSTC_INNO_PARAMETER: c_int = 3;
pub const wxSTC_INNO_SECTION: c_int = 4;
pub const wxSTC_INNO_PREPROC: c_int = 5;
pub const wxSTC_INNO_INLINE_EXPANSION: c_int = 6;
pub const wxSTC_INNO_COMMENT_PASCAL: c_int = 7;
pub const wxSTC_INNO_KEYWORD_PASCAL: c_int = 8;
pub const wxSTC_INNO_KEYWORD_USER: c_int = 9;
pub const wxSTC_INNO_STRING_DOUBLE: c_int = 10;
pub const wxSTC_INNO_STRING_SINGLE: c_int = 11;
pub const wxSTC_INNO_IDENTIFIER: c_int = 12;
pub const wxSTC_OPAL_SPACE: c_int = 0;
pub const wxSTC_OPAL_COMMENT_BLOCK: c_int = 1;
pub const wxSTC_OPAL_COMMENT_LINE: c_int = 2;
pub const wxSTC_OPAL_INTEGER: c_int = 3;
pub const wxSTC_OPAL_KEYWORD: c_int = 4;
pub const wxSTC_OPAL_SORT: c_int = 5;
pub const wxSTC_OPAL_STRING: c_int = 6;
pub const wxSTC_OPAL_PAR: c_int = 7;
pub const wxSTC_OPAL_BOOL_CONST: c_int = 8;
pub const wxSTC_OPAL_DEFAULT: c_int = 32;
pub const wxSTC_SPICE_DEFAULT: c_int = 0;
pub const wxSTC_SPICE_IDENTIFIER: c_int = 1;
pub const wxSTC_SPICE_KEYWORD: c_int = 2;
pub const wxSTC_SPICE_KEYWORD2: c_int = 3;
pub const wxSTC_SPICE_KEYWORD3: c_int = 4;
pub const wxSTC_SPICE_NUMBER: c_int = 5;
pub const wxSTC_SPICE_DELIMITER: c_int = 6;
pub const wxSTC_SPICE_VALUE: c_int = 7;
pub const wxSTC_SPICE_COMMENTLINE: c_int = 8;
pub const wxSTC_CMAKE_DEFAULT: c_int = 0;
pub const wxSTC_CMAKE_COMMENT: c_int = 1;
pub const wxSTC_CMAKE_STRINGDQ: c_int = 2;
pub const wxSTC_CMAKE_STRINGLQ: c_int = 3;
pub const wxSTC_CMAKE_STRINGRQ: c_int = 4;
pub const wxSTC_CMAKE_COMMANDS: c_int = 5;
pub const wxSTC_CMAKE_PARAMETERS: c_int = 6;
pub const wxSTC_CMAKE_VARIABLE: c_int = 7;
pub const wxSTC_CMAKE_USERDEFINED: c_int = 8;
pub const wxSTC_CMAKE_WHILEDEF: c_int = 9;
pub const wxSTC_CMAKE_FOREACHDEF: c_int = 10;
pub const wxSTC_CMAKE_IFDEFINEDEF: c_int = 11;
pub const wxSTC_CMAKE_MACRODEF: c_int = 12;
pub const wxSTC_CMAKE_STRINGVAR: c_int = 13;
pub const wxSTC_CMAKE_NUMBER: c_int = 14;
pub const wxSTC_GAP_DEFAULT: c_int = 0;
pub const wxSTC_GAP_IDENTIFIER: c_int = 1;
pub const wxSTC_GAP_KEYWORD: c_int = 2;
pub const wxSTC_GAP_KEYWORD2: c_int = 3;
pub const wxSTC_GAP_KEYWORD3: c_int = 4;
pub const wxSTC_GAP_KEYWORD4: c_int = 5;
pub const wxSTC_GAP_STRING: c_int = 6;
pub const wxSTC_GAP_CHAR: c_int = 7;
pub const wxSTC_GAP_OPERATOR: c_int = 8;
pub const wxSTC_GAP_COMMENT: c_int = 9;
pub const wxSTC_GAP_NUMBER: c_int = 10;
pub const wxSTC_GAP_STRINGEOL: c_int = 11;
pub const wxSTC_PLM_DEFAULT: c_int = 0;
pub const wxSTC_PLM_COMMENT: c_int = 1;
pub const wxSTC_PLM_STRING: c_int = 2;
pub const wxSTC_PLM_NUMBER: c_int = 3;
pub const wxSTC_PLM_IDENTIFIER: c_int = 4;
pub const wxSTC_PLM_OPERATOR: c_int = 5;
pub const wxSTC_PLM_CONTROL: c_int = 6;
pub const wxSTC_PLM_KEYWORD: c_int = 7;
pub const wxSTC_4GL_DEFAULT: c_int = 0;
pub const wxSTC_4GL_NUMBER: c_int = 1;
pub const wxSTC_4GL_WORD: c_int = 2;
pub const wxSTC_4GL_STRING: c_int = 3;
pub const wxSTC_4GL_CHARACTER: c_int = 4;
pub const wxSTC_4GL_PREPROCESSOR: c_int = 5;
pub const wxSTC_4GL_OPERATOR: c_int = 6;
pub const wxSTC_4GL_IDENTIFIER: c_int = 7;
pub const wxSTC_4GL_BLOCK: c_int = 8;
pub const wxSTC_4GL_END: c_int = 9;
pub const wxSTC_4GL_COMMENT1: c_int = 10;
pub const wxSTC_4GL_COMMENT2: c_int = 11;
pub const wxSTC_4GL_COMMENT3: c_int = 12;
pub const wxSTC_4GL_COMMENT4: c_int = 13;
pub const wxSTC_4GL_COMMENT5: c_int = 14;
pub const wxSTC_4GL_COMMENT6: c_int = 15;
pub const wxSTC_4GL_DEFAULT_: c_int = 16;
pub const wxSTC_4GL_NUMBER_: c_int = 17;
pub const wxSTC_4GL_WORD_: c_int = 18;
pub const wxSTC_4GL_STRING_: c_int = 19;
pub const wxSTC_4GL_CHARACTER_: c_int = 20;
pub const wxSTC_4GL_PREPROCESSOR_: c_int = 21;
pub const wxSTC_4GL_OPERATOR_: c_int = 22;
pub const wxSTC_4GL_IDENTIFIER_: c_int = 23;
pub const wxSTC_4GL_BLOCK_: c_int = 24;
pub const wxSTC_4GL_END_: c_int = 25;
pub const wxSTC_4GL_COMMENT1_: c_int = 26;
pub const wxSTC_4GL_COMMENT2_: c_int = 27;
pub const wxSTC_4GL_COMMENT3_: c_int = 28;
pub const wxSTC_4GL_COMMENT4_: c_int = 29;
pub const wxSTC_4GL_COMMENT5_: c_int = 30;
pub const wxSTC_4GL_COMMENT6_: c_int = 31;
pub const wxSTC_ABAQUS_DEFAULT: c_int = 0;
pub const wxSTC_ABAQUS_COMMENT: c_int = 1;
pub const wxSTC_ABAQUS_COMMENTBLOCK: c_int = 2;
pub const wxSTC_ABAQUS_NUMBER: c_int = 3;
pub const wxSTC_ABAQUS_STRING: c_int = 4;
pub const wxSTC_ABAQUS_OPERATOR: c_int = 5;
pub const wxSTC_ABAQUS_WORD: c_int = 6;
pub const wxSTC_ABAQUS_PROCESSOR: c_int = 7;
pub const wxSTC_ABAQUS_COMMAND: c_int = 8;
pub const wxSTC_ABAQUS_SLASHCOMMAND: c_int = 9;
pub const wxSTC_ABAQUS_STARCOMMAND: c_int = 10;
pub const wxSTC_ABAQUS_ARGUMENT: c_int = 11;
pub const wxSTC_ABAQUS_FUNCTION: c_int = 12;
pub const wxSTC_ASY_DEFAULT: c_int = 0;
pub const wxSTC_ASY_COMMENT: c_int = 1;
pub const wxSTC_ASY_COMMENTLINE: c_int = 2;
pub const wxSTC_ASY_NUMBER: c_int = 3;
pub const wxSTC_ASY_WORD: c_int = 4;
pub const wxSTC_ASY_STRING: c_int = 5;
pub const wxSTC_ASY_CHARACTER: c_int = 6;
pub const wxSTC_ASY_OPERATOR: c_int = 7;
pub const wxSTC_ASY_IDENTIFIER: c_int = 8;
pub const wxSTC_ASY_STRINGEOL: c_int = 9;
pub const wxSTC_ASY_COMMENTLINEDOC: c_int = 10;
pub const wxSTC_ASY_WORD2: c_int = 11;
pub const wxSTC_R_DEFAULT: c_int = 0;
pub const wxSTC_R_COMMENT: c_int = 1;
pub const wxSTC_R_KWORD: c_int = 2;
pub const wxSTC_R_BASEKWORD: c_int = 3;
pub const wxSTC_R_OTHERKWORD: c_int = 4;
pub const wxSTC_R_NUMBER: c_int = 5;
pub const wxSTC_R_STRING: c_int = 6;
pub const wxSTC_R_STRING2: c_int = 7;
pub const wxSTC_R_OPERATOR: c_int = 8;
pub const wxSTC_R_IDENTIFIER: c_int = 9;
pub const wxSTC_R_INFIX: c_int = 10;
pub const wxSTC_R_INFIXEOL: c_int = 11;
pub const wxSTC_MAGIK_DEFAULT: c_int = 0;
pub const wxSTC_MAGIK_COMMENT: c_int = 1;
pub const wxSTC_MAGIK_HYPER_COMMENT: c_int = 16;
pub const wxSTC_MAGIK_STRING: c_int = 2;
pub const wxSTC_MAGIK_CHARACTER: c_int = 3;
pub const wxSTC_MAGIK_NUMBER: c_int = 4;
pub const wxSTC_MAGIK_IDENTIFIER: c_int = 5;
pub const wxSTC_MAGIK_OPERATOR: c_int = 6;
pub const wxSTC_MAGIK_FLOW: c_int = 7;
pub const wxSTC_MAGIK_CONTAINER: c_int = 8;
pub const wxSTC_MAGIK_BRACKET_BLOCK: c_int = 9;
pub const wxSTC_MAGIK_BRACE_BLOCK: c_int = 10;
pub const wxSTC_MAGIK_SQBRACKET_BLOCK: c_int = 11;
pub const wxSTC_MAGIK_UNKNOWN_KEYWORD: c_int = 12;
pub const wxSTC_MAGIK_KEYWORD: c_int = 13;
pub const wxSTC_MAGIK_PRAGMA: c_int = 14;
pub const wxSTC_MAGIK_SYMBOL: c_int = 15;
pub const wxSTC_POWERSHELL_DEFAULT: c_int = 0;
pub const wxSTC_POWERSHELL_COMMENT: c_int = 1;
pub const wxSTC_POWERSHELL_STRING: c_int = 2;
pub const wxSTC_POWERSHELL_CHARACTER: c_int = 3;
pub const wxSTC_POWERSHELL_NUMBER: c_int = 4;
pub const wxSTC_POWERSHELL_VARIABLE: c_int = 5;
pub const wxSTC_POWERSHELL_OPERATOR: c_int = 6;
pub const wxSTC_POWERSHELL_IDENTIFIER: c_int = 7;
pub const wxSTC_POWERSHELL_KEYWORD: c_int = 8;
pub const wxSTC_POWERSHELL_CMDLET: c_int = 9;
pub const wxSTC_POWERSHELL_ALIAS: c_int = 10;
pub const wxSTC_POWERSHELL_FUNCTION: c_int = 11;
pub const wxSTC_POWERSHELL_USER1: c_int = 12;
pub const wxSTC_POWERSHELL_COMMENTSTREAM: c_int = 13;
pub const wxSTC_MYSQL_DEFAULT: c_int = 0;
pub const wxSTC_MYSQL_COMMENT: c_int = 1;
pub const wxSTC_MYSQL_COMMENTLINE: c_int = 2;
pub const wxSTC_MYSQL_VARIABLE: c_int = 3;
pub const wxSTC_MYSQL_SYSTEMVARIABLE: c_int = 4;
pub const wxSTC_MYSQL_KNOWNSYSTEMVARIABLE: c_int = 5;
pub const wxSTC_MYSQL_NUMBER: c_int = 6;
pub const wxSTC_MYSQL_MAJORKEYWORD: c_int = 7;
pub const wxSTC_MYSQL_KEYWORD: c_int = 8;
pub const wxSTC_MYSQL_DATABASEOBJECT: c_int = 9;
pub const wxSTC_MYSQL_PROCEDUREKEYWORD: c_int = 10;
pub const wxSTC_MYSQL_STRING: c_int = 11;
pub const wxSTC_MYSQL_SQSTRING: c_int = 12;
pub const wxSTC_MYSQL_DQSTRING: c_int = 13;
pub const wxSTC_MYSQL_OPERATOR: c_int = 14;
pub const wxSTC_MYSQL_FUNCTION: c_int = 15;
pub const wxSTC_MYSQL_IDENTIFIER: c_int = 16;
pub const wxSTC_MYSQL_QUOTEDIDENTIFIER: c_int = 17;
pub const wxSTC_MYSQL_USER1: c_int = 18;
pub const wxSTC_MYSQL_USER2: c_int = 19;
pub const wxSTC_MYSQL_USER3: c_int = 20;
pub const wxSTC_MYSQL_HIDDENCOMMAND: c_int = 21;
pub const wxSTC_PO_DEFAULT: c_int = 0;
pub const wxSTC_PO_COMMENT: c_int = 1;
pub const wxSTC_PO_MSGID: c_int = 2;
pub const wxSTC_PO_MSGID_TEXT: c_int = 3;
pub const wxSTC_PO_MSGSTR: c_int = 4;
pub const wxSTC_PO_MSGSTR_TEXT: c_int = 5;
pub const wxSTC_PO_MSGCTXT: c_int = 6;
pub const wxSTC_PO_MSGCTXT_TEXT: c_int = 7;
pub const wxSTC_PO_FUZZY: c_int = 8;
pub const wxSTC_PAS_DEFAULT: c_int = 0;
pub const wxSTC_PAS_IDENTIFIER: c_int = 1;
pub const wxSTC_PAS_COMMENT: c_int = 2;
pub const wxSTC_PAS_COMMENT2: c_int = 3;
pub const wxSTC_PAS_COMMENTLINE: c_int = 4;
pub const wxSTC_PAS_PREPROCESSOR: c_int = 5;
pub const wxSTC_PAS_PREPROCESSOR2: c_int = 6;
pub const wxSTC_PAS_NUMBER: c_int = 7;
pub const wxSTC_PAS_HEXNUMBER: c_int = 8;
pub const wxSTC_PAS_WORD: c_int = 9;
pub const wxSTC_PAS_STRING: c_int = 10;
pub const wxSTC_PAS_STRINGEOL: c_int = 11;
pub const wxSTC_PAS_CHARACTER: c_int = 12;
pub const wxSTC_PAS_OPERATOR: c_int = 13;
pub const wxSTC_PAS_ASM: c_int = 14;
pub const wxSTC_SORCUS_DEFAULT: c_int = 0;
pub const wxSTC_SORCUS_COMMAND: c_int = 1;
pub const wxSTC_SORCUS_PARAMETER: c_int = 2;
pub const wxSTC_SORCUS_COMMENTLINE: c_int = 3;
pub const wxSTC_SORCUS_STRING: c_int = 4;
pub const wxSTC_SORCUS_STRINGEOL: c_int = 5;
pub const wxSTC_SORCUS_IDENTIFIER: c_int = 6;
pub const wxSTC_SORCUS_OPERATOR: c_int = 7;
pub const wxSTC_SORCUS_NUMBER: c_int = 8;
pub const wxSTC_SORCUS_CONSTANT: c_int = 9;
pub const wxSTC_POWERPRO_DEFAULT: c_int = 0;
pub const wxSTC_POWERPRO_COMMENTBLOCK: c_int = 1;
pub const wxSTC_POWERPRO_COMMENTLINE: c_int = 2;
pub const wxSTC_POWERPRO_NUMBER: c_int = 3;
pub const wxSTC_POWERPRO_WORD: c_int = 4;
pub const wxSTC_POWERPRO_WORD2: c_int = 5;
pub const wxSTC_POWERPRO_WORD3: c_int = 6;
pub const wxSTC_POWERPRO_WORD4: c_int = 7;
pub const wxSTC_POWERPRO_DOUBLEQUOTEDSTRING: c_int = 8;
pub const wxSTC_POWERPRO_SINGLEQUOTEDSTRING: c_int = 9;
pub const wxSTC_POWERPRO_LINECONTINUE: c_int = 10;
pub const wxSTC_POWERPRO_OPERATOR: c_int = 11;
pub const wxSTC_POWERPRO_IDENTIFIER: c_int = 12;
pub const wxSTC_POWERPRO_STRINGEOL: c_int = 13;
pub const wxSTC_POWERPRO_VERBATIM: c_int = 14;
pub const wxSTC_POWERPRO_ALTQUOTE: c_int = 15;
pub const wxSTC_POWERPRO_FUNCTION: c_int = 16;
pub const wxSTC_SML_DEFAULT: c_int = 0;
pub const wxSTC_SML_IDENTIFIER: c_int = 1;
pub const wxSTC_SML_TAGNAME: c_int = 2;
pub const wxSTC_SML_KEYWORD: c_int = 3;
pub const wxSTC_SML_KEYWORD2: c_int = 4;
pub const wxSTC_SML_KEYWORD3: c_int = 5;
pub const wxSTC_SML_LINENUM: c_int = 6;
pub const wxSTC_SML_OPERATOR: c_int = 7;
pub const wxSTC_SML_NUMBER: c_int = 8;
pub const wxSTC_SML_CHAR: c_int = 9;
pub const wxSTC_SML_STRING: c_int = 11;
pub const wxSTC_SML_COMMENT: c_int = 12;
pub const wxSTC_SML_COMMENT1: c_int = 13;
pub const wxSTC_SML_COMMENT2: c_int = 14;
pub const wxSTC_SML_COMMENT3: c_int = 15;
pub const wxSTC_MARKDOWN_DEFAULT: c_int = 0;
pub const wxSTC_MARKDOWN_LINE_BEGIN: c_int = 1;
pub const wxSTC_MARKDOWN_STRONG1: c_int = 2;
pub const wxSTC_MARKDOWN_STRONG2: c_int = 3;
pub const wxSTC_MARKDOWN_EM1: c_int = 4;
pub const wxSTC_MARKDOWN_EM2: c_int = 5;
pub const wxSTC_MARKDOWN_HEADER1: c_int = 6;
pub const wxSTC_MARKDOWN_HEADER2: c_int = 7;
pub const wxSTC_MARKDOWN_HEADER3: c_int = 8;
pub const wxSTC_MARKDOWN_HEADER4: c_int = 9;
pub const wxSTC_MARKDOWN_HEADER5: c_int = 10;
pub const wxSTC_MARKDOWN_HEADER6: c_int = 11;
pub const wxSTC_MARKDOWN_PRECHAR: c_int = 12;
pub const wxSTC_MARKDOWN_ULIST_ITEM: c_int = 13;
pub const wxSTC_MARKDOWN_OLIST_ITEM: c_int = 14;
pub const wxSTC_MARKDOWN_BLOCKQUOTE: c_int = 15;
pub const wxSTC_MARKDOWN_STRIKEOUT: c_int = 16;
pub const wxSTC_MARKDOWN_HRULE: c_int = 17;
pub const wxSTC_MARKDOWN_LINK: c_int = 18;
pub const wxSTC_MARKDOWN_CODE: c_int = 19;
pub const wxSTC_MARKDOWN_CODE2: c_int = 20;
pub const wxSTC_MARKDOWN_CODEBK: c_int = 21;
pub const wxSTC_TXT2TAGS_DEFAULT: c_int = 0;
pub const wxSTC_TXT2TAGS_LINE_BEGIN: c_int = 1;
pub const wxSTC_TXT2TAGS_STRONG1: c_int = 2;
pub const wxSTC_TXT2TAGS_STRONG2: c_int = 3;
pub const wxSTC_TXT2TAGS_EM1: c_int = 4;
pub const wxSTC_TXT2TAGS_EM2: c_int = 5;
pub const wxSTC_TXT2TAGS_HEADER1: c_int = 6;
pub const wxSTC_TXT2TAGS_HEADER2: c_int = 7;
pub const wxSTC_TXT2TAGS_HEADER3: c_int = 8;
pub const wxSTC_TXT2TAGS_HEADER4: c_int = 9;
pub const wxSTC_TXT2TAGS_HEADER5: c_int = 10;
pub const wxSTC_TXT2TAGS_HEADER6: c_int = 11;
pub const wxSTC_TXT2TAGS_PRECHAR: c_int = 12;
pub const wxSTC_TXT2TAGS_ULIST_ITEM: c_int = 13;
pub const wxSTC_TXT2TAGS_OLIST_ITEM: c_int = 14;
pub const wxSTC_TXT2TAGS_BLOCKQUOTE: c_int = 15;
pub const wxSTC_TXT2TAGS_STRIKEOUT: c_int = 16;
pub const wxSTC_TXT2TAGS_HRULE: c_int = 17;
pub const wxSTC_TXT2TAGS_LINK: c_int = 18;
pub const wxSTC_TXT2TAGS_CODE: c_int = 19;
pub const wxSTC_TXT2TAGS_CODE2: c_int = 20;
pub const wxSTC_TXT2TAGS_CODEBK: c_int = 21;
pub const wxSTC_TXT2TAGS_COMMENT: c_int = 22;
pub const wxSTC_TXT2TAGS_OPTION: c_int = 23;
pub const wxSTC_TXT2TAGS_PREPROC: c_int = 24;
pub const wxSTC_TXT2TAGS_POSTPROC: c_int = 25;
pub const wxSTC_A68K_DEFAULT: c_int = 0;
pub const wxSTC_A68K_COMMENT: c_int = 1;
pub const wxSTC_A68K_NUMBER_DEC: c_int = 2;
pub const wxSTC_A68K_NUMBER_BIN: c_int = 3;
pub const wxSTC_A68K_NUMBER_HEX: c_int = 4;
pub const wxSTC_A68K_STRING1: c_int = 5;
pub const wxSTC_A68K_OPERATOR: c_int = 6;
pub const wxSTC_A68K_CPUINSTRUCTION: c_int = 7;
pub const wxSTC_A68K_EXTINSTRUCTION: c_int = 8;
pub const wxSTC_A68K_REGISTER: c_int = 9;
pub const wxSTC_A68K_DIRECTIVE: c_int = 10;
pub const wxSTC_A68K_MACRO_ARG: c_int = 11;
pub const wxSTC_A68K_LABEL: c_int = 12;
pub const wxSTC_A68K_STRING2: c_int = 13;
pub const wxSTC_A68K_IDENTIFIER: c_int = 14;
pub const wxSTC_A68K_MACRO_DECLARATION: c_int = 15;
pub const wxSTC_A68K_COMMENT_WORD: c_int = 16;
pub const wxSTC_A68K_COMMENT_SPECIAL: c_int = 17;
pub const wxSTC_A68K_COMMENT_DOXYGEN: c_int = 18;
pub const wxSTC_MODULA_DEFAULT: c_int = 0;
pub const wxSTC_MODULA_COMMENT: c_int = 1;
pub const wxSTC_MODULA_DOXYCOMM: c_int = 2;
pub const wxSTC_MODULA_DOXYKEY: c_int = 3;
pub const wxSTC_MODULA_KEYWORD: c_int = 4;
pub const wxSTC_MODULA_RESERVED: c_int = 5;
pub const wxSTC_MODULA_NUMBER: c_int = 6;
pub const wxSTC_MODULA_BASENUM: c_int = 7;
pub const wxSTC_MODULA_FLOAT: c_int = 8;
pub const wxSTC_MODULA_STRING: c_int = 9;
pub const wxSTC_MODULA_STRSPEC: c_int = 10;
pub const wxSTC_MODULA_CHAR: c_int = 11;
pub const wxSTC_MODULA_CHARSPEC: c_int = 12;
pub const wxSTC_MODULA_PROC: c_int = 13;
pub const wxSTC_MODULA_PRAGMA: c_int = 14;
pub const wxSTC_MODULA_PRGKEY: c_int = 15;
pub const wxSTC_MODULA_OPERATOR: c_int = 16;
pub const wxSTC_MODULA_BADSTR: c_int = 17;
pub const wxSTC_COFFEESCRIPT_DEFAULT: c_int = 0;
pub const wxSTC_COFFEESCRIPT_COMMENT: c_int = 1;
pub const wxSTC_COFFEESCRIPT_COMMENTLINE: c_int = 2;
pub const wxSTC_COFFEESCRIPT_COMMENTDOC: c_int = 3;
pub const wxSTC_COFFEESCRIPT_NUMBER: c_int = 4;
pub const wxSTC_COFFEESCRIPT_WORD: c_int = 5;
pub const wxSTC_COFFEESCRIPT_STRING: c_int = 6;
pub const wxSTC_COFFEESCRIPT_CHARACTER: c_int = 7;
pub const wxSTC_COFFEESCRIPT_UUID: c_int = 8;
pub const wxSTC_COFFEESCRIPT_PREPROCESSOR: c_int = 9;
pub const wxSTC_COFFEESCRIPT_OPERATOR: c_int = 10;
pub const wxSTC_COFFEESCRIPT_IDENTIFIER: c_int = 11;
pub const wxSTC_COFFEESCRIPT_STRINGEOL: c_int = 12;
pub const wxSTC_COFFEESCRIPT_VERBATIM: c_int = 13;
pub const wxSTC_COFFEESCRIPT_REGEX: c_int = 14;
pub const wxSTC_COFFEESCRIPT_COMMENTLINEDOC: c_int = 15;
pub const wxSTC_COFFEESCRIPT_WORD2: c_int = 16;
pub const wxSTC_COFFEESCRIPT_COMMENTDOCKEYWORD: c_int = 17;
pub const wxSTC_COFFEESCRIPT_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const wxSTC_COFFEESCRIPT_GLOBALCLASS: c_int = 19;
pub const wxSTC_COFFEESCRIPT_STRINGRAW: c_int = 20;
pub const wxSTC_COFFEESCRIPT_TRIPLEVERBATIM: c_int = 21;
pub const wxSTC_COFFEESCRIPT_HASHQUOTEDSTRING: c_int = 22;
pub const wxSTC_COFFEESCRIPT_COMMENTBLOCK: c_int = 22;
pub const wxSTC_COFFEESCRIPT_VERBOSE_REGEX: c_int = 23;
pub const wxSTC_COFFEESCRIPT_VERBOSE_REGEX_COMMENT: c_int = 24;
pub const wxSTC_AVS_DEFAULT: c_int = 0;
pub const wxSTC_AVS_COMMENTBLOCK: c_int = 1;
pub const wxSTC_AVS_COMMENTBLOCKN: c_int = 2;
pub const wxSTC_AVS_COMMENTLINE: c_int = 3;
pub const wxSTC_AVS_NUMBER: c_int = 4;
pub const wxSTC_AVS_OPERATOR: c_int = 5;
pub const wxSTC_AVS_IDENTIFIER: c_int = 6;
pub const wxSTC_AVS_STRING: c_int = 7;
pub const wxSTC_AVS_TRIPLESTRING: c_int = 8;
pub const wxSTC_AVS_KEYWORD: c_int = 9;
pub const wxSTC_AVS_FILTER: c_int = 10;
pub const wxSTC_AVS_PLUGIN: c_int = 11;
pub const wxSTC_AVS_FUNCTION: c_int = 12;
pub const wxSTC_AVS_CLIPPROP: c_int = 13;
pub const wxSTC_AVS_USERDFN: c_int = 14;
pub const wxSTC_ECL_DEFAULT: c_int = 0;
pub const wxSTC_ECL_COMMENT: c_int = 1;
pub const wxSTC_ECL_COMMENTLINE: c_int = 2;
pub const wxSTC_ECL_NUMBER: c_int = 3;
pub const wxSTC_ECL_STRING: c_int = 4;
pub const wxSTC_ECL_WORD0: c_int = 5;
pub const wxSTC_ECL_OPERATOR: c_int = 6;
pub const wxSTC_ECL_CHARACTER: c_int = 7;
pub const wxSTC_ECL_UUID: c_int = 8;
pub const wxSTC_ECL_PREPROCESSOR: c_int = 9;
pub const wxSTC_ECL_UNKNOWN: c_int = 10;
pub const wxSTC_ECL_IDENTIFIER: c_int = 11;
pub const wxSTC_ECL_STRINGEOL: c_int = 12;
pub const wxSTC_ECL_VERBATIM: c_int = 13;
pub const wxSTC_ECL_REGEX: c_int = 14;
pub const wxSTC_ECL_COMMENTLINEDOC: c_int = 15;
pub const wxSTC_ECL_WORD1: c_int = 16;
pub const wxSTC_ECL_COMMENTDOCKEYWORD: c_int = 17;
pub const wxSTC_ECL_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const wxSTC_ECL_WORD2: c_int = 19;
pub const wxSTC_ECL_WORD3: c_int = 20;
pub const wxSTC_ECL_WORD4: c_int = 21;
pub const wxSTC_ECL_WORD5: c_int = 22;
pub const wxSTC_ECL_COMMENTDOC: c_int = 23;
pub const wxSTC_ECL_ADDED: c_int = 24;
pub const wxSTC_ECL_DELETED: c_int = 25;
pub const wxSTC_ECL_CHANGED: c_int = 26;
pub const wxSTC_ECL_MOVED: c_int = 27;
pub const wxSTC_OSCRIPT_DEFAULT: c_int = 0;
pub const wxSTC_OSCRIPT_LINE_COMMENT: c_int = 1;
pub const wxSTC_OSCRIPT_BLOCK_COMMENT: c_int = 2;
pub const wxSTC_OSCRIPT_DOC_COMMENT: c_int = 3;
pub const wxSTC_OSCRIPT_PREPROCESSOR: c_int = 4;
pub const wxSTC_OSCRIPT_NUMBER: c_int = 5;
pub const wxSTC_OSCRIPT_SINGLEQUOTE_STRING: c_int = 6;
pub const wxSTC_OSCRIPT_DOUBLEQUOTE_STRING: c_int = 7;
pub const wxSTC_OSCRIPT_CONSTANT: c_int = 8;
pub const wxSTC_OSCRIPT_IDENTIFIER: c_int = 9;
pub const wxSTC_OSCRIPT_GLOBAL: c_int = 10;
pub const wxSTC_OSCRIPT_KEYWORD: c_int = 11;
pub const wxSTC_OSCRIPT_OPERATOR: c_int = 12;
pub const wxSTC_OSCRIPT_LABEL: c_int = 13;
pub const wxSTC_OSCRIPT_TYPE: c_int = 14;
pub const wxSTC_OSCRIPT_FUNCTION: c_int = 15;
pub const wxSTC_OSCRIPT_OBJECT: c_int = 16;
pub const wxSTC_OSCRIPT_PROPERTY: c_int = 17;
pub const wxSTC_OSCRIPT_METHOD: c_int = 18;
pub const wxSTC_VISUALPROLOG_DEFAULT: c_int = 0;
pub const wxSTC_VISUALPROLOG_KEY_MAJOR: c_int = 1;
pub const wxSTC_VISUALPROLOG_KEY_MINOR: c_int = 2;
pub const wxSTC_VISUALPROLOG_KEY_DIRECTIVE: c_int = 3;
pub const wxSTC_VISUALPROLOG_COMMENT_BLOCK: c_int = 4;
pub const wxSTC_VISUALPROLOG_COMMENT_LINE: c_int = 5;
pub const wxSTC_VISUALPROLOG_COMMENT_KEY: c_int = 6;
pub const wxSTC_VISUALPROLOG_COMMENT_KEY_ERROR: c_int = 7;
pub const wxSTC_VISUALPROLOG_IDENTIFIER: c_int = 8;
pub const wxSTC_VISUALPROLOG_VARIABLE: c_int = 9;
pub const wxSTC_VISUALPROLOG_ANONYMOUS: c_int = 10;
pub const wxSTC_VISUALPROLOG_NUMBER: c_int = 11;
pub const wxSTC_VISUALPROLOG_OPERATOR: c_int = 12;
pub const wxSTC_VISUALPROLOG_CHARACTER: c_int = 13;
pub const wxSTC_VISUALPROLOG_CHARACTER_TOO_MANY: c_int = 14;
pub const wxSTC_VISUALPROLOG_CHARACTER_ESCAPE_ERROR: c_int = 15;
pub const wxSTC_VISUALPROLOG_STRING: c_int = 16;
pub const wxSTC_VISUALPROLOG_STRING_ESCAPE: c_int = 17;
pub const wxSTC_VISUALPROLOG_STRING_ESCAPE_ERROR: c_int = 18;
pub const wxSTC_VISUALPROLOG_STRING_EOL_OPEN: c_int = 19;
pub const wxSTC_VISUALPROLOG_STRING_VERBATIM: c_int = 20;
pub const wxSTC_VISUALPROLOG_STRING_VERBATIM_SPECIAL: c_int = 21;
pub const wxSTC_VISUALPROLOG_STRING_VERBATIM_EOL: c_int = 22;
pub const wxSTC_CMD_REDO: c_int = 2011;
pub const wxSTC_CMD_SELECTALL: c_int = 2013;
pub const wxSTC_CMD_UNDO: c_int = 2176;
pub const wxSTC_CMD_CUT: c_int = 2177;
pub const wxSTC_CMD_COPY: c_int = 2178;
pub const wxSTC_CMD_PASTE: c_int = 2179;
pub const wxSTC_CMD_CLEAR: c_int = 2180;
pub const wxSTC_CMD_LINEDOWN: c_int = 2300;
pub const wxSTC_CMD_LINEDOWNEXTEND: c_int = 2301;
pub const wxSTC_CMD_LINEUP: c_int = 2302;
pub const wxSTC_CMD_LINEUPEXTEND: c_int = 2303;
pub const wxSTC_CMD_CHARLEFT: c_int = 2304;
pub const wxSTC_CMD_CHARLEFTEXTEND: c_int = 2305;
pub const wxSTC_CMD_CHARRIGHT: c_int = 2306;
pub const wxSTC_CMD_CHARRIGHTEXTEND: c_int = 2307;
pub const wxSTC_CMD_WORDLEFT: c_int = 2308;
pub const wxSTC_CMD_WORDLEFTEXTEND: c_int = 2309;
pub const wxSTC_CMD_WORDRIGHT: c_int = 2310;
pub const wxSTC_CMD_WORDRIGHTEXTEND: c_int = 2311;
pub const wxSTC_CMD_HOME: c_int = 2312;
pub const wxSTC_CMD_HOMEEXTEND: c_int = 2313;
pub const wxSTC_CMD_LINEEND: c_int = 2314;
pub const wxSTC_CMD_LINEENDEXTEND: c_int = 2315;
pub const wxSTC_CMD_DOCUMENTSTART: c_int = 2316;
pub const wxSTC_CMD_DOCUMENTSTARTEXTEND: c_int = 2317;
pub const wxSTC_CMD_DOCUMENTEND: c_int = 2318;
pub const wxSTC_CMD_DOCUMENTENDEXTEND: c_int = 2319;
pub const wxSTC_CMD_PAGEUP: c_int = 2320;
pub const wxSTC_CMD_PAGEUPEXTEND: c_int = 2321;
pub const wxSTC_CMD_PAGEDOWN: c_int = 2322;
pub const wxSTC_CMD_PAGEDOWNEXTEND: c_int = 2323;
pub const wxSTC_CMD_EDITTOGGLEOVERTYPE: c_int = 2324;
pub const wxSTC_CMD_CANCEL: c_int = 2325;
pub const wxSTC_CMD_DELETEBACK: c_int = 2326;
pub const wxSTC_CMD_TAB: c_int = 2327;
pub const wxSTC_CMD_BACKTAB: c_int = 2328;
pub const wxSTC_CMD_NEWLINE: c_int = 2329;
pub const wxSTC_CMD_FORMFEED: c_int = 2330;
pub const wxSTC_CMD_VCHOME: c_int = 2331;
pub const wxSTC_CMD_VCHOMEEXTEND: c_int = 2332;
pub const wxSTC_CMD_ZOOMIN: c_int = 2333;
pub const wxSTC_CMD_ZOOMOUT: c_int = 2334;
pub const wxSTC_CMD_DELWORDLEFT: c_int = 2335;
pub const wxSTC_CMD_DELWORDRIGHT: c_int = 2336;
pub const wxSTC_CMD_DELWORDRIGHTEND: c_int = 2518;
pub const wxSTC_CMD_LINECUT: c_int = 2337;
pub const wxSTC_CMD_LINEDELETE: c_int = 2338;
pub const wxSTC_CMD_LINETRANSPOSE: c_int = 2339;
pub const wxSTC_CMD_LINEDUPLICATE: c_int = 2404;
pub const wxSTC_CMD_LOWERCASE: c_int = 2340;
pub const wxSTC_CMD_UPPERCASE: c_int = 2341;
pub const wxSTC_CMD_LINESCROLLDOWN: c_int = 2342;
pub const wxSTC_CMD_LINESCROLLUP: c_int = 2343;
pub const wxSTC_CMD_DELETEBACKNOTLINE: c_int = 2344;
pub const wxSTC_CMD_HOMEDISPLAY: c_int = 2345;
pub const wxSTC_CMD_HOMEDISPLAYEXTEND: c_int = 2346;
pub const wxSTC_CMD_LINEENDDISPLAY: c_int = 2347;
pub const wxSTC_CMD_LINEENDDISPLAYEXTEND: c_int = 2348;
pub const wxSTC_CMD_HOMEWRAP: c_int = 2349;
pub const wxSTC_CMD_HOMEWRAPEXTEND: c_int = 2450;
pub const wxSTC_CMD_LINEENDWRAP: c_int = 2451;
pub const wxSTC_CMD_LINEENDWRAPEXTEND: c_int = 2452;
pub const wxSTC_CMD_VCHOMEWRAP: c_int = 2453;
pub const wxSTC_CMD_VCHOMEWRAPEXTEND: c_int = 2454;
pub const wxSTC_CMD_LINECOPY: c_int = 2455;
pub const wxSTC_CMD_WORDPARTLEFT: c_int = 2390;
pub const wxSTC_CMD_WORDPARTLEFTEXTEND: c_int = 2391;
pub const wxSTC_CMD_WORDPARTRIGHT: c_int = 2392;
pub const wxSTC_CMD_WORDPARTRIGHTEXTEND: c_int = 2393;
pub const wxSTC_CMD_DELLINELEFT: c_int = 2395;
pub const wxSTC_CMD_DELLINERIGHT: c_int = 2396;
pub const wxSTC_CMD_PARADOWN: c_int = 2413;
pub const wxSTC_CMD_PARADOWNEXTEND: c_int = 2414;
pub const wxSTC_CMD_PARAUP: c_int = 2415;
pub const wxSTC_CMD_PARAUPEXTEND: c_int = 2416;
pub const wxSTC_CMD_LINEDOWNRECTEXTEND: c_int = 2426;
pub const wxSTC_CMD_LINEUPRECTEXTEND: c_int = 2427;
pub const wxSTC_CMD_CHARLEFTRECTEXTEND: c_int = 2428;
pub const wxSTC_CMD_CHARRIGHTRECTEXTEND: c_int = 2429;
pub const wxSTC_CMD_HOMERECTEXTEND: c_int = 2430;
pub const wxSTC_CMD_VCHOMERECTEXTEND: c_int = 2431;
pub const wxSTC_CMD_LINEENDRECTEXTEND: c_int = 2432;
pub const wxSTC_CMD_PAGEUPRECTEXTEND: c_int = 2433;
pub const wxSTC_CMD_PAGEDOWNRECTEXTEND: c_int = 2434;
pub const wxSTC_CMD_STUTTEREDPAGEUP: c_int = 2435;
pub const wxSTC_CMD_STUTTEREDPAGEUPEXTEND: c_int = 2436;
pub const wxSTC_CMD_STUTTEREDPAGEDOWN: c_int = 2437;
pub const wxSTC_CMD_STUTTEREDPAGEDOWNEXTEND: c_int = 2438;
pub const wxSTC_CMD_WORDLEFTEND: c_int = 2439;
pub const wxSTC_CMD_WORDLEFTENDEXTEND: c_int = 2440;
pub const wxSTC_CMD_WORDRIGHTEND: c_int = 2441;
pub const wxSTC_CMD_WORDRIGHTENDEXTEND: c_int = 2442;
pub const wxSTC_CMD_VERTICALCENTRECARET: c_int = 2619;
pub const wxSTC_CMD_MOVESELECTEDLINESUP: c_int = 2620;
pub const wxSTC_CMD_MOVESELECTEDLINESDOWN: c_int = 2621;
pub const wxSTC_CMD_SCROLLTOSTART: c_int = 2628;
pub const wxSTC_CMD_SCROLLTOEND: c_int = 2629;

pub const wxSTACKWALKER_MAX_DEPTH: c_int = (200);

//  ENUM: wxFontFamily
pub const wxFONTFAMILY_DEFAULT: c_int = wxDEFAULT;
pub const wxFONTFAMILY_DECORATIVE: c_int = wxDECORATIVE;
pub const wxFONTFAMILY_ROMAN: c_int = wxROMAN;
pub const wxFONTFAMILY_SCRIPT: c_int = wxSCRIPT;
pub const wxFONTFAMILY_SWISS: c_int = wxSWISS;
pub const wxFONTFAMILY_MODERN: c_int = wxMODERN;
pub const wxFONTFAMILY_TELETYPE: c_int = wxTELETYPE;
pub const wxFONTFAMILY_MAX: c_int = wxTELETYPE + 1;
pub const wxFONTFAMILY_UNKNOWN: c_int = wxFONTFAMILY_MAX;
//  ENUM: wxFontStyle
pub const wxFONTSTYLE_NORMAL: c_int = wxNORMAL;
pub const wxFONTSTYLE_ITALIC: c_int = wxITALIC;
pub const wxFONTSTYLE_SLANT: c_int = wxSLANT;
pub const wxFONTSTYLE_MAX: c_int = wxSLANT + 1;
//  ENUM: wxFontWeight
pub const wxFONTWEIGHT_NORMAL: c_int = wxNORMAL;
pub const wxFONTWEIGHT_LIGHT: c_int = wxLIGHT;
pub const wxFONTWEIGHT_BOLD: c_int = wxBOLD;
pub const wxFONTWEIGHT_MAX: c_int = wxBOLD + 1;
//  ENUM: wxFontSymbolicSize
pub const wxFONTSIZE_XX_SMALL: c_int = -3;
pub const wxFONTSIZE_X_SMALL: c_int = -3 + 1;
pub const wxFONTSIZE_SMALL: c_int = -3 + 2;
pub const wxFONTSIZE_MEDIUM: c_int = -3 + 3;
pub const wxFONTSIZE_LARGE: c_int = -3 + 4;
pub const wxFONTSIZE_X_LARGE: c_int = -3 + 5;
pub const wxFONTSIZE_XX_LARGE: c_int = -3 + 6;
//  ENUM: wxFontFlag
pub const wxFONTFLAG_DEFAULT: c_int = 0;
pub const wxFONTFLAG_ITALIC: c_int = 1 << 0;
pub const wxFONTFLAG_SLANT: c_int = 1 << 1;
pub const wxFONTFLAG_LIGHT: c_int = 1 << 2;
pub const wxFONTFLAG_BOLD: c_int = 1 << 3;
pub const wxFONTFLAG_ANTIALIASED: c_int = 1 << 4;
pub const wxFONTFLAG_NOT_ANTIALIASED: c_int = 1 << 5;
pub const wxFONTFLAG_UNDERLINED: c_int = 1 << 6;
pub const wxFONTFLAG_STRIKETHROUGH: c_int = 1 << 7;
pub const wxFONTFLAG_MASK: c_int = wxFONTFLAG_ITALIC             |
                      wxFONTFLAG_SLANT              |
                      wxFONTFLAG_LIGHT              |
                      wxFONTFLAG_BOLD               |
                      wxFONTFLAG_ANTIALIASED        |
                      wxFONTFLAG_NOT_ANTIALIASED    |
                      wxFONTFLAG_UNDERLINED         |
                      wxFONTFLAG_STRIKETHROUGH;
//  ENUM: wxFontEncoding
pub const wxFONTENCODING_SYSTEM: c_int = -1;
pub const wxFONTENCODING_DEFAULT: c_int = -1 + 1;
pub const wxFONTENCODING_ISO8859_1: c_int = -1 + 2;
pub const wxFONTENCODING_ISO8859_2: c_int = -1 + 3;
pub const wxFONTENCODING_ISO8859_3: c_int = -1 + 4;
pub const wxFONTENCODING_ISO8859_4: c_int = -1 + 5;
pub const wxFONTENCODING_ISO8859_5: c_int = -1 + 6;
pub const wxFONTENCODING_ISO8859_6: c_int = -1 + 7;
pub const wxFONTENCODING_ISO8859_7: c_int = -1 + 8;
pub const wxFONTENCODING_ISO8859_8: c_int = -1 + 9;
pub const wxFONTENCODING_ISO8859_9: c_int = -1 + 10;
pub const wxFONTENCODING_ISO8859_10: c_int = -1 + 11;
pub const wxFONTENCODING_ISO8859_11: c_int = -1 + 12;
pub const wxFONTENCODING_ISO8859_12: c_int = -1 + 13;
pub const wxFONTENCODING_ISO8859_13: c_int = -1 + 14;
pub const wxFONTENCODING_ISO8859_14: c_int = -1 + 15;
pub const wxFONTENCODING_ISO8859_15: c_int = -1 + 16;
pub const wxFONTENCODING_ISO8859_MAX: c_int = -1 + 17;
pub const wxFONTENCODING_KOI8: c_int = -1 + 18;
pub const wxFONTENCODING_KOI8_U: c_int = -1 + 19;
pub const wxFONTENCODING_ALTERNATIVE: c_int = -1 + 20;
pub const wxFONTENCODING_BULGARIAN: c_int = -1 + 21;
pub const wxFONTENCODING_CP437: c_int = -1 + 22;
pub const wxFONTENCODING_CP850: c_int = -1 + 23;
pub const wxFONTENCODING_CP852: c_int = -1 + 24;
pub const wxFONTENCODING_CP855: c_int = -1 + 25;
pub const wxFONTENCODING_CP866: c_int = -1 + 26;
pub const wxFONTENCODING_CP874: c_int = -1 + 27;
pub const wxFONTENCODING_CP932: c_int = -1 + 28;
pub const wxFONTENCODING_CP936: c_int = -1 + 29;
pub const wxFONTENCODING_CP949: c_int = -1 + 30;
pub const wxFONTENCODING_CP950: c_int = -1 + 31;
pub const wxFONTENCODING_CP1250: c_int = -1 + 32;
pub const wxFONTENCODING_CP1251: c_int = -1 + 33;
pub const wxFONTENCODING_CP1252: c_int = -1 + 34;
pub const wxFONTENCODING_CP1253: c_int = -1 + 35;
pub const wxFONTENCODING_CP1254: c_int = -1 + 36;
pub const wxFONTENCODING_CP1255: c_int = -1 + 37;
pub const wxFONTENCODING_CP1256: c_int = -1 + 38;
pub const wxFONTENCODING_CP1257: c_int = -1 + 39;
pub const wxFONTENCODING_CP1258: c_int = -1 + 40;
pub const wxFONTENCODING_CP1361: c_int = -1 + 41;
pub const wxFONTENCODING_CP12_MAX: c_int = -1 + 42;
pub const wxFONTENCODING_UTF7: c_int = -1 + 43;
pub const wxFONTENCODING_UTF8: c_int = -1 + 44;
pub const wxFONTENCODING_EUC_JP: c_int = -1 + 45;
pub const wxFONTENCODING_UTF16BE: c_int = -1 + 46;
pub const wxFONTENCODING_UTF16LE: c_int = -1 + 47;
pub const wxFONTENCODING_UTF32BE: c_int = -1 + 48;
pub const wxFONTENCODING_UTF32LE: c_int = -1 + 49;
pub const wxFONTENCODING_MACROMAN: c_int = -1 + 50;
pub const wxFONTENCODING_MACJAPANESE: c_int = -1 + 51;
pub const wxFONTENCODING_MACCHINESETRAD: c_int = -1 + 52;
pub const wxFONTENCODING_MACKOREAN: c_int = -1 + 53;
pub const wxFONTENCODING_MACARABIC: c_int = -1 + 54;
pub const wxFONTENCODING_MACHEBREW: c_int = -1 + 55;
pub const wxFONTENCODING_MACGREEK: c_int = -1 + 56;
pub const wxFONTENCODING_MACCYRILLIC: c_int = -1 + 57;
pub const wxFONTENCODING_MACDEVANAGARI: c_int = -1 + 58;
pub const wxFONTENCODING_MACGURMUKHI: c_int = -1 + 59;
pub const wxFONTENCODING_MACGUJARATI: c_int = -1 + 60;
pub const wxFONTENCODING_MACORIYA: c_int = -1 + 61;
pub const wxFONTENCODING_MACBENGALI: c_int = -1 + 62;
pub const wxFONTENCODING_MACTAMIL: c_int = -1 + 63;
pub const wxFONTENCODING_MACTELUGU: c_int = -1 + 64;
pub const wxFONTENCODING_MACKANNADA: c_int = -1 + 65;
pub const wxFONTENCODING_MACMALAJALAM: c_int = -1 + 66;
pub const wxFONTENCODING_MACSINHALESE: c_int = -1 + 67;
pub const wxFONTENCODING_MACBURMESE: c_int = -1 + 68;
pub const wxFONTENCODING_MACKHMER: c_int = -1 + 69;
pub const wxFONTENCODING_MACTHAI: c_int = -1 + 70;
pub const wxFONTENCODING_MACLAOTIAN: c_int = -1 + 71;
pub const wxFONTENCODING_MACGEORGIAN: c_int = -1 + 72;
pub const wxFONTENCODING_MACARMENIAN: c_int = -1 + 73;
pub const wxFONTENCODING_MACCHINESESIMP: c_int = -1 + 74;
pub const wxFONTENCODING_MACTIBETAN: c_int = -1 + 75;
pub const wxFONTENCODING_MACMONGOLIAN: c_int = -1 + 76;
pub const wxFONTENCODING_MACETHIOPIC: c_int = -1 + 77;
pub const wxFONTENCODING_MACCENTRALEUR: c_int = -1 + 78;
pub const wxFONTENCODING_MACVIATNAMESE: c_int = -1 + 79;
pub const wxFONTENCODING_MACARABICEXT: c_int = -1 + 80;
pub const wxFONTENCODING_MACSYMBOL: c_int = -1 + 81;
pub const wxFONTENCODING_MACDINGBATS: c_int = -1 + 82;
pub const wxFONTENCODING_MACTURKISH: c_int = -1 + 83;
pub const wxFONTENCODING_MACCROATIAN: c_int = -1 + 84;
pub const wxFONTENCODING_MACICELANDIC: c_int = -1 + 85;
pub const wxFONTENCODING_MACROMANIAN: c_int = -1 + 86;
pub const wxFONTENCODING_MACCELTIC: c_int = -1 + 87;
pub const wxFONTENCODING_MACGAELIC: c_int = -1 + 88;
pub const wxFONTENCODING_MACKEYBOARD: c_int = -1 + 89;
pub const wxFONTENCODING_ISO2022_JP: c_int = -1 + 90;
pub const wxFONTENCODING_MAX: c_int = -1 + 91;
pub const wxFONTENCODING_MACMIN: c_int = wxFONTENCODING_MACROMAN;
pub const wxFONTENCODING_MACMAX: c_int = wxFONTENCODING_MACKEYBOARD;
pub const wxFONTENCODING_UTF16: c_int = wxFONTENCODING_MACKEYBOARD + 1;
pub const wxFONTENCODING_UTF32: c_int = wxFONTENCODING_MACKEYBOARD + 2;
pub const wxFONTENCODING_UNICODE: c_int = wxFONTENCODING_MACKEYBOARD + 3;
pub const wxFONTENCODING_GB2312: c_int = wxFONTENCODING_CP936;
pub const wxFONTENCODING_BIG5: c_int = wxFONTENCODING_CP950;
pub const wxFONTENCODING_SHIFT_JIS: c_int = wxFONTENCODING_CP932;
pub const wxFONTENCODING_EUC_KR: c_int = wxFONTENCODING_CP949;
pub const wxFONTENCODING_JOHAB: c_int = wxFONTENCODING_CP1361;
pub const wxFONTENCODING_VIETNAMESE: c_int = wxFONTENCODING_CP1258;

//  ENUM: wxToolBarToolStyle
pub const wxTOOL_STYLE_BUTTON: c_int = 1;
pub const wxTOOL_STYLE_SEPARATOR: c_int = 2;
pub const wxTOOL_STYLE_CONTROL: c_int = 2 + 1;
//  ENUM: @50
pub const wxTB_HORIZONTAL: c_int = wxHORIZONTAL;
pub const wxTB_TOP: c_int = wxTB_HORIZONTAL;
pub const wxTB_VERTICAL: c_int = wxVERTICAL;
pub const wxTB_LEFT: c_int = wxTB_VERTICAL;
pub const wxTB_3DBUTTONS: c_int = wxTB_VERTICAL + 1;
pub const wxTB_FLAT: c_int = wxTB_VERTICAL + 2;
pub const wxTB_DOCKABLE: c_int = wxTB_VERTICAL + 3;
pub const wxTB_NOICONS: c_int = wxTB_VERTICAL + 4;
pub const wxTB_TEXT: c_int = wxTB_VERTICAL + 5;
pub const wxTB_NODIVIDER: c_int = wxTB_VERTICAL + 6;
pub const wxTB_NOALIGN: c_int = wxTB_VERTICAL + 7;
pub const wxTB_HORZ_LAYOUT: c_int = wxTB_VERTICAL + 8;
pub const wxTB_HORZ_TEXT: c_int = wxTB_HORZ_LAYOUT | wxTB_TEXT;
pub const wxTB_NO_TOOLTIPS: c_int = wxTB_HORZ_LAYOUT | wxTB_TEXT + 1;
pub const wxTB_BOTTOM: c_int = wxTB_HORZ_LAYOUT | wxTB_TEXT + 2;
pub const wxTB_RIGHT: c_int = wxTB_HORZ_LAYOUT | wxTB_TEXT + 3;
pub const wxTB_DEFAULT_STYLE: c_int = wxTB_HORIZONTAL | wxTB_FLAT;

//  ENUM: wxOutCode
pub const wxInside: c_int = 0x00;
pub const wxOutLeft: c_int = 0x01;
pub const wxOutRight: c_int = 0x02;
pub const wxOutTop: c_int = 0x08;
pub const wxOutBottom: c_int = 0x04;

pub const wxSPLASH_CENTRE_ON_PARENT: c_int = 0x01;
pub const wxSPLASH_CENTRE_ON_SCREEN: c_int = 0x02;
pub const wxSPLASH_NO_CENTRE: c_int = 0x00;
pub const wxSPLASH_TIMEOUT: c_int = 0x04;
pub const wxSPLASH_NO_TIMEOUT: c_int = 0x00;

pub const wxSTB_SIZEGRIP: c_long = 0x0010;
pub const wxSTB_SHOW_TIPS: c_long = 0x0020;
pub const wxSTB_ELLIPSIZE_START: c_int = 0x0040;
pub const wxSTB_ELLIPSIZE_MIDDLE: c_int = 0x0080;
pub const wxSTB_ELLIPSIZE_END: c_long = 0x0100;
pub const wxSTB_DEFAULT_STYLE: c_long = (wxSTB_SIZEGRIP|wxSTB_ELLIPSIZE_END|wxSTB_SHOW_TIPS|wxFULL_REPAINT_ON_RESIZE);
pub const wxSB_NORMAL: c_int = 0x0000;
pub const wxSB_FLAT: c_int = 0x0001;
pub const wxSB_RAISED: c_int = 0x0002;
pub const wxSB_SUNKEN: c_int = 0x0003;

//  ENUM: wxStockLabelQueryFlag
pub const wxSTOCK_NOFLAGS: c_int = 0;
pub const wxSTOCK_WITH_MNEMONIC: c_int = 1;
pub const wxSTOCK_WITH_ACCELERATOR: c_int = 2;
pub const wxSTOCK_WITHOUT_ELLIPSIS: c_int = 4;
pub const wxSTOCK_FOR_BUTTON: c_int = wxSTOCK_WITHOUT_ELLIPSIS | wxSTOCK_WITH_MNEMONIC;

//  ENUM: wxHtmlURLType
pub const wxHTML_URL_PAGE: c_int = 0;
pub const wxHTML_URL_IMAGE: c_int = 0 + 1;
pub const wxHTML_URL_OTHER: c_int = 0 + 2;

//  ENUM: wxRibbonArtSetting
pub const wxRIBBON_ART_TAB_SEPARATION_SIZE: c_int = 0;
pub const wxRIBBON_ART_PAGE_BORDER_LEFT_SIZE: c_int = 0 + 1;
pub const wxRIBBON_ART_PAGE_BORDER_TOP_SIZE: c_int = 0 + 2;
pub const wxRIBBON_ART_PAGE_BORDER_RIGHT_SIZE: c_int = 0 + 3;
pub const wxRIBBON_ART_PAGE_BORDER_BOTTOM_SIZE: c_int = 0 + 4;
pub const wxRIBBON_ART_PANEL_X_SEPARATION_SIZE: c_int = 0 + 5;
pub const wxRIBBON_ART_PANEL_Y_SEPARATION_SIZE: c_int = 0 + 6;
pub const wxRIBBON_ART_TOOL_GROUP_SEPARATION_SIZE: c_int = 0 + 7;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_LEFT_SIZE: c_int = 0 + 8;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_RIGHT_SIZE: c_int = 0 + 9;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_TOP_SIZE: c_int = 0 + 10;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_BOTTOM_SIZE: c_int = 0 + 11;
pub const wxRIBBON_ART_PANEL_LABEL_FONT: c_int = 0 + 12;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_FONT: c_int = 0 + 13;
pub const wxRIBBON_ART_TAB_LABEL_FONT: c_int = 0 + 14;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_COLOUR: c_int = 0 + 15;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_DISABLED_COLOUR: c_int = 0 + 16;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BORDER_COLOUR: c_int = 0 + 17;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 18;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 19;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_COLOUR: c_int = 0 + 20;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 21;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BORDER_COLOUR: c_int = 0 + 22;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 23;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 24;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 25;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 26;
pub const wxRIBBON_ART_GALLERY_BORDER_COLOUR: c_int = 0 + 27;
pub const wxRIBBON_ART_GALLERY_HOVER_BACKGROUND_COLOUR: c_int = 0 + 28;
pub const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_COLOUR: c_int = 0 + 29;
pub const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 30;
pub const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_TOP_COLOUR: c_int = 0 + 31;
pub const wxRIBBON_ART_GALLERY_BUTTON_FACE_COLOUR: c_int = 0 + 32;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_COLOUR: c_int = 0 + 33;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 34;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 35;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_FACE_COLOUR: c_int = 0 + 36;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 37;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 38;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 39;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_FACE_COLOUR: c_int = 0 + 40;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_COLOUR: c_int = 0 + 41;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 42;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_TOP_COLOUR: c_int = 0 + 43;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_FACE_COLOUR: c_int = 0 + 44;
pub const wxRIBBON_ART_GALLERY_ITEM_BORDER_COLOUR: c_int = 0 + 45;
pub const wxRIBBON_ART_TAB_LABEL_COLOUR: c_int = 0 + 46;
pub const wxRIBBON_ART_TAB_SEPARATOR_COLOUR: c_int = 0 + 47;
pub const wxRIBBON_ART_TAB_SEPARATOR_GRADIENT_COLOUR: c_int = 0 + 48;
pub const wxRIBBON_ART_TAB_CTRL_BACKGROUND_COLOUR: c_int = 0 + 49;
pub const wxRIBBON_ART_TAB_CTRL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 50;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 51;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 52;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_COLOUR: c_int = 0 + 53;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 54;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 55;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 56;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 57;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 58;
pub const wxRIBBON_ART_TAB_BORDER_COLOUR: c_int = 0 + 59;
pub const wxRIBBON_ART_PANEL_BORDER_COLOUR: c_int = 0 + 60;
pub const wxRIBBON_ART_PANEL_BORDER_GRADIENT_COLOUR: c_int = 0 + 61;
pub const wxRIBBON_ART_PANEL_MINIMISED_BORDER_COLOUR: c_int = 0 + 62;
pub const wxRIBBON_ART_PANEL_MINIMISED_BORDER_GRADIENT_COLOUR: c_int = 0 + 63;
pub const wxRIBBON_ART_PANEL_LABEL_BACKGROUND_COLOUR: c_int = 0 + 64;
pub const wxRIBBON_ART_PANEL_LABEL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 65;
pub const wxRIBBON_ART_PANEL_LABEL_COLOUR: c_int = 0 + 66;
pub const wxRIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_COLOUR: c_int = 0 + 67;
pub const wxRIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 68;
pub const wxRIBBON_ART_PANEL_HOVER_LABEL_COLOUR: c_int = 0 + 69;
pub const wxRIBBON_ART_PANEL_MINIMISED_LABEL_COLOUR: c_int = 0 + 70;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 71;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 72;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 73;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 74;
pub const wxRIBBON_ART_PAGE_BORDER_COLOUR: c_int = 0 + 75;
pub const wxRIBBON_ART_PAGE_BACKGROUND_TOP_COLOUR: c_int = 0 + 76;
pub const wxRIBBON_ART_PAGE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 77;
pub const wxRIBBON_ART_PAGE_BACKGROUND_COLOUR: c_int = 0 + 78;
pub const wxRIBBON_ART_PAGE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 79;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 80;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 81;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_COLOUR: c_int = 0 + 82;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 83;
pub const wxRIBBON_ART_TOOLBAR_BORDER_COLOUR: c_int = 0 + 84;
pub const wxRIBBON_ART_TOOLBAR_HOVER_BORDER_COLOUR: c_int = 0 + 85;
pub const wxRIBBON_ART_TOOLBAR_FACE_COLOUR: c_int = 0 + 86;
pub const wxRIBBON_ART_TOOL_BACKGROUND_TOP_COLOUR: c_int = 0 + 87;
pub const wxRIBBON_ART_TOOL_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 88;
pub const wxRIBBON_ART_TOOL_BACKGROUND_COLOUR: c_int = 0 + 89;
pub const wxRIBBON_ART_TOOL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 90;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 91;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 92;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_COLOUR: c_int = 0 + 93;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 94;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 95;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 96;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 97;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 98;
//  ENUM: wxRibbonScrollButtonStyle
pub const wxRIBBON_SCROLL_BTN_LEFT: c_int = 0;
pub const wxRIBBON_SCROLL_BTN_RIGHT: c_int = 1;
pub const wxRIBBON_SCROLL_BTN_UP: c_int = 2;
pub const wxRIBBON_SCROLL_BTN_DOWN: c_int = 3;
pub const wxRIBBON_SCROLL_BTN_DIRECTION_MASK: c_int = 3;
pub const wxRIBBON_SCROLL_BTN_NORMAL: c_int = 0;
pub const wxRIBBON_SCROLL_BTN_HOVERED: c_int = 4;
pub const wxRIBBON_SCROLL_BTN_ACTIVE: c_int = 8;
pub const wxRIBBON_SCROLL_BTN_STATE_MASK: c_int = 12;
pub const wxRIBBON_SCROLL_BTN_FOR_OTHER: c_int = 0;
pub const wxRIBBON_SCROLL_BTN_FOR_TABS: c_int = 16;
pub const wxRIBBON_SCROLL_BTN_FOR_PAGE: c_int = 32;
pub const wxRIBBON_SCROLL_BTN_FOR_MASK: c_int = 48;
//  ENUM: wxRibbonButtonKind
pub const wxRIBBON_BUTTON_NORMAL: c_int = 1 << 0;
pub const wxRIBBON_BUTTON_DROPDOWN: c_int = 1 << 1;
pub const wxRIBBON_BUTTON_HYBRID: c_int = wxRIBBON_BUTTON_NORMAL | wxRIBBON_BUTTON_DROPDOWN;
pub const wxRIBBON_BUTTON_TOGGLE: c_int = 1 << 2;

//  ENUM: @58
pub const wxEXTEND_LAST_ON_EACH_LINE: c_int = 0;
pub const wxREMOVE_LEADING_SPACES: c_int = 0 + 1;
pub const wxWRAPSIZER_DEFAULT_FLAGS: c_int = 0 + 2;

pub const wxCP_DEFAULT_STYLE: c_long = (wxTAB_TRAVERSAL | wxNO_BORDER);
pub const wxCP_NO_TLW_RESIZE: c_int = (0x0002);

pub const wxFC_DEFAULT_STYLE: c_int = wxFC_OPEN;
//  ENUM: @17
pub const wxFC_OPEN: c_int = 0x0001;
pub const wxFC_SAVE: c_int = 0x0002;
pub const wxFC_MULTIPLE: c_int = 0x0004;
pub const wxFC_NOSHOWHIDDEN: c_int = 0x0008;

pub const wxPG_COLOUR_WEB_BASE: c_int = 0x10000;
pub const wxPG_COLOUR_CUSTOM: c_int = 0xFFFFFF;
pub const wxPG_COLOUR_UNSPECIFIED: c_int = (wxPG_COLOUR_CUSTOM+1);
pub const wxPG_PROP_TRANSLATE_CUSTOM: c_long = wxPG_PROP_CLASS_SPECIFIC_1;

//  ENUM: wxAutomationInstanceFlags
pub const wxAutomationInstance_UseExistingOnly: c_int = 0;
pub const wxAutomationInstance_CreateIfNeeded: c_int = 1;
pub const wxAutomationInstance_SilentIfNone: c_int = 2;
//  ENUM: wxOleConvertVariantFlags
pub const wxOleConvertVariant_Default: c_int = 0;
pub const wxOleConvertVariant_ReturnSafeArrays: c_int = 1;

pub const wxID_HTML_HELPFRAME: c_int = (wxID_HIGHEST + 1);
pub const wxHF_EMBEDDED: c_int = 0x00008000;
pub const wxHF_DIALOG: c_int = 0x00010000;
pub const wxHF_FRAME: c_int = 0x00020000;
pub const wxHF_MODAL: c_int = 0x00040000;

// NODEF: WXTRACE
// NODEF: WXTRACELEVEL

pub const wxICON_SCREEN_DEPTH: c_int = (-1);

//  ENUM: wxIPCFormat

//  SKIP: wxDEFINE_EVENT
//  SKIP: wxDECLARE_EVENT
//  SKIP: wxDECLARE_EXPORTED_EVENT
//  SKIP: wxEVENT_HANDLER_CAST
//  SKIP: wx__DECLARE_EVT1
//  SKIP: wx__DECLARE_EVT2
//  SKIP: wx__DECLARE_EVT0
// NODEF: wxDECLARE_EVENT_TABLE
// NODEF: wxBEGIN_EVENT_TABLE
// NODEF: wxEND_EVENT_TABLE
//  ENUM: wxEventPropagation
pub const wxEVENT_PROPAGATE_NONE: c_int = 0;
//  SKIP: wxEVENT_PROPAGATE_MAX
//  ENUM: wxEventCategory
pub const wxEVT_CATEGORY_UI: c_int = 1;
pub const wxEVT_CATEGORY_USER_INPUT: c_int = 2;
pub const wxEVT_CATEGORY_SOCKET: c_int = 4;
pub const wxEVT_CATEGORY_TIMER: c_int = 8;
pub const wxEVT_CATEGORY_THREAD: c_int = 16;
pub const wxEVT_CATEGORY_ALL: c_int =
        wxEVT_CATEGORY_UI|wxEVT_CATEGORY_USER_INPUT|wxEVT_CATEGORY_SOCKET| 
        wxEVT_CATEGORY_TIMER|wxEVT_CATEGORY_THREAD;
//  ENUM: wxKeyCategoryFlags
pub const WXK_CATEGORY_ARROW: c_int = 0;
pub const WXK_CATEGORY_PAGING: c_int = 0 + 1;
pub const WXK_CATEGORY_JUMP: c_int = 0 + 2;
pub const WXK_CATEGORY_TAB: c_int = 0 + 3;
pub const WXK_CATEGORY_CUT: c_int = 0 + 4;
pub const WXK_CATEGORY_NAVIGATION: c_int = 0 + 5;
//  ENUM: @13
pub const wxJOYSTICK1: c_int = 0;
pub const wxJOYSTICK2: c_int = 0 + 1;
//  ENUM: @14
pub const wxJOY_BUTTON_ANY: c_int = -1;
pub const wxJOY_BUTTON1: c_int = 1;
pub const wxJOY_BUTTON2: c_int = 2;
pub const wxJOY_BUTTON3: c_int = 4;
pub const wxJOY_BUTTON4: c_int = 8;
//  ENUM: wxUpdateUIMode
pub const wxUPDATE_UI_PROCESS_ALL: c_int = 0;
pub const wxUPDATE_UI_PROCESS_SPECIFIED: c_int = 0 + 1;
//  ENUM: wxMouseWheelAxis
pub const wxMOUSE_WHEEL_VERTICAL: c_int = 0;
pub const wxMOUSE_WHEEL_HORIZONTAL: c_int = 0 + 1;
//  ENUM: wxIdleMode
pub const wxIDLE_PROCESS_ALL: c_int = 0;
pub const wxIDLE_PROCESS_SPECIFIED: c_int = 0 + 1;

// NODEF: wxDECLARE_APP
// NODEF: wxIMPLEMENT_APP
//  SKIP: wxDISABLE_DEBUG_SUPPORT

//  ENUM: @41
pub const Selected: c_int = 0x00010000;
pub const ChoicePopup: c_int = 0x00020000;
pub const Control: c_int = 0x00040000;
pub const Disabled: c_int = 0x00080000;
pub const DontUseCellFgCol: c_int = 0x00100000;
pub const DontUseCellBgCol: c_int = 0x00200000;
pub const DontUseCellColours: c_int = DontUseCellFgCol |
                              DontUseCellBgCol;

//  ENUM: @21
pub const WX_GL_RGBA: c_int = 1;
pub const WX_GL_BUFFER_SIZE: c_int = 1 + 1;
pub const WX_GL_LEVEL: c_int = 1 + 2;
pub const WX_GL_DOUBLEBUFFER: c_int = 1 + 3;
pub const WX_GL_STEREO: c_int = 1 + 4;
pub const WX_GL_AUX_BUFFERS: c_int = 1 + 5;
pub const WX_GL_MIN_RED: c_int = 1 + 6;
pub const WX_GL_MIN_GREEN: c_int = 1 + 7;
pub const WX_GL_MIN_BLUE: c_int = 1 + 8;
pub const WX_GL_MIN_ALPHA: c_int = 1 + 9;
pub const WX_GL_DEPTH_SIZE: c_int = 1 + 10;
pub const WX_GL_STENCIL_SIZE: c_int = 1 + 11;
pub const WX_GL_MIN_ACCUM_RED: c_int = 1 + 12;
pub const WX_GL_MIN_ACCUM_GREEN: c_int = 1 + 13;
pub const WX_GL_MIN_ACCUM_BLUE: c_int = 1 + 14;
pub const WX_GL_MIN_ACCUM_ALPHA: c_int = 1 + 15;
pub const WX_GL_SAMPLE_BUFFERS: c_int = 1 + 16;
pub const WX_GL_SAMPLES: c_int = 1 + 17;
pub const WX_GL_CORE_PROFILE: c_int = 1 + 18;
pub const WX_GL_MAJOR_VERSION: c_int = 1 + 19;
pub const WX_GL_MINOR_VERSION: c_int = 1 + 20;

// NODEF: wxCLASSINFO
// NODEF: wxDECLARE_ABSTRACT_CLASS
// NODEF: wxDECLARE_DYNAMIC_CLASS
// NODEF: wxDECLARE_CLASS
// NODEF: wxIMPLEMENT_ABSTRACT_CLASS
// NODEF: wxIMPLEMENT_ABSTRACT_CLASS2
// NODEF: wxIMPLEMENT_DYNAMIC_CLASS
// NODEF: wxIMPLEMENT_DYNAMIC_CLASS2
// NODEF: wxIMPLEMENT_CLASS
// NODEF: wxIMPLEMENT_CLASS2
// NODEF: wx_const_cast
// NODEF: wx_reinterpret_cast
// NODEF: wx_static_cast
// NODEF: wx_truncate_cast
// NODEF: wxConstCast
// NODEF: wxDynamicCast
// NODEF: wxDynamicCastThis
// NODEF: wxStaticCast
// NODEF: WXDEBUG_NEW

//  ENUM: wxTaskBarIconType
pub const wxTBI_DOCK: c_int = 0;
pub const wxTBI_CUSTOM_STATUSITEM: c_int = 0 + 1;
pub const wxTBI_DEFAULT_TYPE: c_int = 0 + 2;

//  ENUM: @46
pub const Option_AllowPixelFontSize: c_int = 0x0001;

//  ENUM: wxScrollbarVisibility
pub const wxSHOW_SB_NEVER: c_int = -1;
pub const wxSHOW_SB_DEFAULT: c_int = -1 + 1;
pub const wxSHOW_SB_ALWAYS: c_int = -1 + 2;

//  ENUM: wxSocketError
pub const wxSOCKET_NOERROR: c_int = 0;
pub const wxSOCKET_INVOP: c_int = 0 + 1;
pub const wxSOCKET_IOERR: c_int = 0 + 2;
pub const wxSOCKET_INVADDR: c_int = 0 + 3;
pub const wxSOCKET_INVSOCK: c_int = 0 + 4;
pub const wxSOCKET_NOHOST: c_int = 0 + 5;
pub const wxSOCKET_INVPORT: c_int = 0 + 6;
pub const wxSOCKET_WOULDBLOCK: c_int = 0 + 7;
pub const wxSOCKET_TIMEDOUT: c_int = 0 + 8;
pub const wxSOCKET_MEMERR: c_int = 0 + 9;
//  ENUM: wxSocketEventFlags
pub const wxSOCKET_INPUT: c_int = 0;
pub const wxSOCKET_OUTPUT: c_int = 0 + 1;
pub const wxSOCKET_CONNECTION: c_int = 0 + 2;
pub const wxSOCKET_LOST: c_int = 0 + 3;
//  ENUM: @47
pub const wxSOCKET_NONE: c_int = 0;
pub const wxSOCKET_NOWAIT: c_int = 1;
pub const wxSOCKET_WAITALL: c_int = 2;
pub const wxSOCKET_BLOCK: c_int = 4;
pub const wxSOCKET_REUSEADDR: c_int = 8;
pub const wxSOCKET_BROADCAST: c_int = 16;
pub const wxSOCKET_NOBIND: c_int = 32;
pub const wxSOCKET_NOWAIT_READ: c_int = 64;
pub const wxSOCKET_WAITALL_READ: c_int = 128;
pub const wxSOCKET_NOWAIT_WRITE: c_int = 256;
pub const wxSOCKET_WAITALL_WRITE: c_int = 512;

//  ENUM: @49
pub const wxTP_DEFAULT: c_int = 0;

//  ENUM: ConversionFlags
pub const Escape: c_int = 0x01;
pub const QuoteStrings: c_int = 0x02;

//  ENUM: wxOwnerDrawnComboBoxPaintingFlags
pub const wxODCB_PAINTING_CONTROL: c_int = 0x0001;
pub const wxODCB_PAINTING_SELECTED: c_int = 0x0002;
//  ENUM: @40
pub const wxODCB_DCLICK_CYCLES: c_int = wxCC_SPECIAL_DCLICK;
pub const wxODCB_STD_CONTROL_PAINT: c_int = 0x1000;

pub const wxDVC_DEFAULT_RENDERER_SIZE: c_int = 20;
pub const wxDVC_DEFAULT_WIDTH: c_int = 80;
pub const wxDVC_TOGGLE_DEFAULT_WIDTH: c_int = 30;
pub const wxDVC_DEFAULT_MINWIDTH: c_int = 30;
pub const wxDVR_DEFAULT_ALIGNMENT: c_int = -1;
pub const wxDV_SINGLE: c_int = 0x0000;
pub const wxDV_MULTIPLE: c_int = 0x0001;
pub const wxDV_NO_HEADER: c_int = 0x0002;
pub const wxDV_HORIZ_RULES: c_int = 0x0004;
pub const wxDV_VERT_RULES: c_int = 0x0008;
pub const wxDV_ROW_LINES: c_int = 0x0010;
pub const wxDV_VARIABLE_LINE_HEIGHT: c_int = 0x0020;
//  ENUM: wxDataViewCellMode
pub const wxDATAVIEW_CELL_INERT: c_int = 0;
pub const wxDATAVIEW_CELL_ACTIVATABLE: c_int = 0 + 1;
pub const wxDATAVIEW_CELL_EDITABLE: c_int = 0 + 2;
//  ENUM: wxDataViewCellRenderState
pub const wxDATAVIEW_CELL_SELECTED: c_int = 1;
pub const wxDATAVIEW_CELL_PRELIT: c_int = 2;
pub const wxDATAVIEW_CELL_INSENSITIVE: c_int = 4;
pub const wxDATAVIEW_CELL_FOCUSED: c_int = 8;
//  ENUM: wxDataViewColumnFlags
pub const wxDATAVIEW_COL_RESIZABLE: c_int = 1;
pub const wxDATAVIEW_COL_SORTABLE: c_int = 2;
pub const wxDATAVIEW_COL_REORDERABLE: c_int = 4;
pub const wxDATAVIEW_COL_HIDDEN: c_int = 8;

// NODEF: wxFORCE_LINK_THIS_MODULE
// NODEF: wxFORCE_LINK_MODULE

// NODEF: wxANY_AS
// NODEF: wxANY_CHECK_TYPE
// NODEF: wxANY_VALUE_TYPE_CHECK_TYPE
//  ENUM: @0
pub const WX_ANY_VALUE_BUFFER_SIZE: c_int = 16;

// NODEF: wxDISABLE_DEBUG_LOGGING_IN_RELEASE_BUILD
//  ENUM: wxLogLevelValues
pub const wxLOG_FatalError: c_int = 0;
pub const wxLOG_Error: c_int = 0 + 1;
pub const wxLOG_Warning: c_int = 0 + 2;
pub const wxLOG_Message: c_int = 0 + 3;
pub const wxLOG_Status: c_int = 0 + 4;
pub const wxLOG_Info: c_int = 0 + 5;
pub const wxLOG_Debug: c_int = 0 + 6;
pub const wxLOG_Trace: c_int = 0 + 7;
pub const wxLOG_Progress: c_int = 0 + 8;
pub const wxLOG_User: c_int = 100;
pub const wxLOG_Max: c_int = 10000;

// NODEF: wxLongLongFmtSpec

pub const wxHL_CONTEXTMENU: c_long = 0x0001;
pub const wxHL_ALIGN_LEFT: c_int = 0x0002;
pub const wxHL_ALIGN_RIGHT: c_int = 0x0004;
pub const wxHL_ALIGN_CENTRE: c_long = 0x0008;
pub const wxHL_DEFAULT_STYLE: c_long = (wxHL_CONTEXTMENU|wxNO_BORDER|wxHL_ALIGN_CENTRE);

// NODEF: wxCONCAT
// NODEF: wxCONCAT3
// NODEF: wxCONCAT4
// NODEF: wxCONCAT5
// NODEF: wxSTRINGIZE
// NODEF: wxSTRINGIZE_T
// NODEF: __WXFUNCTION__

pub const wxCHK_2STATE: c_int = 0x4000;
pub const wxCHK_3STATE: c_int = 0x1000;
pub const wxCHK_ALLOW_3RD_STATE_FOR_USER: c_int = 0x2000;
//  ENUM: wxCheckBoxState
pub const wxCHK_UNCHECKED: c_int = 0;
pub const wxCHK_CHECKED: c_int = 0 + 1;
pub const wxCHK_UNDETERMINED: c_int = 0 + 2;

//  ENUM: wxPropertySheetDialogFlags
pub const wxPROPSHEET_DEFAULT: c_int = 0x0001;
pub const wxPROPSHEET_NOTEBOOK: c_int = 0x0002;
pub const wxPROPSHEET_TOOLBOOK: c_int = 0x0004;
pub const wxPROPSHEET_CHOICEBOOK: c_int = 0x0008;
pub const wxPROPSHEET_LISTBOOK: c_int = 0x0010;
pub const wxPROPSHEET_BUTTONTOOLBOOK: c_int = 0x0020;
pub const wxPROPSHEET_TREEBOOK: c_int = 0x0040;
pub const wxPROPSHEET_SHRINKTOFIT: c_int = 0x0100;

// NODEF: wxPLURAL
// NODEF: wxTRANSLATE

//  ENUM: EntryType
pub const Type_Unknown: c_int = 0;
pub const Type_String: c_int = 0 + 1;
pub const Type_Boolean: c_int = 0 + 2;
pub const Type_Integer: c_int = 0 + 3;
pub const Type_Float: c_int = 0 + 4;

//  ENUM: TZ
pub const Local: c_int = 0;
pub const GMT_12: c_int = 0 + 1;
pub const GMT_11: c_int = 0 + 2;
pub const GMT_10: c_int = 0 + 3;
pub const GMT_9: c_int = 0 + 4;
pub const GMT_8: c_int = 0 + 5;
pub const GMT_7: c_int = 0 + 6;
pub const GMT_6: c_int = 0 + 7;
pub const GMT_5: c_int = 0 + 8;
pub const GMT_4: c_int = 0 + 9;
pub const GMT_3: c_int = 0 + 10;
pub const GMT_2: c_int = 0 + 11;
pub const GMT_1: c_int = 0 + 12;
pub const GMT0: c_int = 0 + 13;
pub const GMT1: c_int = 0 + 14;
pub const GMT2: c_int = 0 + 15;
pub const GMT3: c_int = 0 + 16;
pub const GMT4: c_int = 0 + 17;
pub const GMT5: c_int = 0 + 18;
pub const GMT6: c_int = 0 + 19;
pub const GMT7: c_int = 0 + 20;
pub const GMT8: c_int = 0 + 21;
pub const GMT9: c_int = 0 + 22;
pub const GMT10: c_int = 0 + 23;
pub const GMT11: c_int = 0 + 24;
pub const GMT12: c_int = 0 + 25;
pub const GMT13: c_int = 0 + 26;
pub const WET: c_int = GMT0;
pub const WEST: c_int = GMT1;
pub const CET: c_int = GMT1;
pub const CEST: c_int = GMT2;
pub const EET: c_int = GMT2;
pub const EEST: c_int = GMT3;
pub const MSK: c_int = GMT3;
pub const MSD: c_int = GMT4;
pub const AST: c_int = GMT_4;
pub const ADT: c_int = GMT_3;
pub const EST: c_int = GMT_5;
pub const EDT: c_int = GMT_4;
pub const CST: c_int = GMT_6;
pub const CDT: c_int = GMT_5;
pub const MST: c_int = GMT_7;
pub const MDT: c_int = GMT_6;
pub const PST: c_int = GMT_8;
pub const PDT: c_int = GMT_7;
pub const HST: c_int = GMT_10;
pub const AKST: c_int = GMT_9;
pub const AKDT: c_int = GMT_8;
pub const A_WST: c_int = GMT8;
pub const A_CST: c_int = GMT13 + 1;
pub const A_EST: c_int = GMT10;
pub const A_ESST: c_int = GMT11;
pub const NZST: c_int = GMT12;
pub const NZDT: c_int = GMT13;
pub const UTC: c_int = GMT0;
//  ENUM: Calendar
pub const Gregorian: c_int = 0;
pub const Julian: c_int = 0 + 1;
//  ENUM: Country
pub const Country_Unknown: c_int = 0;
pub const Country_Default: c_int = 0 + 1;
pub const Country_WesternEurope_Start: c_int = 0 + 2;
pub const Country_EEC: c_int = Country_WesternEurope_Start;
pub const France: c_int = Country_WesternEurope_Start + 1;
pub const Germany: c_int = Country_WesternEurope_Start + 2;
pub const UK: c_int = Country_WesternEurope_Start + 3;
pub const Country_WesternEurope_End: c_int = UK;
pub const Russia: c_int = UK + 1;
pub const USA: c_int = UK + 2;
//  ENUM: Month
pub const Jan: c_int = 0;
pub const Feb: c_int = 0 + 1;
pub const Mar: c_int = 0 + 2;
pub const Apr: c_int = 0 + 3;
pub const May: c_int = 0 + 4;
pub const Jun: c_int = 0 + 5;
pub const Jul: c_int = 0 + 6;
pub const Aug: c_int = 0 + 7;
pub const Sep: c_int = 0 + 8;
pub const Oct: c_int = 0 + 9;
pub const Nov: c_int = 0 + 10;
pub const Dec: c_int = 0 + 11;
pub const Inv_Month: c_int = 0 + 12;
//  ENUM: WeekDay
pub const Sun: c_int = 0;
pub const Mon: c_int = 0 + 1;
pub const Tue: c_int = 0 + 2;
pub const Wed: c_int = 0 + 3;
pub const Thu: c_int = 0 + 4;
pub const Fri: c_int = 0 + 5;
pub const Sat: c_int = 0 + 6;
pub const Inv_WeekDay: c_int = 0 + 7;
//  ENUM: Year
//  SKIP: Inv_Year
//  ENUM: NameFlags
pub const Name_Full: c_int = 0x01;
pub const Name_Abbr: c_int = 0x02;
//  ENUM: WeekFlags
pub const Default_First: c_int = 0;
pub const Monday_First: c_int = 0 + 1;
pub const Sunday_First: c_int = 0 + 2;

//  ENUM: @15
pub const Event_Skip: c_int = -1;
pub const Event_Ignore: c_int = 0;
pub const Event_Processed: c_int = 1;

//  ENUM: wxRichTextOddEvenPage
pub const wxRICHTEXT_PAGE_ODD: c_int = 0;
pub const wxRICHTEXT_PAGE_EVEN: c_int = 0 + 1;
pub const wxRICHTEXT_PAGE_ALL: c_int = 0 + 2;
//  ENUM: wxRichTextPageLocation
pub const wxRICHTEXT_PAGE_LEFT: c_int = 0;
pub const wxRICHTEXT_PAGE_CENTRE: c_int = 0 + 1;
pub const wxRICHTEXT_PAGE_RIGHT: c_int = 0 + 2;

pub const wxRICHTEXT_FORMAT_STYLE_EDITOR: c_int = 0x0001;
pub const wxRICHTEXT_FORMAT_FONT: c_int = 0x0002;
pub const wxRICHTEXT_FORMAT_TABS: c_int = 0x0004;
pub const wxRICHTEXT_FORMAT_BULLETS: c_int = 0x0008;
pub const wxRICHTEXT_FORMAT_INDENTS_SPACING: c_int = 0x0010;

//  ENUM: @1
pub const typeCaption: c_int = 0;
pub const typeGripper: c_int = 0 + 1;
pub const typeDock: c_int = 0 + 2;
pub const typeDockSizer: c_int = 0 + 3;
pub const typePane: c_int = 0 + 4;
pub const typePaneSizer: c_int = 0 + 5;
pub const typeBackground: c_int = 0 + 6;
pub const typePaneBorder: c_int = 0 + 7;
pub const typePaneButton: c_int = 0 + 8;

pub const wxIMAGE_OPTION_QUALITY: &str = "quality";
pub const wxIMAGE_OPTION_FILENAME: &str = "FileName";
pub const wxIMAGE_OPTION_RESOLUTION: &str = "Resolution";
pub const wxIMAGE_OPTION_RESOLUTIONX: &str = "ResolutionX";
pub const wxIMAGE_OPTION_RESOLUTIONY: &str = "ResolutionY";
pub const wxIMAGE_OPTION_RESOLUTIONUNIT: &str = "ResolutionUnit";
pub const wxIMAGE_OPTION_MAX_WIDTH: &str = "MaxWidth";
pub const wxIMAGE_OPTION_MAX_HEIGHT: &str = "MaxHeight";
pub const wxIMAGE_OPTION_ORIGINAL_WIDTH: &str = "OriginalWidth";
pub const wxIMAGE_OPTION_ORIGINAL_HEIGHT: &str = "OriginalHeight";
pub const wxIMAGE_OPTION_BMP_FORMAT: &str = "wxBMP_FORMAT";
pub const wxIMAGE_OPTION_CUR_HOTSPOT_X: &str = "HotSpotX";
pub const wxIMAGE_OPTION_CUR_HOTSPOT_Y: &str = "HotSpotY";
pub const wxIMAGE_OPTION_GIF_COMMENT: &str = "GifComment";
//  ENUM: wxImageResolution
pub const wxIMAGE_RESOLUTION_NONE: c_int = 0;
pub const wxIMAGE_RESOLUTION_INCHES: c_int = 1;
pub const wxIMAGE_RESOLUTION_CM: c_int = 2;
//  ENUM: wxImageResizeQuality
pub const wxIMAGE_QUALITY_NEAREST: c_int = 0;
pub const wxIMAGE_QUALITY_BILINEAR: c_int = 0 + 1;
pub const wxIMAGE_QUALITY_BICUBIC: c_int = 0 + 2;
pub const wxIMAGE_QUALITY_BOX_AVERAGE: c_int = 0 + 3;
pub const wxIMAGE_QUALITY_NORMAL: c_int = 0 + 4;
pub const wxIMAGE_QUALITY_HIGH: c_int = 0 + 5;
//  ENUM: wxImagePNGType
pub const wxPNG_TYPE_COLOUR: c_int = 0;
pub const wxPNG_TYPE_GREY: c_int = 2;
pub const wxPNG_TYPE_GREY_RED: c_int = 3;
pub const wxPNG_TYPE_PALETTE: c_int = 4;
//  ENUM: @31
pub const wxBMP_24BPP: c_int = 24;
pub const wxBMP_8BPP: c_int =  8;
pub const wxBMP_8BPP_GREY: c_int =  9;
pub const wxBMP_8BPP_GRAY: c_int =  wxBMP_8BPP_GREY;
pub const wxBMP_8BPP_RED: c_int = 10;
pub const wxBMP_8BPP_PALETTE: c_int = 11;
pub const wxBMP_4BPP: c_int =  4;
pub const wxBMP_1BPP: c_int =  1;
pub const wxBMP_1BPP_BW: c_int =  2;

//  ENUM: Kind
pub const Kind_General: c_int = 0;
pub const Kind_Advanced: c_int = 0 + 1;

//  ENUM: wxSystemFont
pub const wxSYS_OEM_FIXED_FONT: c_int = 10;
pub const wxSYS_ANSI_FIXED_FONT: c_int = 10 + 1;
pub const wxSYS_ANSI_VAR_FONT: c_int = 10 + 2;
pub const wxSYS_SYSTEM_FONT: c_int = 10 + 3;
pub const wxSYS_DEVICE_DEFAULT_FONT: c_int = 10 + 4;
pub const wxSYS_DEFAULT_GUI_FONT: c_int = 10 + 5;
//  ENUM: wxSystemColour
pub const wxSYS_COLOUR_SCROLLBAR: c_int = 0;
pub const wxSYS_COLOUR_DESKTOP: c_int = 0 + 1;
pub const wxSYS_COLOUR_ACTIVECAPTION: c_int = 0 + 2;
pub const wxSYS_COLOUR_INACTIVECAPTION: c_int = 0 + 3;
pub const wxSYS_COLOUR_MENU: c_int = 0 + 4;
pub const wxSYS_COLOUR_WINDOW: c_int = 0 + 5;
pub const wxSYS_COLOUR_WINDOWFRAME: c_int = 0 + 6;
pub const wxSYS_COLOUR_MENUTEXT: c_int = 0 + 7;
pub const wxSYS_COLOUR_WINDOWTEXT: c_int = 0 + 8;
pub const wxSYS_COLOUR_CAPTIONTEXT: c_int = 0 + 9;
pub const wxSYS_COLOUR_ACTIVEBORDER: c_int = 0 + 10;
pub const wxSYS_COLOUR_INACTIVEBORDER: c_int = 0 + 11;
pub const wxSYS_COLOUR_APPWORKSPACE: c_int = 0 + 12;
pub const wxSYS_COLOUR_HIGHLIGHT: c_int = 0 + 13;
pub const wxSYS_COLOUR_HIGHLIGHTTEXT: c_int = 0 + 14;
pub const wxSYS_COLOUR_BTNFACE: c_int = 0 + 15;
pub const wxSYS_COLOUR_BTNSHADOW: c_int = 0 + 16;
pub const wxSYS_COLOUR_GRAYTEXT: c_int = 0 + 17;
pub const wxSYS_COLOUR_BTNTEXT: c_int = 0 + 18;
pub const wxSYS_COLOUR_INACTIVECAPTIONTEXT: c_int = 0 + 19;
pub const wxSYS_COLOUR_BTNHIGHLIGHT: c_int = 0 + 20;
pub const wxSYS_COLOUR_3DDKSHADOW: c_int = 0 + 21;
pub const wxSYS_COLOUR_3DLIGHT: c_int = 0 + 22;
pub const wxSYS_COLOUR_INFOTEXT: c_int = 0 + 23;
pub const wxSYS_COLOUR_INFOBK: c_int = 0 + 24;
pub const wxSYS_COLOUR_LISTBOX: c_int = 0 + 25;
pub const wxSYS_COLOUR_HOTLIGHT: c_int = 0 + 26;
pub const wxSYS_COLOUR_GRADIENTACTIVECAPTION: c_int = 0 + 27;
pub const wxSYS_COLOUR_GRADIENTINACTIVECAPTION: c_int = 0 + 28;
pub const wxSYS_COLOUR_MENUHILIGHT: c_int = 0 + 29;
pub const wxSYS_COLOUR_MENUBAR: c_int = 0 + 30;
pub const wxSYS_COLOUR_LISTBOXTEXT: c_int = 0 + 31;
pub const wxSYS_COLOUR_LISTBOXHIGHLIGHTTEXT: c_int = 0 + 32;
pub const wxSYS_COLOUR_BACKGROUND: c_int = wxSYS_COLOUR_DESKTOP;
pub const wxSYS_COLOUR_3DFACE: c_int = wxSYS_COLOUR_BTNFACE;
pub const wxSYS_COLOUR_3DSHADOW: c_int = wxSYS_COLOUR_BTNSHADOW;
pub const wxSYS_COLOUR_BTNHILIGHT: c_int = wxSYS_COLOUR_BTNHIGHLIGHT;
pub const wxSYS_COLOUR_3DHIGHLIGHT: c_int = wxSYS_COLOUR_BTNHIGHLIGHT;
pub const wxSYS_COLOUR_3DHILIGHT: c_int = wxSYS_COLOUR_BTNHIGHLIGHT;
pub const wxSYS_COLOUR_FRAMEBK: c_int = wxSYS_COLOUR_BTNFACE;
//  ENUM: wxSystemMetric
pub const wxSYS_MOUSE_BUTTONS: c_int = 0;
pub const wxSYS_BORDER_X: c_int = 0 + 1;
pub const wxSYS_BORDER_Y: c_int = 0 + 2;
pub const wxSYS_CURSOR_X: c_int = 0 + 3;
pub const wxSYS_CURSOR_Y: c_int = 0 + 4;
pub const wxSYS_DCLICK_X: c_int = 0 + 5;
pub const wxSYS_DCLICK_Y: c_int = 0 + 6;
pub const wxSYS_DRAG_X: c_int = 0 + 7;
pub const wxSYS_DRAG_Y: c_int = 0 + 8;
pub const wxSYS_EDGE_X: c_int = 0 + 9;
pub const wxSYS_EDGE_Y: c_int = 0 + 10;
pub const wxSYS_HSCROLL_ARROW_X: c_int = 0 + 11;
pub const wxSYS_HSCROLL_ARROW_Y: c_int = 0 + 12;
pub const wxSYS_HTHUMB_X: c_int = 0 + 13;
pub const wxSYS_ICON_X: c_int = 0 + 14;
pub const wxSYS_ICON_Y: c_int = 0 + 15;
pub const wxSYS_ICONSPACING_X: c_int = 0 + 16;
pub const wxSYS_ICONSPACING_Y: c_int = 0 + 17;
pub const wxSYS_WINDOWMIN_X: c_int = 0 + 18;
pub const wxSYS_WINDOWMIN_Y: c_int = 0 + 19;
pub const wxSYS_SCREEN_X: c_int = 0 + 20;
pub const wxSYS_SCREEN_Y: c_int = 0 + 21;
pub const wxSYS_FRAMESIZE_X: c_int = 0 + 22;
pub const wxSYS_FRAMESIZE_Y: c_int = 0 + 23;
pub const wxSYS_SMALLICON_X: c_int = 0 + 24;
pub const wxSYS_SMALLICON_Y: c_int = 0 + 25;
pub const wxSYS_HSCROLL_Y: c_int = 0 + 26;
pub const wxSYS_VSCROLL_X: c_int = 0 + 27;
pub const wxSYS_VSCROLL_ARROW_X: c_int = 0 + 28;
pub const wxSYS_VSCROLL_ARROW_Y: c_int = 0 + 29;
pub const wxSYS_VTHUMB_Y: c_int = 0 + 30;
pub const wxSYS_CAPTION_Y: c_int = 0 + 31;
pub const wxSYS_MENU_Y: c_int = 0 + 32;
pub const wxSYS_NETWORK_PRESENT: c_int = 0 + 33;
pub const wxSYS_PENWINDOWS_PRESENT: c_int = 0 + 34;
pub const wxSYS_SHOW_SOUNDS: c_int = 0 + 35;
pub const wxSYS_SWAP_BUTTONS: c_int = 0 + 36;
pub const wxSYS_DCLICK_MSEC: c_int = 0 + 37;
//  ENUM: wxSystemFeature
pub const wxSYS_CAN_DRAW_FRAME_DECORATIONS: c_int = 1;
pub const wxSYS_CAN_ICONIZE_FRAME: c_int = 1 + 1;
pub const wxSYS_TABLET_PRESENT: c_int = 1 + 2;
//  ENUM: wxSystemScreenType
pub const wxSYS_SCREEN_NONE: c_int = 0;
pub const wxSYS_SCREEN_TINY: c_int = 0 + 1;
pub const wxSYS_SCREEN_PDA: c_int = 0 + 2;
pub const wxSYS_SCREEN_SMALL: c_int = 0 + 3;
pub const wxSYS_SCREEN_DESKTOP: c_int = 0 + 4;

// NODEF: wxDEBUG_LEVEL
// NODEF: __WXDEBUG__
// NODEF: wxASSERT
// NODEF: wxASSERT_LEVEL_2
// NODEF: wxASSERT_LEVEL_2_MSG
// NODEF: wxASSERT_MIN_BITSIZE
// NODEF: wxASSERT_MSG
// NODEF: wxCHECK
// NODEF: wxCHECK_MSG
// NODEF: wxCHECK_RET
// NODEF: wxCHECK2
// NODEF: wxCHECK2_MSG
// NODEF: wxCOMPILE_TIME_ASSERT
// NODEF: wxCOMPILE_TIME_ASSERT2
//  SKIP: wxDISABLE_ASSERTS_IN_RELEASE_BUILD
// NODEF: wxFAIL
// NODEF: wxFAIL_MSG

//  ENUM: @11
pub const wxDIRCTRL_DIR_ONLY: c_int = 0x0010;
pub const wxDIRCTRL_SELECT_FIRST: c_int = 0x0020;
pub const wxDIRCTRL_SHOW_FILTERS: c_int = 0x0040;
pub const wxDIRCTRL_3D_INTERNAL: c_int = 0x0080;
pub const wxDIRCTRL_EDIT_LABELS: c_int = 0x0100;
pub const wxDIRCTRL_MULTIPLE: c_int = 0x0200;

//  ENUM: wxDirTraverseResult
pub const wxDIR_IGNORE: c_int = -1;
pub const wxDIR_STOP: c_int = -1 + 1;
pub const wxDIR_CONTINUE: c_int = -1 + 2;
//  ENUM: wxDirFlags
pub const wxDIR_FILES: c_int = 0x0001;
pub const wxDIR_DIRS: c_int = 0x0002;
pub const wxDIR_HIDDEN: c_int = 0x0004;
pub const wxDIR_DOTDOT: c_int = 0x0008;
pub const wxDIR_NO_FOLLOW: c_int = 0x0010;
pub const wxDIR_DEFAULT: c_int = wxDIR_FILES | wxDIR_DIRS | wxDIR_HIDDEN;

pub const wxBUFFER_VIRTUAL_AREA: c_int = 0x01;
pub const wxBUFFER_CLIENT_AREA: c_int = 0x02;
pub const wxBUFFER_USES_SHARED_BUFFER: c_int = 0x04;

//  ENUM: wxEOL
pub const wxEOL_NATIVE: c_int = 0;
pub const wxEOL_UNIX: c_int = 0 + 1;
pub const wxEOL_MAC: c_int = 0 + 2;
pub const wxEOL_DOS: c_int = 0 + 3;

//  ENUM: wxURLError
pub const wxURL_NOERR: c_int = 0;
pub const wxURL_SNTXERR: c_int = 0 + 1;
pub const wxURL_NOPROTO: c_int = 0 + 2;
pub const wxURL_NOHOST: c_int = 0 + 3;
pub const wxURL_NOPATH: c_int = 0 + 4;
pub const wxURL_CONNERR: c_int = 0 + 5;
pub const wxURL_PROTOERR: c_int = 0 + 6;

//  ENUM: wxTextFileType
pub const wxTextFileType_None: c_int = 0;
pub const wxTextFileType_Unix: c_int = 0 + 1;
pub const wxTextFileType_Dos: c_int = 0 + 2;
pub const wxTextFileType_Mac: c_int = 0 + 3;
pub const wxTextFileType_Os2: c_int = 0 + 4;

//  ENUM: wxPowerType
pub const wxPOWER_SOCKET: c_int = 0;
pub const wxPOWER_BATTERY: c_int = 0 + 1;
pub const wxPOWER_UNKNOWN: c_int = 0 + 2;
//  ENUM: wxBatteryState
pub const wxBATTERY_NORMAL_STATE: c_int = 0;
pub const wxBATTERY_LOW_STATE: c_int = 0 + 1;
pub const wxBATTERY_CRITICAL_STATE: c_int = 0 + 2;
pub const wxBATTERY_SHUTDOWN_STATE: c_int = 0 + 3;
pub const wxBATTERY_UNKNOWN_STATE: c_int = 0 + 4;

pub const wxXML_NO_INDENTATION: c_int = (-1);
//  ENUM: wxXmlNodeType
pub const wxXML_ELEMENT_NODE: c_int =  1;
pub const wxXML_ATTRIBUTE_NODE: c_int =  2;
pub const wxXML_TEXT_NODE: c_int =  3;
pub const wxXML_CDATA_SECTION_NODE: c_int =  4;
pub const wxXML_ENTITY_REF_NODE: c_int =  5;
pub const wxXML_ENTITY_NODE: c_int =  6;
pub const wxXML_PI_NODE: c_int =  7;
pub const wxXML_COMMENT_NODE: c_int =  8;
pub const wxXML_DOCUMENT_NODE: c_int =  9;
pub const wxXML_DOCUMENT_TYPE_NODE: c_int = 10;
pub const wxXML_DOCUMENT_FRAG_NODE: c_int = 11;
pub const wxXML_NOTATION_NODE: c_int = 12;
pub const wxXML_HTML_DOCUMENT_NODE: c_int = 13;
//  ENUM: wxXmlDocumentLoadFlag
pub const wxXMLDOC_NONE: c_int = 0;
pub const wxXMLDOC_KEEP_WHITESPACE_NODES: c_int = 0 + 1;

pub const wxLC_VRULES: c_int = 0x0001;
pub const wxLC_HRULES: c_int = 0x0002;
pub const wxLC_ICON: c_int = 0x0004;
pub const wxLC_SMALL_ICON: c_int = 0x0008;
pub const wxLC_LIST: c_int = 0x0010;
pub const wxLC_REPORT: c_int = 0x0020;
pub const wxLC_ALIGN_TOP: c_int = 0x0040;
pub const wxLC_ALIGN_LEFT: c_int = 0x0080;
pub const wxLC_AUTOARRANGE: c_int = 0x0100;
pub const wxLC_VIRTUAL: c_int = 0x0200;
pub const wxLC_EDIT_LABELS: c_int = 0x0400;
pub const wxLC_NO_HEADER: c_int = 0x0800;
pub const wxLC_NO_SORT_HEADER: c_int = 0x1000;
pub const wxLC_SINGLE_SEL: c_int = 0x2000;
pub const wxLC_SORT_ASCENDING: c_int = 0x4000;
pub const wxLC_SORT_DESCENDING: c_int = 0x8000;
pub const wxLC_MASK_TYPE: c_int = (wxLC_ICON | wxLC_SMALL_ICON | wxLC_LIST | wxLC_REPORT);
pub const wxLC_MASK_ALIGN: c_int = (wxLC_ALIGN_TOP | wxLC_ALIGN_LEFT);
pub const wxLC_MASK_SORT: c_int = (wxLC_SORT_ASCENDING | wxLC_SORT_DESCENDING);
pub const wxLIST_MASK_STATE: c_int = 0x0001;
pub const wxLIST_MASK_TEXT: c_int = 0x0002;
pub const wxLIST_MASK_IMAGE: c_int = 0x0004;
pub const wxLIST_MASK_DATA: c_int = 0x0008;
pub const wxLIST_SET_ITEM: c_int = 0x0010;
pub const wxLIST_MASK_WIDTH: c_int = 0x0020;
pub const wxLIST_MASK_FORMAT: c_int = 0x0040;
pub const wxLIST_STATE_DONTCARE: c_int = 0x0000;
pub const wxLIST_STATE_DROPHILITED: c_int = 0x0001;
pub const wxLIST_STATE_FOCUSED: c_int = 0x0002;
pub const wxLIST_STATE_SELECTED: c_int = 0x0004;
pub const wxLIST_STATE_CUT: c_int = 0x0008;
pub const wxLIST_STATE_DISABLED: c_int = 0x0010;
pub const wxLIST_STATE_FILTERED: c_int = 0x0020;
pub const wxLIST_STATE_INUSE: c_int = 0x0040;
pub const wxLIST_STATE_PICKED: c_int = 0x0080;
pub const wxLIST_STATE_SOURCE: c_int = 0x0100;
pub const wxLIST_HITTEST_ABOVE: c_int = 0x0001;
pub const wxLIST_HITTEST_BELOW: c_int = 0x0002;
pub const wxLIST_HITTEST_NOWHERE: c_int = 0x0004;
pub const wxLIST_HITTEST_ONITEMICON: c_int = 0x0020;
pub const wxLIST_HITTEST_ONITEMLABEL: c_int = 0x0080;
pub const wxLIST_HITTEST_ONITEMRIGHT: c_int = 0x0100;
pub const wxLIST_HITTEST_ONITEMSTATEICON: c_int = 0x0200;
pub const wxLIST_HITTEST_TOLEFT: c_int = 0x0400;
pub const wxLIST_HITTEST_TORIGHT: c_int = 0x0800;
pub const wxLIST_HITTEST_ONITEM: c_int = (wxLIST_HITTEST_ONITEMICON | wxLIST_HITTEST_ONITEMLABEL | wxLIST_HITTEST_ONITEMSTATEICON);
pub const wxLIST_GETSUBITEMRECT_WHOLEITEM: c_int = -1;
//  ENUM: @33
pub const wxLIST_NEXT_ABOVE: c_int = 0;
pub const wxLIST_NEXT_ALL: c_int = 0 + 1;
pub const wxLIST_NEXT_BELOW: c_int = 0 + 2;
pub const wxLIST_NEXT_LEFT: c_int = 0 + 3;
pub const wxLIST_NEXT_RIGHT: c_int = 0 + 4;
//  ENUM: @34
pub const wxLIST_ALIGN_DEFAULT: c_int = 0;
pub const wxLIST_ALIGN_LEFT: c_int = 0 + 1;
pub const wxLIST_ALIGN_TOP: c_int = 0 + 2;
pub const wxLIST_ALIGN_SNAP_TO_GRID: c_int = 0 + 3;
//  ENUM: wxListColumnFormat
pub const wxLIST_FORMAT_LEFT: c_int = 0;
pub const wxLIST_FORMAT_RIGHT: c_int = 0 + 1;
pub const wxLIST_FORMAT_CENTRE: c_int = 0 + 2;
pub const wxLIST_FORMAT_CENTER: c_int = wxLIST_FORMAT_CENTRE;
//  ENUM: @35
pub const wxLIST_AUTOSIZE: c_int = -1;
pub const wxLIST_AUTOSIZE_USEHEADER: c_int = -2;
//  ENUM: @36
pub const wxLIST_RECT_BOUNDS: c_int = 0;
pub const wxLIST_RECT_ICON: c_int = 0 + 1;
pub const wxLIST_RECT_LABEL: c_int = 0 + 2;
//  ENUM: @37
pub const wxLIST_FIND_UP: c_int = 0;
pub const wxLIST_FIND_DOWN: c_int = 0 + 1;
pub const wxLIST_FIND_LEFT: c_int = 0 + 2;
pub const wxLIST_FIND_RIGHT: c_int = 0 + 3;

//  SKIP: wxTreeListEventHandler
//  ENUM: @53
pub const wxTL_SINGLE: c_int = 0x0000;
pub const wxTL_MULTIPLE: c_int = 0x0001;
pub const wxTL_CHECKBOX: c_int = 0x0002;
pub const wxTL_3STATE: c_int = 0x0004;
pub const wxTL_USER_3STATE: c_int = 0x0008;
pub const wxTL_NO_HEADER: c_int = 0x0010;
pub const wxTL_DEFAULT_STYLE: c_int = wxTL_SINGLE;
pub const wxTL_STYLE_MASK: c_int = wxTL_SINGLE |
                          wxTL_MULTIPLE |
                          wxTL_CHECKBOX |
                          wxTL_3STATE |
                          wxTL_USER_3STATE;

//  ENUM: @6
pub const NUM_CUSTOM: c_int = 16;

//  ENUM: AccessMode
pub const Read: c_int = 0;
pub const Write: c_int = 0 + 1;
//  ENUM: StdKey
pub const HKCR: c_int = 0;
pub const HKCU: c_int = 0 + 1;
pub const HKLM: c_int = 0 + 2;
pub const HKUSR: c_int = 0 + 3;
pub const HKPD: c_int = 0 + 4;
pub const HKCC: c_int = 0 + 5;
pub const HKDD: c_int = 0 + 6;
pub const HKMAX: c_int = 0 + 7;
//  ENUM: ValueType
pub const Type_None: c_int = 0;
pub const Type_Expand_String: c_int = 0 + 1;
pub const Type_Binary: c_int = 0 + 2;
pub const Type_Dword: c_int = 0 + 3;
pub const Type_Dword_little_endian: c_int = 0 + 4;
pub const Type_Dword_big_endian: c_int = 0 + 5;
pub const Type_Link: c_int = 0 + 6;
pub const Type_Multi_String: c_int = 0 + 7;
pub const Type_Resource_list: c_int = 0 + 8;
pub const Type_Full_resource_descriptor: c_int = 0 + 9;
pub const Type_Resource_requirements_list: c_int = 0 + 10;
//  ENUM: WOW64ViewMode
pub const WOW64ViewMode_Default: c_int = 0;
pub const WOW64ViewMode_32: c_int = 0 + 1;
pub const WOW64ViewMode_64: c_int = 0 + 2;

//  ENUM: wxAuiToolBarStyle
//  ENUM: wxAuiToolBarArtSetting
//  ENUM: wxAuiToolBarToolTextOrientation

//  ENUM: @27
pub const wxPAGE_ODD: c_int = 0;
pub const wxPAGE_EVEN: c_int = 0 + 1;
pub const wxPAGE_ALL: c_int = 0 + 2;

pub const wxHTML_ALIGN_LEFT: c_int = 0x0000;
pub const wxHTML_ALIGN_RIGHT: c_int = 0x0002;
pub const wxHTML_ALIGN_JUSTIFY: c_int = 0x0010;
pub const wxHTML_ALIGN_TOP: c_int = 0x0004;
pub const wxHTML_ALIGN_BOTTOM: c_int = 0x0008;
pub const wxHTML_ALIGN_CENTER: c_int = 0x0001;
pub const wxHTML_CLR_FOREGROUND: c_int = 0x0001;
pub const wxHTML_CLR_BACKGROUND: c_int = 0x0002;
pub const wxHTML_CLR_TRANSPARENT_BACKGROUND: c_int = 0x0004;
pub const wxHTML_UNITS_PIXELS: c_int = 0x0001;
pub const wxHTML_UNITS_PERCENT: c_int = 0x0002;
pub const wxHTML_INDENT_LEFT: c_int = 0x0010;
pub const wxHTML_INDENT_RIGHT: c_int = 0x0020;
pub const wxHTML_INDENT_TOP: c_int = 0x0040;
pub const wxHTML_INDENT_BOTTOM: c_int = 0x0080;
pub const wxHTML_INDENT_HORIZONTAL: c_int = (wxHTML_INDENT_LEFT | wxHTML_INDENT_RIGHT);
pub const wxHTML_INDENT_VERTICAL: c_int = (wxHTML_INDENT_TOP | wxHTML_INDENT_BOTTOM);
pub const wxHTML_INDENT_ALL: c_int = (wxHTML_INDENT_VERTICAL | wxHTML_INDENT_HORIZONTAL);
pub const wxHTML_COND_ISANCHOR: c_int = 1;
pub const wxHTML_COND_ISIMAGEMAP: c_int = 2;
pub const wxHTML_COND_USER: c_int = 10000;

//  ENUM: ResourceCat
pub const ResourceCat_None: c_int = 0;
pub const ResourceCat_Messages: c_int = 0 + 1;

//  ENUM: wxPenStyle
pub const wxPENSTYLE_INVALID: c_int = -1;
pub const wxPENSTYLE_SOLID: c_int = -1 + 1;
pub const wxPENSTYLE_DOT: c_int = -1 + 2;
pub const wxPENSTYLE_LONG_DASH: c_int = -1 + 3;
pub const wxPENSTYLE_SHORT_DASH: c_int = -1 + 4;
pub const wxPENSTYLE_DOT_DASH: c_int = -1 + 5;
pub const wxPENSTYLE_USER_DASH: c_int = -1 + 6;
pub const wxPENSTYLE_TRANSPARENT: c_int = -1 + 7;
pub const wxPENSTYLE_STIPPLE_MASK_OPAQUE: c_int = -1 + 8;
pub const wxPENSTYLE_STIPPLE_MASK: c_int = -1 + 9;
pub const wxPENSTYLE_STIPPLE: c_int = -1 + 10;
pub const wxPENSTYLE_BDIAGONAL_HATCH: c_int = -1 + 11;
pub const wxPENSTYLE_CROSSDIAG_HATCH: c_int = -1 + 12;
pub const wxPENSTYLE_FDIAGONAL_HATCH: c_int = -1 + 13;
pub const wxPENSTYLE_CROSS_HATCH: c_int = -1 + 14;
pub const wxPENSTYLE_HORIZONTAL_HATCH: c_int = -1 + 15;
pub const wxPENSTYLE_VERTICAL_HATCH: c_int = -1 + 16;
pub const wxPENSTYLE_FIRST_HATCH: c_int = -1 + 17;
pub const wxPENSTYLE_LAST_HATCH: c_int = -1 + 18;
//  ENUM: wxPenJoin
pub const wxJOIN_INVALID: c_int = -1;
pub const wxJOIN_BEVEL: c_int = 120;
pub const wxJOIN_MITER: c_int = 120 + 1;
pub const wxJOIN_ROUND: c_int = 120 + 2;
//  ENUM: wxPenCap
pub const wxCAP_INVALID: c_int = -1;
pub const wxCAP_ROUND: c_int = 130;
pub const wxCAP_PROJECTING: c_int = 130 + 1;
pub const wxCAP_BUTT: c_int = 130 + 2;

pub const wxPG_DEFAULT_STYLE: c_int = (0);
pub const wxPGMAN_DEFAULT_STYLE: c_int = (0);
//  ENUM: wxPG_WINDOW_STYLES
pub const wxPG_AUTO_SORT: c_int = 0x00000010;
pub const wxPG_HIDE_CATEGORIES: c_int = 0x00000020;
pub const wxPG_ALPHABETIC_MODE: c_int = (wxPG_HIDE_CATEGORIES|wxPG_AUTO_SORT);
pub const wxPG_BOLD_MODIFIED: c_int = 0x00000040;
pub const wxPG_SPLITTER_AUTO_CENTER: c_int = 0x00000080;
pub const wxPG_TOOLTIPS: c_int = 0x00000100;
pub const wxPG_HIDE_MARGIN: c_int = 0x00000200;
pub const wxPG_STATIC_SPLITTER: c_int = 0x00000400;
pub const wxPG_STATIC_LAYOUT: c_int = (wxPG_HIDE_MARGIN|wxPG_STATIC_SPLITTER);
pub const wxPG_LIMITED_EDITING: c_int = 0x00000800;
pub const wxPG_TOOLBAR: c_int = 0x00001000;
pub const wxPG_DESCRIPTION: c_int = 0x00002000;
pub const wxPG_NO_INTERNAL_BORDER: c_int = 0x00004000;
//  ENUM: wxPG_EX_WINDOW_STYLES
pub const wxPG_EX_INIT_NOCAT: c_int = 0x00001000;
pub const wxPG_EX_NO_FLAT_TOOLBAR: c_int = 0x00002000;
pub const wxPG_EX_MODE_BUTTONS: c_int = 0x00008000;
pub const wxPG_EX_HELP_AS_TOOLTIPS: c_int = 0x00010000;
pub const wxPG_EX_NATIVE_DOUBLE_BUFFERING: c_int = 0x00080000;
pub const wxPG_EX_AUTO_UNSPECIFIED_VALUES: c_int = 0x00200000;
pub const wxPG_EX_WRITEONLY_BUILTIN_ATTRIBUTES: c_int = 0x00400000;
pub const wxPG_EX_HIDE_PAGE_BUTTONS: c_int = 0x01000000;
pub const wxPG_EX_MULTIPLE_SELECTION: c_int = 0x02000000;
pub const wxPG_EX_ENABLE_TLP_TRACKING: c_int = 0x04000000;
pub const wxPG_EX_NO_TOOLBAR_DIVIDER: c_int = 0x04000000;
pub const wxPG_EX_TOOLBAR_SEPARATOR: c_int = 0x08000000;
//  ENUM: wxPG_VALIDATION_FAILURE_BEHAVIOR_FLAGS
pub const wxPG_VFB_STAY_IN_PROPERTY: c_int = 0x01;
pub const wxPG_VFB_BEEP: c_int = 0x02;
pub const wxPG_VFB_MARK_CELL: c_int = 0x04;
pub const wxPG_VFB_SHOW_MESSAGE: c_int = 0x08;
pub const wxPG_VFB_SHOW_MESSAGEBOX: c_int = 0x10;
pub const wxPG_VFB_SHOW_MESSAGE_ON_STATUSBAR: c_int = 0x20;
pub const wxPG_VFB_DEFAULT: c_int = wxPG_VFB_MARK_CELL |
                                      wxPG_VFB_SHOW_MESSAGEBOX;
//  ENUM: wxPG_KEYBOARD_ACTIONS
pub const wxPG_ACTION_INVALID: c_int = 0;
pub const wxPG_ACTION_NEXT_PROPERTY: c_int = 0 + 1;
pub const wxPG_ACTION_PREV_PROPERTY: c_int = 0 + 2;
pub const wxPG_ACTION_EXPAND_PROPERTY: c_int = 0 + 3;
pub const wxPG_ACTION_COLLAPSE_PROPERTY: c_int = 0 + 4;
pub const wxPG_ACTION_CANCEL_EDIT: c_int = 0 + 5;
pub const wxPG_ACTION_EDIT: c_int = 0 + 6;
pub const wxPG_ACTION_PRESS_BUTTON: c_int = 0 + 7;
pub const wxPG_ACTION_MAX: c_int = 0 + 8;

//  ENUM: wxFlexSizerGrowMode
pub const wxFLEX_GROWMODE_NONE: c_int = 0;
pub const wxFLEX_GROWMODE_SPECIFIED: c_int = 0 + 1;
pub const wxFLEX_GROWMODE_ALL: c_int = 0 + 2;

pub const wxSL_HORIZONTAL: c_int = wxHORIZONTAL /* 0x0004 */;
pub const wxSL_VERTICAL: c_int = wxVERTICAL   /* 0x0008 */;
pub const wxSL_TICKS: c_int = 0x0010;
pub const wxSL_AUTOTICKS: c_int = wxSL_TICKS;
pub const wxSL_LEFT: c_int = 0x0040;
pub const wxSL_TOP: c_int = 0x0080;
pub const wxSL_RIGHT: c_int = 0x0100;
pub const wxSL_BOTTOM: c_int = 0x0200;
pub const wxSL_BOTH: c_int = 0x0400;
pub const wxSL_SELRANGE: c_int = 0x0800;
pub const wxSL_INVERSE: c_int = 0x1000;
pub const wxSL_MIN_MAX_LABELS: c_int = 0x2000;
pub const wxSL_VALUE_LABEL: c_int = 0x4000;
pub const wxSL_LABELS: c_int = (wxSL_MIN_MAX_LABELS|wxSL_VALUE_LABEL);

//  ENUM: wxXmlResourceFlags
pub const wxXRC_USE_LOCALE: c_int = 1;
pub const wxXRC_NO_SUBCLASSING: c_int = 2;
pub const wxXRC_NO_RELOADING: c_int = 4;

//  ENUM: wxTextValidatorStyle
pub const wxFILTER_NONE: c_int = 0;
pub const wxFILTER_EMPTY: c_int = 0 + 1;
pub const wxFILTER_ASCII: c_int = 0 + 2;
pub const wxFILTER_ALPHA: c_int = 0 + 3;
pub const wxFILTER_ALPHANUMERIC: c_int = 0 + 4;
pub const wxFILTER_DIGITS: c_int = 0 + 5;
pub const wxFILTER_NUMERIC: c_int = 0 + 6;
pub const wxFILTER_INCLUDE_LIST: c_int = 0 + 7;
pub const wxFILTER_INCLUDE_CHAR_LIST: c_int = 0 + 8;
pub const wxFILTER_EXCLUDE_LIST: c_int = 0 + 9;
pub const wxFILTER_EXCLUDE_CHAR_LIST: c_int = 0 + 10;

pub const wxHW_SCROLLBAR_NEVER: c_int = 0x0002;
pub const wxHW_SCROLLBAR_AUTO: c_int = 0x0004;
pub const wxHW_NO_SELECTION: c_int = 0x0008;
pub const wxHW_DEFAULT_STYLE: c_int = wxHW_SCROLLBAR_AUTO;
//  ENUM: wxHtmlOpeningStatus
pub const wxHTML_OPEN: c_int = 0;
pub const wxHTML_BLOCK: c_int = 0 + 1;
pub const wxHTML_REDIRECT: c_int = 0 + 2;

//  ENUM: wxRibbonGalleryButtonState
pub const wxRIBBON_GALLERY_BUTTON_NORMAL: c_int = 0;
pub const wxRIBBON_GALLERY_BUTTON_HOVERED: c_int = 0 + 1;
pub const wxRIBBON_GALLERY_BUTTON_ACTIVE: c_int = 0 + 2;
pub const wxRIBBON_GALLERY_BUTTON_DISABLED: c_int = 0 + 3;

//  ENUM: EditableStateFlags
pub const SelectionState: c_int = 0x01;
pub const ExpandedState: c_int = 0x02;
pub const ScrollPosState: c_int = 0x04;
pub const PageState: c_int = 0x08;
pub const SplitterPosState: c_int = 0x10;
pub const DescBoxState: c_int = 0x20;
pub const AllStates: c_int = SelectionState |
                           ExpandedState |
                           ScrollPosState |
                           PageState |
                           SplitterPosState |
                           DescBoxState;

//  ENUM: wxGridSelectionModes
pub const wxGridSelectCells: c_int = 0;
pub const wxGridSelectRows: c_int = 0 + 1;
pub const wxGridSelectColumns: c_int = 0 + 2;
pub const wxGridSelectRowsOrColumns: c_int = 0 + 3;
//  ENUM: CellSpan
pub const CellSpan_Inside: c_int = -1;
pub const CellSpan_None: c_int = 0;
pub const CellSpan_Main: c_int = 0 + 1;
//  ENUM: TabBehaviour
pub const Tab_Stop: c_int = 0;
pub const Tab_Wrap: c_int = 0 + 1;
pub const Tab_Leave: c_int = 0 + 2;

// NODEF: wxBITMAP
// NODEF: wxBITMAP_PNG
// NODEF: wxBITMAP_PNG_FROM_DATA
// NODEF: wxICON
//  ENUM: wxBitmapType
pub const wxBITMAP_TYPE_INVALID: c_int = 0;
pub const wxBITMAP_TYPE_BMP: c_int = 0 + 1;
pub const wxBITMAP_TYPE_BMP_RESOURCE: c_int = 0 + 2;
pub const wxBITMAP_TYPE_RESOURCE: c_int = wxBITMAP_TYPE_BMP_RESOURCE;
pub const wxBITMAP_TYPE_ICO: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 1;
pub const wxBITMAP_TYPE_ICO_RESOURCE: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 2;
pub const wxBITMAP_TYPE_CUR: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 3;
pub const wxBITMAP_TYPE_CUR_RESOURCE: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 4;
pub const wxBITMAP_TYPE_XBM: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 5;
pub const wxBITMAP_TYPE_XBM_DATA: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 6;
pub const wxBITMAP_TYPE_XPM: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 7;
pub const wxBITMAP_TYPE_XPM_DATA: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 8;
pub const wxBITMAP_TYPE_TIFF: c_int = wxBITMAP_TYPE_BMP_RESOURCE + 9;
pub const wxBITMAP_TYPE_TIF: c_int = wxBITMAP_TYPE_TIFF;
pub const wxBITMAP_TYPE_TIFF_RESOURCE: c_int = wxBITMAP_TYPE_TIFF + 1;
pub const wxBITMAP_TYPE_TIF_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE;
pub const wxBITMAP_TYPE_GIF: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 1;
pub const wxBITMAP_TYPE_GIF_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 2;
pub const wxBITMAP_TYPE_PNG: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 3;
pub const wxBITMAP_TYPE_PNG_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 4;
pub const wxBITMAP_TYPE_JPEG: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 5;
pub const wxBITMAP_TYPE_JPEG_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 6;
pub const wxBITMAP_TYPE_PNM: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 7;
pub const wxBITMAP_TYPE_PNM_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 8;
pub const wxBITMAP_TYPE_PCX: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 9;
pub const wxBITMAP_TYPE_PCX_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 10;
pub const wxBITMAP_TYPE_PICT: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 11;
pub const wxBITMAP_TYPE_PICT_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 12;
pub const wxBITMAP_TYPE_ICON: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 13;
pub const wxBITMAP_TYPE_ICON_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 14;
pub const wxBITMAP_TYPE_ANI: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 15;
pub const wxBITMAP_TYPE_IFF: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 16;
pub const wxBITMAP_TYPE_TGA: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 17;
pub const wxBITMAP_TYPE_MACCURSOR: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 18;
pub const wxBITMAP_TYPE_MACCURSOR_RESOURCE: c_int = wxBITMAP_TYPE_TIFF_RESOURCE + 19;
pub const wxBITMAP_TYPE_ANY: c_int = 50;
//  ENUM: wxPolygonFillMode
pub const wxODDEVEN_RULE: c_int = 1;
pub const wxWINDING_RULE: c_int = 1 + 1;
//  ENUM: wxStockCursor
pub const wxCURSOR_NONE: c_int = 0;
pub const wxCURSOR_ARROW: c_int = 0 + 1;
pub const wxCURSOR_RIGHT_ARROW: c_int = 0 + 2;
pub const wxCURSOR_BULLSEYE: c_int = 0 + 3;
pub const wxCURSOR_CHAR: c_int = 0 + 4;
pub const wxCURSOR_CROSS: c_int = 0 + 5;
pub const wxCURSOR_HAND: c_int = 0 + 6;
pub const wxCURSOR_IBEAM: c_int = 0 + 7;
pub const wxCURSOR_LEFT_BUTTON: c_int = 0 + 8;
pub const wxCURSOR_MAGNIFIER: c_int = 0 + 9;
pub const wxCURSOR_MIDDLE_BUTTON: c_int = 0 + 10;
pub const wxCURSOR_NO_ENTRY: c_int = 0 + 11;
pub const wxCURSOR_PAINT_BRUSH: c_int = 0 + 12;
pub const wxCURSOR_PENCIL: c_int = 0 + 13;
pub const wxCURSOR_POINT_LEFT: c_int = 0 + 14;
pub const wxCURSOR_POINT_RIGHT: c_int = 0 + 15;
pub const wxCURSOR_QUESTION_ARROW: c_int = 0 + 16;
pub const wxCURSOR_RIGHT_BUTTON: c_int = 0 + 17;
pub const wxCURSOR_SIZENESW: c_int = 0 + 18;
pub const wxCURSOR_SIZENS: c_int = 0 + 19;
pub const wxCURSOR_SIZENWSE: c_int = 0 + 20;
pub const wxCURSOR_SIZEWE: c_int = 0 + 21;
pub const wxCURSOR_SIZING: c_int = 0 + 22;
pub const wxCURSOR_SPRAYCAN: c_int = 0 + 23;
pub const wxCURSOR_WAIT: c_int = 0 + 24;
pub const wxCURSOR_WATCH: c_int = 0 + 25;
pub const wxCURSOR_BLANK: c_int = 0 + 26;
pub const wxCURSOR_DEFAULT: c_int = 0 + 27;
pub const wxCURSOR_COPY_ARROW: c_int = 0 + 28;
pub const wxCURSOR_CROSS_REVERSE: c_int = 0 + 29;
pub const wxCURSOR_DOUBLE_ARROW: c_int = 0 + 30;
pub const wxCURSOR_BASED_ARROW_UP: c_int = 0 + 31;
pub const wxCURSOR_BASED_ARROW_DOWN: c_int = 0 + 32;
pub const wxCURSOR_ARROWWAIT: c_int = 0 + 33;
pub const wxCURSOR_MAX: c_int = 0 + 34;

pub const wxTBK_BUTTONBAR: c_int = 0x0100;
pub const wxTBK_HORZ_LAYOUT: c_int = 0x8000;

pub const wxRE_READONLY: c_int = 0x0010;
pub const wxRE_MULTILINE: c_int = 0x0020;
pub const wxRE_CENTRE_CARET: c_int = 0x8000;
pub const wxRE_CENTER_CARET: c_int = wxRE_CENTRE_CARET;
pub const wxRICHTEXT_SHIFT_DOWN: c_int = 0x01;
pub const wxRICHTEXT_CTRL_DOWN: c_int = 0x02;
pub const wxRICHTEXT_ALT_DOWN: c_int = 0x04;
pub const wxRICHTEXT_EX_NO_GUIDELINES: c_int = 0x00000100;
//  SKIP: wxRICHTEXT_DEFAULT_OVERALL_SIZE
//  SKIP: wxRICHTEXT_DEFAULT_IMAGE_SIZE
pub const wxRICHTEXT_DEFAULT_SPACING: c_int = 3;
pub const wxRICHTEXT_DEFAULT_MARGIN: c_int = 3;
//  SKIP: wxRICHTEXT_DEFAULT_UNFOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_FOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_UNSELECTED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_TYPE_COLOUR
//  SKIP: wxRICHTEXT_DEFAULT_FOCUS_RECT_COLOUR
pub const wxRICHTEXT_DEFAULT_CARET_WIDTH: c_int = 2;
pub const wxRICHTEXT_DEFAULT_DELAYED_LAYOUT_THRESHOLD: c_int = 20000;
pub const wxRICHTEXT_DEFAULT_LAYOUT_INTERVAL: c_int = 50;
pub const wxID_RICHTEXT_PROPERTIES1: c_int = (wxID_HIGHEST + 1);
pub const wxID_RICHTEXT_PROPERTIES2: c_int = (wxID_HIGHEST + 2);
pub const wxID_RICHTEXT_PROPERTIES3: c_int = (wxID_HIGHEST + 3);
//  ENUM: wxRichTextCtrlSelectionState
pub const wxRichTextCtrlSelectionState_Normal: c_int = 0;
pub const wxRichTextCtrlSelectionState_CommonAncestor: c_int = 0 + 1;

pub const wxCLRP_USE_TEXTCTRL: c_int = (wxPB_USE_TEXTCTRL);
pub const wxCLRP_DEFAULT_STYLE: c_int = 0;
pub const wxCLRP_SHOW_LABEL: c_int = 0x0008;

pub const wxLB_DEFAULT: c_int = wxBK_DEFAULT;
pub const wxLB_TOP: c_int = wxBK_TOP;
pub const wxLB_BOTTOM: c_int = wxBK_BOTTOM;
pub const wxLB_LEFT: c_int = wxBK_LEFT;
pub const wxLB_RIGHT: c_int = wxBK_RIGHT;
pub const wxLB_ALIGN_MASK: c_int = wxBK_ALIGN_MASK;

pub const wxFD_DEFAULT_STYLE: c_int = wxFD_OPEN;
//  ENUM: @18
pub const wxFD_OPEN: c_int = 0x0001;
pub const wxFD_SAVE: c_int = 0x0002;
pub const wxFD_OVERWRITE_PROMPT: c_int = 0x0004;
pub const wxFD_FILE_MUST_EXIST: c_int = 0x0010;
pub const wxFD_MULTIPLE: c_int = 0x0020;
pub const wxFD_CHANGE_DIR: c_int = 0x0080;
pub const wxFD_PREVIEW: c_int = 0x0100;

//  ENUM: wxProtocolError
pub const wxPROTO_NOERR: c_int = 0;
pub const wxPROTO_NETERR: c_int = 0 + 1;
pub const wxPROTO_PROTERR: c_int = 0 + 2;
pub const wxPROTO_CONNERR: c_int = 0 + 3;
pub const wxPROTO_INVVAL: c_int = 0 + 4;
pub const wxPROTO_NOHNDLR: c_int = 0 + 5;
pub const wxPROTO_NOFILE: c_int = 0 + 6;
pub const wxPROTO_ABRT: c_int = 0 + 7;
pub const wxPROTO_RCNCT: c_int = 0 + 8;
pub const wxPROTO_STREAMING: c_int = 0 + 9;

//  ENUM: wxFindReplaceFlags
pub const wxFR_DOWN: c_int = 1;
pub const wxFR_WHOLEWORD: c_int = 2;
pub const wxFR_MATCHCASE: c_int = 4;
//  ENUM: wxFindReplaceDialogStyles
pub const wxFR_REPLACEDIALOG: c_int = 1;
pub const wxFR_NOUPDOWN: c_int = 2;
pub const wxFR_NOMATCHCASE: c_int = 4;
pub const wxFR_NOWHOLEWORD: c_int = 8;

//  ENUM: wxLayoutDirection
pub const wxLayout_Default: c_int = 0;
pub const wxLayout_LeftToRight: c_int = 0 + 1;
pub const wxLayout_RightToLeft: c_int = 0 + 2;
//  ENUM: wxLocaleCategory
pub const wxLOCALE_CAT_NUMBER: c_int = 0;
pub const wxLOCALE_CAT_DATE: c_int = 0 + 1;
pub const wxLOCALE_CAT_MONEY: c_int = 0 + 2;
pub const wxLOCALE_CAT_DEFAULT: c_int = 0 + 3;
//  ENUM: wxLocaleInfo
pub const wxLOCALE_THOUSANDS_SEP: c_int = 0;
pub const wxLOCALE_DECIMAL_POINT: c_int = 0 + 1;
pub const wxLOCALE_SHORT_DATE_FMT: c_int = 0 + 2;
pub const wxLOCALE_LONG_DATE_FMT: c_int = 0 + 3;
pub const wxLOCALE_DATE_TIME_FMT: c_int = 0 + 4;
pub const wxLOCALE_TIME_FMT: c_int = 0 + 5;

//  ENUM: wxAuiNotebookOption
pub const wxAUI_NB_TOP: c_int = 1 << 0;
pub const wxAUI_NB_LEFT: c_int = 1 << 1;
pub const wxAUI_NB_RIGHT: c_int = 1 << 2;
pub const wxAUI_NB_BOTTOM: c_int = 1 << 3;
pub const wxAUI_NB_TAB_SPLIT: c_int = 1 << 4;
pub const wxAUI_NB_TAB_MOVE: c_int = 1 << 5;
pub const wxAUI_NB_TAB_EXTERNAL_MOVE: c_int = 1 << 6;
pub const wxAUI_NB_TAB_FIXED_WIDTH: c_int = 1 << 7;
pub const wxAUI_NB_SCROLL_BUTTONS: c_int = 1 << 8;
pub const wxAUI_NB_WINDOWLIST_BUTTON: c_int = 1 << 9;
pub const wxAUI_NB_CLOSE_BUTTON: c_int = 1 << 10;
pub const wxAUI_NB_CLOSE_ON_ACTIVE_TAB: c_int = 1 << 11;
pub const wxAUI_NB_CLOSE_ON_ALL_TABS: c_int = 1 << 12;
pub const wxAUI_NB_MIDDLE_CLICK_CLOSE: c_int = 1 << 13;
pub const wxAUI_NB_DEFAULT_STYLE: c_int = wxAUI_NB_TOP |
                             wxAUI_NB_TAB_SPLIT |
                             wxAUI_NB_TAB_MOVE |
                             wxAUI_NB_SCROLL_BUTTONS |
                             wxAUI_NB_CLOSE_ON_ACTIVE_TAB |
                             wxAUI_NB_MIDDLE_CLICK_CLOSE;

// NODEF: wxGetVariantCast

// NODEF: wxTheClipboard

//  ENUM: wxFSVolumeFlags
pub const wxFS_VOL_MOUNTED: c_int = 0x0001;
pub const wxFS_VOL_REMOVABLE: c_int = 0x0002;
pub const wxFS_VOL_READONLY: c_int = 0x0004;
pub const wxFS_VOL_REMOTE: c_int = 0x0008;
//  ENUM: wxFSVolumeKind
pub const wxFS_VOL_FLOPPY: c_int = 0;
pub const wxFS_VOL_DISK: c_int = 0 + 1;
pub const wxFS_VOL_CDROM: c_int = 0 + 2;
pub const wxFS_VOL_DVDROM: c_int = 0 + 3;
pub const wxFS_VOL_NETWORK: c_int = 0 + 4;
pub const wxFS_VOL_OTHER: c_int = 0 + 5;
pub const wxFS_VOL_MAX: c_int = 0 + 6;
//  ENUM: wxFSIconType
pub const wxFS_VOL_ICO_SMALL: c_int = 0;
pub const wxFS_VOL_ICO_LARGE: c_int = 0 + 1;
pub const wxFS_VOL_ICO_SEL_SMALL: c_int = 0 + 2;
pub const wxFS_VOL_ICO_SEL_LARGE: c_int = 0 + 3;
pub const wxFS_VOL_ICO_MAX: c_int = 0 + 4;

//  ENUM: wxPrintBin
pub const wxPRINTBIN_DEFAULT: c_int = 0;
pub const wxPRINTBIN_ONLYONE: c_int = 0 + 1;
pub const wxPRINTBIN_LOWER: c_int = 0 + 2;
pub const wxPRINTBIN_MIDDLE: c_int = 0 + 3;
pub const wxPRINTBIN_MANUAL: c_int = 0 + 4;
pub const wxPRINTBIN_ENVELOPE: c_int = 0 + 5;
pub const wxPRINTBIN_ENVMANUAL: c_int = 0 + 6;
pub const wxPRINTBIN_AUTO: c_int = 0 + 7;
pub const wxPRINTBIN_TRACTOR: c_int = 0 + 8;
pub const wxPRINTBIN_SMALLFMT: c_int = 0 + 9;
pub const wxPRINTBIN_LARGEFMT: c_int = 0 + 10;
pub const wxPRINTBIN_LARGECAPACITY: c_int = 0 + 11;
pub const wxPRINTBIN_CASSETTE: c_int = 0 + 12;
pub const wxPRINTBIN_FORMSOURCE: c_int = 0 + 13;
pub const wxPRINTBIN_USER: c_int = 0 + 14;

//  ENUM: TransferMode
pub const NONE: c_int = 0;
pub const ASCII: c_int = 0 + 1;
pub const BINARY: c_int = 0 + 2;

//  ENUM: @7
pub const wxCC_SPECIAL_DCLICK: c_int = 0x0100;
pub const wxCC_STD_BUTTON: c_int = 0x0200;

// NODEF: wxCRIT_SECT_DECLARE
// NODEF: wxCRIT_SECT_DECLARE_MEMBER
// NODEF: wxCRIT_SECT_LOCKER
// NODEF: wxCRITICAL_SECTION
// NODEF: wxLEAVE_CRIT_SECT
// NODEF: wxENTER_CRIT_SECT
//  ENUM: wxCondError
pub const wxCOND_NO_ERROR: c_int = 0;
pub const wxCOND_INVALID: c_int = 0 + 1;
pub const wxCOND_TIMEOUT: c_int = 0 + 2;
pub const wxCOND_MISC_ERROR: c_int = 0 + 3;
//  ENUM: wxCriticalSectionType
pub const wxCRITSEC_DEFAULT: c_int = 0;
pub const wxCRITSEC_NON_RECURSIVE: c_int = 0 + 1;
//  ENUM: wxThreadWait
pub const wxTHREAD_WAIT_BLOCK: c_int = 0;
pub const wxTHREAD_WAIT_YIELD: c_int = 0 + 1;
pub const wxTHREAD_WAIT_DEFAULT: c_int = wxTHREAD_WAIT_YIELD;
//  ENUM: wxThreadKind
pub const wxTHREAD_DETACHED: c_int = 0;
pub const wxTHREAD_JOINABLE: c_int = 0 + 1;
//  ENUM: wxThreadError
pub const wxTHREAD_NO_ERROR: c_int = 0;
pub const wxTHREAD_NO_RESOURCE: c_int = 0 + 1;
pub const wxTHREAD_RUNNING: c_int = 0 + 2;
pub const wxTHREAD_NOT_RUNNING: c_int = 0 + 3;
pub const wxTHREAD_KILLED: c_int = 0 + 4;
pub const wxTHREAD_MISC_ERROR: c_int = 0 + 5;
//  ENUM: wxSemaError
pub const wxSEMA_NO_ERROR: c_int = 0;
pub const wxSEMA_INVALID: c_int = 0 + 1;
pub const wxSEMA_BUSY: c_int = 0 + 2;
pub const wxSEMA_TIMEOUT: c_int = 0 + 3;
pub const wxSEMA_OVERFLOW: c_int = 0 + 4;
pub const wxSEMA_MISC_ERROR: c_int = 0 + 5;
//  ENUM: wxMutexType
pub const wxMUTEX_DEFAULT: c_int = 0;
pub const wxMUTEX_RECURSIVE: c_int = 0 + 1;
//  ENUM: wxMutexError
pub const wxMUTEX_NO_ERROR: c_int = 0;
pub const wxMUTEX_INVALID: c_int = 0 + 1;
pub const wxMUTEX_DEAD_LOCK: c_int = 0 + 2;
pub const wxMUTEX_BUSY: c_int = 0 + 3;
pub const wxMUTEX_UNLOCKED: c_int = 0 + 4;
pub const wxMUTEX_TIMEOUT: c_int = 0 + 5;
pub const wxMUTEX_MISC_ERROR: c_int = 0 + 6;

pub const wxPD_CAN_ABORT: c_int = 0x0001;
pub const wxPD_APP_MODAL: c_int = 0x0002;
pub const wxPD_AUTO_HIDE: c_int = 0x0004;
pub const wxPD_ELAPSED_TIME: c_int = 0x0008;
pub const wxPD_ESTIMATED_TIME: c_int = 0x0010;
pub const wxPD_SMOOTH: c_int = 0x0020;
pub const wxPD_REMAINING_TIME: c_int = 0x0040;
pub const wxPD_CAN_SKIP: c_int = 0x0080;

pub const wxTIMER_CONTINUOUS: bool = false;
pub const wxTIMER_ONE_SHOT: bool = true;

pub const wxGRID_AUTOSIZE: c_int = (-1);
//  ENUM: wxGridDirection
pub const wxGRID_COLUMN: c_int = 0;
pub const wxGRID_ROW: c_int = 0 + 1;
//  ENUM: wxGridCellFloatFormat
pub const wxGRID_FLOAT_FORMAT_FIXED: c_int = 0x0010;
pub const wxGRID_FLOAT_FORMAT_SCIENTIFIC: c_int = 0x0020;
pub const wxGRID_FLOAT_FORMAT_COMPACT: c_int = 0x0040;
pub const wxGRID_FLOAT_FORMAT_UPPER: c_int = 0x0080;
pub const wxGRID_FLOAT_FORMAT_DEFAULT: c_int = wxGRID_FLOAT_FORMAT_FIXED;
//  ENUM: wxGridTableRequest
pub const wxGRIDTABLE_REQUEST_VIEW_GET_VALUES: c_int = 2000;
pub const wxGRIDTABLE_REQUEST_VIEW_SEND_VALUES: c_int = 2000 + 1;
pub const wxGRIDTABLE_NOTIFY_ROWS_INSERTED: c_int = 2000 + 2;
pub const wxGRIDTABLE_NOTIFY_ROWS_APPENDED: c_int = 2000 + 3;
pub const wxGRIDTABLE_NOTIFY_ROWS_DELETED: c_int = 2000 + 4;
pub const wxGRIDTABLE_NOTIFY_COLS_INSERTED: c_int = 2000 + 5;
pub const wxGRIDTABLE_NOTIFY_COLS_APPENDED: c_int = 2000 + 6;
pub const wxGRIDTABLE_NOTIFY_COLS_DELETED: c_int = 2000 + 7;
//  ENUM: wxGridRenderStyle
pub const wxGRID_DRAW_ROWS_HEADER: c_int = 0x001;
pub const wxGRID_DRAW_COLS_HEADER: c_int = 0x002;
pub const wxGRID_DRAW_CELL_LINES: c_int = 0x004;
pub const wxGRID_DRAW_BOX_RECT: c_int = 0x008;
pub const wxGRID_DRAW_SELECTION: c_int = 0x010;
pub const wxGRID_DRAW_DEFAULT: c_int = wxGRID_DRAW_ROWS_HEADER |
                          wxGRID_DRAW_COLS_HEADER |
                          wxGRID_DRAW_CELL_LINES |
                          wxGRID_DRAW_BOX_RECT;

pub const wxCHB_DEFAULT: c_int = wxBK_DEFAULT;
pub const wxCHB_TOP: c_int = wxBK_TOP;
pub const wxCHB_BOTTOM: c_int = wxBK_BOTTOM;
pub const wxCHB_LEFT: c_int = wxBK_LEFT;
pub const wxCHB_RIGHT: c_int = wxBK_RIGHT;
pub const wxCHB_ALIGN_MASK: c_int = wxBK_ALIGN_MASK;

//  ENUM: wxFSWFlags
pub const wxFSW_EVENT_CREATE: c_int = 0x01;
pub const wxFSW_EVENT_DELETE: c_int = 0x02;
pub const wxFSW_EVENT_RENAME: c_int = 0x04;
pub const wxFSW_EVENT_MODIFY: c_int = 0x08;
pub const wxFSW_EVENT_ACCESS: c_int = 0x10;
pub const wxFSW_EVENT_ATTRIB: c_int = 0x20;
pub const wxFSW_EVENT_UNMOUNT: c_int = 0x2000;
pub const wxFSW_EVENT_WARNING: c_int = 0x40;
pub const wxFSW_EVENT_ERROR: c_int = 0x80;
pub const wxFSW_EVENT_ALL: c_int = wxFSW_EVENT_CREATE | wxFSW_EVENT_DELETE |
                         wxFSW_EVENT_RENAME | wxFSW_EVENT_MODIFY |
                         wxFSW_EVENT_ACCESS | wxFSW_EVENT_ATTRIB |
                         wxFSW_EVENT_WARNING | wxFSW_EVENT_ERROR;
//  ENUM: wxFSWWarningType
pub const wxFSW_WARNING_NONE: c_int = 0;
pub const wxFSW_WARNING_GENERAL: c_int = 0 + 1;
pub const wxFSW_WARNING_OVERFLOW: c_int = 0 + 2;

//  ENUM: wxFileSystemOpenFlags
pub const wxFS_READ: c_int = 1;
pub const wxFS_SEEKABLE: c_int = 4;

//  ENUM: wxBase64DecodeMode
pub const wxBase64DecodeMode_Strict: c_int = 0;
pub const wxBase64DecodeMode_SkipWS: c_int = 0 + 1;
pub const wxBase64DecodeMode_Relaxed: c_int = 0 + 2;

//  ENUM: @24
pub const wxHD_ALLOW_REORDER: c_int = 0x0001;
pub const wxHD_ALLOW_HIDE: c_int = 0x0002;
pub const wxHD_DEFAULT_STYLE: c_int = wxHD_ALLOW_REORDER;

pub const wxDEFAULT_FRAME_STYLE: c_long = (wxSYSTEM_MENU |          wxRESIZE_BORDER |        wxMINIMIZE_BOX |         wxMAXIMIZE_BOX |         wxCLOSE_BOX |            wxCAPTION |              wxCLIP_CHILDREN);
//  ENUM: @51
pub const wxUSER_ATTENTION_INFO: c_int = 1;
pub const wxUSER_ATTENTION_ERROR: c_int = 2;
//  ENUM: @52
pub const wxFULLSCREEN_NOMENUBAR: c_int = 0x0001;
pub const wxFULLSCREEN_NOTOOLBAR: c_int = 0x0002;
pub const wxFULLSCREEN_NOSTATUSBAR: c_int = 0x0004;
pub const wxFULLSCREEN_NOBORDER: c_int = 0x0008;
pub const wxFULLSCREEN_NOCAPTION: c_int = 0x0010;
pub const wxFULLSCREEN_ALL: c_int = wxFULLSCREEN_NOMENUBAR | wxFULLSCREEN_NOTOOLBAR |
                          wxFULLSCREEN_NOSTATUSBAR | wxFULLSCREEN_NOBORDER |
                          wxFULLSCREEN_NOCAPTION;

//  ENUM: wxAcceleratorEntryFlags
pub const wxACCEL_NORMAL: c_int = 0;
pub const wxACCEL_ALT: c_int = 0 + 1;
pub const wxACCEL_CTRL: c_int = 0 + 2;
pub const wxACCEL_SHIFT: c_int = 0 + 3;
pub const wxACCEL_RAW_CTRL: c_int = 0 + 4;
pub const wxACCEL_CMD: c_int = 0 + 5;

//  ENUM: wxEdge
pub const wxLeft: c_int = 0;
pub const wxTop: c_int = 0 + 1;
pub const wxRight: c_int = 0 + 2;
pub const wxBottom: c_int = 0 + 3;
pub const wxWidth: c_int = 0 + 4;
pub const wxHeight: c_int = 0 + 5;
pub const wxCentre: c_int = 0 + 6;
pub const wxCenter: c_int = wxCentre;
pub const wxCentreX: c_int = wxCentre + 1;
pub const wxCentreY: c_int = wxCentre + 2;
//  ENUM: wxRelationship
pub const wxUnconstrained: c_int = 0;
pub const wxAsIs: c_int = 0 + 1;
pub const wxPercentOf: c_int = 0 + 2;
pub const wxAbove: c_int = 0 + 3;
pub const wxBelow: c_int = 0 + 4;
pub const wxLeftOf: c_int = 0 + 5;
pub const wxRightOf: c_int = 0 + 6;
pub const wxSameAs: c_int = 0 + 7;
pub const wxAbsolute: c_int = 0 + 8;

//  ENUM: wxStreamError
pub const wxSTREAM_NO_ERROR: c_int = 0;
pub const wxSTREAM_EOF: c_int = 0 + 1;
pub const wxSTREAM_WRITE_ERROR: c_int = 0 + 2;
pub const wxSTREAM_READ_ERROR: c_int = 0 + 3;
//  ENUM: wxStreamProtocolType
pub const wxSTREAM_PROTOCOL: c_int = 0;
pub const wxSTREAM_MIMETYPE: c_int = 0 + 1;
pub const wxSTREAM_ENCODING: c_int = 0 + 2;
pub const wxSTREAM_FILEEXT: c_int = 0 + 3;

//  ENUM: wxZipMethod
pub const wxZIP_METHOD_STORE: c_int = 0;
pub const wxZIP_METHOD_SHRINK: c_int = 0 + 1;
pub const wxZIP_METHOD_REDUCE1: c_int = 0 + 2;
pub const wxZIP_METHOD_REDUCE2: c_int = 0 + 3;
pub const wxZIP_METHOD_REDUCE3: c_int = 0 + 4;
pub const wxZIP_METHOD_REDUCE4: c_int = 0 + 5;
pub const wxZIP_METHOD_IMPLODE: c_int = 0 + 6;
pub const wxZIP_METHOD_TOKENIZE: c_int = 0 + 7;
pub const wxZIP_METHOD_DEFLATE: c_int = 0 + 8;
pub const wxZIP_METHOD_DEFLATE64: c_int = 0 + 9;
pub const wxZIP_METHOD_BZIP2: c_int = 12;
pub const wxZIP_METHOD_DEFAULT: c_int = 0xffff;
//  ENUM: wxZipSystem
pub const wxZIP_SYSTEM_MSDOS: c_int = 0;
pub const wxZIP_SYSTEM_AMIGA: c_int = 0 + 1;
pub const wxZIP_SYSTEM_OPENVMS: c_int = 0 + 2;
pub const wxZIP_SYSTEM_UNIX: c_int = 0 + 3;
pub const wxZIP_SYSTEM_VM_CMS: c_int = 0 + 4;
pub const wxZIP_SYSTEM_ATARI_ST: c_int = 0 + 5;
pub const wxZIP_SYSTEM_OS2_HPFS: c_int = 0 + 6;
pub const wxZIP_SYSTEM_MACINTOSH: c_int = 0 + 7;
pub const wxZIP_SYSTEM_Z_SYSTEM: c_int = 0 + 8;
pub const wxZIP_SYSTEM_CPM: c_int = 0 + 9;
pub const wxZIP_SYSTEM_WINDOWS_NTFS: c_int = 0 + 10;
pub const wxZIP_SYSTEM_MVS: c_int = 0 + 11;
pub const wxZIP_SYSTEM_VSE: c_int = 0 + 12;
pub const wxZIP_SYSTEM_ACORN_RISC: c_int = 0 + 13;
pub const wxZIP_SYSTEM_VFAT: c_int = 0 + 14;
pub const wxZIP_SYSTEM_ALTERNATE_MVS: c_int = 0 + 15;
pub const wxZIP_SYSTEM_BEOS: c_int = 0 + 16;
pub const wxZIP_SYSTEM_TANDEM: c_int = 0 + 17;
pub const wxZIP_SYSTEM_OS_400: c_int = 0 + 18;
//  ENUM: wxZipAttributes
pub const wxZIP_A_RDONLY: c_int = 0x01;
pub const wxZIP_A_HIDDEN: c_int = 0x02;
pub const wxZIP_A_SYSTEM: c_int = 0x04;
pub const wxZIP_A_SUBDIR: c_int = 0x10;
pub const wxZIP_A_ARCH: c_int = 0x20;
pub const wxZIP_A_MASK: c_int = 0x37;
//  ENUM: wxZipFlags
pub const wxZIP_ENCRYPTED: c_int = 0x0001;
pub const wxZIP_DEFLATE_NORMAL: c_int = 0x0000;
pub const wxZIP_DEFLATE_EXTRA: c_int = 0x0002;
pub const wxZIP_DEFLATE_FAST: c_int = 0x0004;
pub const wxZIP_DEFLATE_SUPERFAST: c_int = 0x0006;
pub const wxZIP_DEFLATE_MASK: c_int = 0x0006;
pub const wxZIP_SUMS_FOLLOW: c_int = 0x0008;
pub const wxZIP_ENHANCED: c_int = 0x0010;
pub const wxZIP_PATCH: c_int = 0x0020;
pub const wxZIP_STRONG_ENC: c_int = 0x0040;
pub const wxZIP_UNUSED: c_int = 0x0F80;
pub const wxZIP_RESERVED: c_int = 0xF000;

//  ENUM: @49

//  ENUM: wxTarType
pub const wxTAR_REGTYPE: char = '0';
pub const wxTAR_LNKTYPE: char = '1';
pub const wxTAR_SYMTYPE: char = '2';
pub const wxTAR_CHRTYPE: char = '3';
pub const wxTAR_BLKTYPE: char = '4';
pub const wxTAR_DIRTYPE: char = '5';
pub const wxTAR_FIFOTYPE: char = '6';
pub const wxTAR_CONTTYPE: char = '7';
//  ENUM: wxTarFormat
pub const wxTAR_USTAR: c_int = 0;
pub const wxTAR_PAX: c_int = 0 + 1;

pub const wxDIALOG_NO_PARENT: c_int = 0x00000020;
pub const wxDEFAULT_DIALOG_STYLE: c_long = (wxCAPTION | wxSYSTEM_MENU | wxCLOSE_BOX);
pub const wxDIALOG_ADAPTATION_NONE: c_int = 0;
pub const wxDIALOG_ADAPTATION_STANDARD_SIZER: c_int = 1;
pub const wxDIALOG_ADAPTATION_ANY_SIZER: c_int = 2;
pub const wxDIALOG_ADAPTATION_LOOSE_BUTTONS: c_int = 3;
//  ENUM: wxDialogLayoutAdaptationMode
pub const wxDIALOG_ADAPTATION_MODE_DEFAULT: c_int = 0;
pub const wxDIALOG_ADAPTATION_MODE_ENABLED: c_int = 1;
pub const wxDIALOG_ADAPTATION_MODE_DISABLED: c_int = 2;

//  ENUM: Reason
pub const Reason_Mouse: c_int = 0;
pub const Reason_Unknown: c_int = 0 + 1;

//  ENUM: wxAuiPaneDockArtSetting
//  ENUM: wxAuiPaneDockArtGradients
pub const wxAUI_GRADIENT_NONE: c_int = 0;
pub const wxAUI_GRADIENT_VERTICAL: c_int = 1;
pub const wxAUI_GRADIENT_HORIZONTAL: c_int = 2;
//  ENUM: wxAuiPaneButtonState
pub const wxAUI_BUTTON_STATE_NORMAL: c_int = 0;
pub const wxAUI_BUTTON_STATE_HOVER: c_int = 1 << 1;
pub const wxAUI_BUTTON_STATE_PRESSED: c_int = 1 << 2;
pub const wxAUI_BUTTON_STATE_DISABLED: c_int = 1 << 3;
pub const wxAUI_BUTTON_STATE_HIDDEN: c_int = 1 << 4;
pub const wxAUI_BUTTON_STATE_CHECKED: c_int = 1 << 5;
//  ENUM: wxAuiButtonId
pub const wxAUI_BUTTON_CLOSE: c_int = 101;
pub const wxAUI_BUTTON_MAXIMIZE_RESTORE: c_int = 102;
pub const wxAUI_BUTTON_MINIMIZE: c_int = 103;
pub const wxAUI_BUTTON_PIN: c_int = 104;
pub const wxAUI_BUTTON_OPTIONS: c_int = 105;
pub const wxAUI_BUTTON_WINDOWLIST: c_int = 106;
pub const wxAUI_BUTTON_LEFT: c_int = 107;
pub const wxAUI_BUTTON_RIGHT: c_int = 108;
pub const wxAUI_BUTTON_UP: c_int = 109;
pub const wxAUI_BUTTON_DOWN: c_int = 110;
pub const wxAUI_BUTTON_CUSTOM1: c_int = 201;
pub const wxAUI_BUTTON_CUSTOM2: c_int = 202;
pub const wxAUI_BUTTON_CUSTOM3: c_int = 203;

// NODEF: wxT
// NODEF: wxT_2
// NODEF: wxS
// NODEF: _T

//  ENUM: wxRegionContain
pub const wxOutRegion: c_int = 0;
pub const wxPartRegion: c_int = 1;
pub const wxInRegion: c_int = 2;

pub const wxSW_NOBORDER: c_int = 0x0000;
pub const wxSW_BORDER: c_int = 0x0020;
pub const wxSW_3DSASH: c_int = 0x0040;
pub const wxSW_3DBORDER: c_int = 0x0080;
pub const wxSW_3D: c_int = (wxSW_3DSASH | wxSW_3DBORDER);
//  ENUM: wxSashEdgePosition
pub const wxSASH_TOP: c_int = 0;
pub const wxSASH_RIGHT: c_int = 0 + 1;
pub const wxSASH_BOTTOM: c_int = 0 + 2;
pub const wxSASH_LEFT: c_int = 0 + 3;
pub const wxSASH_NONE: c_int = 100;
//  ENUM: wxSashDragStatus
pub const wxSASH_STATUS_OK: c_int = 0;
pub const wxSASH_STATUS_OUT_OF_RANGE: c_int = 0 + 1;

//  ENUM: @5
pub const wxC2S_NAME: c_int = 1;
pub const wxC2S_CSS_SYNTAX: c_int = 2;
pub const wxC2S_HTML_SYNTAX: c_int = 4;

//  ENUM: wxNumValidatorStyle
pub const wxNUM_VAL_DEFAULT: c_int = 0;
pub const wxNUM_VAL_THOUSANDS_SEPARATOR: c_int = 1;
pub const wxNUM_VAL_ZERO_AS_BLANK: c_int = 2;
pub const wxNUM_VAL_NO_TRAILING_ZEROES: c_int = 2 + 1;


pub const wxTR_NO_BUTTONS: c_int = 0x0000;
pub const wxTR_HAS_BUTTONS: c_int = 0x0001;
pub const wxTR_NO_LINES: c_int = 0x0004;
pub const wxTR_LINES_AT_ROOT: c_int = 0x0008;
pub const wxTR_TWIST_BUTTONS: c_int = 0x0010;
pub const wxTR_SINGLE: c_int = 0x0000;
pub const wxTR_MULTIPLE: c_int = 0x0020;
pub const wxTR_HAS_VARIABLE_ROW_HEIGHT: c_int = 0x0080;
pub const wxTR_EDIT_LABELS: c_int = 0x0200;
pub const wxTR_ROW_LINES: c_int = 0x0400;
pub const wxTR_HIDE_ROOT: c_int = 0x0800;
pub const wxTR_FULL_ROW_HIGHLIGHT: c_int = 0x2000;
pub const wxTR_DEFAULT_STYLE: c_int = (wxTR_HAS_BUTTONS | wxTR_LINES_AT_ROOT);
//  ENUM: wxTreeItemIcon
pub const wxTreeItemIcon_Normal: c_int = 0;
pub const wxTreeItemIcon_Selected: c_int = 0 + 1;
pub const wxTreeItemIcon_Expanded: c_int = 0 + 2;
pub const wxTreeItemIcon_SelectedExpanded: c_int = 0 + 3;
pub const wxTreeItemIcon_Max: c_int = 0 + 4;

// NODEF: wxCHANGE_UMASK
//  ENUM: wxPosixPermissions
pub const wxS_IRUSR: c_int = 00400;
pub const wxS_IWUSR: c_int = 00200;
pub const wxS_IXUSR: c_int = 00100;
pub const wxS_IRGRP: c_int = 00040;
pub const wxS_IWGRP: c_int = 00020;
pub const wxS_IXGRP: c_int = 00010;
pub const wxS_IROTH: c_int = 00004;
pub const wxS_IWOTH: c_int = 00002;
pub const wxS_IXOTH: c_int = 00001;
pub const wxPOSIX_USER_READ: c_int = wxS_IRUSR;
pub const wxPOSIX_USER_WRITE: c_int = wxS_IWUSR;
pub const wxPOSIX_USER_EXECUTE: c_int = wxS_IXUSR;
pub const wxPOSIX_GROUP_READ: c_int = wxS_IRGRP;
pub const wxPOSIX_GROUP_WRITE: c_int = wxS_IWGRP;
pub const wxPOSIX_GROUP_EXECUTE: c_int = wxS_IXGRP;
pub const wxPOSIX_OTHERS_READ: c_int = wxS_IROTH;
pub const wxPOSIX_OTHERS_WRITE: c_int = wxS_IWOTH;
pub const wxPOSIX_OTHERS_EXECUTE: c_int = wxS_IXOTH;
pub const wxS_DEFAULT: c_int = (wxPOSIX_USER_READ | wxPOSIX_USER_WRITE | 
                   wxPOSIX_GROUP_READ | wxPOSIX_GROUP_WRITE | 
                   wxPOSIX_OTHERS_READ | wxPOSIX_OTHERS_WRITE);
pub const wxS_DIR_DEFAULT: c_int = (wxPOSIX_USER_READ | wxPOSIX_USER_WRITE | wxPOSIX_USER_EXECUTE | 
                       wxPOSIX_GROUP_READ | wxPOSIX_GROUP_WRITE | wxPOSIX_GROUP_EXECUTE | 
                       wxPOSIX_OTHERS_READ | wxPOSIX_OTHERS_WRITE | wxPOSIX_OTHERS_EXECUTE);
//  ENUM: wxSeekMode
pub const wxFromStart: c_int = 0;
pub const wxFromCurrent: c_int = 0 + 1;
pub const wxFromEnd: c_int = 0 + 2;
//  ENUM: wxFileKind
pub const wxFILE_KIND_UNKNOWN: c_int = 0;
pub const wxFILE_KIND_DISK: c_int = 0 + 1;
pub const wxFILE_KIND_TERMINAL: c_int = 0 + 2;
pub const wxFILE_KIND_PIPE: c_int = 0 + 3;

pub const wxACC_STATE_SYSTEM_ALERT_HIGH: c_int = 0x00000001;
pub const wxACC_STATE_SYSTEM_ALERT_MEDIUM: c_int = 0x00000002;
pub const wxACC_STATE_SYSTEM_ALERT_LOW: c_int = 0x00000004;
pub const wxACC_STATE_SYSTEM_ANIMATED: c_int = 0x00000008;
pub const wxACC_STATE_SYSTEM_BUSY: c_int = 0x00000010;
pub const wxACC_STATE_SYSTEM_CHECKED: c_int = 0x00000020;
pub const wxACC_STATE_SYSTEM_COLLAPSED: c_int = 0x00000040;
pub const wxACC_STATE_SYSTEM_DEFAULT: c_int = 0x00000080;
pub const wxACC_STATE_SYSTEM_EXPANDED: c_int = 0x00000100;
pub const wxACC_STATE_SYSTEM_EXTSELECTABLE: c_int = 0x00000200;
pub const wxACC_STATE_SYSTEM_FLOATING: c_int = 0x00000400;
pub const wxACC_STATE_SYSTEM_FOCUSABLE: c_int = 0x00000800;
pub const wxACC_STATE_SYSTEM_FOCUSED: c_int = 0x00001000;
pub const wxACC_STATE_SYSTEM_HOTTRACKED: c_int = 0x00002000;
pub const wxACC_STATE_SYSTEM_INVISIBLE: c_int = 0x00004000;
pub const wxACC_STATE_SYSTEM_MARQUEED: c_int = 0x00008000;
pub const wxACC_STATE_SYSTEM_MIXED: c_int = 0x00010000;
pub const wxACC_STATE_SYSTEM_MULTISELECTABLE: c_int = 0x00020000;
pub const wxACC_STATE_SYSTEM_OFFSCREEN: c_int = 0x00040000;
pub const wxACC_STATE_SYSTEM_PRESSED: c_int = 0x00080000;
pub const wxACC_STATE_SYSTEM_PROTECTED: c_int = 0x00100000;
pub const wxACC_STATE_SYSTEM_READONLY: c_int = 0x00200000;
pub const wxACC_STATE_SYSTEM_SELECTABLE: c_int = 0x00400000;
pub const wxACC_STATE_SYSTEM_SELECTED: c_int = 0x00800000;
pub const wxACC_STATE_SYSTEM_SELFVOICING: c_int = 0x01000000;
pub const wxACC_STATE_SYSTEM_UNAVAILABLE: c_int = 0x02000000;
pub const wxACC_EVENT_SYSTEM_SOUND: c_int = 0x0001;
pub const wxACC_EVENT_SYSTEM_ALERT: c_int = 0x0002;
pub const wxACC_EVENT_SYSTEM_FOREGROUND: c_int = 0x0003;
pub const wxACC_EVENT_SYSTEM_MENUSTART: c_int = 0x0004;
pub const wxACC_EVENT_SYSTEM_MENUEND: c_int = 0x0005;
pub const wxACC_EVENT_SYSTEM_MENUPOPUPSTART: c_int = 0x0006;
pub const wxACC_EVENT_SYSTEM_MENUPOPUPEND: c_int = 0x0007;
pub const wxACC_EVENT_SYSTEM_CAPTURESTART: c_int = 0x0008;
pub const wxACC_EVENT_SYSTEM_CAPTUREEND: c_int = 0x0009;
pub const wxACC_EVENT_SYSTEM_MOVESIZESTART: c_int = 0x000A;
pub const wxACC_EVENT_SYSTEM_MOVESIZEEND: c_int = 0x000B;
pub const wxACC_EVENT_SYSTEM_CONTEXTHELPSTART: c_int = 0x000C;
pub const wxACC_EVENT_SYSTEM_CONTEXTHELPEND: c_int = 0x000D;
pub const wxACC_EVENT_SYSTEM_DRAGDROPSTART: c_int = 0x000E;
pub const wxACC_EVENT_SYSTEM_DRAGDROPEND: c_int = 0x000F;
pub const wxACC_EVENT_SYSTEM_DIALOGSTART: c_int = 0x0010;
pub const wxACC_EVENT_SYSTEM_DIALOGEND: c_int = 0x0011;
pub const wxACC_EVENT_SYSTEM_SCROLLINGSTART: c_int = 0x0012;
pub const wxACC_EVENT_SYSTEM_SCROLLINGEND: c_int = 0x0013;
pub const wxACC_EVENT_SYSTEM_SWITCHSTART: c_int = 0x0014;
pub const wxACC_EVENT_SYSTEM_SWITCHEND: c_int = 0x0015;
pub const wxACC_EVENT_SYSTEM_MINIMIZESTART: c_int = 0x0016;
pub const wxACC_EVENT_SYSTEM_MINIMIZEEND: c_int = 0x0017;
pub const wxACC_EVENT_OBJECT_CREATE: c_int = 0x8000;
pub const wxACC_EVENT_OBJECT_DESTROY: c_int = 0x8001;
pub const wxACC_EVENT_OBJECT_SHOW: c_int = 0x8002;
pub const wxACC_EVENT_OBJECT_HIDE: c_int = 0x8003;
pub const wxACC_EVENT_OBJECT_REORDER: c_int = 0x8004;
pub const wxACC_EVENT_OBJECT_FOCUS: c_int = 0x8005;
pub const wxACC_EVENT_OBJECT_SELECTION: c_int = 0x8006;
pub const wxACC_EVENT_OBJECT_SELECTIONADD: c_int = 0x8007;
pub const wxACC_EVENT_OBJECT_SELECTIONREMOVE: c_int = 0x8008;
pub const wxACC_EVENT_OBJECT_SELECTIONWITHIN: c_int = 0x8009;
pub const wxACC_EVENT_OBJECT_STATECHANGE: c_int = 0x800A;
pub const wxACC_EVENT_OBJECT_LOCATIONCHANGE: c_int = 0x800B;
pub const wxACC_EVENT_OBJECT_NAMECHANGE: c_int = 0x800C;
pub const wxACC_EVENT_OBJECT_DESCRIPTIONCHANGE: c_int = 0x800D;
pub const wxACC_EVENT_OBJECT_VALUECHANGE: c_int = 0x800E;
pub const wxACC_EVENT_OBJECT_PARENTCHANGE: c_int = 0x800F;
pub const wxACC_EVENT_OBJECT_HELPCHANGE: c_int = 0x8010;
pub const wxACC_EVENT_OBJECT_DEFACTIONCHANGE: c_int = 0x8011;
pub const wxACC_EVENT_OBJECT_ACCELERATORCHANGE: c_int = 0x8012;
//  ENUM: wxAccStatus
pub const wxACC_FAIL: c_int = 0;
pub const wxACC_FALSE: c_int = 0 + 1;
pub const wxACC_OK: c_int = 0 + 2;
pub const wxACC_NOT_IMPLEMENTED: c_int = 0 + 3;
pub const wxACC_NOT_SUPPORTED: c_int = 0 + 4;
//  ENUM: wxNavDir
pub const wxNAVDIR_DOWN: c_int = 0;
pub const wxNAVDIR_FIRSTCHILD: c_int = 0 + 1;
pub const wxNAVDIR_LASTCHILD: c_int = 0 + 2;
pub const wxNAVDIR_LEFT: c_int = 0 + 3;
pub const wxNAVDIR_NEXT: c_int = 0 + 4;
pub const wxNAVDIR_PREVIOUS: c_int = 0 + 5;
pub const wxNAVDIR_RIGHT: c_int = 0 + 6;
pub const wxNAVDIR_UP: c_int = 0 + 7;
//  ENUM: wxAccRole
pub const wxROLE_NONE: c_int = 0;
pub const wxROLE_SYSTEM_ALERT: c_int = 0 + 1;
pub const wxROLE_SYSTEM_ANIMATION: c_int = 0 + 2;
pub const wxROLE_SYSTEM_APPLICATION: c_int = 0 + 3;
pub const wxROLE_SYSTEM_BORDER: c_int = 0 + 4;
pub const wxROLE_SYSTEM_BUTTONDROPDOWN: c_int = 0 + 5;
pub const wxROLE_SYSTEM_BUTTONDROPDOWNGRID: c_int = 0 + 6;
pub const wxROLE_SYSTEM_BUTTONMENU: c_int = 0 + 7;
pub const wxROLE_SYSTEM_CARET: c_int = 0 + 8;
pub const wxROLE_SYSTEM_CELL: c_int = 0 + 9;
pub const wxROLE_SYSTEM_CHARACTER: c_int = 0 + 10;
pub const wxROLE_SYSTEM_CHART: c_int = 0 + 11;
pub const wxROLE_SYSTEM_CHECKBUTTON: c_int = 0 + 12;
pub const wxROLE_SYSTEM_CLIENT: c_int = 0 + 13;
pub const wxROLE_SYSTEM_CLOCK: c_int = 0 + 14;
pub const wxROLE_SYSTEM_COLUMN: c_int = 0 + 15;
pub const wxROLE_SYSTEM_COLUMNHEADER: c_int = 0 + 16;
pub const wxROLE_SYSTEM_COMBOBOX: c_int = 0 + 17;
pub const wxROLE_SYSTEM_CURSOR: c_int = 0 + 18;
pub const wxROLE_SYSTEM_DIAGRAM: c_int = 0 + 19;
pub const wxROLE_SYSTEM_DIAL: c_int = 0 + 20;
pub const wxROLE_SYSTEM_DIALOG: c_int = 0 + 21;
pub const wxROLE_SYSTEM_DOCUMENT: c_int = 0 + 22;
pub const wxROLE_SYSTEM_DROPLIST: c_int = 0 + 23;
pub const wxROLE_SYSTEM_EQUATION: c_int = 0 + 24;
pub const wxROLE_SYSTEM_GRAPHIC: c_int = 0 + 25;
pub const wxROLE_SYSTEM_GRIP: c_int = 0 + 26;
pub const wxROLE_SYSTEM_GROUPING: c_int = 0 + 27;
pub const wxROLE_SYSTEM_HELPBALLOON: c_int = 0 + 28;
pub const wxROLE_SYSTEM_HOTKEYFIELD: c_int = 0 + 29;
pub const wxROLE_SYSTEM_INDICATOR: c_int = 0 + 30;
pub const wxROLE_SYSTEM_LINK: c_int = 0 + 31;
pub const wxROLE_SYSTEM_LIST: c_int = 0 + 32;
pub const wxROLE_SYSTEM_LISTITEM: c_int = 0 + 33;
pub const wxROLE_SYSTEM_MENUBAR: c_int = 0 + 34;
pub const wxROLE_SYSTEM_MENUITEM: c_int = 0 + 35;
pub const wxROLE_SYSTEM_MENUPOPUP: c_int = 0 + 36;
pub const wxROLE_SYSTEM_OUTLINE: c_int = 0 + 37;
pub const wxROLE_SYSTEM_OUTLINEITEM: c_int = 0 + 38;
pub const wxROLE_SYSTEM_PAGETAB: c_int = 0 + 39;
pub const wxROLE_SYSTEM_PAGETABLIST: c_int = 0 + 40;
pub const wxROLE_SYSTEM_PANE: c_int = 0 + 41;
pub const wxROLE_SYSTEM_PROGRESSBAR: c_int = 0 + 42;
pub const wxROLE_SYSTEM_PROPERTYPAGE: c_int = 0 + 43;
pub const wxROLE_SYSTEM_PUSHBUTTON: c_int = 0 + 44;
pub const wxROLE_SYSTEM_RADIOBUTTON: c_int = 0 + 45;
pub const wxROLE_SYSTEM_ROW: c_int = 0 + 46;
pub const wxROLE_SYSTEM_ROWHEADER: c_int = 0 + 47;
pub const wxROLE_SYSTEM_SCROLLBAR: c_int = 0 + 48;
pub const wxROLE_SYSTEM_SEPARATOR: c_int = 0 + 49;
pub const wxROLE_SYSTEM_SLIDER: c_int = 0 + 50;
pub const wxROLE_SYSTEM_SOUND: c_int = 0 + 51;
pub const wxROLE_SYSTEM_SPINBUTTON: c_int = 0 + 52;
pub const wxROLE_SYSTEM_STATICTEXT: c_int = 0 + 53;
pub const wxROLE_SYSTEM_STATUSBAR: c_int = 0 + 54;
pub const wxROLE_SYSTEM_TABLE: c_int = 0 + 55;
pub const wxROLE_SYSTEM_TEXT: c_int = 0 + 56;
pub const wxROLE_SYSTEM_TITLEBAR: c_int = 0 + 57;
pub const wxROLE_SYSTEM_TOOLBAR: c_int = 0 + 58;
pub const wxROLE_SYSTEM_TOOLTIP: c_int = 0 + 59;
pub const wxROLE_SYSTEM_WHITESPACE: c_int = 0 + 60;
pub const wxROLE_SYSTEM_WINDOW: c_int = 0 + 61;
//  ENUM: wxAccObject
pub const wxOBJID_WINDOW: c_long =    0x00000000;
pub const wxOBJID_SYSMENU: c_long =   0xFFFFFFFF;
pub const wxOBJID_TITLEBAR: c_long =  0xFFFFFFFE;
pub const wxOBJID_MENU: c_long =      0xFFFFFFFD;
pub const wxOBJID_CLIENT: c_long =    0xFFFFFFFC;
pub const wxOBJID_VSCROLL: c_long =   0xFFFFFFFB;
pub const wxOBJID_HSCROLL: c_long =   0xFFFFFFFA;
pub const wxOBJID_SIZEGRIP: c_long =  0xFFFFFFF9;
pub const wxOBJID_CARET: c_long =     0xFFFFFFF8;
pub const wxOBJID_CURSOR: c_long =    0xFFFFFFF7;
pub const wxOBJID_ALERT: c_long =     0xFFFFFFF6;
pub const wxOBJID_SOUND: c_long =     0xFFFFFFF5;
//  ENUM: wxAccSelectionFlags
pub const wxACC_SEL_NONE: c_int = 0;
pub const wxACC_SEL_TAKEFOCUS: c_int = 1;
pub const wxACC_SEL_TAKESELECTION: c_int = 2;
pub const wxACC_SEL_EXTENDSELECTION: c_int = 4;
pub const wxACC_SEL_ADDSELECTION: c_int = 8;
pub const wxACC_SEL_REMOVESELECTION: c_int = 16;

pub const wxHLB_DEFAULT_STYLE: c_long = wxBORDER_SUNKEN;
pub const wxHLB_MULTIPLE: c_int = wxLB_MULTIPLE;

//  ENUM: wxLayoutOrientation
pub const wxLAYOUT_HORIZONTAL: c_int = 0;
pub const wxLAYOUT_VERTICAL: c_int = 0 + 1;
//  ENUM: wxLayoutAlignment
pub const wxLAYOUT_NONE: c_int = 0;
pub const wxLAYOUT_TOP: c_int = 0 + 1;
pub const wxLAYOUT_LEFT: c_int = 0 + 2;
pub const wxLAYOUT_RIGHT: c_int = 0 + 3;
pub const wxLAYOUT_BOTTOM: c_int = 0 + 4;

//  ENUM: wxRibbonPanelOption
pub const wxRIBBON_PANEL_NO_AUTO_MINIMISE: c_int = 0;
pub const wxRIBBON_PANEL_EXT_BUTTON: c_int = 0 + 1;
pub const wxRIBBON_PANEL_MINIMISE_BUTTON: c_int = 0 + 2;
pub const wxRIBBON_PANEL_STRETCH: c_int = 0 + 3;
pub const wxRIBBON_PANEL_FLEXIBLE: c_int = 0 + 4;
pub const wxRIBBON_PANEL_DEFAULT_STYLE: c_int = 0 + 5;

pub const wxPG_ATTR_DEFAULT_VALUE: &str = "DefaultValue";
pub const wxPG_ATTR_MIN: &str = "Min";
pub const wxPG_ATTR_MAX: &str = "Max";
pub const wxPG_ATTR_UNITS: &str = "Units";
pub const wxPG_ATTR_HINT: &str = "Hint";
pub const wxPG_ATTR_INLINE_HELP: &str = "InlineHelp";
pub const wxPG_ATTR_AUTOCOMPLETE: &str = "AutoComplete";
pub const wxPG_BOOL_USE_CHECKBOX: &str = "UseCheckbox";
pub const wxPG_BOOL_USE_DOUBLE_CLICK_CYCLING: &str = "UseDClickCycling";
pub const wxPG_FLOAT_PRECISION: &str = "Precision";
pub const wxPG_STRING_PASSWORD: &str = "Password";
pub const wxPG_UINT_BASE: &str = "Base";
pub const wxPG_UINT_PREFIX: &str = "Prefix";
pub const wxPG_FILE_WILDCARD: &str = "Wildcard";
pub const wxPG_FILE_SHOW_FULL_PATH: &str = "ShowFullPath";
pub const wxPG_FILE_SHOW_RELATIVE_PATH: &str = "ShowRelativePath";
pub const wxPG_FILE_INITIAL_PATH: &str = "InitialPath";
pub const wxPG_FILE_DIALOG_TITLE: &str = "DialogTitle";
pub const wxPG_FILE_DIALOG_STYLE: &str = "DialogStyle";
pub const wxPG_DIR_DIALOG_MESSAGE: &str = "DialogMessage";
pub const wxPG_ARRAY_DELIMITER: &str = "Delimiter";
pub const wxPG_DATE_FORMAT: &str = "DateFormat";
pub const wxPG_DATE_PICKER_STYLE: &str = "PickerStyle";
pub const wxPG_ATTR_SPINCTRL_STEP: &str = "Step";
pub const wxPG_ATTR_SPINCTRL_WRAP: &str = "Wrap";
pub const wxPG_ATTR_SPINCTRL_MOTIONSPIN: &str = "MotionSpin";
pub const wxPG_ATTR_MULTICHOICE_USERSTRINGMODE: &str = "UserStringMode";
pub const wxPG_COLOUR_ALLOW_CUSTOM: &str = "AllowCustom";
pub const wxPG_COLOUR_HAS_ALPHA: &str = "HasAlpha";
pub const wxPG_PROP_MAX: c_long = wxPG_PROP_AUTO_UNSPECIFIED;
//  SKIP: wxPG_PROP_PARENTAL_FLAGS
//  SKIP: wxNullProperty
pub const wxPG_CUSTOM_IMAGE_SPACINGY: c_int = 1;
pub const wxPG_CAPRECTXMARGIN: c_int = 2;
pub const wxPG_CAPRECTYMARGIN: c_int = 1;
//  SKIP: wxPGChoicesEmptyData
//  ENUM: wxPGPropertyFlags
pub const wxPG_PROP_MODIFIED: c_long = 0x0001;
pub const wxPG_PROP_DISABLED: c_long = 0x0002;
pub const wxPG_PROP_HIDDEN: c_long = 0x0004;
pub const wxPG_PROP_CUSTOMIMAGE: c_long = 0x0008;
pub const wxPG_PROP_NOEDITOR: c_long = 0x0010;
pub const wxPG_PROP_COLLAPSED: c_long = 0x0020;
pub const wxPG_PROP_INVALID_VALUE: c_long = 0x0040;
pub const wxPG_PROP_WAS_MODIFIED: c_long = 0x0200;
pub const wxPG_PROP_AGGREGATE: c_long = 0x0400;
pub const wxPG_PROP_CHILDREN_ARE_COPIES: c_long = 0x0800;
pub const wxPG_PROP_PROPERTY: c_long = 0x1000;
pub const wxPG_PROP_CATEGORY: c_long = 0x2000;
pub const wxPG_PROP_MISC_PARENT: c_long = 0x4000;
pub const wxPG_PROP_READONLY: c_long = 0x8000;
pub const wxPG_PROP_COMPOSED_VALUE: c_long = 0x00010000;
pub const wxPG_PROP_USES_COMMON_VALUE: c_long = 0x00020000;
pub const wxPG_PROP_AUTO_UNSPECIFIED: c_long = 0x00040000;
pub const wxPG_PROP_CLASS_SPECIFIC_1: c_long = 0x00080000;
pub const wxPG_PROP_CLASS_SPECIFIC_2: c_long = 0x00100000;
//  SKIP: wxPG_PROP_BEING_DELETED

pub const wxTE_NO_VSCROLL: c_int = 0x0002;
pub const wxTE_READONLY: c_int = 0x0010;
pub const wxTE_MULTILINE: c_int = 0x0020;
pub const wxTE_PROCESS_TAB: c_int = 0x0040;
pub const wxTE_LEFT: c_int = 0x0000;
pub const wxTE_CENTER: c_int = wxALIGN_CENTER_HORIZONTAL;
pub const wxTE_RIGHT: c_int = wxALIGN_RIGHT;
pub const wxTE_CENTRE: c_int = wxTE_CENTER;
pub const wxTE_RICH: c_int = 0x0080;
pub const wxTE_PROCESS_ENTER: c_int = 0x0400;
pub const wxTE_PASSWORD: c_int = 0x0800;
pub const wxTE_AUTO_URL: c_int = 0x1000;
pub const wxTE_NOHIDESEL: c_int = 0x2000;
pub const wxTE_DONTWRAP: c_long = wxHSCROLL;
pub const wxTE_CHARWRAP: c_int = 0x4000;
pub const wxTE_WORDWRAP: c_int = 0x0001;
pub const wxTE_BESTWRAP: c_int = 0x0000;
pub const wxTE_RICH2: c_int = 0x8000;
pub const wxTEXT_TYPE_ANY: c_int = 0;
//  ENUM: wxTextAttrAlignment
pub const wxTEXT_ALIGNMENT_DEFAULT: c_int = 0;
pub const wxTEXT_ALIGNMENT_LEFT: c_int = 0 + 1;
pub const wxTEXT_ALIGNMENT_CENTRE: c_int = 0 + 2;
pub const wxTEXT_ALIGNMENT_CENTER: c_int = wxTEXT_ALIGNMENT_CENTRE;
pub const wxTEXT_ALIGNMENT_RIGHT: c_int = wxTEXT_ALIGNMENT_CENTRE + 1;
pub const wxTEXT_ALIGNMENT_JUSTIFIED: c_int = wxTEXT_ALIGNMENT_CENTRE + 2;
//  ENUM: wxTextAttrFlags
pub const wxTEXT_ATTR_TEXT_COLOUR: c_int = 0x00000001;
pub const wxTEXT_ATTR_BACKGROUND_COLOUR: c_int = 0x00000002;
pub const wxTEXT_ATTR_FONT_FACE: c_int = 0x00000004;
pub const wxTEXT_ATTR_FONT_POINT_SIZE: c_int = 0x00000008;
pub const wxTEXT_ATTR_FONT_PIXEL_SIZE: c_int = 0x10000000;
pub const wxTEXT_ATTR_FONT_WEIGHT: c_int = 0x00000010;
pub const wxTEXT_ATTR_FONT_ITALIC: c_int = 0x00000020;
pub const wxTEXT_ATTR_FONT_UNDERLINE: c_int = 0x00000040;
pub const wxTEXT_ATTR_FONT_STRIKETHROUGH: c_int = 0x08000000;
pub const wxTEXT_ATTR_FONT_ENCODING: c_int = 0x02000000;
pub const wxTEXT_ATTR_FONT_FAMILY: c_int = 0x04000000;
pub const wxTEXT_ATTR_FONT_SIZE: c_int = 
        ( wxTEXT_ATTR_FONT_POINT_SIZE | wxTEXT_ATTR_FONT_PIXEL_SIZE );
pub const wxTEXT_ATTR_FONT: c_int = 
        ( wxTEXT_ATTR_FONT_FACE | wxTEXT_ATTR_FONT_SIZE | wxTEXT_ATTR_FONT_WEIGHT | 
            wxTEXT_ATTR_FONT_ITALIC | wxTEXT_ATTR_FONT_UNDERLINE | wxTEXT_ATTR_FONT_STRIKETHROUGH | wxTEXT_ATTR_FONT_ENCODING | wxTEXT_ATTR_FONT_FAMILY );
pub const wxTEXT_ATTR_ALIGNMENT: c_int = 0x00000080;
pub const wxTEXT_ATTR_LEFT_INDENT: c_int = 0x00000100;
pub const wxTEXT_ATTR_RIGHT_INDENT: c_int = 0x00000200;
pub const wxTEXT_ATTR_TABS: c_int = 0x00000400;
pub const wxTEXT_ATTR_PARA_SPACING_AFTER: c_int = 0x00000800;
pub const wxTEXT_ATTR_PARA_SPACING_BEFORE: c_int = 0x00001000;
pub const wxTEXT_ATTR_LINE_SPACING: c_int = 0x00002000;
pub const wxTEXT_ATTR_CHARACTER_STYLE_NAME: c_int = 0x00004000;
pub const wxTEXT_ATTR_PARAGRAPH_STYLE_NAME: c_int = 0x00008000;
pub const wxTEXT_ATTR_LIST_STYLE_NAME: c_int = 0x00010000;
pub const wxTEXT_ATTR_BULLET_STYLE: c_int = 0x00020000;
pub const wxTEXT_ATTR_BULLET_NUMBER: c_int = 0x00040000;
pub const wxTEXT_ATTR_BULLET_TEXT: c_int = 0x00080000;
pub const wxTEXT_ATTR_BULLET_NAME: c_int = 0x00100000;
pub const wxTEXT_ATTR_BULLET: c_int = 
        ( wxTEXT_ATTR_BULLET_STYLE | wxTEXT_ATTR_BULLET_NUMBER | wxTEXT_ATTR_BULLET_TEXT | 
          wxTEXT_ATTR_BULLET_NAME );
pub const wxTEXT_ATTR_URL: c_int = 0x00200000;
pub const wxTEXT_ATTR_PAGE_BREAK: c_int = 0x00400000;
pub const wxTEXT_ATTR_EFFECTS: c_int = 0x00800000;
pub const wxTEXT_ATTR_OUTLINE_LEVEL: c_int = 0x01000000;
pub const wxTEXT_ATTR_CHARACTER: c_int = 
        (wxTEXT_ATTR_FONT|wxTEXT_ATTR_EFFECTS| 
            wxTEXT_ATTR_BACKGROUND_COLOUR|wxTEXT_ATTR_TEXT_COLOUR|wxTEXT_ATTR_CHARACTER_STYLE_NAME|wxTEXT_ATTR_URL);
pub const wxTEXT_ATTR_PARAGRAPH: c_int = 
        (wxTEXT_ATTR_ALIGNMENT|wxTEXT_ATTR_LEFT_INDENT|wxTEXT_ATTR_RIGHT_INDENT|wxTEXT_ATTR_TABS|
            wxTEXT_ATTR_PARA_SPACING_BEFORE|wxTEXT_ATTR_PARA_SPACING_AFTER|wxTEXT_ATTR_LINE_SPACING|
            wxTEXT_ATTR_BULLET|wxTEXT_ATTR_PARAGRAPH_STYLE_NAME|wxTEXT_ATTR_LIST_STYLE_NAME|wxTEXT_ATTR_OUTLINE_LEVEL);
pub const wxTEXT_ATTR_ALL: c_int = (wxTEXT_ATTR_CHARACTER|wxTEXT_ATTR_PARAGRAPH);
//  ENUM: wxTextAttrBulletStyle
pub const wxTEXT_ATTR_BULLET_STYLE_NONE: c_int = 0x00000000;
pub const wxTEXT_ATTR_BULLET_STYLE_ARABIC: c_int = 0x00000001;
pub const wxTEXT_ATTR_BULLET_STYLE_LETTERS_UPPER: c_int = 0x00000002;
pub const wxTEXT_ATTR_BULLET_STYLE_LETTERS_LOWER: c_int = 0x00000004;
pub const wxTEXT_ATTR_BULLET_STYLE_ROMAN_UPPER: c_int = 0x00000008;
pub const wxTEXT_ATTR_BULLET_STYLE_ROMAN_LOWER: c_int = 0x00000010;
pub const wxTEXT_ATTR_BULLET_STYLE_SYMBOL: c_int = 0x00000020;
pub const wxTEXT_ATTR_BULLET_STYLE_BITMAP: c_int = 0x00000040;
pub const wxTEXT_ATTR_BULLET_STYLE_PARENTHESES: c_int = 0x00000080;
pub const wxTEXT_ATTR_BULLET_STYLE_PERIOD: c_int = 0x00000100;
pub const wxTEXT_ATTR_BULLET_STYLE_STANDARD: c_int = 0x00000200;
pub const wxTEXT_ATTR_BULLET_STYLE_RIGHT_PARENTHESIS: c_int = 0x00000400;
pub const wxTEXT_ATTR_BULLET_STYLE_OUTLINE: c_int = 0x00000800;
pub const wxTEXT_ATTR_BULLET_STYLE_ALIGN_LEFT: c_int = 0x00000000;
pub const wxTEXT_ATTR_BULLET_STYLE_ALIGN_RIGHT: c_int = 0x00001000;
pub const wxTEXT_ATTR_BULLET_STYLE_ALIGN_CENTRE: c_int = 0x00002000;
pub const wxTEXT_ATTR_BULLET_STYLE_CONTINUATION: c_int = 0x00004000;
//  ENUM: wxTextAttrEffects
pub const wxTEXT_ATTR_EFFECT_NONE: c_int = 0x00000000;
pub const wxTEXT_ATTR_EFFECT_CAPITALS: c_int = 0x00000001;
pub const wxTEXT_ATTR_EFFECT_SMALL_CAPITALS: c_int = 0x00000002;
pub const wxTEXT_ATTR_EFFECT_STRIKETHROUGH: c_int = 0x00000004;
pub const wxTEXT_ATTR_EFFECT_DOUBLE_STRIKETHROUGH: c_int = 0x00000008;
pub const wxTEXT_ATTR_EFFECT_SHADOW: c_int = 0x00000010;
pub const wxTEXT_ATTR_EFFECT_EMBOSS: c_int = 0x00000020;
pub const wxTEXT_ATTR_EFFECT_OUTLINE: c_int = 0x00000040;
pub const wxTEXT_ATTR_EFFECT_ENGRAVE: c_int = 0x00000080;
pub const wxTEXT_ATTR_EFFECT_SUPERSCRIPT: c_int = 0x00000100;
pub const wxTEXT_ATTR_EFFECT_SUBSCRIPT: c_int = 0x00000200;
//  ENUM: wxTextAttrLineSpacing
pub const wxTEXT_ATTR_LINE_SPACING_NORMAL: c_int = 10;
pub const wxTEXT_ATTR_LINE_SPACING_HALF: c_int = 15;
pub const wxTEXT_ATTR_LINE_SPACING_TWICE: c_int = 20;
//  ENUM: wxTextCtrlHitTestResult
pub const wxTE_HT_UNKNOWN: c_int = -2;
pub const wxTE_HT_BEFORE: c_int = -2 + 1;
pub const wxTE_HT_ON_TEXT: c_int = -2 + 2;
pub const wxTE_HT_BELOW: c_int = -2 + 3;
pub const wxTE_HT_BEYOND: c_int = -2 + 4;

pub const wxRICHTEXT_FIXED_WIDTH: c_int = 0x01;
pub const wxRICHTEXT_FIXED_HEIGHT: c_int = 0x02;
pub const wxRICHTEXT_VARIABLE_WIDTH: c_int = 0x04;
pub const wxRICHTEXT_VARIABLE_HEIGHT: c_int = 0x08;
pub const wxRICHTEXT_LAYOUT_SPECIFIED_RECT: c_int = 0x10;
pub const wxRICHTEXT_DRAW_IGNORE_CACHE: c_int = 0x01;
pub const wxRICHTEXT_DRAW_SELECTED: c_int = 0x02;
pub const wxRICHTEXT_DRAW_PRINT: c_int = 0x04;
pub const wxRICHTEXT_DRAW_GUIDELINES: c_int = 0x08;
pub const wxRICHTEXT_FORMATTED: c_int = 0x01;
pub const wxRICHTEXT_UNFORMATTED: c_int = 0x02;
pub const wxRICHTEXT_CACHE_SIZE: c_int = 0x04;
pub const wxRICHTEXT_HEIGHT_ONLY: c_int = 0x08;
pub const wxRICHTEXT_SETSTYLE_NONE: c_int = 0x00;
pub const wxRICHTEXT_SETSTYLE_WITH_UNDO: c_int = 0x01;
pub const wxRICHTEXT_SETSTYLE_OPTIMIZE: c_int = 0x02;
pub const wxRICHTEXT_SETSTYLE_PARAGRAPHS_ONLY: c_int = 0x04;
pub const wxRICHTEXT_SETSTYLE_CHARACTERS_ONLY: c_int = 0x08;
pub const wxRICHTEXT_SETSTYLE_RENUMBER: c_int = 0x10;
pub const wxRICHTEXT_SETSTYLE_SPECIFY_LEVEL: c_int = 0x20;
pub const wxRICHTEXT_SETSTYLE_RESET: c_int = 0x40;
pub const wxRICHTEXT_SETSTYLE_REMOVE: c_int = 0x80;
pub const wxRICHTEXT_SETPROPERTIES_NONE: c_int = 0x00;
pub const wxRICHTEXT_SETPROPERTIES_WITH_UNDO: c_int = 0x01;
pub const wxRICHTEXT_SETPROPERTIES_PARAGRAPHS_ONLY: c_int = 0x02;
pub const wxRICHTEXT_SETPROPERTIES_CHARACTERS_ONLY: c_int = 0x04;
pub const wxRICHTEXT_SETPROPERTIES_RESET: c_int = 0x08;
pub const wxRICHTEXT_SETPROPERTIES_REMOVE: c_int = 0x10;
pub const wxRICHTEXT_INSERT_NONE: c_int = 0x00;
pub const wxRICHTEXT_INSERT_WITH_PREVIOUS_PARAGRAPH_STYLE: c_int = 0x01;
pub const wxRICHTEXT_INSERT_INTERACTIVE: c_int = 0x02;
pub const wxTEXT_ATTR_KEEP_FIRST_PARA_STYLE: c_int = 0x10000000;
pub const wxSCRIPT_MUL_FACTOR: f32 = 1.5;
//  SKIP: wxRICHTEXT_ALL
//  SKIP: wxRICHTEXT_NONE
//  SKIP: wxRICHTEXT_NO_SELECTION
pub const wxRICHTEXT_HANDLER_INCLUDE_STYLESHEET: c_int = 0x0001;
pub const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_MEMORY: c_int = 0x0010;
pub const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_FILES: c_int = 0x0020;
pub const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_BASE64: c_int = 0x0040;
pub const wxRICHTEXT_HANDLER_NO_HEADER_FOOTER: c_int = 0x0080;
pub const wxRICHTEXT_HANDLER_CONVERT_FACENAMES: c_int = 0x0100;
//  ENUM: wxRichTextFileType
pub const wxRICHTEXT_TYPE_ANY: c_int = 0;
pub const wxRICHTEXT_TYPE_TEXT: c_int = 0 + 1;
pub const wxRICHTEXT_TYPE_XML: c_int = 0 + 2;
pub const wxRICHTEXT_TYPE_HTML: c_int = 0 + 3;
pub const wxRICHTEXT_TYPE_RTF: c_int = 0 + 4;
pub const wxRICHTEXT_TYPE_PDF: c_int = 0 + 5;
//  ENUM: wxRichTextHitTestFlags
pub const wxRICHTEXT_HITTEST_NONE: c_int =    0x01;
pub const wxRICHTEXT_HITTEST_BEFORE: c_int =  0x02;
pub const wxRICHTEXT_HITTEST_AFTER: c_int =   0x04;
pub const wxRICHTEXT_HITTEST_ON: c_int =      0x08;
pub const wxRICHTEXT_HITTEST_OUTSIDE: c_int = 0x10;
pub const wxRICHTEXT_HITTEST_NO_NESTED_OBJECTS: c_int = 0x20;
pub const wxRICHTEXT_HITTEST_NO_FLOATING_OBJECTS: c_int = 0x40;
pub const wxRICHTEXT_HITTEST_HONOUR_ATOMIC: c_int = 0x80;
//  ENUM: wxTextBoxAttrFlags
pub const wxTEXT_BOX_ATTR_FLOAT: c_int = 0x00000001;
pub const wxTEXT_BOX_ATTR_CLEAR: c_int = 0x00000002;
pub const wxTEXT_BOX_ATTR_COLLAPSE_BORDERS: c_int = 0x00000004;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT: c_int = 0x00000008;
//  ENUM: wxTextAttrValueFlags
pub const wxTEXT_ATTR_VALUE_VALID: c_int = 0x1000;
pub const wxTEXT_ATTR_VALUE_VALID_MASK: c_int = 0x1000;
//  ENUM: wxTextAttrUnits
pub const wxTEXT_ATTR_UNITS_TENTHS_MM: c_int = 0x0001;
pub const wxTEXT_ATTR_UNITS_PIXELS: c_int = 0x0002;
pub const wxTEXT_ATTR_UNITS_PERCENTAGE: c_int = 0x0004;
pub const wxTEXT_ATTR_UNITS_POINTS: c_int = 0x0008;
pub const wxTEXT_ATTR_UNITS_HUNDREDTHS_POINT: c_int = 0x0100;
pub const wxTEXT_ATTR_UNITS_MASK: c_int = 0x010F;
//  ENUM: wxTextBoxAttrPosition
pub const wxTEXT_BOX_ATTR_POSITION_STATIC: c_int = 0x0000;
pub const wxTEXT_BOX_ATTR_POSITION_RELATIVE: c_int = 0x0010;
pub const wxTEXT_BOX_ATTR_POSITION_ABSOLUTE: c_int = 0x0020;
pub const wxTEXT_BOX_ATTR_POSITION_FIXED: c_int = 0x0040;
pub const wxTEXT_BOX_ATTR_POSITION_MASK: c_int = 0x00F0;
//  ENUM: wxTextAttrBorderStyle
pub const wxTEXT_BOX_ATTR_BORDER_NONE: c_int = 0;
pub const wxTEXT_BOX_ATTR_BORDER_SOLID: c_int = 1;
pub const wxTEXT_BOX_ATTR_BORDER_DOTTED: c_int = 2;
pub const wxTEXT_BOX_ATTR_BORDER_DASHED: c_int = 3;
pub const wxTEXT_BOX_ATTR_BORDER_DOUBLE: c_int = 4;
pub const wxTEXT_BOX_ATTR_BORDER_GROOVE: c_int = 5;
pub const wxTEXT_BOX_ATTR_BORDER_RIDGE: c_int = 6;
pub const wxTEXT_BOX_ATTR_BORDER_INSET: c_int = 7;
pub const wxTEXT_BOX_ATTR_BORDER_OUTSET: c_int = 8;
//  ENUM: wxTextAttrBorderFlags
pub const wxTEXT_BOX_ATTR_BORDER_STYLE: c_int = 0x0001;
pub const wxTEXT_BOX_ATTR_BORDER_COLOUR: c_int = 0x0002;
//  ENUM: wxTextAttrBorderWidth
pub const wxTEXT_BOX_ATTR_BORDER_THIN: c_int = -1;
pub const wxTEXT_BOX_ATTR_BORDER_MEDIUM: c_int = -2;
pub const wxTEXT_BOX_ATTR_BORDER_THICK: c_int = -3;
//  ENUM: wxTextBoxAttrFloatStyle
pub const wxTEXT_BOX_ATTR_FLOAT_NONE: c_int = 0;
pub const wxTEXT_BOX_ATTR_FLOAT_LEFT: c_int = 1;
pub const wxTEXT_BOX_ATTR_FLOAT_RIGHT: c_int = 2;
//  ENUM: wxTextBoxAttrClearStyle
pub const wxTEXT_BOX_ATTR_CLEAR_NONE: c_int = 0;
pub const wxTEXT_BOX_ATTR_CLEAR_LEFT: c_int = 1;
pub const wxTEXT_BOX_ATTR_CLEAR_RIGHT: c_int = 2;
pub const wxTEXT_BOX_ATTR_CLEAR_BOTH: c_int = 3;
//  ENUM: wxTextBoxAttrCollapseMode
pub const wxTEXT_BOX_ATTR_COLLAPSE_NONE: c_int = 0;
pub const wxTEXT_BOX_ATTR_COLLAPSE_FULL: c_int = 1;
//  ENUM: wxTextBoxAttrVerticalAlignment
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_NONE: c_int =       0;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_TOP: c_int =       1;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_CENTRE: c_int =     2;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_BOTTOM: c_int =    3;
//  ENUM: wxRichTextCommandId
pub const wxRICHTEXT_INSERT: c_int = 0;
pub const wxRICHTEXT_DELETE: c_int = 0 + 1;
pub const wxRICHTEXT_CHANGE_ATTRIBUTES: c_int = 0 + 2;
pub const wxRICHTEXT_CHANGE_STYLE: c_int = 0 + 3;
pub const wxRICHTEXT_CHANGE_OBJECT: c_int = 0 + 4;

//  ENUM: wxAuiManagerDock
pub const wxAUI_DOCK_NONE: c_int = 0;
pub const wxAUI_DOCK_TOP: c_int = 1;
pub const wxAUI_DOCK_RIGHT: c_int = 2;
pub const wxAUI_DOCK_BOTTOM: c_int = 3;
pub const wxAUI_DOCK_LEFT: c_int = 4;
pub const wxAUI_DOCK_CENTER: c_int = 5;
pub const wxAUI_DOCK_CENTRE: c_int = wxAUI_DOCK_CENTER;
//  ENUM: wxAuiManagerOption
pub const wxAUI_MGR_ALLOW_FLOATING: c_int = 1 << 0;
pub const wxAUI_MGR_ALLOW_ACTIVE_PANE: c_int = 1 << 1;
pub const wxAUI_MGR_TRANSPARENT_DRAG: c_int = 1 << 2;
pub const wxAUI_MGR_TRANSPARENT_HINT: c_int = 1 << 3;
pub const wxAUI_MGR_VENETIAN_BLINDS_HINT: c_int = 1 << 4;
pub const wxAUI_MGR_RECTANGLE_HINT: c_int = 1 << 5;
pub const wxAUI_MGR_HINT_FADE: c_int = 1 << 6;
pub const wxAUI_MGR_NO_VENETIAN_BLINDS_FADE: c_int = 1 << 7;
pub const wxAUI_MGR_LIVE_RESIZE: c_int = 1 << 8;
pub const wxAUI_MGR_DEFAULT: c_int = wxAUI_MGR_ALLOW_FLOATING |
                        wxAUI_MGR_TRANSPARENT_HINT |
                        wxAUI_MGR_HINT_FADE |
                        wxAUI_MGR_NO_VENETIAN_BLINDS_FADE;

//  ENUM: wxMediaState
pub const wxMEDIASTATE_STOPPED: c_int = 0;
pub const wxMEDIASTATE_PAUSED: c_int = 0 + 1;
pub const wxMEDIASTATE_PLAYING: c_int = 0 + 2;
//  ENUM: wxMediaCtrlPlayerControls
pub const wxMEDIACTRLPLAYERCONTROLS_NONE: c_int =   0;
pub const wxMEDIACTRLPLAYERCONTROLS_STEP: c_int =   1 << 0;
pub const wxMEDIACTRLPLAYERCONTROLS_VOLUME: c_int =   1 << 1;
pub const wxMEDIACTRLPLAYERCONTROLS_DEFAULT: c_int =
                    wxMEDIACTRLPLAYERCONTROLS_STEP |
                    wxMEDIACTRLPLAYERCONTROLS_VOLUME;

pub const wxRICHTEXT_ORGANISER_DELETE_STYLES: c_int = 0x0001;
pub const wxRICHTEXT_ORGANISER_CREATE_STYLES: c_int = 0x0002;
pub const wxRICHTEXT_ORGANISER_APPLY_STYLES: c_int = 0x0004;
pub const wxRICHTEXT_ORGANISER_EDIT_STYLES: c_int = 0x0008;
pub const wxRICHTEXT_ORGANISER_RENAME_STYLES: c_int = 0x0010;
pub const wxRICHTEXT_ORGANISER_OK_CANCEL: c_int = 0x0020;
pub const wxRICHTEXT_ORGANISER_RENUMBER: c_int = 0x0040;
pub const wxRICHTEXT_ORGANISER_SHOW_CHARACTER: c_int = 0x0100;
pub const wxRICHTEXT_ORGANISER_SHOW_PARAGRAPH: c_int = 0x0200;
pub const wxRICHTEXT_ORGANISER_SHOW_LIST: c_int = 0x0400;
pub const wxRICHTEXT_ORGANISER_SHOW_BOX: c_int = 0x0800;
pub const wxRICHTEXT_ORGANISER_SHOW_ALL: c_int = 0x1000;
pub const wxRICHTEXT_ORGANISER_ORGANISE: c_int = (wxRICHTEXT_ORGANISER_SHOW_ALL|wxRICHTEXT_ORGANISER_DELETE_STYLES|wxRICHTEXT_ORGANISER_CREATE_STYLES|wxRICHTEXT_ORGANISER_APPLY_STYLES|wxRICHTEXT_ORGANISER_EDIT_STYLES|wxRICHTEXT_ORGANISER_RENAME_STYLES);
pub const wxRICHTEXT_ORGANISER_BROWSE: c_int = (wxRICHTEXT_ORGANISER_SHOW_ALL|wxRICHTEXT_ORGANISER_OK_CANCEL);
pub const wxRICHTEXT_ORGANISER_BROWSE_NUMBERING: c_int = (wxRICHTEXT_ORGANISER_SHOW_LIST|wxRICHTEXT_ORGANISER_OK_CANCEL|wxRICHTEXT_ORGANISER_RENUMBER);

pub const wxBITMAP_SCREEN_DEPTH: c_int = (-1);

//  ENUM: BufMode

// NODEF: wxCHECK_VERSION
// NODEF: wxCHECK_VERSION_FULL

//  ENUM: @9
pub const wxCONFIG_USE_LOCAL_FILE: c_int = 1;
pub const wxCONFIG_USE_GLOBAL_FILE: c_int = 2;
pub const wxCONFIG_USE_RELATIVE_PATH: c_int = 4;
pub const wxCONFIG_USE_NO_ESCAPE_CHARACTERS: c_int = 8;
pub const wxCONFIG_USE_SUBDIR: c_int = 16;

//  ENUM: @30
pub const FALLBACK_NONE: c_int = 0;
pub const FALLBACK_SYSTEM: c_int = 1;
pub const FALLBACK_NEAREST_LARGER: c_int = 2;

pub const wxPREVIEW_PRINT: c_int = 1;
pub const wxPREVIEW_PREVIOUS: c_int = 2;
pub const wxPREVIEW_NEXT: c_int = 4;
pub const wxPREVIEW_ZOOM: c_int = 8;
pub const wxPREVIEW_FIRST: c_int = 16;
pub const wxPREVIEW_LAST: c_int = 32;
pub const wxPREVIEW_GOTO: c_int = 64;
pub const wxPREVIEW_DEFAULT: c_int = (wxPREVIEW_PREVIOUS|wxPREVIEW_NEXT|wxPREVIEW_ZOOM|wxPREVIEW_FIRST|wxPREVIEW_GOTO|wxPREVIEW_LAST);
pub const wxID_PREVIEW_CLOSE: c_int = 1;
pub const wxID_PREVIEW_NEXT: c_int = 2;
pub const wxID_PREVIEW_PREVIOUS: c_int = 3;
pub const wxID_PREVIEW_PRINT: c_int = 4;
pub const wxID_PREVIEW_ZOOM: c_int = 5;
pub const wxID_PREVIEW_FIRST: c_int = 6;
pub const wxID_PREVIEW_LAST: c_int = 7;
pub const wxID_PREVIEW_GOTO: c_int = 8;
pub const wxID_PREVIEW_ZOOM_IN: c_int = 9;
pub const wxID_PREVIEW_ZOOM_OUT: c_int = 10;
//  ENUM: wxPrinterError
pub const wxPRINTER_NO_ERROR: c_int = 0;
pub const wxPRINTER_CANCELLED: c_int = 0 + 1;
pub const wxPRINTER_ERROR: c_int = 0 + 2;
//  ENUM: wxPreviewFrameModalityKind
pub const wxPreviewFrame_AppModal: c_int = 0;
pub const wxPreviewFrame_WindowModal: c_int = 0 + 1;
pub const wxPreviewFrame_NonModal: c_int = 0 + 2;

//  ENUM: @42
pub const wxRE_EXTENDED: c_int = 0;
pub const wxRE_ADVANCED: c_int = 1;
pub const wxRE_BASIC: c_int = 2;
pub const wxRE_ICASE: c_int = 4;
pub const wxRE_NOSUB: c_int = 8;
pub const wxRE_NEWLINE: c_int = 16;
pub const wxRE_DEFAULT: c_int = wxRE_EXTENDED;
//  ENUM: @43
pub const wxRE_NOTBOL: c_int = 32;
pub const wxRE_NOTEOL: c_int = 64;

//  ENUM: HTMLCursor
pub const HTMLCursor_Default: c_int = 0;
pub const HTMLCursor_Link: c_int = 0 + 1;
pub const HTMLCursor_Text: c_int = 0 + 2;

//  ENUM: Origin
pub const Origin_Unknown: c_int = 0;
pub const Origin_Keyboard: c_int = 0 + 1;
pub const Origin_HelpButton: c_int = 0 + 2;

//  ENUM: wxCmdLineEntryFlags
pub const wxCMD_LINE_OPTION_MANDATORY: c_int = 0x01;
pub const wxCMD_LINE_PARAM_OPTIONAL: c_int = 0x02;
pub const wxCMD_LINE_PARAM_MULTIPLE: c_int = 0x04;
pub const wxCMD_LINE_OPTION_HELP: c_int = 0x08;
pub const wxCMD_LINE_NEEDS_SEPARATOR: c_int = 0x10;
pub const wxCMD_LINE_SWITCH_NEGATABLE: c_int = 0x20;
//  ENUM: wxCmdLineParamType
pub const wxCMD_LINE_VAL_STRING: c_int = 0;
pub const wxCMD_LINE_VAL_NUMBER: c_int = 0 + 1;
pub const wxCMD_LINE_VAL_DATE: c_int = 0 + 2;
pub const wxCMD_LINE_VAL_DOUBLE: c_int = 0 + 3;
pub const wxCMD_LINE_VAL_NONE: c_int = 0 + 4;
//  ENUM: wxCmdLineEntryType
pub const wxCMD_LINE_SWITCH: c_int = 0;
pub const wxCMD_LINE_OPTION: c_int = 0 + 1;
pub const wxCMD_LINE_PARAM: c_int = 0 + 2;
pub const wxCMD_LINE_USAGE_TEXT: c_int = 0 + 3;
pub const wxCMD_LINE_NONE: c_int = 0 + 4;
//  ENUM: wxCmdLineSwitchState
pub const wxCMD_SWITCH_OFF: c_int = 0;
pub const wxCMD_SWITCH_ON: c_int = 0 + 1;
//  ENUM: wxCmdLineSplitType
pub const wxCMD_LINE_SPLIT_DOS: c_int = 0;
pub const wxCMD_LINE_SPLIT_UNIX: c_int = 0 + 1;

pub const wxBK_DEFAULT: c_int = 0x0000;
pub const wxBK_TOP: c_int = 0x0010;
pub const wxBK_BOTTOM: c_int = 0x0020;
pub const wxBK_LEFT: c_int = 0x0040;
pub const wxBK_RIGHT: c_int = 0x0080;
pub const wxBK_ALIGN_MASK: c_int = (wxBK_TOP | wxBK_BOTTOM | wxBK_LEFT | wxBK_RIGHT);
//  SKIP: wxBookCtrl
//  ENUM: @2
pub const wxBK_HITTEST_NOWHERE: c_int = 1;
pub const wxBK_HITTEST_ONICON: c_int = 2;
pub const wxBK_HITTEST_ONLABEL: c_int = 4;
pub const wxBK_HITTEST_ONITEM: c_int = wxBK_HITTEST_ONICON | wxBK_HITTEST_ONLABEL;
pub const wxBK_HITTEST_ONPAGE: c_int = 8;

pub const wxSP_NOBORDER: c_int = 0x0000;
pub const wxSP_THIN_SASH: c_int = 0x0000;
pub const wxSP_NOSASH: c_int = 0x0010;
pub const wxSP_PERMIT_UNSPLIT: c_int = 0x0040;
pub const wxSP_LIVE_UPDATE: c_int = 0x0080;
pub const wxSP_3DSASH: c_int = 0x0100;
pub const wxSP_3DBORDER: c_int = 0x0200;
pub const wxSP_NO_XP_THEME: c_int = 0x0400;
pub const wxSP_BORDER: c_int = wxSP_3DBORDER;
pub const wxSP_3D: c_int = (wxSP_3DBORDER | wxSP_3DSASH);
//  ENUM: wxSplitMode
pub const wxSPLIT_HORIZONTAL: c_int = 1;
pub const wxSPLIT_VERTICAL: c_int = 1 + 1;
//  ENUM: @48
pub const wxSPLIT_DRAG_NONE: c_int = 0;
pub const wxSPLIT_DRAG_DRAGGING: c_int = 0 + 1;
pub const wxSPLIT_DRAG_LEFT_DOWN: c_int = 0 + 2;

pub const wxWIZARD_EX_HELPBUTTON: c_int = 0x00000010;
pub const wxWIZARD_VALIGN_TOP: c_int = 0x01;
pub const wxWIZARD_VALIGN_CENTRE: c_int = 0x02;
pub const wxWIZARD_VALIGN_BOTTOM: c_int = 0x04;
pub const wxWIZARD_HALIGN_LEFT: c_int = 0x08;
pub const wxWIZARD_HALIGN_CENTRE: c_int = 0x10;
pub const wxWIZARD_HALIGN_RIGHT: c_int = 0x20;
pub const wxWIZARD_TILE: c_int = 0x40;

//  ENUM: wxNavigationKeyEventFlags
pub const IsBackward: c_int = 0x0000;
pub const IsForward: c_int = 0x0001;
pub const WinChange: c_int = 0x0002;
pub const FromTab: c_int = 0x0004;

//  ENUM: @28
pub const wxWEBKIT_STATE_START: c_int = 1;
pub const wxWEBKIT_STATE_NEGOTIATING: c_int = 2;
pub const wxWEBKIT_STATE_REDIRECTING: c_int = 4;
pub const wxWEBKIT_STATE_TRANSFERRING: c_int = 8;
pub const wxWEBKIT_STATE_STOP: c_int = 16;
pub const wxWEBKIT_STATE_FAILED: c_int = 32;
//  ENUM: @29
pub const wxWEBKIT_NAV_LINK_CLICKED: c_int = 1;
pub const wxWEBKIT_NAV_BACK_NEXT: c_int = 2;
pub const wxWEBKIT_NAV_FORM_SUBMITTED: c_int = 4;
pub const wxWEBKIT_NAV_RELOAD: c_int = 8;
pub const wxWEBKIT_NAV_FORM_RESUBMITTED: c_int = 16;
pub const wxWEBKIT_NAV_OTHER: c_int = 32;

pub const wxDD_CHANGE_DIR: c_int = 0x0100;
pub const wxDD_DIR_MUST_EXIST: c_int = 0x0200;
pub const wxDD_NEW_DIR_BUTTON: c_int = 0;
pub const wxDD_DEFAULT_STYLE: c_long = (wxDEFAULT_DIALOG_STYLE|wxRESIZE_BORDER);

//  SKIP: wxTLS_TYPE
// NODEF: wxTLS_VALUE
// NODEF: wxTLS_PTR

//  ENUM: @44
pub const wxCONTROL_DISABLED: c_int = 0x00000001;
pub const wxCONTROL_FOCUSED: c_int = 0x00000002;
pub const wxCONTROL_PRESSED: c_int = 0x00000004;
pub const wxCONTROL_SPECIAL: c_int = 0x00000008;
pub const wxCONTROL_ISDEFAULT: c_int = wxCONTROL_SPECIAL;
pub const wxCONTROL_ISSUBMENU: c_int = wxCONTROL_SPECIAL;
pub const wxCONTROL_EXPANDED: c_int = wxCONTROL_SPECIAL;
pub const wxCONTROL_SIZEGRIP: c_int = wxCONTROL_SPECIAL;
pub const wxCONTROL_FLAT: c_int = wxCONTROL_SPECIAL;
pub const wxCONTROL_CURRENT: c_int = 0x00000010;
pub const wxCONTROL_SELECTED: c_int = 0x00000020;
pub const wxCONTROL_CHECKED: c_int = 0x00000040;
pub const wxCONTROL_CHECKABLE: c_int = 0x00000080;
pub const wxCONTROL_UNDETERMINED: c_int = wxCONTROL_CHECKABLE;
//  ENUM: wxTitleBarButton
pub const wxTITLEBAR_BUTTON_CLOSE: c_int = 0x01000000;
pub const wxTITLEBAR_BUTTON_MAXIMIZE: c_int = 0x02000000;
pub const wxTITLEBAR_BUTTON_ICONIZE: c_int = 0x04000000;
pub const wxTITLEBAR_BUTTON_RESTORE: c_int = 0x08000000;
pub const wxTITLEBAR_BUTTON_HELP: c_int = 0x10000000;
//  ENUM: wxHeaderSortIconType
pub const wxHDR_SORT_ICON_NONE: c_int = 0;
pub const wxHDR_SORT_ICON_UP: c_int = 0 + 1;
pub const wxHDR_SORT_ICON_DOWN: c_int = 0 + 2;

//  ENUM: wxNumValidatorStyle

// NODEF: wxON_BLOCK_EXIT
// NODEF: wxON_BLOCK_EXIT0
// NODEF: wxON_BLOCK_EXIT1
// NODEF: wxON_BLOCK_EXIT2
// NODEF: wxON_BLOCK_EXIT3
// NODEF: wxON_BLOCK_EXIT_OBJ
// NODEF: wxON_BLOCK_EXIT_OBJ0
// NODEF: wxON_BLOCK_EXIT_OBJ1
// NODEF: wxON_BLOCK_EXIT_OBJ2
// NODEF: wxON_BLOCK_EXIT_OBJ3
// NODEF: wxON_BLOCK_EXIT_THIS
// NODEF: wxON_BLOCK_EXIT_THIS0
// NODEF: wxON_BLOCK_EXIT_THIS1
// NODEF: wxON_BLOCK_EXIT_THIS2
// NODEF: wxON_BLOCK_EXIT_THIS3
// NODEF: wxON_BLOCK_EXIT_SET
// NODEF: wxON_BLOCK_EXIT_NULL

//  ENUM: wxLanguage
pub const wxLANGUAGE_DEFAULT: c_int = 0;
pub const wxLANGUAGE_UNKNOWN: c_int = 0 + 1;
pub const wxLANGUAGE_ABKHAZIAN: c_int = 0 + 2;
pub const wxLANGUAGE_AFAR: c_int = 0 + 3;
pub const wxLANGUAGE_AFRIKAANS: c_int = 0 + 4;
pub const wxLANGUAGE_ALBANIAN: c_int = 0 + 5;
pub const wxLANGUAGE_AMHARIC: c_int = 0 + 6;
pub const wxLANGUAGE_ARABIC: c_int = 0 + 7;
pub const wxLANGUAGE_ARABIC_ALGERIA: c_int = 0 + 8;
pub const wxLANGUAGE_ARABIC_BAHRAIN: c_int = 0 + 9;
pub const wxLANGUAGE_ARABIC_EGYPT: c_int = 0 + 10;
pub const wxLANGUAGE_ARABIC_IRAQ: c_int = 0 + 11;
pub const wxLANGUAGE_ARABIC_JORDAN: c_int = 0 + 12;
pub const wxLANGUAGE_ARABIC_KUWAIT: c_int = 0 + 13;
pub const wxLANGUAGE_ARABIC_LEBANON: c_int = 0 + 14;
pub const wxLANGUAGE_ARABIC_LIBYA: c_int = 0 + 15;
pub const wxLANGUAGE_ARABIC_MOROCCO: c_int = 0 + 16;
pub const wxLANGUAGE_ARABIC_OMAN: c_int = 0 + 17;
pub const wxLANGUAGE_ARABIC_QATAR: c_int = 0 + 18;
pub const wxLANGUAGE_ARABIC_SAUDI_ARABIA: c_int = 0 + 19;
pub const wxLANGUAGE_ARABIC_SUDAN: c_int = 0 + 20;
pub const wxLANGUAGE_ARABIC_SYRIA: c_int = 0 + 21;
pub const wxLANGUAGE_ARABIC_TUNISIA: c_int = 0 + 22;
pub const wxLANGUAGE_ARABIC_UAE: c_int = 0 + 23;
pub const wxLANGUAGE_ARABIC_YEMEN: c_int = 0 + 24;
pub const wxLANGUAGE_ARMENIAN: c_int = 0 + 25;
pub const wxLANGUAGE_ASSAMESE: c_int = 0 + 26;
pub const wxLANGUAGE_ASTURIAN: c_int = 0 + 27;
pub const wxLANGUAGE_AYMARA: c_int = 0 + 28;
pub const wxLANGUAGE_AZERI: c_int = 0 + 29;
pub const wxLANGUAGE_AZERI_CYRILLIC: c_int = 0 + 30;
pub const wxLANGUAGE_AZERI_LATIN: c_int = 0 + 31;
pub const wxLANGUAGE_BASHKIR: c_int = 0 + 32;
pub const wxLANGUAGE_BASQUE: c_int = 0 + 33;
pub const wxLANGUAGE_BELARUSIAN: c_int = 0 + 34;
pub const wxLANGUAGE_BENGALI: c_int = 0 + 35;
pub const wxLANGUAGE_BHUTANI: c_int = 0 + 36;
pub const wxLANGUAGE_BIHARI: c_int = 0 + 37;
pub const wxLANGUAGE_BISLAMA: c_int = 0 + 38;
pub const wxLANGUAGE_BOSNIAN: c_int = 0 + 39;
pub const wxLANGUAGE_BRETON: c_int = 0 + 40;
pub const wxLANGUAGE_BULGARIAN: c_int = 0 + 41;
pub const wxLANGUAGE_BURMESE: c_int = 0 + 42;
pub const wxLANGUAGE_CAMBODIAN: c_int = 0 + 43;
pub const wxLANGUAGE_CATALAN: c_int = 0 + 44;
pub const wxLANGUAGE_CHINESE: c_int = 0 + 45;
pub const wxLANGUAGE_CHINESE_SIMPLIFIED: c_int = 0 + 46;
pub const wxLANGUAGE_CHINESE_TRADITIONAL: c_int = 0 + 47;
pub const wxLANGUAGE_CHINESE_HONGKONG: c_int = 0 + 48;
pub const wxLANGUAGE_CHINESE_MACAU: c_int = 0 + 49;
pub const wxLANGUAGE_CHINESE_SINGAPORE: c_int = 0 + 50;
pub const wxLANGUAGE_CHINESE_TAIWAN: c_int = 0 + 51;
pub const wxLANGUAGE_CORSICAN: c_int = 0 + 52;
pub const wxLANGUAGE_CROATIAN: c_int = 0 + 53;
pub const wxLANGUAGE_CZECH: c_int = 0 + 54;
pub const wxLANGUAGE_DANISH: c_int = 0 + 55;
pub const wxLANGUAGE_DUTCH: c_int = 0 + 56;
pub const wxLANGUAGE_DUTCH_BELGIAN: c_int = 0 + 57;
pub const wxLANGUAGE_ENGLISH: c_int = 0 + 58;
pub const wxLANGUAGE_ENGLISH_UK: c_int = 0 + 59;
pub const wxLANGUAGE_ENGLISH_US: c_int = 0 + 60;
pub const wxLANGUAGE_ENGLISH_AUSTRALIA: c_int = 0 + 61;
pub const wxLANGUAGE_ENGLISH_BELIZE: c_int = 0 + 62;
pub const wxLANGUAGE_ENGLISH_BOTSWANA: c_int = 0 + 63;
pub const wxLANGUAGE_ENGLISH_CANADA: c_int = 0 + 64;
pub const wxLANGUAGE_ENGLISH_CARIBBEAN: c_int = 0 + 65;
pub const wxLANGUAGE_ENGLISH_DENMARK: c_int = 0 + 66;
pub const wxLANGUAGE_ENGLISH_EIRE: c_int = 0 + 67;
pub const wxLANGUAGE_ENGLISH_JAMAICA: c_int = 0 + 68;
pub const wxLANGUAGE_ENGLISH_NEW_ZEALAND: c_int = 0 + 69;
pub const wxLANGUAGE_ENGLISH_PHILIPPINES: c_int = 0 + 70;
pub const wxLANGUAGE_ENGLISH_SOUTH_AFRICA: c_int = 0 + 71;
pub const wxLANGUAGE_ENGLISH_TRINIDAD: c_int = 0 + 72;
pub const wxLANGUAGE_ENGLISH_ZIMBABWE: c_int = 0 + 73;
pub const wxLANGUAGE_ESPERANTO: c_int = 0 + 74;
pub const wxLANGUAGE_ESTONIAN: c_int = 0 + 75;
pub const wxLANGUAGE_FAEROESE: c_int = 0 + 76;
pub const wxLANGUAGE_FARSI: c_int = 0 + 77;
pub const wxLANGUAGE_FIJI: c_int = 0 + 78;
pub const wxLANGUAGE_FINNISH: c_int = 0 + 79;
pub const wxLANGUAGE_FRENCH: c_int = 0 + 80;
pub const wxLANGUAGE_FRENCH_BELGIAN: c_int = 0 + 81;
pub const wxLANGUAGE_FRENCH_CANADIAN: c_int = 0 + 82;
pub const wxLANGUAGE_FRENCH_LUXEMBOURG: c_int = 0 + 83;
pub const wxLANGUAGE_FRENCH_MONACO: c_int = 0 + 84;
pub const wxLANGUAGE_FRENCH_SWISS: c_int = 0 + 85;
pub const wxLANGUAGE_FRISIAN: c_int = 0 + 86;
pub const wxLANGUAGE_GALICIAN: c_int = 0 + 87;
pub const wxLANGUAGE_GEORGIAN: c_int = 0 + 88;
pub const wxLANGUAGE_GERMAN: c_int = 0 + 89;
pub const wxLANGUAGE_GERMAN_AUSTRIAN: c_int = 0 + 90;
pub const wxLANGUAGE_GERMAN_BELGIUM: c_int = 0 + 91;
pub const wxLANGUAGE_GERMAN_LIECHTENSTEIN: c_int = 0 + 92;
pub const wxLANGUAGE_GERMAN_LUXEMBOURG: c_int = 0 + 93;
pub const wxLANGUAGE_GERMAN_SWISS: c_int = 0 + 94;
pub const wxLANGUAGE_GREEK: c_int = 0 + 95;
pub const wxLANGUAGE_GREENLANDIC: c_int = 0 + 96;
pub const wxLANGUAGE_GUARANI: c_int = 0 + 97;
pub const wxLANGUAGE_GUJARATI: c_int = 0 + 98;
pub const wxLANGUAGE_HAUSA: c_int = 0 + 99;
pub const wxLANGUAGE_HEBREW: c_int = 0 + 100;
pub const wxLANGUAGE_HINDI: c_int = 0 + 101;
pub const wxLANGUAGE_HUNGARIAN: c_int = 0 + 102;
pub const wxLANGUAGE_ICELANDIC: c_int = 0 + 103;
pub const wxLANGUAGE_INDONESIAN: c_int = 0 + 104;
pub const wxLANGUAGE_INTERLINGUA: c_int = 0 + 105;
pub const wxLANGUAGE_INTERLINGUE: c_int = 0 + 106;
pub const wxLANGUAGE_INUKTITUT: c_int = 0 + 107;
pub const wxLANGUAGE_INUPIAK: c_int = 0 + 108;
pub const wxLANGUAGE_IRISH: c_int = 0 + 109;
pub const wxLANGUAGE_ITALIAN: c_int = 0 + 110;
pub const wxLANGUAGE_ITALIAN_SWISS: c_int = 0 + 111;
pub const wxLANGUAGE_JAPANESE: c_int = 0 + 112;
pub const wxLANGUAGE_JAVANESE: c_int = 0 + 113;
pub const wxLANGUAGE_KANNADA: c_int = 0 + 114;
pub const wxLANGUAGE_KASHMIRI: c_int = 0 + 115;
pub const wxLANGUAGE_KASHMIRI_INDIA: c_int = 0 + 116;
pub const wxLANGUAGE_KAZAKH: c_int = 0 + 117;
pub const wxLANGUAGE_KERNEWEK: c_int = 0 + 118;
pub const wxLANGUAGE_KINYARWANDA: c_int = 0 + 119;
pub const wxLANGUAGE_KIRGHIZ: c_int = 0 + 120;
pub const wxLANGUAGE_KIRUNDI: c_int = 0 + 121;
pub const wxLANGUAGE_KONKANI: c_int = 0 + 122;
pub const wxLANGUAGE_KOREAN: c_int = 0 + 123;
pub const wxLANGUAGE_KURDISH: c_int = 0 + 124;
pub const wxLANGUAGE_LAOTHIAN: c_int = 0 + 125;
pub const wxLANGUAGE_LATIN: c_int = 0 + 126;
pub const wxLANGUAGE_LATVIAN: c_int = 0 + 127;
pub const wxLANGUAGE_LINGALA: c_int = 0 + 128;
pub const wxLANGUAGE_LITHUANIAN: c_int = 0 + 129;
pub const wxLANGUAGE_MACEDONIAN: c_int = 0 + 130;
pub const wxLANGUAGE_MALAGASY: c_int = 0 + 131;
pub const wxLANGUAGE_MALAY: c_int = 0 + 132;
pub const wxLANGUAGE_MALAYALAM: c_int = 0 + 133;
pub const wxLANGUAGE_MALAY_BRUNEI_DARUSSALAM: c_int = 0 + 134;
pub const wxLANGUAGE_MALAY_MALAYSIA: c_int = 0 + 135;
pub const wxLANGUAGE_MALTESE: c_int = 0 + 136;
pub const wxLANGUAGE_MANIPURI: c_int = 0 + 137;
pub const wxLANGUAGE_MAORI: c_int = 0 + 138;
pub const wxLANGUAGE_MARATHI: c_int = 0 + 139;
pub const wxLANGUAGE_MOLDAVIAN: c_int = 0 + 140;
pub const wxLANGUAGE_MONGOLIAN: c_int = 0 + 141;
pub const wxLANGUAGE_NAURU: c_int = 0 + 142;
pub const wxLANGUAGE_NEPALI: c_int = 0 + 143;
pub const wxLANGUAGE_NEPALI_INDIA: c_int = 0 + 144;
pub const wxLANGUAGE_NORWEGIAN_BOKMAL: c_int = 0 + 145;
pub const wxLANGUAGE_NORWEGIAN_NYNORSK: c_int = 0 + 146;
pub const wxLANGUAGE_OCCITAN: c_int = 0 + 147;
pub const wxLANGUAGE_ORIYA: c_int = 0 + 148;
pub const wxLANGUAGE_OROMO: c_int = 0 + 149;
pub const wxLANGUAGE_PASHTO: c_int = 0 + 150;
pub const wxLANGUAGE_POLISH: c_int = 0 + 151;
pub const wxLANGUAGE_PORTUGUESE: c_int = 0 + 152;
pub const wxLANGUAGE_PORTUGUESE_BRAZILIAN: c_int = 0 + 153;
pub const wxLANGUAGE_PUNJABI: c_int = 0 + 154;
pub const wxLANGUAGE_QUECHUA: c_int = 0 + 155;
pub const wxLANGUAGE_RHAETO_ROMANCE: c_int = 0 + 156;
pub const wxLANGUAGE_ROMANIAN: c_int = 0 + 157;
pub const wxLANGUAGE_RUSSIAN: c_int = 0 + 158;
pub const wxLANGUAGE_RUSSIAN_UKRAINE: c_int = 0 + 159;
pub const wxLANGUAGE_SAMI: c_int = 0 + 160;
pub const wxLANGUAGE_SAMOAN: c_int = 0 + 161;
pub const wxLANGUAGE_SANGHO: c_int = 0 + 162;
pub const wxLANGUAGE_SANSKRIT: c_int = 0 + 163;
pub const wxLANGUAGE_SCOTS_GAELIC: c_int = 0 + 164;
pub const wxLANGUAGE_SERBIAN: c_int = 0 + 165;
pub const wxLANGUAGE_SERBIAN_CYRILLIC: c_int = 0 + 166;
pub const wxLANGUAGE_SERBIAN_LATIN: c_int = 0 + 167;
pub const wxLANGUAGE_SERBO_CROATIAN: c_int = 0 + 168;
pub const wxLANGUAGE_SESOTHO: c_int = 0 + 169;
pub const wxLANGUAGE_SETSWANA: c_int = 0 + 170;
pub const wxLANGUAGE_SHONA: c_int = 0 + 171;
pub const wxLANGUAGE_SINDHI: c_int = 0 + 172;
pub const wxLANGUAGE_SINHALESE: c_int = 0 + 173;
pub const wxLANGUAGE_SISWATI: c_int = 0 + 174;
pub const wxLANGUAGE_SLOVAK: c_int = 0 + 175;
pub const wxLANGUAGE_SLOVENIAN: c_int = 0 + 176;
pub const wxLANGUAGE_SOMALI: c_int = 0 + 177;
pub const wxLANGUAGE_SPANISH: c_int = 0 + 178;
pub const wxLANGUAGE_SPANISH_ARGENTINA: c_int = 0 + 179;
pub const wxLANGUAGE_SPANISH_BOLIVIA: c_int = 0 + 180;
pub const wxLANGUAGE_SPANISH_CHILE: c_int = 0 + 181;
pub const wxLANGUAGE_SPANISH_COLOMBIA: c_int = 0 + 182;
pub const wxLANGUAGE_SPANISH_COSTA_RICA: c_int = 0 + 183;
pub const wxLANGUAGE_SPANISH_DOMINICAN_REPUBLIC: c_int = 0 + 184;
pub const wxLANGUAGE_SPANISH_ECUADOR: c_int = 0 + 185;
pub const wxLANGUAGE_SPANISH_EL_SALVADOR: c_int = 0 + 186;
pub const wxLANGUAGE_SPANISH_GUATEMALA: c_int = 0 + 187;
pub const wxLANGUAGE_SPANISH_HONDURAS: c_int = 0 + 188;
pub const wxLANGUAGE_SPANISH_MEXICAN: c_int = 0 + 189;
pub const wxLANGUAGE_SPANISH_MODERN: c_int = 0 + 190;
pub const wxLANGUAGE_SPANISH_NICARAGUA: c_int = 0 + 191;
pub const wxLANGUAGE_SPANISH_PANAMA: c_int = 0 + 192;
pub const wxLANGUAGE_SPANISH_PARAGUAY: c_int = 0 + 193;
pub const wxLANGUAGE_SPANISH_PERU: c_int = 0 + 194;
pub const wxLANGUAGE_SPANISH_PUERTO_RICO: c_int = 0 + 195;
pub const wxLANGUAGE_SPANISH_URUGUAY: c_int = 0 + 196;
pub const wxLANGUAGE_SPANISH_US: c_int = 0 + 197;
pub const wxLANGUAGE_SPANISH_VENEZUELA: c_int = 0 + 198;
pub const wxLANGUAGE_SUNDANESE: c_int = 0 + 199;
pub const wxLANGUAGE_SWAHILI: c_int = 0 + 200;
pub const wxLANGUAGE_SWEDISH: c_int = 0 + 201;
pub const wxLANGUAGE_SWEDISH_FINLAND: c_int = 0 + 202;
pub const wxLANGUAGE_TAGALOG: c_int = 0 + 203;
pub const wxLANGUAGE_TAJIK: c_int = 0 + 204;
pub const wxLANGUAGE_TAMIL: c_int = 0 + 205;
pub const wxLANGUAGE_TATAR: c_int = 0 + 206;
pub const wxLANGUAGE_TELUGU: c_int = 0 + 207;
pub const wxLANGUAGE_THAI: c_int = 0 + 208;
pub const wxLANGUAGE_TIBETAN: c_int = 0 + 209;
pub const wxLANGUAGE_TIGRINYA: c_int = 0 + 210;
pub const wxLANGUAGE_TONGA: c_int = 0 + 211;
pub const wxLANGUAGE_TSONGA: c_int = 0 + 212;
pub const wxLANGUAGE_TURKISH: c_int = 0 + 213;
pub const wxLANGUAGE_TURKMEN: c_int = 0 + 214;
pub const wxLANGUAGE_TWI: c_int = 0 + 215;
pub const wxLANGUAGE_UIGHUR: c_int = 0 + 216;
pub const wxLANGUAGE_UKRAINIAN: c_int = 0 + 217;
pub const wxLANGUAGE_URDU: c_int = 0 + 218;
pub const wxLANGUAGE_URDU_INDIA: c_int = 0 + 219;
pub const wxLANGUAGE_URDU_PAKISTAN: c_int = 0 + 220;
pub const wxLANGUAGE_UZBEK: c_int = 0 + 221;
pub const wxLANGUAGE_UZBEK_CYRILLIC: c_int = 0 + 222;
pub const wxLANGUAGE_UZBEK_LATIN: c_int = 0 + 223;
pub const wxLANGUAGE_VALENCIAN: c_int = 0 + 224;
pub const wxLANGUAGE_VIETNAMESE: c_int = 0 + 225;
pub const wxLANGUAGE_VOLAPUK: c_int = 0 + 226;
pub const wxLANGUAGE_WELSH: c_int = 0 + 227;
pub const wxLANGUAGE_WOLOF: c_int = 0 + 228;
pub const wxLANGUAGE_XHOSA: c_int = 0 + 229;
pub const wxLANGUAGE_YIDDISH: c_int = 0 + 230;
pub const wxLANGUAGE_YORUBA: c_int = 0 + 231;
pub const wxLANGUAGE_ZHUANG: c_int = 0 + 232;
pub const wxLANGUAGE_ZULU: c_int = 0 + 233;
pub const wxLANGUAGE_KABYLE: c_int = 0 + 234;
pub const wxLANGUAGE_USER_DEFINED: c_int = 0 + 235;

//  ENUM: wxTipKind
pub const wxTipKind_None: c_int = 0;
pub const wxTipKind_TopLeft: c_int = 0 + 1;
pub const wxTipKind_Top: c_int = 0 + 2;
pub const wxTipKind_TopRight: c_int = 0 + 3;
pub const wxTipKind_BottomLeft: c_int = 0 + 4;
pub const wxTipKind_Bottom: c_int = 0 + 5;
pub const wxTipKind_BottomRight: c_int = 0 + 6;
pub const wxTipKind_Auto: c_int = 0 + 7;

//  ENUM: @8
pub const MovableButton: c_int = 0x0001;
pub const BitmapButton: c_int = 0x0002;
pub const ButtonSpacing: c_int = 0x0004;
pub const TextIndent: c_int = 0x0008;
pub const PaintControl: c_int = 0x0010;
pub const PaintWritable: c_int = 0x0020;
pub const Borderless: c_int = 0x0040;
pub const All: c_int = MovableButton | BitmapButton | ButtonSpacing |
                          TextIndent | PaintControl | PaintWritable |
                          Borderless;

//  ENUM: wxEllipsizeFlags
pub const wxELLIPSIZE_FLAGS_NONE: c_int = 0;
pub const wxELLIPSIZE_FLAGS_PROCESS_MNEMONICS: c_int = 1;
pub const wxELLIPSIZE_FLAGS_EXPAND_TABS: c_int = 2;
pub const wxELLIPSIZE_FLAGS_DEFAULT: c_int = wxELLIPSIZE_FLAGS_PROCESS_MNEMONICS|
                                wxELLIPSIZE_FLAGS_EXPAND_TABS;
//  ENUM: wxEllipsizeMode
pub const wxELLIPSIZE_NONE: c_int = 0;
pub const wxELLIPSIZE_START: c_int = 0 + 1;
pub const wxELLIPSIZE_MIDDLE: c_int = 0 + 2;
pub const wxELLIPSIZE_END: c_int = 0 + 3;

pub const wxBU_LEFT: c_int = 0x0040;
pub const wxBU_TOP: c_int = 0x0080;
pub const wxBU_RIGHT: c_int = 0x0100;
pub const wxBU_BOTTOM: c_int = 0x0200;
pub const wxBU_ALIGN_MASK: c_int = ( wxBU_LEFT | wxBU_TOP | wxBU_RIGHT | wxBU_BOTTOM );
pub const wxBU_EXACTFIT: c_int = 0x0001;
pub const wxBU_NOTEXT: c_int = 0x0002;
pub const wxBU_AUTODRAW: c_int = 0x0004;

//  ENUM: wxBOM
pub const wxBOM_Unknown: c_int = -1;
pub const wxBOM_None: c_int = -1 + 1;
pub const wxBOM_UTF32BE: c_int = -1 + 2;
pub const wxBOM_UTF32LE: c_int = -1 + 3;
pub const wxBOM_UTF16BE: c_int = -1 + 4;
pub const wxBOM_UTF16LE: c_int = -1 + 5;
pub const wxBOM_UTF8: c_int = -1 + 6;

//  ENUM: wxAttrKind
pub const Any: c_int = 0;
pub const Cell: c_int = 0 + 1;
pub const Row: c_int = 0 + 2;
pub const Col: c_int = 0 + 3;
pub const Default: c_int = 0 + 4;
pub const Merged: c_int = 0 + 5;

pub const wxHELP_NETSCAPE: c_int = 1;
//  ENUM: wxHelpSearchMode
pub const wxHELP_SEARCH_INDEX: c_int = 0;
pub const wxHELP_SEARCH_ALL: c_int = 0 + 1;

//  ENUM: @57

