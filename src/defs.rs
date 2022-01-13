#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use crate::manual::*;

pub const wxNB_DEFAULT: u32 = wxBK_DEFAULT;
pub const wxNB_TOP: u32 = wxBK_TOP;
pub const wxNB_BOTTOM: u32 = wxBK_BOTTOM;
pub const wxNB_LEFT: u32 = wxBK_LEFT;
pub const wxNB_RIGHT: u32 = wxBK_RIGHT;
pub const wxNB_FIXEDWIDTH: u32 = 0x0100;
pub const wxNB_MULTILINE: u32 = 0x0200;
pub const wxNB_NOPAGETHEME: u32 = 0x0400;
//  ENUM: @37
pub const wxNB_HITTEST_NOWHERE: u32 = wxBK_HITTEST_NOWHERE;
pub const wxNB_HITTEST_ONICON: u32 = wxBK_HITTEST_ONICON;
pub const wxNB_HITTEST_ONLABEL: u32 = wxBK_HITTEST_ONLABEL;
pub const wxNB_HITTEST_ONITEM: u32 = wxBK_HITTEST_ONITEM;
pub const wxNB_HITTEST_ONPAGE: u32 = wxBK_HITTEST_ONPAGE;

pub const wxPG_ATTR_DEFAULT_VALUE: &str = "DefaultValue";
pub const wxPG_ATTR_MIN: &str = "Min";
pub const wxPG_ATTR_MAX: &str = "Max";
pub const wxPG_ATTR_UNITS: &str = "Units";
pub const wxPG_ATTR_HINT: &str = "Hint";
pub const wxPG_ATTR_AUTOCOMPLETE: &str = "AutoComplete";
pub const wxPG_BOOL_USE_CHECKBOX: &str = "UseCheckbox";
pub const wxPG_BOOL_USE_DOUBLE_CLICK_CYCLING: &str = "UseDClickCycling";
pub const wxPG_FLOAT_PRECISION: &str = "Precision";
pub const wxPG_STRING_PASSWORD: &str = "Password";
pub const wxPG_UINT_BASE: &str = "Base";
pub const wxPG_UINT_PREFIX: &str = "Prefix";
pub const wxPG_DIALOG_TITLE: &str = "DialogTitle";
pub const wxPG_FILE_WILDCARD: &str = "Wildcard";
pub const wxPG_FILE_SHOW_FULL_PATH: &str = "ShowFullPath";
pub const wxPG_FILE_SHOW_RELATIVE_PATH: &str = "ShowRelativePath";
pub const wxPG_FILE_INITIAL_PATH: &str = "InitialPath";
pub const wxPG_FILE_DIALOG_STYLE: &str = "DialogStyle";
pub const wxPG_ARRAY_DELIMITER: &str = "Delimiter";
pub const wxPG_DATE_FORMAT: &str = "DateFormat";
pub const wxPG_DATE_PICKER_STYLE: &str = "PickerStyle";
pub const wxPG_ATTR_SPINCTRL_STEP: &str = "Step";
pub const wxPG_ATTR_SPINCTRL_WRAP: &str = "Wrap";
pub const wxPG_ATTR_SPINCTRL_MOTION: &str = "MotionSpin";
pub const wxPG_ATTR_MULTICHOICE_USERSTRINGMODE: &str = "UserStringMode";
pub const wxPG_COLOUR_ALLOW_CUSTOM: &str = "AllowCustom";
pub const wxPG_COLOUR_HAS_ALPHA: &str = "HasAlpha";
pub const wxPG_PROP_MAX: u32 = wxPG_PROP_AUTO_UNSPECIFIED;
//  SKIP: wxPG_PROP_PARENTAL_FLAGS
pub const wxPG_STRING_STORED_FLAGS: u32 = (wxPG_PROP_DISABLED|wxPG_PROP_HIDDEN|wxPG_PROP_NOEDITOR|wxPG_PROP_COLLAPSED);
//  SKIP: wxNullProperty
//  SKIP: wxPGChoicesEmptyData
//  ENUM: wxPGPropertyFlags
pub const wxPG_PROP_MODIFIED: u32 = 0x0001;
pub const wxPG_PROP_DISABLED: u32 = 0x0002;
pub const wxPG_PROP_HIDDEN: u32 = 0x0004;
pub const wxPG_PROP_CUSTOMIMAGE: u32 = 0x0008;
pub const wxPG_PROP_NOEDITOR: u32 = 0x0010;
pub const wxPG_PROP_COLLAPSED: u32 = 0x0020;
pub const wxPG_PROP_INVALID_VALUE: u32 = 0x0040;
pub const wxPG_PROP_WAS_MODIFIED: u32 = 0x0200;
pub const wxPG_PROP_AGGREGATE: u32 = 0x0400;
pub const wxPG_PROP_CHILDREN_ARE_COPIES: u32 = 0x0800;
pub const wxPG_PROP_PROPERTY: u32 = 0x1000;
pub const wxPG_PROP_CATEGORY: u32 = 0x2000;
pub const wxPG_PROP_MISC_PARENT: u32 = 0x4000;
pub const wxPG_PROP_READONLY: u32 = 0x8000;
pub const wxPG_PROP_COMPOSED_VALUE: u32 = 0x00010000;
pub const wxPG_PROP_USES_COMMON_VALUE: u32 = 0x00020000;
pub const wxPG_PROP_AUTO_UNSPECIFIED: u32 = 0x00040000;
pub const wxPG_PROP_CLASS_SPECIFIC_1: u32 = 0x00080000;
pub const wxPG_PROP_CLASS_SPECIFIC_2: u32 = 0x00100000;
//  SKIP: wxPG_PROP_BEING_DELETED

pub const wxHW_SCROLLBAR_NEVER: u32 = 0x0002;
pub const wxHW_SCROLLBAR_AUTO: u32 = 0x0004;
pub const wxHW_NO_SELECTION: u32 = 0x0008;
pub const wxHW_DEFAULT_STYLE: u32 = wxHW_SCROLLBAR_AUTO;
//  ENUM: wxHtmlOpeningStatus
pub const wxHTML_OPEN: u32 = 0;
pub const wxHTML_BLOCK: u32 = 0 + 1;
pub const wxHTML_REDIRECT: u32 = 0 + 2;

pub const wxFRAME_NO_TASKBAR: u32 = 0x0002;
pub const wxFRAME_TOOL_WINDOW: u32 = 0x0004;
pub const wxFRAME_FLOAT_ON_PARENT: u32 = 0x0008;

//  ENUM: @16
pub const Event_Skip: u32 = -1;
pub const Event_Ignore: u32 = 0;
pub const Event_Processed: u32 = 1;

//  ENUM: @4
pub const wxCAL_SUNDAY_FIRST: u32 = 0x0080;
pub const wxCAL_MONDAY_FIRST: u32 = 0x0001;
pub const wxCAL_SHOW_HOLIDAYS: u32 = 0x0002;
pub const wxCAL_NO_YEAR_CHANGE: u32 = 0x0004;
pub const wxCAL_NO_MONTH_CHANGE: u32 = 0x000c;
pub const wxCAL_SEQUENTIAL_MONTH_SELECTION: u32 = 0x0010;
pub const wxCAL_SHOW_SURROUNDING_WEEKS: u32 = 0x0020;
pub const wxCAL_SHOW_WEEK_NUMBERS: u32 = 0x0040;
//  ENUM: wxCalendarDateBorder
pub const wxCAL_BORDER_NONE: u32 = 0;
pub const wxCAL_BORDER_SQUARE: u32 = 0 + 1;
pub const wxCAL_BORDER_ROUND: u32 = 0 + 2;
//  ENUM: wxCalendarHitTestResult
pub const wxCAL_HITTEST_NOWHERE: u32 = 0;
pub const wxCAL_HITTEST_HEADER: u32 = 0 + 1;
pub const wxCAL_HITTEST_DAY: u32 = 0 + 2;
pub const wxCAL_HITTEST_INCMONTH: u32 = 0 + 3;
pub const wxCAL_HITTEST_DECMONTH: u32 = 0 + 4;
pub const wxCAL_HITTEST_SURROUNDING_WEEK: u32 = 0 + 5;
pub const wxCAL_HITTEST_WEEK: u32 = 0 + 6;

//  ENUM: OpenMode
pub const read: u32 = 0;
pub const write: u32 = 0 + 1;
pub const read_write: u32 = 0 + 2;
pub const write_append: u32 = 0 + 3;
pub const write_excl: u32 = 0 + 4;
//  ENUM: @17
pub const fd_invalid: u32 = -1;
pub const fd_stdin: u32 = -1 + 1;
pub const fd_stdout: u32 = -1 + 2;
pub const fd_stderr: u32 = -1 + 3;

//  ENUM: wxStreamError
pub const wxSTREAM_NO_ERROR: u32 = 0;
pub const wxSTREAM_EOF: u32 = 0 + 1;
pub const wxSTREAM_WRITE_ERROR: u32 = 0 + 2;
pub const wxSTREAM_READ_ERROR: u32 = 0 + 3;
//  ENUM: wxStreamProtocolType
pub const wxSTREAM_PROTOCOL: u32 = 0;
pub const wxSTREAM_MIMETYPE: u32 = 0 + 1;
pub const wxSTREAM_ENCODING: u32 = 0 + 2;
pub const wxSTREAM_FILEEXT: u32 = 0 + 3;

//  ENUM: wxHtmlSelectionState
pub const wxHTML_SEL_OUT: u32 = 0;
pub const wxHTML_SEL_IN: u32 = 0 + 1;
pub const wxHTML_SEL_CHANGING: u32 = 0 + 2;
//  ENUM: @27
pub const wxHTML_FIND_EXACT: u32 = 1;
pub const wxHTML_FIND_NEAREST_BEFORE: u32 = 2;
pub const wxHTML_FIND_NEAREST_AFTER: u32 = 4;
//  ENUM: wxHtmlScriptMode
pub const wxHTML_SCRIPT_NORMAL: u32 = 0;
pub const wxHTML_SCRIPT_SUB: u32 = 0 + 1;
pub const wxHTML_SCRIPT_SUP: u32 = 0 + 2;

//  ENUM: wxProtocolError
pub const wxPROTO_NOERR: u32 = 0;
pub const wxPROTO_NETERR: u32 = 0 + 1;
pub const wxPROTO_PROTERR: u32 = 0 + 2;
pub const wxPROTO_CONNERR: u32 = 0 + 3;
pub const wxPROTO_INVVAL: u32 = 0 + 4;
pub const wxPROTO_NOHNDLR: u32 = 0 + 5;
pub const wxPROTO_NOFILE: u32 = 0 + 6;
pub const wxPROTO_ABRT: u32 = 0 + 7;
pub const wxPROTO_RCNCT: u32 = 0 + 8;
pub const wxPROTO_STREAMING: u32 = 0 + 9;

// NODEF: wxDECLARE_APP
// NODEF: wxIMPLEMENT_APP
//  SKIP: wxDISABLE_DEBUG_SUPPORT

//  ENUM: wxLayoutOrientation
pub const wxLAYOUT_HORIZONTAL: u32 = 0;
pub const wxLAYOUT_VERTICAL: u32 = 0 + 1;
//  ENUM: wxLayoutAlignment
pub const wxLAYOUT_NONE: u32 = 0;
pub const wxLAYOUT_TOP: u32 = 0 + 1;
pub const wxLAYOUT_LEFT: u32 = 0 + 2;
pub const wxLAYOUT_RIGHT: u32 = 0 + 3;
pub const wxLAYOUT_BOTTOM: u32 = 0 + 4;

//  SKIP: wxPG_LABEL
pub const wxPG_LABEL_STRING: &str = "@!";
//  SKIP: wxPG_COLOUR_BLACK
//  SKIP: wxPG_COLOUR
//  SKIP: wxPG_DEFAULT_IMAGE_SIZE
//  ENUM: wxPG_PROPERTYVALUES_FLAGS
pub const wxPG_DONT_RECURSE: u32 = 0x00000000;
pub const wxPG_KEEP_STRUCTURE: u32 = 0x00000010;
pub const wxPG_RECURSE: u32 = 0x00000020;
pub const wxPG_INC_ATTRIBUTES: u32 = 0x00000040;
pub const wxPG_RECURSE_STARTS: u32 = 0x00000080;
pub const wxPG_FORCE: u32 = 0x00000100;
pub const wxPG_SORT_TOP_LEVEL_ONLY: u32 = 0x00000200;

pub const wxSTB_SIZEGRIP: u32 = 0x0010;
pub const wxSTB_SHOW_TIPS: u32 = 0x0020;
pub const wxSTB_ELLIPSIZE_START: u32 = 0x0040;
pub const wxSTB_ELLIPSIZE_MIDDLE: u32 = 0x0080;
pub const wxSTB_ELLIPSIZE_END: u32 = 0x0100;
pub const wxSTB_DEFAULT_STYLE: u32 = (wxSTB_SIZEGRIP|wxSTB_ELLIPSIZE_END|wxSTB_SHOW_TIPS|wxFULL_REPAINT_ON_RESIZE);
pub const wxSB_NORMAL: u32 = 0x0000;
pub const wxSB_FLAT: u32 = 0x0001;
pub const wxSB_RAISED: u32 = 0x0002;
pub const wxSB_SUNKEN: u32 = 0x0003;

//  ENUM: wxSVGShapeRenderingMode
pub const wxSVG_SHAPE_RENDERING_AUTO: u32 = 0;
pub const wxSVG_SHAPE_RENDERING_OPTIMIZE_SPEED: u32 = 0 + 1;
pub const wxSVG_SHAPE_RENDERING_CRISP_EDGES: u32 = 0 + 2;
pub const wxSVG_SHAPE_RENDERING_GEOMETRIC_PRECISION: u32 = 0 + 3;
pub const wxSVG_SHAPE_RENDERING_OPTIMISE_SPEED: u32 = wxSVG_SHAPE_RENDERING_OPTIMIZE_SPEED;

//  ENUM: wxScrollbarVisibility
pub const wxSHOW_SB_NEVER: u32 = -1;
pub const wxSHOW_SB_DEFAULT: u32 = -1 + 1;
pub const wxSHOW_SB_ALWAYS: u32 = -1 + 2;

pub const wxSW_NOBORDER: u32 = 0x0000;
pub const wxSW_BORDER: u32 = 0x0020;
pub const wxSW_3DSASH: u32 = 0x0040;
pub const wxSW_3DBORDER: u32 = 0x0080;
pub const wxSW_3D: u32 = (wxSW_3DSASH | wxSW_3DBORDER);
//  ENUM: wxSashEdgePosition
pub const wxSASH_TOP: u32 = 0;
pub const wxSASH_RIGHT: u32 = 0 + 1;
pub const wxSASH_BOTTOM: u32 = 0 + 2;
pub const wxSASH_LEFT: u32 = 0 + 3;
pub const wxSASH_NONE: u32 = 100;
//  ENUM: wxSashDragStatus
pub const wxSASH_STATUS_OK: u32 = 0;
pub const wxSASH_STATUS_OUT_OF_RANGE: u32 = 0 + 1;

pub const wxBUFFER_VIRTUAL_AREA: u32 = 0x01;
pub const wxBUFFER_CLIENT_AREA: u32 = 0x02;
pub const wxBUFFER_USES_SHARED_BUFFER: u32 = 0x04;

//  ENUM: @10
pub const wxCONFIG_USE_LOCAL_FILE: u32 = 1;
pub const wxCONFIG_USE_GLOBAL_FILE: u32 = 2;
pub const wxCONFIG_USE_RELATIVE_PATH: u32 = 4;
pub const wxCONFIG_USE_NO_ESCAPE_CHARACTERS: u32 = 8;
pub const wxCONFIG_USE_SUBDIR: u32 = 16;

//  ENUM: PromptMode
pub const Prompt_Never: u32 = 0;
pub const Prompt_Once: u32 = 0 + 1;
pub const Prompt_Always: u32 = 0 + 2;

//  ENUM: wxBase64DecodeMode
pub const wxBase64DecodeMode_Strict: u32 = 0;
pub const wxBase64DecodeMode_SkipWS: u32 = 0 + 1;
pub const wxBase64DecodeMode_Relaxed: u32 = 0 + 2;

//  ENUM: wxPrintBin
pub const wxPRINTBIN_DEFAULT: u32 = 0;
pub const wxPRINTBIN_ONLYONE: u32 = 0 + 1;
pub const wxPRINTBIN_LOWER: u32 = 0 + 2;
pub const wxPRINTBIN_MIDDLE: u32 = 0 + 3;
pub const wxPRINTBIN_MANUAL: u32 = 0 + 4;
pub const wxPRINTBIN_ENVELOPE: u32 = 0 + 5;
pub const wxPRINTBIN_ENVMANUAL: u32 = 0 + 6;
pub const wxPRINTBIN_AUTO: u32 = 0 + 7;
pub const wxPRINTBIN_TRACTOR: u32 = 0 + 8;
pub const wxPRINTBIN_SMALLFMT: u32 = 0 + 9;
pub const wxPRINTBIN_LARGEFMT: u32 = 0 + 10;
pub const wxPRINTBIN_LARGECAPACITY: u32 = 0 + 11;
pub const wxPRINTBIN_CASSETTE: u32 = 0 + 12;
pub const wxPRINTBIN_FORMSOURCE: u32 = 0 + 13;
pub const wxPRINTBIN_USER: u32 = 0 + 14;

// NODEF: WXTRACE
// NODEF: WXTRACELEVEL

pub const wxSP_NOBORDER: u32 = 0x0000;
pub const wxSP_THIN_SASH: u32 = 0x0000;
pub const wxSP_NOSASH: u32 = 0x0010;
pub const wxSP_PERMIT_UNSPLIT: u32 = 0x0040;
pub const wxSP_LIVE_UPDATE: u32 = 0x0080;
pub const wxSP_3DSASH: u32 = 0x0100;
pub const wxSP_3DBORDER: u32 = 0x0200;
pub const wxSP_NO_XP_THEME: u32 = 0x0400;
pub const wxSP_BORDER: u32 = wxSP_3DBORDER;
pub const wxSP_3D: u32 = (wxSP_3DBORDER | wxSP_3DSASH);
//  ENUM: wxSplitMode
pub const wxSPLIT_HORIZONTAL: u32 = 1;
pub const wxSPLIT_VERTICAL: u32 = 1 + 1;
//  ENUM: @47
pub const wxSPLIT_DRAG_NONE: u32 = 0;
pub const wxSPLIT_DRAG_DRAGGING: u32 = 0 + 1;
pub const wxSPLIT_DRAG_LEFT_DOWN: u32 = 0 + 2;

//  ENUM: wxFSWFlags
pub const wxFSW_EVENT_CREATE: u32 = 0x01;
pub const wxFSW_EVENT_DELETE: u32 = 0x02;
pub const wxFSW_EVENT_RENAME: u32 = 0x04;
pub const wxFSW_EVENT_MODIFY: u32 = 0x08;
pub const wxFSW_EVENT_ACCESS: u32 = 0x10;
pub const wxFSW_EVENT_ATTRIB: u32 = 0x20;
pub const wxFSW_EVENT_UNMOUNT: u32 = 0x2000;
pub const wxFSW_EVENT_WARNING: u32 = 0x40;
pub const wxFSW_EVENT_ERROR: u32 = 0x80;
pub const wxFSW_EVENT_ALL: u32 = wxFSW_EVENT_CREATE | wxFSW_EVENT_DELETE |
                         wxFSW_EVENT_RENAME | wxFSW_EVENT_MODIFY |
                         wxFSW_EVENT_ACCESS | wxFSW_EVENT_ATTRIB |
                         wxFSW_EVENT_WARNING | wxFSW_EVENT_ERROR;
//  ENUM: wxFSWWarningType
pub const wxFSW_WARNING_NONE: u32 = 0;
pub const wxFSW_WARNING_GENERAL: u32 = 0 + 1;
pub const wxFSW_WARNING_OVERFLOW: u32 = 0 + 2;

//  ENUM: wxAnimationDisposal
pub const wxANIM_UNSPECIFIED: u32 = -1;
pub const wxANIM_DONOTREMOVE: u32 = 0;
pub const wxANIM_TOBACKGROUND: u32 = 1;
pub const wxANIM_TOPREVIOUS: u32 = 2;

//  ENUM: wxZlibCompressionLevels
pub const wxZ_DEFAULT_COMPRESSION: u32 = -1;
pub const wxZ_NO_COMPRESSION: u32 = 0;
pub const wxZ_BEST_SPEED: u32 = 1;
pub const wxZ_BEST_COMPRESSION: u32 = 9;
//  ENUM: wxZLibFlags
pub const wxZLIB_NO_HEADER: u32 = 0;
pub const wxZLIB_ZLIB: u32 = 1;
pub const wxZLIB_GZIP: u32 = 2;
pub const wxZLIB_AUTO: u32 = 3;

// NODEF: wxDYNLIB_FUNCTION
//  ENUM: wxDynamicLibraryCategory
pub const wxDL_LIBRARY: u32 = 0;
pub const wxDL_MODULE: u32 = 0 + 1;
//  ENUM: wxPluginCategory
pub const wxDL_PLUGIN_GUI: u32 = 0;
pub const wxDL_PLUGIN_BASE: u32 = 0 + 1;

//  ENUM: wxLayoutDirection
pub const wxLayout_Default: u32 = 0;
pub const wxLayout_LeftToRight: u32 = 0 + 1;
pub const wxLayout_RightToLeft: u32 = 0 + 2;
//  ENUM: wxLocaleCategory
pub const wxLOCALE_CAT_NUMBER: u32 = 0;
pub const wxLOCALE_CAT_DATE: u32 = 0 + 1;
pub const wxLOCALE_CAT_MONEY: u32 = 0 + 2;
pub const wxLOCALE_CAT_DEFAULT: u32 = 0 + 3;
//  ENUM: wxLocaleInfo
pub const wxLOCALE_THOUSANDS_SEP: u32 = 0;
pub const wxLOCALE_DECIMAL_POINT: u32 = 0 + 1;
pub const wxLOCALE_SHORT_DATE_FMT: u32 = 0 + 2;
pub const wxLOCALE_LONG_DATE_FMT: u32 = 0 + 3;
pub const wxLOCALE_DATE_TIME_FMT: u32 = 0 + 4;
pub const wxLOCALE_TIME_FMT: u32 = 0 + 5;
//  ENUM: wxLocaleInitFlags
pub const wxLOCALE_DONT_LOAD_DEFAULT: u32 = 0x0000;
pub const wxLOCALE_LOAD_DEFAULT: u32 = 0x0001;

//  ENUM: @1
pub const typeCaption: u32 = 0;
pub const typeGripper: u32 = 0 + 1;
pub const typeDock: u32 = 0 + 2;
pub const typeDockSizer: u32 = 0 + 3;
pub const typePane: u32 = 0 + 4;
pub const typePaneSizer: u32 = 0 + 5;
pub const typeBackground: u32 = 0 + 6;
pub const typePaneBorder: u32 = 0 + 7;
pub const typePaneButton: u32 = 0 + 8;

pub const wxDIALOG_NO_PARENT: u32 = 0x00000020;
pub const wxDEFAULT_DIALOG_STYLE: u32 = (wxCAPTION | wxSYSTEM_MENU | wxCLOSE_BOX);
pub const wxDIALOG_ADAPTATION_NONE: u32 = 0;
pub const wxDIALOG_ADAPTATION_STANDARD_SIZER: u32 = 1;
pub const wxDIALOG_ADAPTATION_ANY_SIZER: u32 = 2;
pub const wxDIALOG_ADAPTATION_LOOSE_BUTTONS: u32 = 3;
//  ENUM: wxDialogLayoutAdaptationMode
pub const wxDIALOG_ADAPTATION_MODE_DEFAULT: u32 = 0;
pub const wxDIALOG_ADAPTATION_MODE_ENABLED: u32 = 1;
pub const wxDIALOG_ADAPTATION_MODE_DISABLED: u32 = 2;

//  ENUM: HTMLCursor
pub const HTMLCursor_Default: u32 = 0;
pub const HTMLCursor_Link: u32 = 0 + 1;
pub const HTMLCursor_Text: u32 = 0 + 2;

//  ENUM: Reason
pub const Reason_Mouse: u32 = 0;
pub const Reason_Unknown: u32 = 0 + 1;

pub const wxAC_NO_AUTORESIZE: u32 = (0x0010);
pub const wxAC_DEFAULT_STYLE: u32 = (wxBORDER_NONE);
//  ENUM: wxAnimationType
pub const wxANIMATION_TYPE_INVALID: u32 = 0;
pub const wxANIMATION_TYPE_GIF: u32 = 0 + 1;
pub const wxANIMATION_TYPE_ANI: u32 = 0 + 2;
pub const wxANIMATION_TYPE_ANY: u32 = 0 + 3;

pub const wxSPLASH_CENTRE_ON_PARENT: u32 = 0x01;
pub const wxSPLASH_CENTRE_ON_SCREEN: u32 = 0x02;
pub const wxSPLASH_NO_CENTRE: u32 = 0x00;
pub const wxSPLASH_TIMEOUT: u32 = 0x04;
pub const wxSPLASH_NO_TIMEOUT: u32 = 0x00;

pub const wxPREVIEW_PRINT: u32 = 1;
pub const wxPREVIEW_PREVIOUS: u32 = 2;
pub const wxPREVIEW_NEXT: u32 = 4;
pub const wxPREVIEW_ZOOM: u32 = 8;
pub const wxPREVIEW_FIRST: u32 = 16;
pub const wxPREVIEW_LAST: u32 = 32;
pub const wxPREVIEW_GOTO: u32 = 64;
pub const wxPREVIEW_DEFAULT: u32 = (wxPREVIEW_PREVIOUS|wxPREVIEW_NEXT|wxPREVIEW_ZOOM|wxPREVIEW_FIRST|wxPREVIEW_GOTO|wxPREVIEW_LAST);
pub const wxID_PREVIEW_CLOSE: u32 = 1;
pub const wxID_PREVIEW_NEXT: u32 = 2;
pub const wxID_PREVIEW_PREVIOUS: u32 = 3;
pub const wxID_PREVIEW_PRINT: u32 = 4;
pub const wxID_PREVIEW_ZOOM: u32 = 5;
pub const wxID_PREVIEW_FIRST: u32 = 6;
pub const wxID_PREVIEW_LAST: u32 = 7;
pub const wxID_PREVIEW_GOTO: u32 = 8;
pub const wxID_PREVIEW_ZOOM_IN: u32 = 9;
pub const wxID_PREVIEW_ZOOM_OUT: u32 = 10;
//  ENUM: wxPrinterError
pub const wxPRINTER_NO_ERROR: u32 = 0;
pub const wxPRINTER_CANCELLED: u32 = 0 + 1;
pub const wxPRINTER_ERROR: u32 = 0 + 2;
//  ENUM: wxPreviewFrameModalityKind
pub const wxPreviewFrame_AppModal: u32 = 0;
pub const wxPreviewFrame_WindowModal: u32 = 0 + 1;
pub const wxPreviewFrame_NonModal: u32 = 0 + 2;

// NODEF: wxGetVariantCast

pub const wxWIZARD_EX_HELPBUTTON: u32 = 0x00000010;
pub const wxWIZARD_VALIGN_TOP: u32 = 0x01;
pub const wxWIZARD_VALIGN_CENTRE: u32 = 0x02;
pub const wxWIZARD_VALIGN_BOTTOM: u32 = 0x04;
pub const wxWIZARD_HALIGN_LEFT: u32 = 0x08;
pub const wxWIZARD_HALIGN_CENTRE: u32 = 0x10;
pub const wxWIZARD_HALIGN_RIGHT: u32 = 0x20;
pub const wxWIZARD_TILE: u32 = 0x40;

//  ENUM: wxTextFileType
pub const wxTextFileType_None: u32 = 0;
pub const wxTextFileType_Unix: u32 = 0 + 1;
pub const wxTextFileType_Dos: u32 = 0 + 2;
pub const wxTextFileType_Mac: u32 = 0 + 3;
pub const wxTextFileType_Os2: u32 = 0 + 4;

pub const wxFD_DEFAULT_STYLE: u32 = wxFD_OPEN;
//  ENUM: @19
pub const wxFD_OPEN: u32 = 0x0001;
pub const wxFD_SAVE: u32 = 0x0002;
pub const wxFD_OVERWRITE_PROMPT: u32 = 0x0004;
pub const wxFD_NO_FOLLOW: u32 = 0x0008;
pub const wxFD_FILE_MUST_EXIST: u32 = 0x0010;
pub const wxFD_CHANGE_DIR: u32 = 0x0080;
pub const wxFD_PREVIEW: u32 = 0x0100;
pub const wxFD_MULTIPLE: u32 = 0x0200;
pub const wxFD_SHOW_HIDDEN: u32 = 0x0400;

//  ENUM: Direction
pub const Get: u32 = 0x01;
pub const Set: u32 = 0x02;
pub const Both: u32 = 0x03;

pub const wxHF_TOOLBAR: u32 = 0x0001;
pub const wxHF_CONTENTS: u32 = 0x0002;
pub const wxHF_INDEX: u32 = 0x0004;
pub const wxHF_SEARCH: u32 = 0x0008;
pub const wxHF_BOOKMARKS: u32 = 0x0010;
pub const wxHF_OPEN_FILES: u32 = 0x0020;
pub const wxHF_PRINT: u32 = 0x0040;
pub const wxHF_FLAT_TOOLBAR: u32 = 0x0080;
pub const wxHF_MERGE_BOOKS: u32 = 0x0100;
pub const wxHF_ICONS_BOOK: u32 = 0x0200;
pub const wxHF_ICONS_BOOK_CHAPTER: u32 = 0x0400;
pub const wxHF_ICONS_FOLDER: u32 = 0x0000;
pub const wxHF_DEFAULT_STYLE: u32 = (wxHF_TOOLBAR | wxHF_CONTENTS | wxHF_INDEX | wxHF_SEARCH | wxHF_BOOKMARKS | wxHF_PRINT);

//  ENUM: Context
pub const Context_Current: u32 = 0;
pub const Context_Exception: u32 = 0 + 1;

pub const wxST_NO_AUTORESIZE: u32 = 0x0001;
pub const wxST_ELLIPSIZE_START: u32 = 0x0004;
pub const wxST_ELLIPSIZE_MIDDLE: u32 = 0x0008;
pub const wxST_ELLIPSIZE_END: u32 = 0x0010;

//  ENUM: wxSocketError
pub const wxSOCKET_NOERROR: u32 = 0;
pub const wxSOCKET_INVOP: u32 = 0 + 1;
pub const wxSOCKET_IOERR: u32 = 0 + 2;
pub const wxSOCKET_INVADDR: u32 = 0 + 3;
pub const wxSOCKET_INVSOCK: u32 = 0 + 4;
pub const wxSOCKET_NOHOST: u32 = 0 + 5;
pub const wxSOCKET_INVPORT: u32 = 0 + 6;
pub const wxSOCKET_WOULDBLOCK: u32 = 0 + 7;
pub const wxSOCKET_TIMEDOUT: u32 = 0 + 8;
pub const wxSOCKET_MEMERR: u32 = 0 + 9;
//  ENUM: wxSocketEventFlags
pub const wxSOCKET_INPUT: u32 = 0;
pub const wxSOCKET_OUTPUT: u32 = 0 + 1;
pub const wxSOCKET_CONNECTION: u32 = 0 + 2;
pub const wxSOCKET_LOST: u32 = 0 + 3;
//  ENUM: @46
pub const wxSOCKET_NONE: u32 = 0;
pub const wxSOCKET_NOWAIT: u32 = 1;
pub const wxSOCKET_WAITALL: u32 = 2;
pub const wxSOCKET_BLOCK: u32 = 4;
pub const wxSOCKET_REUSEADDR: u32 = 8;
pub const wxSOCKET_BROADCAST: u32 = 16;
pub const wxSOCKET_NOBIND: u32 = 32;
pub const wxSOCKET_NOWAIT_READ: u32 = 64;
pub const wxSOCKET_WAITALL_READ: u32 = 128;
pub const wxSOCKET_NOWAIT_WRITE: u32 = 256;
pub const wxSOCKET_WAITALL_WRITE: u32 = 512;

//  ENUM: wxGridSelectionModes
pub const wxGridSelectCells: u32 = 0;
pub const wxGridSelectRows: u32 = 0 + 1;
pub const wxGridSelectColumns: u32 = 0 + 2;
pub const wxGridSelectRowsOrColumns: u32 = 0 + 3;
pub const wxGridSelectNone: u32 = 0 + 4;
//  ENUM: CellSpan
pub const CellSpan_Inside: u32 = -1;
pub const CellSpan_None: u32 = 0;
pub const CellSpan_Main: u32 = 0 + 1;
//  ENUM: TabBehaviour
pub const Tab_Stop: u32 = 0;
pub const Tab_Wrap: u32 = 0 + 1;
pub const Tab_Leave: u32 = 0 + 2;

pub const wxSTC_INVALID_POSITION: u32 = -1;
pub const wxSTC_START: u32 = 2000;
pub const wxSTC_OPTIONAL_START: u32 = 3000;
pub const wxSTC_LEXER_START: u32 = 4000;
pub const wxSTC_WS_INVISIBLE: u32 = 0;
pub const wxSTC_WS_VISIBLEALWAYS: u32 = 1;
pub const wxSTC_WS_VISIBLEAFTERINDENT: u32 = 2;
pub const wxSTC_WS_VISIBLEONLYININDENT: u32 = 3;
pub const wxSTC_TD_LONGARROW: u32 = 0;
pub const wxSTC_TD_STRIKEOUT: u32 = 1;
pub const wxSTC_EOL_CRLF: u32 = 0;
pub const wxSTC_EOL_CR: u32 = 1;
pub const wxSTC_EOL_LF: u32 = 2;
pub const wxSTC_CP_UTF8: u32 = 65001;
pub const wxSTC_IME_WINDOWED: u32 = 0;
pub const wxSTC_IME_INLINE: u32 = 1;
pub const wxSTC_MARKER_MAX: u32 = 31;
pub const wxSTC_MARK_CIRCLE: u32 = 0;
pub const wxSTC_MARK_ROUNDRECT: u32 = 1;
pub const wxSTC_MARK_ARROW: u32 = 2;
pub const wxSTC_MARK_SMALLRECT: u32 = 3;
pub const wxSTC_MARK_SHORTARROW: u32 = 4;
pub const wxSTC_MARK_EMPTY: u32 = 5;
pub const wxSTC_MARK_ARROWDOWN: u32 = 6;
pub const wxSTC_MARK_MINUS: u32 = 7;
pub const wxSTC_MARK_PLUS: u32 = 8;
pub const wxSTC_MARK_VLINE: u32 = 9;
pub const wxSTC_MARK_LCORNER: u32 = 10;
pub const wxSTC_MARK_TCORNER: u32 = 11;
pub const wxSTC_MARK_BOXPLUS: u32 = 12;
pub const wxSTC_MARK_BOXPLUSCONNECTED: u32 = 13;
pub const wxSTC_MARK_BOXMINUS: u32 = 14;
pub const wxSTC_MARK_BOXMINUSCONNECTED: u32 = 15;
pub const wxSTC_MARK_LCORNERCURVE: u32 = 16;
pub const wxSTC_MARK_TCORNERCURVE: u32 = 17;
pub const wxSTC_MARK_CIRCLEPLUS: u32 = 18;
pub const wxSTC_MARK_CIRCLEPLUSCONNECTED: u32 = 19;
pub const wxSTC_MARK_CIRCLEMINUS: u32 = 20;
pub const wxSTC_MARK_CIRCLEMINUSCONNECTED: u32 = 21;
pub const wxSTC_MARK_BACKGROUND: u32 = 22;
pub const wxSTC_MARK_DOTDOTDOT: u32 = 23;
pub const wxSTC_MARK_ARROWS: u32 = 24;
pub const wxSTC_MARK_PIXMAP: u32 = 25;
pub const wxSTC_MARK_FULLRECT: u32 = 26;
pub const wxSTC_MARK_LEFTRECT: u32 = 27;
pub const wxSTC_MARK_AVAILABLE: u32 = 28;
pub const wxSTC_MARK_UNDERLINE: u32 = 29;
pub const wxSTC_MARK_RGBAIMAGE: u32 = 30;
pub const wxSTC_MARK_BOOKMARK: u32 = 31;
pub const wxSTC_MARK_CHARACTER: u32 = 10000;
pub const wxSTC_MARKNUM_FOLDEREND: u32 = 25;
pub const wxSTC_MARKNUM_FOLDEROPENMID: u32 = 26;
pub const wxSTC_MARKNUM_FOLDERMIDTAIL: u32 = 27;
pub const wxSTC_MARKNUM_FOLDERTAIL: u32 = 28;
pub const wxSTC_MARKNUM_FOLDERSUB: u32 = 29;
pub const wxSTC_MARKNUM_FOLDER: u32 = 30;
pub const wxSTC_MARKNUM_FOLDEROPEN: u32 = 31;
pub const wxSTC_MASK_FOLDERS: u32 = 0xFE000000;
pub const wxSTC_MAX_MARGIN: u32 = 4;
pub const wxSTC_MARGIN_SYMBOL: u32 = 0;
pub const wxSTC_MARGIN_NUMBER: u32 = 1;
pub const wxSTC_MARGIN_BACK: u32 = 2;
pub const wxSTC_MARGIN_FORE: u32 = 3;
pub const wxSTC_MARGIN_TEXT: u32 = 4;
pub const wxSTC_MARGIN_RTEXT: u32 = 5;
pub const wxSTC_MARGIN_COLOUR: u32 = 6;
pub const wxSTC_STYLE_DEFAULT: u32 = 32;
pub const wxSTC_STYLE_LINENUMBER: u32 = 33;
pub const wxSTC_STYLE_BRACELIGHT: u32 = 34;
pub const wxSTC_STYLE_BRACEBAD: u32 = 35;
pub const wxSTC_STYLE_CONTROLCHAR: u32 = 36;
pub const wxSTC_STYLE_INDENTGUIDE: u32 = 37;
pub const wxSTC_STYLE_CALLTIP: u32 = 38;
pub const wxSTC_STYLE_FOLDDISPLAYTEXT: u32 = 39;
pub const wxSTC_STYLE_LASTPREDEFINED: u32 = 39;
pub const wxSTC_STYLE_MAX: u32 = 255;
pub const wxSTC_CHARSET_ANSI: u32 = 0;
pub const wxSTC_CHARSET_DEFAULT: u32 = 1;
pub const wxSTC_CHARSET_BALTIC: u32 = 186;
pub const wxSTC_CHARSET_CHINESEBIG5: u32 = 136;
pub const wxSTC_CHARSET_EASTEUROPE: u32 = 238;
pub const wxSTC_CHARSET_GB2312: u32 = 134;
pub const wxSTC_CHARSET_GREEK: u32 = 161;
pub const wxSTC_CHARSET_HANGUL: u32 = 129;
pub const wxSTC_CHARSET_MAC: u32 = 77;
pub const wxSTC_CHARSET_OEM: u32 = 255;
pub const wxSTC_CHARSET_RUSSIAN: u32 = 204;
pub const wxSTC_CHARSET_OEM866: u32 = 866;
pub const wxSTC_CHARSET_CYRILLIC: u32 = 1251;
pub const wxSTC_CHARSET_SHIFTJIS: u32 = 128;
pub const wxSTC_CHARSET_SYMBOL: u32 = 2;
pub const wxSTC_CHARSET_TURKISH: u32 = 162;
pub const wxSTC_CHARSET_JOHAB: u32 = 130;
pub const wxSTC_CHARSET_HEBREW: u32 = 177;
pub const wxSTC_CHARSET_ARABIC: u32 = 178;
pub const wxSTC_CHARSET_VIETNAMESE: u32 = 163;
pub const wxSTC_CHARSET_THAI: u32 = 222;
pub const wxSTC_CHARSET_8859_15: u32 = 1000;
pub const wxSTC_CASE_MIXED: u32 = 0;
pub const wxSTC_CASE_UPPER: u32 = 1;
pub const wxSTC_CASE_LOWER: u32 = 2;
pub const wxSTC_CASE_CAMEL: u32 = 3;
pub const wxSTC_FONT_SIZE_MULTIPLIER: u32 = 100;
pub const wxSTC_WEIGHT_NORMAL: u32 = 400;
pub const wxSTC_WEIGHT_SEMIBOLD: u32 = 600;
pub const wxSTC_WEIGHT_BOLD: u32 = 700;
pub const wxSTC_INDIC_PLAIN: u32 = 0;
pub const wxSTC_INDIC_SQUIGGLE: u32 = 1;
pub const wxSTC_INDIC_TT: u32 = 2;
pub const wxSTC_INDIC_DIAGONAL: u32 = 3;
pub const wxSTC_INDIC_STRIKE: u32 = 4;
pub const wxSTC_INDIC_HIDDEN: u32 = 5;
pub const wxSTC_INDIC_BOX: u32 = 6;
pub const wxSTC_INDIC_ROUNDBOX: u32 = 7;
pub const wxSTC_INDIC_STRAIGHTBOX: u32 = 8;
pub const wxSTC_INDIC_DASH: u32 = 9;
pub const wxSTC_INDIC_DOTS: u32 = 10;
pub const wxSTC_INDIC_SQUIGGLELOW: u32 = 11;
pub const wxSTC_INDIC_DOTBOX: u32 = 12;
pub const wxSTC_INDIC_SQUIGGLEPIXMAP: u32 = 13;
pub const wxSTC_INDIC_COMPOSITIONTHICK: u32 = 14;
pub const wxSTC_INDIC_COMPOSITIONTHIN: u32 = 15;
pub const wxSTC_INDIC_FULLBOX: u32 = 16;
pub const wxSTC_INDIC_TEXTFORE: u32 = 17;
pub const wxSTC_INDIC_POINT: u32 = 18;
pub const wxSTC_INDIC_POINTCHARACTER: u32 = 19;
pub const wxSTC_INDIC_IME: u32 = 32;
pub const wxSTC_INDIC_IME_MAX: u32 = 35;
pub const wxSTC_INDIC_MAX: u32 = 35;
pub const wxSTC_INDIC_CONTAINER: u32 = 8;
pub const wxSTC_INDICVALUEBIT: u32 = 0x1000000;
pub const wxSTC_INDICVALUEMASK: u32 = 0xFFFFFF;
pub const wxSTC_INDICFLAG_VALUEFORE: u32 = 1;
pub const wxSTC_IV_NONE: u32 = 0;
pub const wxSTC_IV_REAL: u32 = 1;
pub const wxSTC_IV_LOOKFORWARD: u32 = 2;
pub const wxSTC_IV_LOOKBOTH: u32 = 3;
pub const wxSTC_PRINT_NORMAL: u32 = 0;
pub const wxSTC_PRINT_INVERTLIGHT: u32 = 1;
pub const wxSTC_PRINT_BLACKONWHITE: u32 = 2;
pub const wxSTC_PRINT_COLOURONWHITE: u32 = 3;
pub const wxSTC_PRINT_COLOURONWHITEDEFAULTBG: u32 = 4;
pub const wxSTC_FIND_WHOLEWORD: u32 = 0x2;
pub const wxSTC_FIND_MATCHCASE: u32 = 0x4;
pub const wxSTC_FIND_WORDSTART: u32 = 0x00100000;
pub const wxSTC_FIND_REGEXP: u32 = 0x00200000;
pub const wxSTC_FIND_POSIX: u32 = 0x00400000;
pub const wxSTC_FOLDLEVELBASE: u32 = 0x400;
pub const wxSTC_FOLDLEVELWHITEFLAG: u32 = 0x1000;
pub const wxSTC_FOLDLEVELHEADERFLAG: u32 = 0x2000;
pub const wxSTC_FOLDLEVELNUMBERMASK: u32 = 0x0FFF;
pub const wxSTC_FOLDDISPLAYTEXT_HIDDEN: u32 = 0;
pub const wxSTC_FOLDDISPLAYTEXT_STANDARD: u32 = 1;
pub const wxSTC_FOLDDISPLAYTEXT_BOXED: u32 = 2;
pub const wxSTC_FOLDACTION_CONTRACT: u32 = 0;
pub const wxSTC_FOLDACTION_EXPAND: u32 = 1;
pub const wxSTC_FOLDACTION_TOGGLE: u32 = 2;
pub const wxSTC_AUTOMATICFOLD_SHOW: u32 = 0x0001;
pub const wxSTC_AUTOMATICFOLD_CLICK: u32 = 0x0002;
pub const wxSTC_AUTOMATICFOLD_CHANGE: u32 = 0x0004;
pub const wxSTC_FOLDFLAG_LINEBEFORE_EXPANDED: u32 = 0x0002;
pub const wxSTC_FOLDFLAG_LINEBEFORE_CONTRACTED: u32 = 0x0004;
pub const wxSTC_FOLDFLAG_LINEAFTER_EXPANDED: u32 = 0x0008;
pub const wxSTC_FOLDFLAG_LINEAFTER_CONTRACTED: u32 = 0x0010;
pub const wxSTC_FOLDFLAG_LEVELNUMBERS: u32 = 0x0040;
pub const wxSTC_FOLDFLAG_LINESTATE: u32 = 0x0080;
pub const wxSTC_TIME_FOREVER: u32 = 10000000;
pub const wxSTC_IDLESTYLING_NONE: u32 = 0;
pub const wxSTC_IDLESTYLING_TOVISIBLE: u32 = 1;
pub const wxSTC_IDLESTYLING_AFTERVISIBLE: u32 = 2;
pub const wxSTC_IDLESTYLING_ALL: u32 = 3;
pub const wxSTC_WRAP_NONE: u32 = 0;
pub const wxSTC_WRAP_WORD: u32 = 1;
pub const wxSTC_WRAP_CHAR: u32 = 2;
pub const wxSTC_WRAP_WHITESPACE: u32 = 3;
pub const wxSTC_WRAPVISUALFLAG_NONE: u32 = 0x0000;
pub const wxSTC_WRAPVISUALFLAG_END: u32 = 0x0001;
pub const wxSTC_WRAPVISUALFLAG_START: u32 = 0x0002;
pub const wxSTC_WRAPVISUALFLAG_MARGIN: u32 = 0x0004;
pub const wxSTC_WRAPVISUALFLAGLOC_DEFAULT: u32 = 0x0000;
pub const wxSTC_WRAPVISUALFLAGLOC_END_BY_TEXT: u32 = 0x0001;
pub const wxSTC_WRAPVISUALFLAGLOC_START_BY_TEXT: u32 = 0x0002;
pub const wxSTC_WRAPINDENT_FIXED: u32 = 0;
pub const wxSTC_WRAPINDENT_SAME: u32 = 1;
pub const wxSTC_WRAPINDENT_INDENT: u32 = 2;
pub const wxSTC_CACHE_NONE: u32 = 0;
pub const wxSTC_CACHE_CARET: u32 = 1;
pub const wxSTC_CACHE_PAGE: u32 = 2;
pub const wxSTC_CACHE_DOCUMENT: u32 = 3;
pub const wxSTC_PHASES_ONE: u32 = 0;
pub const wxSTC_PHASES_TWO: u32 = 1;
pub const wxSTC_PHASES_MULTIPLE: u32 = 2;
pub const wxSTC_EFF_QUALITY_MASK: u32 = 0xF;
pub const wxSTC_EFF_QUALITY_DEFAULT: u32 = 0;
pub const wxSTC_EFF_QUALITY_NON_ANTIALIASED: u32 = 1;
pub const wxSTC_EFF_QUALITY_ANTIALIASED: u32 = 2;
pub const wxSTC_EFF_QUALITY_LCD_OPTIMIZED: u32 = 3;
pub const wxSTC_MULTIPASTE_ONCE: u32 = 0;
pub const wxSTC_MULTIPASTE_EACH: u32 = 1;
pub const wxSTC_EDGE_NONE: u32 = 0;
pub const wxSTC_EDGE_LINE: u32 = 1;
pub const wxSTC_EDGE_BACKGROUND: u32 = 2;
pub const wxSTC_EDGE_MULTILINE: u32 = 3;
pub const wxSTC_POPUP_NEVER: u32 = 0;
pub const wxSTC_POPUP_ALL: u32 = 1;
pub const wxSTC_POPUP_TEXT: u32 = 2;
pub const wxSTC_STATUS_OK: u32 = 0;
pub const wxSTC_STATUS_FAILURE: u32 = 1;
pub const wxSTC_STATUS_BADALLOC: u32 = 2;
pub const wxSTC_STATUS_WARN_START: u32 = 1000;
pub const wxSTC_STATUS_WARN_REGEX: u32 = 1001;
pub const wxSTC_CURSORNORMAL: u32 = -1;
pub const wxSTC_CURSORARROW: u32 = 2;
pub const wxSTC_CURSORWAIT: u32 = 4;
pub const wxSTC_CURSORREVERSEARROW: u32 = 7;
pub const wxSTC_VISIBLE_SLOP: u32 = 0x01;
pub const wxSTC_VISIBLE_STRICT: u32 = 0x04;
pub const wxSTC_CARET_SLOP: u32 = 0x01;
pub const wxSTC_CARET_STRICT: u32 = 0x04;
pub const wxSTC_CARET_JUMPS: u32 = 0x10;
pub const wxSTC_CARET_EVEN: u32 = 0x08;
pub const wxSTC_SEL_STREAM: u32 = 0;
pub const wxSTC_SEL_RECTANGLE: u32 = 1;
pub const wxSTC_SEL_LINES: u32 = 2;
pub const wxSTC_SEL_THIN: u32 = 3;
pub const wxSTC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE: u32 = 0;
pub const wxSTC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE: u32 = 1;
pub const wxSTC_MULTIAUTOC_ONCE: u32 = 0;
pub const wxSTC_MULTIAUTOC_EACH: u32 = 1;
pub const wxSTC_ORDER_PRESORTED: u32 = 0;
pub const wxSTC_ORDER_PERFORMSORT: u32 = 1;
pub const wxSTC_ORDER_CUSTOM: u32 = 2;
pub const wxSTC_CARETSTICKY_OFF: u32 = 0;
pub const wxSTC_CARETSTICKY_ON: u32 = 1;
pub const wxSTC_CARETSTICKY_WHITESPACE: u32 = 2;
pub const wxSTC_ALPHA_TRANSPARENT: u32 = 0;
pub const wxSTC_ALPHA_OPAQUE: u32 = 255;
pub const wxSTC_ALPHA_NOALPHA: u32 = 256;
pub const wxSTC_CARETSTYLE_INVISIBLE: u32 = 0;
pub const wxSTC_CARETSTYLE_LINE: u32 = 1;
pub const wxSTC_CARETSTYLE_BLOCK: u32 = 2;
pub const wxSTC_MARGINOPTION_NONE: u32 = 0;
pub const wxSTC_MARGINOPTION_SUBLINESELECT: u32 = 1;
pub const wxSTC_ANNOTATION_HIDDEN: u32 = 0;
pub const wxSTC_ANNOTATION_STANDARD: u32 = 1;
pub const wxSTC_ANNOTATION_BOXED: u32 = 2;
pub const wxSTC_ANNOTATION_INDENTED: u32 = 3;
pub const wxSTC_UNDO_MAY_COALESCE: u32 = 1;
pub const wxSTC_VS_NONE: u32 = 0;
pub const wxSTC_VS_RECTANGULARSELECTION: u32 = 1;
pub const wxSTC_VS_USERACCESSIBLE: u32 = 2;
pub const wxSTC_VS_NOWRAPLINESTART: u32 = 4;
pub const wxSTC_TECHNOLOGY_DEFAULT: u32 = 0;
pub const wxSTC_TECHNOLOGY_DIRECTWRITE: u32 = 1;
pub const wxSTC_LINE_END_TYPE_DEFAULT: u32 = 0;
pub const wxSTC_LINE_END_TYPE_UNICODE: u32 = 1;
pub const wxSTC_KEYWORDSET_MAX: u32 = 8;
pub const wxSTC_TYPE_BOOLEAN: u32 = 0;
pub const wxSTC_TYPE_INTEGER: u32 = 1;
pub const wxSTC_TYPE_STRING: u32 = 2;
pub const wxSTC_MOD_INSERTTEXT: u32 = 0x1;
pub const wxSTC_MOD_DELETETEXT: u32 = 0x2;
pub const wxSTC_MOD_CHANGESTYLE: u32 = 0x4;
pub const wxSTC_MOD_CHANGEFOLD: u32 = 0x8;
pub const wxSTC_PERFORMED_USER: u32 = 0x10;
pub const wxSTC_PERFORMED_UNDO: u32 = 0x20;
pub const wxSTC_PERFORMED_REDO: u32 = 0x40;
pub const wxSTC_MULTISTEPUNDOREDO: u32 = 0x80;
pub const wxSTC_LASTSTEPINUNDOREDO: u32 = 0x100;
pub const wxSTC_MOD_CHANGEMARKER: u32 = 0x200;
pub const wxSTC_MOD_BEFOREINSERT: u32 = 0x400;
pub const wxSTC_MOD_BEFOREDELETE: u32 = 0x800;
pub const wxSTC_MULTILINEUNDOREDO: u32 = 0x1000;
pub const wxSTC_STARTACTION: u32 = 0x2000;
pub const wxSTC_MOD_CHANGEINDICATOR: u32 = 0x4000;
pub const wxSTC_MOD_CHANGELINESTATE: u32 = 0x8000;
pub const wxSTC_MOD_CHANGEMARGIN: u32 = 0x10000;
pub const wxSTC_MOD_CHANGEANNOTATION: u32 = 0x20000;
pub const wxSTC_MOD_CONTAINER: u32 = 0x40000;
pub const wxSTC_MOD_LEXERSTATE: u32 = 0x80000;
pub const wxSTC_MOD_INSERTCHECK: u32 = 0x100000;
pub const wxSTC_MOD_CHANGETABSTOPS: u32 = 0x200000;
pub const wxSTC_MODEVENTMASKALL: u32 = 0x3FFFFF;
pub const wxSTC_UPDATE_CONTENT: u32 = 0x1;
pub const wxSTC_UPDATE_SELECTION: u32 = 0x2;
pub const wxSTC_UPDATE_V_SCROLL: u32 = 0x4;
pub const wxSTC_UPDATE_H_SCROLL: u32 = 0x8;
pub const wxSTC_KEY_DOWN: u32 = 300;
pub const wxSTC_KEY_UP: u32 = 301;
pub const wxSTC_KEY_LEFT: u32 = 302;
pub const wxSTC_KEY_RIGHT: u32 = 303;
pub const wxSTC_KEY_HOME: u32 = 304;
pub const wxSTC_KEY_END: u32 = 305;
pub const wxSTC_KEY_PRIOR: u32 = 306;
pub const wxSTC_KEY_NEXT: u32 = 307;
pub const wxSTC_KEY_DELETE: u32 = 308;
pub const wxSTC_KEY_INSERT: u32 = 309;
pub const wxSTC_KEY_ESCAPE: u32 = 7;
pub const wxSTC_KEY_BACK: u32 = 8;
pub const wxSTC_KEY_TAB: u32 = 9;
pub const wxSTC_KEY_RETURN: u32 = 13;
pub const wxSTC_KEY_ADD: u32 = 310;
pub const wxSTC_KEY_SUBTRACT: u32 = 311;
pub const wxSTC_KEY_DIVIDE: u32 = 312;
pub const wxSTC_KEY_WIN: u32 = 313;
pub const wxSTC_KEY_RWIN: u32 = 314;
pub const wxSTC_KEY_MENU: u32 = 315;
pub const wxSTC_KEYMOD_NORM: u32 = 0;
pub const wxSTC_KEYMOD_SHIFT: u32 = 1;
pub const wxSTC_KEYMOD_CTRL: u32 = 2;
pub const wxSTC_KEYMOD_ALT: u32 = 4;
pub const wxSTC_KEYMOD_SUPER: u32 = 8;
pub const wxSTC_KEYMOD_META: u32 = 16;
pub const wxSTC_AC_FILLUP: u32 = 1;
pub const wxSTC_AC_DOUBLECLICK: u32 = 2;
pub const wxSTC_AC_TAB: u32 = 3;
pub const wxSTC_AC_NEWLINE: u32 = 4;
pub const wxSTC_AC_COMMAND: u32 = 5;
pub const wxSTC_LEX_CONTAINER: u32 = 0;
pub const wxSTC_LEX_NULL: u32 = 1;
pub const wxSTC_LEX_PYTHON: u32 = 2;
pub const wxSTC_LEX_CPP: u32 = 3;
pub const wxSTC_LEX_HTML: u32 = 4;
pub const wxSTC_LEX_XML: u32 = 5;
pub const wxSTC_LEX_PERL: u32 = 6;
pub const wxSTC_LEX_SQL: u32 = 7;
pub const wxSTC_LEX_VB: u32 = 8;
pub const wxSTC_LEX_PROPERTIES: u32 = 9;
pub const wxSTC_LEX_ERRORLIST: u32 = 10;
pub const wxSTC_LEX_MAKEFILE: u32 = 11;
pub const wxSTC_LEX_BATCH: u32 = 12;
pub const wxSTC_LEX_XCODE: u32 = 13;
pub const wxSTC_LEX_LATEX: u32 = 14;
pub const wxSTC_LEX_LUA: u32 = 15;
pub const wxSTC_LEX_DIFF: u32 = 16;
pub const wxSTC_LEX_CONF: u32 = 17;
pub const wxSTC_LEX_PASCAL: u32 = 18;
pub const wxSTC_LEX_AVE: u32 = 19;
pub const wxSTC_LEX_ADA: u32 = 20;
pub const wxSTC_LEX_LISP: u32 = 21;
pub const wxSTC_LEX_RUBY: u32 = 22;
pub const wxSTC_LEX_EIFFEL: u32 = 23;
pub const wxSTC_LEX_EIFFELKW: u32 = 24;
pub const wxSTC_LEX_TCL: u32 = 25;
pub const wxSTC_LEX_NNCRONTAB: u32 = 26;
pub const wxSTC_LEX_BULLANT: u32 = 27;
pub const wxSTC_LEX_VBSCRIPT: u32 = 28;
pub const wxSTC_LEX_BAAN: u32 = 31;
pub const wxSTC_LEX_MATLAB: u32 = 32;
pub const wxSTC_LEX_SCRIPTOL: u32 = 33;
pub const wxSTC_LEX_ASM: u32 = 34;
pub const wxSTC_LEX_CPPNOCASE: u32 = 35;
pub const wxSTC_LEX_FORTRAN: u32 = 36;
pub const wxSTC_LEX_F77: u32 = 37;
pub const wxSTC_LEX_CSS: u32 = 38;
pub const wxSTC_LEX_POV: u32 = 39;
pub const wxSTC_LEX_LOUT: u32 = 40;
pub const wxSTC_LEX_ESCRIPT: u32 = 41;
pub const wxSTC_LEX_PS: u32 = 42;
pub const wxSTC_LEX_NSIS: u32 = 43;
pub const wxSTC_LEX_MMIXAL: u32 = 44;
pub const wxSTC_LEX_CLW: u32 = 45;
pub const wxSTC_LEX_CLWNOCASE: u32 = 46;
pub const wxSTC_LEX_LOT: u32 = 47;
pub const wxSTC_LEX_YAML: u32 = 48;
pub const wxSTC_LEX_TEX: u32 = 49;
pub const wxSTC_LEX_METAPOST: u32 = 50;
pub const wxSTC_LEX_POWERBASIC: u32 = 51;
pub const wxSTC_LEX_FORTH: u32 = 52;
pub const wxSTC_LEX_ERLANG: u32 = 53;
pub const wxSTC_LEX_OCTAVE: u32 = 54;
pub const wxSTC_LEX_MSSQL: u32 = 55;
pub const wxSTC_LEX_VERILOG: u32 = 56;
pub const wxSTC_LEX_KIX: u32 = 57;
pub const wxSTC_LEX_GUI4CLI: u32 = 58;
pub const wxSTC_LEX_SPECMAN: u32 = 59;
pub const wxSTC_LEX_AU3: u32 = 60;
pub const wxSTC_LEX_APDL: u32 = 61;
pub const wxSTC_LEX_BASH: u32 = 62;
pub const wxSTC_LEX_ASN1: u32 = 63;
pub const wxSTC_LEX_VHDL: u32 = 64;
pub const wxSTC_LEX_CAML: u32 = 65;
pub const wxSTC_LEX_BLITZBASIC: u32 = 66;
pub const wxSTC_LEX_PUREBASIC: u32 = 67;
pub const wxSTC_LEX_HASKELL: u32 = 68;
pub const wxSTC_LEX_PHPSCRIPT: u32 = 69;
pub const wxSTC_LEX_TADS3: u32 = 70;
pub const wxSTC_LEX_REBOL: u32 = 71;
pub const wxSTC_LEX_SMALLTALK: u32 = 72;
pub const wxSTC_LEX_FLAGSHIP: u32 = 73;
pub const wxSTC_LEX_CSOUND: u32 = 74;
pub const wxSTC_LEX_FREEBASIC: u32 = 75;
pub const wxSTC_LEX_INNOSETUP: u32 = 76;
pub const wxSTC_LEX_OPAL: u32 = 77;
pub const wxSTC_LEX_SPICE: u32 = 78;
pub const wxSTC_LEX_D: u32 = 79;
pub const wxSTC_LEX_CMAKE: u32 = 80;
pub const wxSTC_LEX_GAP: u32 = 81;
pub const wxSTC_LEX_PLM: u32 = 82;
pub const wxSTC_LEX_PROGRESS: u32 = 83;
pub const wxSTC_LEX_ABAQUS: u32 = 84;
pub const wxSTC_LEX_ASYMPTOTE: u32 = 85;
pub const wxSTC_LEX_R: u32 = 86;
pub const wxSTC_LEX_MAGIK: u32 = 87;
pub const wxSTC_LEX_POWERSHELL: u32 = 88;
pub const wxSTC_LEX_MYSQL: u32 = 89;
pub const wxSTC_LEX_PO: u32 = 90;
pub const wxSTC_LEX_TAL: u32 = 91;
pub const wxSTC_LEX_COBOL: u32 = 92;
pub const wxSTC_LEX_TACL: u32 = 93;
pub const wxSTC_LEX_SORCUS: u32 = 94;
pub const wxSTC_LEX_POWERPRO: u32 = 95;
pub const wxSTC_LEX_NIMROD: u32 = 96;
pub const wxSTC_LEX_SML: u32 = 97;
pub const wxSTC_LEX_MARKDOWN: u32 = 98;
pub const wxSTC_LEX_TXT2TAGS: u32 = 99;
pub const wxSTC_LEX_A68K: u32 = 100;
pub const wxSTC_LEX_MODULA: u32 = 101;
pub const wxSTC_LEX_COFFEESCRIPT: u32 = 102;
pub const wxSTC_LEX_TCMD: u32 = 103;
pub const wxSTC_LEX_AVS: u32 = 104;
pub const wxSTC_LEX_ECL: u32 = 105;
pub const wxSTC_LEX_OSCRIPT: u32 = 106;
pub const wxSTC_LEX_VISUALPROLOG: u32 = 107;
pub const wxSTC_LEX_LITERATEHASKELL: u32 = 108;
pub const wxSTC_LEX_STTXT: u32 = 109;
pub const wxSTC_LEX_KVIRC: u32 = 110;
pub const wxSTC_LEX_RUST: u32 = 111;
pub const wxSTC_LEX_DMAP: u32 = 112;
pub const wxSTC_LEX_AS: u32 = 113;
pub const wxSTC_LEX_DMIS: u32 = 114;
pub const wxSTC_LEX_REGISTRY: u32 = 115;
pub const wxSTC_LEX_BIBTEX: u32 = 116;
pub const wxSTC_LEX_SREC: u32 = 117;
pub const wxSTC_LEX_IHEX: u32 = 118;
pub const wxSTC_LEX_TEHEX: u32 = 119;
pub const wxSTC_LEX_JSON: u32 = 120;
pub const wxSTC_LEX_EDIFACT: u32 = 121;
pub const wxSTC_LEX_AUTOMATIC: u32 = 1000;
pub const wxSTC_P_DEFAULT: u32 = 0;
pub const wxSTC_P_COMMENTLINE: u32 = 1;
pub const wxSTC_P_NUMBER: u32 = 2;
pub const wxSTC_P_STRING: u32 = 3;
pub const wxSTC_P_CHARACTER: u32 = 4;
pub const wxSTC_P_WORD: u32 = 5;
pub const wxSTC_P_TRIPLE: u32 = 6;
pub const wxSTC_P_TRIPLEDOUBLE: u32 = 7;
pub const wxSTC_P_CLASSNAME: u32 = 8;
pub const wxSTC_P_DEFNAME: u32 = 9;
pub const wxSTC_P_OPERATOR: u32 = 10;
pub const wxSTC_P_IDENTIFIER: u32 = 11;
pub const wxSTC_P_COMMENTBLOCK: u32 = 12;
pub const wxSTC_P_STRINGEOL: u32 = 13;
pub const wxSTC_P_WORD2: u32 = 14;
pub const wxSTC_P_DECORATOR: u32 = 15;
pub const wxSTC_C_DEFAULT: u32 = 0;
pub const wxSTC_C_COMMENT: u32 = 1;
pub const wxSTC_C_COMMENTLINE: u32 = 2;
pub const wxSTC_C_COMMENTDOC: u32 = 3;
pub const wxSTC_C_NUMBER: u32 = 4;
pub const wxSTC_C_WORD: u32 = 5;
pub const wxSTC_C_STRING: u32 = 6;
pub const wxSTC_C_CHARACTER: u32 = 7;
pub const wxSTC_C_UUID: u32 = 8;
pub const wxSTC_C_PREPROCESSOR: u32 = 9;
pub const wxSTC_C_OPERATOR: u32 = 10;
pub const wxSTC_C_IDENTIFIER: u32 = 11;
pub const wxSTC_C_STRINGEOL: u32 = 12;
pub const wxSTC_C_VERBATIM: u32 = 13;
pub const wxSTC_C_REGEX: u32 = 14;
pub const wxSTC_C_COMMENTLINEDOC: u32 = 15;
pub const wxSTC_C_WORD2: u32 = 16;
pub const wxSTC_C_COMMENTDOCKEYWORD: u32 = 17;
pub const wxSTC_C_COMMENTDOCKEYWORDERROR: u32 = 18;
pub const wxSTC_C_GLOBALCLASS: u32 = 19;
pub const wxSTC_C_STRINGRAW: u32 = 20;
pub const wxSTC_C_TRIPLEVERBATIM: u32 = 21;
pub const wxSTC_C_HASHQUOTEDSTRING: u32 = 22;
pub const wxSTC_C_PREPROCESSORCOMMENT: u32 = 23;
pub const wxSTC_C_PREPROCESSORCOMMENTDOC: u32 = 24;
pub const wxSTC_C_USERLITERAL: u32 = 25;
pub const wxSTC_C_TASKMARKER: u32 = 26;
pub const wxSTC_C_ESCAPESEQUENCE: u32 = 27;
pub const wxSTC_D_DEFAULT: u32 = 0;
pub const wxSTC_D_COMMENT: u32 = 1;
pub const wxSTC_D_COMMENTLINE: u32 = 2;
pub const wxSTC_D_COMMENTDOC: u32 = 3;
pub const wxSTC_D_COMMENTNESTED: u32 = 4;
pub const wxSTC_D_NUMBER: u32 = 5;
pub const wxSTC_D_WORD: u32 = 6;
pub const wxSTC_D_WORD2: u32 = 7;
pub const wxSTC_D_WORD3: u32 = 8;
pub const wxSTC_D_TYPEDEF: u32 = 9;
pub const wxSTC_D_STRING: u32 = 10;
pub const wxSTC_D_STRINGEOL: u32 = 11;
pub const wxSTC_D_CHARACTER: u32 = 12;
pub const wxSTC_D_OPERATOR: u32 = 13;
pub const wxSTC_D_IDENTIFIER: u32 = 14;
pub const wxSTC_D_COMMENTLINEDOC: u32 = 15;
pub const wxSTC_D_COMMENTDOCKEYWORD: u32 = 16;
pub const wxSTC_D_COMMENTDOCKEYWORDERROR: u32 = 17;
pub const wxSTC_D_STRINGB: u32 = 18;
pub const wxSTC_D_STRINGR: u32 = 19;
pub const wxSTC_D_WORD5: u32 = 20;
pub const wxSTC_D_WORD6: u32 = 21;
pub const wxSTC_D_WORD7: u32 = 22;
pub const wxSTC_TCL_DEFAULT: u32 = 0;
pub const wxSTC_TCL_COMMENT: u32 = 1;
pub const wxSTC_TCL_COMMENTLINE: u32 = 2;
pub const wxSTC_TCL_NUMBER: u32 = 3;
pub const wxSTC_TCL_WORD_IN_QUOTE: u32 = 4;
pub const wxSTC_TCL_IN_QUOTE: u32 = 5;
pub const wxSTC_TCL_OPERATOR: u32 = 6;
pub const wxSTC_TCL_IDENTIFIER: u32 = 7;
pub const wxSTC_TCL_SUBSTITUTION: u32 = 8;
pub const wxSTC_TCL_SUB_BRACE: u32 = 9;
pub const wxSTC_TCL_MODIFIER: u32 = 10;
pub const wxSTC_TCL_EXPAND: u32 = 11;
pub const wxSTC_TCL_WORD: u32 = 12;
pub const wxSTC_TCL_WORD2: u32 = 13;
pub const wxSTC_TCL_WORD3: u32 = 14;
pub const wxSTC_TCL_WORD4: u32 = 15;
pub const wxSTC_TCL_WORD5: u32 = 16;
pub const wxSTC_TCL_WORD6: u32 = 17;
pub const wxSTC_TCL_WORD7: u32 = 18;
pub const wxSTC_TCL_WORD8: u32 = 19;
pub const wxSTC_TCL_COMMENT_BOX: u32 = 20;
pub const wxSTC_TCL_BLOCK_COMMENT: u32 = 21;
pub const wxSTC_H_DEFAULT: u32 = 0;
pub const wxSTC_H_TAG: u32 = 1;
pub const wxSTC_H_TAGUNKNOWN: u32 = 2;
pub const wxSTC_H_ATTRIBUTE: u32 = 3;
pub const wxSTC_H_ATTRIBUTEUNKNOWN: u32 = 4;
pub const wxSTC_H_NUMBER: u32 = 5;
pub const wxSTC_H_DOUBLESTRING: u32 = 6;
pub const wxSTC_H_SINGLESTRING: u32 = 7;
pub const wxSTC_H_OTHER: u32 = 8;
pub const wxSTC_H_COMMENT: u32 = 9;
pub const wxSTC_H_ENTITY: u32 = 10;
pub const wxSTC_H_TAGEND: u32 = 11;
pub const wxSTC_H_XMLSTART: u32 = 12;
pub const wxSTC_H_XMLEND: u32 = 13;
pub const wxSTC_H_SCRIPT: u32 = 14;
pub const wxSTC_H_ASP: u32 = 15;
pub const wxSTC_H_ASPAT: u32 = 16;
pub const wxSTC_H_CDATA: u32 = 17;
pub const wxSTC_H_QUESTION: u32 = 18;
pub const wxSTC_H_VALUE: u32 = 19;
pub const wxSTC_H_XCCOMMENT: u32 = 20;
pub const wxSTC_H_SGML_DEFAULT: u32 = 21;
pub const wxSTC_H_SGML_COMMAND: u32 = 22;
pub const wxSTC_H_SGML_1ST_PARAM: u32 = 23;
pub const wxSTC_H_SGML_DOUBLESTRING: u32 = 24;
pub const wxSTC_H_SGML_SIMPLESTRING: u32 = 25;
pub const wxSTC_H_SGML_ERROR: u32 = 26;
pub const wxSTC_H_SGML_SPECIAL: u32 = 27;
pub const wxSTC_H_SGML_ENTITY: u32 = 28;
pub const wxSTC_H_SGML_COMMENT: u32 = 29;
pub const wxSTC_H_SGML_1ST_PARAM_COMMENT: u32 = 30;
pub const wxSTC_H_SGML_BLOCK_DEFAULT: u32 = 31;
pub const wxSTC_HJ_START: u32 = 40;
pub const wxSTC_HJ_DEFAULT: u32 = 41;
pub const wxSTC_HJ_COMMENT: u32 = 42;
pub const wxSTC_HJ_COMMENTLINE: u32 = 43;
pub const wxSTC_HJ_COMMENTDOC: u32 = 44;
pub const wxSTC_HJ_NUMBER: u32 = 45;
pub const wxSTC_HJ_WORD: u32 = 46;
pub const wxSTC_HJ_KEYWORD: u32 = 47;
pub const wxSTC_HJ_DOUBLESTRING: u32 = 48;
pub const wxSTC_HJ_SINGLESTRING: u32 = 49;
pub const wxSTC_HJ_SYMBOLS: u32 = 50;
pub const wxSTC_HJ_STRINGEOL: u32 = 51;
pub const wxSTC_HJ_REGEX: u32 = 52;
pub const wxSTC_HJA_START: u32 = 55;
pub const wxSTC_HJA_DEFAULT: u32 = 56;
pub const wxSTC_HJA_COMMENT: u32 = 57;
pub const wxSTC_HJA_COMMENTLINE: u32 = 58;
pub const wxSTC_HJA_COMMENTDOC: u32 = 59;
pub const wxSTC_HJA_NUMBER: u32 = 60;
pub const wxSTC_HJA_WORD: u32 = 61;
pub const wxSTC_HJA_KEYWORD: u32 = 62;
pub const wxSTC_HJA_DOUBLESTRING: u32 = 63;
pub const wxSTC_HJA_SINGLESTRING: u32 = 64;
pub const wxSTC_HJA_SYMBOLS: u32 = 65;
pub const wxSTC_HJA_STRINGEOL: u32 = 66;
pub const wxSTC_HJA_REGEX: u32 = 67;
pub const wxSTC_HB_START: u32 = 70;
pub const wxSTC_HB_DEFAULT: u32 = 71;
pub const wxSTC_HB_COMMENTLINE: u32 = 72;
pub const wxSTC_HB_NUMBER: u32 = 73;
pub const wxSTC_HB_WORD: u32 = 74;
pub const wxSTC_HB_STRING: u32 = 75;
pub const wxSTC_HB_IDENTIFIER: u32 = 76;
pub const wxSTC_HB_STRINGEOL: u32 = 77;
pub const wxSTC_HBA_START: u32 = 80;
pub const wxSTC_HBA_DEFAULT: u32 = 81;
pub const wxSTC_HBA_COMMENTLINE: u32 = 82;
pub const wxSTC_HBA_NUMBER: u32 = 83;
pub const wxSTC_HBA_WORD: u32 = 84;
pub const wxSTC_HBA_STRING: u32 = 85;
pub const wxSTC_HBA_IDENTIFIER: u32 = 86;
pub const wxSTC_HBA_STRINGEOL: u32 = 87;
pub const wxSTC_HP_START: u32 = 90;
pub const wxSTC_HP_DEFAULT: u32 = 91;
pub const wxSTC_HP_COMMENTLINE: u32 = 92;
pub const wxSTC_HP_NUMBER: u32 = 93;
pub const wxSTC_HP_STRING: u32 = 94;
pub const wxSTC_HP_CHARACTER: u32 = 95;
pub const wxSTC_HP_WORD: u32 = 96;
pub const wxSTC_HP_TRIPLE: u32 = 97;
pub const wxSTC_HP_TRIPLEDOUBLE: u32 = 98;
pub const wxSTC_HP_CLASSNAME: u32 = 99;
pub const wxSTC_HP_DEFNAME: u32 = 100;
pub const wxSTC_HP_OPERATOR: u32 = 101;
pub const wxSTC_HP_IDENTIFIER: u32 = 102;
pub const wxSTC_HPHP_COMPLEX_VARIABLE: u32 = 104;
pub const wxSTC_HPA_START: u32 = 105;
pub const wxSTC_HPA_DEFAULT: u32 = 106;
pub const wxSTC_HPA_COMMENTLINE: u32 = 107;
pub const wxSTC_HPA_NUMBER: u32 = 108;
pub const wxSTC_HPA_STRING: u32 = 109;
pub const wxSTC_HPA_CHARACTER: u32 = 110;
pub const wxSTC_HPA_WORD: u32 = 111;
pub const wxSTC_HPA_TRIPLE: u32 = 112;
pub const wxSTC_HPA_TRIPLEDOUBLE: u32 = 113;
pub const wxSTC_HPA_CLASSNAME: u32 = 114;
pub const wxSTC_HPA_DEFNAME: u32 = 115;
pub const wxSTC_HPA_OPERATOR: u32 = 116;
pub const wxSTC_HPA_IDENTIFIER: u32 = 117;
pub const wxSTC_HPHP_DEFAULT: u32 = 118;
pub const wxSTC_HPHP_HSTRING: u32 = 119;
pub const wxSTC_HPHP_SIMPLESTRING: u32 = 120;
pub const wxSTC_HPHP_WORD: u32 = 121;
pub const wxSTC_HPHP_NUMBER: u32 = 122;
pub const wxSTC_HPHP_VARIABLE: u32 = 123;
pub const wxSTC_HPHP_COMMENT: u32 = 124;
pub const wxSTC_HPHP_COMMENTLINE: u32 = 125;
pub const wxSTC_HPHP_HSTRING_VARIABLE: u32 = 126;
pub const wxSTC_HPHP_OPERATOR: u32 = 127;
pub const wxSTC_PL_DEFAULT: u32 = 0;
pub const wxSTC_PL_ERROR: u32 = 1;
pub const wxSTC_PL_COMMENTLINE: u32 = 2;
pub const wxSTC_PL_POD: u32 = 3;
pub const wxSTC_PL_NUMBER: u32 = 4;
pub const wxSTC_PL_WORD: u32 = 5;
pub const wxSTC_PL_STRING: u32 = 6;
pub const wxSTC_PL_CHARACTER: u32 = 7;
pub const wxSTC_PL_PUNCTUATION: u32 = 8;
pub const wxSTC_PL_PREPROCESSOR: u32 = 9;
pub const wxSTC_PL_OPERATOR: u32 = 10;
pub const wxSTC_PL_IDENTIFIER: u32 = 11;
pub const wxSTC_PL_SCALAR: u32 = 12;
pub const wxSTC_PL_ARRAY: u32 = 13;
pub const wxSTC_PL_HASH: u32 = 14;
pub const wxSTC_PL_SYMBOLTABLE: u32 = 15;
pub const wxSTC_PL_VARIABLE_INDEXER: u32 = 16;
pub const wxSTC_PL_REGEX: u32 = 17;
pub const wxSTC_PL_REGSUBST: u32 = 18;
pub const wxSTC_PL_LONGQUOTE: u32 = 19;
pub const wxSTC_PL_BACKTICKS: u32 = 20;
pub const wxSTC_PL_DATASECTION: u32 = 21;
pub const wxSTC_PL_HERE_DELIM: u32 = 22;
pub const wxSTC_PL_HERE_Q: u32 = 23;
pub const wxSTC_PL_HERE_QQ: u32 = 24;
pub const wxSTC_PL_HERE_QX: u32 = 25;
pub const wxSTC_PL_STRING_Q: u32 = 26;
pub const wxSTC_PL_STRING_QQ: u32 = 27;
pub const wxSTC_PL_STRING_QX: u32 = 28;
pub const wxSTC_PL_STRING_QR: u32 = 29;
pub const wxSTC_PL_STRING_QW: u32 = 30;
pub const wxSTC_PL_POD_VERB: u32 = 31;
pub const wxSTC_PL_SUB_PROTOTYPE: u32 = 40;
pub const wxSTC_PL_FORMAT_IDENT: u32 = 41;
pub const wxSTC_PL_FORMAT: u32 = 42;
pub const wxSTC_PL_STRING_VAR: u32 = 43;
pub const wxSTC_PL_XLAT: u32 = 44;
pub const wxSTC_PL_REGEX_VAR: u32 = 54;
pub const wxSTC_PL_REGSUBST_VAR: u32 = 55;
pub const wxSTC_PL_BACKTICKS_VAR: u32 = 57;
pub const wxSTC_PL_HERE_QQ_VAR: u32 = 61;
pub const wxSTC_PL_HERE_QX_VAR: u32 = 62;
pub const wxSTC_PL_STRING_QQ_VAR: u32 = 64;
pub const wxSTC_PL_STRING_QX_VAR: u32 = 65;
pub const wxSTC_PL_STRING_QR_VAR: u32 = 66;
pub const wxSTC_RB_DEFAULT: u32 = 0;
pub const wxSTC_RB_ERROR: u32 = 1;
pub const wxSTC_RB_COMMENTLINE: u32 = 2;
pub const wxSTC_RB_POD: u32 = 3;
pub const wxSTC_RB_NUMBER: u32 = 4;
pub const wxSTC_RB_WORD: u32 = 5;
pub const wxSTC_RB_STRING: u32 = 6;
pub const wxSTC_RB_CHARACTER: u32 = 7;
pub const wxSTC_RB_CLASSNAME: u32 = 8;
pub const wxSTC_RB_DEFNAME: u32 = 9;
pub const wxSTC_RB_OPERATOR: u32 = 10;
pub const wxSTC_RB_IDENTIFIER: u32 = 11;
pub const wxSTC_RB_REGEX: u32 = 12;
pub const wxSTC_RB_GLOBAL: u32 = 13;
pub const wxSTC_RB_SYMBOL: u32 = 14;
pub const wxSTC_RB_MODULE_NAME: u32 = 15;
pub const wxSTC_RB_INSTANCE_VAR: u32 = 16;
pub const wxSTC_RB_CLASS_VAR: u32 = 17;
pub const wxSTC_RB_BACKTICKS: u32 = 18;
pub const wxSTC_RB_DATASECTION: u32 = 19;
pub const wxSTC_RB_HERE_DELIM: u32 = 20;
pub const wxSTC_RB_HERE_Q: u32 = 21;
pub const wxSTC_RB_HERE_QQ: u32 = 22;
pub const wxSTC_RB_HERE_QX: u32 = 23;
pub const wxSTC_RB_STRING_Q: u32 = 24;
pub const wxSTC_RB_STRING_QQ: u32 = 25;
pub const wxSTC_RB_STRING_QX: u32 = 26;
pub const wxSTC_RB_STRING_QR: u32 = 27;
pub const wxSTC_RB_STRING_QW: u32 = 28;
pub const wxSTC_RB_WORD_DEMOTED: u32 = 29;
pub const wxSTC_RB_STDIN: u32 = 30;
pub const wxSTC_RB_STDOUT: u32 = 31;
pub const wxSTC_RB_STDERR: u32 = 40;
pub const wxSTC_RB_UPPER_BOUND: u32 = 41;
pub const wxSTC_B_DEFAULT: u32 = 0;
pub const wxSTC_B_COMMENT: u32 = 1;
pub const wxSTC_B_NUMBER: u32 = 2;
pub const wxSTC_B_KEYWORD: u32 = 3;
pub const wxSTC_B_STRING: u32 = 4;
pub const wxSTC_B_PREPROCESSOR: u32 = 5;
pub const wxSTC_B_OPERATOR: u32 = 6;
pub const wxSTC_B_IDENTIFIER: u32 = 7;
pub const wxSTC_B_DATE: u32 = 8;
pub const wxSTC_B_STRINGEOL: u32 = 9;
pub const wxSTC_B_KEYWORD2: u32 = 10;
pub const wxSTC_B_KEYWORD3: u32 = 11;
pub const wxSTC_B_KEYWORD4: u32 = 12;
pub const wxSTC_B_CONSTANT: u32 = 13;
pub const wxSTC_B_ASM: u32 = 14;
pub const wxSTC_B_LABEL: u32 = 15;
pub const wxSTC_B_ERROR: u32 = 16;
pub const wxSTC_B_HEXNUMBER: u32 = 17;
pub const wxSTC_B_BINNUMBER: u32 = 18;
pub const wxSTC_B_COMMENTBLOCK: u32 = 19;
pub const wxSTC_B_DOCLINE: u32 = 20;
pub const wxSTC_B_DOCBLOCK: u32 = 21;
pub const wxSTC_B_DOCKEYWORD: u32 = 22;
pub const wxSTC_PROPS_DEFAULT: u32 = 0;
pub const wxSTC_PROPS_COMMENT: u32 = 1;
pub const wxSTC_PROPS_SECTION: u32 = 2;
pub const wxSTC_PROPS_ASSIGNMENT: u32 = 3;
pub const wxSTC_PROPS_DEFVAL: u32 = 4;
pub const wxSTC_PROPS_KEY: u32 = 5;
pub const wxSTC_L_DEFAULT: u32 = 0;
pub const wxSTC_L_COMMAND: u32 = 1;
pub const wxSTC_L_TAG: u32 = 2;
pub const wxSTC_L_MATH: u32 = 3;
pub const wxSTC_L_COMMENT: u32 = 4;
pub const wxSTC_L_TAG2: u32 = 5;
pub const wxSTC_L_MATH2: u32 = 6;
pub const wxSTC_L_COMMENT2: u32 = 7;
pub const wxSTC_L_VERBATIM: u32 = 8;
pub const wxSTC_L_SHORTCMD: u32 = 9;
pub const wxSTC_L_SPECIAL: u32 = 10;
pub const wxSTC_L_CMDOPT: u32 = 11;
pub const wxSTC_L_ERROR: u32 = 12;
pub const wxSTC_LUA_DEFAULT: u32 = 0;
pub const wxSTC_LUA_COMMENT: u32 = 1;
pub const wxSTC_LUA_COMMENTLINE: u32 = 2;
pub const wxSTC_LUA_COMMENTDOC: u32 = 3;
pub const wxSTC_LUA_NUMBER: u32 = 4;
pub const wxSTC_LUA_WORD: u32 = 5;
pub const wxSTC_LUA_STRING: u32 = 6;
pub const wxSTC_LUA_CHARACTER: u32 = 7;
pub const wxSTC_LUA_LITERALSTRING: u32 = 8;
pub const wxSTC_LUA_PREPROCESSOR: u32 = 9;
pub const wxSTC_LUA_OPERATOR: u32 = 10;
pub const wxSTC_LUA_IDENTIFIER: u32 = 11;
pub const wxSTC_LUA_STRINGEOL: u32 = 12;
pub const wxSTC_LUA_WORD2: u32 = 13;
pub const wxSTC_LUA_WORD3: u32 = 14;
pub const wxSTC_LUA_WORD4: u32 = 15;
pub const wxSTC_LUA_WORD5: u32 = 16;
pub const wxSTC_LUA_WORD6: u32 = 17;
pub const wxSTC_LUA_WORD7: u32 = 18;
pub const wxSTC_LUA_WORD8: u32 = 19;
pub const wxSTC_LUA_LABEL: u32 = 20;
pub const wxSTC_ERR_DEFAULT: u32 = 0;
pub const wxSTC_ERR_PYTHON: u32 = 1;
pub const wxSTC_ERR_GCC: u32 = 2;
pub const wxSTC_ERR_MS: u32 = 3;
pub const wxSTC_ERR_CMD: u32 = 4;
pub const wxSTC_ERR_BORLAND: u32 = 5;
pub const wxSTC_ERR_PERL: u32 = 6;
pub const wxSTC_ERR_NET: u32 = 7;
pub const wxSTC_ERR_LUA: u32 = 8;
pub const wxSTC_ERR_CTAG: u32 = 9;
pub const wxSTC_ERR_DIFF_CHANGED: u32 = 10;
pub const wxSTC_ERR_DIFF_ADDITION: u32 = 11;
pub const wxSTC_ERR_DIFF_DELETION: u32 = 12;
pub const wxSTC_ERR_DIFF_MESSAGE: u32 = 13;
pub const wxSTC_ERR_PHP: u32 = 14;
pub const wxSTC_ERR_ELF: u32 = 15;
pub const wxSTC_ERR_IFC: u32 = 16;
pub const wxSTC_ERR_IFORT: u32 = 17;
pub const wxSTC_ERR_ABSF: u32 = 18;
pub const wxSTC_ERR_TIDY: u32 = 19;
pub const wxSTC_ERR_JAVA_STACK: u32 = 20;
pub const wxSTC_ERR_VALUE: u32 = 21;
pub const wxSTC_ERR_GCC_INCLUDED_FROM: u32 = 22;
pub const wxSTC_ERR_ESCSEQ: u32 = 23;
pub const wxSTC_ERR_ESCSEQ_UNKNOWN: u32 = 24;
pub const wxSTC_ERR_ES_BLACK: u32 = 40;
pub const wxSTC_ERR_ES_RED: u32 = 41;
pub const wxSTC_ERR_ES_GREEN: u32 = 42;
pub const wxSTC_ERR_ES_BROWN: u32 = 43;
pub const wxSTC_ERR_ES_BLUE: u32 = 44;
pub const wxSTC_ERR_ES_MAGENTA: u32 = 45;
pub const wxSTC_ERR_ES_CYAN: u32 = 46;
pub const wxSTC_ERR_ES_GRAY: u32 = 47;
pub const wxSTC_ERR_ES_DARK_GRAY: u32 = 48;
pub const wxSTC_ERR_ES_BRIGHT_RED: u32 = 49;
pub const wxSTC_ERR_ES_BRIGHT_GREEN: u32 = 50;
pub const wxSTC_ERR_ES_YELLOW: u32 = 51;
pub const wxSTC_ERR_ES_BRIGHT_BLUE: u32 = 52;
pub const wxSTC_ERR_ES_BRIGHT_MAGENTA: u32 = 53;
pub const wxSTC_ERR_ES_BRIGHT_CYAN: u32 = 54;
pub const wxSTC_ERR_ES_WHITE: u32 = 55;
pub const wxSTC_BAT_DEFAULT: u32 = 0;
pub const wxSTC_BAT_COMMENT: u32 = 1;
pub const wxSTC_BAT_WORD: u32 = 2;
pub const wxSTC_BAT_LABEL: u32 = 3;
pub const wxSTC_BAT_HIDE: u32 = 4;
pub const wxSTC_BAT_COMMAND: u32 = 5;
pub const wxSTC_BAT_IDENTIFIER: u32 = 6;
pub const wxSTC_BAT_OPERATOR: u32 = 7;
pub const wxSTC_TCMD_DEFAULT: u32 = 0;
pub const wxSTC_TCMD_COMMENT: u32 = 1;
pub const wxSTC_TCMD_WORD: u32 = 2;
pub const wxSTC_TCMD_LABEL: u32 = 3;
pub const wxSTC_TCMD_HIDE: u32 = 4;
pub const wxSTC_TCMD_COMMAND: u32 = 5;
pub const wxSTC_TCMD_IDENTIFIER: u32 = 6;
pub const wxSTC_TCMD_OPERATOR: u32 = 7;
pub const wxSTC_TCMD_ENVIRONMENT: u32 = 8;
pub const wxSTC_TCMD_EXPANSION: u32 = 9;
pub const wxSTC_TCMD_CLABEL: u32 = 10;
pub const wxSTC_MAKE_DEFAULT: u32 = 0;
pub const wxSTC_MAKE_COMMENT: u32 = 1;
pub const wxSTC_MAKE_PREPROCESSOR: u32 = 2;
pub const wxSTC_MAKE_IDENTIFIER: u32 = 3;
pub const wxSTC_MAKE_OPERATOR: u32 = 4;
pub const wxSTC_MAKE_TARGET: u32 = 5;
pub const wxSTC_MAKE_IDEOL: u32 = 9;
pub const wxSTC_DIFF_DEFAULT: u32 = 0;
pub const wxSTC_DIFF_COMMENT: u32 = 1;
pub const wxSTC_DIFF_COMMAND: u32 = 2;
pub const wxSTC_DIFF_HEADER: u32 = 3;
pub const wxSTC_DIFF_POSITION: u32 = 4;
pub const wxSTC_DIFF_DELETED: u32 = 5;
pub const wxSTC_DIFF_ADDED: u32 = 6;
pub const wxSTC_DIFF_CHANGED: u32 = 7;
pub const wxSTC_CONF_DEFAULT: u32 = 0;
pub const wxSTC_CONF_COMMENT: u32 = 1;
pub const wxSTC_CONF_NUMBER: u32 = 2;
pub const wxSTC_CONF_IDENTIFIER: u32 = 3;
pub const wxSTC_CONF_EXTENSION: u32 = 4;
pub const wxSTC_CONF_PARAMETER: u32 = 5;
pub const wxSTC_CONF_STRING: u32 = 6;
pub const wxSTC_CONF_OPERATOR: u32 = 7;
pub const wxSTC_CONF_IP: u32 = 8;
pub const wxSTC_CONF_DIRECTIVE: u32 = 9;
pub const wxSTC_AVE_DEFAULT: u32 = 0;
pub const wxSTC_AVE_COMMENT: u32 = 1;
pub const wxSTC_AVE_NUMBER: u32 = 2;
pub const wxSTC_AVE_WORD: u32 = 3;
pub const wxSTC_AVE_STRING: u32 = 6;
pub const wxSTC_AVE_ENUM: u32 = 7;
pub const wxSTC_AVE_STRINGEOL: u32 = 8;
pub const wxSTC_AVE_IDENTIFIER: u32 = 9;
pub const wxSTC_AVE_OPERATOR: u32 = 10;
pub const wxSTC_AVE_WORD1: u32 = 11;
pub const wxSTC_AVE_WORD2: u32 = 12;
pub const wxSTC_AVE_WORD3: u32 = 13;
pub const wxSTC_AVE_WORD4: u32 = 14;
pub const wxSTC_AVE_WORD5: u32 = 15;
pub const wxSTC_AVE_WORD6: u32 = 16;
pub const wxSTC_ADA_DEFAULT: u32 = 0;
pub const wxSTC_ADA_WORD: u32 = 1;
pub const wxSTC_ADA_IDENTIFIER: u32 = 2;
pub const wxSTC_ADA_NUMBER: u32 = 3;
pub const wxSTC_ADA_DELIMITER: u32 = 4;
pub const wxSTC_ADA_CHARACTER: u32 = 5;
pub const wxSTC_ADA_CHARACTEREOL: u32 = 6;
pub const wxSTC_ADA_STRING: u32 = 7;
pub const wxSTC_ADA_STRINGEOL: u32 = 8;
pub const wxSTC_ADA_LABEL: u32 = 9;
pub const wxSTC_ADA_COMMENTLINE: u32 = 10;
pub const wxSTC_ADA_ILLEGAL: u32 = 11;
pub const wxSTC_BAAN_DEFAULT: u32 = 0;
pub const wxSTC_BAAN_COMMENT: u32 = 1;
pub const wxSTC_BAAN_COMMENTDOC: u32 = 2;
pub const wxSTC_BAAN_NUMBER: u32 = 3;
pub const wxSTC_BAAN_WORD: u32 = 4;
pub const wxSTC_BAAN_STRING: u32 = 5;
pub const wxSTC_BAAN_PREPROCESSOR: u32 = 6;
pub const wxSTC_BAAN_OPERATOR: u32 = 7;
pub const wxSTC_BAAN_IDENTIFIER: u32 = 8;
pub const wxSTC_BAAN_STRINGEOL: u32 = 9;
pub const wxSTC_BAAN_WORD2: u32 = 10;
pub const wxSTC_BAAN_WORD3: u32 = 11;
pub const wxSTC_BAAN_WORD4: u32 = 12;
pub const wxSTC_BAAN_WORD5: u32 = 13;
pub const wxSTC_BAAN_WORD6: u32 = 14;
pub const wxSTC_BAAN_WORD7: u32 = 15;
pub const wxSTC_BAAN_WORD8: u32 = 16;
pub const wxSTC_BAAN_WORD9: u32 = 17;
pub const wxSTC_BAAN_TABLEDEF: u32 = 18;
pub const wxSTC_BAAN_TABLESQL: u32 = 19;
pub const wxSTC_BAAN_FUNCTION: u32 = 20;
pub const wxSTC_BAAN_DOMDEF: u32 = 21;
pub const wxSTC_BAAN_FUNCDEF: u32 = 22;
pub const wxSTC_BAAN_OBJECTDEF: u32 = 23;
pub const wxSTC_BAAN_DEFINEDEF: u32 = 24;
pub const wxSTC_LISP_DEFAULT: u32 = 0;
pub const wxSTC_LISP_COMMENT: u32 = 1;
pub const wxSTC_LISP_NUMBER: u32 = 2;
pub const wxSTC_LISP_KEYWORD: u32 = 3;
pub const wxSTC_LISP_KEYWORD_KW: u32 = 4;
pub const wxSTC_LISP_SYMBOL: u32 = 5;
pub const wxSTC_LISP_STRING: u32 = 6;
pub const wxSTC_LISP_STRINGEOL: u32 = 8;
pub const wxSTC_LISP_IDENTIFIER: u32 = 9;
pub const wxSTC_LISP_OPERATOR: u32 = 10;
pub const wxSTC_LISP_SPECIAL: u32 = 11;
pub const wxSTC_LISP_MULTI_COMMENT: u32 = 12;
pub const wxSTC_EIFFEL_DEFAULT: u32 = 0;
pub const wxSTC_EIFFEL_COMMENTLINE: u32 = 1;
pub const wxSTC_EIFFEL_NUMBER: u32 = 2;
pub const wxSTC_EIFFEL_WORD: u32 = 3;
pub const wxSTC_EIFFEL_STRING: u32 = 4;
pub const wxSTC_EIFFEL_CHARACTER: u32 = 5;
pub const wxSTC_EIFFEL_OPERATOR: u32 = 6;
pub const wxSTC_EIFFEL_IDENTIFIER: u32 = 7;
pub const wxSTC_EIFFEL_STRINGEOL: u32 = 8;
pub const wxSTC_NNCRONTAB_DEFAULT: u32 = 0;
pub const wxSTC_NNCRONTAB_COMMENT: u32 = 1;
pub const wxSTC_NNCRONTAB_TASK: u32 = 2;
pub const wxSTC_NNCRONTAB_SECTION: u32 = 3;
pub const wxSTC_NNCRONTAB_KEYWORD: u32 = 4;
pub const wxSTC_NNCRONTAB_MODIFIER: u32 = 5;
pub const wxSTC_NNCRONTAB_ASTERISK: u32 = 6;
pub const wxSTC_NNCRONTAB_NUMBER: u32 = 7;
pub const wxSTC_NNCRONTAB_STRING: u32 = 8;
pub const wxSTC_NNCRONTAB_ENVIRONMENT: u32 = 9;
pub const wxSTC_NNCRONTAB_IDENTIFIER: u32 = 10;
pub const wxSTC_FORTH_DEFAULT: u32 = 0;
pub const wxSTC_FORTH_COMMENT: u32 = 1;
pub const wxSTC_FORTH_COMMENT_ML: u32 = 2;
pub const wxSTC_FORTH_IDENTIFIER: u32 = 3;
pub const wxSTC_FORTH_CONTROL: u32 = 4;
pub const wxSTC_FORTH_KEYWORD: u32 = 5;
pub const wxSTC_FORTH_DEFWORD: u32 = 6;
pub const wxSTC_FORTH_PREWORD1: u32 = 7;
pub const wxSTC_FORTH_PREWORD2: u32 = 8;
pub const wxSTC_FORTH_NUMBER: u32 = 9;
pub const wxSTC_FORTH_STRING: u32 = 10;
pub const wxSTC_FORTH_LOCALE: u32 = 11;
pub const wxSTC_MATLAB_DEFAULT: u32 = 0;
pub const wxSTC_MATLAB_COMMENT: u32 = 1;
pub const wxSTC_MATLAB_COMMAND: u32 = 2;
pub const wxSTC_MATLAB_NUMBER: u32 = 3;
pub const wxSTC_MATLAB_KEYWORD: u32 = 4;
pub const wxSTC_MATLAB_STRING: u32 = 5;
pub const wxSTC_MATLAB_OPERATOR: u32 = 6;
pub const wxSTC_MATLAB_IDENTIFIER: u32 = 7;
pub const wxSTC_MATLAB_DOUBLEQUOTESTRING: u32 = 8;
pub const wxSTC_SCRIPTOL_DEFAULT: u32 = 0;
pub const wxSTC_SCRIPTOL_WHITE: u32 = 1;
pub const wxSTC_SCRIPTOL_COMMENTLINE: u32 = 2;
pub const wxSTC_SCRIPTOL_PERSISTENT: u32 = 3;
pub const wxSTC_SCRIPTOL_CSTYLE: u32 = 4;
pub const wxSTC_SCRIPTOL_COMMENTBLOCK: u32 = 5;
pub const wxSTC_SCRIPTOL_NUMBER: u32 = 6;
pub const wxSTC_SCRIPTOL_STRING: u32 = 7;
pub const wxSTC_SCRIPTOL_CHARACTER: u32 = 8;
pub const wxSTC_SCRIPTOL_STRINGEOL: u32 = 9;
pub const wxSTC_SCRIPTOL_KEYWORD: u32 = 10;
pub const wxSTC_SCRIPTOL_OPERATOR: u32 = 11;
pub const wxSTC_SCRIPTOL_IDENTIFIER: u32 = 12;
pub const wxSTC_SCRIPTOL_TRIPLE: u32 = 13;
pub const wxSTC_SCRIPTOL_CLASSNAME: u32 = 14;
pub const wxSTC_SCRIPTOL_PREPROCESSOR: u32 = 15;
pub const wxSTC_ASM_DEFAULT: u32 = 0;
pub const wxSTC_ASM_COMMENT: u32 = 1;
pub const wxSTC_ASM_NUMBER: u32 = 2;
pub const wxSTC_ASM_STRING: u32 = 3;
pub const wxSTC_ASM_OPERATOR: u32 = 4;
pub const wxSTC_ASM_IDENTIFIER: u32 = 5;
pub const wxSTC_ASM_CPUINSTRUCTION: u32 = 6;
pub const wxSTC_ASM_MATHINSTRUCTION: u32 = 7;
pub const wxSTC_ASM_REGISTER: u32 = 8;
pub const wxSTC_ASM_DIRECTIVE: u32 = 9;
pub const wxSTC_ASM_DIRECTIVEOPERAND: u32 = 10;
pub const wxSTC_ASM_COMMENTBLOCK: u32 = 11;
pub const wxSTC_ASM_CHARACTER: u32 = 12;
pub const wxSTC_ASM_STRINGEOL: u32 = 13;
pub const wxSTC_ASM_EXTINSTRUCTION: u32 = 14;
pub const wxSTC_ASM_COMMENTDIRECTIVE: u32 = 15;
pub const wxSTC_F_DEFAULT: u32 = 0;
pub const wxSTC_F_COMMENT: u32 = 1;
pub const wxSTC_F_NUMBER: u32 = 2;
pub const wxSTC_F_STRING1: u32 = 3;
pub const wxSTC_F_STRING2: u32 = 4;
pub const wxSTC_F_STRINGEOL: u32 = 5;
pub const wxSTC_F_OPERATOR: u32 = 6;
pub const wxSTC_F_IDENTIFIER: u32 = 7;
pub const wxSTC_F_WORD: u32 = 8;
pub const wxSTC_F_WORD2: u32 = 9;
pub const wxSTC_F_WORD3: u32 = 10;
pub const wxSTC_F_PREPROCESSOR: u32 = 11;
pub const wxSTC_F_OPERATOR2: u32 = 12;
pub const wxSTC_F_LABEL: u32 = 13;
pub const wxSTC_F_CONTINUATION: u32 = 14;
pub const wxSTC_CSS_DEFAULT: u32 = 0;
pub const wxSTC_CSS_TAG: u32 = 1;
pub const wxSTC_CSS_CLASS: u32 = 2;
pub const wxSTC_CSS_PSEUDOCLASS: u32 = 3;
pub const wxSTC_CSS_UNKNOWN_PSEUDOCLASS: u32 = 4;
pub const wxSTC_CSS_OPERATOR: u32 = 5;
pub const wxSTC_CSS_IDENTIFIER: u32 = 6;
pub const wxSTC_CSS_UNKNOWN_IDENTIFIER: u32 = 7;
pub const wxSTC_CSS_VALUE: u32 = 8;
pub const wxSTC_CSS_COMMENT: u32 = 9;
pub const wxSTC_CSS_ID: u32 = 10;
pub const wxSTC_CSS_IMPORTANT: u32 = 11;
pub const wxSTC_CSS_DIRECTIVE: u32 = 12;
pub const wxSTC_CSS_DOUBLESTRING: u32 = 13;
pub const wxSTC_CSS_SINGLESTRING: u32 = 14;
pub const wxSTC_CSS_IDENTIFIER2: u32 = 15;
pub const wxSTC_CSS_ATTRIBUTE: u32 = 16;
pub const wxSTC_CSS_IDENTIFIER3: u32 = 17;
pub const wxSTC_CSS_PSEUDOELEMENT: u32 = 18;
pub const wxSTC_CSS_EXTENDED_IDENTIFIER: u32 = 19;
pub const wxSTC_CSS_EXTENDED_PSEUDOCLASS: u32 = 20;
pub const wxSTC_CSS_EXTENDED_PSEUDOELEMENT: u32 = 21;
pub const wxSTC_CSS_MEDIA: u32 = 22;
pub const wxSTC_CSS_VARIABLE: u32 = 23;
pub const wxSTC_POV_DEFAULT: u32 = 0;
pub const wxSTC_POV_COMMENT: u32 = 1;
pub const wxSTC_POV_COMMENTLINE: u32 = 2;
pub const wxSTC_POV_NUMBER: u32 = 3;
pub const wxSTC_POV_OPERATOR: u32 = 4;
pub const wxSTC_POV_IDENTIFIER: u32 = 5;
pub const wxSTC_POV_STRING: u32 = 6;
pub const wxSTC_POV_STRINGEOL: u32 = 7;
pub const wxSTC_POV_DIRECTIVE: u32 = 8;
pub const wxSTC_POV_BADDIRECTIVE: u32 = 9;
pub const wxSTC_POV_WORD2: u32 = 10;
pub const wxSTC_POV_WORD3: u32 = 11;
pub const wxSTC_POV_WORD4: u32 = 12;
pub const wxSTC_POV_WORD5: u32 = 13;
pub const wxSTC_POV_WORD6: u32 = 14;
pub const wxSTC_POV_WORD7: u32 = 15;
pub const wxSTC_POV_WORD8: u32 = 16;
pub const wxSTC_LOUT_DEFAULT: u32 = 0;
pub const wxSTC_LOUT_COMMENT: u32 = 1;
pub const wxSTC_LOUT_NUMBER: u32 = 2;
pub const wxSTC_LOUT_WORD: u32 = 3;
pub const wxSTC_LOUT_WORD2: u32 = 4;
pub const wxSTC_LOUT_WORD3: u32 = 5;
pub const wxSTC_LOUT_WORD4: u32 = 6;
pub const wxSTC_LOUT_STRING: u32 = 7;
pub const wxSTC_LOUT_OPERATOR: u32 = 8;
pub const wxSTC_LOUT_IDENTIFIER: u32 = 9;
pub const wxSTC_LOUT_STRINGEOL: u32 = 10;
pub const wxSTC_ESCRIPT_DEFAULT: u32 = 0;
pub const wxSTC_ESCRIPT_COMMENT: u32 = 1;
pub const wxSTC_ESCRIPT_COMMENTLINE: u32 = 2;
pub const wxSTC_ESCRIPT_COMMENTDOC: u32 = 3;
pub const wxSTC_ESCRIPT_NUMBER: u32 = 4;
pub const wxSTC_ESCRIPT_WORD: u32 = 5;
pub const wxSTC_ESCRIPT_STRING: u32 = 6;
pub const wxSTC_ESCRIPT_OPERATOR: u32 = 7;
pub const wxSTC_ESCRIPT_IDENTIFIER: u32 = 8;
pub const wxSTC_ESCRIPT_BRACE: u32 = 9;
pub const wxSTC_ESCRIPT_WORD2: u32 = 10;
pub const wxSTC_ESCRIPT_WORD3: u32 = 11;
pub const wxSTC_PS_DEFAULT: u32 = 0;
pub const wxSTC_PS_COMMENT: u32 = 1;
pub const wxSTC_PS_DSC_COMMENT: u32 = 2;
pub const wxSTC_PS_DSC_VALUE: u32 = 3;
pub const wxSTC_PS_NUMBER: u32 = 4;
pub const wxSTC_PS_NAME: u32 = 5;
pub const wxSTC_PS_KEYWORD: u32 = 6;
pub const wxSTC_PS_LITERAL: u32 = 7;
pub const wxSTC_PS_IMMEVAL: u32 = 8;
pub const wxSTC_PS_PAREN_ARRAY: u32 = 9;
pub const wxSTC_PS_PAREN_DICT: u32 = 10;
pub const wxSTC_PS_PAREN_PROC: u32 = 11;
pub const wxSTC_PS_TEXT: u32 = 12;
pub const wxSTC_PS_HEXSTRING: u32 = 13;
pub const wxSTC_PS_BASE85STRING: u32 = 14;
pub const wxSTC_PS_BADSTRINGCHAR: u32 = 15;
pub const wxSTC_NSIS_DEFAULT: u32 = 0;
pub const wxSTC_NSIS_COMMENT: u32 = 1;
pub const wxSTC_NSIS_STRINGDQ: u32 = 2;
pub const wxSTC_NSIS_STRINGLQ: u32 = 3;
pub const wxSTC_NSIS_STRINGRQ: u32 = 4;
pub const wxSTC_NSIS_FUNCTION: u32 = 5;
pub const wxSTC_NSIS_VARIABLE: u32 = 6;
pub const wxSTC_NSIS_LABEL: u32 = 7;
pub const wxSTC_NSIS_USERDEFINED: u32 = 8;
pub const wxSTC_NSIS_SECTIONDEF: u32 = 9;
pub const wxSTC_NSIS_SUBSECTIONDEF: u32 = 10;
pub const wxSTC_NSIS_IFDEFINEDEF: u32 = 11;
pub const wxSTC_NSIS_MACRODEF: u32 = 12;
pub const wxSTC_NSIS_STRINGVAR: u32 = 13;
pub const wxSTC_NSIS_NUMBER: u32 = 14;
pub const wxSTC_NSIS_SECTIONGROUP: u32 = 15;
pub const wxSTC_NSIS_PAGEEX: u32 = 16;
pub const wxSTC_NSIS_FUNCTIONDEF: u32 = 17;
pub const wxSTC_NSIS_COMMENTBOX: u32 = 18;
pub const wxSTC_MMIXAL_LEADWS: u32 = 0;
pub const wxSTC_MMIXAL_COMMENT: u32 = 1;
pub const wxSTC_MMIXAL_LABEL: u32 = 2;
pub const wxSTC_MMIXAL_OPCODE: u32 = 3;
pub const wxSTC_MMIXAL_OPCODE_PRE: u32 = 4;
pub const wxSTC_MMIXAL_OPCODE_VALID: u32 = 5;
pub const wxSTC_MMIXAL_OPCODE_UNKNOWN: u32 = 6;
pub const wxSTC_MMIXAL_OPCODE_POST: u32 = 7;
pub const wxSTC_MMIXAL_OPERANDS: u32 = 8;
pub const wxSTC_MMIXAL_NUMBER: u32 = 9;
pub const wxSTC_MMIXAL_REF: u32 = 10;
pub const wxSTC_MMIXAL_CHAR: u32 = 11;
pub const wxSTC_MMIXAL_STRING: u32 = 12;
pub const wxSTC_MMIXAL_REGISTER: u32 = 13;
pub const wxSTC_MMIXAL_HEX: u32 = 14;
pub const wxSTC_MMIXAL_OPERATOR: u32 = 15;
pub const wxSTC_MMIXAL_SYMBOL: u32 = 16;
pub const wxSTC_MMIXAL_INCLUDE: u32 = 17;
pub const wxSTC_CLW_DEFAULT: u32 = 0;
pub const wxSTC_CLW_LABEL: u32 = 1;
pub const wxSTC_CLW_COMMENT: u32 = 2;
pub const wxSTC_CLW_STRING: u32 = 3;
pub const wxSTC_CLW_USER_IDENTIFIER: u32 = 4;
pub const wxSTC_CLW_INTEGER_CONSTANT: u32 = 5;
pub const wxSTC_CLW_REAL_CONSTANT: u32 = 6;
pub const wxSTC_CLW_PICTURE_STRING: u32 = 7;
pub const wxSTC_CLW_KEYWORD: u32 = 8;
pub const wxSTC_CLW_COMPILER_DIRECTIVE: u32 = 9;
pub const wxSTC_CLW_RUNTIME_EXPRESSIONS: u32 = 10;
pub const wxSTC_CLW_BUILTIN_PROCEDURES_FUNCTION: u32 = 11;
pub const wxSTC_CLW_STRUCTURE_DATA_TYPE: u32 = 12;
pub const wxSTC_CLW_ATTRIBUTE: u32 = 13;
pub const wxSTC_CLW_STANDARD_EQUATE: u32 = 14;
pub const wxSTC_CLW_ERROR: u32 = 15;
pub const wxSTC_CLW_DEPRECATED: u32 = 16;
pub const wxSTC_LOT_DEFAULT: u32 = 0;
pub const wxSTC_LOT_HEADER: u32 = 1;
pub const wxSTC_LOT_BREAK: u32 = 2;
pub const wxSTC_LOT_SET: u32 = 3;
pub const wxSTC_LOT_PASS: u32 = 4;
pub const wxSTC_LOT_FAIL: u32 = 5;
pub const wxSTC_LOT_ABORT: u32 = 6;
pub const wxSTC_YAML_DEFAULT: u32 = 0;
pub const wxSTC_YAML_COMMENT: u32 = 1;
pub const wxSTC_YAML_IDENTIFIER: u32 = 2;
pub const wxSTC_YAML_KEYWORD: u32 = 3;
pub const wxSTC_YAML_NUMBER: u32 = 4;
pub const wxSTC_YAML_REFERENCE: u32 = 5;
pub const wxSTC_YAML_DOCUMENT: u32 = 6;
pub const wxSTC_YAML_TEXT: u32 = 7;
pub const wxSTC_YAML_ERROR: u32 = 8;
pub const wxSTC_YAML_OPERATOR: u32 = 9;
pub const wxSTC_TEX_DEFAULT: u32 = 0;
pub const wxSTC_TEX_SPECIAL: u32 = 1;
pub const wxSTC_TEX_GROUP: u32 = 2;
pub const wxSTC_TEX_SYMBOL: u32 = 3;
pub const wxSTC_TEX_COMMAND: u32 = 4;
pub const wxSTC_TEX_TEXT: u32 = 5;
pub const wxSTC_METAPOST_DEFAULT: u32 = 0;
pub const wxSTC_METAPOST_SPECIAL: u32 = 1;
pub const wxSTC_METAPOST_GROUP: u32 = 2;
pub const wxSTC_METAPOST_SYMBOL: u32 = 3;
pub const wxSTC_METAPOST_COMMAND: u32 = 4;
pub const wxSTC_METAPOST_TEXT: u32 = 5;
pub const wxSTC_METAPOST_EXTRA: u32 = 6;
pub const wxSTC_ERLANG_DEFAULT: u32 = 0;
pub const wxSTC_ERLANG_COMMENT: u32 = 1;
pub const wxSTC_ERLANG_VARIABLE: u32 = 2;
pub const wxSTC_ERLANG_NUMBER: u32 = 3;
pub const wxSTC_ERLANG_KEYWORD: u32 = 4;
pub const wxSTC_ERLANG_STRING: u32 = 5;
pub const wxSTC_ERLANG_OPERATOR: u32 = 6;
pub const wxSTC_ERLANG_ATOM: u32 = 7;
pub const wxSTC_ERLANG_FUNCTION_NAME: u32 = 8;
pub const wxSTC_ERLANG_CHARACTER: u32 = 9;
pub const wxSTC_ERLANG_MACRO: u32 = 10;
pub const wxSTC_ERLANG_RECORD: u32 = 11;
pub const wxSTC_ERLANG_PREPROC: u32 = 12;
pub const wxSTC_ERLANG_NODE_NAME: u32 = 13;
pub const wxSTC_ERLANG_COMMENT_FUNCTION: u32 = 14;
pub const wxSTC_ERLANG_COMMENT_MODULE: u32 = 15;
pub const wxSTC_ERLANG_COMMENT_DOC: u32 = 16;
pub const wxSTC_ERLANG_COMMENT_DOC_MACRO: u32 = 17;
pub const wxSTC_ERLANG_ATOM_QUOTED: u32 = 18;
pub const wxSTC_ERLANG_MACRO_QUOTED: u32 = 19;
pub const wxSTC_ERLANG_RECORD_QUOTED: u32 = 20;
pub const wxSTC_ERLANG_NODE_NAME_QUOTED: u32 = 21;
pub const wxSTC_ERLANG_BIFS: u32 = 22;
pub const wxSTC_ERLANG_MODULES: u32 = 23;
pub const wxSTC_ERLANG_MODULES_ATT: u32 = 24;
pub const wxSTC_ERLANG_UNKNOWN: u32 = 31;
pub const wxSTC_MSSQL_DEFAULT: u32 = 0;
pub const wxSTC_MSSQL_COMMENT: u32 = 1;
pub const wxSTC_MSSQL_LINE_COMMENT: u32 = 2;
pub const wxSTC_MSSQL_NUMBER: u32 = 3;
pub const wxSTC_MSSQL_STRING: u32 = 4;
pub const wxSTC_MSSQL_OPERATOR: u32 = 5;
pub const wxSTC_MSSQL_IDENTIFIER: u32 = 6;
pub const wxSTC_MSSQL_VARIABLE: u32 = 7;
pub const wxSTC_MSSQL_COLUMN_NAME: u32 = 8;
pub const wxSTC_MSSQL_STATEMENT: u32 = 9;
pub const wxSTC_MSSQL_DATATYPE: u32 = 10;
pub const wxSTC_MSSQL_SYSTABLE: u32 = 11;
pub const wxSTC_MSSQL_GLOBAL_VARIABLE: u32 = 12;
pub const wxSTC_MSSQL_FUNCTION: u32 = 13;
pub const wxSTC_MSSQL_STORED_PROCEDURE: u32 = 14;
pub const wxSTC_MSSQL_DEFAULT_PREF_DATATYPE: u32 = 15;
pub const wxSTC_MSSQL_COLUMN_NAME_2: u32 = 16;
pub const wxSTC_V_DEFAULT: u32 = 0;
pub const wxSTC_V_COMMENT: u32 = 1;
pub const wxSTC_V_COMMENTLINE: u32 = 2;
pub const wxSTC_V_COMMENTLINEBANG: u32 = 3;
pub const wxSTC_V_NUMBER: u32 = 4;
pub const wxSTC_V_WORD: u32 = 5;
pub const wxSTC_V_STRING: u32 = 6;
pub const wxSTC_V_WORD2: u32 = 7;
pub const wxSTC_V_WORD3: u32 = 8;
pub const wxSTC_V_PREPROCESSOR: u32 = 9;
pub const wxSTC_V_OPERATOR: u32 = 10;
pub const wxSTC_V_IDENTIFIER: u32 = 11;
pub const wxSTC_V_STRINGEOL: u32 = 12;
pub const wxSTC_V_USER: u32 = 19;
pub const wxSTC_V_COMMENT_WORD: u32 = 20;
pub const wxSTC_V_INPUT: u32 = 21;
pub const wxSTC_V_OUTPUT: u32 = 22;
pub const wxSTC_V_INOUT: u32 = 23;
pub const wxSTC_V_PORT_CONNECT: u32 = 24;
pub const wxSTC_KIX_DEFAULT: u32 = 0;
pub const wxSTC_KIX_COMMENT: u32 = 1;
pub const wxSTC_KIX_STRING1: u32 = 2;
pub const wxSTC_KIX_STRING2: u32 = 3;
pub const wxSTC_KIX_NUMBER: u32 = 4;
pub const wxSTC_KIX_VAR: u32 = 5;
pub const wxSTC_KIX_MACRO: u32 = 6;
pub const wxSTC_KIX_KEYWORD: u32 = 7;
pub const wxSTC_KIX_FUNCTIONS: u32 = 8;
pub const wxSTC_KIX_OPERATOR: u32 = 9;
pub const wxSTC_KIX_COMMENTSTREAM: u32 = 10;
pub const wxSTC_KIX_IDENTIFIER: u32 = 31;
pub const wxSTC_GC_DEFAULT: u32 = 0;
pub const wxSTC_GC_COMMENTLINE: u32 = 1;
pub const wxSTC_GC_COMMENTBLOCK: u32 = 2;
pub const wxSTC_GC_GLOBAL: u32 = 3;
pub const wxSTC_GC_EVENT: u32 = 4;
pub const wxSTC_GC_ATTRIBUTE: u32 = 5;
pub const wxSTC_GC_CONTROL: u32 = 6;
pub const wxSTC_GC_COMMAND: u32 = 7;
pub const wxSTC_GC_STRING: u32 = 8;
pub const wxSTC_GC_OPERATOR: u32 = 9;
pub const wxSTC_SN_DEFAULT: u32 = 0;
pub const wxSTC_SN_CODE: u32 = 1;
pub const wxSTC_SN_COMMENTLINE: u32 = 2;
pub const wxSTC_SN_COMMENTLINEBANG: u32 = 3;
pub const wxSTC_SN_NUMBER: u32 = 4;
pub const wxSTC_SN_WORD: u32 = 5;
pub const wxSTC_SN_STRING: u32 = 6;
pub const wxSTC_SN_WORD2: u32 = 7;
pub const wxSTC_SN_WORD3: u32 = 8;
pub const wxSTC_SN_PREPROCESSOR: u32 = 9;
pub const wxSTC_SN_OPERATOR: u32 = 10;
pub const wxSTC_SN_IDENTIFIER: u32 = 11;
pub const wxSTC_SN_STRINGEOL: u32 = 12;
pub const wxSTC_SN_REGEXTAG: u32 = 13;
pub const wxSTC_SN_SIGNAL: u32 = 14;
pub const wxSTC_SN_USER: u32 = 19;
pub const wxSTC_AU3_DEFAULT: u32 = 0;
pub const wxSTC_AU3_COMMENT: u32 = 1;
pub const wxSTC_AU3_COMMENTBLOCK: u32 = 2;
pub const wxSTC_AU3_NUMBER: u32 = 3;
pub const wxSTC_AU3_FUNCTION: u32 = 4;
pub const wxSTC_AU3_KEYWORD: u32 = 5;
pub const wxSTC_AU3_MACRO: u32 = 6;
pub const wxSTC_AU3_STRING: u32 = 7;
pub const wxSTC_AU3_OPERATOR: u32 = 8;
pub const wxSTC_AU3_VARIABLE: u32 = 9;
pub const wxSTC_AU3_SENT: u32 = 10;
pub const wxSTC_AU3_PREPROCESSOR: u32 = 11;
pub const wxSTC_AU3_SPECIAL: u32 = 12;
pub const wxSTC_AU3_EXPAND: u32 = 13;
pub const wxSTC_AU3_COMOBJ: u32 = 14;
pub const wxSTC_AU3_UDF: u32 = 15;
pub const wxSTC_APDL_DEFAULT: u32 = 0;
pub const wxSTC_APDL_COMMENT: u32 = 1;
pub const wxSTC_APDL_COMMENTBLOCK: u32 = 2;
pub const wxSTC_APDL_NUMBER: u32 = 3;
pub const wxSTC_APDL_STRING: u32 = 4;
pub const wxSTC_APDL_OPERATOR: u32 = 5;
pub const wxSTC_APDL_WORD: u32 = 6;
pub const wxSTC_APDL_PROCESSOR: u32 = 7;
pub const wxSTC_APDL_COMMAND: u32 = 8;
pub const wxSTC_APDL_SLASHCOMMAND: u32 = 9;
pub const wxSTC_APDL_STARCOMMAND: u32 = 10;
pub const wxSTC_APDL_ARGUMENT: u32 = 11;
pub const wxSTC_APDL_FUNCTION: u32 = 12;
pub const wxSTC_SH_DEFAULT: u32 = 0;
pub const wxSTC_SH_ERROR: u32 = 1;
pub const wxSTC_SH_COMMENTLINE: u32 = 2;
pub const wxSTC_SH_NUMBER: u32 = 3;
pub const wxSTC_SH_WORD: u32 = 4;
pub const wxSTC_SH_STRING: u32 = 5;
pub const wxSTC_SH_CHARACTER: u32 = 6;
pub const wxSTC_SH_OPERATOR: u32 = 7;
pub const wxSTC_SH_IDENTIFIER: u32 = 8;
pub const wxSTC_SH_SCALAR: u32 = 9;
pub const wxSTC_SH_PARAM: u32 = 10;
pub const wxSTC_SH_BACKTICKS: u32 = 11;
pub const wxSTC_SH_HERE_DELIM: u32 = 12;
pub const wxSTC_SH_HERE_Q: u32 = 13;
pub const wxSTC_ASN1_DEFAULT: u32 = 0;
pub const wxSTC_ASN1_COMMENT: u32 = 1;
pub const wxSTC_ASN1_IDENTIFIER: u32 = 2;
pub const wxSTC_ASN1_STRING: u32 = 3;
pub const wxSTC_ASN1_OID: u32 = 4;
pub const wxSTC_ASN1_SCALAR: u32 = 5;
pub const wxSTC_ASN1_KEYWORD: u32 = 6;
pub const wxSTC_ASN1_ATTRIBUTE: u32 = 7;
pub const wxSTC_ASN1_DESCRIPTOR: u32 = 8;
pub const wxSTC_ASN1_TYPE: u32 = 9;
pub const wxSTC_ASN1_OPERATOR: u32 = 10;
pub const wxSTC_VHDL_DEFAULT: u32 = 0;
pub const wxSTC_VHDL_COMMENT: u32 = 1;
pub const wxSTC_VHDL_COMMENTLINEBANG: u32 = 2;
pub const wxSTC_VHDL_NUMBER: u32 = 3;
pub const wxSTC_VHDL_STRING: u32 = 4;
pub const wxSTC_VHDL_OPERATOR: u32 = 5;
pub const wxSTC_VHDL_IDENTIFIER: u32 = 6;
pub const wxSTC_VHDL_STRINGEOL: u32 = 7;
pub const wxSTC_VHDL_KEYWORD: u32 = 8;
pub const wxSTC_VHDL_STDOPERATOR: u32 = 9;
pub const wxSTC_VHDL_ATTRIBUTE: u32 = 10;
pub const wxSTC_VHDL_STDFUNCTION: u32 = 11;
pub const wxSTC_VHDL_STDPACKAGE: u32 = 12;
pub const wxSTC_VHDL_STDTYPE: u32 = 13;
pub const wxSTC_VHDL_USERWORD: u32 = 14;
pub const wxSTC_VHDL_BLOCK_COMMENT: u32 = 15;
pub const wxSTC_CAML_DEFAULT: u32 = 0;
pub const wxSTC_CAML_IDENTIFIER: u32 = 1;
pub const wxSTC_CAML_TAGNAME: u32 = 2;
pub const wxSTC_CAML_KEYWORD: u32 = 3;
pub const wxSTC_CAML_KEYWORD2: u32 = 4;
pub const wxSTC_CAML_KEYWORD3: u32 = 5;
pub const wxSTC_CAML_LINENUM: u32 = 6;
pub const wxSTC_CAML_OPERATOR: u32 = 7;
pub const wxSTC_CAML_NUMBER: u32 = 8;
pub const wxSTC_CAML_CHAR: u32 = 9;
pub const wxSTC_CAML_WHITE: u32 = 10;
pub const wxSTC_CAML_STRING: u32 = 11;
pub const wxSTC_CAML_COMMENT: u32 = 12;
pub const wxSTC_CAML_COMMENT1: u32 = 13;
pub const wxSTC_CAML_COMMENT2: u32 = 14;
pub const wxSTC_CAML_COMMENT3: u32 = 15;
pub const wxSTC_HA_DEFAULT: u32 = 0;
pub const wxSTC_HA_IDENTIFIER: u32 = 1;
pub const wxSTC_HA_KEYWORD: u32 = 2;
pub const wxSTC_HA_NUMBER: u32 = 3;
pub const wxSTC_HA_STRING: u32 = 4;
pub const wxSTC_HA_CHARACTER: u32 = 5;
pub const wxSTC_HA_CLASS: u32 = 6;
pub const wxSTC_HA_MODULE: u32 = 7;
pub const wxSTC_HA_CAPITAL: u32 = 8;
pub const wxSTC_HA_DATA: u32 = 9;
pub const wxSTC_HA_IMPORT: u32 = 10;
pub const wxSTC_HA_OPERATOR: u32 = 11;
pub const wxSTC_HA_INSTANCE: u32 = 12;
pub const wxSTC_HA_COMMENTLINE: u32 = 13;
pub const wxSTC_HA_COMMENTBLOCK: u32 = 14;
pub const wxSTC_HA_COMMENTBLOCK2: u32 = 15;
pub const wxSTC_HA_COMMENTBLOCK3: u32 = 16;
pub const wxSTC_HA_PRAGMA: u32 = 17;
pub const wxSTC_HA_PREPROCESSOR: u32 = 18;
pub const wxSTC_HA_STRINGEOL: u32 = 19;
pub const wxSTC_HA_RESERVED_OPERATOR: u32 = 20;
pub const wxSTC_HA_LITERATE_COMMENT: u32 = 21;
pub const wxSTC_HA_LITERATE_CODEDELIM: u32 = 22;
pub const wxSTC_T3_DEFAULT: u32 = 0;
pub const wxSTC_T3_X_DEFAULT: u32 = 1;
pub const wxSTC_T3_PREPROCESSOR: u32 = 2;
pub const wxSTC_T3_BLOCK_COMMENT: u32 = 3;
pub const wxSTC_T3_LINE_COMMENT: u32 = 4;
pub const wxSTC_T3_OPERATOR: u32 = 5;
pub const wxSTC_T3_KEYWORD: u32 = 6;
pub const wxSTC_T3_NUMBER: u32 = 7;
pub const wxSTC_T3_IDENTIFIER: u32 = 8;
pub const wxSTC_T3_S_STRING: u32 = 9;
pub const wxSTC_T3_D_STRING: u32 = 10;
pub const wxSTC_T3_X_STRING: u32 = 11;
pub const wxSTC_T3_LIB_DIRECTIVE: u32 = 12;
pub const wxSTC_T3_MSG_PARAM: u32 = 13;
pub const wxSTC_T3_HTML_TAG: u32 = 14;
pub const wxSTC_T3_HTML_DEFAULT: u32 = 15;
pub const wxSTC_T3_HTML_STRING: u32 = 16;
pub const wxSTC_T3_USER1: u32 = 17;
pub const wxSTC_T3_USER2: u32 = 18;
pub const wxSTC_T3_USER3: u32 = 19;
pub const wxSTC_T3_BRACE: u32 = 20;
pub const wxSTC_REBOL_DEFAULT: u32 = 0;
pub const wxSTC_REBOL_COMMENTLINE: u32 = 1;
pub const wxSTC_REBOL_COMMENTBLOCK: u32 = 2;
pub const wxSTC_REBOL_PREFACE: u32 = 3;
pub const wxSTC_REBOL_OPERATOR: u32 = 4;
pub const wxSTC_REBOL_CHARACTER: u32 = 5;
pub const wxSTC_REBOL_QUOTEDSTRING: u32 = 6;
pub const wxSTC_REBOL_BRACEDSTRING: u32 = 7;
pub const wxSTC_REBOL_NUMBER: u32 = 8;
pub const wxSTC_REBOL_PAIR: u32 = 9;
pub const wxSTC_REBOL_TUPLE: u32 = 10;
pub const wxSTC_REBOL_BINARY: u32 = 11;
pub const wxSTC_REBOL_MONEY: u32 = 12;
pub const wxSTC_REBOL_ISSUE: u32 = 13;
pub const wxSTC_REBOL_TAG: u32 = 14;
pub const wxSTC_REBOL_FILE: u32 = 15;
pub const wxSTC_REBOL_EMAIL: u32 = 16;
pub const wxSTC_REBOL_URL: u32 = 17;
pub const wxSTC_REBOL_DATE: u32 = 18;
pub const wxSTC_REBOL_TIME: u32 = 19;
pub const wxSTC_REBOL_IDENTIFIER: u32 = 20;
pub const wxSTC_REBOL_WORD: u32 = 21;
pub const wxSTC_REBOL_WORD2: u32 = 22;
pub const wxSTC_REBOL_WORD3: u32 = 23;
pub const wxSTC_REBOL_WORD4: u32 = 24;
pub const wxSTC_REBOL_WORD5: u32 = 25;
pub const wxSTC_REBOL_WORD6: u32 = 26;
pub const wxSTC_REBOL_WORD7: u32 = 27;
pub const wxSTC_REBOL_WORD8: u32 = 28;
pub const wxSTC_SQL_DEFAULT: u32 = 0;
pub const wxSTC_SQL_COMMENT: u32 = 1;
pub const wxSTC_SQL_COMMENTLINE: u32 = 2;
pub const wxSTC_SQL_COMMENTDOC: u32 = 3;
pub const wxSTC_SQL_NUMBER: u32 = 4;
pub const wxSTC_SQL_WORD: u32 = 5;
pub const wxSTC_SQL_STRING: u32 = 6;
pub const wxSTC_SQL_CHARACTER: u32 = 7;
pub const wxSTC_SQL_SQLPLUS: u32 = 8;
pub const wxSTC_SQL_SQLPLUS_PROMPT: u32 = 9;
pub const wxSTC_SQL_OPERATOR: u32 = 10;
pub const wxSTC_SQL_IDENTIFIER: u32 = 11;
pub const wxSTC_SQL_SQLPLUS_COMMENT: u32 = 13;
pub const wxSTC_SQL_COMMENTLINEDOC: u32 = 15;
pub const wxSTC_SQL_WORD2: u32 = 16;
pub const wxSTC_SQL_COMMENTDOCKEYWORD: u32 = 17;
pub const wxSTC_SQL_COMMENTDOCKEYWORDERROR: u32 = 18;
pub const wxSTC_SQL_USER1: u32 = 19;
pub const wxSTC_SQL_USER2: u32 = 20;
pub const wxSTC_SQL_USER3: u32 = 21;
pub const wxSTC_SQL_USER4: u32 = 22;
pub const wxSTC_SQL_QUOTEDIDENTIFIER: u32 = 23;
pub const wxSTC_SQL_QOPERATOR: u32 = 24;
pub const wxSTC_ST_DEFAULT: u32 = 0;
pub const wxSTC_ST_STRING: u32 = 1;
pub const wxSTC_ST_NUMBER: u32 = 2;
pub const wxSTC_ST_COMMENT: u32 = 3;
pub const wxSTC_ST_SYMBOL: u32 = 4;
pub const wxSTC_ST_BINARY: u32 = 5;
pub const wxSTC_ST_BOOL: u32 = 6;
pub const wxSTC_ST_SELF: u32 = 7;
pub const wxSTC_ST_SUPER: u32 = 8;
pub const wxSTC_ST_NIL: u32 = 9;
pub const wxSTC_ST_GLOBAL: u32 = 10;
pub const wxSTC_ST_RETURN: u32 = 11;
pub const wxSTC_ST_SPECIAL: u32 = 12;
pub const wxSTC_ST_KWSEND: u32 = 13;
pub const wxSTC_ST_ASSIGN: u32 = 14;
pub const wxSTC_ST_CHARACTER: u32 = 15;
pub const wxSTC_ST_SPEC_SEL: u32 = 16;
pub const wxSTC_FS_DEFAULT: u32 = 0;
pub const wxSTC_FS_COMMENT: u32 = 1;
pub const wxSTC_FS_COMMENTLINE: u32 = 2;
pub const wxSTC_FS_COMMENTDOC: u32 = 3;
pub const wxSTC_FS_COMMENTLINEDOC: u32 = 4;
pub const wxSTC_FS_COMMENTDOCKEYWORD: u32 = 5;
pub const wxSTC_FS_COMMENTDOCKEYWORDERROR: u32 = 6;
pub const wxSTC_FS_KEYWORD: u32 = 7;
pub const wxSTC_FS_KEYWORD2: u32 = 8;
pub const wxSTC_FS_KEYWORD3: u32 = 9;
pub const wxSTC_FS_KEYWORD4: u32 = 10;
pub const wxSTC_FS_NUMBER: u32 = 11;
pub const wxSTC_FS_STRING: u32 = 12;
pub const wxSTC_FS_PREPROCESSOR: u32 = 13;
pub const wxSTC_FS_OPERATOR: u32 = 14;
pub const wxSTC_FS_IDENTIFIER: u32 = 15;
pub const wxSTC_FS_DATE: u32 = 16;
pub const wxSTC_FS_STRINGEOL: u32 = 17;
pub const wxSTC_FS_CONSTANT: u32 = 18;
pub const wxSTC_FS_WORDOPERATOR: u32 = 19;
pub const wxSTC_FS_DISABLEDCODE: u32 = 20;
pub const wxSTC_FS_DEFAULT_C: u32 = 21;
pub const wxSTC_FS_COMMENTDOC_C: u32 = 22;
pub const wxSTC_FS_COMMENTLINEDOC_C: u32 = 23;
pub const wxSTC_FS_KEYWORD_C: u32 = 24;
pub const wxSTC_FS_KEYWORD2_C: u32 = 25;
pub const wxSTC_FS_NUMBER_C: u32 = 26;
pub const wxSTC_FS_STRING_C: u32 = 27;
pub const wxSTC_FS_PREPROCESSOR_C: u32 = 28;
pub const wxSTC_FS_OPERATOR_C: u32 = 29;
pub const wxSTC_FS_IDENTIFIER_C: u32 = 30;
pub const wxSTC_FS_STRINGEOL_C: u32 = 31;
pub const wxSTC_CSOUND_DEFAULT: u32 = 0;
pub const wxSTC_CSOUND_COMMENT: u32 = 1;
pub const wxSTC_CSOUND_NUMBER: u32 = 2;
pub const wxSTC_CSOUND_OPERATOR: u32 = 3;
pub const wxSTC_CSOUND_INSTR: u32 = 4;
pub const wxSTC_CSOUND_IDENTIFIER: u32 = 5;
pub const wxSTC_CSOUND_OPCODE: u32 = 6;
pub const wxSTC_CSOUND_HEADERSTMT: u32 = 7;
pub const wxSTC_CSOUND_USERKEYWORD: u32 = 8;
pub const wxSTC_CSOUND_COMMENTBLOCK: u32 = 9;
pub const wxSTC_CSOUND_PARAM: u32 = 10;
pub const wxSTC_CSOUND_ARATE_VAR: u32 = 11;
pub const wxSTC_CSOUND_KRATE_VAR: u32 = 12;
pub const wxSTC_CSOUND_IRATE_VAR: u32 = 13;
pub const wxSTC_CSOUND_GLOBAL_VAR: u32 = 14;
pub const wxSTC_CSOUND_STRINGEOL: u32 = 15;
pub const wxSTC_INNO_DEFAULT: u32 = 0;
pub const wxSTC_INNO_COMMENT: u32 = 1;
pub const wxSTC_INNO_KEYWORD: u32 = 2;
pub const wxSTC_INNO_PARAMETER: u32 = 3;
pub const wxSTC_INNO_SECTION: u32 = 4;
pub const wxSTC_INNO_PREPROC: u32 = 5;
pub const wxSTC_INNO_INLINE_EXPANSION: u32 = 6;
pub const wxSTC_INNO_COMMENT_PASCAL: u32 = 7;
pub const wxSTC_INNO_KEYWORD_PASCAL: u32 = 8;
pub const wxSTC_INNO_KEYWORD_USER: u32 = 9;
pub const wxSTC_INNO_STRING_DOUBLE: u32 = 10;
pub const wxSTC_INNO_STRING_SINGLE: u32 = 11;
pub const wxSTC_INNO_IDENTIFIER: u32 = 12;
pub const wxSTC_OPAL_SPACE: u32 = 0;
pub const wxSTC_OPAL_COMMENT_BLOCK: u32 = 1;
pub const wxSTC_OPAL_COMMENT_LINE: u32 = 2;
pub const wxSTC_OPAL_INTEGER: u32 = 3;
pub const wxSTC_OPAL_KEYWORD: u32 = 4;
pub const wxSTC_OPAL_SORT: u32 = 5;
pub const wxSTC_OPAL_STRING: u32 = 6;
pub const wxSTC_OPAL_PAR: u32 = 7;
pub const wxSTC_OPAL_BOOL_CONST: u32 = 8;
pub const wxSTC_OPAL_DEFAULT: u32 = 32;
pub const wxSTC_SPICE_DEFAULT: u32 = 0;
pub const wxSTC_SPICE_IDENTIFIER: u32 = 1;
pub const wxSTC_SPICE_KEYWORD: u32 = 2;
pub const wxSTC_SPICE_KEYWORD2: u32 = 3;
pub const wxSTC_SPICE_KEYWORD3: u32 = 4;
pub const wxSTC_SPICE_NUMBER: u32 = 5;
pub const wxSTC_SPICE_DELIMITER: u32 = 6;
pub const wxSTC_SPICE_VALUE: u32 = 7;
pub const wxSTC_SPICE_COMMENTLINE: u32 = 8;
pub const wxSTC_CMAKE_DEFAULT: u32 = 0;
pub const wxSTC_CMAKE_COMMENT: u32 = 1;
pub const wxSTC_CMAKE_STRINGDQ: u32 = 2;
pub const wxSTC_CMAKE_STRINGLQ: u32 = 3;
pub const wxSTC_CMAKE_STRINGRQ: u32 = 4;
pub const wxSTC_CMAKE_COMMANDS: u32 = 5;
pub const wxSTC_CMAKE_PARAMETERS: u32 = 6;
pub const wxSTC_CMAKE_VARIABLE: u32 = 7;
pub const wxSTC_CMAKE_USERDEFINED: u32 = 8;
pub const wxSTC_CMAKE_WHILEDEF: u32 = 9;
pub const wxSTC_CMAKE_FOREACHDEF: u32 = 10;
pub const wxSTC_CMAKE_IFDEFINEDEF: u32 = 11;
pub const wxSTC_CMAKE_MACRODEF: u32 = 12;
pub const wxSTC_CMAKE_STRINGVAR: u32 = 13;
pub const wxSTC_CMAKE_NUMBER: u32 = 14;
pub const wxSTC_GAP_DEFAULT: u32 = 0;
pub const wxSTC_GAP_IDENTIFIER: u32 = 1;
pub const wxSTC_GAP_KEYWORD: u32 = 2;
pub const wxSTC_GAP_KEYWORD2: u32 = 3;
pub const wxSTC_GAP_KEYWORD3: u32 = 4;
pub const wxSTC_GAP_KEYWORD4: u32 = 5;
pub const wxSTC_GAP_STRING: u32 = 6;
pub const wxSTC_GAP_CHAR: u32 = 7;
pub const wxSTC_GAP_OPERATOR: u32 = 8;
pub const wxSTC_GAP_COMMENT: u32 = 9;
pub const wxSTC_GAP_NUMBER: u32 = 10;
pub const wxSTC_GAP_STRINGEOL: u32 = 11;
pub const wxSTC_PLM_DEFAULT: u32 = 0;
pub const wxSTC_PLM_COMMENT: u32 = 1;
pub const wxSTC_PLM_STRING: u32 = 2;
pub const wxSTC_PLM_NUMBER: u32 = 3;
pub const wxSTC_PLM_IDENTIFIER: u32 = 4;
pub const wxSTC_PLM_OPERATOR: u32 = 5;
pub const wxSTC_PLM_CONTROL: u32 = 6;
pub const wxSTC_PLM_KEYWORD: u32 = 7;
pub const wxSTC_ABL_DEFAULT: u32 = 0;
pub const wxSTC_ABL_NUMBER: u32 = 1;
pub const wxSTC_ABL_WORD: u32 = 2;
pub const wxSTC_ABL_STRING: u32 = 3;
pub const wxSTC_ABL_CHARACTER: u32 = 4;
pub const wxSTC_ABL_PREPROCESSOR: u32 = 5;
pub const wxSTC_ABL_OPERATOR: u32 = 6;
pub const wxSTC_ABL_IDENTIFIER: u32 = 7;
pub const wxSTC_ABL_BLOCK: u32 = 8;
pub const wxSTC_ABL_END: u32 = 9;
pub const wxSTC_ABL_COMMENT: u32 = 10;
pub const wxSTC_ABL_TASKMARKER: u32 = 11;
pub const wxSTC_ABL_LINECOMMENT: u32 = 12;
pub const wxSTC_ABAQUS_DEFAULT: u32 = 0;
pub const wxSTC_ABAQUS_COMMENT: u32 = 1;
pub const wxSTC_ABAQUS_COMMENTBLOCK: u32 = 2;
pub const wxSTC_ABAQUS_NUMBER: u32 = 3;
pub const wxSTC_ABAQUS_STRING: u32 = 4;
pub const wxSTC_ABAQUS_OPERATOR: u32 = 5;
pub const wxSTC_ABAQUS_WORD: u32 = 6;
pub const wxSTC_ABAQUS_PROCESSOR: u32 = 7;
pub const wxSTC_ABAQUS_COMMAND: u32 = 8;
pub const wxSTC_ABAQUS_SLASHCOMMAND: u32 = 9;
pub const wxSTC_ABAQUS_STARCOMMAND: u32 = 10;
pub const wxSTC_ABAQUS_ARGUMENT: u32 = 11;
pub const wxSTC_ABAQUS_FUNCTION: u32 = 12;
pub const wxSTC_ASY_DEFAULT: u32 = 0;
pub const wxSTC_ASY_COMMENT: u32 = 1;
pub const wxSTC_ASY_COMMENTLINE: u32 = 2;
pub const wxSTC_ASY_NUMBER: u32 = 3;
pub const wxSTC_ASY_WORD: u32 = 4;
pub const wxSTC_ASY_STRING: u32 = 5;
pub const wxSTC_ASY_CHARACTER: u32 = 6;
pub const wxSTC_ASY_OPERATOR: u32 = 7;
pub const wxSTC_ASY_IDENTIFIER: u32 = 8;
pub const wxSTC_ASY_STRINGEOL: u32 = 9;
pub const wxSTC_ASY_COMMENTLINEDOC: u32 = 10;
pub const wxSTC_ASY_WORD2: u32 = 11;
pub const wxSTC_R_DEFAULT: u32 = 0;
pub const wxSTC_R_COMMENT: u32 = 1;
pub const wxSTC_R_KWORD: u32 = 2;
pub const wxSTC_R_BASEKWORD: u32 = 3;
pub const wxSTC_R_OTHERKWORD: u32 = 4;
pub const wxSTC_R_NUMBER: u32 = 5;
pub const wxSTC_R_STRING: u32 = 6;
pub const wxSTC_R_STRING2: u32 = 7;
pub const wxSTC_R_OPERATOR: u32 = 8;
pub const wxSTC_R_IDENTIFIER: u32 = 9;
pub const wxSTC_R_INFIX: u32 = 10;
pub const wxSTC_R_INFIXEOL: u32 = 11;
pub const wxSTC_MAGIK_DEFAULT: u32 = 0;
pub const wxSTC_MAGIK_COMMENT: u32 = 1;
pub const wxSTC_MAGIK_HYPER_COMMENT: u32 = 16;
pub const wxSTC_MAGIK_STRING: u32 = 2;
pub const wxSTC_MAGIK_CHARACTER: u32 = 3;
pub const wxSTC_MAGIK_NUMBER: u32 = 4;
pub const wxSTC_MAGIK_IDENTIFIER: u32 = 5;
pub const wxSTC_MAGIK_OPERATOR: u32 = 6;
pub const wxSTC_MAGIK_FLOW: u32 = 7;
pub const wxSTC_MAGIK_CONTAINER: u32 = 8;
pub const wxSTC_MAGIK_BRACKET_BLOCK: u32 = 9;
pub const wxSTC_MAGIK_BRACE_BLOCK: u32 = 10;
pub const wxSTC_MAGIK_SQBRACKET_BLOCK: u32 = 11;
pub const wxSTC_MAGIK_UNKNOWN_KEYWORD: u32 = 12;
pub const wxSTC_MAGIK_KEYWORD: u32 = 13;
pub const wxSTC_MAGIK_PRAGMA: u32 = 14;
pub const wxSTC_MAGIK_SYMBOL: u32 = 15;
pub const wxSTC_POWERSHELL_DEFAULT: u32 = 0;
pub const wxSTC_POWERSHELL_COMMENT: u32 = 1;
pub const wxSTC_POWERSHELL_STRING: u32 = 2;
pub const wxSTC_POWERSHELL_CHARACTER: u32 = 3;
pub const wxSTC_POWERSHELL_NUMBER: u32 = 4;
pub const wxSTC_POWERSHELL_VARIABLE: u32 = 5;
pub const wxSTC_POWERSHELL_OPERATOR: u32 = 6;
pub const wxSTC_POWERSHELL_IDENTIFIER: u32 = 7;
pub const wxSTC_POWERSHELL_KEYWORD: u32 = 8;
pub const wxSTC_POWERSHELL_CMDLET: u32 = 9;
pub const wxSTC_POWERSHELL_ALIAS: u32 = 10;
pub const wxSTC_POWERSHELL_FUNCTION: u32 = 11;
pub const wxSTC_POWERSHELL_USER1: u32 = 12;
pub const wxSTC_POWERSHELL_COMMENTSTREAM: u32 = 13;
pub const wxSTC_POWERSHELL_HERE_STRING: u32 = 14;
pub const wxSTC_POWERSHELL_HERE_CHARACTER: u32 = 15;
pub const wxSTC_POWERSHELL_COMMENTDOCKEYWORD: u32 = 16;
pub const wxSTC_MYSQL_DEFAULT: u32 = 0;
pub const wxSTC_MYSQL_COMMENT: u32 = 1;
pub const wxSTC_MYSQL_COMMENTLINE: u32 = 2;
pub const wxSTC_MYSQL_VARIABLE: u32 = 3;
pub const wxSTC_MYSQL_SYSTEMVARIABLE: u32 = 4;
pub const wxSTC_MYSQL_KNOWNSYSTEMVARIABLE: u32 = 5;
pub const wxSTC_MYSQL_NUMBER: u32 = 6;
pub const wxSTC_MYSQL_MAJORKEYWORD: u32 = 7;
pub const wxSTC_MYSQL_KEYWORD: u32 = 8;
pub const wxSTC_MYSQL_DATABASEOBJECT: u32 = 9;
pub const wxSTC_MYSQL_PROCEDUREKEYWORD: u32 = 10;
pub const wxSTC_MYSQL_STRING: u32 = 11;
pub const wxSTC_MYSQL_SQSTRING: u32 = 12;
pub const wxSTC_MYSQL_DQSTRING: u32 = 13;
pub const wxSTC_MYSQL_OPERATOR: u32 = 14;
pub const wxSTC_MYSQL_FUNCTION: u32 = 15;
pub const wxSTC_MYSQL_IDENTIFIER: u32 = 16;
pub const wxSTC_MYSQL_QUOTEDIDENTIFIER: u32 = 17;
pub const wxSTC_MYSQL_USER1: u32 = 18;
pub const wxSTC_MYSQL_USER2: u32 = 19;
pub const wxSTC_MYSQL_USER3: u32 = 20;
pub const wxSTC_MYSQL_HIDDENCOMMAND: u32 = 21;
pub const wxSTC_MYSQL_PLACEHOLDER: u32 = 22;
pub const wxSTC_PO_DEFAULT: u32 = 0;
pub const wxSTC_PO_COMMENT: u32 = 1;
pub const wxSTC_PO_MSGID: u32 = 2;
pub const wxSTC_PO_MSGID_TEXT: u32 = 3;
pub const wxSTC_PO_MSGSTR: u32 = 4;
pub const wxSTC_PO_MSGSTR_TEXT: u32 = 5;
pub const wxSTC_PO_MSGCTXT: u32 = 6;
pub const wxSTC_PO_MSGCTXT_TEXT: u32 = 7;
pub const wxSTC_PO_FUZZY: u32 = 8;
pub const wxSTC_PO_PROGRAMMER_COMMENT: u32 = 9;
pub const wxSTC_PO_REFERENCE: u32 = 10;
pub const wxSTC_PO_FLAGS: u32 = 11;
pub const wxSTC_PO_MSGID_TEXT_EOL: u32 = 12;
pub const wxSTC_PO_MSGSTR_TEXT_EOL: u32 = 13;
pub const wxSTC_PO_MSGCTXT_TEXT_EOL: u32 = 14;
pub const wxSTC_PO_ERROR: u32 = 15;
pub const wxSTC_PAS_DEFAULT: u32 = 0;
pub const wxSTC_PAS_IDENTIFIER: u32 = 1;
pub const wxSTC_PAS_COMMENT: u32 = 2;
pub const wxSTC_PAS_COMMENT2: u32 = 3;
pub const wxSTC_PAS_COMMENTLINE: u32 = 4;
pub const wxSTC_PAS_PREPROCESSOR: u32 = 5;
pub const wxSTC_PAS_PREPROCESSOR2: u32 = 6;
pub const wxSTC_PAS_NUMBER: u32 = 7;
pub const wxSTC_PAS_HEXNUMBER: u32 = 8;
pub const wxSTC_PAS_WORD: u32 = 9;
pub const wxSTC_PAS_STRING: u32 = 10;
pub const wxSTC_PAS_STRINGEOL: u32 = 11;
pub const wxSTC_PAS_CHARACTER: u32 = 12;
pub const wxSTC_PAS_OPERATOR: u32 = 13;
pub const wxSTC_PAS_ASM: u32 = 14;
pub const wxSTC_SORCUS_DEFAULT: u32 = 0;
pub const wxSTC_SORCUS_COMMAND: u32 = 1;
pub const wxSTC_SORCUS_PARAMETER: u32 = 2;
pub const wxSTC_SORCUS_COMMENTLINE: u32 = 3;
pub const wxSTC_SORCUS_STRING: u32 = 4;
pub const wxSTC_SORCUS_STRINGEOL: u32 = 5;
pub const wxSTC_SORCUS_IDENTIFIER: u32 = 6;
pub const wxSTC_SORCUS_OPERATOR: u32 = 7;
pub const wxSTC_SORCUS_NUMBER: u32 = 8;
pub const wxSTC_SORCUS_CONSTANT: u32 = 9;
pub const wxSTC_POWERPRO_DEFAULT: u32 = 0;
pub const wxSTC_POWERPRO_COMMENTBLOCK: u32 = 1;
pub const wxSTC_POWERPRO_COMMENTLINE: u32 = 2;
pub const wxSTC_POWERPRO_NUMBER: u32 = 3;
pub const wxSTC_POWERPRO_WORD: u32 = 4;
pub const wxSTC_POWERPRO_WORD2: u32 = 5;
pub const wxSTC_POWERPRO_WORD3: u32 = 6;
pub const wxSTC_POWERPRO_WORD4: u32 = 7;
pub const wxSTC_POWERPRO_DOUBLEQUOTEDSTRING: u32 = 8;
pub const wxSTC_POWERPRO_SINGLEQUOTEDSTRING: u32 = 9;
pub const wxSTC_POWERPRO_LINECONTINUE: u32 = 10;
pub const wxSTC_POWERPRO_OPERATOR: u32 = 11;
pub const wxSTC_POWERPRO_IDENTIFIER: u32 = 12;
pub const wxSTC_POWERPRO_STRINGEOL: u32 = 13;
pub const wxSTC_POWERPRO_VERBATIM: u32 = 14;
pub const wxSTC_POWERPRO_ALTQUOTE: u32 = 15;
pub const wxSTC_POWERPRO_FUNCTION: u32 = 16;
pub const wxSTC_SML_DEFAULT: u32 = 0;
pub const wxSTC_SML_IDENTIFIER: u32 = 1;
pub const wxSTC_SML_TAGNAME: u32 = 2;
pub const wxSTC_SML_KEYWORD: u32 = 3;
pub const wxSTC_SML_KEYWORD2: u32 = 4;
pub const wxSTC_SML_KEYWORD3: u32 = 5;
pub const wxSTC_SML_LINENUM: u32 = 6;
pub const wxSTC_SML_OPERATOR: u32 = 7;
pub const wxSTC_SML_NUMBER: u32 = 8;
pub const wxSTC_SML_CHAR: u32 = 9;
pub const wxSTC_SML_STRING: u32 = 11;
pub const wxSTC_SML_COMMENT: u32 = 12;
pub const wxSTC_SML_COMMENT1: u32 = 13;
pub const wxSTC_SML_COMMENT2: u32 = 14;
pub const wxSTC_SML_COMMENT3: u32 = 15;
pub const wxSTC_MARKDOWN_DEFAULT: u32 = 0;
pub const wxSTC_MARKDOWN_LINE_BEGIN: u32 = 1;
pub const wxSTC_MARKDOWN_STRONG1: u32 = 2;
pub const wxSTC_MARKDOWN_STRONG2: u32 = 3;
pub const wxSTC_MARKDOWN_EM1: u32 = 4;
pub const wxSTC_MARKDOWN_EM2: u32 = 5;
pub const wxSTC_MARKDOWN_HEADER1: u32 = 6;
pub const wxSTC_MARKDOWN_HEADER2: u32 = 7;
pub const wxSTC_MARKDOWN_HEADER3: u32 = 8;
pub const wxSTC_MARKDOWN_HEADER4: u32 = 9;
pub const wxSTC_MARKDOWN_HEADER5: u32 = 10;
pub const wxSTC_MARKDOWN_HEADER6: u32 = 11;
pub const wxSTC_MARKDOWN_PRECHAR: u32 = 12;
pub const wxSTC_MARKDOWN_ULIST_ITEM: u32 = 13;
pub const wxSTC_MARKDOWN_OLIST_ITEM: u32 = 14;
pub const wxSTC_MARKDOWN_BLOCKQUOTE: u32 = 15;
pub const wxSTC_MARKDOWN_STRIKEOUT: u32 = 16;
pub const wxSTC_MARKDOWN_HRULE: u32 = 17;
pub const wxSTC_MARKDOWN_LINK: u32 = 18;
pub const wxSTC_MARKDOWN_CODE: u32 = 19;
pub const wxSTC_MARKDOWN_CODE2: u32 = 20;
pub const wxSTC_MARKDOWN_CODEBK: u32 = 21;
pub const wxSTC_TXT2TAGS_DEFAULT: u32 = 0;
pub const wxSTC_TXT2TAGS_LINE_BEGIN: u32 = 1;
pub const wxSTC_TXT2TAGS_STRONG1: u32 = 2;
pub const wxSTC_TXT2TAGS_STRONG2: u32 = 3;
pub const wxSTC_TXT2TAGS_EM1: u32 = 4;
pub const wxSTC_TXT2TAGS_EM2: u32 = 5;
pub const wxSTC_TXT2TAGS_HEADER1: u32 = 6;
pub const wxSTC_TXT2TAGS_HEADER2: u32 = 7;
pub const wxSTC_TXT2TAGS_HEADER3: u32 = 8;
pub const wxSTC_TXT2TAGS_HEADER4: u32 = 9;
pub const wxSTC_TXT2TAGS_HEADER5: u32 = 10;
pub const wxSTC_TXT2TAGS_HEADER6: u32 = 11;
pub const wxSTC_TXT2TAGS_PRECHAR: u32 = 12;
pub const wxSTC_TXT2TAGS_ULIST_ITEM: u32 = 13;
pub const wxSTC_TXT2TAGS_OLIST_ITEM: u32 = 14;
pub const wxSTC_TXT2TAGS_BLOCKQUOTE: u32 = 15;
pub const wxSTC_TXT2TAGS_STRIKEOUT: u32 = 16;
pub const wxSTC_TXT2TAGS_HRULE: u32 = 17;
pub const wxSTC_TXT2TAGS_LINK: u32 = 18;
pub const wxSTC_TXT2TAGS_CODE: u32 = 19;
pub const wxSTC_TXT2TAGS_CODE2: u32 = 20;
pub const wxSTC_TXT2TAGS_CODEBK: u32 = 21;
pub const wxSTC_TXT2TAGS_COMMENT: u32 = 22;
pub const wxSTC_TXT2TAGS_OPTION: u32 = 23;
pub const wxSTC_TXT2TAGS_PREPROC: u32 = 24;
pub const wxSTC_TXT2TAGS_POSTPROC: u32 = 25;
pub const wxSTC_A68K_DEFAULT: u32 = 0;
pub const wxSTC_A68K_COMMENT: u32 = 1;
pub const wxSTC_A68K_NUMBER_DEC: u32 = 2;
pub const wxSTC_A68K_NUMBER_BIN: u32 = 3;
pub const wxSTC_A68K_NUMBER_HEX: u32 = 4;
pub const wxSTC_A68K_STRING1: u32 = 5;
pub const wxSTC_A68K_OPERATOR: u32 = 6;
pub const wxSTC_A68K_CPUINSTRUCTION: u32 = 7;
pub const wxSTC_A68K_EXTINSTRUCTION: u32 = 8;
pub const wxSTC_A68K_REGISTER: u32 = 9;
pub const wxSTC_A68K_DIRECTIVE: u32 = 10;
pub const wxSTC_A68K_MACRO_ARG: u32 = 11;
pub const wxSTC_A68K_LABEL: u32 = 12;
pub const wxSTC_A68K_STRING2: u32 = 13;
pub const wxSTC_A68K_IDENTIFIER: u32 = 14;
pub const wxSTC_A68K_MACRO_DECLARATION: u32 = 15;
pub const wxSTC_A68K_COMMENT_WORD: u32 = 16;
pub const wxSTC_A68K_COMMENT_SPECIAL: u32 = 17;
pub const wxSTC_A68K_COMMENT_DOXYGEN: u32 = 18;
pub const wxSTC_MODULA_DEFAULT: u32 = 0;
pub const wxSTC_MODULA_COMMENT: u32 = 1;
pub const wxSTC_MODULA_DOXYCOMM: u32 = 2;
pub const wxSTC_MODULA_DOXYKEY: u32 = 3;
pub const wxSTC_MODULA_KEYWORD: u32 = 4;
pub const wxSTC_MODULA_RESERVED: u32 = 5;
pub const wxSTC_MODULA_NUMBER: u32 = 6;
pub const wxSTC_MODULA_BASENUM: u32 = 7;
pub const wxSTC_MODULA_FLOAT: u32 = 8;
pub const wxSTC_MODULA_STRING: u32 = 9;
pub const wxSTC_MODULA_STRSPEC: u32 = 10;
pub const wxSTC_MODULA_CHAR: u32 = 11;
pub const wxSTC_MODULA_CHARSPEC: u32 = 12;
pub const wxSTC_MODULA_PROC: u32 = 13;
pub const wxSTC_MODULA_PRAGMA: u32 = 14;
pub const wxSTC_MODULA_PRGKEY: u32 = 15;
pub const wxSTC_MODULA_OPERATOR: u32 = 16;
pub const wxSTC_MODULA_BADSTR: u32 = 17;
pub const wxSTC_COFFEESCRIPT_DEFAULT: u32 = 0;
pub const wxSTC_COFFEESCRIPT_COMMENT: u32 = 1;
pub const wxSTC_COFFEESCRIPT_COMMENTLINE: u32 = 2;
pub const wxSTC_COFFEESCRIPT_COMMENTDOC: u32 = 3;
pub const wxSTC_COFFEESCRIPT_NUMBER: u32 = 4;
pub const wxSTC_COFFEESCRIPT_WORD: u32 = 5;
pub const wxSTC_COFFEESCRIPT_STRING: u32 = 6;
pub const wxSTC_COFFEESCRIPT_CHARACTER: u32 = 7;
pub const wxSTC_COFFEESCRIPT_UUID: u32 = 8;
pub const wxSTC_COFFEESCRIPT_PREPROCESSOR: u32 = 9;
pub const wxSTC_COFFEESCRIPT_OPERATOR: u32 = 10;
pub const wxSTC_COFFEESCRIPT_IDENTIFIER: u32 = 11;
pub const wxSTC_COFFEESCRIPT_STRINGEOL: u32 = 12;
pub const wxSTC_COFFEESCRIPT_VERBATIM: u32 = 13;
pub const wxSTC_COFFEESCRIPT_REGEX: u32 = 14;
pub const wxSTC_COFFEESCRIPT_COMMENTLINEDOC: u32 = 15;
pub const wxSTC_COFFEESCRIPT_WORD2: u32 = 16;
pub const wxSTC_COFFEESCRIPT_COMMENTDOCKEYWORD: u32 = 17;
pub const wxSTC_COFFEESCRIPT_COMMENTDOCKEYWORDERROR: u32 = 18;
pub const wxSTC_COFFEESCRIPT_GLOBALCLASS: u32 = 19;
pub const wxSTC_COFFEESCRIPT_STRINGRAW: u32 = 20;
pub const wxSTC_COFFEESCRIPT_TRIPLEVERBATIM: u32 = 21;
pub const wxSTC_COFFEESCRIPT_COMMENTBLOCK: u32 = 22;
pub const wxSTC_COFFEESCRIPT_VERBOSE_REGEX: u32 = 23;
pub const wxSTC_COFFEESCRIPT_VERBOSE_REGEX_COMMENT: u32 = 24;
pub const wxSTC_COFFEESCRIPT_INSTANCEPROPERTY: u32 = 25;
pub const wxSTC_AVS_DEFAULT: u32 = 0;
pub const wxSTC_AVS_COMMENTBLOCK: u32 = 1;
pub const wxSTC_AVS_COMMENTBLOCKN: u32 = 2;
pub const wxSTC_AVS_COMMENTLINE: u32 = 3;
pub const wxSTC_AVS_NUMBER: u32 = 4;
pub const wxSTC_AVS_OPERATOR: u32 = 5;
pub const wxSTC_AVS_IDENTIFIER: u32 = 6;
pub const wxSTC_AVS_STRING: u32 = 7;
pub const wxSTC_AVS_TRIPLESTRING: u32 = 8;
pub const wxSTC_AVS_KEYWORD: u32 = 9;
pub const wxSTC_AVS_FILTER: u32 = 10;
pub const wxSTC_AVS_PLUGIN: u32 = 11;
pub const wxSTC_AVS_FUNCTION: u32 = 12;
pub const wxSTC_AVS_CLIPPROP: u32 = 13;
pub const wxSTC_AVS_USERDFN: u32 = 14;
pub const wxSTC_ECL_DEFAULT: u32 = 0;
pub const wxSTC_ECL_COMMENT: u32 = 1;
pub const wxSTC_ECL_COMMENTLINE: u32 = 2;
pub const wxSTC_ECL_NUMBER: u32 = 3;
pub const wxSTC_ECL_STRING: u32 = 4;
pub const wxSTC_ECL_WORD0: u32 = 5;
pub const wxSTC_ECL_OPERATOR: u32 = 6;
pub const wxSTC_ECL_CHARACTER: u32 = 7;
pub const wxSTC_ECL_UUID: u32 = 8;
pub const wxSTC_ECL_PREPROCESSOR: u32 = 9;
pub const wxSTC_ECL_UNKNOWN: u32 = 10;
pub const wxSTC_ECL_IDENTIFIER: u32 = 11;
pub const wxSTC_ECL_STRINGEOL: u32 = 12;
pub const wxSTC_ECL_VERBATIM: u32 = 13;
pub const wxSTC_ECL_REGEX: u32 = 14;
pub const wxSTC_ECL_COMMENTLINEDOC: u32 = 15;
pub const wxSTC_ECL_WORD1: u32 = 16;
pub const wxSTC_ECL_COMMENTDOCKEYWORD: u32 = 17;
pub const wxSTC_ECL_COMMENTDOCKEYWORDERROR: u32 = 18;
pub const wxSTC_ECL_WORD2: u32 = 19;
pub const wxSTC_ECL_WORD3: u32 = 20;
pub const wxSTC_ECL_WORD4: u32 = 21;
pub const wxSTC_ECL_WORD5: u32 = 22;
pub const wxSTC_ECL_COMMENTDOC: u32 = 23;
pub const wxSTC_ECL_ADDED: u32 = 24;
pub const wxSTC_ECL_DELETED: u32 = 25;
pub const wxSTC_ECL_CHANGED: u32 = 26;
pub const wxSTC_ECL_MOVED: u32 = 27;
pub const wxSTC_OSCRIPT_DEFAULT: u32 = 0;
pub const wxSTC_OSCRIPT_LINE_COMMENT: u32 = 1;
pub const wxSTC_OSCRIPT_BLOCK_COMMENT: u32 = 2;
pub const wxSTC_OSCRIPT_DOC_COMMENT: u32 = 3;
pub const wxSTC_OSCRIPT_PREPROCESSOR: u32 = 4;
pub const wxSTC_OSCRIPT_NUMBER: u32 = 5;
pub const wxSTC_OSCRIPT_SINGLEQUOTE_STRING: u32 = 6;
pub const wxSTC_OSCRIPT_DOUBLEQUOTE_STRING: u32 = 7;
pub const wxSTC_OSCRIPT_CONSTANT: u32 = 8;
pub const wxSTC_OSCRIPT_IDENTIFIER: u32 = 9;
pub const wxSTC_OSCRIPT_GLOBAL: u32 = 10;
pub const wxSTC_OSCRIPT_KEYWORD: u32 = 11;
pub const wxSTC_OSCRIPT_OPERATOR: u32 = 12;
pub const wxSTC_OSCRIPT_LABEL: u32 = 13;
pub const wxSTC_OSCRIPT_TYPE: u32 = 14;
pub const wxSTC_OSCRIPT_FUNCTION: u32 = 15;
pub const wxSTC_OSCRIPT_OBJECT: u32 = 16;
pub const wxSTC_OSCRIPT_PROPERTY: u32 = 17;
pub const wxSTC_OSCRIPT_METHOD: u32 = 18;
pub const wxSTC_VISUALPROLOG_DEFAULT: u32 = 0;
pub const wxSTC_VISUALPROLOG_KEY_MAJOR: u32 = 1;
pub const wxSTC_VISUALPROLOG_KEY_MINOR: u32 = 2;
pub const wxSTC_VISUALPROLOG_KEY_DIRECTIVE: u32 = 3;
pub const wxSTC_VISUALPROLOG_COMMENT_BLOCK: u32 = 4;
pub const wxSTC_VISUALPROLOG_COMMENT_LINE: u32 = 5;
pub const wxSTC_VISUALPROLOG_COMMENT_KEY: u32 = 6;
pub const wxSTC_VISUALPROLOG_COMMENT_KEY_ERROR: u32 = 7;
pub const wxSTC_VISUALPROLOG_IDENTIFIER: u32 = 8;
pub const wxSTC_VISUALPROLOG_VARIABLE: u32 = 9;
pub const wxSTC_VISUALPROLOG_ANONYMOUS: u32 = 10;
pub const wxSTC_VISUALPROLOG_NUMBER: u32 = 11;
pub const wxSTC_VISUALPROLOG_OPERATOR: u32 = 12;
pub const wxSTC_VISUALPROLOG_CHARACTER: u32 = 13;
pub const wxSTC_VISUALPROLOG_CHARACTER_TOO_MANY: u32 = 14;
pub const wxSTC_VISUALPROLOG_CHARACTER_ESCAPE_ERROR: u32 = 15;
pub const wxSTC_VISUALPROLOG_STRING: u32 = 16;
pub const wxSTC_VISUALPROLOG_STRING_ESCAPE: u32 = 17;
pub const wxSTC_VISUALPROLOG_STRING_ESCAPE_ERROR: u32 = 18;
pub const wxSTC_VISUALPROLOG_STRING_EOL_OPEN: u32 = 19;
pub const wxSTC_VISUALPROLOG_STRING_VERBATIM: u32 = 20;
pub const wxSTC_VISUALPROLOG_STRING_VERBATIM_SPECIAL: u32 = 21;
pub const wxSTC_VISUALPROLOG_STRING_VERBATIM_EOL: u32 = 22;
pub const wxSTC_STTXT_DEFAULT: u32 = 0;
pub const wxSTC_STTXT_COMMENT: u32 = 1;
pub const wxSTC_STTXT_COMMENTLINE: u32 = 2;
pub const wxSTC_STTXT_KEYWORD: u32 = 3;
pub const wxSTC_STTXT_TYPE: u32 = 4;
pub const wxSTC_STTXT_FUNCTION: u32 = 5;
pub const wxSTC_STTXT_FB: u32 = 6;
pub const wxSTC_STTXT_NUMBER: u32 = 7;
pub const wxSTC_STTXT_HEXNUMBER: u32 = 8;
pub const wxSTC_STTXT_PRAGMA: u32 = 9;
pub const wxSTC_STTXT_OPERATOR: u32 = 10;
pub const wxSTC_STTXT_CHARACTER: u32 = 11;
pub const wxSTC_STTXT_STRING1: u32 = 12;
pub const wxSTC_STTXT_STRING2: u32 = 13;
pub const wxSTC_STTXT_STRINGEOL: u32 = 14;
pub const wxSTC_STTXT_IDENTIFIER: u32 = 15;
pub const wxSTC_STTXT_DATETIME: u32 = 16;
pub const wxSTC_STTXT_VARS: u32 = 17;
pub const wxSTC_STTXT_PRAGMAS: u32 = 18;
pub const wxSTC_KVIRC_DEFAULT: u32 = 0;
pub const wxSTC_KVIRC_COMMENT: u32 = 1;
pub const wxSTC_KVIRC_COMMENTBLOCK: u32 = 2;
pub const wxSTC_KVIRC_STRING: u32 = 3;
pub const wxSTC_KVIRC_WORD: u32 = 4;
pub const wxSTC_KVIRC_KEYWORD: u32 = 5;
pub const wxSTC_KVIRC_FUNCTION_KEYWORD: u32 = 6;
pub const wxSTC_KVIRC_FUNCTION: u32 = 7;
pub const wxSTC_KVIRC_VARIABLE: u32 = 8;
pub const wxSTC_KVIRC_NUMBER: u32 = 9;
pub const wxSTC_KVIRC_OPERATOR: u32 = 10;
pub const wxSTC_KVIRC_STRING_FUNCTION: u32 = 11;
pub const wxSTC_KVIRC_STRING_VARIABLE: u32 = 12;
pub const wxSTC_RUST_DEFAULT: u32 = 0;
pub const wxSTC_RUST_COMMENTBLOCK: u32 = 1;
pub const wxSTC_RUST_COMMENTLINE: u32 = 2;
pub const wxSTC_RUST_COMMENTBLOCKDOC: u32 = 3;
pub const wxSTC_RUST_COMMENTLINEDOC: u32 = 4;
pub const wxSTC_RUST_NUMBER: u32 = 5;
pub const wxSTC_RUST_WORD: u32 = 6;
pub const wxSTC_RUST_WORD2: u32 = 7;
pub const wxSTC_RUST_WORD3: u32 = 8;
pub const wxSTC_RUST_WORD4: u32 = 9;
pub const wxSTC_RUST_WORD5: u32 = 10;
pub const wxSTC_RUST_WORD6: u32 = 11;
pub const wxSTC_RUST_WORD7: u32 = 12;
pub const wxSTC_RUST_STRING: u32 = 13;
pub const wxSTC_RUST_STRINGR: u32 = 14;
pub const wxSTC_RUST_CHARACTER: u32 = 15;
pub const wxSTC_RUST_OPERATOR: u32 = 16;
pub const wxSTC_RUST_IDENTIFIER: u32 = 17;
pub const wxSTC_RUST_LIFETIME: u32 = 18;
pub const wxSTC_RUST_MACRO: u32 = 19;
pub const wxSTC_RUST_LEXERROR: u32 = 20;
pub const wxSTC_RUST_BYTESTRING: u32 = 21;
pub const wxSTC_RUST_BYTESTRINGR: u32 = 22;
pub const wxSTC_RUST_BYTECHARACTER: u32 = 23;
pub const wxSTC_DMAP_DEFAULT: u32 = 0;
pub const wxSTC_DMAP_COMMENT: u32 = 1;
pub const wxSTC_DMAP_NUMBER: u32 = 2;
pub const wxSTC_DMAP_STRING1: u32 = 3;
pub const wxSTC_DMAP_STRING2: u32 = 4;
pub const wxSTC_DMAP_STRINGEOL: u32 = 5;
pub const wxSTC_DMAP_OPERATOR: u32 = 6;
pub const wxSTC_DMAP_IDENTIFIER: u32 = 7;
pub const wxSTC_DMAP_WORD: u32 = 8;
pub const wxSTC_DMAP_WORD2: u32 = 9;
pub const wxSTC_DMAP_WORD3: u32 = 10;
pub const wxSTC_DMIS_DEFAULT: u32 = 0;
pub const wxSTC_DMIS_COMMENT: u32 = 1;
pub const wxSTC_DMIS_STRING: u32 = 2;
pub const wxSTC_DMIS_NUMBER: u32 = 3;
pub const wxSTC_DMIS_KEYWORD: u32 = 4;
pub const wxSTC_DMIS_MAJORWORD: u32 = 5;
pub const wxSTC_DMIS_MINORWORD: u32 = 6;
pub const wxSTC_DMIS_UNSUPPORTED_MAJOR: u32 = 7;
pub const wxSTC_DMIS_UNSUPPORTED_MINOR: u32 = 8;
pub const wxSTC_DMIS_LABEL: u32 = 9;
pub const wxSTC_REG_DEFAULT: u32 = 0;
pub const wxSTC_REG_COMMENT: u32 = 1;
pub const wxSTC_REG_VALUENAME: u32 = 2;
pub const wxSTC_REG_STRING: u32 = 3;
pub const wxSTC_REG_HEXDIGIT: u32 = 4;
pub const wxSTC_REG_VALUETYPE: u32 = 5;
pub const wxSTC_REG_ADDEDKEY: u32 = 6;
pub const wxSTC_REG_DELETEDKEY: u32 = 7;
pub const wxSTC_REG_ESCAPED: u32 = 8;
pub const wxSTC_REG_KEYPATH_GUID: u32 = 9;
pub const wxSTC_REG_STRING_GUID: u32 = 10;
pub const wxSTC_REG_PARAMETER: u32 = 11;
pub const wxSTC_REG_OPERATOR: u32 = 12;
pub const wxSTC_BIBTEX_DEFAULT: u32 = 0;
pub const wxSTC_BIBTEX_ENTRY: u32 = 1;
pub const wxSTC_BIBTEX_UNKNOWN_ENTRY: u32 = 2;
pub const wxSTC_BIBTEX_KEY: u32 = 3;
pub const wxSTC_BIBTEX_PARAMETER: u32 = 4;
pub const wxSTC_BIBTEX_VALUE: u32 = 5;
pub const wxSTC_BIBTEX_COMMENT: u32 = 6;
pub const wxSTC_HEX_DEFAULT: u32 = 0;
pub const wxSTC_HEX_RECSTART: u32 = 1;
pub const wxSTC_HEX_RECTYPE: u32 = 2;
pub const wxSTC_HEX_RECTYPE_UNKNOWN: u32 = 3;
pub const wxSTC_HEX_BYTECOUNT: u32 = 4;
pub const wxSTC_HEX_BYTECOUNT_WRONG: u32 = 5;
pub const wxSTC_HEX_NOADDRESS: u32 = 6;
pub const wxSTC_HEX_DATAADDRESS: u32 = 7;
pub const wxSTC_HEX_RECCOUNT: u32 = 8;
pub const wxSTC_HEX_STARTADDRESS: u32 = 9;
pub const wxSTC_HEX_ADDRESSFIELD_UNKNOWN: u32 = 10;
pub const wxSTC_HEX_EXTENDEDADDRESS: u32 = 11;
pub const wxSTC_HEX_DATA_ODD: u32 = 12;
pub const wxSTC_HEX_DATA_EVEN: u32 = 13;
pub const wxSTC_HEX_DATA_UNKNOWN: u32 = 14;
pub const wxSTC_HEX_DATA_EMPTY: u32 = 15;
pub const wxSTC_HEX_CHECKSUM: u32 = 16;
pub const wxSTC_HEX_CHECKSUM_WRONG: u32 = 17;
pub const wxSTC_HEX_GARBAGE: u32 = 18;
pub const wxSTC_JSON_DEFAULT: u32 = 0;
pub const wxSTC_JSON_NUMBER: u32 = 1;
pub const wxSTC_JSON_STRING: u32 = 2;
pub const wxSTC_JSON_STRINGEOL: u32 = 3;
pub const wxSTC_JSON_PROPERTYNAME: u32 = 4;
pub const wxSTC_JSON_ESCAPESEQUENCE: u32 = 5;
pub const wxSTC_JSON_LINECOMMENT: u32 = 6;
pub const wxSTC_JSON_BLOCKCOMMENT: u32 = 7;
pub const wxSTC_JSON_OPERATOR: u32 = 8;
pub const wxSTC_JSON_URI: u32 = 9;
pub const wxSTC_JSON_COMPACTIRI: u32 = 10;
pub const wxSTC_JSON_KEYWORD: u32 = 11;
pub const wxSTC_JSON_LDKEYWORD: u32 = 12;
pub const wxSTC_JSON_ERROR: u32 = 13;
pub const wxSTC_EDI_DEFAULT: u32 = 0;
pub const wxSTC_EDI_SEGMENTSTART: u32 = 1;
pub const wxSTC_EDI_SEGMENTEND: u32 = 2;
pub const wxSTC_EDI_SEP_ELEMENT: u32 = 3;
pub const wxSTC_EDI_SEP_COMPOSITE: u32 = 4;
pub const wxSTC_EDI_SEP_RELEASE: u32 = 5;
pub const wxSTC_EDI_UNA: u32 = 6;
pub const wxSTC_EDI_UNH: u32 = 7;
pub const wxSTC_EDI_BADSEGMENT: u32 = 8;
pub const wxSTC_INDIC0_MASK: u32 = 0x20;
pub const wxSTC_INDIC1_MASK: u32 = 0x40;
pub const wxSTC_INDIC2_MASK: u32 = 0x80;
pub const wxSTC_INDICS_MASK: u32 = 0xE0;
pub const wxSTC_CMD_REDO: u32 = 2011;
pub const wxSTC_CMD_SELECTALL: u32 = 2013;
pub const wxSTC_CMD_UNDO: u32 = 2176;
pub const wxSTC_CMD_CUT: u32 = 2177;
pub const wxSTC_CMD_COPY: u32 = 2178;
pub const wxSTC_CMD_PASTE: u32 = 2179;
pub const wxSTC_CMD_CLEAR: u32 = 2180;
pub const wxSTC_CMD_LINEDOWN: u32 = 2300;
pub const wxSTC_CMD_LINEDOWNEXTEND: u32 = 2301;
pub const wxSTC_CMD_LINEUP: u32 = 2302;
pub const wxSTC_CMD_LINEUPEXTEND: u32 = 2303;
pub const wxSTC_CMD_CHARLEFT: u32 = 2304;
pub const wxSTC_CMD_CHARLEFTEXTEND: u32 = 2305;
pub const wxSTC_CMD_CHARRIGHT: u32 = 2306;
pub const wxSTC_CMD_CHARRIGHTEXTEND: u32 = 2307;
pub const wxSTC_CMD_WORDLEFT: u32 = 2308;
pub const wxSTC_CMD_WORDLEFTEXTEND: u32 = 2309;
pub const wxSTC_CMD_WORDRIGHT: u32 = 2310;
pub const wxSTC_CMD_WORDRIGHTEXTEND: u32 = 2311;
pub const wxSTC_CMD_HOME: u32 = 2312;
pub const wxSTC_CMD_HOMEEXTEND: u32 = 2313;
pub const wxSTC_CMD_LINEEND: u32 = 2314;
pub const wxSTC_CMD_LINEENDEXTEND: u32 = 2315;
pub const wxSTC_CMD_DOCUMENTSTART: u32 = 2316;
pub const wxSTC_CMD_DOCUMENTSTARTEXTEND: u32 = 2317;
pub const wxSTC_CMD_DOCUMENTEND: u32 = 2318;
pub const wxSTC_CMD_DOCUMENTENDEXTEND: u32 = 2319;
pub const wxSTC_CMD_PAGEUP: u32 = 2320;
pub const wxSTC_CMD_PAGEUPEXTEND: u32 = 2321;
pub const wxSTC_CMD_PAGEDOWN: u32 = 2322;
pub const wxSTC_CMD_PAGEDOWNEXTEND: u32 = 2323;
pub const wxSTC_CMD_EDITTOGGLEOVERTYPE: u32 = 2324;
pub const wxSTC_CMD_CANCEL: u32 = 2325;
pub const wxSTC_CMD_DELETEBACK: u32 = 2326;
pub const wxSTC_CMD_TAB: u32 = 2327;
pub const wxSTC_CMD_BACKTAB: u32 = 2328;
pub const wxSTC_CMD_NEWLINE: u32 = 2329;
pub const wxSTC_CMD_FORMFEED: u32 = 2330;
pub const wxSTC_CMD_VCHOME: u32 = 2331;
pub const wxSTC_CMD_VCHOMEEXTEND: u32 = 2332;
pub const wxSTC_CMD_ZOOMIN: u32 = 2333;
pub const wxSTC_CMD_ZOOMOUT: u32 = 2334;
pub const wxSTC_CMD_DELWORDLEFT: u32 = 2335;
pub const wxSTC_CMD_DELWORDRIGHT: u32 = 2336;
pub const wxSTC_CMD_DELWORDRIGHTEND: u32 = 2518;
pub const wxSTC_CMD_LINECUT: u32 = 2337;
pub const wxSTC_CMD_LINEDELETE: u32 = 2338;
pub const wxSTC_CMD_LINETRANSPOSE: u32 = 2339;
pub const wxSTC_CMD_LINEDUPLICATE: u32 = 2404;
pub const wxSTC_CMD_LOWERCASE: u32 = 2340;
pub const wxSTC_CMD_UPPERCASE: u32 = 2341;
pub const wxSTC_CMD_LINESCROLLDOWN: u32 = 2342;
pub const wxSTC_CMD_LINESCROLLUP: u32 = 2343;
pub const wxSTC_CMD_DELETEBACKNOTLINE: u32 = 2344;
pub const wxSTC_CMD_HOMEDISPLAY: u32 = 2345;
pub const wxSTC_CMD_HOMEDISPLAYEXTEND: u32 = 2346;
pub const wxSTC_CMD_LINEENDDISPLAY: u32 = 2347;
pub const wxSTC_CMD_LINEENDDISPLAYEXTEND: u32 = 2348;
pub const wxSTC_CMD_HOMEWRAP: u32 = 2349;
pub const wxSTC_CMD_HOMEWRAPEXTEND: u32 = 2450;
pub const wxSTC_CMD_LINEENDWRAP: u32 = 2451;
pub const wxSTC_CMD_LINEENDWRAPEXTEND: u32 = 2452;
pub const wxSTC_CMD_VCHOMEWRAP: u32 = 2453;
pub const wxSTC_CMD_VCHOMEWRAPEXTEND: u32 = 2454;
pub const wxSTC_CMD_LINECOPY: u32 = 2455;
pub const wxSTC_CMD_WORDPARTLEFT: u32 = 2390;
pub const wxSTC_CMD_WORDPARTLEFTEXTEND: u32 = 2391;
pub const wxSTC_CMD_WORDPARTRIGHT: u32 = 2392;
pub const wxSTC_CMD_WORDPARTRIGHTEXTEND: u32 = 2393;
pub const wxSTC_CMD_DELLINELEFT: u32 = 2395;
pub const wxSTC_CMD_DELLINERIGHT: u32 = 2396;
pub const wxSTC_CMD_PARADOWN: u32 = 2413;
pub const wxSTC_CMD_PARADOWNEXTEND: u32 = 2414;
pub const wxSTC_CMD_PARAUP: u32 = 2415;
pub const wxSTC_CMD_PARAUPEXTEND: u32 = 2416;
pub const wxSTC_CMD_LINEDOWNRECTEXTEND: u32 = 2426;
pub const wxSTC_CMD_LINEUPRECTEXTEND: u32 = 2427;
pub const wxSTC_CMD_CHARLEFTRECTEXTEND: u32 = 2428;
pub const wxSTC_CMD_CHARRIGHTRECTEXTEND: u32 = 2429;
pub const wxSTC_CMD_HOMERECTEXTEND: u32 = 2430;
pub const wxSTC_CMD_VCHOMERECTEXTEND: u32 = 2431;
pub const wxSTC_CMD_LINEENDRECTEXTEND: u32 = 2432;
pub const wxSTC_CMD_PAGEUPRECTEXTEND: u32 = 2433;
pub const wxSTC_CMD_PAGEDOWNRECTEXTEND: u32 = 2434;
pub const wxSTC_CMD_STUTTEREDPAGEUP: u32 = 2435;
pub const wxSTC_CMD_STUTTEREDPAGEUPEXTEND: u32 = 2436;
pub const wxSTC_CMD_STUTTEREDPAGEDOWN: u32 = 2437;
pub const wxSTC_CMD_STUTTEREDPAGEDOWNEXTEND: u32 = 2438;
pub const wxSTC_CMD_WORDLEFTEND: u32 = 2439;
pub const wxSTC_CMD_WORDLEFTENDEXTEND: u32 = 2440;
pub const wxSTC_CMD_WORDRIGHTEND: u32 = 2441;
pub const wxSTC_CMD_WORDRIGHTENDEXTEND: u32 = 2442;
pub const wxSTC_CMD_VERTICALCENTRECARET: u32 = 2619;
pub const wxSTC_CMD_MOVESELECTEDLINESUP: u32 = 2620;
pub const wxSTC_CMD_MOVESELECTEDLINESDOWN: u32 = 2621;
pub const wxSTC_CMD_SCROLLTOSTART: u32 = 2628;
pub const wxSTC_CMD_SCROLLTOEND: u32 = 2629;
pub const wxSTC_CMD_VCHOMEDISPLAY: u32 = 2652;
pub const wxSTC_CMD_VCHOMEDISPLAYEXTEND: u32 = 2653;

//  ENUM: @26
pub const wxID_HTML_PANEL: i32 = wxID_HIGHEST + 10;
pub const wxID_HTML_BACK: i32 = wxID_HIGHEST + 10 + 1;
pub const wxID_HTML_FORWARD: i32 = wxID_HIGHEST + 10 + 2;
pub const wxID_HTML_UPNODE: i32 = wxID_HIGHEST + 10 + 3;
pub const wxID_HTML_UP: i32 = wxID_HIGHEST + 10 + 4;
pub const wxID_HTML_DOWN: i32 = wxID_HIGHEST + 10 + 5;
pub const wxID_HTML_PRINT: i32 = wxID_HIGHEST + 10 + 6;
pub const wxID_HTML_OPENFILE: i32 = wxID_HIGHEST + 10 + 7;
pub const wxID_HTML_OPTIONS: i32 = wxID_HIGHEST + 10 + 8;
pub const wxID_HTML_BOOKMARKSLIST: i32 = wxID_HIGHEST + 10 + 9;
pub const wxID_HTML_BOOKMARKSADD: i32 = wxID_HIGHEST + 10 + 10;
pub const wxID_HTML_BOOKMARKSREMOVE: i32 = wxID_HIGHEST + 10 + 11;
pub const wxID_HTML_TREECTRL: i32 = wxID_HIGHEST + 10 + 12;
pub const wxID_HTML_INDEXPAGE: i32 = wxID_HIGHEST + 10 + 13;
pub const wxID_HTML_INDEXLIST: i32 = wxID_HIGHEST + 10 + 14;
pub const wxID_HTML_INDEXTEXT: i32 = wxID_HIGHEST + 10 + 15;
pub const wxID_HTML_INDEXBUTTON: i32 = wxID_HIGHEST + 10 + 16;
pub const wxID_HTML_INDEXBUTTONALL: i32 = wxID_HIGHEST + 10 + 17;
pub const wxID_HTML_NOTEBOOK: i32 = wxID_HIGHEST + 10 + 18;
pub const wxID_HTML_SEARCHPAGE: i32 = wxID_HIGHEST + 10 + 19;
pub const wxID_HTML_SEARCHTEXT: i32 = wxID_HIGHEST + 10 + 20;
pub const wxID_HTML_SEARCHLIST: i32 = wxID_HIGHEST + 10 + 21;
pub const wxID_HTML_SEARCHBUTTON: i32 = wxID_HIGHEST + 10 + 22;
pub const wxID_HTML_SEARCHCHOICE: i32 = wxID_HIGHEST + 10 + 23;
pub const wxID_HTML_COUNTINFO: i32 = wxID_HIGHEST + 10 + 24;

// NODEF: wxDEFINE_EVENT
// NODEF: wxDECLARE_EVENT
// NODEF: wxDECLARE_EXPORTED_EVENT
// NODEF: wxEVENT_HANDLER_CAST
// NODEF: wx__DECLARE_EVT1
// NODEF: wx__DECLARE_EVT2
// NODEF: wx__DECLARE_EVT0
// NODEF: wxDECLARE_EVENT_TABLE
// NODEF: wxBEGIN_EVENT_TABLE
// NODEF: wxEND_EVENT_TABLE
//  ENUM: wxEventPropagation
pub const wxEVENT_PROPAGATE_NONE: u32 = 0;
//  SKIP: wxEVENT_PROPAGATE_MAX
//  ENUM: wxEventCategory
pub const wxEVT_CATEGORY_UI: u32 = 1;
pub const wxEVT_CATEGORY_USER_INPUT: u32 = 2;
pub const wxEVT_CATEGORY_SOCKET: u32 = 4;
pub const wxEVT_CATEGORY_TIMER: u32 = 8;
pub const wxEVT_CATEGORY_THREAD: u32 = 16;
pub const wxEVT_CATEGORY_ALL: u32 =
        wxEVT_CATEGORY_UI|wxEVT_CATEGORY_USER_INPUT|wxEVT_CATEGORY_SOCKET| 
        wxEVT_CATEGORY_TIMER|wxEVT_CATEGORY_THREAD;
//  ENUM: wxKeyCategoryFlags
pub const WXK_CATEGORY_ARROW: u32 = 0;
pub const WXK_CATEGORY_PAGING: u32 = 0 + 1;
pub const WXK_CATEGORY_JUMP: u32 = 0 + 2;
pub const WXK_CATEGORY_TAB: u32 = 0 + 3;
pub const WXK_CATEGORY_CUT: u32 = 0 + 4;
pub const WXK_CATEGORY_NAVIGATION: u32 = 0 + 5;
//  ENUM: @14
pub const wxJOYSTICK1: u32 = 0;
pub const wxJOYSTICK2: u32 = 0 + 1;
//  ENUM: @15
pub const wxJOY_BUTTON_ANY: i32 = -1;
pub const wxJOY_BUTTON1: i32 = 1;
pub const wxJOY_BUTTON2: i32 = 2;
pub const wxJOY_BUTTON3: i32 = 4;
pub const wxJOY_BUTTON4: i32 = 8;
//  ENUM: wxUpdateUIMode
pub const wxUPDATE_UI_PROCESS_ALL: u32 = 0;
pub const wxUPDATE_UI_PROCESS_SPECIFIED: u32 = 0 + 1;
//  ENUM: wxMouseWheelAxis
pub const wxMOUSE_WHEEL_VERTICAL: u32 = 0;
pub const wxMOUSE_WHEEL_HORIZONTAL: u32 = 0 + 1;
//  ENUM: wxIdleMode
pub const wxIDLE_PROCESS_ALL: u32 = 0;
pub const wxIDLE_PROCESS_SPECIFIED: u32 = 0 + 1;

//  ENUM: wxRibbonBarOption
pub const wxRIBBON_BAR_SHOW_PAGE_LABELS: u32 = 0;
pub const wxRIBBON_BAR_SHOW_PAGE_ICONS: u32 = 0 + 1;
pub const wxRIBBON_BAR_FLOW_HORIZONTAL: u32 = 0 + 2;
pub const wxRIBBON_BAR_FLOW_VERTICAL: u32 = 0 + 3;
pub const wxRIBBON_BAR_SHOW_PANEL_EXT_BUTTONS: u32 = 0 + 4;
pub const wxRIBBON_BAR_SHOW_PANEL_MINIMISE_BUTTONS: u32 = 0 + 5;
pub const wxRIBBON_BAR_ALWAYS_SHOW_TABS: u32 = 0 + 6;
pub const wxRIBBON_BAR_SHOW_TOGGLE_BUTTON: u32 = 0 + 7;
pub const wxRIBBON_BAR_SHOW_HELP_BUTTON: u32 = 0 + 8;
pub const wxRIBBON_BAR_DEFAULT_STYLE: u32 = 0 + 9;
pub const wxRIBBON_BAR_FOLDBAR_STYLE: u32 = 0 + 10;
//  ENUM: wxRibbonDisplayMode
pub const wxRIBBON_BAR_PINNED: u32 = 0;
pub const wxRIBBON_BAR_MINIMIZED: u32 = 0 + 1;
pub const wxRIBBON_BAR_EXPANDED: u32 = 0 + 2;

//  ENUM: wxOperatingSystemId
pub const wxOS_UNKNOWN: u32 = 0;
pub const wxOS_MAC_OS: u32 = 1 << 0;
pub const wxOS_MAC_OSX_DARWIN: u32 = 1 << 1;
pub const wxOS_MAC: u32 = wxOS_MAC_OS|wxOS_MAC_OSX_DARWIN;
pub const wxOS_WINDOWS_NT: u32 = 1 << 3;
pub const wxOS_WINDOWS: u32 = wxOS_WINDOWS_NT;
pub const wxOS_UNIX_LINUX: u32 = 1 << 6;
pub const wxOS_UNIX_FREEBSD: u32 = 1 << 7;
pub const wxOS_UNIX_OPENBSD: u32 = 1 << 8;
pub const wxOS_UNIX_NETBSD: u32 = 1 << 9;
pub const wxOS_UNIX_SOLARIS: u32 = 1 << 10;
pub const wxOS_UNIX_AIX: u32 = 1 << 11;
pub const wxOS_UNIX_HPUX: u32 = 1 << 12;
pub const wxOS_UNIX: u32 = wxOS_UNIX_LINUX     |
                wxOS_UNIX_FREEBSD   |
                wxOS_UNIX_OPENBSD   |
                wxOS_UNIX_NETBSD    |
                wxOS_UNIX_SOLARIS   |
                wxOS_UNIX_AIX       |
                wxOS_UNIX_HPUX;
//  ENUM: wxPortId
pub const wxPORT_UNKNOWN: u32 = 0;
pub const wxPORT_BASE: u32 = 1 << 0;
pub const wxPORT_MSW: u32 = 1 << 1;
pub const wxPORT_MOTIF: u32 = 1 << 2;
pub const wxPORT_GTK: u32 = 1 << 3;
pub const wxPORT_DFB: u32 = 1 << 4;
pub const wxPORT_X11: u32 = 1 << 5;
pub const wxPORT_MAC: u32 = 1 << 7;
pub const wxPORT_COCOA: u32 = 1 << 8;
pub const wxPORT_QT: u32 = 1 << 10;
//  ENUM: wxBitness
pub const wxBITNESS_INVALID: i32 = -1;
pub const wxBITNESS_32: i32 = -1 + 1;
pub const wxBITNESS_64: i32 = -1 + 2;
pub const wxBITNESS_MAX: i32 = -1 + 3;
//  ENUM: wxArchitecture
pub const wxARCH_INVALID: i32 = -1;
pub const wxARCH_32: i32 = -1 + 1;
pub const wxARCH_64: i32 = -1 + 2;
pub const wxARCH_MAX: i32 = -1 + 3;
//  ENUM: wxEndianness
pub const wxENDIAN_INVALID: i32 = -1;
pub const wxENDIAN_BIG: i32 = -1 + 1;
pub const wxENDIAN_LITTLE: i32 = -1 + 2;
pub const wxENDIAN_PDP: i32 = -1 + 3;
pub const wxENDIAN_MAX: i32 = -1 + 4;

// NODEF: wxPLURAL
// NODEF: wxGETTEXT_IN_CONTEXT
// NODEF: wxGETTEXT_IN_CONTEXT_PLURAL
// NODEF: wxTRANSLATE

//  ENUM: wxMouseButton
pub const wxMOUSE_BTN_ANY: i32 = -1;
pub const wxMOUSE_BTN_NONE: i32 = 0;
pub const wxMOUSE_BTN_LEFT: i32 = 1;
pub const wxMOUSE_BTN_MIDDLE: i32 = 2;
pub const wxMOUSE_BTN_RIGHT: i32 = 3;
pub const wxMOUSE_BTN_AUX1: i32 = 4;
pub const wxMOUSE_BTN_AUX2: i32 = 5;
pub const wxMOUSE_BTN_MAX: i32 = 5 + 1;

pub const wxBK_DEFAULT: u32 = 0x0000;
pub const wxBK_TOP: u32 = 0x0010;
pub const wxBK_BOTTOM: u32 = 0x0020;
pub const wxBK_LEFT: u32 = 0x0040;
pub const wxBK_RIGHT: u32 = 0x0080;
pub const wxBK_ALIGN_MASK: u32 = (wxBK_TOP | wxBK_BOTTOM | wxBK_LEFT | wxBK_RIGHT);
//  SKIP: wxBookCtrl
//  ENUM: @2
pub const wxBK_HITTEST_NOWHERE: u32 = 1;
pub const wxBK_HITTEST_ONICON: u32 = 2;
pub const wxBK_HITTEST_ONLABEL: u32 = 4;
pub const wxBK_HITTEST_ONITEM: u32 = 16;
pub const wxBK_HITTEST_ONPAGE: u32 = 8;

//  ENUM: AccessMode
pub const Read: u32 = 0;
pub const Write: u32 = 0 + 1;
//  ENUM: StdKey
pub const HKCR: u32 = 0;
pub const HKCU: u32 = 0 + 1;
pub const HKLM: u32 = 0 + 2;
pub const HKUSR: u32 = 0 + 3;
pub const HKPD: u32 = 0 + 4;
pub const HKCC: u32 = 0 + 5;
pub const HKDD: u32 = 0 + 6;
pub const HKMAX: u32 = 0 + 7;
//  ENUM: ValueType
pub const Type_None: u32 = 0;
pub const Type_String: u32 = 0 + 1;
pub const Type_Expand_String: u32 = 0 + 2;
pub const Type_Binary: u32 = 0 + 3;
pub const Type_Dword: u32 = 0 + 4;
pub const Type_Dword_little_endian: u32 = 0 + 5;
pub const Type_Dword_big_endian: u32 = 0 + 6;
pub const Type_Link: u32 = 0 + 7;
pub const Type_Multi_String: u32 = 0 + 8;
pub const Type_Resource_list: u32 = 0 + 9;
pub const Type_Full_resource_descriptor: u32 = 0 + 10;
pub const Type_Resource_requirements_list: u32 = 0 + 11;
//  ENUM: WOW64ViewMode
pub const WOW64ViewMode_Default: u32 = 0;
pub const WOW64ViewMode_32: u32 = 0 + 1;
pub const WOW64ViewMode_64: u32 = 0 + 2;

pub const wxDEFAULT_FRAME_STYLE: u32 = (wxSYSTEM_MENU |          wxRESIZE_BORDER |        wxMINIMIZE_BOX |         wxMAXIMIZE_BOX |         wxCLOSE_BOX |            wxCAPTION |              wxCLIP_CHILDREN);
//  ENUM: @50
pub const wxUSER_ATTENTION_INFO: u32 = 1;
pub const wxUSER_ATTENTION_ERROR: u32 = 2;
//  ENUM: @51
pub const wxFULLSCREEN_NOMENUBAR: u32 = 0x0001;
pub const wxFULLSCREEN_NOTOOLBAR: u32 = 0x0002;
pub const wxFULLSCREEN_NOSTATUSBAR: u32 = 0x0004;
pub const wxFULLSCREEN_NOBORDER: u32 = 0x0008;
pub const wxFULLSCREEN_NOCAPTION: u32 = 0x0010;
pub const wxFULLSCREEN_ALL: u32 = wxFULLSCREEN_NOMENUBAR | wxFULLSCREEN_NOTOOLBAR |
                          wxFULLSCREEN_NOSTATUSBAR | wxFULLSCREEN_NOBORDER |
                          wxFULLSCREEN_NOCAPTION;

pub const wxEL_ALLOW_NEW: u32 = 0x0100;
pub const wxEL_ALLOW_EDIT: u32 = 0x0200;
pub const wxEL_ALLOW_DELETE: u32 = 0x0400;
pub const wxEL_NO_REORDER: u32 = 0x0800;
pub const wxEL_DEFAULT_STYLE: u32 = (wxEL_ALLOW_NEW | wxEL_ALLOW_EDIT | wxEL_ALLOW_DELETE);

pub const wxFC_DEFAULT_STYLE: u32 = wxFC_OPEN;
//  ENUM: @18
pub const wxFC_OPEN: u32 = 0x0001;
pub const wxFC_SAVE: u32 = 0x0002;
pub const wxFC_MULTIPLE: u32 = 0x0004;
pub const wxFC_NOSHOWHIDDEN: u32 = 0x0008;

pub const wxSL_HORIZONTAL: u32 = wxHORIZONTAL /* 0x0004 */;
pub const wxSL_VERTICAL: u32 = wxVERTICAL   /* 0x0008 */;
pub const wxSL_TICKS: u32 = 0x0010;
pub const wxSL_AUTOTICKS: u32 = wxSL_TICKS;
pub const wxSL_LEFT: u32 = 0x0040;
pub const wxSL_TOP: u32 = 0x0080;
pub const wxSL_RIGHT: u32 = 0x0100;
pub const wxSL_BOTTOM: u32 = 0x0200;
pub const wxSL_BOTH: u32 = 0x0400;
pub const wxSL_SELRANGE: u32 = 0x0800;
pub const wxSL_INVERSE: u32 = 0x1000;
pub const wxSL_MIN_MAX_LABELS: u32 = 0x2000;
pub const wxSL_VALUE_LABEL: u32 = 0x4000;
pub const wxSL_LABELS: u32 = (wxSL_MIN_MAX_LABELS|wxSL_VALUE_LABEL);

//  ENUM: wxRibbonGalleryButtonState
pub const wxRIBBON_GALLERY_BUTTON_NORMAL: u32 = 0;
pub const wxRIBBON_GALLERY_BUTTON_HOVERED: u32 = 0 + 1;
pub const wxRIBBON_GALLERY_BUTTON_ACTIVE: u32 = 0 + 2;
pub const wxRIBBON_GALLERY_BUTTON_DISABLED: u32 = 0 + 3;

//  ENUM: @22
pub const WX_GL_RGBA: u32 = 1;
pub const WX_GL_BUFFER_SIZE: u32 = 1 + 1;
pub const WX_GL_LEVEL: u32 = 1 + 2;
pub const WX_GL_DOUBLEBUFFER: u32 = 1 + 3;
pub const WX_GL_STEREO: u32 = 1 + 4;
pub const WX_GL_AUX_BUFFERS: u32 = 1 + 5;
pub const WX_GL_MIN_RED: u32 = 1 + 6;
pub const WX_GL_MIN_GREEN: u32 = 1 + 7;
pub const WX_GL_MIN_BLUE: u32 = 1 + 8;
pub const WX_GL_MIN_ALPHA: u32 = 1 + 9;
pub const WX_GL_DEPTH_SIZE: u32 = 1 + 10;
pub const WX_GL_STENCIL_SIZE: u32 = 1 + 11;
pub const WX_GL_MIN_ACCUM_RED: u32 = 1 + 12;
pub const WX_GL_MIN_ACCUM_GREEN: u32 = 1 + 13;
pub const WX_GL_MIN_ACCUM_BLUE: u32 = 1 + 14;
pub const WX_GL_MIN_ACCUM_ALPHA: u32 = 1 + 15;
pub const WX_GL_SAMPLE_BUFFERS: u32 = 1 + 16;
pub const WX_GL_SAMPLES: u32 = 1 + 17;
pub const WX_GL_FRAMEBUFFER_SRGB: u32 = 1 + 18;
pub const WX_GL_MAJOR_VERSION: u32 = 1 + 19;
pub const WX_GL_MINOR_VERSION: u32 = 1 + 20;
pub const WX_GL_CORE_PROFILE: u32 = 1 + 21;
pub const wx_GL_COMPAT_PROFILE: u32 = 1 + 22;
pub const WX_GL_FORWARD_COMPAT: u32 = 1 + 23;
pub const WX_GL_ES2: u32 = 1 + 24;
pub const WX_GL_DEBUG: u32 = 1 + 25;
pub const WX_GL_ROBUST_ACCESS: u32 = 1 + 26;
pub const WX_GL_NO_RESET_NOTIFY: u32 = 1 + 27;
pub const WX_GL_LOSE_ON_RESET: u32 = 1 + 28;
pub const WX_GL_RESET_ISOLATION: u32 = 1 + 29;
pub const WX_GL_RELEASE_FLUSH: u32 = 1 + 30;
pub const WX_GL_RELEASE_NONE: u32 = 1 + 31;

pub const wxACC_SELF: u32 = 0;
pub const wxACC_STATE_SYSTEM_ALERT_HIGH: u32 = 0x00000001;
pub const wxACC_STATE_SYSTEM_ALERT_MEDIUM: u32 = 0x00000002;
pub const wxACC_STATE_SYSTEM_ALERT_LOW: u32 = 0x00000004;
pub const wxACC_STATE_SYSTEM_ANIMATED: u32 = 0x00000008;
pub const wxACC_STATE_SYSTEM_BUSY: u32 = 0x00000010;
pub const wxACC_STATE_SYSTEM_CHECKED: u32 = 0x00000020;
pub const wxACC_STATE_SYSTEM_COLLAPSED: u32 = 0x00000040;
pub const wxACC_STATE_SYSTEM_DEFAULT: u32 = 0x00000080;
pub const wxACC_STATE_SYSTEM_EXPANDED: u32 = 0x00000100;
pub const wxACC_STATE_SYSTEM_EXTSELECTABLE: u32 = 0x00000200;
pub const wxACC_STATE_SYSTEM_FLOATING: u32 = 0x00000400;
pub const wxACC_STATE_SYSTEM_FOCUSABLE: u32 = 0x00000800;
pub const wxACC_STATE_SYSTEM_FOCUSED: u32 = 0x00001000;
pub const wxACC_STATE_SYSTEM_HOTTRACKED: u32 = 0x00002000;
pub const wxACC_STATE_SYSTEM_INVISIBLE: u32 = 0x00004000;
pub const wxACC_STATE_SYSTEM_MARQUEED: u32 = 0x00008000;
pub const wxACC_STATE_SYSTEM_MIXED: u32 = 0x00010000;
pub const wxACC_STATE_SYSTEM_MULTISELECTABLE: u32 = 0x00020000;
pub const wxACC_STATE_SYSTEM_OFFSCREEN: u32 = 0x00040000;
pub const wxACC_STATE_SYSTEM_PRESSED: u32 = 0x00080000;
pub const wxACC_STATE_SYSTEM_PROTECTED: u32 = 0x00100000;
pub const wxACC_STATE_SYSTEM_READONLY: u32 = 0x00200000;
pub const wxACC_STATE_SYSTEM_SELECTABLE: u32 = 0x00400000;
pub const wxACC_STATE_SYSTEM_SELECTED: u32 = 0x00800000;
pub const wxACC_STATE_SYSTEM_SELFVOICING: u32 = 0x01000000;
pub const wxACC_STATE_SYSTEM_UNAVAILABLE: u32 = 0x02000000;
pub const wxACC_EVENT_SYSTEM_SOUND: u32 = 0x0001;
pub const wxACC_EVENT_SYSTEM_ALERT: u32 = 0x0002;
pub const wxACC_EVENT_SYSTEM_FOREGROUND: u32 = 0x0003;
pub const wxACC_EVENT_SYSTEM_MENUSTART: u32 = 0x0004;
pub const wxACC_EVENT_SYSTEM_MENUEND: u32 = 0x0005;
pub const wxACC_EVENT_SYSTEM_MENUPOPUPSTART: u32 = 0x0006;
pub const wxACC_EVENT_SYSTEM_MENUPOPUPEND: u32 = 0x0007;
pub const wxACC_EVENT_SYSTEM_CAPTURESTART: u32 = 0x0008;
pub const wxACC_EVENT_SYSTEM_CAPTUREEND: u32 = 0x0009;
pub const wxACC_EVENT_SYSTEM_MOVESIZESTART: u32 = 0x000A;
pub const wxACC_EVENT_SYSTEM_MOVESIZEEND: u32 = 0x000B;
pub const wxACC_EVENT_SYSTEM_CONTEXTHELPSTART: u32 = 0x000C;
pub const wxACC_EVENT_SYSTEM_CONTEXTHELPEND: u32 = 0x000D;
pub const wxACC_EVENT_SYSTEM_DRAGDROPSTART: u32 = 0x000E;
pub const wxACC_EVENT_SYSTEM_DRAGDROPEND: u32 = 0x000F;
pub const wxACC_EVENT_SYSTEM_DIALOGSTART: u32 = 0x0010;
pub const wxACC_EVENT_SYSTEM_DIALOGEND: u32 = 0x0011;
pub const wxACC_EVENT_SYSTEM_SCROLLINGSTART: u32 = 0x0012;
pub const wxACC_EVENT_SYSTEM_SCROLLINGEND: u32 = 0x0013;
pub const wxACC_EVENT_SYSTEM_SWITCHSTART: u32 = 0x0014;
pub const wxACC_EVENT_SYSTEM_SWITCHEND: u32 = 0x0015;
pub const wxACC_EVENT_SYSTEM_MINIMIZESTART: u32 = 0x0016;
pub const wxACC_EVENT_SYSTEM_MINIMIZEEND: u32 = 0x0017;
pub const wxACC_EVENT_OBJECT_CREATE: u32 = 0x8000;
pub const wxACC_EVENT_OBJECT_DESTROY: u32 = 0x8001;
pub const wxACC_EVENT_OBJECT_SHOW: u32 = 0x8002;
pub const wxACC_EVENT_OBJECT_HIDE: u32 = 0x8003;
pub const wxACC_EVENT_OBJECT_REORDER: u32 = 0x8004;
pub const wxACC_EVENT_OBJECT_FOCUS: u32 = 0x8005;
pub const wxACC_EVENT_OBJECT_SELECTION: u32 = 0x8006;
pub const wxACC_EVENT_OBJECT_SELECTIONADD: u32 = 0x8007;
pub const wxACC_EVENT_OBJECT_SELECTIONREMOVE: u32 = 0x8008;
pub const wxACC_EVENT_OBJECT_SELECTIONWITHIN: u32 = 0x8009;
pub const wxACC_EVENT_OBJECT_STATECHANGE: u32 = 0x800A;
pub const wxACC_EVENT_OBJECT_LOCATIONCHANGE: u32 = 0x800B;
pub const wxACC_EVENT_OBJECT_NAMECHANGE: u32 = 0x800C;
pub const wxACC_EVENT_OBJECT_DESCRIPTIONCHANGE: u32 = 0x800D;
pub const wxACC_EVENT_OBJECT_VALUECHANGE: u32 = 0x800E;
pub const wxACC_EVENT_OBJECT_PARENTCHANGE: u32 = 0x800F;
pub const wxACC_EVENT_OBJECT_HELPCHANGE: u32 = 0x8010;
pub const wxACC_EVENT_OBJECT_DEFACTIONCHANGE: u32 = 0x8011;
pub const wxACC_EVENT_OBJECT_ACCELERATORCHANGE: u32 = 0x8012;
//  ENUM: wxAccStatus
pub const wxACC_FAIL: u32 = 0;
pub const wxACC_FALSE: u32 = 0 + 1;
pub const wxACC_OK: u32 = 0 + 2;
pub const wxACC_NOT_IMPLEMENTED: u32 = 0 + 3;
pub const wxACC_NOT_SUPPORTED: u32 = 0 + 4;
pub const wxACC_INVALID_ARG: u32 = 0 + 5;
//  ENUM: wxNavDir
pub const wxNAVDIR_FIRSTCHILD: u32 = 0;
pub const wxNAVDIR_LASTCHILD: u32 = 0 + 1;
pub const wxNAVDIR_DOWN: u32 = 0 + 2;
pub const wxNAVDIR_LEFT: u32 = 0 + 3;
pub const wxNAVDIR_NEXT: u32 = 0 + 4;
pub const wxNAVDIR_PREVIOUS: u32 = 0 + 5;
pub const wxNAVDIR_RIGHT: u32 = 0 + 6;
pub const wxNAVDIR_UP: u32 = 0 + 7;
//  ENUM: wxAccRole
pub const wxROLE_NONE: u32 = 0;
pub const wxROLE_SYSTEM_ALERT: u32 = 0 + 1;
pub const wxROLE_SYSTEM_ANIMATION: u32 = 0 + 2;
pub const wxROLE_SYSTEM_APPLICATION: u32 = 0 + 3;
pub const wxROLE_SYSTEM_BORDER: u32 = 0 + 4;
pub const wxROLE_SYSTEM_BUTTONDROPDOWN: u32 = 0 + 5;
pub const wxROLE_SYSTEM_BUTTONDROPDOWNGRID: u32 = 0 + 6;
pub const wxROLE_SYSTEM_BUTTONMENU: u32 = 0 + 7;
pub const wxROLE_SYSTEM_CARET: u32 = 0 + 8;
pub const wxROLE_SYSTEM_CELL: u32 = 0 + 9;
pub const wxROLE_SYSTEM_CHARACTER: u32 = 0 + 10;
pub const wxROLE_SYSTEM_CHART: u32 = 0 + 11;
pub const wxROLE_SYSTEM_CHECKBUTTON: u32 = 0 + 12;
pub const wxROLE_SYSTEM_CLIENT: u32 = 0 + 13;
pub const wxROLE_SYSTEM_CLOCK: u32 = 0 + 14;
pub const wxROLE_SYSTEM_COLUMN: u32 = 0 + 15;
pub const wxROLE_SYSTEM_COLUMNHEADER: u32 = 0 + 16;
pub const wxROLE_SYSTEM_COMBOBOX: u32 = 0 + 17;
pub const wxROLE_SYSTEM_CURSOR: u32 = 0 + 18;
pub const wxROLE_SYSTEM_DIAGRAM: u32 = 0 + 19;
pub const wxROLE_SYSTEM_DIAL: u32 = 0 + 20;
pub const wxROLE_SYSTEM_DIALOG: u32 = 0 + 21;
pub const wxROLE_SYSTEM_DOCUMENT: u32 = 0 + 22;
pub const wxROLE_SYSTEM_DROPLIST: u32 = 0 + 23;
pub const wxROLE_SYSTEM_EQUATION: u32 = 0 + 24;
pub const wxROLE_SYSTEM_GRAPHIC: u32 = 0 + 25;
pub const wxROLE_SYSTEM_GRIP: u32 = 0 + 26;
pub const wxROLE_SYSTEM_GROUPING: u32 = 0 + 27;
pub const wxROLE_SYSTEM_HELPBALLOON: u32 = 0 + 28;
pub const wxROLE_SYSTEM_HOTKEYFIELD: u32 = 0 + 29;
pub const wxROLE_SYSTEM_INDICATOR: u32 = 0 + 30;
pub const wxROLE_SYSTEM_LINK: u32 = 0 + 31;
pub const wxROLE_SYSTEM_LIST: u32 = 0 + 32;
pub const wxROLE_SYSTEM_LISTITEM: u32 = 0 + 33;
pub const wxROLE_SYSTEM_MENUBAR: u32 = 0 + 34;
pub const wxROLE_SYSTEM_MENUITEM: u32 = 0 + 35;
pub const wxROLE_SYSTEM_MENUPOPUP: u32 = 0 + 36;
pub const wxROLE_SYSTEM_OUTLINE: u32 = 0 + 37;
pub const wxROLE_SYSTEM_OUTLINEITEM: u32 = 0 + 38;
pub const wxROLE_SYSTEM_PAGETAB: u32 = 0 + 39;
pub const wxROLE_SYSTEM_PAGETABLIST: u32 = 0 + 40;
pub const wxROLE_SYSTEM_PANE: u32 = 0 + 41;
pub const wxROLE_SYSTEM_PROGRESSBAR: u32 = 0 + 42;
pub const wxROLE_SYSTEM_PROPERTYPAGE: u32 = 0 + 43;
pub const wxROLE_SYSTEM_PUSHBUTTON: u32 = 0 + 44;
pub const wxROLE_SYSTEM_RADIOBUTTON: u32 = 0 + 45;
pub const wxROLE_SYSTEM_ROW: u32 = 0 + 46;
pub const wxROLE_SYSTEM_ROWHEADER: u32 = 0 + 47;
pub const wxROLE_SYSTEM_SCROLLBAR: u32 = 0 + 48;
pub const wxROLE_SYSTEM_SEPARATOR: u32 = 0 + 49;
pub const wxROLE_SYSTEM_SLIDER: u32 = 0 + 50;
pub const wxROLE_SYSTEM_SOUND: u32 = 0 + 51;
pub const wxROLE_SYSTEM_SPINBUTTON: u32 = 0 + 52;
pub const wxROLE_SYSTEM_STATICTEXT: u32 = 0 + 53;
pub const wxROLE_SYSTEM_STATUSBAR: u32 = 0 + 54;
pub const wxROLE_SYSTEM_TABLE: u32 = 0 + 55;
pub const wxROLE_SYSTEM_TEXT: u32 = 0 + 56;
pub const wxROLE_SYSTEM_TITLEBAR: u32 = 0 + 57;
pub const wxROLE_SYSTEM_TOOLBAR: u32 = 0 + 58;
pub const wxROLE_SYSTEM_TOOLTIP: u32 = 0 + 59;
pub const wxROLE_SYSTEM_WHITESPACE: u32 = 0 + 60;
pub const wxROLE_SYSTEM_WINDOW: u32 = 0 + 61;
//  ENUM: wxAccObject
pub const wxOBJID_WINDOW: u32 =    0x00000000;
pub const wxOBJID_SYSMENU: u32 =   0xFFFFFFFF;
pub const wxOBJID_TITLEBAR: u32 =  0xFFFFFFFE;
pub const wxOBJID_MENU: u32 =      0xFFFFFFFD;
pub const wxOBJID_CLIENT: u32 =    0xFFFFFFFC;
pub const wxOBJID_VSCROLL: u32 =   0xFFFFFFFB;
pub const wxOBJID_HSCROLL: u32 =   0xFFFFFFFA;
pub const wxOBJID_SIZEGRIP: u32 =  0xFFFFFFF9;
pub const wxOBJID_CARET: u32 =     0xFFFFFFF8;
pub const wxOBJID_CURSOR: u32 =    0xFFFFFFF7;
pub const wxOBJID_ALERT: u32 =     0xFFFFFFF6;
pub const wxOBJID_SOUND: u32 =     0xFFFFFFF5;
//  ENUM: wxAccSelectionFlags
pub const wxACC_SEL_NONE: u32 = 0;
pub const wxACC_SEL_TAKEFOCUS: u32 = 1;
pub const wxACC_SEL_TAKESELECTION: u32 = 2;
pub const wxACC_SEL_EXTENDSELECTION: u32 = 4;
pub const wxACC_SEL_ADDSELECTION: u32 = 8;
pub const wxACC_SEL_REMOVESELECTION: u32 = 16;

//  ENUM: ResourceCat
pub const ResourceCat_None: u32 = 0;
pub const ResourceCat_Messages: u32 = 0 + 1;
//  ENUM: Dir
pub const Dir_Cache: u32 = 0;
pub const Dir_Documents: u32 = 0 + 1;
pub const Dir_Desktop: u32 = 0 + 2;
pub const Dir_Downloads: u32 = 0 + 3;
pub const Dir_Music: u32 = 0 + 4;
pub const Dir_Pictures: u32 = 0 + 5;
pub const Dir_Videos: u32 = 0 + 6;
//  ENUM: FileLayout
pub const FileLayout_Classic: u32 = 0;
pub const FileLayout_XDG: u32 = 0 + 1;
//  ENUM: ConfigFileConv
pub const ConfigFileConv_Dot: u32 = 0;
pub const ConfigFileConv_Ext: u32 = 0 + 1;

// NODEF: wxLongLongFmtSpec

pub const wxCLRP_USE_TEXTCTRL: u32 = (wxPB_USE_TEXTCTRL);
pub const wxCLRP_DEFAULT_STYLE: u32 = 0;
pub const wxCLRP_SHOW_LABEL: u32 = 0x0008;
pub const wxCLRP_SHOW_ALPHA: u32 = 0x0010;

//  ENUM: wxHtmlURLType
pub const wxHTML_URL_PAGE: u32 = 0;
pub const wxHTML_URL_IMAGE: u32 = 0 + 1;
pub const wxHTML_URL_OTHER: u32 = 0 + 2;

//  ENUM: NumericType
pub const Signed: u32 = 0;
pub const Unsigned: u32 = 0 + 1;
pub const Float: u32 = 0 + 2;

//  ENUM: wxToolBarToolStyle
pub const wxTOOL_STYLE_BUTTON: u32 = 1;
pub const wxTOOL_STYLE_SEPARATOR: u32 = 2;
pub const wxTOOL_STYLE_CONTROL: u32 = 2 + 1;
//  ENUM: @49
pub const wxTB_HORIZONTAL: u32 = wxHORIZONTAL;
pub const wxTB_TOP: u32 = wxTB_HORIZONTAL;
pub const wxTB_VERTICAL: u32 = wxVERTICAL;
pub const wxTB_LEFT: u32 = wxTB_VERTICAL;
pub const wxTB_FLAT: u32 = wxTB_VERTICAL + 1;
pub const wxTB_DOCKABLE: u32 = wxTB_VERTICAL + 2;
pub const wxTB_NOICONS: u32 = wxTB_VERTICAL + 3;
pub const wxTB_TEXT: u32 = wxTB_VERTICAL + 4;
pub const wxTB_NODIVIDER: u32 = wxTB_VERTICAL + 5;
pub const wxTB_NOALIGN: u32 = wxTB_VERTICAL + 6;
pub const wxTB_HORZ_LAYOUT: u32 = wxTB_VERTICAL + 7;
pub const wxTB_HORZ_TEXT: u32 = wxTB_HORZ_LAYOUT | wxTB_TEXT;
pub const wxTB_NO_TOOLTIPS: u32 = wxTB_HORZ_LAYOUT | wxTB_TEXT + 1;
pub const wxTB_BOTTOM: u32 = wxTB_HORZ_LAYOUT | wxTB_TEXT + 2;
pub const wxTB_RIGHT: u32 = wxTB_HORZ_LAYOUT | wxTB_TEXT + 3;
pub const wxTB_DEFAULT_STYLE: u32 = wxTB_HORIZONTAL;

//  ENUM: wxAuiNotebookOption
pub const wxAUI_NB_TOP: u32 = 1 << 0;
pub const wxAUI_NB_LEFT: u32 = 1 << 1;
pub const wxAUI_NB_RIGHT: u32 = 1 << 2;
pub const wxAUI_NB_BOTTOM: u32 = 1 << 3;
pub const wxAUI_NB_TAB_SPLIT: u32 = 1 << 4;
pub const wxAUI_NB_TAB_MOVE: u32 = 1 << 5;
pub const wxAUI_NB_TAB_EXTERNAL_MOVE: u32 = 1 << 6;
pub const wxAUI_NB_TAB_FIXED_WIDTH: u32 = 1 << 7;
pub const wxAUI_NB_SCROLL_BUTTONS: u32 = 1 << 8;
pub const wxAUI_NB_WINDOWLIST_BUTTON: u32 = 1 << 9;
pub const wxAUI_NB_CLOSE_BUTTON: u32 = 1 << 10;
pub const wxAUI_NB_CLOSE_ON_ACTIVE_TAB: u32 = 1 << 11;
pub const wxAUI_NB_CLOSE_ON_ALL_TABS: u32 = 1 << 12;
pub const wxAUI_NB_MIDDLE_CLICK_CLOSE: u32 = 1 << 13;
pub const wxAUI_NB_DEFAULT_STYLE: u32 = wxAUI_NB_TOP |
                             wxAUI_NB_TAB_SPLIT |
                             wxAUI_NB_TAB_MOVE |
                             wxAUI_NB_SCROLL_BUTTONS |
                             wxAUI_NB_CLOSE_ON_ACTIVE_TAB |
                             wxAUI_NB_MIDDLE_CLICK_CLOSE;

// NODEF: wxCRIT_SECT_DECLARE
// NODEF: wxCRIT_SECT_DECLARE_MEMBER
// NODEF: wxCRIT_SECT_LOCKER
// NODEF: wxCRITICAL_SECTION
// NODEF: wxLEAVE_CRIT_SECT
// NODEF: wxENTER_CRIT_SECT
//  ENUM: wxCondError
pub const wxCOND_NO_ERROR: u32 = 0;
pub const wxCOND_INVALID: u32 = 0 + 1;
pub const wxCOND_TIMEOUT: u32 = 0 + 2;
pub const wxCOND_MISC_ERROR: u32 = 0 + 3;
//  ENUM: wxCriticalSectionType
pub const wxCRITSEC_DEFAULT: u32 = 0;
pub const wxCRITSEC_NON_RECURSIVE: u32 = 0 + 1;
//  ENUM: wxThreadWait
pub const wxTHREAD_WAIT_BLOCK: u32 = 0;
pub const wxTHREAD_WAIT_YIELD: u32 = 0 + 1;
pub const wxTHREAD_WAIT_DEFAULT: u32 = wxTHREAD_WAIT_YIELD;
//  ENUM: wxThreadKind
pub const wxTHREAD_DETACHED: u32 = 0;
pub const wxTHREAD_JOINABLE: u32 = 0 + 1;
//  ENUM: wxThreadError
pub const wxTHREAD_NO_ERROR: u32 = 0;
pub const wxTHREAD_NO_RESOURCE: u32 = 0 + 1;
pub const wxTHREAD_RUNNING: u32 = 0 + 2;
pub const wxTHREAD_NOT_RUNNING: u32 = 0 + 3;
pub const wxTHREAD_KILLED: u32 = 0 + 4;
pub const wxTHREAD_MISC_ERROR: u32 = 0 + 5;
//  ENUM: wxSemaError
pub const wxSEMA_NO_ERROR: u32 = 0;
pub const wxSEMA_INVALID: u32 = 0 + 1;
pub const wxSEMA_BUSY: u32 = 0 + 2;
pub const wxSEMA_TIMEOUT: u32 = 0 + 3;
pub const wxSEMA_OVERFLOW: u32 = 0 + 4;
pub const wxSEMA_MISC_ERROR: u32 = 0 + 5;
//  ENUM: wxMutexType
pub const wxMUTEX_DEFAULT: u32 = 0;
pub const wxMUTEX_RECURSIVE: u32 = 0 + 1;
//  ENUM: wxMutexError
pub const wxMUTEX_NO_ERROR: u32 = 0;
pub const wxMUTEX_INVALID: u32 = 0 + 1;
pub const wxMUTEX_DEAD_LOCK: u32 = 0 + 2;
pub const wxMUTEX_BUSY: u32 = 0 + 3;
pub const wxMUTEX_UNLOCKED: u32 = 0 + 4;
pub const wxMUTEX_TIMEOUT: u32 = 0 + 5;
pub const wxMUTEX_MISC_ERROR: u32 = 0 + 6;

//  ENUM: wxMessageOutputFlags
pub const wxMSGOUT_PREFER_STDERR: u32 = 0;
pub const wxMSGOUT_PREFER_MSGBOX: u32 = 1;

pub const wxSTACKWALKER_MAX_DEPTH: u32 = (200);

pub const wxRE_READONLY: u32 = 0x0010;
pub const wxRE_MULTILINE: u32 = 0x0020;
pub const wxRE_CENTRE_CARET: u32 = 0x8000;
pub const wxRE_CENTER_CARET: u32 = wxRE_CENTRE_CARET;
pub const wxRICHTEXT_SHIFT_DOWN: u32 = 0x01;
pub const wxRICHTEXT_CTRL_DOWN: u32 = 0x02;
pub const wxRICHTEXT_ALT_DOWN: u32 = 0x04;
pub const wxRICHTEXT_EX_NO_GUIDELINES: u32 = 0x00000100;
//  SKIP: wxRICHTEXT_DEFAULT_OVERALL_SIZE
//  SKIP: wxRICHTEXT_DEFAULT_IMAGE_SIZE
pub const wxRICHTEXT_DEFAULT_SPACING: u32 = 3;
pub const wxRICHTEXT_DEFAULT_MARGIN: u32 = 3;
//  SKIP: wxRICHTEXT_DEFAULT_UNFOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_FOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_UNSELECTED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_TYPE_COLOUR
//  SKIP: wxRICHTEXT_DEFAULT_FOCUS_RECT_COLOUR
pub const wxRICHTEXT_DEFAULT_CARET_WIDTH: u32 = 2;
pub const wxRICHTEXT_DEFAULT_DELAYED_LAYOUT_THRESHOLD: u32 = 20000;
pub const wxRICHTEXT_DEFAULT_LAYOUT_INTERVAL: u32 = 50;
pub const wxRICHTEXT_DEFAULT_DELAYED_IMAGE_PROCESSING_INTERVAL: u32 = 200;
pub const wxID_RICHTEXT_PROPERTIES1: i32 = (wxID_HIGHEST + 1);
pub const wxID_RICHTEXT_PROPERTIES2: i32 = (wxID_HIGHEST + 2);
pub const wxID_RICHTEXT_PROPERTIES3: i32 = (wxID_HIGHEST + 3);
//  ENUM: wxRichTextCtrlSelectionState
pub const wxRichTextCtrlSelectionState_Normal: u32 = 0;
pub const wxRichTextCtrlSelectionState_CommonAncestor: u32 = 0 + 1;

pub const wxSIZE_AUTO_WIDTH: u32 = 0x0001;
pub const wxSIZE_AUTO_HEIGHT: u32 = 0x0002;
pub const wxSIZE_AUTO: u32 = (wxSIZE_AUTO_WIDTH|wxSIZE_AUTO_HEIGHT);
pub const wxSIZE_USE_EXISTING: u32 = 0x0000;
pub const wxSIZE_ALLOW_MINUS_ONE: u32 = 0x0004;
pub const wxSIZE_NO_ADJUSTMENTS: u32 = 0x0008;
pub const wxSIZE_FORCE: u32 = 0x0010;
pub const wxSIZE_FORCE_EVENT: u32 = 0x0020;
pub const wxVSCROLL: u32 = 0x80000000;
pub const wxHSCROLL: u32 = 0x40000000;
pub const wxCAPTION: u32 = 0x20000000;
pub const wxDOUBLE_BORDER: u32 = wxBORDER_DOUBLE;
pub const wxSUNKEN_BORDER: u32 = wxBORDER_SUNKEN;
pub const wxRAISED_BORDER: u32 = wxBORDER_RAISED;
pub const wxBORDER: u32 = wxBORDER_SIMPLE;
pub const wxSIMPLE_BORDER: u32 = wxBORDER_SIMPLE;
pub const wxSTATIC_BORDER: u32 = wxBORDER_STATIC;
pub const wxNO_BORDER: u32 = wxBORDER_NONE;
pub const wxALWAYS_SHOW_SB: u32 = 0x00800000;
pub const wxCLIP_CHILDREN: u32 = 0x00400000;
pub const wxCLIP_SIBLINGS: u32 = 0x20000000;
pub const wxTRANSPARENT_WINDOW: u32 = 0x00100000;
pub const wxTAB_TRAVERSAL: u32 = 0x00080000;
pub const wxWANTS_CHARS: u32 = 0x00040000;
pub const wxRETAINED: u32 = 0x00000000;
pub const wxBACKINGSTORE: u32 = wxRETAINED;
pub const wxPOPUP_WINDOW: u32 = 0x00020000;
pub const wxFULL_REPAINT_ON_RESIZE: u32 = 0x00010000;
pub const wxNO_FULL_REPAINT_ON_RESIZE: u32 = 0;
pub const wxWINDOW_STYLE_MASK: u32 = (wxVSCROLL|wxHSCROLL|wxBORDER_MASK|wxALWAYS_SHOW_SB|wxCLIP_CHILDREN| wxCLIP_SIBLINGS|wxTRANSPARENT_WINDOW|wxTAB_TRAVERSAL|wxWANTS_CHARS| wxRETAINED|wxPOPUP_WINDOW|wxFULL_REPAINT_ON_RESIZE);
pub const wxWS_EX_BLOCK_EVENTS: u32 = 0x00000002;
pub const wxWS_EX_TRANSIENT: u32 = 0x00000004;
pub const wxWS_EX_THEMED_BACKGROUND: u32 = 0x00000008;
pub const wxWS_EX_PROCESS_IDLE: u32 = 0x00000010;
pub const wxWS_EX_PROCESS_UI_UPDATES: u32 = 0x00000020;
pub const wxFRAME_EX_METAL: u32 = 0x00000040;
pub const wxDIALOG_EX_METAL: u32 = 0x00000040;
pub const wxWS_EX_CONTEXTHELP: u32 = 0x00000080;
pub const wxFRAME_EX_CONTEXTHELP: u32 = wxWS_EX_CONTEXTHELP;
pub const wxDIALOG_EX_CONTEXTHELP: u32 = wxWS_EX_CONTEXTHELP;
pub const wxFRAME_DRAWER: u32 = 0x0020;
pub const wxFRAME_NO_WINDOW_MENU: u32 = 0x0100;
pub const wxMB_DOCKABLE: u32 = 0x0001;
pub const wxMENU_TEAROFF: u32 = 0x0001;
pub const wxCOLOURED: u32 = 0x0800;
pub const wxFIXED_LENGTH: u32 = 0x0400;
pub const wxLB_SORT: u32 = 0x0010;
pub const wxLB_SINGLE: u32 = 0x0020;
pub const wxLB_MULTIPLE: u32 = 0x0040;
pub const wxLB_EXTENDED: u32 = 0x0080;
pub const wxLB_NEEDED_SB: u32 = 0x0000;
pub const wxLB_OWNERDRAW: u32 = 0x0100;
pub const wxLB_ALWAYS_SB: u32 = 0x0200;
pub const wxLB_NO_SB: u32 = 0x0400;
pub const wxLB_HSCROLL: u32 = wxHSCROLL;
pub const wxLB_INT_HEIGHT: u32 = 0x0800;
pub const wxCB_SIMPLE: u32 = 0x0004;
pub const wxCB_SORT: u32 = 0x0008;
pub const wxCB_READONLY: u32 = 0x0010;
pub const wxCB_DROPDOWN: u32 = 0x0020;
pub const wxRA_LEFTTORIGHT: u32 = 0x0001;
pub const wxRA_TOPTOBOTTOM: u32 = 0x0002;
pub const wxRA_SPECIFY_COLS: u32 = wxHORIZONTAL;
pub const wxRA_SPECIFY_ROWS: u32 = wxVERTICAL;
pub const wxRA_HORIZONTAL: u32 = wxHORIZONTAL;
pub const wxRA_VERTICAL: u32 = wxVERTICAL;
pub const wxRB_GROUP: u32 = 0x0004;
pub const wxRB_SINGLE: u32 = 0x0008;
pub const wxSB_HORIZONTAL: u32 = wxHORIZONTAL;
pub const wxSB_VERTICAL: u32 = wxVERTICAL;
pub const wxSP_HORIZONTAL: u32 = wxHORIZONTAL /*  4 */;
pub const wxSP_VERTICAL: u32 = wxVERTICAL   /*  8 */;
pub const wxSP_ARROW_KEYS: u32 = 0x4000;
pub const wxSP_WRAP: u32 = 0x8000;
pub const wxTC_RIGHTJUSTIFY: u32 = 0x0010;
pub const wxTC_FIXEDWIDTH: u32 = 0x0020;
pub const wxTC_TOP: u32 = 0x0000    /*  default */;
pub const wxTC_LEFT: u32 = 0x0020;
pub const wxTC_RIGHT: u32 = 0x0040;
pub const wxTC_BOTTOM: u32 = 0x0080;
pub const wxTC_MULTILINE: u32 = 0x0200    /* == wxNB_MULTILINE */;
pub const wxTC_OWNERDRAW: u32 = 0x0400;
pub const wxBI_EXPAND: u32 = wxEXPAND;
pub const wxLI_HORIZONTAL: u32 = wxHORIZONTAL;
pub const wxLI_VERTICAL: u32 = wxVERTICAL;
pub const wxYES: u32 = 0x00000002;
pub const wxOK: u32 = 0x00000004;
pub const wxNO: u32 = 0x00000008;
pub const wxYES_NO: u32 = (wxYES | wxNO);
pub const wxCANCEL: u32 = 0x00000010;
pub const wxAPPLY: u32 = 0x00000020;
pub const wxCLOSE: u32 = 0x00000040;
pub const wxOK_DEFAULT: u32 = 0x00000000  /* has no effect (default) */;
pub const wxYES_DEFAULT: u32 = 0x00000000  /* has no effect (default) */;
pub const wxNO_DEFAULT: u32 = 0x00000080  /* only valid with wxYES_NO */;
pub const wxCANCEL_DEFAULT: u32 = 0x80000000  /* only valid with wxCANCEL */;
pub const wxICON_EXCLAMATION: u32 = 0x00000100;
pub const wxICON_HAND: u32 = 0x00000200;
pub const wxICON_WARNING: u32 = wxICON_EXCLAMATION;
pub const wxICON_ERROR: u32 = wxICON_HAND;
pub const wxICON_QUESTION: u32 = 0x00000400;
pub const wxICON_INFORMATION: u32 = 0x00000800;
pub const wxICON_STOP: u32 = wxICON_HAND;
pub const wxICON_ASTERISK: u32 = wxICON_INFORMATION;
pub const wxHELP: u32 = 0x00001000;
pub const wxFORWARD: u32 = 0x00002000;
pub const wxBACKWARD: u32 = 0x00004000;
pub const wxRESET: u32 = 0x00008000;
pub const wxMORE: u32 = 0x00010000;
pub const wxSETUP: u32 = 0x00020000;
pub const wxICON_NONE: u32 = 0x00040000;
pub const wxICON_AUTH_NEEDED: u32 = 0x00080000;
pub const wxICON_MASK: u32 = (wxICON_EXCLAMATION|wxICON_HAND|wxICON_QUESTION|wxICON_INFORMATION|wxICON_NONE);
pub const wxNOT_FOUND: i32 = (-1);
pub const wxPRINT_QUALITY_HIGH: i32 = -1;
pub const wxPRINT_QUALITY_MEDIUM: i32 = -2;
pub const wxPRINT_QUALITY_LOW: i32 = -3;
pub const wxPRINT_QUALITY_DRAFT: i32 = -4;
pub const wxSTAY_ON_TOP: u32 = 0x8000;
pub const wxICONIZE: u32 = 0x4000;
pub const wxMINIMIZE: u32 = wxICONIZE;
pub const wxMAXIMIZE: u32 = 0x2000;
pub const wxCLOSE_BOX: u32 = 0x1000;
pub const wxSYSTEM_MENU: u32 = 0x0800;
pub const wxMINIMIZE_BOX: u32 = 0x0400;
pub const wxMAXIMIZE_BOX: u32 = 0x0200;
pub const wxTINY_CAPTION: u32 = 0x0080;
pub const wxRESIZE_BORDER: u32 = 0x0040;
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
// NODEF: wxDEPRECATED_MSG
// NODEF: wxDEPRECATED
// NODEF: wxDEPRECATED_BUT_USED_INTERNALLY
// NODEF: wxDEPRECATED_INLINE
// NODEF: wxDEPRECATED_ACCESSOR
// NODEF: wxDEPRECATED_BUT_USED_INTERNALLY_INLINE
// NODEF: wxOVERRIDE
// NODEF: wxSUPPRESS_GCC_PRIVATE_DTOR_WARNING
//  ENUM: wxGeometryCentre
pub const wxCENTRE: u32 = 0x0001;
pub const wxCENTER: u32 = wxCENTRE;
//  ENUM: wxOrientation
pub const wxHORIZONTAL: u32 = 0x0004;
pub const wxVERTICAL: u32 = 0x0008;
pub const wxBOTH: u32 = wxVERTICAL | wxHORIZONTAL;
pub const wxORIENTATION_MASK: u32 = wxBOTH;
//  ENUM: wxDirection
pub const wxLEFT: u32 = 0x0010;
pub const wxRIGHT: u32 = 0x0020;
pub const wxUP: u32 = 0x0040;
pub const wxDOWN: u32 = 0x0080;
pub const wxTOP: u32 = wxUP;
pub const wxBOTTOM: u32 = wxDOWN;
pub const wxNORTH: u32 = wxUP;
pub const wxSOUTH: u32 = wxDOWN;
pub const wxWEST: u32 = wxLEFT;
pub const wxEAST: u32 = wxRIGHT;
pub const wxALL: u32 = (wxUP | wxDOWN | wxRIGHT | wxLEFT);
pub const wxDIRECTION_MASK: u32 = wxALL;
//  ENUM: wxAlignment
pub const wxALIGN_INVALID: i32 = -1;
pub const wxALIGN_NOT: i32 = 0x0000;
pub const wxALIGN_CENTER_HORIZONTAL: i32 = 0x0100;
pub const wxALIGN_CENTRE_HORIZONTAL: i32 = wxALIGN_CENTER_HORIZONTAL;
pub const wxALIGN_LEFT: i32 = wxALIGN_NOT;
pub const wxALIGN_TOP: i32 = wxALIGN_NOT;
pub const wxALIGN_RIGHT: i32 = 0x0200;
pub const wxALIGN_BOTTOM: i32 = 0x0400;
pub const wxALIGN_CENTER_VERTICAL: i32 = 0x0800;
pub const wxALIGN_CENTRE_VERTICAL: i32 = wxALIGN_CENTER_VERTICAL;
pub const wxALIGN_CENTER: i32 = (wxALIGN_CENTER_HORIZONTAL | wxALIGN_CENTER_VERTICAL);
pub const wxALIGN_CENTRE: i32 = wxALIGN_CENTER;
pub const wxALIGN_MASK: i32 = 0x0f00;
//  ENUM: wxSizerFlagBits
pub const wxFIXED_MINSIZE: u32 = 0x8000;
pub const wxRESERVE_SPACE_EVEN_IF_HIDDEN: u32 = 0x0002;
pub const wxSIZER_FLAG_BITS_MASK: u32 = 0x8002;
//  ENUM: wxStretch
pub const wxSTRETCH_NOT: u32 = 0x0000;
pub const wxSHRINK: u32 = 0x1000;
pub const wxGROW: u32 = 0x2000;
pub const wxEXPAND: u32 = wxGROW;
pub const wxSHAPED: u32 = 0x4000;
pub const wxTILE: u32 = wxSHAPED | wxFIXED_MINSIZE;
pub const wxSTRETCH_MASK: u32 = 0x7000;
//  ENUM: wxBorder
pub const wxBORDER_DEFAULT: u32 = 0;
pub const wxBORDER_NONE: u32 = 0x00200000;
pub const wxBORDER_STATIC: u32 = 0x01000000;
pub const wxBORDER_SIMPLE: u32 = 0x02000000;
pub const wxBORDER_RAISED: u32 = 0x04000000;
pub const wxBORDER_SUNKEN: u32 = 0x08000000;
pub const wxBORDER_DOUBLE: u32 = 0x10000000;
pub const wxBORDER_THEME: u32 = wxBORDER_DOUBLE;
pub const wxBORDER_MASK: u32 = 0x1f200000;
//  ENUM: wxBackgroundStyle
pub const wxBG_STYLE_ERASE: u32 = 0;
pub const wxBG_STYLE_SYSTEM: u32 = 0 + 1;
pub const wxBG_STYLE_PAINT: u32 = 0 + 2;
pub const wxBG_STYLE_COLOUR: u32 = 0 + 3;
pub const wxBG_STYLE_TRANSPARENT: u32 = 0 + 4;
//  ENUM: wxStandardID
pub const wxID_AUTO_LOWEST: i32 = 0;
pub const wxID_AUTO_HIGHEST: i32 = 0 + 1;
pub const wxID_NONE: i32 = -3;
pub const wxID_SEPARATOR: i32 = -2;
pub const wxID_ANY: i32 = -1;
pub const wxID_LOWEST: i32 = 4999;
pub const wxID_OPEN: i32 = 4999 + 1;
pub const wxID_CLOSE: i32 = 4999 + 2;
pub const wxID_NEW: i32 = 4999 + 3;
pub const wxID_SAVE: i32 = 4999 + 4;
pub const wxID_SAVEAS: i32 = 4999 + 5;
pub const wxID_REVERT: i32 = 4999 + 6;
pub const wxID_EXIT: i32 = 4999 + 7;
pub const wxID_UNDO: i32 = 4999 + 8;
pub const wxID_REDO: i32 = 4999 + 9;
pub const wxID_HELP: i32 = 4999 + 10;
pub const wxID_PRINT: i32 = 4999 + 11;
pub const wxID_PRINT_SETUP: i32 = 4999 + 12;
pub const wxID_PAGE_SETUP: i32 = 4999 + 13;
pub const wxID_PREVIEW: i32 = 4999 + 14;
pub const wxID_ABOUT: i32 = 4999 + 15;
pub const wxID_HELP_CONTENTS: i32 = 4999 + 16;
pub const wxID_HELP_INDEX: i32 = 4999 + 17;
pub const wxID_HELP_SEARCH: i32 = 4999 + 18;
pub const wxID_HELP_COMMANDS: i32 = 4999 + 19;
pub const wxID_HELP_PROCEDURES: i32 = 4999 + 20;
pub const wxID_HELP_CONTEXT: i32 = 4999 + 21;
pub const wxID_CLOSE_ALL: i32 = 4999 + 22;
pub const wxID_PREFERENCES: i32 = 4999 + 23;
pub const wxID_EDIT: i32 = 5030;
pub const wxID_CUT: i32 = 5030 + 1;
pub const wxID_COPY: i32 = 5030 + 2;
pub const wxID_PASTE: i32 = 5030 + 3;
pub const wxID_CLEAR: i32 = 5030 + 4;
pub const wxID_FIND: i32 = 5030 + 5;
pub const wxID_DUPLICATE: i32 = 5030 + 6;
pub const wxID_SELECTALL: i32 = 5030 + 7;
pub const wxID_DELETE: i32 = 5030 + 8;
pub const wxID_REPLACE: i32 = 5030 + 9;
pub const wxID_REPLACE_ALL: i32 = 5030 + 10;
pub const wxID_PROPERTIES: i32 = 5030 + 11;
pub const wxID_VIEW_DETAILS: i32 = 5030 + 12;
pub const wxID_VIEW_LARGEICONS: i32 = 5030 + 13;
pub const wxID_VIEW_SMALLICONS: i32 = 5030 + 14;
pub const wxID_VIEW_LIST: i32 = 5030 + 15;
pub const wxID_VIEW_SORTDATE: i32 = 5030 + 16;
pub const wxID_VIEW_SORTNAME: i32 = 5030 + 17;
pub const wxID_VIEW_SORTSIZE: i32 = 5030 + 18;
pub const wxID_VIEW_SORTTYPE: i32 = 5030 + 19;
pub const wxID_FILE: i32 = 5050;
pub const wxID_FILE1: i32 = 5050 + 1;
pub const wxID_FILE2: i32 = 5050 + 2;
pub const wxID_FILE3: i32 = 5050 + 3;
pub const wxID_FILE4: i32 = 5050 + 4;
pub const wxID_FILE5: i32 = 5050 + 5;
pub const wxID_FILE6: i32 = 5050 + 6;
pub const wxID_FILE7: i32 = 5050 + 7;
pub const wxID_FILE8: i32 = 5050 + 8;
pub const wxID_FILE9: i32 = 5050 + 9;
pub const wxID_OK: i32 = 5100;
pub const wxID_CANCEL: i32 = 5100 + 1;
pub const wxID_APPLY: i32 = 5100 + 2;
pub const wxID_YES: i32 = 5100 + 3;
pub const wxID_NO: i32 = 5100 + 4;
pub const wxID_STATIC: i32 = 5100 + 5;
pub const wxID_FORWARD: i32 = 5100 + 6;
pub const wxID_BACKWARD: i32 = 5100 + 7;
pub const wxID_DEFAULT: i32 = 5100 + 8;
pub const wxID_MORE: i32 = 5100 + 9;
pub const wxID_SETUP: i32 = 5100 + 10;
pub const wxID_RESET: i32 = 5100 + 11;
pub const wxID_CONTEXT_HELP: i32 = 5100 + 12;
pub const wxID_YESTOALL: i32 = 5100 + 13;
pub const wxID_NOTOALL: i32 = 5100 + 14;
pub const wxID_ABORT: i32 = 5100 + 15;
pub const wxID_RETRY: i32 = 5100 + 16;
pub const wxID_IGNORE: i32 = 5100 + 17;
pub const wxID_ADD: i32 = 5100 + 18;
pub const wxID_REMOVE: i32 = 5100 + 19;
pub const wxID_UP: i32 = 5100 + 20;
pub const wxID_DOWN: i32 = 5100 + 21;
pub const wxID_HOME: i32 = 5100 + 22;
pub const wxID_REFRESH: i32 = 5100 + 23;
pub const wxID_STOP: i32 = 5100 + 24;
pub const wxID_INDEX: i32 = 5100 + 25;
pub const wxID_BOLD: i32 = 5100 + 26;
pub const wxID_ITALIC: i32 = 5100 + 27;
pub const wxID_JUSTIFY_CENTER: i32 = 5100 + 28;
pub const wxID_JUSTIFY_FILL: i32 = 5100 + 29;
pub const wxID_JUSTIFY_RIGHT: i32 = 5100 + 30;
pub const wxID_JUSTIFY_LEFT: i32 = 5100 + 31;
pub const wxID_UNDERLINE: i32 = 5100 + 32;
pub const wxID_INDENT: i32 = 5100 + 33;
pub const wxID_UNINDENT: i32 = 5100 + 34;
pub const wxID_ZOOM_100: i32 = 5100 + 35;
pub const wxID_ZOOM_FIT: i32 = 5100 + 36;
pub const wxID_ZOOM_IN: i32 = 5100 + 37;
pub const wxID_ZOOM_OUT: i32 = 5100 + 38;
pub const wxID_UNDELETE: i32 = 5100 + 39;
pub const wxID_REVERT_TO_SAVED: i32 = 5100 + 40;
pub const wxID_CDROM: i32 = 5100 + 41;
pub const wxID_CONVERT: i32 = 5100 + 42;
pub const wxID_EXECUTE: i32 = 5100 + 43;
pub const wxID_FLOPPY: i32 = 5100 + 44;
pub const wxID_HARDDISK: i32 = 5100 + 45;
pub const wxID_BOTTOM: i32 = 5100 + 46;
pub const wxID_FIRST: i32 = 5100 + 47;
pub const wxID_LAST: i32 = 5100 + 48;
pub const wxID_TOP: i32 = 5100 + 49;
pub const wxID_INFO: i32 = 5100 + 50;
pub const wxID_JUMP_TO: i32 = 5100 + 51;
pub const wxID_NETWORK: i32 = 5100 + 52;
pub const wxID_SELECT_COLOR: i32 = 5100 + 53;
pub const wxID_SELECT_FONT: i32 = 5100 + 54;
pub const wxID_SORT_ASCENDING: i32 = 5100 + 55;
pub const wxID_SORT_DESCENDING: i32 = 5100 + 56;
pub const wxID_SPELL_CHECK: i32 = 5100 + 57;
pub const wxID_STRIKETHROUGH: i32 = 5100 + 58;
pub const wxID_SYSTEM_MENU: i32 = 5200;
pub const wxID_CLOSE_FRAME: i32 = 5200 + 1;
pub const wxID_MOVE_FRAME: i32 = 5200 + 2;
pub const wxID_RESIZE_FRAME: i32 = 5200 + 3;
pub const wxID_MAXIMIZE_FRAME: i32 = 5200 + 4;
pub const wxID_ICONIZE_FRAME: i32 = 5200 + 5;
pub const wxID_RESTORE_FRAME: i32 = 5200 + 6;
pub const wxID_MDI_WINDOW_FIRST: i32 = 5230;
pub const wxID_MDI_WINDOW_CASCADE: i32 = wxID_MDI_WINDOW_FIRST;
pub const wxID_MDI_WINDOW_TILE_HORZ: i32 = wxID_MDI_WINDOW_FIRST + 1;
pub const wxID_MDI_WINDOW_TILE_VERT: i32 = wxID_MDI_WINDOW_FIRST + 2;
pub const wxID_MDI_WINDOW_ARRANGE_ICONS: i32 = wxID_MDI_WINDOW_FIRST + 3;
pub const wxID_MDI_WINDOW_PREV: i32 = wxID_MDI_WINDOW_FIRST + 4;
pub const wxID_MDI_WINDOW_NEXT: i32 = wxID_MDI_WINDOW_FIRST + 5;
pub const wxID_MDI_WINDOW_LAST: i32 = wxID_MDI_WINDOW_NEXT;
pub const wxID_FILEDLGG: i32 = 5900;
pub const wxID_FILECTRL: i32 = 5950;
pub const wxID_HIGHEST: i32 = 5999;
//  ENUM: wxItemKind
pub const wxITEM_SEPARATOR: i32 = -1;
pub const wxITEM_NORMAL: i32 = -1 + 1;
pub const wxITEM_CHECK: i32 = -1 + 2;
pub const wxITEM_RADIO: i32 = -1 + 3;
pub const wxITEM_DROPDOWN: i32 = -1 + 4;
pub const wxITEM_MAX: i32 = -1 + 5;
//  ENUM: wxHitTest
pub const wxHT_NOWHERE: u32 = 0;
pub const wxHT_SCROLLBAR_FIRST: u32 = wxHT_NOWHERE;
pub const wxHT_SCROLLBAR_ARROW_LINE_1: u32 = wxHT_NOWHERE + 1;
pub const wxHT_SCROLLBAR_ARROW_LINE_2: u32 = wxHT_NOWHERE + 2;
pub const wxHT_SCROLLBAR_ARROW_PAGE_1: u32 = wxHT_NOWHERE + 3;
pub const wxHT_SCROLLBAR_ARROW_PAGE_2: u32 = wxHT_NOWHERE + 4;
pub const wxHT_SCROLLBAR_THUMB: u32 = wxHT_NOWHERE + 5;
pub const wxHT_SCROLLBAR_BAR_1: u32 = wxHT_NOWHERE + 6;
pub const wxHT_SCROLLBAR_BAR_2: u32 = wxHT_NOWHERE + 7;
pub const wxHT_SCROLLBAR_LAST: u32 = wxHT_NOWHERE + 8;
pub const wxHT_WINDOW_OUTSIDE: u32 = wxHT_NOWHERE + 9;
pub const wxHT_WINDOW_INSIDE: u32 = wxHT_NOWHERE + 10;
pub const wxHT_WINDOW_VERT_SCROLLBAR: u32 = wxHT_NOWHERE + 11;
pub const wxHT_WINDOW_HORZ_SCROLLBAR: u32 = wxHT_NOWHERE + 12;
pub const wxHT_WINDOW_CORNER: u32 = wxHT_NOWHERE + 13;
pub const wxHT_MAX: u32 = wxHT_NOWHERE + 14;
//  ENUM: wxDataFormatId
pub const wxDF_INVALID: u32 =          0;
pub const wxDF_TEXT: u32 =             1;
pub const wxDF_BITMAP: u32 =           2;
pub const wxDF_METAFILE: u32 =         3;
pub const wxDF_SYLK: u32 =             4;
pub const wxDF_DIF: u32 =              5;
pub const wxDF_TIFF: u32 =             6;
pub const wxDF_OEMTEXT: u32 =          7;
pub const wxDF_DIB: u32 =              8;
pub const wxDF_PALETTE: u32 =          9;
pub const wxDF_PENDATA: u32 =          10;
pub const wxDF_RIFF: u32 =             11;
pub const wxDF_WAVE: u32 =             12;
pub const wxDF_UNICODETEXT: u32 =      13;
pub const wxDF_ENHMETAFILE: u32 =      14;
pub const wxDF_FILENAME: u32 =         15;
pub const wxDF_LOCALE: u32 =           16;
pub const wxDF_PRIVATE: u32 =          20;
pub const wxDF_HTML: u32 =             30;
pub const wxDF_PNG: u32 =              31;
pub const wxDF_MAX: u32 =              31 + 1;
//  ENUM: wxKeyCode
pub const WXK_NONE: u32 =    0;
pub const WXK_CONTROL_A: u32 = 1;
pub const WXK_CONTROL_B: u32 = 1 + 1;
pub const WXK_CONTROL_C: u32 = 1 + 2;
pub const WXK_CONTROL_D: u32 = 1 + 3;
pub const WXK_CONTROL_E: u32 = 1 + 4;
pub const WXK_CONTROL_F: u32 = 1 + 5;
pub const WXK_CONTROL_G: u32 = 1 + 6;
pub const WXK_CONTROL_H: u32 = 1 + 7;
pub const WXK_CONTROL_I: u32 = 1 + 8;
pub const WXK_CONTROL_J: u32 = 1 + 9;
pub const WXK_CONTROL_K: u32 = 1 + 10;
pub const WXK_CONTROL_L: u32 = 1 + 11;
pub const WXK_CONTROL_M: u32 = 1 + 12;
pub const WXK_CONTROL_N: u32 = 1 + 13;
pub const WXK_CONTROL_O: u32 = 1 + 14;
pub const WXK_CONTROL_P: u32 = 1 + 15;
pub const WXK_CONTROL_Q: u32 = 1 + 16;
pub const WXK_CONTROL_R: u32 = 1 + 17;
pub const WXK_CONTROL_S: u32 = 1 + 18;
pub const WXK_CONTROL_T: u32 = 1 + 19;
pub const WXK_CONTROL_U: u32 = 1 + 20;
pub const WXK_CONTROL_V: u32 = 1 + 21;
pub const WXK_CONTROL_W: u32 = 1 + 22;
pub const WXK_CONTROL_X: u32 = 1 + 23;
pub const WXK_CONTROL_Y: u32 = 1 + 24;
pub const WXK_CONTROL_Z: u32 = 1 + 25;
pub const WXK_BACK: u32 =    8;
pub const WXK_TAB: u32 =    9;
pub const WXK_RETURN: u32 =    13;
pub const WXK_ESCAPE: u32 =    27;
pub const WXK_SPACE: u32 =    32;
pub const WXK_DELETE: u32 =    127;
pub const WXK_START: u32 = 300;
pub const WXK_LBUTTON: u32 = 300 + 1;
pub const WXK_RBUTTON: u32 = 300 + 2;
pub const WXK_CANCEL: u32 = 300 + 3;
pub const WXK_MBUTTON: u32 = 300 + 4;
pub const WXK_CLEAR: u32 = 300 + 5;
pub const WXK_SHIFT: u32 = 300 + 6;
pub const WXK_ALT: u32 = 300 + 7;
pub const WXK_CONTROL: u32 = 300 + 8;
pub const WXK_RAW_CONTROL: u32 = 300 + 9;
pub const WXK_MENU: u32 = 300 + 10;
pub const WXK_PAUSE: u32 = 300 + 11;
pub const WXK_CAPITAL: u32 = 300 + 12;
pub const WXK_END: u32 = 300 + 13;
pub const WXK_HOME: u32 = 300 + 14;
pub const WXK_LEFT: u32 = 300 + 15;
pub const WXK_UP: u32 = 300 + 16;
pub const WXK_RIGHT: u32 = 300 + 17;
pub const WXK_DOWN: u32 = 300 + 18;
pub const WXK_SELECT: u32 = 300 + 19;
pub const WXK_PRINT: u32 = 300 + 20;
pub const WXK_EXECUTE: u32 = 300 + 21;
pub const WXK_SNAPSHOT: u32 = 300 + 22;
pub const WXK_INSERT: u32 = 300 + 23;
pub const WXK_HELP: u32 = 300 + 24;
pub const WXK_NUMPAD0: u32 = 300 + 25;
pub const WXK_NUMPAD1: u32 = 300 + 26;
pub const WXK_NUMPAD2: u32 = 300 + 27;
pub const WXK_NUMPAD3: u32 = 300 + 28;
pub const WXK_NUMPAD4: u32 = 300 + 29;
pub const WXK_NUMPAD5: u32 = 300 + 30;
pub const WXK_NUMPAD6: u32 = 300 + 31;
pub const WXK_NUMPAD7: u32 = 300 + 32;
pub const WXK_NUMPAD8: u32 = 300 + 33;
pub const WXK_NUMPAD9: u32 = 300 + 34;
pub const WXK_MULTIPLY: u32 = 300 + 35;
pub const WXK_ADD: u32 = 300 + 36;
pub const WXK_SEPARATOR: u32 = 300 + 37;
pub const WXK_SUBTRACT: u32 = 300 + 38;
pub const WXK_DECIMAL: u32 = 300 + 39;
pub const WXK_DIVIDE: u32 = 300 + 40;
pub const WXK_F1: u32 = 300 + 41;
pub const WXK_F2: u32 = 300 + 42;
pub const WXK_F3: u32 = 300 + 43;
pub const WXK_F4: u32 = 300 + 44;
pub const WXK_F5: u32 = 300 + 45;
pub const WXK_F6: u32 = 300 + 46;
pub const WXK_F7: u32 = 300 + 47;
pub const WXK_F8: u32 = 300 + 48;
pub const WXK_F9: u32 = 300 + 49;
pub const WXK_F10: u32 = 300 + 50;
pub const WXK_F11: u32 = 300 + 51;
pub const WXK_F12: u32 = 300 + 52;
pub const WXK_F13: u32 = 300 + 53;
pub const WXK_F14: u32 = 300 + 54;
pub const WXK_F15: u32 = 300 + 55;
pub const WXK_F16: u32 = 300 + 56;
pub const WXK_F17: u32 = 300 + 57;
pub const WXK_F18: u32 = 300 + 58;
pub const WXK_F19: u32 = 300 + 59;
pub const WXK_F20: u32 = 300 + 60;
pub const WXK_F21: u32 = 300 + 61;
pub const WXK_F22: u32 = 300 + 62;
pub const WXK_F23: u32 = 300 + 63;
pub const WXK_F24: u32 = 300 + 64;
pub const WXK_NUMLOCK: u32 = 300 + 65;
pub const WXK_SCROLL: u32 = 300 + 66;
pub const WXK_PAGEUP: u32 = 300 + 67;
pub const WXK_PAGEDOWN: u32 = 300 + 68;
pub const WXK_NUMPAD_SPACE: u32 = 300 + 69;
pub const WXK_NUMPAD_TAB: u32 = 300 + 70;
pub const WXK_NUMPAD_ENTER: u32 = 300 + 71;
pub const WXK_NUMPAD_F1: u32 = 300 + 72;
pub const WXK_NUMPAD_F2: u32 = 300 + 73;
pub const WXK_NUMPAD_F3: u32 = 300 + 74;
pub const WXK_NUMPAD_F4: u32 = 300 + 75;
pub const WXK_NUMPAD_HOME: u32 = 300 + 76;
pub const WXK_NUMPAD_LEFT: u32 = 300 + 77;
pub const WXK_NUMPAD_UP: u32 = 300 + 78;
pub const WXK_NUMPAD_RIGHT: u32 = 300 + 79;
pub const WXK_NUMPAD_DOWN: u32 = 300 + 80;
pub const WXK_NUMPAD_PAGEUP: u32 = 300 + 81;
pub const WXK_NUMPAD_PAGEDOWN: u32 = 300 + 82;
pub const WXK_NUMPAD_END: u32 = 300 + 83;
pub const WXK_NUMPAD_BEGIN: u32 = 300 + 84;
pub const WXK_NUMPAD_INSERT: u32 = 300 + 85;
pub const WXK_NUMPAD_DELETE: u32 = 300 + 86;
pub const WXK_NUMPAD_EQUAL: u32 = 300 + 87;
pub const WXK_NUMPAD_MULTIPLY: u32 = 300 + 88;
pub const WXK_NUMPAD_ADD: u32 = 300 + 89;
pub const WXK_NUMPAD_SEPARATOR: u32 = 300 + 90;
pub const WXK_NUMPAD_SUBTRACT: u32 = 300 + 91;
pub const WXK_NUMPAD_DECIMAL: u32 = 300 + 92;
pub const WXK_NUMPAD_DIVIDE: u32 = 300 + 93;
pub const WXK_WINDOWS_LEFT: u32 = 300 + 94;
pub const WXK_WINDOWS_RIGHT: u32 = 300 + 95;
pub const WXK_WINDOWS_MENU: u32 = 300 + 96;
pub const WXK_COMMAND: u32 = 300 + 97;
pub const WXK_SPECIAL1: u32 = 193;
pub const WXK_SPECIAL2: u32 = 193 + 1;
pub const WXK_SPECIAL3: u32 = 193 + 2;
pub const WXK_SPECIAL4: u32 = 193 + 3;
pub const WXK_SPECIAL5: u32 = 193 + 4;
pub const WXK_SPECIAL6: u32 = 193 + 5;
pub const WXK_SPECIAL7: u32 = 193 + 6;
pub const WXK_SPECIAL8: u32 = 193 + 7;
pub const WXK_SPECIAL9: u32 = 193 + 8;
pub const WXK_SPECIAL10: u32 = 193 + 9;
pub const WXK_SPECIAL11: u32 = 193 + 10;
pub const WXK_SPECIAL12: u32 = 193 + 11;
pub const WXK_SPECIAL13: u32 = 193 + 12;
pub const WXK_SPECIAL14: u32 = 193 + 13;
pub const WXK_SPECIAL15: u32 = 193 + 14;
pub const WXK_SPECIAL16: u32 = 193 + 15;
pub const WXK_SPECIAL17: u32 = 193 + 16;
pub const WXK_SPECIAL18: u32 = 193 + 17;
pub const WXK_SPECIAL19: u32 = 193 + 18;
pub const WXK_SPECIAL20: u32 = 193 + 19;
pub const WXK_BROWSER_BACK: u32 = 501;
pub const WXK_BROWSER_FORWARD: u32 = 501 + 1;
pub const WXK_BROWSER_REFRESH: u32 = 501 + 2;
pub const WXK_BROWSER_STOP: u32 = 501 + 3;
pub const WXK_BROWSER_SEARCH: u32 = 501 + 4;
pub const WXK_BROWSER_FAVORITES: u32 = 501 + 5;
pub const WXK_BROWSER_HOME: u32 = 501 + 6;
pub const WXK_VOLUME_MUTE: u32 = 501 + 7;
pub const WXK_VOLUME_DOWN: u32 = 501 + 8;
pub const WXK_VOLUME_UP: u32 = 501 + 9;
pub const WXK_MEDIA_NEXT_TRACK: u32 = 501 + 10;
pub const WXK_MEDIA_PREV_TRACK: u32 = 501 + 11;
pub const WXK_MEDIA_STOP: u32 = 501 + 12;
pub const WXK_MEDIA_PLAY_PAUSE: u32 = 501 + 13;
pub const WXK_LAUNCH_MAIL: u32 = 501 + 14;
pub const WXK_LAUNCH_APP1: u32 = 501 + 15;
pub const WXK_LAUNCH_APP2: u32 = 501 + 16;
//  ENUM: wxKeyModifier
pub const wxMOD_NONE: u32 = 0x0000;
pub const wxMOD_ALT: u32 = 0x0001;
pub const wxMOD_CONTROL: u32 = 0x0002;
pub const wxMOD_ALTGR: u32 = wxMOD_ALT | wxMOD_CONTROL;
pub const wxMOD_SHIFT: u32 = 0x0004;
pub const wxMOD_META: u32 = 0x0008;
pub const wxMOD_WIN: u32 = wxMOD_META;
pub const wxMOD_RAW_CONTROL: u32 = wxMOD_META + 1;
pub const wxMOD_CMD: u32 = wxMOD_CONTROL;
pub const wxMOD_ALL: u32 = 0xffff;
//  ENUM: wxPaperSize
pub const wxPAPER_10X11: u32 = 0;
pub const wxPAPER_10X14: u32 = 0 + 1;
pub const wxPAPER_11X17: u32 = 0 + 2;
pub const wxPAPER_12X11: u32 = 0 + 3;
pub const wxPAPER_15X11: u32 = 0 + 4;
pub const wxPAPER_9X11: u32 = 0 + 5;
pub const wxPAPER_A2: u32 = 0 + 6;
pub const wxPAPER_A3: u32 = 0 + 7;
pub const wxPAPER_A3_EXTRA: u32 = 0 + 8;
pub const wxPAPER_A3_EXTRA_TRANSVERSE: u32 = 0 + 9;
pub const wxPAPER_A3_ROTATED: u32 = 0 + 10;
pub const wxPAPER_A3_TRANSVERSE: u32 = 0 + 11;
pub const wxPAPER_A4: u32 = 0 + 12;
pub const wxPAPER_A4SMALL: u32 = 0 + 13;
pub const wxPAPER_A4_EXTRA: u32 = 0 + 14;
pub const wxPAPER_A4_PLUS: u32 = 0 + 15;
pub const wxPAPER_A4_ROTATED: u32 = 0 + 16;
pub const wxPAPER_A4_TRANSVERSE: u32 = 0 + 17;
pub const wxPAPER_A5: u32 = 0 + 18;
pub const wxPAPER_A5_EXTRA: u32 = 0 + 19;
pub const wxPAPER_A5_ROTATED: u32 = 0 + 20;
pub const wxPAPER_A5_TRANSVERSE: u32 = 0 + 21;
pub const wxPAPER_A6: u32 = 0 + 22;
pub const wxPAPER_A6_ROTATED: u32 = 0 + 23;
pub const wxPAPER_A_PLUS: u32 = 0 + 24;
pub const wxPAPER_B4: u32 = 0 + 25;
pub const wxPAPER_B4_JIS_ROTATED: u32 = 0 + 26;
pub const wxPAPER_B5: u32 = 0 + 27;
pub const wxPAPER_B5_EXTRA: u32 = 0 + 28;
pub const wxPAPER_B5_JIS_ROTATED: u32 = 0 + 29;
pub const wxPAPER_B5_TRANSVERSE: u32 = 0 + 30;
pub const wxPAPER_B6_JIS: u32 = 0 + 31;
pub const wxPAPER_B6_JIS_ROTATED: u32 = 0 + 32;
pub const wxPAPER_B_PLUS: u32 = 0 + 33;
pub const wxPAPER_CSHEET: u32 = 0 + 34;
pub const wxPAPER_DBL_JAPANESE_POSTCARD: u32 = 0 + 35;
pub const wxPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 0 + 36;
pub const wxPAPER_DSHEET: u32 = 0 + 37;
pub const wxPAPER_ENV_10: u32 = 0 + 38;
pub const wxPAPER_ENV_11: u32 = 0 + 39;
pub const wxPAPER_ENV_12: u32 = 0 + 40;
pub const wxPAPER_ENV_14: u32 = 0 + 41;
pub const wxPAPER_ENV_9: u32 = 0 + 42;
pub const wxPAPER_ENV_B4: u32 = 0 + 43;
pub const wxPAPER_ENV_B5: u32 = 0 + 44;
pub const wxPAPER_ENV_B6: u32 = 0 + 45;
pub const wxPAPER_ENV_C3: u32 = 0 + 46;
pub const wxPAPER_ENV_C4: u32 = 0 + 47;
pub const wxPAPER_ENV_C5: u32 = 0 + 48;
pub const wxPAPER_ENV_C6: u32 = 0 + 49;
pub const wxPAPER_ENV_C65: u32 = 0 + 50;
pub const wxPAPER_ENV_DL: u32 = 0 + 51;
pub const wxPAPER_ENV_INVITE: u32 = 0 + 52;
pub const wxPAPER_ENV_ITALY: u32 = 0 + 53;
pub const wxPAPER_ENV_MONARCH: u32 = 0 + 54;
pub const wxPAPER_ENV_PERSONAL: u32 = 0 + 55;
pub const wxPAPER_ESHEET: u32 = 0 + 56;
pub const wxPAPER_EXECUTIVE: u32 = 0 + 57;
pub const wxPAPER_FANFOLD_LGL_GERMAN: u32 = 0 + 58;
pub const wxPAPER_FANFOLD_STD_GERMAN: u32 = 0 + 59;
pub const wxPAPER_FANFOLD_US: u32 = 0 + 60;
pub const wxPAPER_FOLIO: u32 = 0 + 61;
pub const wxPAPER_ISO_B4: u32 = 0 + 62;
pub const wxPAPER_JAPANESE_POSTCARD: u32 = 0 + 63;
pub const wxPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 0 + 64;
pub const wxPAPER_JENV_CHOU3: u32 = 0 + 65;
pub const wxPAPER_JENV_CHOU3_ROTATED: u32 = 0 + 66;
pub const wxPAPER_JENV_CHOU4: u32 = 0 + 67;
pub const wxPAPER_JENV_CHOU4_ROTATED: u32 = 0 + 68;
pub const wxPAPER_JENV_KAKU2: u32 = 0 + 69;
pub const wxPAPER_JENV_KAKU2_ROTATED: u32 = 0 + 70;
pub const wxPAPER_JENV_KAKU3: u32 = 0 + 71;
pub const wxPAPER_JENV_KAKU3_ROTATED: u32 = 0 + 72;
pub const wxPAPER_JENV_YOU4: u32 = 0 + 73;
pub const wxPAPER_JENV_YOU4_ROTATED: u32 = 0 + 74;
pub const wxPAPER_LEDGER: u32 = 0 + 75;
pub const wxPAPER_LEGAL: u32 = 0 + 76;
pub const wxPAPER_LEGAL_EXTRA: u32 = 0 + 77;
pub const wxPAPER_LETTER: u32 = 0 + 78;
pub const wxPAPER_LETTERSMALL: u32 = 0 + 79;
pub const wxPAPER_LETTER_EXTRA: u32 = 0 + 80;
pub const wxPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 0 + 81;
pub const wxPAPER_LETTER_PLUS: u32 = 0 + 82;
pub const wxPAPER_LETTER_ROTATED: u32 = 0 + 83;
pub const wxPAPER_LETTER_TRANSVERSE: u32 = 0 + 84;
pub const wxPAPER_NONE: u32 = 0 + 85;
pub const wxPAPER_NOTE: u32 = 0 + 86;
pub const wxPAPER_P16K: u32 = 0 + 87;
pub const wxPAPER_P16K_ROTATED: u32 = 0 + 88;
pub const wxPAPER_P32K: u32 = 0 + 89;
pub const wxPAPER_P32KBIG: u32 = 0 + 90;
pub const wxPAPER_P32KBIG_ROTATED: u32 = 0 + 91;
pub const wxPAPER_P32K_ROTATED: u32 = 0 + 92;
pub const wxPAPER_PENV_1: u32 = 0 + 93;
pub const wxPAPER_PENV_10: u32 = 0 + 94;
pub const wxPAPER_PENV_10_ROTATED: u32 = 0 + 95;
pub const wxPAPER_PENV_1_ROTATED: u32 = 0 + 96;
pub const wxPAPER_PENV_2: u32 = 0 + 97;
pub const wxPAPER_PENV_2_ROTATED: u32 = 0 + 98;
pub const wxPAPER_PENV_3: u32 = 0 + 99;
pub const wxPAPER_PENV_3_ROTATED: u32 = 0 + 100;
pub const wxPAPER_PENV_4: u32 = 0 + 101;
pub const wxPAPER_PENV_4_ROTATED: u32 = 0 + 102;
pub const wxPAPER_PENV_5: u32 = 0 + 103;
pub const wxPAPER_PENV_5_ROTATED: u32 = 0 + 104;
pub const wxPAPER_PENV_6: u32 = 0 + 105;
pub const wxPAPER_PENV_6_ROTATED: u32 = 0 + 106;
pub const wxPAPER_PENV_7: u32 = 0 + 107;
pub const wxPAPER_PENV_7_ROTATED: u32 = 0 + 108;
pub const wxPAPER_PENV_8: u32 = 0 + 109;
pub const wxPAPER_PENV_8_ROTATED: u32 = 0 + 110;
pub const wxPAPER_PENV_9: u32 = 0 + 111;
pub const wxPAPER_PENV_9_ROTATED: u32 = 0 + 112;
pub const wxPAPER_QUARTO: u32 = 0 + 113;
pub const wxPAPER_STATEMENT: u32 = 0 + 114;
pub const wxPAPER_TABLOID: u32 = 0 + 115;
pub const wxPAPER_TABLOID_EXTRA: u32 = 0 + 116;
//  ENUM: wxPrintOrientation
pub const wxPORTRAIT: u32 = 0;
pub const wxLANDSCAPE: u32 = 0 + 1;
//  ENUM: wxDuplexMode
pub const wxDUPLEX_SIMPLEX: u32 = 0;
pub const wxDUPLEX_HORIZONTAL: u32 = 0 + 1;
pub const wxDUPLEX_VERTICAL: u32 = 0 + 2;
//  ENUM: wxPrintMode
pub const wxPRINT_MODE_NONE: u32 =    0;
pub const wxPRINT_MODE_PREVIEW: u32 = 1;
pub const wxPRINT_MODE_FILE: u32 =    2;
pub const wxPRINT_MODE_PRINTER: u32 = 3;
pub const wxPRINT_MODE_STREAM: u32 =  4;
//  ENUM: wxUpdateUI
pub const wxUPDATE_UI_NONE: u32 = 0;
pub const wxUPDATE_UI_RECURSE: u32 = 0 + 1;
pub const wxUPDATE_UI_FROMIDLE: u32 = 0 + 2;

//  SKIP: wxPG_IT_CHILDREN
//  ENUM: wxPG_ITERATOR_FLAGS
pub const wxPG_ITERATE_PROPERTIES: u32 = wxPG_PROP_PROPERTY |
                          wxPG_PROP_MISC_PARENT |
                          wxPG_PROP_AGGREGATE |
                          wxPG_PROP_COLLAPSED |
                          wxPG_IT_CHILDREN(wxPG_PROP_MISC_PARENT) |
                          wxPG_IT_CHILDREN(wxPG_PROP_CATEGORY);
pub const wxPG_ITERATE_HIDDEN: u32 = wxPG_PROP_HIDDEN |
                      wxPG_IT_CHILDREN(wxPG_PROP_COLLAPSED);
pub const wxPG_ITERATE_FIXED_CHILDREN: u32 = wxPG_IT_CHILDREN(wxPG_PROP_AGGREGATE) |
                              wxPG_ITERATE_PROPERTIES;
pub const wxPG_ITERATE_CATEGORIES: u32 = wxPG_PROP_CATEGORY |
                          wxPG_IT_CHILDREN(wxPG_PROP_CATEGORY) |
                          wxPG_PROP_COLLAPSED;
pub const wxPG_ITERATE_ALL_PARENTS: u32 = wxPG_PROP_MISC_PARENT |
                           wxPG_PROP_AGGREGATE |
                           wxPG_PROP_CATEGORY;
pub const wxPG_ITERATE_ALL_PARENTS_RECURSIVELY: u32 = wxPG_ITERATE_ALL_PARENTS |
                                       wxPG_IT_CHILDREN(
                                                wxPG_ITERATE_ALL_PARENTS);
pub const wxPG_ITERATOR_FLAGS_ALL: u32 = wxPG_PROP_PROPERTY |
                          wxPG_PROP_MISC_PARENT |
                          wxPG_PROP_AGGREGATE |
                          wxPG_PROP_HIDDEN |
                          wxPG_PROP_CATEGORY |
                          wxPG_PROP_COLLAPSED;
pub const wxPG_ITERATOR_MASK_OP_ITEM: u32 = wxPG_ITERATOR_FLAGS_ALL;
pub const wxPG_ITERATOR_MASK_OP_PARENT: u32 = wxPG_ITERATOR_FLAGS_ALL;
pub const wxPG_ITERATE_VISIBLE: u32 = wxPG_ITERATE_PROPERTIES |
                       wxPG_PROP_CATEGORY |
                       wxPG_IT_CHILDREN(wxPG_PROP_AGGREGATE);
pub const wxPG_ITERATE_ALL: u32 = wxPG_ITERATE_VISIBLE |
                   wxPG_ITERATE_HIDDEN;
pub const wxPG_ITERATE_NORMAL: u32 = wxPG_ITERATE_PROPERTIES |
                      wxPG_ITERATE_HIDDEN;
pub const wxPG_ITERATE_DEFAULT: u32 = wxPG_ITERATE_NORMAL;

//  ENUM: wxLanguage
pub const wxLANGUAGE_DEFAULT: u32 = 0;
pub const wxLANGUAGE_UNKNOWN: u32 = 0 + 1;
pub const wxLANGUAGE_ABKHAZIAN: u32 = 0 + 2;
pub const wxLANGUAGE_AFAR: u32 = 0 + 3;
pub const wxLANGUAGE_AFRIKAANS: u32 = 0 + 4;
pub const wxLANGUAGE_ALBANIAN: u32 = 0 + 5;
pub const wxLANGUAGE_AMHARIC: u32 = 0 + 6;
pub const wxLANGUAGE_ARABIC: u32 = 0 + 7;
pub const wxLANGUAGE_ARABIC_ALGERIA: u32 = 0 + 8;
pub const wxLANGUAGE_ARABIC_BAHRAIN: u32 = 0 + 9;
pub const wxLANGUAGE_ARABIC_EGYPT: u32 = 0 + 10;
pub const wxLANGUAGE_ARABIC_IRAQ: u32 = 0 + 11;
pub const wxLANGUAGE_ARABIC_JORDAN: u32 = 0 + 12;
pub const wxLANGUAGE_ARABIC_KUWAIT: u32 = 0 + 13;
pub const wxLANGUAGE_ARABIC_LEBANON: u32 = 0 + 14;
pub const wxLANGUAGE_ARABIC_LIBYA: u32 = 0 + 15;
pub const wxLANGUAGE_ARABIC_MOROCCO: u32 = 0 + 16;
pub const wxLANGUAGE_ARABIC_OMAN: u32 = 0 + 17;
pub const wxLANGUAGE_ARABIC_QATAR: u32 = 0 + 18;
pub const wxLANGUAGE_ARABIC_SAUDI_ARABIA: u32 = 0 + 19;
pub const wxLANGUAGE_ARABIC_SUDAN: u32 = 0 + 20;
pub const wxLANGUAGE_ARABIC_SYRIA: u32 = 0 + 21;
pub const wxLANGUAGE_ARABIC_TUNISIA: u32 = 0 + 22;
pub const wxLANGUAGE_ARABIC_UAE: u32 = 0 + 23;
pub const wxLANGUAGE_ARABIC_YEMEN: u32 = 0 + 24;
pub const wxLANGUAGE_ARMENIAN: u32 = 0 + 25;
pub const wxLANGUAGE_ASSAMESE: u32 = 0 + 26;
pub const wxLANGUAGE_ASTURIAN: u32 = 0 + 27;
pub const wxLANGUAGE_AYMARA: u32 = 0 + 28;
pub const wxLANGUAGE_AZERI: u32 = 0 + 29;
pub const wxLANGUAGE_AZERI_CYRILLIC: u32 = 0 + 30;
pub const wxLANGUAGE_AZERI_LATIN: u32 = 0 + 31;
pub const wxLANGUAGE_BASHKIR: u32 = 0 + 32;
pub const wxLANGUAGE_BASQUE: u32 = 0 + 33;
pub const wxLANGUAGE_BELARUSIAN: u32 = 0 + 34;
pub const wxLANGUAGE_BENGALI: u32 = 0 + 35;
pub const wxLANGUAGE_BHUTANI: u32 = 0 + 36;
pub const wxLANGUAGE_BIHARI: u32 = 0 + 37;
pub const wxLANGUAGE_BISLAMA: u32 = 0 + 38;
pub const wxLANGUAGE_BOSNIAN: u32 = 0 + 39;
pub const wxLANGUAGE_BRETON: u32 = 0 + 40;
pub const wxLANGUAGE_BULGARIAN: u32 = 0 + 41;
pub const wxLANGUAGE_BURMESE: u32 = 0 + 42;
pub const wxLANGUAGE_CATALAN: u32 = 0 + 43;
pub const wxLANGUAGE_CHINESE: u32 = 0 + 44;
pub const wxLANGUAGE_CHINESE_SIMPLIFIED: u32 = 0 + 45;
pub const wxLANGUAGE_CHINESE_TRADITIONAL: u32 = 0 + 46;
pub const wxLANGUAGE_CHINESE_HONGKONG: u32 = 0 + 47;
pub const wxLANGUAGE_CHINESE_MACAU: u32 = 0 + 48;
pub const wxLANGUAGE_CHINESE_SINGAPORE: u32 = 0 + 49;
pub const wxLANGUAGE_CHINESE_TAIWAN: u32 = 0 + 50;
pub const wxLANGUAGE_CORSICAN: u32 = 0 + 51;
pub const wxLANGUAGE_CROATIAN: u32 = 0 + 52;
pub const wxLANGUAGE_CZECH: u32 = 0 + 53;
pub const wxLANGUAGE_DANISH: u32 = 0 + 54;
pub const wxLANGUAGE_DUTCH: u32 = 0 + 55;
pub const wxLANGUAGE_DUTCH_BELGIAN: u32 = 0 + 56;
pub const wxLANGUAGE_ENGLISH: u32 = 0 + 57;
pub const wxLANGUAGE_ENGLISH_UK: u32 = 0 + 58;
pub const wxLANGUAGE_ENGLISH_US: u32 = 0 + 59;
pub const wxLANGUAGE_ENGLISH_AUSTRALIA: u32 = 0 + 60;
pub const wxLANGUAGE_ENGLISH_BELIZE: u32 = 0 + 61;
pub const wxLANGUAGE_ENGLISH_BOTSWANA: u32 = 0 + 62;
pub const wxLANGUAGE_ENGLISH_CANADA: u32 = 0 + 63;
pub const wxLANGUAGE_ENGLISH_CARIBBEAN: u32 = 0 + 64;
pub const wxLANGUAGE_ENGLISH_DENMARK: u32 = 0 + 65;
pub const wxLANGUAGE_ENGLISH_EIRE: u32 = 0 + 66;
pub const wxLANGUAGE_ENGLISH_ISRAEL: u32 = 0 + 67;
pub const wxLANGUAGE_ENGLISH_JAMAICA: u32 = 0 + 68;
pub const wxLANGUAGE_ENGLISH_NEW_ZEALAND: u32 = 0 + 69;
pub const wxLANGUAGE_ENGLISH_PHILIPPINES: u32 = 0 + 70;
pub const wxLANGUAGE_ENGLISH_SOUTH_AFRICA: u32 = 0 + 71;
pub const wxLANGUAGE_ENGLISH_TRINIDAD: u32 = 0 + 72;
pub const wxLANGUAGE_ENGLISH_ZIMBABWE: u32 = 0 + 73;
pub const wxLANGUAGE_ESPERANTO: u32 = 0 + 74;
pub const wxLANGUAGE_ESTONIAN: u32 = 0 + 75;
pub const wxLANGUAGE_FAEROESE: u32 = 0 + 76;
pub const wxLANGUAGE_FARSI: u32 = 0 + 77;
pub const wxLANGUAGE_FIJI: u32 = 0 + 78;
pub const wxLANGUAGE_FINNISH: u32 = 0 + 79;
pub const wxLANGUAGE_FRENCH: u32 = 0 + 80;
pub const wxLANGUAGE_FRENCH_BELGIAN: u32 = 0 + 81;
pub const wxLANGUAGE_FRENCH_CANADIAN: u32 = 0 + 82;
pub const wxLANGUAGE_FRENCH_LUXEMBOURG: u32 = 0 + 83;
pub const wxLANGUAGE_FRENCH_MONACO: u32 = 0 + 84;
pub const wxLANGUAGE_FRENCH_SWISS: u32 = 0 + 85;
pub const wxLANGUAGE_FRISIAN: u32 = 0 + 86;
pub const wxLANGUAGE_GALICIAN: u32 = 0 + 87;
pub const wxLANGUAGE_GEORGIAN: u32 = 0 + 88;
pub const wxLANGUAGE_GERMAN: u32 = 0 + 89;
pub const wxLANGUAGE_GERMAN_AUSTRIAN: u32 = 0 + 90;
pub const wxLANGUAGE_GERMAN_BELGIUM: u32 = 0 + 91;
pub const wxLANGUAGE_GERMAN_LIECHTENSTEIN: u32 = 0 + 92;
pub const wxLANGUAGE_GERMAN_LUXEMBOURG: u32 = 0 + 93;
pub const wxLANGUAGE_GERMAN_SWISS: u32 = 0 + 94;
pub const wxLANGUAGE_GREEK: u32 = 0 + 95;
pub const wxLANGUAGE_GREENLANDIC: u32 = 0 + 96;
pub const wxLANGUAGE_GUARANI: u32 = 0 + 97;
pub const wxLANGUAGE_GUJARATI: u32 = 0 + 98;
pub const wxLANGUAGE_HAUSA: u32 = 0 + 99;
pub const wxLANGUAGE_HEBREW: u32 = 0 + 100;
pub const wxLANGUAGE_HINDI: u32 = 0 + 101;
pub const wxLANGUAGE_HUNGARIAN: u32 = 0 + 102;
pub const wxLANGUAGE_ICELANDIC: u32 = 0 + 103;
pub const wxLANGUAGE_INDONESIAN: u32 = 0 + 104;
pub const wxLANGUAGE_INTERLINGUA: u32 = 0 + 105;
pub const wxLANGUAGE_INTERLINGUE: u32 = 0 + 106;
pub const wxLANGUAGE_INUKTITUT: u32 = 0 + 107;
pub const wxLANGUAGE_INUPIAK: u32 = 0 + 108;
pub const wxLANGUAGE_IRISH: u32 = 0 + 109;
pub const wxLANGUAGE_ITALIAN: u32 = 0 + 110;
pub const wxLANGUAGE_ITALIAN_SWISS: u32 = 0 + 111;
pub const wxLANGUAGE_JAPANESE: u32 = 0 + 112;
pub const wxLANGUAGE_JAVANESE: u32 = 0 + 113;
pub const wxLANGUAGE_KABYLE: u32 = 0 + 114;
pub const wxLANGUAGE_KANNADA: u32 = 0 + 115;
pub const wxLANGUAGE_KASHMIRI: u32 = 0 + 116;
pub const wxLANGUAGE_KASHMIRI_INDIA: u32 = 0 + 117;
pub const wxLANGUAGE_KAZAKH: u32 = 0 + 118;
pub const wxLANGUAGE_KERNEWEK: u32 = 0 + 119;
pub const wxLANGUAGE_KHMER: u32 = 0 + 120;
pub const wxLANGUAGE_KINYARWANDA: u32 = 0 + 121;
pub const wxLANGUAGE_KIRGHIZ: u32 = 0 + 122;
pub const wxLANGUAGE_KIRUNDI: u32 = 0 + 123;
pub const wxLANGUAGE_KONKANI: u32 = 0 + 124;
pub const wxLANGUAGE_KOREAN: u32 = 0 + 125;
pub const wxLANGUAGE_KURDISH: u32 = 0 + 126;
pub const wxLANGUAGE_LAOTHIAN: u32 = 0 + 127;
pub const wxLANGUAGE_LATIN: u32 = 0 + 128;
pub const wxLANGUAGE_LATVIAN: u32 = 0 + 129;
pub const wxLANGUAGE_LINGALA: u32 = 0 + 130;
pub const wxLANGUAGE_LITHUANIAN: u32 = 0 + 131;
pub const wxLANGUAGE_MACEDONIAN: u32 = 0 + 132;
pub const wxLANGUAGE_MALAGASY: u32 = 0 + 133;
pub const wxLANGUAGE_MALAY: u32 = 0 + 134;
pub const wxLANGUAGE_MALAYALAM: u32 = 0 + 135;
pub const wxLANGUAGE_MALAY_BRUNEI_DARUSSALAM: u32 = 0 + 136;
pub const wxLANGUAGE_MALAY_MALAYSIA: u32 = 0 + 137;
pub const wxLANGUAGE_MALTESE: u32 = 0 + 138;
pub const wxLANGUAGE_MANIPURI: u32 = 0 + 139;
pub const wxLANGUAGE_MAORI: u32 = 0 + 140;
pub const wxLANGUAGE_MARATHI: u32 = 0 + 141;
pub const wxLANGUAGE_MOLDAVIAN: u32 = 0 + 142;
pub const wxLANGUAGE_MONGOLIAN: u32 = 0 + 143;
pub const wxLANGUAGE_NAURU: u32 = 0 + 144;
pub const wxLANGUAGE_NEPALI: u32 = 0 + 145;
pub const wxLANGUAGE_NEPALI_INDIA: u32 = 0 + 146;
pub const wxLANGUAGE_NORWEGIAN_BOKMAL: u32 = 0 + 147;
pub const wxLANGUAGE_NORWEGIAN_NYNORSK: u32 = 0 + 148;
pub const wxLANGUAGE_OCCITAN: u32 = 0 + 149;
pub const wxLANGUAGE_ORIYA: u32 = 0 + 150;
pub const wxLANGUAGE_OROMO: u32 = 0 + 151;
pub const wxLANGUAGE_PASHTO: u32 = 0 + 152;
pub const wxLANGUAGE_POLISH: u32 = 0 + 153;
pub const wxLANGUAGE_PORTUGUESE: u32 = 0 + 154;
pub const wxLANGUAGE_PORTUGUESE_BRAZILIAN: u32 = 0 + 155;
pub const wxLANGUAGE_PUNJABI: u32 = 0 + 156;
pub const wxLANGUAGE_QUECHUA: u32 = 0 + 157;
pub const wxLANGUAGE_RHAETO_ROMANCE: u32 = 0 + 158;
pub const wxLANGUAGE_ROMANIAN: u32 = 0 + 159;
pub const wxLANGUAGE_RUSSIAN: u32 = 0 + 160;
pub const wxLANGUAGE_RUSSIAN_UKRAINE: u32 = 0 + 161;
pub const wxLANGUAGE_SAMI: u32 = 0 + 162;
pub const wxLANGUAGE_SAMOAN: u32 = 0 + 163;
pub const wxLANGUAGE_SANGHO: u32 = 0 + 164;
pub const wxLANGUAGE_SANSKRIT: u32 = 0 + 165;
pub const wxLANGUAGE_SCOTS_GAELIC: u32 = 0 + 166;
pub const wxLANGUAGE_SERBIAN: u32 = 0 + 167;
pub const wxLANGUAGE_SERBIAN_CYRILLIC: u32 = 0 + 168;
pub const wxLANGUAGE_SERBIAN_LATIN: u32 = 0 + 169;
pub const wxLANGUAGE_SERBO_CROATIAN: u32 = 0 + 170;
pub const wxLANGUAGE_SESOTHO: u32 = 0 + 171;
pub const wxLANGUAGE_SETSWANA: u32 = 0 + 172;
pub const wxLANGUAGE_SHONA: u32 = 0 + 173;
pub const wxLANGUAGE_SINDHI: u32 = 0 + 174;
pub const wxLANGUAGE_SINHALESE: u32 = 0 + 175;
pub const wxLANGUAGE_SISWATI: u32 = 0 + 176;
pub const wxLANGUAGE_SLOVAK: u32 = 0 + 177;
pub const wxLANGUAGE_SLOVENIAN: u32 = 0 + 178;
pub const wxLANGUAGE_SOMALI: u32 = 0 + 179;
pub const wxLANGUAGE_SPANISH: u32 = 0 + 180;
pub const wxLANGUAGE_SPANISH_ARGENTINA: u32 = 0 + 181;
pub const wxLANGUAGE_SPANISH_BOLIVIA: u32 = 0 + 182;
pub const wxLANGUAGE_SPANISH_CHILE: u32 = 0 + 183;
pub const wxLANGUAGE_SPANISH_COLOMBIA: u32 = 0 + 184;
pub const wxLANGUAGE_SPANISH_COSTA_RICA: u32 = 0 + 185;
pub const wxLANGUAGE_SPANISH_DOMINICAN_REPUBLIC: u32 = 0 + 186;
pub const wxLANGUAGE_SPANISH_ECUADOR: u32 = 0 + 187;
pub const wxLANGUAGE_SPANISH_EL_SALVADOR: u32 = 0 + 188;
pub const wxLANGUAGE_SPANISH_GUATEMALA: u32 = 0 + 189;
pub const wxLANGUAGE_SPANISH_HONDURAS: u32 = 0 + 190;
pub const wxLANGUAGE_SPANISH_MEXICAN: u32 = 0 + 191;
pub const wxLANGUAGE_SPANISH_MODERN: u32 = 0 + 192;
pub const wxLANGUAGE_SPANISH_NICARAGUA: u32 = 0 + 193;
pub const wxLANGUAGE_SPANISH_PANAMA: u32 = 0 + 194;
pub const wxLANGUAGE_SPANISH_PARAGUAY: u32 = 0 + 195;
pub const wxLANGUAGE_SPANISH_PERU: u32 = 0 + 196;
pub const wxLANGUAGE_SPANISH_PUERTO_RICO: u32 = 0 + 197;
pub const wxLANGUAGE_SPANISH_URUGUAY: u32 = 0 + 198;
pub const wxLANGUAGE_SPANISH_US: u32 = 0 + 199;
pub const wxLANGUAGE_SPANISH_VENEZUELA: u32 = 0 + 200;
pub const wxLANGUAGE_SUNDANESE: u32 = 0 + 201;
pub const wxLANGUAGE_SWAHILI: u32 = 0 + 202;
pub const wxLANGUAGE_SWEDISH: u32 = 0 + 203;
pub const wxLANGUAGE_SWEDISH_FINLAND: u32 = 0 + 204;
pub const wxLANGUAGE_TAGALOG: u32 = 0 + 205;
pub const wxLANGUAGE_TAJIK: u32 = 0 + 206;
pub const wxLANGUAGE_TAMIL: u32 = 0 + 207;
pub const wxLANGUAGE_TATAR: u32 = 0 + 208;
pub const wxLANGUAGE_TELUGU: u32 = 0 + 209;
pub const wxLANGUAGE_THAI: u32 = 0 + 210;
pub const wxLANGUAGE_TIBETAN: u32 = 0 + 211;
pub const wxLANGUAGE_TIGRINYA: u32 = 0 + 212;
pub const wxLANGUAGE_TONGA: u32 = 0 + 213;
pub const wxLANGUAGE_TSONGA: u32 = 0 + 214;
pub const wxLANGUAGE_TURKISH: u32 = 0 + 215;
pub const wxLANGUAGE_TURKMEN: u32 = 0 + 216;
pub const wxLANGUAGE_TWI: u32 = 0 + 217;
pub const wxLANGUAGE_UIGHUR: u32 = 0 + 218;
pub const wxLANGUAGE_UKRAINIAN: u32 = 0 + 219;
pub const wxLANGUAGE_URDU: u32 = 0 + 220;
pub const wxLANGUAGE_URDU_INDIA: u32 = 0 + 221;
pub const wxLANGUAGE_URDU_PAKISTAN: u32 = 0 + 222;
pub const wxLANGUAGE_UZBEK: u32 = 0 + 223;
pub const wxLANGUAGE_UZBEK_CYRILLIC: u32 = 0 + 224;
pub const wxLANGUAGE_UZBEK_LATIN: u32 = 0 + 225;
pub const wxLANGUAGE_VALENCIAN: u32 = 0 + 226;
pub const wxLANGUAGE_VIETNAMESE: u32 = 0 + 227;
pub const wxLANGUAGE_VOLAPUK: u32 = 0 + 228;
pub const wxLANGUAGE_WELSH: u32 = 0 + 229;
pub const wxLANGUAGE_WOLOF: u32 = 0 + 230;
pub const wxLANGUAGE_XHOSA: u32 = 0 + 231;
pub const wxLANGUAGE_YIDDISH: u32 = 0 + 232;
pub const wxLANGUAGE_YORUBA: u32 = 0 + 233;
pub const wxLANGUAGE_ZHUANG: u32 = 0 + 234;
pub const wxLANGUAGE_ZULU: u32 = 0 + 235;
pub const wxLANGUAGE_USER_DEFINED: u32 = 0 + 236;
pub const wxLANGUAGE_CAMBODIAN: u32 = wxLANGUAGE_KHMER;

pub const wxDVC_DEFAULT_RENDERER_SIZE: u32 = 20;
pub const wxDVC_DEFAULT_WIDTH: u32 = 80;
pub const wxDVC_TOGGLE_DEFAULT_WIDTH: u32 = 30;
pub const wxDVC_DEFAULT_MINWIDTH: u32 = 30;
pub const wxDVR_DEFAULT_ALIGNMENT: i32 = -1;
pub const wxDV_SINGLE: u32 = 0x0000;
pub const wxDV_MULTIPLE: u32 = 0x0001;
pub const wxDV_NO_HEADER: u32 = 0x0002;
pub const wxDV_HORIZ_RULES: u32 = 0x0004;
pub const wxDV_VERT_RULES: u32 = 0x0008;
pub const wxDV_ROW_LINES: u32 = 0x0010;
pub const wxDV_VARIABLE_LINE_HEIGHT: u32 = 0x0020;
//  ENUM: wxDataViewCellMode
pub const wxDATAVIEW_CELL_INERT: u32 = 0;
pub const wxDATAVIEW_CELL_ACTIVATABLE: u32 = 0 + 1;
pub const wxDATAVIEW_CELL_EDITABLE: u32 = 0 + 2;
//  ENUM: wxDataViewCellRenderState
pub const wxDATAVIEW_CELL_SELECTED: u32 = 1;
pub const wxDATAVIEW_CELL_PRELIT: u32 = 2;
pub const wxDATAVIEW_CELL_INSENSITIVE: u32 = 4;
pub const wxDATAVIEW_CELL_FOCUSED: u32 = 8;
//  ENUM: wxDataViewColumnFlags
pub const wxDATAVIEW_COL_RESIZABLE: u32 = 1;
pub const wxDATAVIEW_COL_SORTABLE: u32 = 2;
pub const wxDATAVIEW_COL_REORDERABLE: u32 = 4;
pub const wxDATAVIEW_COL_HIDDEN: u32 = 8;

//  ENUM: wxBrushStyle
pub const wxBRUSHSTYLE_INVALID: i32 = -1;
pub const wxBRUSHSTYLE_SOLID: i32 = wxSOLID;
pub const wxBRUSHSTYLE_TRANSPARENT: i32 = wxTRANSPARENT;
pub const wxBRUSHSTYLE_STIPPLE_MASK_OPAQUE: i32 = wxSTIPPLE_MASK_OPAQUE;
pub const wxBRUSHSTYLE_STIPPLE_MASK: i32 = wxSTIPPLE_MASK;
pub const wxBRUSHSTYLE_STIPPLE: i32 = wxSTIPPLE;
pub const wxBRUSHSTYLE_BDIAGONAL_HATCH: i32 = wxSTIPPLE + 1;
pub const wxBRUSHSTYLE_CROSSDIAG_HATCH: i32 = wxSTIPPLE + 2;
pub const wxBRUSHSTYLE_FDIAGONAL_HATCH: i32 = wxSTIPPLE + 3;
pub const wxBRUSHSTYLE_CROSS_HATCH: i32 = wxSTIPPLE + 4;
pub const wxBRUSHSTYLE_HORIZONTAL_HATCH: i32 = wxSTIPPLE + 5;
pub const wxBRUSHSTYLE_VERTICAL_HATCH: i32 = wxSTIPPLE + 6;
pub const wxBRUSHSTYLE_FIRST_HATCH: i32 = wxSTIPPLE + 7;
pub const wxBRUSHSTYLE_LAST_HATCH: i32 = wxSTIPPLE + 8;

//  ENUM: wxURLError
pub const wxURL_NOERR: u32 = 0;
pub const wxURL_SNTXERR: u32 = 0 + 1;
pub const wxURL_NOPROTO: u32 = 0 + 2;
pub const wxURL_NOHOST: u32 = 0 + 3;
pub const wxURL_NOPATH: u32 = 0 + 4;
pub const wxURL_CONNERR: u32 = 0 + 5;
pub const wxURL_PROTOERR: u32 = 0 + 6;

pub const wxLB_DEFAULT: u32 = wxBK_DEFAULT;
pub const wxLB_TOP: u32 = wxBK_TOP;
pub const wxLB_BOTTOM: u32 = wxBK_BOTTOM;
pub const wxLB_LEFT: u32 = wxBK_LEFT;
pub const wxLB_RIGHT: u32 = wxBK_RIGHT;
pub const wxLB_ALIGN_MASK: u32 = wxBK_ALIGN_MASK;

//  SKIP: wxPG_NULL_BITMAP
//  SKIP: wxPG_INVALID_VALUE
pub const wxPG_BASE_OCT: u32 = 8;
pub const wxPG_BASE_DEC: u32 = 10;
pub const wxPG_BASE_HEX: u32 = 16;
pub const wxPG_BASE_HEXL: u32 = 32;
pub const wxPG_PREFIX_NONE: u32 = 0;
pub const wxPG_PREFIX_0x: u32 = 1;
pub const wxPG_PREFIX_DOLLAR_SIGN: u32 = 2;
//  ENUM: wxPG_GETPROPERTYVALUES_FLAGS
//  ENUM: wxPG_MISC_ARG_FLAGS
pub const wxPG_FULL_VALUE: u32 = 0x00000001;
pub const wxPG_REPORT_ERROR: u32 = 0x00000002;
pub const wxPG_PROPERTY_SPECIFIC: u32 = 0x00000004;
pub const wxPG_EDITABLE_VALUE: u32 = 0x00000008;
pub const wxPG_COMPOSITE_FRAGMENT: u32 = 0x00000010;
pub const wxPG_UNEDITABLE_COMPOSITE_FRAGMENT: u32 = 0x00000020;
pub const wxPG_VALUE_IS_CURRENT: u32 = 0x00000040;
pub const wxPG_PROGRAMMATIC_VALUE: u32 = 0x00000080;
//  ENUM: wxPG_SETVALUE_FLAGS
pub const wxPG_SETVAL_REFRESH_EDITOR: u32 = 0x0001;
pub const wxPG_SETVAL_AGGREGATED: u32 = 0x0002;
pub const wxPG_SETVAL_FROM_PARENT: u32 = 0x0004;
pub const wxPG_SETVAL_BY_USER: u32 = 0x0008;

//  ENUM: wxTextValidatorStyle
pub const wxFILTER_NONE: u32 = 0;
pub const wxFILTER_EMPTY: u32 = 0 + 1;
pub const wxFILTER_ASCII: u32 = 0 + 2;
pub const wxFILTER_ALPHA: u32 = 0 + 3;
pub const wxFILTER_ALPHANUMERIC: u32 = 0 + 4;
pub const wxFILTER_DIGITS: u32 = 0 + 5;
pub const wxFILTER_NUMERIC: u32 = 0 + 6;
pub const wxFILTER_INCLUDE_LIST: u32 = 0 + 7;
pub const wxFILTER_INCLUDE_CHAR_LIST: u32 = 0 + 8;
pub const wxFILTER_EXCLUDE_LIST: u32 = 0 + 9;
pub const wxFILTER_EXCLUDE_CHAR_LIST: u32 = 0 + 10;
pub const wxFILTER_XDIGITS: u32 = 0 + 11;
pub const wxFILTER_SPACE: u32 = 0 + 12;

//  ENUM: EditableStateFlags
pub const SelectionState: u32 = 0x01;
pub const ExpandedState: u32 = 0x02;
pub const ScrollPosState: u32 = 0x04;
pub const PageState: u32 = 0x08;
pub const SplitterPosState: u32 = 0x10;
pub const DescBoxState: u32 = 0x20;
pub const AllStates: u32 = SelectionState |
                           ExpandedState |
                           ScrollPosState |
                           PageState |
                           SplitterPosState |
                           DescBoxState;

//  ENUM: wxAttrKind
pub const Any: u32 = 0;
pub const Cell: u32 = 0 + 1;
pub const Row: u32 = 0 + 2;
pub const Col: u32 = 0 + 3;
pub const Default: u32 = 0 + 4;
pub const Merged: u32 = 0 + 5;

//  ENUM: wxShowEffect
pub const wxSHOW_EFFECT_NONE: u32 = 0;
pub const wxSHOW_EFFECT_ROLL_TO_LEFT: u32 = 0 + 1;
pub const wxSHOW_EFFECT_ROLL_TO_RIGHT: u32 = 0 + 2;
pub const wxSHOW_EFFECT_ROLL_TO_TOP: u32 = 0 + 3;
pub const wxSHOW_EFFECT_ROLL_TO_BOTTOM: u32 = 0 + 4;
pub const wxSHOW_EFFECT_SLIDE_TO_LEFT: u32 = 0 + 5;
pub const wxSHOW_EFFECT_SLIDE_TO_RIGHT: u32 = 0 + 6;
pub const wxSHOW_EFFECT_SLIDE_TO_TOP: u32 = 0 + 7;
pub const wxSHOW_EFFECT_SLIDE_TO_BOTTOM: u32 = 0 + 8;
pub const wxSHOW_EFFECT_BLEND: u32 = 0 + 9;
pub const wxSHOW_EFFECT_EXPAND: u32 = 0 + 10;
pub const wxSHOW_EFFECT_MAX: u32 = 0 + 11;
//  ENUM: @55
pub const wxTOUCH_NONE: u32 = 0;
pub const wxTOUCH_VERTICAL_PAN_GESTURE: u32 = 0 + 1;
pub const wxTOUCH_HORIZONTAL_PAN_GESTURE: u32 = 0 + 2;
pub const wxTOUCH_PAN_GESTURES: u32 = 0 + 3;
pub const wxTOUCH_ZOOM_GESTURE: u32 = 0 + 4;
pub const wxTOUCH_ROTATE_GESTURE: u32 = 0 + 5;
pub const wxTOUCH_PRESS_GESTURES: u32 = 0 + 6;
pub const wxTOUCH_ALL_GESTURES: u32 = 0 + 7;
//  ENUM: @56
pub const wxSEND_EVENT_POST: u32 = 1;
//  ENUM: wxWindowVariant
pub const wxWINDOW_VARIANT_NORMAL: u32 = 0;
pub const wxWINDOW_VARIANT_SMALL: u32 = 0 + 1;
pub const wxWINDOW_VARIANT_MINI: u32 = 0 + 2;
pub const wxWINDOW_VARIANT_LARGE: u32 = 0 + 3;
pub const wxWINDOW_VARIANT_MAX: u32 = 0 + 4;

//  ENUM: @7
pub const wxCC_SPECIAL_DCLICK: u32 = 0x0100;
pub const wxCC_STD_BUTTON: u32 = 0x0200;

//  ENUM: @41
pub const wxRE_EXTENDED: u32 = 0;
pub const wxRE_ADVANCED: u32 = 1;
pub const wxRE_BASIC: u32 = 2;
pub const wxRE_ICASE: u32 = 4;
pub const wxRE_NOSUB: u32 = 8;
pub const wxRE_NEWLINE: u32 = 16;
pub const wxRE_DEFAULT: u32 = wxRE_EXTENDED;
//  ENUM: @42
pub const wxRE_NOTBOL: u32 = 32;
pub const wxRE_NOTEOL: u32 = 64;

//  ENUM: EntryType
pub const Type_Unknown: u32 = 0;
pub const Type_Boolean: u32 = 0 + 1;
pub const Type_Integer: u32 = 0 + 2;
pub const Type_Float: u32 = 0 + 3;

//  ENUM: wxOwnerDrawnComboBoxPaintingFlags
pub const wxODCB_PAINTING_CONTROL: u32 = 0x0001;
pub const wxODCB_PAINTING_SELECTED: u32 = 0x0002;
//  ENUM: @39
pub const wxODCB_DCLICK_CYCLES: u32 = wxCC_SPECIAL_DCLICK;
pub const wxODCB_STD_CONTROL_PAINT: u32 = 0x1000;

//  ENUM: wxOutCode
pub const wxInside: u32 = 0x00;
pub const wxOutLeft: u32 = 0x01;
pub const wxOutRight: u32 = 0x02;
pub const wxOutTop: u32 = 0x08;
pub const wxOutBottom: u32 = 0x04;

//  ENUM: @38
pub const Timeout_Auto: i32 = -1;
pub const Timeout_Never: i32 = 0;

//  ENUM: @58
pub const wxEXTEND_LAST_ON_EACH_LINE: u32 = 0;
pub const wxREMOVE_LEADING_SPACES: u32 = 0 + 1;
pub const wxWRAPSIZER_DEFAULT_FLAGS: u32 = 0 + 2;

//  ENUM: wxIPCFormat
pub const wxIPC_INVALID: u32 =     0;
pub const wxIPC_TEXT: u32 =        1;
pub const wxIPC_BITMAP: u32 =      2;
pub const wxIPC_METAFILE: u32 =    3;
pub const wxIPC_SYLK: u32 =        4;
pub const wxIPC_DIF: u32 =         5;
pub const wxIPC_TIFF: u32 =        6;
pub const wxIPC_OEMTEXT: u32 =     7;
pub const wxIPC_DIB: u32 =         8;
pub const wxIPC_PALETTE: u32 =     9;
pub const wxIPC_PENDATA: u32 =     10;
pub const wxIPC_RIFF: u32 =        11;
pub const wxIPC_WAVE: u32 =        12;
pub const wxIPC_UTF16TEXT: u32 =   13;
pub const wxIPC_ENHMETAFILE: u32 = 14;
pub const wxIPC_FILENAME: u32 =    15;
pub const wxIPC_LOCALE: u32 =      16;
pub const wxIPC_UTF8TEXT: u32 =    17;
pub const wxIPC_UTF32TEXT: u32 =   18;
pub const wxIPC_UNICODETEXT: u32 = wxIPC_UTF16TEXT;
pub const wxIPC_PRIVATE: u32 =     20;

pub const wxXML_NO_INDENTATION: i32 = (-1);
//  ENUM: wxXmlNodeType
pub const wxXML_ELEMENT_NODE: u32 =  1;
pub const wxXML_ATTRIBUTE_NODE: u32 =  2;
pub const wxXML_TEXT_NODE: u32 =  3;
pub const wxXML_CDATA_SECTION_NODE: u32 =  4;
pub const wxXML_ENTITY_REF_NODE: u32 =  5;
pub const wxXML_ENTITY_NODE: u32 =  6;
pub const wxXML_PI_NODE: u32 =  7;
pub const wxXML_COMMENT_NODE: u32 =  8;
pub const wxXML_DOCUMENT_NODE: u32 =  9;
pub const wxXML_DOCUMENT_TYPE_NODE: u32 = 10;
pub const wxXML_DOCUMENT_FRAG_NODE: u32 = 11;
pub const wxXML_NOTATION_NODE: u32 = 12;
pub const wxXML_HTML_DOCUMENT_NODE: u32 = 13;
//  ENUM: wxXmlDocumentLoadFlag
pub const wxXMLDOC_NONE: u32 = 0;
pub const wxXMLDOC_KEEP_WHITESPACE_NODES: u32 = 0 + 1;

//  ENUM: wxIPCFormat

//  ENUM: @9
pub const ShowBelow: u32 = 0x0000;
pub const ShowAbove: u32 = 0x0001;
pub const CanDeferShow: u32 = 0x0002;

pub const wxHL_CONTEXTMENU: u32 = 0x0001;
pub const wxHL_ALIGN_LEFT: u32 = 0x0002;
pub const wxHL_ALIGN_RIGHT: u32 = 0x0004;
pub const wxHL_ALIGN_CENTRE: u32 = 0x0008;
pub const wxHL_DEFAULT_STYLE: u32 = (wxHL_CONTEXTMENU|wxNO_BORDER|wxHL_ALIGN_CENTRE);

pub const wxPD_CAN_ABORT: u32 = 0x0001;
pub const wxPD_APP_MODAL: u32 = 0x0002;
pub const wxPD_AUTO_HIDE: u32 = 0x0004;
pub const wxPD_ELAPSED_TIME: u32 = 0x0008;
pub const wxPD_ESTIMATED_TIME: u32 = 0x0010;
pub const wxPD_SMOOTH: u32 = 0x0020;
pub const wxPD_REMAINING_TIME: u32 = 0x0040;
pub const wxPD_CAN_SKIP: u32 = 0x0080;

pub const wxPG_COLOUR_WEB_BASE: u32 = 0x10000;
pub const wxPG_COLOUR_CUSTOM: u32 = 0xFFFFFF;
pub const wxPG_COLOUR_UNSPECIFIED: u32 = (wxPG_COLOUR_CUSTOM+1);
pub const wxPG_PROP_TRANSLATE_CUSTOM: u32 = wxPG_PROP_CLASS_SPECIFIC_1;

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
pub const wxIMAGE_OPTION_GIF_TRANSPARENCY: &str = "Transparency";
pub const wxIMAGE_OPTION_GIF_TRANSPARENCY_HIGHLIGHT: &str = "Highlight";
pub const wxIMAGE_OPTION_GIF_TRANSPARENCY_UNCHANGED: &str = "Unchanged";
pub const wxIMAGE_OPTION_PNG_FORMAT: &str = "PngFormat";
pub const wxIMAGE_OPTION_PNG_BITDEPTH: &str = "PngBitDepth";
pub const wxIMAGE_OPTION_PNG_FILTER: &str = "PngF";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_LEVEL: &str = "PngZL";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_MEM_LEVEL: &str = "PngZM";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_STRATEGY: &str = "PngZS";
pub const wxIMAGE_OPTION_PNG_COMPRESSION_BUFFER_SIZE: &str = "PngZB";
pub const wxIMAGE_OPTION_TIFF_BITSPERSAMPLE: &str = "BitsPerSample";
pub const wxIMAGE_OPTION_TIFF_SAMPLESPERPIXEL: &str = "SamplesPerPixel";
pub const wxIMAGE_OPTION_TIFF_COMPRESSION: &str = "Compression";
pub const wxIMAGE_OPTION_TIFF_PHOTOMETRIC: &str = "Photometric";
pub const wxIMAGE_OPTION_TIFF_IMAGEDESCRIPTOR: &str = "ImageDescriptor";
//  ENUM: wxImageResolution
pub const wxIMAGE_RESOLUTION_NONE: u32 = 0;
pub const wxIMAGE_RESOLUTION_INCHES: u32 = 1;
pub const wxIMAGE_RESOLUTION_CM: u32 = 2;
//  ENUM: wxImageResizeQuality
pub const wxIMAGE_QUALITY_NEAREST: u32 = 0;
pub const wxIMAGE_QUALITY_BILINEAR: u32 = 0 + 1;
pub const wxIMAGE_QUALITY_BICUBIC: u32 = 0 + 2;
pub const wxIMAGE_QUALITY_BOX_AVERAGE: u32 = 0 + 3;
pub const wxIMAGE_QUALITY_NORMAL: u32 = 0 + 4;
pub const wxIMAGE_QUALITY_HIGH: u32 = 0 + 5;
//  ENUM: wxImageAlphaBlendMode
pub const wxIMAGE_ALPHA_BLEND_OVER: u32 = 0;
pub const wxIMAGE_ALPHA_BLEND_COMPOSE: u32 = 1;
//  ENUM: wxImagePNGType
pub const wxPNG_TYPE_COLOUR: u32 = 0;
pub const wxPNG_TYPE_GREY: u32 = 2;
pub const wxPNG_TYPE_GREY_RED: u32 = 3;
pub const wxPNG_TYPE_PALETTE: u32 = 4;
//  ENUM: @30
pub const wxBMP_24BPP: u32 = 24;
pub const wxBMP_8BPP: u32 =  8;
pub const wxBMP_8BPP_GREY: u32 =  9;
pub const wxBMP_8BPP_GRAY: u32 =  wxBMP_8BPP_GREY;
pub const wxBMP_8BPP_RED: u32 = 10;
pub const wxBMP_8BPP_PALETTE: u32 = 11;
pub const wxBMP_4BPP: u32 =  4;
pub const wxBMP_1BPP: u32 =  1;
pub const wxBMP_1BPP_BW: u32 =  2;

//  ENUM: wxAuiPaneDockArtSetting
pub const wxAUI_DOCKART_SASH_SIZE: u32 = 0;
pub const wxAUI_DOCKART_CAPTION_SIZE: u32 = 1;
pub const wxAUI_DOCKART_GRIPPER_SIZE: u32 = 2;
pub const wxAUI_DOCKART_PANE_BORDER_SIZE: u32 = 3;
pub const wxAUI_DOCKART_PANE_BUTTON_SIZE: u32 = 4;
pub const wxAUI_DOCKART_BACKGROUND_COLOUR: u32 = 5;
pub const wxAUI_DOCKART_SASH_COLOUR: u32 = 6;
pub const wxAUI_DOCKART_ACTIVE_CAPTION_COLOUR: u32 = 7;
pub const wxAUI_DOCKART_ACTIVE_CAPTION_GRADIENT_COLOUR: u32 = 8;
pub const wxAUI_DOCKART_INACTIVE_CAPTION_COLOUR: u32 = 9;
pub const wxAUI_DOCKART_INACTIVE_CAPTION_GRADIENT_COLOUR: u32 = 10;
pub const wxAUI_DOCKART_ACTIVE_CAPTION_TEXT_COLOUR: u32 = 11;
pub const wxAUI_DOCKART_INACTIVE_CAPTION_TEXT_COLOUR: u32 = 12;
pub const wxAUI_DOCKART_BORDER_COLOUR: u32 = 13;
pub const wxAUI_DOCKART_GRIPPER_COLOUR: u32 = 14;
pub const wxAUI_DOCKART_CAPTION_FONT: u32 = 15;
pub const wxAUI_DOCKART_GRADIENT_TYPE: u32 = 16;
//  ENUM: wxAuiPaneDockArtGradients
pub const wxAUI_GRADIENT_NONE: u32 = 0;
pub const wxAUI_GRADIENT_VERTICAL: u32 = 1;
pub const wxAUI_GRADIENT_HORIZONTAL: u32 = 2;
//  ENUM: wxAuiPaneButtonState
pub const wxAUI_BUTTON_STATE_NORMAL: u32 = 0;
pub const wxAUI_BUTTON_STATE_HOVER: u32 = 1 << 1;
pub const wxAUI_BUTTON_STATE_PRESSED: u32 = 1 << 2;
pub const wxAUI_BUTTON_STATE_DISABLED: u32 = 1 << 3;
pub const wxAUI_BUTTON_STATE_HIDDEN: u32 = 1 << 4;
pub const wxAUI_BUTTON_STATE_CHECKED: u32 = 1 << 5;
//  ENUM: wxAuiButtonId
pub const wxAUI_BUTTON_CLOSE: u32 = 101;
pub const wxAUI_BUTTON_MAXIMIZE_RESTORE: u32 = 102;
pub const wxAUI_BUTTON_MINIMIZE: u32 = 103;
pub const wxAUI_BUTTON_PIN: u32 = 104;
pub const wxAUI_BUTTON_OPTIONS: u32 = 105;
pub const wxAUI_BUTTON_WINDOWLIST: u32 = 106;
pub const wxAUI_BUTTON_LEFT: u32 = 107;
pub const wxAUI_BUTTON_RIGHT: u32 = 108;
pub const wxAUI_BUTTON_UP: u32 = 109;
pub const wxAUI_BUTTON_DOWN: u32 = 110;
pub const wxAUI_BUTTON_CUSTOM1: u32 = 201;
pub const wxAUI_BUTTON_CUSTOM2: u32 = 202;
pub const wxAUI_BUTTON_CUSTOM3: u32 = 203;

//  ENUM: wxEOL
pub const wxEOL_NATIVE: u32 = 0;
pub const wxEOL_UNIX: u32 = 0 + 1;
pub const wxEOL_MAC: u32 = 0 + 2;
pub const wxEOL_DOS: u32 = 0 + 3;

//  ENUM: wxDirTraverseResult
pub const wxDIR_IGNORE: i32 = -1;
pub const wxDIR_STOP: i32 = -1 + 1;
pub const wxDIR_CONTINUE: i32 = -1 + 2;
//  ENUM: wxDirFlags
pub const wxDIR_FILES: u32 = 0x0001;
pub const wxDIR_DIRS: u32 = 0x0002;
pub const wxDIR_HIDDEN: u32 = 0x0004;
pub const wxDIR_DOTDOT: u32 = 0x0008;
pub const wxDIR_NO_FOLLOW: u32 = 0x0010;
pub const wxDIR_DEFAULT: u32 = wxDIR_FILES | wxDIR_DIRS | wxDIR_HIDDEN;

pub const wxBITMAP_SCREEN_DEPTH: i32 = (-1);

//  ENUM: wxRibbonPanelOption
pub const wxRIBBON_PANEL_NO_AUTO_MINIMISE: u32 = 0;
pub const wxRIBBON_PANEL_EXT_BUTTON: u32 = 0 + 1;
pub const wxRIBBON_PANEL_MINIMISE_BUTTON: u32 = 0 + 2;
pub const wxRIBBON_PANEL_STRETCH: u32 = 0 + 3;
pub const wxRIBBON_PANEL_FLEXIBLE: u32 = 0 + 4;
pub const wxRIBBON_PANEL_DEFAULT_STYLE: u32 = 0 + 5;

//  ENUM: State
pub const State_Idle: u32 = 0;
pub const State_Unauthorized: u32 = 0 + 1;
pub const State_Active: u32 = 0 + 2;
pub const State_Completed: u32 = 0 + 3;
pub const State_Failed: u32 = 0 + 4;
pub const State_Cancelled: u32 = 0 + 5;
//  ENUM: Storage
pub const Storage_Memory: u32 = 0;
pub const Storage_File: u32 = 0 + 1;
pub const Storage_None: u32 = 0 + 2;

//  ENUM: wxAuiToolBarStyle
pub const wxAUI_TB_TEXT: u32 = 1 << 0;
pub const wxAUI_TB_NO_TOOLTIPS: u32 = 1 << 1;
pub const wxAUI_TB_NO_AUTORESIZE: u32 = 1 << 2;
pub const wxAUI_TB_GRIPPER: u32 = 1 << 3;
pub const wxAUI_TB_OVERFLOW: u32 = 1 << 4;
pub const wxAUI_TB_VERTICAL: u32 = 1 << 5;
pub const wxAUI_TB_HORZ_LAYOUT: u32 = 1 << 6;
pub const wxAUI_TB_HORIZONTAL: u32 = 1 << 7;
pub const wxAUI_TB_PLAIN_BACKGROUND: u32 = 1 << 8;
pub const wxAUI_TB_HORZ_TEXT: u32 = (wxAUI_TB_HORZ_LAYOUT | wxAUI_TB_TEXT);
pub const wxAUI_ORIENTATION_MASK: u32 = (wxAUI_TB_VERTICAL | wxAUI_TB_HORIZONTAL);
pub const wxAUI_TB_DEFAULT_STYLE: u32 = 0;
//  ENUM: wxAuiToolBarArtSetting
pub const wxAUI_TBART_SEPARATOR_SIZE: u32 = 0;
pub const wxAUI_TBART_GRIPPER_SIZE: u32 = 1;
//  SKIP: wxAUI_TBART_OVERFLOW_SIZE
//  ENUM: wxAuiToolBarToolTextOrientation
pub const wxAUI_TBTOOL_TEXT_LEFT: u32 = 0;
pub const wxAUI_TBTOOL_TEXT_RIGHT: u32 = 1;
pub const wxAUI_TBTOOL_TEXT_TOP: u32 = 2;
pub const wxAUI_TBTOOL_TEXT_BOTTOM: u32 = 3;

//  ENUM: Origin
pub const Origin_Unknown: u32 = 0;
pub const Origin_Keyboard: u32 = 0 + 1;
pub const Origin_HelpButton: u32 = 0 + 2;


//  ENUM: Source
pub const Source_Server: u32 = 0;
pub const Source_Proxy: u32 = 0 + 1;

pub const wxCHOICE_WIDTH: u32 = 150;
pub const wxCHOICE_HEIGHT: u32 = 200;
pub const wxCHOICEDLG_STYLE: u32 = (wxDEFAULT_DIALOG_STYLE | wxOK | wxCANCEL | wxCENTRE | wxRESIZE_BORDER);

//  ENUM: wxAuiToolBarStyle
//  ENUM: wxAuiToolBarArtSetting
//  ENUM: wxAuiToolBarToolTextOrientation
//  ENUM: wxAuiPaneDockArtSetting

//  ENUM: wxRibbonArtSetting
pub const wxRIBBON_ART_TAB_SEPARATION_SIZE: u32 = 0;
pub const wxRIBBON_ART_PAGE_BORDER_LEFT_SIZE: u32 = 0 + 1;
pub const wxRIBBON_ART_PAGE_BORDER_TOP_SIZE: u32 = 0 + 2;
pub const wxRIBBON_ART_PAGE_BORDER_RIGHT_SIZE: u32 = 0 + 3;
pub const wxRIBBON_ART_PAGE_BORDER_BOTTOM_SIZE: u32 = 0 + 4;
pub const wxRIBBON_ART_PANEL_X_SEPARATION_SIZE: u32 = 0 + 5;
pub const wxRIBBON_ART_PANEL_Y_SEPARATION_SIZE: u32 = 0 + 6;
pub const wxRIBBON_ART_TOOL_GROUP_SEPARATION_SIZE: u32 = 0 + 7;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_LEFT_SIZE: u32 = 0 + 8;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_RIGHT_SIZE: u32 = 0 + 9;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_TOP_SIZE: u32 = 0 + 10;
pub const wxRIBBON_ART_GALLERY_BITMAP_PADDING_BOTTOM_SIZE: u32 = 0 + 11;
pub const wxRIBBON_ART_PANEL_LABEL_FONT: u32 = 0 + 12;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_FONT: u32 = 0 + 13;
pub const wxRIBBON_ART_TAB_LABEL_FONT: u32 = 0 + 14;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_COLOUR: u32 = 0 + 15;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_DISABLED_COLOUR: u32 = 0 + 16;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BORDER_COLOUR: u32 = 0 + 17;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_COLOUR: u32 = 0 + 18;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 19;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_COLOUR: u32 = 0 + 20;
pub const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 21;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BORDER_COLOUR: u32 = 0 + 22;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_COLOUR: u32 = 0 + 23;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 24;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_COLOUR: u32 = 0 + 25;
pub const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 26;
pub const wxRIBBON_ART_GALLERY_BORDER_COLOUR: u32 = 0 + 27;
pub const wxRIBBON_ART_GALLERY_HOVER_BACKGROUND_COLOUR: u32 = 0 + 28;
pub const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_COLOUR: u32 = 0 + 29;
pub const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 30;
pub const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_TOP_COLOUR: u32 = 0 + 31;
pub const wxRIBBON_ART_GALLERY_BUTTON_FACE_COLOUR: u32 = 0 + 32;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_COLOUR: u32 = 0 + 33;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 34;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_TOP_COLOUR: u32 = 0 + 35;
pub const wxRIBBON_ART_GALLERY_BUTTON_HOVER_FACE_COLOUR: u32 = 0 + 36;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_COLOUR: u32 = 0 + 37;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 38;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_TOP_COLOUR: u32 = 0 + 39;
pub const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_FACE_COLOUR: u32 = 0 + 40;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_COLOUR: u32 = 0 + 41;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 42;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_TOP_COLOUR: u32 = 0 + 43;
pub const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_FACE_COLOUR: u32 = 0 + 44;
pub const wxRIBBON_ART_GALLERY_ITEM_BORDER_COLOUR: u32 = 0 + 45;
pub const wxRIBBON_ART_TAB_LABEL_COLOUR: u32 = 0 + 46;
pub const wxRIBBON_ART_TAB_ACTIVE_LABEL_COLOUR: u32 = 0 + 47;
pub const wxRIBBON_ART_TAB_HOVER_LABEL_COLOUR: u32 = 0 + 48;
pub const wxRIBBON_ART_TAB_SEPARATOR_COLOUR: u32 = 0 + 49;
pub const wxRIBBON_ART_TAB_SEPARATOR_GRADIENT_COLOUR: u32 = 0 + 50;
pub const wxRIBBON_ART_TAB_CTRL_BACKGROUND_COLOUR: u32 = 0 + 51;
pub const wxRIBBON_ART_TAB_CTRL_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 52;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_TOP_COLOUR: u32 = 0 + 53;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 54;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_COLOUR: u32 = 0 + 55;
pub const wxRIBBON_ART_TAB_HOVER_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 56;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_COLOUR: u32 = 0 + 57;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 58;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_COLOUR: u32 = 0 + 59;
pub const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 60;
pub const wxRIBBON_ART_TAB_BORDER_COLOUR: u32 = 0 + 61;
pub const wxRIBBON_ART_PANEL_BORDER_COLOUR: u32 = 0 + 62;
pub const wxRIBBON_ART_PANEL_BORDER_GRADIENT_COLOUR: u32 = 0 + 63;
pub const wxRIBBON_ART_PANEL_HOVER_BORDER_COLOUR: u32 = 0 + 64;
pub const wxRIBBON_ART_PANEL_HOVER_BORDER_GRADIENT_COLOUR: u32 = 0 + 65;
pub const wxRIBBON_ART_PANEL_MINIMISED_BORDER_COLOUR: u32 = 0 + 66;
pub const wxRIBBON_ART_PANEL_MINIMISED_BORDER_GRADIENT_COLOUR: u32 = 0 + 67;
pub const wxRIBBON_ART_PANEL_LABEL_BACKGROUND_COLOUR: u32 = 0 + 68;
pub const wxRIBBON_ART_PANEL_LABEL_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 69;
pub const wxRIBBON_ART_PANEL_LABEL_COLOUR: u32 = 0 + 70;
pub const wxRIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_COLOUR: u32 = 0 + 71;
pub const wxRIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 72;
pub const wxRIBBON_ART_PANEL_HOVER_LABEL_COLOUR: u32 = 0 + 73;
pub const wxRIBBON_ART_PANEL_MINIMISED_LABEL_COLOUR: u32 = 0 + 74;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_COLOUR: u32 = 0 + 75;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 76;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_COLOUR: u32 = 0 + 77;
pub const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 78;
pub const wxRIBBON_ART_PAGE_BORDER_COLOUR: u32 = 0 + 79;
pub const wxRIBBON_ART_PAGE_BACKGROUND_TOP_COLOUR: u32 = 0 + 80;
pub const wxRIBBON_ART_PAGE_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 81;
pub const wxRIBBON_ART_PAGE_BACKGROUND_COLOUR: u32 = 0 + 82;
pub const wxRIBBON_ART_PAGE_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 83;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_COLOUR: u32 = 0 + 84;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 85;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_COLOUR: u32 = 0 + 86;
pub const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 87;
pub const wxRIBBON_ART_TOOLBAR_BORDER_COLOUR: u32 = 0 + 88;
pub const wxRIBBON_ART_TOOLBAR_HOVER_BORDER_COLOUR: u32 = 0 + 89;
pub const wxRIBBON_ART_TOOLBAR_FACE_COLOUR: u32 = 0 + 90;
pub const wxRIBBON_ART_TOOL_BACKGROUND_TOP_COLOUR: u32 = 0 + 91;
pub const wxRIBBON_ART_TOOL_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 92;
pub const wxRIBBON_ART_TOOL_BACKGROUND_COLOUR: u32 = 0 + 93;
pub const wxRIBBON_ART_TOOL_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 94;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_COLOUR: u32 = 0 + 95;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 96;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_COLOUR: u32 = 0 + 97;
pub const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 98;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_COLOUR: u32 = 0 + 99;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: u32 = 0 + 100;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_COLOUR: u32 = 0 + 101;
pub const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: u32 = 0 + 102;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_COLOUR: u32 = 0 + 103;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_GRADIENT_COLOUR: u32 = 0 + 104;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_TOP_COLOUR: u32 = 0 + 105;
pub const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_GRADIENT_TOP_COLOUR: u32 = 0 + 106;
//  ENUM: wxRibbonScrollButtonStyle
pub const wxRIBBON_SCROLL_BTN_LEFT: u32 = 0;
pub const wxRIBBON_SCROLL_BTN_RIGHT: u32 = 1;
pub const wxRIBBON_SCROLL_BTN_UP: u32 = 2;
pub const wxRIBBON_SCROLL_BTN_DOWN: u32 = 3;
pub const wxRIBBON_SCROLL_BTN_DIRECTION_MASK: u32 = 3;
pub const wxRIBBON_SCROLL_BTN_NORMAL: u32 = 0;
pub const wxRIBBON_SCROLL_BTN_HOVERED: u32 = 4;
pub const wxRIBBON_SCROLL_BTN_ACTIVE: u32 = 8;
pub const wxRIBBON_SCROLL_BTN_STATE_MASK: u32 = 12;
pub const wxRIBBON_SCROLL_BTN_FOR_OTHER: u32 = 0;
pub const wxRIBBON_SCROLL_BTN_FOR_TABS: u32 = 16;
pub const wxRIBBON_SCROLL_BTN_FOR_PAGE: u32 = 32;
pub const wxRIBBON_SCROLL_BTN_FOR_MASK: u32 = 48;
//  ENUM: wxRibbonButtonKind
pub const wxRIBBON_BUTTON_NORMAL: u32 = 1 << 0;
pub const wxRIBBON_BUTTON_DROPDOWN: u32 = 1 << 1;
pub const wxRIBBON_BUTTON_HYBRID: u32 = wxRIBBON_BUTTON_NORMAL | wxRIBBON_BUTTON_DROPDOWN;
pub const wxRIBBON_BUTTON_TOGGLE: u32 = 1 << 2;

//  ENUM: wxWebViewZoom
pub const wxWEBVIEW_ZOOM_TINY: u32 = 0;
pub const wxWEBVIEW_ZOOM_SMALL: u32 = 0 + 1;
pub const wxWEBVIEW_ZOOM_MEDIUM: u32 = 0 + 2;
pub const wxWEBVIEW_ZOOM_LARGE: u32 = 0 + 3;
pub const wxWEBVIEW_ZOOM_LARGEST: u32 = 0 + 4;
//  ENUM: wxWebViewZoomType
pub const wxWEBVIEW_ZOOM_TYPE_LAYOUT: u32 = 0;
pub const wxWEBVIEW_ZOOM_TYPE_TEXT: u32 = 0 + 1;
//  ENUM: wxWebViewNavigationError
pub const wxWEBVIEW_NAV_ERR_CONNECTION: u32 = 0;
pub const wxWEBVIEW_NAV_ERR_CERTIFICATE: u32 = 0 + 1;
pub const wxWEBVIEW_NAV_ERR_AUTH: u32 = 0 + 2;
pub const wxWEBVIEW_NAV_ERR_SECURITY: u32 = 0 + 3;
pub const wxWEBVIEW_NAV_ERR_NOT_FOUND: u32 = 0 + 4;
pub const wxWEBVIEW_NAV_ERR_REQUEST: u32 = 0 + 5;
pub const wxWEBVIEW_NAV_ERR_USER_CANCELLED: u32 = 0 + 6;
pub const wxWEBVIEW_NAV_ERR_OTHER: u32 = 0 + 7;
//  ENUM: wxWebViewReloadFlags
pub const wxWEBVIEW_RELOAD_DEFAULT: u32 = 0;
pub const wxWEBVIEW_RELOAD_NO_CACHE: u32 = 0 + 1;
//  ENUM: wxWebViewFindFlags
pub const wxWEBVIEW_FIND_WRAP: u32 =             0x0001;
pub const wxWEBVIEW_FIND_ENTIRE_WORD: u32 =      0x0002;
pub const wxWEBVIEW_FIND_MATCH_CASE: u32 =       0x0004;
pub const wxWEBVIEW_FIND_HIGHLIGHT_RESULT: u32 = 0x0008;
pub const wxWEBVIEW_FIND_BACKWARDS: u32 =        0x0010;
pub const wxWEBVIEW_FIND_DEFAULT: u32 =          0;
//  ENUM: wxWebViewNavigationActionFlags
pub const wxWEBVIEW_NAV_ACTION_NONE: u32 = 0;
pub const wxWEBVIEW_NAV_ACTION_USER: u32 = 0 + 1;
pub const wxWEBVIEW_NAV_ACTION_OTHER: u32 = 0 + 2;
//  ENUM: wxWebViewUserScriptInjectionTime
pub const wxWEBVIEW_INJECT_AT_DOCUMENT_START: u32 = 0;
pub const wxWEBVIEW_INJECT_AT_DOCUMENT_END: u32 = 0 + 1;
//  ENUM: wxWebViewIE_EmulationLevel
pub const wxWEBVIEWIE_EMU_DEFAULT: u32 =    0;
pub const wxWEBVIEWIE_EMU_IE7: u32 =        7000;
pub const wxWEBVIEWIE_EMU_IE8: u32 =        8000;
pub const wxWEBVIEWIE_EMU_IE8_FORCE: u32 =  8888;
pub const wxWEBVIEWIE_EMU_IE9: u32 =        9000;
pub const wxWEBVIEWIE_EMU_IE9_FORCE: u32 =  9999;
pub const wxWEBVIEWIE_EMU_IE10: u32 =       10000;
pub const wxWEBVIEWIE_EMU_IE10_FORCE: u32 = 10001;
pub const wxWEBVIEWIE_EMU_IE11: u32 =       11000;
pub const wxWEBVIEWIE_EMU_IE11_FORCE: u32 = 11001;

//  ENUM: ConversionFlags
pub const Escape: u32 = 0x01;
pub const QuoteStrings: u32 = 0x02;

pub const wxTR_NO_BUTTONS: u32 = 0x0000;
pub const wxTR_HAS_BUTTONS: u32 = 0x0001;
pub const wxTR_NO_LINES: u32 = 0x0004;
pub const wxTR_LINES_AT_ROOT: u32 = 0x0008;
pub const wxTR_TWIST_BUTTONS: u32 = 0x0010;
pub const wxTR_SINGLE: u32 = 0x0000;
pub const wxTR_MULTIPLE: u32 = 0x0020;
pub const wxTR_HAS_VARIABLE_ROW_HEIGHT: u32 = 0x0080;
pub const wxTR_EDIT_LABELS: u32 = 0x0200;
pub const wxTR_ROW_LINES: u32 = 0x0400;
pub const wxTR_HIDE_ROOT: u32 = 0x0800;
pub const wxTR_FULL_ROW_HIGHLIGHT: u32 = 0x2000;
pub const wxTR_DEFAULT_STYLE: u32 = (wxTR_HAS_BUTTONS | wxTR_LINES_AT_ROOT);
//  ENUM: wxTreeItemIcon
pub const wxTreeItemIcon_Normal: u32 = 0;
pub const wxTreeItemIcon_Selected: u32 = 0 + 1;
pub const wxTreeItemIcon_Expanded: u32 = 0 + 2;
pub const wxTreeItemIcon_SelectedExpanded: u32 = 0 + 3;
pub const wxTreeItemIcon_Max: u32 = 0 + 4;

// NODEF: wxTheClipboard

//  ENUM: wxFSVolumeFlags
pub const wxFS_VOL_MOUNTED: u32 = 0x0001;
pub const wxFS_VOL_REMOVABLE: u32 = 0x0002;
pub const wxFS_VOL_READONLY: u32 = 0x0004;
pub const wxFS_VOL_REMOTE: u32 = 0x0008;
//  ENUM: wxFSVolumeKind
pub const wxFS_VOL_FLOPPY: u32 = 0;
pub const wxFS_VOL_DISK: u32 = 0 + 1;
pub const wxFS_VOL_CDROM: u32 = 0 + 2;
pub const wxFS_VOL_DVDROM: u32 = 0 + 3;
pub const wxFS_VOL_NETWORK: u32 = 0 + 4;
pub const wxFS_VOL_OTHER: u32 = 0 + 5;
pub const wxFS_VOL_MAX: u32 = 0 + 6;
//  ENUM: wxFSIconType
pub const wxFS_VOL_ICO_SMALL: u32 = 0;
pub const wxFS_VOL_ICO_LARGE: u32 = 0 + 1;
pub const wxFS_VOL_ICO_SEL_SMALL: u32 = 0 + 2;
pub const wxFS_VOL_ICO_SEL_LARGE: u32 = 0 + 3;
pub const wxFS_VOL_ICO_MAX: u32 = 0 + 4;

//  ENUM: wxAutomationInstanceFlags
pub const wxAutomationInstance_UseExistingOnly: u32 = 0;
pub const wxAutomationInstance_CreateIfNeeded: u32 = 1;
pub const wxAutomationInstance_SilentIfNone: u32 = 2;

//  ENUM: wxEdge
pub const wxLeft: u32 = 0;
pub const wxTop: u32 = 0 + 1;
pub const wxRight: u32 = 0 + 2;
pub const wxBottom: u32 = 0 + 3;
pub const wxWidth: u32 = 0 + 4;
pub const wxHeight: u32 = 0 + 5;
pub const wxCentre: u32 = 0 + 6;
pub const wxCenter: u32 = wxCentre;
pub const wxCentreX: u32 = wxCentre + 1;
pub const wxCentreY: u32 = wxCentre + 2;
//  ENUM: wxRelationship
pub const wxUnconstrained: u32 = 0;
pub const wxAsIs: u32 = 0 + 1;
pub const wxPercentOf: u32 = 0 + 2;
pub const wxAbove: u32 = 0 + 3;
pub const wxBelow: u32 = 0 + 4;
pub const wxLeftOf: u32 = 0 + 5;
pub const wxRightOf: u32 = 0 + 6;
pub const wxSameAs: u32 = 0 + 7;
pub const wxAbsolute: u32 = 0 + 8;

//  ENUM: @23
pub const wxCOL_WIDTH_DEFAULT: i32 = -1;
pub const wxCOL_WIDTH_AUTOSIZE: i32 = -2;
//  ENUM: @24
pub const wxCOL_RESIZABLE: u32 = 1;
pub const wxCOL_SORTABLE: u32 = 2;
pub const wxCOL_REORDERABLE: u32 = 4;
pub const wxCOL_HIDDEN: u32 = 8;
pub const wxCOL_DEFAULT_FLAGS: u32 = wxCOL_RESIZABLE | wxCOL_REORDERABLE;

pub const wxCHK_2STATE: u32 = 0x4000;
pub const wxCHK_3STATE: u32 = 0x1000;
pub const wxCHK_ALLOW_3RD_STATE_FOR_USER: u32 = 0x2000;
//  ENUM: wxCheckBoxState
pub const wxCHK_UNCHECKED: u32 = 0;
pub const wxCHK_CHECKED: u32 = 0 + 1;
pub const wxCHK_UNDETERMINED: u32 = 0 + 2;

//  ENUM: @43
pub const wxCONTROL_NONE: u32 = 0x00000000;
pub const wxCONTROL_DISABLED: u32 = 0x00000001;
pub const wxCONTROL_FOCUSED: u32 = 0x00000002;
pub const wxCONTROL_PRESSED: u32 = 0x00000004;
pub const wxCONTROL_SPECIAL: u32 = 0x00000008;
pub const wxCONTROL_ISDEFAULT: u32 = wxCONTROL_SPECIAL;
pub const wxCONTROL_ISSUBMENU: u32 = wxCONTROL_SPECIAL;
pub const wxCONTROL_EXPANDED: u32 = wxCONTROL_SPECIAL;
pub const wxCONTROL_SIZEGRIP: u32 = wxCONTROL_SPECIAL;
pub const wxCONTROL_FLAT: u32 = wxCONTROL_SPECIAL;
pub const wxCONTROL_CELL: u32 = wxCONTROL_SPECIAL;
pub const wxCONTROL_CURRENT: u32 = 0x00000010;
pub const wxCONTROL_SELECTED: u32 = 0x00000020;
pub const wxCONTROL_CHECKED: u32 = 0x00000040;
pub const wxCONTROL_CHECKABLE: u32 = 0x00000080;
pub const wxCONTROL_UNDETERMINED: u32 = wxCONTROL_CHECKABLE;
//  ENUM: wxTitleBarButton
pub const wxTITLEBAR_BUTTON_CLOSE: u32 = 0x01000000;
pub const wxTITLEBAR_BUTTON_MAXIMIZE: u32 = 0x02000000;
pub const wxTITLEBAR_BUTTON_ICONIZE: u32 = 0x04000000;
pub const wxTITLEBAR_BUTTON_RESTORE: u32 = 0x08000000;
pub const wxTITLEBAR_BUTTON_HELP: u32 = 0x10000000;
//  ENUM: wxHeaderSortIconType
pub const wxHDR_SORT_ICON_NONE: u32 = 0;
pub const wxHDR_SORT_ICON_UP: u32 = 0 + 1;
pub const wxHDR_SORT_ICON_DOWN: u32 = 0 + 2;

//  ENUM: @11
pub const wxDP_DEFAULT: u32 = 0;
pub const wxDP_SPIN: u32 = 1;
pub const wxDP_DROPDOWN: u32 = 2;
pub const wxDP_SHOWCENTURY: u32 = 4;
pub const wxDP_ALLOWNONE: u32 = 8;

//  ENUM: @6
pub const NUM_CUSTOM: u32 = 16;

// NODEF: wxT
// NODEF: wxT_2
// NODEF: wxS
// NODEF: _T

//  ENUM: wxNavigationKeyEventFlags
pub const IsBackward: u32 = 0x0000;
pub const IsForward: u32 = 0x0001;
pub const WinChange: u32 = 0x0002;
pub const FromTab: u32 = 0x0004;

pub const wxTBK_BUTTONBAR: u32 = 0x0100;
pub const wxTBK_HORZ_LAYOUT: u32 = 0x8000;

//  ENUM: @57
pub const NO_IMAGE: i32 = -1;

//  ENUM: wxSystemFont
pub const wxSYS_OEM_FIXED_FONT: u32 = 10;
pub const wxSYS_ANSI_FIXED_FONT: u32 = 10 + 1;
pub const wxSYS_ANSI_VAR_FONT: u32 = 10 + 2;
pub const wxSYS_SYSTEM_FONT: u32 = 10 + 3;
pub const wxSYS_DEVICE_DEFAULT_FONT: u32 = 10 + 4;
pub const wxSYS_DEFAULT_GUI_FONT: u32 = 10 + 5;
//  ENUM: wxSystemColour
pub const wxSYS_COLOUR_SCROLLBAR: u32 = 0;
pub const wxSYS_COLOUR_DESKTOP: u32 = 0 + 1;
pub const wxSYS_COLOUR_ACTIVECAPTION: u32 = 0 + 2;
pub const wxSYS_COLOUR_INACTIVECAPTION: u32 = 0 + 3;
pub const wxSYS_COLOUR_MENU: u32 = 0 + 4;
pub const wxSYS_COLOUR_WINDOW: u32 = 0 + 5;
pub const wxSYS_COLOUR_WINDOWFRAME: u32 = 0 + 6;
pub const wxSYS_COLOUR_MENUTEXT: u32 = 0 + 7;
pub const wxSYS_COLOUR_WINDOWTEXT: u32 = 0 + 8;
pub const wxSYS_COLOUR_CAPTIONTEXT: u32 = 0 + 9;
pub const wxSYS_COLOUR_ACTIVEBORDER: u32 = 0 + 10;
pub const wxSYS_COLOUR_INACTIVEBORDER: u32 = 0 + 11;
pub const wxSYS_COLOUR_APPWORKSPACE: u32 = 0 + 12;
pub const wxSYS_COLOUR_HIGHLIGHT: u32 = 0 + 13;
pub const wxSYS_COLOUR_HIGHLIGHTTEXT: u32 = 0 + 14;
pub const wxSYS_COLOUR_BTNFACE: u32 = 0 + 15;
pub const wxSYS_COLOUR_BTNSHADOW: u32 = 0 + 16;
pub const wxSYS_COLOUR_GRAYTEXT: u32 = 0 + 17;
pub const wxSYS_COLOUR_BTNTEXT: u32 = 0 + 18;
pub const wxSYS_COLOUR_INACTIVECAPTIONTEXT: u32 = 0 + 19;
pub const wxSYS_COLOUR_BTNHIGHLIGHT: u32 = 0 + 20;
pub const wxSYS_COLOUR_3DDKSHADOW: u32 = 0 + 21;
pub const wxSYS_COLOUR_3DLIGHT: u32 = 0 + 22;
pub const wxSYS_COLOUR_INFOTEXT: u32 = 0 + 23;
pub const wxSYS_COLOUR_INFOBK: u32 = 0 + 24;
pub const wxSYS_COLOUR_LISTBOX: u32 = 0 + 25;
pub const wxSYS_COLOUR_HOTLIGHT: u32 = 0 + 26;
pub const wxSYS_COLOUR_GRADIENTACTIVECAPTION: u32 = 0 + 27;
pub const wxSYS_COLOUR_GRADIENTINACTIVECAPTION: u32 = 0 + 28;
pub const wxSYS_COLOUR_MENUHILIGHT: u32 = 0 + 29;
pub const wxSYS_COLOUR_MENUBAR: u32 = 0 + 30;
pub const wxSYS_COLOUR_LISTBOXTEXT: u32 = 0 + 31;
pub const wxSYS_COLOUR_LISTBOXHIGHLIGHTTEXT: u32 = 0 + 32;
pub const wxSYS_COLOUR_BACKGROUND: u32 = wxSYS_COLOUR_DESKTOP;
pub const wxSYS_COLOUR_3DFACE: u32 = wxSYS_COLOUR_BTNFACE;
pub const wxSYS_COLOUR_3DSHADOW: u32 = wxSYS_COLOUR_BTNSHADOW;
pub const wxSYS_COLOUR_BTNHILIGHT: u32 = wxSYS_COLOUR_BTNHIGHLIGHT;
pub const wxSYS_COLOUR_3DHIGHLIGHT: u32 = wxSYS_COLOUR_BTNHIGHLIGHT;
pub const wxSYS_COLOUR_3DHILIGHT: u32 = wxSYS_COLOUR_BTNHIGHLIGHT;
pub const wxSYS_COLOUR_FRAMEBK: u32 = wxSYS_COLOUR_BTNFACE;
//  ENUM: wxSystemMetric
pub const wxSYS_MOUSE_BUTTONS: u32 = 0;
pub const wxSYS_BORDER_X: u32 = 0 + 1;
pub const wxSYS_BORDER_Y: u32 = 0 + 2;
pub const wxSYS_CURSOR_X: u32 = 0 + 3;
pub const wxSYS_CURSOR_Y: u32 = 0 + 4;
pub const wxSYS_DCLICK_X: u32 = 0 + 5;
pub const wxSYS_DCLICK_Y: u32 = 0 + 6;
pub const wxSYS_DRAG_X: u32 = 0 + 7;
pub const wxSYS_DRAG_Y: u32 = 0 + 8;
pub const wxSYS_EDGE_X: u32 = 0 + 9;
pub const wxSYS_EDGE_Y: u32 = 0 + 10;
pub const wxSYS_HSCROLL_ARROW_X: u32 = 0 + 11;
pub const wxSYS_HSCROLL_ARROW_Y: u32 = 0 + 12;
pub const wxSYS_HTHUMB_X: u32 = 0 + 13;
pub const wxSYS_ICON_X: u32 = 0 + 14;
pub const wxSYS_ICON_Y: u32 = 0 + 15;
pub const wxSYS_ICONSPACING_X: u32 = 0 + 16;
pub const wxSYS_ICONSPACING_Y: u32 = 0 + 17;
pub const wxSYS_WINDOWMIN_X: u32 = 0 + 18;
pub const wxSYS_WINDOWMIN_Y: u32 = 0 + 19;
pub const wxSYS_SCREEN_X: u32 = 0 + 20;
pub const wxSYS_SCREEN_Y: u32 = 0 + 21;
pub const wxSYS_FRAMESIZE_X: u32 = 0 + 22;
pub const wxSYS_FRAMESIZE_Y: u32 = 0 + 23;
pub const wxSYS_SMALLICON_X: u32 = 0 + 24;
pub const wxSYS_SMALLICON_Y: u32 = 0 + 25;
pub const wxSYS_HSCROLL_Y: u32 = 0 + 26;
pub const wxSYS_VSCROLL_X: u32 = 0 + 27;
pub const wxSYS_VSCROLL_ARROW_X: u32 = 0 + 28;
pub const wxSYS_VSCROLL_ARROW_Y: u32 = 0 + 29;
pub const wxSYS_VTHUMB_Y: u32 = 0 + 30;
pub const wxSYS_CAPTION_Y: u32 = 0 + 31;
pub const wxSYS_MENU_Y: u32 = 0 + 32;
pub const wxSYS_NETWORK_PRESENT: u32 = 0 + 33;
pub const wxSYS_PENWINDOWS_PRESENT: u32 = 0 + 34;
pub const wxSYS_SHOW_SOUNDS: u32 = 0 + 35;
pub const wxSYS_SWAP_BUTTONS: u32 = 0 + 36;
pub const wxSYS_DCLICK_MSEC: u32 = 0 + 37;
pub const wxSYS_CARET_ON_MSEC: u32 = 0 + 38;
pub const wxSYS_CARET_OFF_MSEC: u32 = 0 + 39;
pub const wxSYS_CARET_TIMEOUT_MSEC: u32 = 0 + 40;
//  ENUM: wxSystemFeature
pub const wxSYS_CAN_DRAW_FRAME_DECORATIONS: u32 = 1;
pub const wxSYS_CAN_ICONIZE_FRAME: u32 = 1 + 1;
pub const wxSYS_TABLET_PRESENT: u32 = 1 + 2;
//  ENUM: wxSystemScreenType
pub const wxSYS_SCREEN_NONE: u32 = 0;
pub const wxSYS_SCREEN_TINY: u32 = 0 + 1;
pub const wxSYS_SCREEN_PDA: u32 = 0 + 2;
pub const wxSYS_SCREEN_SMALL: u32 = 0 + 3;
pub const wxSYS_SCREEN_DESKTOP: u32 = 0 + 4;

//  ENUM: wxRegionContain
pub const wxOutRegion: u32 = 0;
pub const wxPartRegion: u32 = 1;
pub const wxInRegion: u32 = 2;

pub const wxFLP_OPEN: u32 = 0x0400;
pub const wxFLP_SAVE: u32 = 0x0800;
pub const wxFLP_OVERWRITE_PROMPT: u32 = 0x1000;
pub const wxFLP_FILE_MUST_EXIST: u32 = 0x2000;
pub const wxFLP_CHANGE_DIR: u32 = 0x4000;
pub const wxFLP_SMALL: u32 = wxPB_SMALL;
pub const wxFLP_USE_TEXTCTRL: u32 = (wxPB_USE_TEXTCTRL);
pub const wxFLP_DEFAULT_STYLE: u32 = (wxFLP_OPEN|wxFLP_FILE_MUST_EXIST);
pub const wxDIRP_DIR_MUST_EXIST: u32 = 0x0008;
pub const wxDIRP_CHANGE_DIR: u32 = 0x0010;
pub const wxDIRP_SMALL: u32 = wxPB_SMALL;
pub const wxDIRP_USE_TEXTCTRL: u32 = (wxPB_USE_TEXTCTRL);
pub const wxDIRP_DEFAULT_STYLE: u32 = (wxDIRP_DIR_MUST_EXIST);

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
// NODEF: wx_truncate_cast
// NODEF: wxConstCast
// NODEF: wxDynamicCast
// NODEF: wxDynamicCastThis
// NODEF: wxStaticCast
// NODEF: WXDEBUG_NEW

//  ENUM: @5
pub const wxC2S_NAME: u32 = 1;
pub const wxC2S_CSS_SYNTAX: u32 = 2;
pub const wxC2S_HTML_SYNTAX: u32 = 4;

//  ENUM: wxRichTextOddEvenPage
pub const wxRICHTEXT_PAGE_ODD: u32 = 0;
pub const wxRICHTEXT_PAGE_EVEN: u32 = 0 + 1;
pub const wxRICHTEXT_PAGE_ALL: u32 = 0 + 2;
//  ENUM: wxRichTextPageLocation
pub const wxRICHTEXT_PAGE_LEFT: u32 = 0;
pub const wxRICHTEXT_PAGE_CENTRE: u32 = 0 + 1;
pub const wxRICHTEXT_PAGE_RIGHT: u32 = 0 + 2;

//  ENUM: @45
pub const Option_AllowPixelFontSize: u32 = 0x0001;

pub const wxPU_CONTAINS_CONTROLS: u32 = 0x0001;

pub const wxIMAGELIST_DRAW_NORMAL: u32 = 0x0001;
pub const wxIMAGELIST_DRAW_TRANSPARENT: u32 = 0x0002;
pub const wxIMAGELIST_DRAW_SELECTED: u32 = 0x0004;
pub const wxIMAGELIST_DRAW_FOCUSED: u32 = 0x0008;
//  ENUM: @31
pub const wxIMAGE_LIST_NORMAL: u32 = 0;
pub const wxIMAGE_LIST_SMALL: u32 = 0 + 1;
pub const wxIMAGE_LIST_STATE: u32 = 0 + 2;

// NODEF: wxDEBUG_LEVEL
// NODEF: __WXDEBUG__
// NODEF: wxASSERT
// NODEF: wxASSERT_LEVEL_2
// NODEF: wxASSERT_LEVEL_2_MSG
// NODEF: wxASSERT_MIN_BITSIZE
// NODEF: wxASSERT_MSG
// NODEF: wxASSERT_MSG_AT
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
// NODEF: wxFAIL_MSG_AT

pub const wxGRID_AUTOSIZE: i32 = (-1);
//  ENUM: wxGridCellFloatFormat
pub const wxGRID_FLOAT_FORMAT_FIXED: u32 = 0x0010;
pub const wxGRID_FLOAT_FORMAT_SCIENTIFIC: u32 = 0x0020;
pub const wxGRID_FLOAT_FORMAT_COMPACT: u32 = 0x0040;
pub const wxGRID_FLOAT_FORMAT_UPPER: u32 = 0x0080;
pub const wxGRID_FLOAT_FORMAT_DEFAULT: u32 = wxGRID_FLOAT_FORMAT_FIXED;
//  ENUM: wxGridTableRequest
pub const wxGRIDTABLE_NOTIFY_ROWS_INSERTED: u32 = 0;
pub const wxGRIDTABLE_NOTIFY_ROWS_APPENDED: u32 = 0 + 1;
pub const wxGRIDTABLE_NOTIFY_ROWS_DELETED: u32 = 0 + 2;
pub const wxGRIDTABLE_NOTIFY_COLS_INSERTED: u32 = 0 + 3;
pub const wxGRIDTABLE_NOTIFY_COLS_APPENDED: u32 = 0 + 4;
pub const wxGRIDTABLE_NOTIFY_COLS_DELETED: u32 = 0 + 5;
//  ENUM: wxGridRenderStyle
pub const wxGRID_DRAW_ROWS_HEADER: u32 = 0x001;
pub const wxGRID_DRAW_COLS_HEADER: u32 = 0x002;
pub const wxGRID_DRAW_CELL_LINES: u32 = 0x004;
pub const wxGRID_DRAW_BOX_RECT: u32 = 0x008;
pub const wxGRID_DRAW_SELECTION: u32 = 0x010;
pub const wxGRID_DRAW_DEFAULT: u32 = wxGRID_DRAW_ROWS_HEADER |
                          wxGRID_DRAW_COLS_HEADER |
                          wxGRID_DRAW_CELL_LINES |
                          wxGRID_DRAW_BOX_RECT;

// NODEF: wxCHECK_VERSION
// NODEF: wxCHECK_VERSION_FULL

//  SKIP: wxInvalidDateTime

pub const wxTIMER_CONTINUOUS: bool = false;
pub const wxTIMER_ONE_SHOT: bool = true;

// NODEF: wxDISABLE_DEBUG_LOGGING_IN_RELEASE_BUILD
//  ENUM: wxLogLevelValues
pub const wxLOG_FatalError: u32 = 0;
pub const wxLOG_Error: u32 = 0 + 1;
pub const wxLOG_Warning: u32 = 0 + 2;
pub const wxLOG_Message: u32 = 0 + 3;
pub const wxLOG_Status: u32 = 0 + 4;
pub const wxLOG_Info: u32 = 0 + 5;
pub const wxLOG_Debug: u32 = 0 + 6;
pub const wxLOG_Trace: u32 = 0 + 7;
pub const wxLOG_Progress: u32 = 0 + 8;
pub const wxLOG_User: u32 = 100;
pub const wxLOG_Max: u32 = 10000;

//  ENUM: @25
pub const wxHD_ALLOW_REORDER: u32 = 0x0001;
pub const wxHD_ALLOW_HIDE: u32 = 0x0002;
pub const wxHD_BITMAP_ON_RIGHT: u32 = 0x0004;
pub const wxHD_DEFAULT_STYLE: u32 = wxHD_ALLOW_REORDER;

//  ENUM: wxSignal
pub const wxSIGNONE: u32 = 0;
pub const wxSIGHUP: u32 = 0 + 1;
pub const wxSIGINT: u32 = 0 + 2;
pub const wxSIGQUIT: u32 = 0 + 3;
pub const wxSIGILL: u32 = 0 + 4;
pub const wxSIGTRAP: u32 = 0 + 5;
pub const wxSIGABRT: u32 = 0 + 6;
pub const wxSIGEMT: u32 = 0 + 7;
pub const wxSIGFPE: u32 = 0 + 8;
pub const wxSIGKILL: u32 = 0 + 9;
pub const wxSIGBUS: u32 = 0 + 10;
pub const wxSIGSEGV: u32 = 0 + 11;
pub const wxSIGSYS: u32 = 0 + 12;
pub const wxSIGPIPE: u32 = 0 + 13;
pub const wxSIGALRM: u32 = 0 + 14;
pub const wxSIGTERM: u32 = 0 + 15;
//  ENUM: wxKillError
pub const wxKILL_OK: u32 = 0;
pub const wxKILL_BAD_SIGNAL: u32 = 0 + 1;
pub const wxKILL_ACCESS_DENIED: u32 = 0 + 2;
pub const wxKILL_NO_PROCESS: u32 = 0 + 3;
pub const wxKILL_ERROR: u32 = 0 + 4;
//  ENUM: wxKillFlags
pub const wxKILL_NOCHILDREN: u32 = 0;
pub const wxKILL_CHILDREN: u32 = 1;
//  ENUM: wxShutdownFlags
pub const wxSHUTDOWN_FORCE: u32 = 1;
pub const wxSHUTDOWN_POWEROFF: u32 = 2;
pub const wxSHUTDOWN_REBOOT: u32 = 4;
pub const wxSHUTDOWN_LOGOFF: u32 = 8;
//  ENUM: @53
pub const wxStrip_Mnemonics: u32 = 1;
pub const wxStrip_Accel: u32 = 2;
pub const wxStrip_CJKMnemonics: u32 = 4;
pub const wxStrip_All: u32 = wxStrip_Mnemonics | wxStrip_Accel;
pub const wxStrip_Menu: u32 = wxStrip_All | wxStrip_CJKMnemonics;
//  ENUM: @54
pub const wxEXEC_ASYNC: u32 = 0;
pub const wxEXEC_SYNC: u32 = 1;
pub const wxEXEC_SHOW_CONSOLE: u32 = 2;
pub const wxEXEC_MAKE_GROUP_LEADER: u32 = 4;
pub const wxEXEC_NODISABLE: u32 = 8;
pub const wxEXEC_NOEVENTS: u32 = 16;
pub const wxEXEC_HIDE_CONSOLE: u32 = 32;
pub const wxEXEC_BLOCK: u32 = wxEXEC_SYNC | wxEXEC_NOEVENTS;

//  ENUM: wxFlexSizerGrowMode
pub const wxFLEX_GROWMODE_NONE: u32 = 0;
pub const wxFLEX_GROWMODE_SPECIFIED: u32 = 0 + 1;
pub const wxFLEX_GROWMODE_ALL: u32 = 0 + 2;

pub const wxCP_DEFAULT_STYLE: u32 = (wxTAB_TRAVERSAL | wxNO_BORDER);
pub const wxCP_NO_TLW_RESIZE: u32 = (0x0002);

//  ENUM: wxFindReplaceFlags
pub const wxFR_DOWN: u32 = 1;
pub const wxFR_WHOLEWORD: u32 = 2;
pub const wxFR_MATCHCASE: u32 = 4;
//  ENUM: wxFindReplaceDialogStyles
pub const wxFR_REPLACEDIALOG: u32 = 1;
pub const wxFR_NOUPDOWN: u32 = 2;
pub const wxFR_NOMATCHCASE: u32 = 4;
pub const wxFR_NOWHOLEWORD: u32 = 8;

//  ENUM: wxBOM
pub const wxBOM_Unknown: i32 = -1;
pub const wxBOM_None: i32 = -1 + 1;
pub const wxBOM_UTF32BE: i32 = -1 + 2;
pub const wxBOM_UTF32LE: i32 = -1 + 3;
pub const wxBOM_UTF16BE: i32 = -1 + 4;
pub const wxBOM_UTF16LE: i32 = -1 + 5;
pub const wxBOM_UTF8: i32 = -1 + 6;

//  ENUM: @48
pub const wxTP_DEFAULT: u32 = 0;

//  ENUM: @8
pub const MovableButton: u32 = 0x0001;
pub const BitmapButton: u32 = 0x0002;
pub const ButtonSpacing: u32 = 0x0004;
pub const TextIndent: u32 = 0x0008;
pub const PaintControl: u32 = 0x0010;
pub const PaintWritable: u32 = 0x0020;
pub const Borderless: u32 = 0x0040;
pub const All: u32 = MovableButton | BitmapButton | ButtonSpacing |
                          TextIndent | PaintControl | PaintWritable |
                          Borderless;

//  ENUM: wxZipMethod
pub const wxZIP_METHOD_STORE: u32 = 0;
pub const wxZIP_METHOD_SHRINK: u32 = 0 + 1;
pub const wxZIP_METHOD_REDUCE1: u32 = 0 + 2;
pub const wxZIP_METHOD_REDUCE2: u32 = 0 + 3;
pub const wxZIP_METHOD_REDUCE3: u32 = 0 + 4;
pub const wxZIP_METHOD_REDUCE4: u32 = 0 + 5;
pub const wxZIP_METHOD_IMPLODE: u32 = 0 + 6;
pub const wxZIP_METHOD_TOKENIZE: u32 = 0 + 7;
pub const wxZIP_METHOD_DEFLATE: u32 = 0 + 8;
pub const wxZIP_METHOD_DEFLATE64: u32 = 0 + 9;
pub const wxZIP_METHOD_BZIP2: u32 = 12;
pub const wxZIP_METHOD_DEFAULT: u32 = 0xffff;
//  ENUM: wxZipSystem
pub const wxZIP_SYSTEM_MSDOS: u32 = 0;
pub const wxZIP_SYSTEM_AMIGA: u32 = 0 + 1;
pub const wxZIP_SYSTEM_OPENVMS: u32 = 0 + 2;
pub const wxZIP_SYSTEM_UNIX: u32 = 0 + 3;
pub const wxZIP_SYSTEM_VM_CMS: u32 = 0 + 4;
pub const wxZIP_SYSTEM_ATARI_ST: u32 = 0 + 5;
pub const wxZIP_SYSTEM_OS2_HPFS: u32 = 0 + 6;
pub const wxZIP_SYSTEM_MACINTOSH: u32 = 0 + 7;
pub const wxZIP_SYSTEM_Z_SYSTEM: u32 = 0 + 8;
pub const wxZIP_SYSTEM_CPM: u32 = 0 + 9;
pub const wxZIP_SYSTEM_WINDOWS_NTFS: u32 = 0 + 10;
pub const wxZIP_SYSTEM_MVS: u32 = 0 + 11;
pub const wxZIP_SYSTEM_VSE: u32 = 0 + 12;
pub const wxZIP_SYSTEM_ACORN_RISC: u32 = 0 + 13;
pub const wxZIP_SYSTEM_VFAT: u32 = 0 + 14;
pub const wxZIP_SYSTEM_ALTERNATE_MVS: u32 = 0 + 15;
pub const wxZIP_SYSTEM_BEOS: u32 = 0 + 16;
pub const wxZIP_SYSTEM_TANDEM: u32 = 0 + 17;
pub const wxZIP_SYSTEM_OS_400: u32 = 0 + 18;
//  ENUM: wxZipAttributes
pub const wxZIP_A_RDONLY: u32 = 0x01;
pub const wxZIP_A_HIDDEN: u32 = 0x02;
pub const wxZIP_A_SYSTEM: u32 = 0x04;
pub const wxZIP_A_SUBDIR: u32 = 0x10;
pub const wxZIP_A_ARCH: u32 = 0x20;
pub const wxZIP_A_MASK: u32 = 0x37;
//  ENUM: wxZipFlags
pub const wxZIP_ENCRYPTED: u32 = 0x0001;
pub const wxZIP_DEFLATE_NORMAL: u32 = 0x0000;
pub const wxZIP_DEFLATE_EXTRA: u32 = 0x0002;
pub const wxZIP_DEFLATE_FAST: u32 = 0x0004;
pub const wxZIP_DEFLATE_SUPERFAST: u32 = 0x0006;
pub const wxZIP_DEFLATE_MASK: u32 = 0x0006;
pub const wxZIP_SUMS_FOLLOW: u32 = 0x0008;
pub const wxZIP_ENHANCED: u32 = 0x0010;
pub const wxZIP_PATCH: u32 = 0x0020;
pub const wxZIP_STRONG_ENC: u32 = 0x0040;
pub const wxZIP_UNUSED: u32 = 0x0F80;
pub const wxZIP_RESERVED: u32 = 0xF000;
//  ENUM: wxZipArchiveFormat
pub const wxZIP_FORMAT_DEFAULT: u32 = 0;
pub const wxZIP_FORMAT_ZIP64: u32 = 0 + 1;

pub const wxDEFAULT_DELIMITERS: &str = " \t\r\n";
//  ENUM: wxStringTokenizerMode
pub const wxTOKEN_INVALID: i32 = -1;
pub const wxTOKEN_DEFAULT: i32 = -1 + 1;
pub const wxTOKEN_RET_EMPTY: i32 = -1 + 2;
pub const wxTOKEN_RET_EMPTY_ALL: i32 = -1 + 3;
pub const wxTOKEN_RET_DELIMS: i32 = -1 + 4;
pub const wxTOKEN_STRTOK: i32 = -1 + 5;

//  ENUM: wxTipKind
pub const wxTipKind_None: u32 = 0;
pub const wxTipKind_TopLeft: u32 = 0 + 1;
pub const wxTipKind_Top: u32 = 0 + 2;
pub const wxTipKind_TopRight: u32 = 0 + 3;
pub const wxTipKind_BottomLeft: u32 = 0 + 4;
pub const wxTipKind_Bottom: u32 = 0 + 5;
pub const wxTipKind_BottomRight: u32 = 0 + 6;
pub const wxTipKind_Auto: u32 = 0 + 7;

//  ENUM: wxFontFamily
pub const wxFONTFAMILY_DEFAULT: i32 = wxDEFAULT;
pub const wxFONTFAMILY_DECORATIVE: i32 = wxDECORATIVE;
pub const wxFONTFAMILY_ROMAN: i32 = wxROMAN;
pub const wxFONTFAMILY_SCRIPT: i32 = wxSCRIPT;
pub const wxFONTFAMILY_SWISS: i32 = wxSWISS;
pub const wxFONTFAMILY_MODERN: i32 = wxMODERN;
pub const wxFONTFAMILY_TELETYPE: i32 = wxTELETYPE;
pub const wxFONTFAMILY_MAX: i32 = wxTELETYPE + 1;
pub const wxFONTFAMILY_UNKNOWN: i32 = wxFONTFAMILY_MAX;
//  ENUM: wxFontStyle
pub const wxFONTSTYLE_NORMAL: i32 = wxNORMAL;
pub const wxFONTSTYLE_ITALIC: i32 = wxITALIC;
pub const wxFONTSTYLE_SLANT: i32 = wxSLANT;
pub const wxFONTSTYLE_MAX: i32 = wxSLANT + 1;
//  ENUM: wxFontWeight
pub const wxFONTWEIGHT_INVALID: u32 = 0;
pub const wxFONTWEIGHT_THIN: u32 = 100;
pub const wxFONTWEIGHT_EXTRALIGHT: u32 = 200;
pub const wxFONTWEIGHT_LIGHT: u32 = 300;
pub const wxFONTWEIGHT_NORMAL: u32 = 400;
pub const wxFONTWEIGHT_MEDIUM: u32 = 500;
pub const wxFONTWEIGHT_SEMIBOLD: u32 = 600;
pub const wxFONTWEIGHT_BOLD: u32 = 700;
pub const wxFONTWEIGHT_EXTRABOLD: u32 = 800;
pub const wxFONTWEIGHT_HEAVY: u32 = 900;
pub const wxFONTWEIGHT_EXTRAHEAVY: u32 = 1000;
pub const wxFONTWEIGHT_MAX: u32 = wxFONTWEIGHT_EXTRAHEAVY;
//  ENUM: wxFontSymbolicSize
pub const wxFONTSIZE_XX_SMALL: i32 = -3;
pub const wxFONTSIZE_X_SMALL: i32 = -3 + 1;
pub const wxFONTSIZE_SMALL: i32 = -3 + 2;
pub const wxFONTSIZE_MEDIUM: i32 = -3 + 3;
pub const wxFONTSIZE_LARGE: i32 = -3 + 4;
pub const wxFONTSIZE_X_LARGE: i32 = -3 + 5;
pub const wxFONTSIZE_XX_LARGE: i32 = -3 + 6;
//  ENUM: wxFontFlag
pub const wxFONTFLAG_DEFAULT: u32 = 0;
pub const wxFONTFLAG_ITALIC: u32 = 1 << 0;
pub const wxFONTFLAG_SLANT: u32 = 1 << 1;
pub const wxFONTFLAG_LIGHT: u32 = 1 << 2;
pub const wxFONTFLAG_BOLD: u32 = 1 << 3;
pub const wxFONTFLAG_ANTIALIASED: u32 = 1 << 4;
pub const wxFONTFLAG_NOT_ANTIALIASED: u32 = 1 << 5;
pub const wxFONTFLAG_UNDERLINED: u32 = 1 << 6;
pub const wxFONTFLAG_STRIKETHROUGH: u32 = 1 << 7;
pub const wxFONTFLAG_MASK: u32 = wxFONTFLAG_ITALIC             |
                      wxFONTFLAG_SLANT              |
                      wxFONTFLAG_LIGHT              |
                      wxFONTFLAG_BOLD               |
                      wxFONTFLAG_ANTIALIASED        |
                      wxFONTFLAG_NOT_ANTIALIASED    |
                      wxFONTFLAG_UNDERLINED         |
                      wxFONTFLAG_STRIKETHROUGH;
//  ENUM: wxFontEncoding
pub const wxFONTENCODING_SYSTEM: i32 = -1;
pub const wxFONTENCODING_DEFAULT: i32 = -1 + 1;
pub const wxFONTENCODING_ISO8859_1: i32 = -1 + 2;
pub const wxFONTENCODING_ISO8859_2: i32 = -1 + 3;
pub const wxFONTENCODING_ISO8859_3: i32 = -1 + 4;
pub const wxFONTENCODING_ISO8859_4: i32 = -1 + 5;
pub const wxFONTENCODING_ISO8859_5: i32 = -1 + 6;
pub const wxFONTENCODING_ISO8859_6: i32 = -1 + 7;
pub const wxFONTENCODING_ISO8859_7: i32 = -1 + 8;
pub const wxFONTENCODING_ISO8859_8: i32 = -1 + 9;
pub const wxFONTENCODING_ISO8859_9: i32 = -1 + 10;
pub const wxFONTENCODING_ISO8859_10: i32 = -1 + 11;
pub const wxFONTENCODING_ISO8859_11: i32 = -1 + 12;
pub const wxFONTENCODING_ISO8859_12: i32 = -1 + 13;
pub const wxFONTENCODING_ISO8859_13: i32 = -1 + 14;
pub const wxFONTENCODING_ISO8859_14: i32 = -1 + 15;
pub const wxFONTENCODING_ISO8859_15: i32 = -1 + 16;
pub const wxFONTENCODING_ISO8859_MAX: i32 = -1 + 17;
pub const wxFONTENCODING_KOI8: i32 = -1 + 18;
pub const wxFONTENCODING_KOI8_U: i32 = -1 + 19;
pub const wxFONTENCODING_ALTERNATIVE: i32 = -1 + 20;
pub const wxFONTENCODING_BULGARIAN: i32 = -1 + 21;
pub const wxFONTENCODING_CP437: i32 = -1 + 22;
pub const wxFONTENCODING_CP850: i32 = -1 + 23;
pub const wxFONTENCODING_CP852: i32 = -1 + 24;
pub const wxFONTENCODING_CP855: i32 = -1 + 25;
pub const wxFONTENCODING_CP866: i32 = -1 + 26;
pub const wxFONTENCODING_CP874: i32 = -1 + 27;
pub const wxFONTENCODING_CP932: i32 = -1 + 28;
pub const wxFONTENCODING_CP936: i32 = -1 + 29;
pub const wxFONTENCODING_CP949: i32 = -1 + 30;
pub const wxFONTENCODING_CP950: i32 = -1 + 31;
pub const wxFONTENCODING_CP1250: i32 = -1 + 32;
pub const wxFONTENCODING_CP1251: i32 = -1 + 33;
pub const wxFONTENCODING_CP1252: i32 = -1 + 34;
pub const wxFONTENCODING_CP1253: i32 = -1 + 35;
pub const wxFONTENCODING_CP1254: i32 = -1 + 36;
pub const wxFONTENCODING_CP1255: i32 = -1 + 37;
pub const wxFONTENCODING_CP1256: i32 = -1 + 38;
pub const wxFONTENCODING_CP1257: i32 = -1 + 39;
pub const wxFONTENCODING_CP1258: i32 = -1 + 40;
pub const wxFONTENCODING_CP1361: i32 = -1 + 41;
pub const wxFONTENCODING_CP12_MAX: i32 = -1 + 42;
pub const wxFONTENCODING_UTF7: i32 = -1 + 43;
pub const wxFONTENCODING_UTF8: i32 = -1 + 44;
pub const wxFONTENCODING_EUC_JP: i32 = -1 + 45;
pub const wxFONTENCODING_UTF16BE: i32 = -1 + 46;
pub const wxFONTENCODING_UTF16LE: i32 = -1 + 47;
pub const wxFONTENCODING_UTF32BE: i32 = -1 + 48;
pub const wxFONTENCODING_UTF32LE: i32 = -1 + 49;
pub const wxFONTENCODING_MACROMAN: i32 = -1 + 50;
pub const wxFONTENCODING_MACJAPANESE: i32 = -1 + 51;
pub const wxFONTENCODING_MACCHINESETRAD: i32 = -1 + 52;
pub const wxFONTENCODING_MACKOREAN: i32 = -1 + 53;
pub const wxFONTENCODING_MACARABIC: i32 = -1 + 54;
pub const wxFONTENCODING_MACHEBREW: i32 = -1 + 55;
pub const wxFONTENCODING_MACGREEK: i32 = -1 + 56;
pub const wxFONTENCODING_MACCYRILLIC: i32 = -1 + 57;
pub const wxFONTENCODING_MACDEVANAGARI: i32 = -1 + 58;
pub const wxFONTENCODING_MACGURMUKHI: i32 = -1 + 59;
pub const wxFONTENCODING_MACGUJARATI: i32 = -1 + 60;
pub const wxFONTENCODING_MACORIYA: i32 = -1 + 61;
pub const wxFONTENCODING_MACBENGALI: i32 = -1 + 62;
pub const wxFONTENCODING_MACTAMIL: i32 = -1 + 63;
pub const wxFONTENCODING_MACTELUGU: i32 = -1 + 64;
pub const wxFONTENCODING_MACKANNADA: i32 = -1 + 65;
pub const wxFONTENCODING_MACMALAJALAM: i32 = -1 + 66;
pub const wxFONTENCODING_MACSINHALESE: i32 = -1 + 67;
pub const wxFONTENCODING_MACBURMESE: i32 = -1 + 68;
pub const wxFONTENCODING_MACKHMER: i32 = -1 + 69;
pub const wxFONTENCODING_MACTHAI: i32 = -1 + 70;
pub const wxFONTENCODING_MACLAOTIAN: i32 = -1 + 71;
pub const wxFONTENCODING_MACGEORGIAN: i32 = -1 + 72;
pub const wxFONTENCODING_MACARMENIAN: i32 = -1 + 73;
pub const wxFONTENCODING_MACCHINESESIMP: i32 = -1 + 74;
pub const wxFONTENCODING_MACTIBETAN: i32 = -1 + 75;
pub const wxFONTENCODING_MACMONGOLIAN: i32 = -1 + 76;
pub const wxFONTENCODING_MACETHIOPIC: i32 = -1 + 77;
pub const wxFONTENCODING_MACCENTRALEUR: i32 = -1 + 78;
pub const wxFONTENCODING_MACVIATNAMESE: i32 = -1 + 79;
pub const wxFONTENCODING_MACARABICEXT: i32 = -1 + 80;
pub const wxFONTENCODING_MACSYMBOL: i32 = -1 + 81;
pub const wxFONTENCODING_MACDINGBATS: i32 = -1 + 82;
pub const wxFONTENCODING_MACTURKISH: i32 = -1 + 83;
pub const wxFONTENCODING_MACCROATIAN: i32 = -1 + 84;
pub const wxFONTENCODING_MACICELANDIC: i32 = -1 + 85;
pub const wxFONTENCODING_MACROMANIAN: i32 = -1 + 86;
pub const wxFONTENCODING_MACCELTIC: i32 = -1 + 87;
pub const wxFONTENCODING_MACGAELIC: i32 = -1 + 88;
pub const wxFONTENCODING_MACKEYBOARD: i32 = -1 + 89;
pub const wxFONTENCODING_ISO2022_JP: i32 = -1 + 90;
pub const wxFONTENCODING_MAX: i32 = -1 + 91;
pub const wxFONTENCODING_MACMIN: i32 = wxFONTENCODING_MACROMAN;
pub const wxFONTENCODING_MACMAX: i32 = wxFONTENCODING_MACKEYBOARD;
pub const wxFONTENCODING_UTF16: i32 = wxFONTENCODING_MACKEYBOARD + 1;
pub const wxFONTENCODING_UTF32: i32 = wxFONTENCODING_MACKEYBOARD + 2;
pub const wxFONTENCODING_UNICODE: i32 = wxFONTENCODING_MACKEYBOARD + 3;
pub const wxFONTENCODING_GB2312: i32 = wxFONTENCODING_CP936;
pub const wxFONTENCODING_BIG5: i32 = wxFONTENCODING_CP950;
pub const wxFONTENCODING_SHIFT_JIS: i32 = wxFONTENCODING_CP932;
pub const wxFONTENCODING_EUC_KR: i32 = wxFONTENCODING_CP949;
pub const wxFONTENCODING_JOHAB: i32 = wxFONTENCODING_CP1361;
pub const wxFONTENCODING_VIETNAMESE: i32 = wxFONTENCODING_CP1258;

//  ENUM: wxURIHostType
pub const wxURI_REGNAME: u32 = 0;
pub const wxURI_IPV4ADDRESS: u32 = 0 + 1;
pub const wxURI_IPV6ADDRESS: u32 = 0 + 2;
pub const wxURI_IPVFUTURE: u32 = 0 + 3;

//  ENUM: wxFileHistoryMenuPathStyle
pub const wxFH_PATH_SHOW_IF_DIFFERENT: u32 = 0;
pub const wxFH_PATH_SHOW_NEVER: u32 = 0 + 1;
pub const wxFH_PATH_SHOW_ALWAYS: u32 = 0 + 2;

// NODEF: wxDROP_ICON
//  ENUM: @13
pub const wxDrag_CopyOnly: u32 = 0;
pub const wxDrag_AllowMove: u32 = 1;
pub const wxDrag_DefaultMove: u32 = 3;
//  ENUM: wxDragResult
pub const wxDragError: u32 = 0;
pub const wxDragNone: u32 = 0 + 1;
pub const wxDragCopy: u32 = 0 + 2;
pub const wxDragMove: u32 = 0 + 3;
pub const wxDragLink: u32 = 0 + 4;
pub const wxDragCancel: u32 = 0 + 5;

//  ENUM: @12
pub const wxDIRCTRL_DIR_ONLY: u32 = 0x0010;
pub const wxDIRCTRL_SELECT_FIRST: u32 = 0x0020;
pub const wxDIRCTRL_SHOW_FILTERS: u32 = 0x0040;
pub const wxDIRCTRL_3D_INTERNAL: u32 = 0x0080;
pub const wxDIRCTRL_EDIT_LABELS: u32 = 0x0100;
pub const wxDIRCTRL_MULTIPLE: u32 = 0x0200;
pub const wxDIRCTRL_DEFAULT_STYLE: u32 = wxDIRCTRL_3D_INTERNAL;

//  ENUM: Origin
pub const Program: u32 = 0;
pub const Key: u32 = 0 + 1;
pub const Mouse: u32 = 0 + 2;

pub const wxFRAME_SHAPED: u32 = 0x0010;

//  ENUM: wxAuiManagerDock
pub const wxAUI_DOCK_NONE: u32 = 0;
pub const wxAUI_DOCK_TOP: u32 = 1;
pub const wxAUI_DOCK_RIGHT: u32 = 2;
pub const wxAUI_DOCK_BOTTOM: u32 = 3;
pub const wxAUI_DOCK_LEFT: u32 = 4;
pub const wxAUI_DOCK_CENTER: u32 = 5;
pub const wxAUI_DOCK_CENTRE: u32 = wxAUI_DOCK_CENTER;
//  ENUM: wxAuiManagerOption
pub const wxAUI_MGR_ALLOW_FLOATING: u32 = 1 << 0;
pub const wxAUI_MGR_ALLOW_ACTIVE_PANE: u32 = 1 << 1;
pub const wxAUI_MGR_TRANSPARENT_DRAG: u32 = 1 << 2;
pub const wxAUI_MGR_TRANSPARENT_HINT: u32 = 1 << 3;
pub const wxAUI_MGR_VENETIAN_BLINDS_HINT: u32 = 1 << 4;
pub const wxAUI_MGR_RECTANGLE_HINT: u32 = 1 << 5;
pub const wxAUI_MGR_HINT_FADE: u32 = 1 << 6;
pub const wxAUI_MGR_NO_VENETIAN_BLINDS_FADE: u32 = 1 << 7;
pub const wxAUI_MGR_LIVE_RESIZE: u32 = 1 << 8;
pub const wxAUI_MGR_DEFAULT: u32 = wxAUI_MGR_ALLOW_FLOATING |
                        wxAUI_MGR_TRANSPARENT_HINT |
                        wxAUI_MGR_HINT_FADE |
                        wxAUI_MGR_NO_VENETIAN_BLINDS_FADE;

pub const wxTextEntryDialogStyle: u32 = (wxOK | wxCANCEL | wxCENTRE);

pub const wxHLB_DEFAULT_STYLE: u32 = wxBORDER_SUNKEN;
pub const wxHLB_MULTIPLE: u32 = wxLB_MULTIPLE;

//  ENUM: TransferMode
pub const NONE: u32 = 0;
pub const ASCII: u32 = 0 + 1;
pub const BINARY: u32 = 0 + 2;

pub const wxPG_DEFAULT_STYLE: u32 = (0);
pub const wxPGMAN_DEFAULT_STYLE: u32 = (0);
//  SKIP: wxPGVFBFlags
//  ENUM: wxPG_WINDOW_STYLES
pub const wxPG_AUTO_SORT: u32 = 0x00000010;
pub const wxPG_HIDE_CATEGORIES: u32 = 0x00000020;
pub const wxPG_ALPHABETIC_MODE: u32 = (wxPG_HIDE_CATEGORIES|wxPG_AUTO_SORT);
pub const wxPG_BOLD_MODIFIED: u32 = 0x00000040;
pub const wxPG_SPLITTER_AUTO_CENTER: u32 = 0x00000080;
pub const wxPG_TOOLTIPS: u32 = 0x00000100;
pub const wxPG_HIDE_MARGIN: u32 = 0x00000200;
pub const wxPG_STATIC_SPLITTER: u32 = 0x00000400;
pub const wxPG_STATIC_LAYOUT: u32 = (wxPG_HIDE_MARGIN|wxPG_STATIC_SPLITTER);
pub const wxPG_LIMITED_EDITING: u32 = 0x00000800;
pub const wxPG_TOOLBAR: u32 = 0x00001000;
pub const wxPG_DESCRIPTION: u32 = 0x00002000;
pub const wxPG_NO_INTERNAL_BORDER: u32 = 0x00004000;
pub const wxPG_WINDOW_STYLE_MASK: u32 = wxPG_AUTO_SORT|wxPG_HIDE_CATEGORIES|wxPG_BOLD_MODIFIED|
                         wxPG_SPLITTER_AUTO_CENTER|wxPG_TOOLTIPS|wxPG_HIDE_MARGIN|
                         wxPG_STATIC_SPLITTER|wxPG_LIMITED_EDITING|wxPG_TOOLBAR|
                         wxPG_DESCRIPTION|wxPG_NO_INTERNAL_BORDER;
//  ENUM: wxPG_EX_WINDOW_STYLES
pub const wxPG_EX_INIT_NOCAT: u32 = 0x00001000;
pub const wxPG_EX_NO_FLAT_TOOLBAR: u32 = 0x00002000;
pub const wxPG_EX_MODE_BUTTONS: u32 = 0x00008000;
pub const wxPG_EX_HELP_AS_TOOLTIPS: u32 = 0x00010000;
pub const wxPG_EX_NATIVE_DOUBLE_BUFFERING: u32 = 0x00080000;
pub const wxPG_EX_AUTO_UNSPECIFIED_VALUES: u32 = 0x00200000;
pub const wxPG_EX_WRITEONLY_BUILTIN_ATTRIBUTES: u32 = 0x00400000;
pub const wxPG_EX_HIDE_PAGE_BUTTONS: u32 = 0x01000000;
pub const wxPG_EX_MULTIPLE_SELECTION: u32 = 0x02000000;
pub const wxPG_EX_ENABLE_TLP_TRACKING: u32 = 0x04000000;
pub const wxPG_EX_NO_TOOLBAR_DIVIDER: u32 = 0x04000000;
pub const wxPG_EX_TOOLBAR_SEPARATOR: u32 = 0x08000000;
pub const wxPG_EX_ALWAYS_ALLOW_FOCUS: u32 = 0x00100000;
pub const wxPG_EX_WINDOW_PG_STYLE_MASK: u32 = wxPG_EX_INIT_NOCAT|wxPG_EX_HELP_AS_TOOLTIPS|wxPG_EX_NATIVE_DOUBLE_BUFFERING|
                               wxPG_EX_AUTO_UNSPECIFIED_VALUES|wxPG_EX_WRITEONLY_BUILTIN_ATTRIBUTES|
                               wxPG_EX_MULTIPLE_SELECTION|wxPG_EX_ENABLE_TLP_TRACKING|wxPG_EX_ALWAYS_ALLOW_FOCUS;
pub const wxPG_EX_WINDOW_PGMAN_STYLE_MASK: u32 = wxPG_EX_NO_FLAT_TOOLBAR|wxPG_EX_MODE_BUTTONS|wxPG_EX_HIDE_PAGE_BUTTONS|
                                  wxPG_EX_NO_TOOLBAR_DIVIDER|wxPG_EX_TOOLBAR_SEPARATOR;
pub const wxPG_EX_WINDOW_STYLE_MASK: u32 = wxPG_EX_WINDOW_PG_STYLE_MASK|wxPG_EX_WINDOW_PGMAN_STYLE_MASK;
//  ENUM: wxPG_VALIDATION_FAILURE_BEHAVIOR_FLAGS
pub const wxPG_VFB_STAY_IN_PROPERTY: u32 = 0x01;
pub const wxPG_VFB_BEEP: u32 = 0x02;
pub const wxPG_VFB_MARK_CELL: u32 = 0x04;
pub const wxPG_VFB_SHOW_MESSAGE: u32 = 0x08;
pub const wxPG_VFB_SHOW_MESSAGEBOX: u32 = 0x10;
pub const wxPG_VFB_SHOW_MESSAGE_ON_STATUSBAR: u32 = 0x20;
pub const wxPG_VFB_DEFAULT: u32 = wxPG_VFB_MARK_CELL |
                                      wxPG_VFB_SHOW_MESSAGEBOX;
//  ENUM: wxPG_KEYBOARD_ACTIONS
pub const wxPG_ACTION_INVALID: u32 = 0;
pub const wxPG_ACTION_NEXT_PROPERTY: u32 = 0 + 1;
pub const wxPG_ACTION_PREV_PROPERTY: u32 = 0 + 2;
pub const wxPG_ACTION_EXPAND_PROPERTY: u32 = 0 + 3;
pub const wxPG_ACTION_COLLAPSE_PROPERTY: u32 = 0 + 4;
pub const wxPG_ACTION_CANCEL_EDIT: u32 = 0 + 5;
pub const wxPG_ACTION_EDIT: u32 = 0 + 6;
pub const wxPG_ACTION_PRESS_BUTTON: u32 = 0 + 7;
pub const wxPG_ACTION_MAX: u32 = 0 + 8;

//  ENUM: wxXmlResourceFlags
pub const wxXRC_USE_LOCALE: u32 = 1;
pub const wxXRC_NO_SUBCLASSING: u32 = 2;
pub const wxXRC_NO_RELOADING: u32 = 4;
pub const wxXRC_USE_ENVVARS: u32 = 8;

// NODEF: wxCHECK_GCC_VERSION
// NODEF: wxCHECK_SUNCC_VERSION
// NODEF: wxCHECK_VISUALC_VERSION
// NODEF: wxCHECK_W32API_VERSION

//  ENUM: wxStockLabelQueryFlag
pub const wxSTOCK_NOFLAGS: u32 = 0;
pub const wxSTOCK_WITH_MNEMONIC: u32 = 1;
pub const wxSTOCK_WITH_ACCELERATOR: u32 = 2;
pub const wxSTOCK_WITHOUT_ELLIPSIS: u32 = 4;
pub const wxSTOCK_FOR_BUTTON: u32 = wxSTOCK_WITHOUT_ELLIPSIS | wxSTOCK_WITH_MNEMONIC;

//  ENUM: @48

pub const wxRICHTEXT_FIXED_WIDTH: u32 = 0x01;
pub const wxRICHTEXT_FIXED_HEIGHT: u32 = 0x02;
pub const wxRICHTEXT_VARIABLE_WIDTH: u32 = 0x04;
pub const wxRICHTEXT_VARIABLE_HEIGHT: u32 = 0x08;
pub const wxRICHTEXT_LAYOUT_SPECIFIED_RECT: u32 = 0x10;
pub const wxRICHTEXT_DRAW_IGNORE_CACHE: u32 = 0x01;
pub const wxRICHTEXT_DRAW_SELECTED: u32 = 0x02;
pub const wxRICHTEXT_DRAW_PRINT: u32 = 0x04;
pub const wxRICHTEXT_DRAW_GUIDELINES: u32 = 0x08;
pub const wxRICHTEXT_FORMATTED: u32 = 0x01;
pub const wxRICHTEXT_UNFORMATTED: u32 = 0x02;
pub const wxRICHTEXT_CACHE_SIZE: u32 = 0x04;
pub const wxRICHTEXT_HEIGHT_ONLY: u32 = 0x08;
pub const wxRICHTEXT_SETSTYLE_NONE: u32 = 0x00;
pub const wxRICHTEXT_SETSTYLE_WITH_UNDO: u32 = 0x01;
pub const wxRICHTEXT_SETSTYLE_OPTIMIZE: u32 = 0x02;
pub const wxRICHTEXT_SETSTYLE_PARAGRAPHS_ONLY: u32 = 0x04;
pub const wxRICHTEXT_SETSTYLE_CHARACTERS_ONLY: u32 = 0x08;
pub const wxRICHTEXT_SETSTYLE_RENUMBER: u32 = 0x10;
pub const wxRICHTEXT_SETSTYLE_SPECIFY_LEVEL: u32 = 0x20;
pub const wxRICHTEXT_SETSTYLE_RESET: u32 = 0x40;
pub const wxRICHTEXT_SETSTYLE_REMOVE: u32 = 0x80;
pub const wxRICHTEXT_SETPROPERTIES_NONE: u32 = 0x00;
pub const wxRICHTEXT_SETPROPERTIES_WITH_UNDO: u32 = 0x01;
pub const wxRICHTEXT_SETPROPERTIES_PARAGRAPHS_ONLY: u32 = 0x02;
pub const wxRICHTEXT_SETPROPERTIES_CHARACTERS_ONLY: u32 = 0x04;
pub const wxRICHTEXT_SETPROPERTIES_RESET: u32 = 0x08;
pub const wxRICHTEXT_SETPROPERTIES_REMOVE: u32 = 0x10;
pub const wxRICHTEXT_INSERT_NONE: u32 = 0x00;
pub const wxRICHTEXT_INSERT_WITH_PREVIOUS_PARAGRAPH_STYLE: u32 = 0x01;
pub const wxRICHTEXT_INSERT_INTERACTIVE: u32 = 0x02;
pub const wxTEXT_ATTR_KEEP_FIRST_PARA_STYLE: u32 = 0x10000000;
pub const wxSCRIPT_MUL_FACTOR: f32 = 1.5;
//  SKIP: wxRICHTEXT_ALL
//  SKIP: wxRICHTEXT_NONE
//  SKIP: wxRICHTEXT_NO_SELECTION
pub const wxRICHTEXT_HANDLER_INCLUDE_STYLESHEET: u32 = 0x0001;
pub const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_MEMORY: u32 = 0x0010;
pub const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_FILES: u32 = 0x0020;
pub const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_BASE64: u32 = 0x0040;
pub const wxRICHTEXT_HANDLER_NO_HEADER_FOOTER: u32 = 0x0080;
pub const wxRICHTEXT_HANDLER_CONVERT_FACENAMES: u32 = 0x0100;
pub const wxRICHTEXT_HANDLER_USE_CSS: u32 = 0x1000;
//  ENUM: wxRichTextFileType
pub const wxRICHTEXT_TYPE_ANY: u32 = 0;
pub const wxRICHTEXT_TYPE_TEXT: u32 = 0 + 1;
pub const wxRICHTEXT_TYPE_XML: u32 = 0 + 2;
pub const wxRICHTEXT_TYPE_HTML: u32 = 0 + 3;
pub const wxRICHTEXT_TYPE_RTF: u32 = 0 + 4;
pub const wxRICHTEXT_TYPE_PDF: u32 = 0 + 5;
//  ENUM: wxRichTextHitTestFlags
pub const wxRICHTEXT_HITTEST_NONE: u32 =    0x01;
pub const wxRICHTEXT_HITTEST_BEFORE: u32 =  0x02;
pub const wxRICHTEXT_HITTEST_AFTER: u32 =   0x04;
pub const wxRICHTEXT_HITTEST_ON: u32 =      0x08;
pub const wxRICHTEXT_HITTEST_OUTSIDE: u32 = 0x10;
pub const wxRICHTEXT_HITTEST_NO_NESTED_OBJECTS: u32 = 0x20;
pub const wxRICHTEXT_HITTEST_NO_FLOATING_OBJECTS: u32 = 0x40;
pub const wxRICHTEXT_HITTEST_HONOUR_ATOMIC: u32 = 0x80;
//  ENUM: wxTextBoxAttrFlags
pub const wxTEXT_BOX_ATTR_FLOAT: u32 = 0x00000001;
pub const wxTEXT_BOX_ATTR_CLEAR: u32 = 0x00000002;
pub const wxTEXT_BOX_ATTR_COLLAPSE_BORDERS: u32 = 0x00000004;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT: u32 = 0x00000008;
pub const wxTEXT_BOX_ATTR_BOX_STYLE_NAME: u32 = 0x00000010;
pub const wxTEXT_BOX_ATTR_WHITESPACE: u32 = 0x00000020;
pub const wxTEXT_BOX_ATTR_CORNER_RADIUS: u32 = 0x00000040;
//  ENUM: wxTextAttrValueFlags
pub const wxTEXT_ATTR_VALUE_VALID: u32 = 0x1000;
pub const wxTEXT_ATTR_VALUE_VALID_MASK: u32 = 0x1000;
//  ENUM: wxTextAttrUnits
pub const wxTEXT_ATTR_UNITS_TENTHS_MM: u32 = 0x0001;
pub const wxTEXT_ATTR_UNITS_PIXELS: u32 = 0x0002;
pub const wxTEXT_ATTR_UNITS_PERCENTAGE: u32 = 0x0004;
pub const wxTEXT_ATTR_UNITS_POINTS: u32 = 0x0008;
pub const wxTEXT_ATTR_UNITS_HUNDREDTHS_POINT: u32 = 0x0100;
pub const wxTEXT_ATTR_UNITS_MASK: u32 = 0x010F;
//  ENUM: wxTextBoxAttrPosition
pub const wxTEXT_BOX_ATTR_POSITION_STATIC: u32 = 0x0000;
pub const wxTEXT_BOX_ATTR_POSITION_RELATIVE: u32 = 0x0010;
pub const wxTEXT_BOX_ATTR_POSITION_ABSOLUTE: u32 = 0x0020;
pub const wxTEXT_BOX_ATTR_POSITION_FIXED: u32 = 0x0040;
pub const wxTEXT_BOX_ATTR_POSITION_MASK: u32 = 0x00F0;
//  ENUM: wxTextAttrBorderStyle
pub const wxTEXT_BOX_ATTR_BORDER_NONE: u32 = 0;
pub const wxTEXT_BOX_ATTR_BORDER_SOLID: u32 = 1;
pub const wxTEXT_BOX_ATTR_BORDER_DOTTED: u32 = 2;
pub const wxTEXT_BOX_ATTR_BORDER_DASHED: u32 = 3;
pub const wxTEXT_BOX_ATTR_BORDER_DOUBLE: u32 = 4;
pub const wxTEXT_BOX_ATTR_BORDER_GROOVE: u32 = 5;
pub const wxTEXT_BOX_ATTR_BORDER_RIDGE: u32 = 6;
pub const wxTEXT_BOX_ATTR_BORDER_INSET: u32 = 7;
pub const wxTEXT_BOX_ATTR_BORDER_OUTSET: u32 = 8;
//  ENUM: wxTextAttrBorderFlags
pub const wxTEXT_BOX_ATTR_BORDER_STYLE: u32 = 0x0001;
pub const wxTEXT_BOX_ATTR_BORDER_COLOUR: u32 = 0x0002;
//  ENUM: wxTextAttrBorderWidth
pub const wxTEXT_BOX_ATTR_BORDER_THIN: i32 = -1;
pub const wxTEXT_BOX_ATTR_BORDER_MEDIUM: i32 = -2;
pub const wxTEXT_BOX_ATTR_BORDER_THICK: i32 = -3;
//  ENUM: wxTextBoxAttrFloatStyle
pub const wxTEXT_BOX_ATTR_FLOAT_NONE: u32 = 0;
pub const wxTEXT_BOX_ATTR_FLOAT_LEFT: u32 = 1;
pub const wxTEXT_BOX_ATTR_FLOAT_RIGHT: u32 = 2;
//  ENUM: wxTextBoxAttrClearStyle
pub const wxTEXT_BOX_ATTR_CLEAR_NONE: u32 = 0;
pub const wxTEXT_BOX_ATTR_CLEAR_LEFT: u32 = 1;
pub const wxTEXT_BOX_ATTR_CLEAR_RIGHT: u32 = 2;
pub const wxTEXT_BOX_ATTR_CLEAR_BOTH: u32 = 3;
//  ENUM: wxTextBoxAttrCollapseMode
pub const wxTEXT_BOX_ATTR_COLLAPSE_NONE: u32 = 0;
pub const wxTEXT_BOX_ATTR_COLLAPSE_FULL: u32 = 1;
//  ENUM: wxTextBoxAttrVerticalAlignment
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_NONE: u32 =       0;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_TOP: u32 =       1;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_CENTRE: u32 =     2;
pub const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_BOTTOM: u32 =    3;
//  ENUM: wxTextBoxAttrWhitespaceMode
pub const wxTEXT_BOX_ATTR_WHITESPACE_NONE: u32 = 0;
pub const wxTEXT_BOX_ATTR_WHITESPACE_NORMAL: u32 = 1;
pub const wxTEXT_BOX_ATTR_WHITESPACE_NO_WRAP: u32 = 2;
pub const wxTEXT_BOX_ATTR_WHITESPACE_PREFORMATTED: u32 = 3;
pub const wxTEXT_BOX_ATTR_WHITESPACE_PREFORMATTED_LINE: u32 = 4;
pub const wxTEXT_BOX_ATTR_WHITESPACE_PREFORMATTED_WRAP: u32 = 5;
//  ENUM: wxRichTextCommandId
pub const wxRICHTEXT_INSERT: u32 = 0;
pub const wxRICHTEXT_DELETE: u32 = 0 + 1;
pub const wxRICHTEXT_CHANGE_ATTRIBUTES: u32 = 0 + 2;
pub const wxRICHTEXT_CHANGE_STYLE: u32 = 0 + 3;
pub const wxRICHTEXT_CHANGE_OBJECT: u32 = 0 + 4;

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

//  ENUM: wxPathFormat
pub const wxPATH_NATIVE: u32 = 0;
pub const wxPATH_UNIX: u32 = 0 + 1;
pub const wxPATH_BEOS: u32 = wxPATH_UNIX;
pub const wxPATH_MAC: u32 = wxPATH_UNIX + 1;
pub const wxPATH_DOS: u32 = wxPATH_UNIX + 2;
pub const wxPATH_WIN: u32 = wxPATH_DOS;
pub const wxPATH_OS2: u32 = wxPATH_DOS;
pub const wxPATH_VMS: u32 = wxPATH_DOS + 1;
pub const wxPATH_MAX: u32 = wxPATH_DOS + 2;
//  ENUM: wxSizeConvention
pub const wxSIZE_CONV_TRADITIONAL: u32 = 0;
pub const wxSIZE_CONV_IEC: u32 = 0 + 1;
pub const wxSIZE_CONV_SI: u32 = 0 + 2;
//  ENUM: wxPathNormalize
pub const wxPATH_NORM_ENV_VARS: u32 = 0x0001;
pub const wxPATH_NORM_DOTS: u32 = 0x0002;
pub const wxPATH_NORM_TILDE: u32 = 0x0004;
pub const wxPATH_NORM_CASE: u32 = 0x0008;
pub const wxPATH_NORM_ABSOLUTE: u32 = 0x0010;
pub const wxPATH_NORM_LONG: u32 =     0x0020;
pub const wxPATH_NORM_SHORTCUT: u32 = 0x0040;
pub const wxPATH_NORM_ALL: u32 = 0x00ff & !wxPATH_NORM_CASE;
//  ENUM: @20
pub const wxPATH_RMDIR_FULL: u32 = 1;
pub const wxPATH_RMDIR_RECURSIVE: u32 = 2;
//  ENUM: @21
pub const wxFILE_EXISTS_REGULAR: u32 = 0x0001;
pub const wxFILE_EXISTS_DIR: u32 = 0x0002;
pub const wxFILE_EXISTS_SYMLINK: u32 = 0x1004;
pub const wxFILE_EXISTS_DEVICE: u32 = 0x0008;
pub const wxFILE_EXISTS_FIFO: u32 = 0x0016;
pub const wxFILE_EXISTS_SOCKET: u32 = 0x0032;
//  SKIP: wxFILE_EXISTS_NO_FOLLOW

//  ENUM: wxTaskBarIconType
pub const wxTBI_DOCK: u32 = 0;
pub const wxTBI_CUSTOM_STATUSITEM: u32 = 0 + 1;
pub const wxTBI_DEFAULT_TYPE: u32 = 0 + 2;

//  ENUM: wxAcceleratorEntryFlags
pub const wxACCEL_NORMAL: u32 = 0;
pub const wxACCEL_ALT: u32 = 0 + 1;
pub const wxACCEL_CTRL: u32 = 0 + 2;
pub const wxACCEL_SHIFT: u32 = 0 + 3;
pub const wxACCEL_RAW_CTRL: u32 = 0 + 4;
pub const wxACCEL_CMD: u32 = 0 + 5;

//  ENUM: wxMessageQueueError
pub const wxMSGQUEUE_NO_ERROR: u32 = 0;
pub const wxMSGQUEUE_TIMEOUT: u32 = 0 + 1;
pub const wxMSGQUEUE_MISC_ERROR: u32 = 0 + 2;

//  ENUM: BufMode

//  ENUM: Style
pub const Style_None: u32 = 0x00;
pub const Style_WithThousandsSep: u32 = 0x01;
pub const Style_NoTrailingZeroes: u32 = 0x02;

//  ENUM: wxFileSystemOpenFlags
pub const wxFS_READ: u32 = 1;
pub const wxFS_SEEKABLE: u32 = 4;

pub const wxLC_VRULES: u32 = 0x0001;
pub const wxLC_HRULES: u32 = 0x0002;
pub const wxLC_ICON: u32 = 0x0004;
pub const wxLC_SMALL_ICON: u32 = 0x0008;
pub const wxLC_LIST: u32 = 0x0010;
pub const wxLC_REPORT: u32 = 0x0020;
pub const wxLC_ALIGN_TOP: u32 = 0x0040;
pub const wxLC_ALIGN_LEFT: u32 = 0x0080;
pub const wxLC_AUTOARRANGE: u32 = 0x0100;
pub const wxLC_VIRTUAL: u32 = 0x0200;
pub const wxLC_EDIT_LABELS: u32 = 0x0400;
pub const wxLC_NO_HEADER: u32 = 0x0800;
pub const wxLC_NO_SORT_HEADER: u32 = 0x1000;
pub const wxLC_SINGLE_SEL: u32 = 0x2000;
pub const wxLC_SORT_ASCENDING: u32 = 0x4000;
pub const wxLC_SORT_DESCENDING: u32 = 0x8000;
pub const wxLC_MASK_TYPE: u32 = (wxLC_ICON | wxLC_SMALL_ICON | wxLC_LIST | wxLC_REPORT);
pub const wxLC_MASK_ALIGN: u32 = (wxLC_ALIGN_TOP | wxLC_ALIGN_LEFT);
pub const wxLC_MASK_SORT: u32 = (wxLC_SORT_ASCENDING | wxLC_SORT_DESCENDING);
pub const wxLIST_MASK_STATE: u32 = 0x0001;
pub const wxLIST_MASK_TEXT: u32 = 0x0002;
pub const wxLIST_MASK_IMAGE: u32 = 0x0004;
pub const wxLIST_MASK_DATA: u32 = 0x0008;
pub const wxLIST_SET_ITEM: u32 = 0x0010;
pub const wxLIST_MASK_WIDTH: u32 = 0x0020;
pub const wxLIST_MASK_FORMAT: u32 = 0x0040;
pub const wxLIST_STATE_DONTCARE: u32 = 0x0000;
pub const wxLIST_STATE_DROPHILITED: u32 = 0x0001;
pub const wxLIST_STATE_FOCUSED: u32 = 0x0002;
pub const wxLIST_STATE_SELECTED: u32 = 0x0004;
pub const wxLIST_STATE_CUT: u32 = 0x0008;
pub const wxLIST_HITTEST_ABOVE: u32 = 0x0001;
pub const wxLIST_HITTEST_BELOW: u32 = 0x0002;
pub const wxLIST_HITTEST_NOWHERE: u32 = 0x0004;
pub const wxLIST_HITTEST_ONITEMICON: u32 = 0x0020;
pub const wxLIST_HITTEST_ONITEMLABEL: u32 = 0x0080;
pub const wxLIST_HITTEST_ONITEMSTATEICON: u32 = 0x0200;
pub const wxLIST_HITTEST_TOLEFT: u32 = 0x0400;
pub const wxLIST_HITTEST_TORIGHT: u32 = 0x0800;
pub const wxLIST_HITTEST_ONITEM: u32 = (wxLIST_HITTEST_ONITEMICON | wxLIST_HITTEST_ONITEMLABEL | wxLIST_HITTEST_ONITEMSTATEICON);
pub const wxLIST_GETSUBITEMRECT_WHOLEITEM: i32 = -1;
//  ENUM: @32
pub const wxLIST_NEXT_ABOVE: u32 = 0;
pub const wxLIST_NEXT_ALL: u32 = 0 + 1;
pub const wxLIST_NEXT_BELOW: u32 = 0 + 2;
pub const wxLIST_NEXT_LEFT: u32 = 0 + 3;
pub const wxLIST_NEXT_RIGHT: u32 = 0 + 4;
//  ENUM: @33
pub const wxLIST_ALIGN_DEFAULT: u32 = 0;
pub const wxLIST_ALIGN_LEFT: u32 = 0 + 1;
pub const wxLIST_ALIGN_TOP: u32 = 0 + 2;
pub const wxLIST_ALIGN_SNAP_TO_GRID: u32 = 0 + 3;
//  ENUM: wxListColumnFormat
pub const wxLIST_FORMAT_LEFT: u32 = 0;
pub const wxLIST_FORMAT_RIGHT: u32 = 0 + 1;
pub const wxLIST_FORMAT_CENTRE: u32 = 0 + 2;
pub const wxLIST_FORMAT_CENTER: u32 = wxLIST_FORMAT_CENTRE;
//  ENUM: @34
pub const wxLIST_AUTOSIZE: i32 = -1;
pub const wxLIST_AUTOSIZE_USEHEADER: i32 = -2;
//  ENUM: @35
pub const wxLIST_RECT_BOUNDS: u32 = 0;
pub const wxLIST_RECT_ICON: u32 = 0 + 1;
pub const wxLIST_RECT_LABEL: u32 = 0 + 2;
//  ENUM: @36
pub const wxLIST_FIND_UP: u32 = 0;
pub const wxLIST_FIND_DOWN: u32 = 0 + 1;
pub const wxLIST_FIND_LEFT: u32 = 0 + 2;
pub const wxLIST_FIND_RIGHT: u32 = 0 + 3;

//  ENUM: @29
pub const FALLBACK_NONE: u32 = 0;
pub const FALLBACK_SYSTEM: u32 = 1;
pub const FALLBACK_NEAREST_LARGER: u32 = 2;

// NODEF: wxCHANGE_UMASK
//  ENUM: wxPosixPermissions
pub const wxS_IRUSR: u32 = 00400;
pub const wxS_IWUSR: u32 = 00200;
pub const wxS_IXUSR: u32 = 00100;
pub const wxS_IRGRP: u32 = 00040;
pub const wxS_IWGRP: u32 = 00020;
pub const wxS_IXGRP: u32 = 00010;
pub const wxS_IROTH: u32 = 00004;
pub const wxS_IWOTH: u32 = 00002;
pub const wxS_IXOTH: u32 = 00001;
pub const wxPOSIX_USER_READ: u32 = wxS_IRUSR;
pub const wxPOSIX_USER_WRITE: u32 = wxS_IWUSR;
pub const wxPOSIX_USER_EXECUTE: u32 = wxS_IXUSR;
pub const wxPOSIX_GROUP_READ: u32 = wxS_IRGRP;
pub const wxPOSIX_GROUP_WRITE: u32 = wxS_IWGRP;
pub const wxPOSIX_GROUP_EXECUTE: u32 = wxS_IXGRP;
pub const wxPOSIX_OTHERS_READ: u32 = wxS_IROTH;
pub const wxPOSIX_OTHERS_WRITE: u32 = wxS_IWOTH;
pub const wxPOSIX_OTHERS_EXECUTE: u32 = wxS_IXOTH;
pub const wxS_DEFAULT: u32 = (wxPOSIX_USER_READ | wxPOSIX_USER_WRITE | 
                   wxPOSIX_GROUP_READ | wxPOSIX_GROUP_WRITE | 
                   wxPOSIX_OTHERS_READ | wxPOSIX_OTHERS_WRITE);
pub const wxS_DIR_DEFAULT: u32 = (wxPOSIX_USER_READ | wxPOSIX_USER_WRITE | wxPOSIX_USER_EXECUTE | 
                       wxPOSIX_GROUP_READ | wxPOSIX_GROUP_WRITE | wxPOSIX_GROUP_EXECUTE | 
                       wxPOSIX_OTHERS_READ | wxPOSIX_OTHERS_WRITE | wxPOSIX_OTHERS_EXECUTE);
//  ENUM: wxSeekMode
pub const wxFromStart: u32 = 0;
pub const wxFromCurrent: u32 = 0 + 1;
pub const wxFromEnd: u32 = 0 + 2;
//  ENUM: wxFileKind
pub const wxFILE_KIND_UNKNOWN: u32 = 0;
pub const wxFILE_KIND_DISK: u32 = 0 + 1;
pub const wxFILE_KIND_TERMINAL: u32 = 0 + 2;
pub const wxFILE_KIND_PIPE: u32 = 0 + 3;

//  ENUM: Kind
pub const Kind_General: u32 = 0;
pub const Kind_Advanced: u32 = 0 + 1;

//  ENUM: wxOleConvertVariantFlags
pub const wxOleConvertVariant_Default: u32 = 0;
pub const wxOleConvertVariant_ReturnSafeArrays: u32 = 1;

pub const wxRICHTEXT_ORGANISER_DELETE_STYLES: u32 = 0x0001;
pub const wxRICHTEXT_ORGANISER_CREATE_STYLES: u32 = 0x0002;
pub const wxRICHTEXT_ORGANISER_APPLY_STYLES: u32 = 0x0004;
pub const wxRICHTEXT_ORGANISER_EDIT_STYLES: u32 = 0x0008;
pub const wxRICHTEXT_ORGANISER_RENAME_STYLES: u32 = 0x0010;
pub const wxRICHTEXT_ORGANISER_OK_CANCEL: u32 = 0x0020;
pub const wxRICHTEXT_ORGANISER_RENUMBER: u32 = 0x0040;
pub const wxRICHTEXT_ORGANISER_SHOW_CHARACTER: u32 = 0x0100;
pub const wxRICHTEXT_ORGANISER_SHOW_PARAGRAPH: u32 = 0x0200;
pub const wxRICHTEXT_ORGANISER_SHOW_LIST: u32 = 0x0400;
pub const wxRICHTEXT_ORGANISER_SHOW_BOX: u32 = 0x0800;
pub const wxRICHTEXT_ORGANISER_SHOW_ALL: u32 = 0x1000;
pub const wxRICHTEXT_ORGANISER_ORGANISE: u32 = (wxRICHTEXT_ORGANISER_SHOW_ALL|wxRICHTEXT_ORGANISER_DELETE_STYLES|wxRICHTEXT_ORGANISER_CREATE_STYLES|wxRICHTEXT_ORGANISER_APPLY_STYLES|wxRICHTEXT_ORGANISER_EDIT_STYLES|wxRICHTEXT_ORGANISER_RENAME_STYLES);
pub const wxRICHTEXT_ORGANISER_BROWSE: u32 = (wxRICHTEXT_ORGANISER_SHOW_ALL|wxRICHTEXT_ORGANISER_OK_CANCEL);
pub const wxRICHTEXT_ORGANISER_BROWSE_NUMBERING: u32 = (wxRICHTEXT_ORGANISER_SHOW_LIST|wxRICHTEXT_ORGANISER_OK_CANCEL|wxRICHTEXT_ORGANISER_RENUMBER);

pub const wxTE_NO_VSCROLL: u32 = 0x0002;
pub const wxTE_READONLY: u32 = 0x0010;
pub const wxTE_MULTILINE: u32 = 0x0020;
pub const wxTE_PROCESS_TAB: u32 = 0x0040;
pub const wxTE_LEFT: u32 = 0x0000;
pub const wxTE_CENTER: i32 = wxALIGN_CENTER_HORIZONTAL;
pub const wxTE_RIGHT: i32 = wxALIGN_RIGHT;
pub const wxTE_CENTRE: i32 = wxTE_CENTER;
pub const wxTE_RICH: u32 = 0x0080;
pub const wxTE_PROCESS_ENTER: u32 = 0x0400;
pub const wxTE_PASSWORD: u32 = 0x0800;
pub const wxTE_AUTO_URL: u32 = 0x1000;
pub const wxTE_NOHIDESEL: u32 = 0x2000;
pub const wxTE_DONTWRAP: u32 = wxHSCROLL;
pub const wxTE_CHARWRAP: u32 = 0x4000;
pub const wxTE_WORDWRAP: u32 = 0x0001;
pub const wxTE_BESTWRAP: u32 = 0x0000;
pub const wxTE_RICH2: u32 = 0x8000;
pub const wxTEXT_TYPE_ANY: u32 = 0;
//  ENUM: wxTextAttrAlignment
pub const wxTEXT_ALIGNMENT_DEFAULT: u32 = 0;
pub const wxTEXT_ALIGNMENT_LEFT: u32 = 0 + 1;
pub const wxTEXT_ALIGNMENT_CENTRE: u32 = 0 + 2;
pub const wxTEXT_ALIGNMENT_CENTER: u32 = wxTEXT_ALIGNMENT_CENTRE;
pub const wxTEXT_ALIGNMENT_RIGHT: u32 = wxTEXT_ALIGNMENT_CENTRE + 1;
pub const wxTEXT_ALIGNMENT_JUSTIFIED: u32 = wxTEXT_ALIGNMENT_CENTRE + 2;
//  ENUM: wxTextAttrFlags
pub const wxTEXT_ATTR_TEXT_COLOUR: u32 = 0x00000001;
pub const wxTEXT_ATTR_BACKGROUND_COLOUR: u32 = 0x00000002;
pub const wxTEXT_ATTR_FONT_FACE: u32 = 0x00000004;
pub const wxTEXT_ATTR_FONT_POINT_SIZE: u32 = 0x00000008;
pub const wxTEXT_ATTR_FONT_PIXEL_SIZE: u32 = 0x10000000;
pub const wxTEXT_ATTR_FONT_WEIGHT: u32 = 0x00000010;
pub const wxTEXT_ATTR_FONT_ITALIC: u32 = 0x00000020;
pub const wxTEXT_ATTR_FONT_UNDERLINE: u32 = 0x00000040;
pub const wxTEXT_ATTR_FONT_STRIKETHROUGH: u32 = 0x08000000;
pub const wxTEXT_ATTR_FONT_ENCODING: u32 = 0x02000000;
pub const wxTEXT_ATTR_FONT_FAMILY: u32 = 0x04000000;
pub const wxTEXT_ATTR_FONT_SIZE: u32 = 
        ( wxTEXT_ATTR_FONT_POINT_SIZE | wxTEXT_ATTR_FONT_PIXEL_SIZE );
pub const wxTEXT_ATTR_FONT: u32 = 
        ( wxTEXT_ATTR_FONT_FACE | wxTEXT_ATTR_FONT_SIZE | wxTEXT_ATTR_FONT_WEIGHT | 
            wxTEXT_ATTR_FONT_ITALIC | wxTEXT_ATTR_FONT_UNDERLINE | wxTEXT_ATTR_FONT_STRIKETHROUGH | wxTEXT_ATTR_FONT_ENCODING | wxTEXT_ATTR_FONT_FAMILY );
pub const wxTEXT_ATTR_ALIGNMENT: u32 = 0x00000080;
pub const wxTEXT_ATTR_LEFT_INDENT: u32 = 0x00000100;
pub const wxTEXT_ATTR_RIGHT_INDENT: u32 = 0x00000200;
pub const wxTEXT_ATTR_TABS: u32 = 0x00000400;
pub const wxTEXT_ATTR_PARA_SPACING_AFTER: u32 = 0x00000800;
pub const wxTEXT_ATTR_PARA_SPACING_BEFORE: u32 = 0x00001000;
pub const wxTEXT_ATTR_LINE_SPACING: u32 = 0x00002000;
pub const wxTEXT_ATTR_CHARACTER_STYLE_NAME: u32 = 0x00004000;
pub const wxTEXT_ATTR_PARAGRAPH_STYLE_NAME: u32 = 0x00008000;
pub const wxTEXT_ATTR_LIST_STYLE_NAME: u32 = 0x00010000;
pub const wxTEXT_ATTR_BULLET_STYLE: u32 = 0x00020000;
pub const wxTEXT_ATTR_BULLET_NUMBER: u32 = 0x00040000;
pub const wxTEXT_ATTR_BULLET_TEXT: u32 = 0x00080000;
pub const wxTEXT_ATTR_BULLET_NAME: u32 = 0x00100000;
pub const wxTEXT_ATTR_BULLET: u32 = 
        ( wxTEXT_ATTR_BULLET_STYLE | wxTEXT_ATTR_BULLET_NUMBER | wxTEXT_ATTR_BULLET_TEXT | 
          wxTEXT_ATTR_BULLET_NAME );
pub const wxTEXT_ATTR_URL: u32 = 0x00200000;
pub const wxTEXT_ATTR_PAGE_BREAK: u32 = 0x00400000;
pub const wxTEXT_ATTR_EFFECTS: u32 = 0x00800000;
pub const wxTEXT_ATTR_OUTLINE_LEVEL: u32 = 0x01000000;
pub const wxTEXT_ATTR_AVOID_PAGE_BREAK_BEFORE: u32 = 0x20000000;
pub const wxTEXT_ATTR_AVOID_PAGE_BREAK_AFTER: u32 =  0x40000000;
pub const wxTEXT_ATTR_CHARACTER: u32 = 
        (wxTEXT_ATTR_FONT|wxTEXT_ATTR_EFFECTS| 
            wxTEXT_ATTR_BACKGROUND_COLOUR|wxTEXT_ATTR_TEXT_COLOUR|wxTEXT_ATTR_CHARACTER_STYLE_NAME|wxTEXT_ATTR_URL);
pub const wxTEXT_ATTR_PARAGRAPH: u32 = 
        (wxTEXT_ATTR_ALIGNMENT|wxTEXT_ATTR_LEFT_INDENT|wxTEXT_ATTR_RIGHT_INDENT|wxTEXT_ATTR_TABS|
            wxTEXT_ATTR_PARA_SPACING_BEFORE|wxTEXT_ATTR_PARA_SPACING_AFTER|wxTEXT_ATTR_LINE_SPACING|
            wxTEXT_ATTR_BULLET|wxTEXT_ATTR_PARAGRAPH_STYLE_NAME|wxTEXT_ATTR_LIST_STYLE_NAME|wxTEXT_ATTR_OUTLINE_LEVEL|
            wxTEXT_ATTR_PAGE_BREAK|wxTEXT_ATTR_AVOID_PAGE_BREAK_BEFORE|wxTEXT_ATTR_AVOID_PAGE_BREAK_AFTER);
pub const wxTEXT_ATTR_ALL: u32 = (wxTEXT_ATTR_CHARACTER|wxTEXT_ATTR_PARAGRAPH);
//  ENUM: wxTextAttrBulletStyle
pub const wxTEXT_ATTR_BULLET_STYLE_NONE: u32 = 0x00000000;
pub const wxTEXT_ATTR_BULLET_STYLE_ARABIC: u32 = 0x00000001;
pub const wxTEXT_ATTR_BULLET_STYLE_LETTERS_UPPER: u32 = 0x00000002;
pub const wxTEXT_ATTR_BULLET_STYLE_LETTERS_LOWER: u32 = 0x00000004;
pub const wxTEXT_ATTR_BULLET_STYLE_ROMAN_UPPER: u32 = 0x00000008;
pub const wxTEXT_ATTR_BULLET_STYLE_ROMAN_LOWER: u32 = 0x00000010;
pub const wxTEXT_ATTR_BULLET_STYLE_SYMBOL: u32 = 0x00000020;
pub const wxTEXT_ATTR_BULLET_STYLE_BITMAP: u32 = 0x00000040;
pub const wxTEXT_ATTR_BULLET_STYLE_PARENTHESES: u32 = 0x00000080;
pub const wxTEXT_ATTR_BULLET_STYLE_PERIOD: u32 = 0x00000100;
pub const wxTEXT_ATTR_BULLET_STYLE_STANDARD: u32 = 0x00000200;
pub const wxTEXT_ATTR_BULLET_STYLE_RIGHT_PARENTHESIS: u32 = 0x00000400;
pub const wxTEXT_ATTR_BULLET_STYLE_OUTLINE: u32 = 0x00000800;
pub const wxTEXT_ATTR_BULLET_STYLE_ALIGN_LEFT: u32 = 0x00000000;
pub const wxTEXT_ATTR_BULLET_STYLE_ALIGN_RIGHT: u32 = 0x00001000;
pub const wxTEXT_ATTR_BULLET_STYLE_ALIGN_CENTRE: u32 = 0x00002000;
pub const wxTEXT_ATTR_BULLET_STYLE_CONTINUATION: u32 = 0x00004000;
//  ENUM: wxTextAttrEffects
pub const wxTEXT_ATTR_EFFECT_NONE: u32 = 0x00000000;
pub const wxTEXT_ATTR_EFFECT_CAPITALS: u32 = 0x00000001;
pub const wxTEXT_ATTR_EFFECT_SMALL_CAPITALS: u32 = 0x00000002;
pub const wxTEXT_ATTR_EFFECT_STRIKETHROUGH: u32 = 0x00000004;
pub const wxTEXT_ATTR_EFFECT_DOUBLE_STRIKETHROUGH: u32 = 0x00000008;
pub const wxTEXT_ATTR_EFFECT_SHADOW: u32 = 0x00000010;
pub const wxTEXT_ATTR_EFFECT_EMBOSS: u32 = 0x00000020;
pub const wxTEXT_ATTR_EFFECT_OUTLINE: u32 = 0x00000040;
pub const wxTEXT_ATTR_EFFECT_ENGRAVE: u32 = 0x00000080;
pub const wxTEXT_ATTR_EFFECT_SUPERSCRIPT: u32 = 0x00000100;
pub const wxTEXT_ATTR_EFFECT_SUBSCRIPT: u32 = 0x00000200;
pub const wxTEXT_ATTR_EFFECT_RTL: u32 = 0x00000400;
pub const wxTEXT_ATTR_EFFECT_SUPPRESS_HYPHENATION: u32 = 0x00001000;
//  ENUM: wxTextAttrLineSpacing
pub const wxTEXT_ATTR_LINE_SPACING_NORMAL: u32 = 10;
pub const wxTEXT_ATTR_LINE_SPACING_HALF: u32 = 15;
pub const wxTEXT_ATTR_LINE_SPACING_TWICE: u32 = 20;
//  ENUM: wxTextAttrUnderlineType
pub const wxTEXT_ATTR_UNDERLINE_NONE: u32 = 0;
pub const wxTEXT_ATTR_UNDERLINE_SOLID: u32 = 0 + 1;
pub const wxTEXT_ATTR_UNDERLINE_DOUBLE: u32 = 0 + 2;
pub const wxTEXT_ATTR_UNDERLINE_SPECIAL: u32 = 0 + 3;
//  ENUM: wxTextCtrlHitTestResult
pub const wxTE_HT_UNKNOWN: i32 = -2;
pub const wxTE_HT_BEFORE: i32 = -2 + 1;
pub const wxTE_HT_ON_TEXT: i32 = -2 + 2;
pub const wxTE_HT_BELOW: i32 = -2 + 3;
pub const wxTE_HT_BEYOND: i32 = -2 + 4;

//  ENUM: ScaleMode
pub const Scale_None: u32 = 0;
pub const Scale_Fill: u32 = 0 + 1;
pub const Scale_AspectFit: u32 = 0 + 2;
pub const Scale_AspectFill: u32 = 0 + 3;

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

//  ENUM: wxPowerType
pub const wxPOWER_SOCKET: u32 = 0;
pub const wxPOWER_BATTERY: u32 = 0 + 1;
pub const wxPOWER_UNKNOWN: u32 = 0 + 2;
//  ENUM: wxBatteryState
pub const wxBATTERY_NORMAL_STATE: u32 = 0;
pub const wxBATTERY_LOW_STATE: u32 = 0 + 1;
pub const wxBATTERY_CRITICAL_STATE: u32 = 0 + 2;
pub const wxBATTERY_SHUTDOWN_STATE: u32 = 0 + 3;
pub const wxBATTERY_UNKNOWN_STATE: u32 = 0 + 4;
//  ENUM: wxPowerResourceKind
pub const wxPOWER_RESOURCE_SCREEN: u32 = 0;
pub const wxPOWER_RESOURCE_SYSTEM: u32 = 0 + 1;

//  ENUM: wxRibbonButtonBarButtonState
pub const wxRIBBON_BUTTONBAR_BUTTON_SMALL: u32 = 0 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_MEDIUM: u32 = 1 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_LARGE: u32 = 2 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_SIZE_MASK: u32 = 3 << 0;
pub const wxRIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED: u32 = 1 << 3;
pub const wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED: u32 = 1 << 4;
pub const wxRIBBON_BUTTONBAR_BUTTON_HOVER_MASK: u32 = wxRIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED | wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED;
pub const wxRIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE: u32 = 1 << 5;
pub const wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE: u32 = 1 << 6;
pub const wxRIBBON_BUTTONBAR_BUTTON_ACTIVE_MASK: u32 = wxRIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE | wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE;
pub const wxRIBBON_BUTTONBAR_BUTTON_DISABLED: u32 = 1 << 7;
pub const wxRIBBON_BUTTONBAR_BUTTON_TOGGLED: u32 = 1 << 8;
pub const wxRIBBON_BUTTONBAR_BUTTON_STATE_MASK: u32 = 0x1F8;

//  ENUM: wxRasterOperationMode
pub const wxCLEAR: u32 = 0;
pub const wxXOR: u32 = 0 + 1;
pub const wxINVERT: u32 = 0 + 2;
pub const wxOR_REVERSE: u32 = 0 + 3;
pub const wxAND_REVERSE: u32 = 0 + 4;
pub const wxCOPY: u32 = 0 + 5;
pub const wxAND: u32 = 0 + 6;
pub const wxAND_INVERT: u32 = 0 + 7;
pub const wxNO_OP: u32 = 0 + 8;
pub const wxNOR: u32 = 0 + 9;
pub const wxEQUIV: u32 = 0 + 10;
pub const wxSRC_INVERT: u32 = 0 + 11;
pub const wxOR_INVERT: u32 = 0 + 12;
pub const wxNAND: u32 = 0 + 13;
pub const wxOR: u32 = 0 + 14;
pub const wxSET: u32 = 0 + 15;
//  ENUM: wxFloodFillStyle
pub const wxFLOOD_SURFACE: u32 = 1;
pub const wxFLOOD_BORDER: u32 = 1 + 1;
//  ENUM: wxMappingMode
pub const wxMM_TEXT: u32 = 1;
pub const wxMM_METRIC: u32 = 1 + 1;
pub const wxMM_LOMETRIC: u32 = 1 + 2;
pub const wxMM_TWIPS: u32 = 1 + 3;
pub const wxMM_POINTS: u32 = 1 + 4;

//  ENUM: wxTaskBarButtonState
pub const wxTASKBAR_BUTTON_NO_PROGRESS: u32 = 0;
pub const wxTASKBAR_BUTTON_INDETERMINATE: u32 = 1;
pub const wxTASKBAR_BUTTON_NORMAL: u32 = 2;
pub const wxTASKBAR_BUTTON_ERROR: u32 = 4;
pub const wxTASKBAR_BUTTON_PAUSED: u32 = 8;
//  ENUM: wxTaskBarJumpListItemType
pub const wxTASKBAR_JUMP_LIST_SEPARATOR: u32 = 0;
pub const wxTASKBAR_JUMP_LIST_TASK: u32 = 0 + 1;
pub const wxTASKBAR_JUMP_LIST_DESTINATION: u32 = 0 + 2;

// NODEF: wxCONCAT
// NODEF: wxCONCAT3
// NODEF: wxCONCAT4
// NODEF: wxCONCAT5
// NODEF: wxSTRINGIZE
// NODEF: wxSTRINGIZE_T
// NODEF: __WXFUNCTION__

pub const wxICON_SCREEN_DEPTH: i32 = (-1);

//  ENUM: wxPenStyle
pub const wxPENSTYLE_INVALID: i32 = -1;
pub const wxPENSTYLE_SOLID: i32 = -1 + 1;
pub const wxPENSTYLE_DOT: i32 = -1 + 2;
pub const wxPENSTYLE_LONG_DASH: i32 = -1 + 3;
pub const wxPENSTYLE_SHORT_DASH: i32 = -1 + 4;
pub const wxPENSTYLE_DOT_DASH: i32 = -1 + 5;
pub const wxPENSTYLE_USER_DASH: i32 = -1 + 6;
pub const wxPENSTYLE_TRANSPARENT: i32 = -1 + 7;
pub const wxPENSTYLE_STIPPLE_MASK_OPAQUE: i32 = -1 + 8;
pub const wxPENSTYLE_STIPPLE_MASK: i32 = -1 + 9;
pub const wxPENSTYLE_STIPPLE: i32 = -1 + 10;
pub const wxPENSTYLE_BDIAGONAL_HATCH: i32 = -1 + 11;
pub const wxPENSTYLE_CROSSDIAG_HATCH: i32 = -1 + 12;
pub const wxPENSTYLE_FDIAGONAL_HATCH: i32 = -1 + 13;
pub const wxPENSTYLE_CROSS_HATCH: i32 = -1 + 14;
pub const wxPENSTYLE_HORIZONTAL_HATCH: i32 = -1 + 15;
pub const wxPENSTYLE_VERTICAL_HATCH: i32 = -1 + 16;
pub const wxPENSTYLE_FIRST_HATCH: i32 = -1 + 17;
pub const wxPENSTYLE_LAST_HATCH: i32 = -1 + 18;
//  ENUM: wxPenQuality
pub const wxPEN_QUALITY_DEFAULT: u32 = 0;
pub const wxPEN_QUALITY_LOW: u32 = 0 + 1;
pub const wxPEN_QUALITY_HIGH: u32 = 0 + 2;
//  ENUM: wxPenJoin
pub const wxJOIN_INVALID: i32 = -1;
pub const wxJOIN_BEVEL: i32 = 120;
pub const wxJOIN_MITER: i32 = 120 + 1;
pub const wxJOIN_ROUND: i32 = 120 + 2;
//  ENUM: wxPenCap
pub const wxCAP_INVALID: i32 = -1;
pub const wxCAP_ROUND: i32 = 130;
pub const wxCAP_PROJECTING: i32 = 130 + 1;
pub const wxCAP_BUTT: i32 = 130 + 2;

//  ENUM: wxRichTextStyleType
pub const wxRICHTEXT_STYLE_ALL: u32 = 0;
pub const wxRICHTEXT_STYLE_PARAGRAPH: u32 = 0 + 1;
pub const wxRICHTEXT_STYLE_CHARACTER: u32 = 0 + 2;
pub const wxRICHTEXT_STYLE_LIST: u32 = 0 + 3;
pub const wxRICHTEXT_STYLE_BOX: u32 = 0 + 4;

pub const wxSOUND_SYNC: u32 = 0;
pub const wxSOUND_ASYNC: u32 = 1;
pub const wxSOUND_LOOP: u32 = 2;

pub const wxID_HTML_HELPFRAME: i32 = (wxID_HIGHEST + 1);
pub const wxHF_EMBEDDED: u32 = 0x00008000;
pub const wxHF_DIALOG: u32 = 0x00010000;
pub const wxHF_FRAME: u32 = 0x00020000;
pub const wxHF_MODAL: u32 = 0x00040000;


pub const wxFNTP_FONTDESC_AS_LABEL: u32 = 0x0008;
pub const wxFNTP_USEFONT_FOR_LABEL: u32 = 0x0010;
pub const wxFONTBTN_DEFAULT_STYLE: u32 = (wxFNTP_FONTDESC_AS_LABEL | wxFNTP_USEFONT_FOR_LABEL);
pub const wxFNTP_USE_TEXTCTRL: u32 = (wxPB_USE_TEXTCTRL);
pub const wxFNTP_DEFAULT_STYLE: u32 = (wxFNTP_FONTDESC_AS_LABEL|wxFNTP_USEFONT_FOR_LABEL);

//  ENUM: wxNumValidatorStyle
pub const wxNUM_VAL_DEFAULT: u32 = 0;
pub const wxNUM_VAL_THOUSANDS_SEPARATOR: u32 = 1;
pub const wxNUM_VAL_ZERO_AS_BLANK: u32 = 2;
pub const wxNUM_VAL_NO_TRAILING_ZEROES: u32 = 2 + 1;

//  ENUM: wxAntialiasMode
pub const wxANTIALIAS_NONE: u32 = 0;
pub const wxANTIALIAS_DEFAULT: u32 = 0 + 1;
//  ENUM: wxInterpolationQuality
pub const wxINTERPOLATION_DEFAULT: u32 = 0;
pub const wxINTERPOLATION_NONE: u32 = 0 + 1;
pub const wxINTERPOLATION_FAST: u32 = 0 + 2;
pub const wxINTERPOLATION_GOOD: u32 = 0 + 3;
pub const wxINTERPOLATION_BEST: u32 = 0 + 4;
//  ENUM: wxCompositionMode
pub const wxCOMPOSITION_INVALID: i32 = -1;
pub const wxCOMPOSITION_CLEAR: i32 = -1 + 1;
pub const wxCOMPOSITION_SOURCE: i32 = -1 + 2;
pub const wxCOMPOSITION_OVER: i32 = -1 + 3;
pub const wxCOMPOSITION_IN: i32 = -1 + 4;
pub const wxCOMPOSITION_OUT: i32 = -1 + 5;
pub const wxCOMPOSITION_ATOP: i32 = -1 + 6;
pub const wxCOMPOSITION_DEST: i32 = -1 + 7;
pub const wxCOMPOSITION_DEST_OVER: i32 = -1 + 8;
pub const wxCOMPOSITION_DEST_IN: i32 = -1 + 9;
pub const wxCOMPOSITION_DEST_OUT: i32 = -1 + 10;
pub const wxCOMPOSITION_DEST_ATOP: i32 = -1 + 11;
pub const wxCOMPOSITION_XOR: i32 = -1 + 12;
pub const wxCOMPOSITION_ADD: i32 = -1 + 13;
//  ENUM: wxGradientType
pub const wxGRADIENT_NONE: u32 = 0;
pub const wxGRADIENT_LINEAR: u32 = 0 + 1;
pub const wxGRADIENT_RADIAL: u32 = 0 + 2;

//  ENUM: @40
pub const Selected: u32 = 0x00010000;
pub const ChoicePopup: u32 = 0x00020000;
pub const Control: u32 = 0x00040000;
pub const Disabled: u32 = 0x00080000;
pub const DontUseCellFgCol: u32 = 0x00100000;
pub const DontUseCellBgCol: u32 = 0x00200000;
pub const DontUseCellColours: u32 = DontUseCellFgCol |
                              DontUseCellBgCol;

//  ENUM: wxMessageQueueError

// NODEF: wxFORCE_LINK_THIS_MODULE
// NODEF: wxFORCE_LINK_MODULE

//  ENUM: @44
pub const wxRICHTEXT_FIELD_STYLE_COMPOSITE: u32 = 0x01;
pub const wxRICHTEXT_FIELD_STYLE_RECTANGLE: u32 = 0x02;
pub const wxRICHTEXT_FIELD_STYLE_NO_BORDER: u32 = 0x04;
pub const wxRICHTEXT_FIELD_STYLE_START_TAG: u32 = 0x08;
pub const wxRICHTEXT_FIELD_STYLE_END_TAG: u32 = 0x10;

//  ENUM: wxMediaState
pub const wxMEDIASTATE_STOPPED: u32 = 0;
pub const wxMEDIASTATE_PAUSED: u32 = 0 + 1;
pub const wxMEDIASTATE_PLAYING: u32 = 0 + 2;
//  ENUM: wxMediaCtrlPlayerControls
pub const wxMEDIACTRLPLAYERCONTROLS_NONE: u32 =   0;
pub const wxMEDIACTRLPLAYERCONTROLS_STEP: u32 =   1 << 0;
pub const wxMEDIACTRLPLAYERCONTROLS_VOLUME: u32 =   1 << 1;
pub const wxMEDIACTRLPLAYERCONTROLS_DEFAULT: u32 =
                    wxMEDIACTRLPLAYERCONTROLS_STEP |
                    wxMEDIACTRLPLAYERCONTROLS_VOLUME;

// NODEF: wxBITMAP
// NODEF: wxBITMAP_PNG
// NODEF: wxBITMAP_PNG_FROM_DATA
// NODEF: wxICON
//  ENUM: wxBitmapType
pub const wxBITMAP_TYPE_INVALID: u32 = 0;
pub const wxBITMAP_TYPE_BMP: u32 = 0 + 1;
pub const wxBITMAP_TYPE_BMP_RESOURCE: u32 = 0 + 2;
pub const wxBITMAP_TYPE_RESOURCE: u32 = wxBITMAP_TYPE_BMP_RESOURCE;
pub const wxBITMAP_TYPE_ICO: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 1;
pub const wxBITMAP_TYPE_ICO_RESOURCE: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 2;
pub const wxBITMAP_TYPE_CUR: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 3;
pub const wxBITMAP_TYPE_CUR_RESOURCE: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 4;
pub const wxBITMAP_TYPE_XBM: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 5;
pub const wxBITMAP_TYPE_XBM_DATA: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 6;
pub const wxBITMAP_TYPE_XPM: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 7;
pub const wxBITMAP_TYPE_XPM_DATA: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 8;
pub const wxBITMAP_TYPE_TIFF: u32 = wxBITMAP_TYPE_BMP_RESOURCE + 9;
pub const wxBITMAP_TYPE_TIF: u32 = wxBITMAP_TYPE_TIFF;
pub const wxBITMAP_TYPE_TIFF_RESOURCE: u32 = wxBITMAP_TYPE_TIFF + 1;
pub const wxBITMAP_TYPE_TIF_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE;
pub const wxBITMAP_TYPE_GIF: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 1;
pub const wxBITMAP_TYPE_GIF_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 2;
pub const wxBITMAP_TYPE_PNG: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 3;
pub const wxBITMAP_TYPE_PNG_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 4;
pub const wxBITMAP_TYPE_JPEG: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 5;
pub const wxBITMAP_TYPE_JPEG_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 6;
pub const wxBITMAP_TYPE_PNM: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 7;
pub const wxBITMAP_TYPE_PNM_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 8;
pub const wxBITMAP_TYPE_PCX: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 9;
pub const wxBITMAP_TYPE_PCX_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 10;
pub const wxBITMAP_TYPE_PICT: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 11;
pub const wxBITMAP_TYPE_PICT_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 12;
pub const wxBITMAP_TYPE_ICON: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 13;
pub const wxBITMAP_TYPE_ICON_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 14;
pub const wxBITMAP_TYPE_ANI: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 15;
pub const wxBITMAP_TYPE_IFF: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 16;
pub const wxBITMAP_TYPE_TGA: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 17;
pub const wxBITMAP_TYPE_MACCURSOR: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 18;
pub const wxBITMAP_TYPE_MACCURSOR_RESOURCE: u32 = wxBITMAP_TYPE_TIFF_RESOURCE + 19;
pub const wxBITMAP_TYPE_ANY: u32 = 50;
//  ENUM: wxPolygonFillMode
pub const wxODDEVEN_RULE: u32 = 1;
pub const wxWINDING_RULE: u32 = 1 + 1;
//  ENUM: wxStockCursor
pub const wxCURSOR_NONE: u32 = 0;
pub const wxCURSOR_ARROW: u32 = 0 + 1;
pub const wxCURSOR_RIGHT_ARROW: u32 = 0 + 2;
pub const wxCURSOR_BULLSEYE: u32 = 0 + 3;
pub const wxCURSOR_CHAR: u32 = 0 + 4;
pub const wxCURSOR_CROSS: u32 = 0 + 5;
pub const wxCURSOR_HAND: u32 = 0 + 6;
pub const wxCURSOR_IBEAM: u32 = 0 + 7;
pub const wxCURSOR_LEFT_BUTTON: u32 = 0 + 8;
pub const wxCURSOR_MAGNIFIER: u32 = 0 + 9;
pub const wxCURSOR_MIDDLE_BUTTON: u32 = 0 + 10;
pub const wxCURSOR_NO_ENTRY: u32 = 0 + 11;
pub const wxCURSOR_PAINT_BRUSH: u32 = 0 + 12;
pub const wxCURSOR_PENCIL: u32 = 0 + 13;
pub const wxCURSOR_POINT_LEFT: u32 = 0 + 14;
pub const wxCURSOR_POINT_RIGHT: u32 = 0 + 15;
pub const wxCURSOR_QUESTION_ARROW: u32 = 0 + 16;
pub const wxCURSOR_RIGHT_BUTTON: u32 = 0 + 17;
pub const wxCURSOR_SIZENESW: u32 = 0 + 18;
pub const wxCURSOR_SIZENS: u32 = 0 + 19;
pub const wxCURSOR_SIZENWSE: u32 = 0 + 20;
pub const wxCURSOR_SIZEWE: u32 = 0 + 21;
pub const wxCURSOR_SIZING: u32 = 0 + 22;
pub const wxCURSOR_SPRAYCAN: u32 = 0 + 23;
pub const wxCURSOR_WAIT: u32 = 0 + 24;
pub const wxCURSOR_WATCH: u32 = 0 + 25;
pub const wxCURSOR_BLANK: u32 = 0 + 26;
pub const wxCURSOR_DEFAULT: u32 = 0 + 27;
pub const wxCURSOR_COPY_ARROW: u32 = 0 + 28;
pub const wxCURSOR_CROSS_REVERSE: u32 = 0 + 29;
pub const wxCURSOR_DOUBLE_ARROW: u32 = 0 + 30;
pub const wxCURSOR_BASED_ARROW_UP: u32 = 0 + 31;
pub const wxCURSOR_BASED_ARROW_DOWN: u32 = 0 + 32;
pub const wxCURSOR_ARROWWAIT: u32 = 0 + 33;
pub const wxCURSOR_MAX: u32 = 0 + 34;
//  ENUM: wxEllipsizeFlags
pub const wxELLIPSIZE_FLAGS_NONE: u32 = 0;
pub const wxELLIPSIZE_FLAGS_PROCESS_MNEMONICS: u32 = 1;
pub const wxELLIPSIZE_FLAGS_EXPAND_TABS: u32 = 2;
pub const wxELLIPSIZE_FLAGS_DEFAULT: u32 = wxELLIPSIZE_FLAGS_PROCESS_MNEMONICS|
                                wxELLIPSIZE_FLAGS_EXPAND_TABS;
//  ENUM: wxEllipsizeMode
pub const wxELLIPSIZE_NONE: u32 = 0;
pub const wxELLIPSIZE_START: u32 = 0 + 1;
pub const wxELLIPSIZE_MIDDLE: u32 = 0 + 2;
pub const wxELLIPSIZE_END: u32 = 0 + 3;

pub const wxRICHTEXT_FORMAT_STYLE_EDITOR: u32 = 0x0001;
pub const wxRICHTEXT_FORMAT_FONT: u32 = 0x0002;
pub const wxRICHTEXT_FORMAT_TABS: u32 = 0x0004;
pub const wxRICHTEXT_FORMAT_BULLETS: u32 = 0x0008;
pub const wxRICHTEXT_FORMAT_INDENTS_SPACING: u32 = 0x0010;


//  ENUM: wxNumValidatorStyle

pub const wxGA_HORIZONTAL: u32 = wxHORIZONTAL;
pub const wxGA_VERTICAL: u32 = wxVERTICAL;
pub const wxGA_PROGRESS: u32 = 0x0010;
pub const wxGA_SMOOTH: u32 = 0x0020;
pub const wxGA_TEXT: u32 = 0x0040;

pub const wxHTML_ALIGN_LEFT: u32 = 0x0000;
pub const wxHTML_ALIGN_RIGHT: u32 = 0x0002;
pub const wxHTML_ALIGN_JUSTIFY: u32 = 0x0010;
pub const wxHTML_ALIGN_TOP: u32 = 0x0004;
pub const wxHTML_ALIGN_BOTTOM: u32 = 0x0008;
pub const wxHTML_ALIGN_CENTER: u32 = 0x0001;
pub const wxHTML_CLR_FOREGROUND: u32 = 0x0001;
pub const wxHTML_CLR_BACKGROUND: u32 = 0x0002;
pub const wxHTML_CLR_TRANSPARENT_BACKGROUND: u32 = 0x0004;
pub const wxHTML_UNITS_PIXELS: u32 = 0x0001;
pub const wxHTML_UNITS_PERCENT: u32 = 0x0002;
pub const wxHTML_INDENT_LEFT: u32 = 0x0010;
pub const wxHTML_INDENT_RIGHT: u32 = 0x0020;
pub const wxHTML_INDENT_TOP: u32 = 0x0040;
pub const wxHTML_INDENT_BOTTOM: u32 = 0x0080;
pub const wxHTML_INDENT_HORIZONTAL: u32 = (wxHTML_INDENT_LEFT | wxHTML_INDENT_RIGHT);
pub const wxHTML_INDENT_VERTICAL: u32 = (wxHTML_INDENT_TOP | wxHTML_INDENT_BOTTOM);
pub const wxHTML_INDENT_ALL: u32 = (wxHTML_INDENT_VERTICAL | wxHTML_INDENT_HORIZONTAL);
pub const wxHTML_COND_ISANCHOR: u32 = 1;
pub const wxHTML_COND_ISIMAGEMAP: u32 = 2;
pub const wxHTML_COND_USER: u32 = 10000;

//  ENUM: wxPropertySheetDialogFlags
pub const wxPROPSHEET_DEFAULT: u32 = 0x0001;
pub const wxPROPSHEET_NOTEBOOK: u32 = 0x0002;
pub const wxPROPSHEET_TOOLBOOK: u32 = 0x0004;
pub const wxPROPSHEET_CHOICEBOOK: u32 = 0x0008;
pub const wxPROPSHEET_LISTBOOK: u32 = 0x0010;
pub const wxPROPSHEET_BUTTONTOOLBOOK: u32 = 0x0020;
pub const wxPROPSHEET_TREEBOOK: u32 = 0x0040;
pub const wxPROPSHEET_SHRINKTOFIT: u32 = 0x0100;

//  SKIP: wxTLS_TYPE
// NODEF: wxTLS_VALUE
// NODEF: wxTLS_PTR

pub const wxPG_PROP_PASSWORD: u32 = wxPG_PROP_CLASS_SPECIFIC_2;
pub const wxPG_PROP_STATIC_CHOICES: u32 = wxPG_PROP_CLASS_SPECIFIC_1;
pub const wxPG_PROP_SHOW_FULL_FILENAME: u32 = wxPG_PROP_CLASS_SPECIFIC_1;
pub const wxPG_PROP_ACTIVE_BTN: u32 = wxPG_PROP_CLASS_SPECIFIC_1;
pub const wxPG_PROP_USE_CHECKBOX: u32 = wxPG_PROP_CLASS_SPECIFIC_1;
pub const wxPG_PROP_USE_DCC: u32 = wxPG_PROP_CLASS_SPECIFIC_2;
pub const wxAEDIALOG_STYLE: u32 = (wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER | wxOK | wxCANCEL | wxCENTRE);
//  ENUM: wxPGNumericValidationConstants
pub const wxPG_PROPERTY_VALIDATION_ERROR_MESSAGE: u32 = 0;
pub const wxPG_PROPERTY_VALIDATION_SATURATE: u32 = 1;
pub const wxPG_PROPERTY_VALIDATION_WRAP: u32 = 2;

//  ENUM: wxCmdLineEntryFlags
pub const wxCMD_LINE_OPTION_MANDATORY: u32 = 0x01;
pub const wxCMD_LINE_PARAM_OPTIONAL: u32 = 0x02;
pub const wxCMD_LINE_PARAM_MULTIPLE: u32 = 0x04;
pub const wxCMD_LINE_OPTION_HELP: u32 = 0x08;
pub const wxCMD_LINE_NEEDS_SEPARATOR: u32 = 0x10;
pub const wxCMD_LINE_SWITCH_NEGATABLE: u32 = 0x20;
pub const wxCMD_LINE_HIDDEN: u32 = 0x40;
//  ENUM: wxCmdLineParamType
pub const wxCMD_LINE_VAL_STRING: u32 = 0;
pub const wxCMD_LINE_VAL_NUMBER: u32 = 0 + 1;
pub const wxCMD_LINE_VAL_DATE: u32 = 0 + 2;
pub const wxCMD_LINE_VAL_DOUBLE: u32 = 0 + 3;
pub const wxCMD_LINE_VAL_NONE: u32 = 0 + 4;
//  ENUM: wxCmdLineEntryType
pub const wxCMD_LINE_SWITCH: u32 = 0;
pub const wxCMD_LINE_OPTION: u32 = 0 + 1;
pub const wxCMD_LINE_PARAM: u32 = 0 + 2;
pub const wxCMD_LINE_USAGE_TEXT: u32 = 0 + 3;
pub const wxCMD_LINE_NONE: u32 = 0 + 4;
//  ENUM: wxCmdLineSwitchState
pub const wxCMD_SWITCH_OFF: u32 = 0;
pub const wxCMD_SWITCH_ON: u32 = 0 + 1;
//  ENUM: wxCmdLineSplitType
pub const wxCMD_LINE_SPLIT_DOS: u32 = 0;
pub const wxCMD_LINE_SPLIT_UNIX: u32 = 0 + 1;

pub const wxBU_LEFT: u32 = 0x0040;
pub const wxBU_TOP: u32 = 0x0080;
pub const wxBU_RIGHT: u32 = 0x0100;
pub const wxBU_BOTTOM: u32 = 0x0200;
pub const wxBU_ALIGN_MASK: u32 = ( wxBU_LEFT | wxBU_TOP | wxBU_RIGHT | wxBU_BOTTOM );
pub const wxBU_EXACTFIT: u32 = 0x0001;
pub const wxBU_NOTEXT: u32 = 0x0002;
pub const wxBU_AUTODRAW: u32 = 0x0004;

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
pub const wxTAR_USTAR: u32 = 0;
pub const wxTAR_PAX: u32 = 0 + 1;

pub const wxHELP_NETSCAPE: u32 = 1;
//  ENUM: wxHelpSearchMode
pub const wxHELP_SEARCH_INDEX: u32 = 0;
pub const wxHELP_SEARCH_ALL: u32 = 0 + 1;

//  ENUM: @3

//  ENUM: @0
pub const WX_ANY_VALUE_BUFFER_SIZE: u32 = 16;

pub const wxCHB_DEFAULT: u32 = wxBK_DEFAULT;
pub const wxCHB_TOP: u32 = wxBK_TOP;
pub const wxCHB_BOTTOM: u32 = wxBK_BOTTOM;
pub const wxCHB_LEFT: u32 = wxBK_LEFT;
pub const wxCHB_RIGHT: u32 = wxBK_RIGHT;
pub const wxCHB_ALIGN_MASK: u32 = wxBK_ALIGN_MASK;

pub const wxDD_CHANGE_DIR: u32 = 0x0100;
pub const wxDD_DIR_MUST_EXIST: u32 = 0x0200;
pub const wxDD_MULTIPLE: u32 = 0x0400;
pub const wxDD_SHOW_HIDDEN: u32 = 0x0001;
pub const wxDD_NEW_DIR_BUTTON: u32 = 0;
pub const wxDD_DEFAULT_STYLE: u32 = (wxDEFAULT_DIALOG_STYLE|wxRESIZE_BORDER);

pub const wxPB_USE_TEXTCTRL: u32 = 0x0002;
pub const wxPB_SMALL: u32 = 0x8000;

//  ENUM: TZ
pub const Local: u32 = 0;
pub const GMT_12: u32 = 0 + 1;
pub const GMT_11: u32 = 0 + 2;
pub const GMT_10: u32 = 0 + 3;
pub const GMT_9: u32 = 0 + 4;
pub const GMT_8: u32 = 0 + 5;
pub const GMT_7: u32 = 0 + 6;
pub const GMT_6: u32 = 0 + 7;
pub const GMT_5: u32 = 0 + 8;
pub const GMT_4: u32 = 0 + 9;
pub const GMT_3: u32 = 0 + 10;
pub const GMT_2: u32 = 0 + 11;
pub const GMT_1: u32 = 0 + 12;
pub const GMT0: u32 = 0 + 13;
pub const GMT1: u32 = 0 + 14;
pub const GMT2: u32 = 0 + 15;
pub const GMT3: u32 = 0 + 16;
pub const GMT4: u32 = 0 + 17;
pub const GMT5: u32 = 0 + 18;
pub const GMT6: u32 = 0 + 19;
pub const GMT7: u32 = 0 + 20;
pub const GMT8: u32 = 0 + 21;
pub const GMT9: u32 = 0 + 22;
pub const GMT10: u32 = 0 + 23;
pub const GMT11: u32 = 0 + 24;
pub const GMT12: u32 = 0 + 25;
pub const GMT13: u32 = 0 + 26;
pub const WET: u32 = GMT0;
pub const WEST: u32 = GMT1;
pub const CET: u32 = GMT1;
pub const CEST: u32 = GMT2;
pub const EET: u32 = GMT2;
pub const EEST: u32 = GMT3;
pub const MSK: u32 = GMT3;
pub const MSD: u32 = GMT4;
pub const AST: u32 = GMT_4;
pub const ADT: u32 = GMT_3;
pub const EST: u32 = GMT_5;
pub const EDT: u32 = GMT_4;
pub const CST: u32 = GMT_6;
pub const CDT: u32 = GMT_5;
pub const MST: u32 = GMT_7;
pub const MDT: u32 = GMT_6;
pub const PST: u32 = GMT_8;
pub const PDT: u32 = GMT_7;
pub const HST: u32 = GMT_10;
pub const AKST: u32 = GMT_9;
pub const AKDT: u32 = GMT_8;
pub const A_WST: u32 = GMT8;
pub const A_CST: u32 = GMT13 + 1;
pub const A_EST: u32 = GMT10;
pub const A_ESST: u32 = GMT11;
pub const NZST: u32 = GMT12;
pub const NZDT: u32 = GMT13;
pub const UTC: u32 = GMT0;
//  ENUM: Calendar
pub const Gregorian: u32 = 0;
pub const Julian: u32 = 0 + 1;
//  ENUM: Country
pub const Country_Unknown: u32 = 0;
pub const Country_Default: u32 = 0 + 1;
pub const Country_WesternEurope_Start: u32 = 0 + 2;
pub const Country_EEC: u32 = Country_WesternEurope_Start;
pub const France: u32 = Country_WesternEurope_Start + 1;
pub const Germany: u32 = Country_WesternEurope_Start + 2;
pub const UK: u32 = Country_WesternEurope_Start + 3;
pub const Country_WesternEurope_End: u32 = UK;
pub const Russia: u32 = UK + 1;
pub const USA: u32 = UK + 2;
//  ENUM: Month
pub const Jan: u32 = 0;
pub const Feb: u32 = 0 + 1;
pub const Mar: u32 = 0 + 2;
pub const Apr: u32 = 0 + 3;
pub const May: u32 = 0 + 4;
pub const Jun: u32 = 0 + 5;
pub const Jul: u32 = 0 + 6;
pub const Aug: u32 = 0 + 7;
pub const Sep: u32 = 0 + 8;
pub const Oct: u32 = 0 + 9;
pub const Nov: u32 = 0 + 10;
pub const Dec: u32 = 0 + 11;
pub const Inv_Month: u32 = 0 + 12;
//  ENUM: WeekDay
pub const Sun: u32 = 0;
pub const Mon: u32 = 0 + 1;
pub const Tue: u32 = 0 + 2;
pub const Wed: u32 = 0 + 3;
pub const Thu: u32 = 0 + 4;
pub const Fri: u32 = 0 + 5;
pub const Sat: u32 = 0 + 6;
pub const Inv_WeekDay: u32 = 0 + 7;
//  ENUM: Year
//  SKIP: Inv_Year
//  ENUM: NameFlags
pub const Name_Full: u32 = 0x01;
pub const Name_Abbr: u32 = 0x02;
//  ENUM: WeekFlags
pub const Default_First: u32 = 0;
pub const Monday_First: u32 = 0 + 1;
pub const Sunday_First: u32 = 0 + 2;

//  ENUM: @28
pub const wxPAGE_ODD: u32 = 0;
pub const wxPAGE_EVEN: u32 = 0 + 1;
pub const wxPAGE_ALL: u32 = 0 + 2;

//  SKIP: wxTreeListEventHandler
//  ENUM: @52
pub const wxTL_SINGLE: u32 = 0x0000;
pub const wxTL_MULTIPLE: u32 = 0x0001;
pub const wxTL_CHECKBOX: u32 = 0x0002;
pub const wxTL_3STATE: u32 = 0x0004;
pub const wxTL_USER_3STATE: u32 = 0x0008;
pub const wxTL_NO_HEADER: u32 = 0x0010;
pub const wxTL_DEFAULT_STYLE: u32 = wxTL_SINGLE;
pub const wxTL_STYLE_MASK: u32 = wxTL_SINGLE |
                          wxTL_MULTIPLE |
                          wxTL_CHECKBOX |
                          wxTL_3STATE |
                          wxTL_USER_3STATE;

