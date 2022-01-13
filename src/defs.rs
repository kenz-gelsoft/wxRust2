#![allow(unused_parens)]
const wxNB_DEFAULT: i32 = wxBK_DEFAULT;
const wxNB_TOP: i32 = wxBK_TOP;
const wxNB_BOTTOM: i32 = wxBK_BOTTOM;
const wxNB_LEFT: i32 = wxBK_LEFT;
const wxNB_RIGHT: i32 = wxBK_RIGHT;
const wxNB_FIXEDWIDTH: i32 = 0x0100;
const wxNB_MULTILINE: i32 = 0x0200;
const wxNB_NOPAGETHEME: i32 = 0x0400;
//  ENUM: @37
const wxNB_HITTEST_NOWHERE: i32 = wxBK_HITTEST_NOWHERE;
const wxNB_HITTEST_ONICON: i32 = wxBK_HITTEST_ONICON;
const wxNB_HITTEST_ONLABEL: i32 = wxBK_HITTEST_ONLABEL;
const wxNB_HITTEST_ONITEM: i32 = wxBK_HITTEST_ONITEM;
const wxNB_HITTEST_ONPAGE: i32 = wxBK_HITTEST_ONPAGE;

const wxPG_ATTR_DEFAULT_VALUE: &str = "DefaultValue";
const wxPG_ATTR_MIN: &str = "Min";
const wxPG_ATTR_MAX: &str = "Max";
const wxPG_ATTR_UNITS: &str = "Units";
const wxPG_ATTR_HINT: &str = "Hint";
const wxPG_ATTR_AUTOCOMPLETE: &str = "AutoComplete";
const wxPG_BOOL_USE_CHECKBOX: &str = "UseCheckbox";
const wxPG_BOOL_USE_DOUBLE_CLICK_CYCLING: &str = "UseDClickCycling";
const wxPG_FLOAT_PRECISION: &str = "Precision";
const wxPG_STRING_PASSWORD: &str = "Password";
const wxPG_UINT_BASE: &str = "Base";
const wxPG_UINT_PREFIX: &str = "Prefix";
const wxPG_DIALOG_TITLE: &str = "DialogTitle";
const wxPG_FILE_WILDCARD: &str = "Wildcard";
const wxPG_FILE_SHOW_FULL_PATH: &str = "ShowFullPath";
const wxPG_FILE_SHOW_RELATIVE_PATH: &str = "ShowRelativePath";
const wxPG_FILE_INITIAL_PATH: &str = "InitialPath";
const wxPG_FILE_DIALOG_STYLE: &str = "DialogStyle";
const wxPG_ARRAY_DELIMITER: &str = "Delimiter";
const wxPG_DATE_FORMAT: &str = "DateFormat";
const wxPG_DATE_PICKER_STYLE: &str = "PickerStyle";
const wxPG_ATTR_SPINCTRL_STEP: &str = "Step";
const wxPG_ATTR_SPINCTRL_WRAP: &str = "Wrap";
const wxPG_ATTR_SPINCTRL_MOTION: &str = "MotionSpin";
const wxPG_ATTR_MULTICHOICE_USERSTRINGMODE: &str = "UserStringMode";
const wxPG_COLOUR_ALLOW_CUSTOM: &str = "AllowCustom";
const wxPG_COLOUR_HAS_ALPHA: &str = "HasAlpha";
const wxPG_PROP_MAX: i32 = wxPG_PROP_AUTO_UNSPECIFIED;
const wxPG_PROP_PARENTAL_FLAGS: i32 =     ((wxPGPropertyFlags)(wxPG_PROP_AGGREGATE | \
                         wxPG_PROP_CATEGORY | \
                         wxPG_PROP_MISC_PARENT));
const wxPG_STRING_STORED_FLAGS: i32 =     (wxPG_PROP_DISABLED|wxPG_PROP_HIDDEN|wxPG_PROP_NOEDITOR|wxPG_PROP_COLLAPSED);
//  SKIP: wxNullProperty
//  SKIP: wxPGChoicesEmptyData
//  ENUM: wxPGPropertyFlags
const wxPG_PROP_MODIFIED: i32 = 0x0001;
const wxPG_PROP_DISABLED: i32 = 0x0002;
const wxPG_PROP_HIDDEN: i32 = 0x0004;
const wxPG_PROP_CUSTOMIMAGE: i32 = 0x0008;
const wxPG_PROP_NOEDITOR: i32 = 0x0010;
const wxPG_PROP_COLLAPSED: i32 = 0x0020;
const wxPG_PROP_INVALID_VALUE: i32 = 0x0040;
const wxPG_PROP_WAS_MODIFIED: i32 = 0x0200;
const wxPG_PROP_AGGREGATE: i32 = 0x0400;
const wxPG_PROP_CHILDREN_ARE_COPIES: i32 = 0x0800;
const wxPG_PROP_PROPERTY: i32 = 0x1000;
const wxPG_PROP_CATEGORY: i32 = 0x2000;
const wxPG_PROP_MISC_PARENT: i32 = 0x4000;
const wxPG_PROP_READONLY: i32 = 0x8000;
const wxPG_PROP_COMPOSED_VALUE: i32 = 0x00010000;
const wxPG_PROP_USES_COMMON_VALUE: i32 = 0x00020000;
const wxPG_PROP_AUTO_UNSPECIFIED: i32 = 0x00040000;
const wxPG_PROP_CLASS_SPECIFIC_1: i32 = 0x00080000;
const wxPG_PROP_CLASS_SPECIFIC_2: i32 = 0x00100000;
const wxPG_PROP_BEING_DELETED: i32 = 0x00200000


wxPG_PROP_CLASS_SPECIFIC_3          = 0x00400000;

const wxHW_SCROLLBAR_NEVER: i32 = 0x0002;
const wxHW_SCROLLBAR_AUTO: i32 = 0x0004;
const wxHW_NO_SELECTION: i32 = 0x0008;
const wxHW_DEFAULT_STYLE: i32 = wxHW_SCROLLBAR_AUTO;
//  ENUM: wxHtmlOpeningStatus
const wxHTML_OPEN: i32 = 0;
const wxHTML_BLOCK: i32 = 0 + 1;
const wxHTML_REDIRECT: i32 = 0 + 2;

const wxFRAME_NO_TASKBAR: i32 = 0x0002;
const wxFRAME_TOOL_WINDOW: i32 = 0x0004;
const wxFRAME_FLOAT_ON_PARENT: i32 = 0x0008;

//  ENUM: @16
const Event_Skip: i32 = -1;
const Event_Ignore: i32 = 0;
const Event_Processed: i32 = 1;

//  ENUM: @4
const wxCAL_SUNDAY_FIRST: i32 = 0x0080;
const wxCAL_MONDAY_FIRST: i32 = 0x0001;
const wxCAL_SHOW_HOLIDAYS: i32 = 0x0002;
const wxCAL_NO_YEAR_CHANGE: i32 = 0x0004;
const wxCAL_NO_MONTH_CHANGE: i32 = 0x000c;
const wxCAL_SEQUENTIAL_MONTH_SELECTION: i32 = 0x0010;
const wxCAL_SHOW_SURROUNDING_WEEKS: i32 = 0x0020;
const wxCAL_SHOW_WEEK_NUMBERS: i32 = 0x0040;
//  ENUM: wxCalendarDateBorder
const wxCAL_BORDER_NONE: i32 = 0;
const wxCAL_BORDER_SQUARE: i32 = 0 + 1;
const wxCAL_BORDER_ROUND: i32 = 0 + 2;
//  ENUM: wxCalendarHitTestResult
const wxCAL_HITTEST_NOWHERE: i32 = 0;
const wxCAL_HITTEST_HEADER: i32 = 0 + 1;
const wxCAL_HITTEST_DAY: i32 = 0 + 2;
const wxCAL_HITTEST_INCMONTH: i32 = 0 + 3;
const wxCAL_HITTEST_DECMONTH: i32 = 0 + 4;
const wxCAL_HITTEST_SURROUNDING_WEEK: i32 = 0 + 5;
const wxCAL_HITTEST_WEEK: i32 = 0 + 6;

//  ENUM: OpenMode
const read: i32 = 0;
const write: i32 = 0 + 1;
const read_write: i32 = 0 + 2;
const write_append: i32 = 0 + 3;
const write_excl: i32 = 0 + 4;
//  ENUM: @17
const fd_invalid: i32 = -1;
const fd_stdin: i32 = -1 + 1;
const fd_stdout: i32 = -1 + 2;
const fd_stderr: i32 = -1 + 3;

//  ENUM: wxStreamError
const wxSTREAM_NO_ERROR: i32 = 0;
const wxSTREAM_EOF: i32 = 0 + 1;
const wxSTREAM_WRITE_ERROR: i32 = 0 + 2;
const wxSTREAM_READ_ERROR: i32 = 0 + 3;
//  ENUM: wxStreamProtocolType
const wxSTREAM_PROTOCOL: i32 = 0;
const wxSTREAM_MIMETYPE: i32 = 0 + 1;
const wxSTREAM_ENCODING: i32 = 0 + 2;
const wxSTREAM_FILEEXT: i32 = 0 + 3;

//  ENUM: wxHtmlSelectionState
const wxHTML_SEL_OUT: i32 = 0;
const wxHTML_SEL_IN: i32 = 0 + 1;
const wxHTML_SEL_CHANGING: i32 = 0 + 2;
//  ENUM: @27
const wxHTML_FIND_EXACT: i32 = 1;
const wxHTML_FIND_NEAREST_BEFORE: i32 = 2;
const wxHTML_FIND_NEAREST_AFTER: i32 = 4;
//  ENUM: wxHtmlScriptMode
const wxHTML_SCRIPT_NORMAL: i32 = 0;
const wxHTML_SCRIPT_SUB: i32 = 0 + 1;
const wxHTML_SCRIPT_SUP: i32 = 0 + 2;

//  ENUM: wxProtocolError
const wxPROTO_NOERR: i32 = 0;
const wxPROTO_NETERR: i32 = 0 + 1;
const wxPROTO_PROTERR: i32 = 0 + 2;
const wxPROTO_CONNERR: i32 = 0 + 3;
const wxPROTO_INVVAL: i32 = 0 + 4;
const wxPROTO_NOHNDLR: i32 = 0 + 5;
const wxPROTO_NOFILE: i32 = 0 + 6;
const wxPROTO_ABRT: i32 = 0 + 7;
const wxPROTO_RCNCT: i32 = 0 + 8;
const wxPROTO_STREAMING: i32 = 0 + 9;

// NODEF: wxDECLARE_APP
// NODEF: wxIMPLEMENT_APP
//  SKIP: wxDISABLE_DEBUG_SUPPORT

//  ENUM: wxLayoutOrientation
const wxLAYOUT_HORIZONTAL: i32 = 0;
const wxLAYOUT_VERTICAL: i32 = 0 + 1;
//  ENUM: wxLayoutAlignment
const wxLAYOUT_NONE: i32 = 0;
const wxLAYOUT_TOP: i32 = 0 + 1;
const wxLAYOUT_LEFT: i32 = 0 + 2;
const wxLAYOUT_RIGHT: i32 = 0 + 3;
const wxLAYOUT_BOTTOM: i32 = 0 + 4;

//  SKIP: wxPG_LABEL
const wxPG_LABEL_STRING: &str = "@!";
const wxPG_COLOUR_BLACK: i32 = (*wxBLACK);
//  SKIP: wxPG_COLOUR
//  SKIP: wxPG_DEFAULT_IMAGE_SIZE
//  ENUM: wxPG_PROPERTYVALUES_FLAGS
const wxPG_DONT_RECURSE: i32 = 0x00000000;
const wxPG_KEEP_STRUCTURE: i32 = 0x00000010;
const wxPG_RECURSE: i32 = 0x00000020;
const wxPG_INC_ATTRIBUTES: i32 = 0x00000040;
const wxPG_RECURSE_STARTS: i32 = 0x00000080;
const wxPG_FORCE: i32 = 0x00000100;
const wxPG_SORT_TOP_LEVEL_ONLY: i32 = 0x00000200;

const wxSTB_SIZEGRIP: i32 = 0x0010;
const wxSTB_SHOW_TIPS: i32 = 0x0020;
const wxSTB_ELLIPSIZE_START: i32 = 0x0040;
const wxSTB_ELLIPSIZE_MIDDLE: i32 = 0x0080;
const wxSTB_ELLIPSIZE_END: i32 = 0x0100;
const wxSTB_DEFAULT_STYLE: i32 = (wxSTB_SIZEGRIP|wxSTB_ELLIPSIZE_END|wxSTB_SHOW_TIPS|wxFULL_REPAINT_ON_RESIZE);
const wxSB_NORMAL: i32 = 0x0000;
const wxSB_FLAT: i32 = 0x0001;
const wxSB_RAISED: i32 = 0x0002;
const wxSB_SUNKEN: i32 = 0x0003;

//  ENUM: wxSVGShapeRenderingMode
const wxSVG_SHAPE_RENDERING_AUTO: i32 = 0;
const wxSVG_SHAPE_RENDERING_OPTIMIZE_SPEED: i32 = 0 + 1;
const wxSVG_SHAPE_RENDERING_CRISP_EDGES: i32 = 0 + 2;
const wxSVG_SHAPE_RENDERING_GEOMETRIC_PRECISION: i32 = 0 + 3;
const wxSVG_SHAPE_RENDERING_OPTIMISE_SPEED: i32 = wxSVG_SHAPE_RENDERING_OPTIMIZE_SPEED;

//  ENUM: wxScrollbarVisibility
const wxSHOW_SB_NEVER: i32 = -1;
const wxSHOW_SB_DEFAULT: i32 = -1 + 1;
const wxSHOW_SB_ALWAYS: i32 = -1 + 2;

const wxSW_NOBORDER: i32 = 0x0000;
const wxSW_BORDER: i32 = 0x0020;
const wxSW_3DSASH: i32 = 0x0040;
const wxSW_3DBORDER: i32 = 0x0080;
const wxSW_3D: i32 = (wxSW_3DSASH | wxSW_3DBORDER);
//  ENUM: wxSashEdgePosition
const wxSASH_TOP: i32 = 0;
const wxSASH_RIGHT: i32 = 0 + 1;
const wxSASH_BOTTOM: i32 = 0 + 2;
const wxSASH_LEFT: i32 = 0 + 3;
const wxSASH_NONE: i32 = 100;
//  ENUM: wxSashDragStatus
const wxSASH_STATUS_OK: i32 = 0;
const wxSASH_STATUS_OUT_OF_RANGE: i32 = 0 + 1;

const wxBUFFER_VIRTUAL_AREA: i32 = 0x01;
const wxBUFFER_CLIENT_AREA: i32 = 0x02;
const wxBUFFER_USES_SHARED_BUFFER: i32 = 0x04;

//  ENUM: @10
const wxCONFIG_USE_LOCAL_FILE: i32 = 1;
const wxCONFIG_USE_GLOBAL_FILE: i32 = 2;
const wxCONFIG_USE_RELATIVE_PATH: i32 = 4;
const wxCONFIG_USE_NO_ESCAPE_CHARACTERS: i32 = 8;
const wxCONFIG_USE_SUBDIR: i32 = 16;

//  ENUM: PromptMode
const Prompt_Never: i32 = 0;
const Prompt_Once: i32 = 0 + 1;
const Prompt_Always: i32 = 0 + 2;

//  ENUM: wxBase64DecodeMode
const wxBase64DecodeMode_Strict: i32 = 0;
const wxBase64DecodeMode_SkipWS: i32 = 0 + 1;
const wxBase64DecodeMode_Relaxed: i32 = 0 + 2;

//  ENUM: wxPrintBin
const wxPRINTBIN_DEFAULT: i32 = 0;
const wxPRINTBIN_ONLYONE: i32 = 0 + 1;
const wxPRINTBIN_LOWER: i32 = 0 + 2;
const wxPRINTBIN_MIDDLE: i32 = 0 + 3;
const wxPRINTBIN_MANUAL: i32 = 0 + 4;
const wxPRINTBIN_ENVELOPE: i32 = 0 + 5;
const wxPRINTBIN_ENVMANUAL: i32 = 0 + 6;
const wxPRINTBIN_AUTO: i32 = 0 + 7;
const wxPRINTBIN_TRACTOR: i32 = 0 + 8;
const wxPRINTBIN_SMALLFMT: i32 = 0 + 9;
const wxPRINTBIN_LARGEFMT: i32 = 0 + 10;
const wxPRINTBIN_LARGECAPACITY: i32 = 0 + 11;
const wxPRINTBIN_CASSETTE: i32 = 0 + 12;
const wxPRINTBIN_FORMSOURCE: i32 = 0 + 13;
const wxPRINTBIN_USER: i32 = 0 + 14;

// NODEF: WXTRACE
// NODEF: WXTRACELEVEL

const wxSP_NOBORDER: i32 = 0x0000;
const wxSP_THIN_SASH: i32 = 0x0000;
const wxSP_NOSASH: i32 = 0x0010;
const wxSP_PERMIT_UNSPLIT: i32 = 0x0040;
const wxSP_LIVE_UPDATE: i32 = 0x0080;
const wxSP_3DSASH: i32 = 0x0100;
const wxSP_3DBORDER: i32 = 0x0200;
const wxSP_NO_XP_THEME: i32 = 0x0400;
const wxSP_BORDER: i32 = wxSP_3DBORDER;
const wxSP_3D: i32 = (wxSP_3DBORDER | wxSP_3DSASH);
//  ENUM: wxSplitMode
const wxSPLIT_HORIZONTAL: i32 = 1;
const wxSPLIT_VERTICAL: i32 = 1 + 1;
//  ENUM: @47
const wxSPLIT_DRAG_NONE: i32 = 0;
const wxSPLIT_DRAG_DRAGGING: i32 = 0 + 1;
const wxSPLIT_DRAG_LEFT_DOWN: i32 = 0 + 2;

//  ENUM: wxFSWFlags
const wxFSW_EVENT_CREATE: i32 = 0x01;
const wxFSW_EVENT_DELETE: i32 = 0x02;
const wxFSW_EVENT_RENAME: i32 = 0x04;
const wxFSW_EVENT_MODIFY: i32 = 0x08;
const wxFSW_EVENT_ACCESS: i32 = 0x10;
const wxFSW_EVENT_ATTRIB: i32 = 0x20;
const wxFSW_EVENT_UNMOUNT: i32 = 0x2000;
const wxFSW_EVENT_WARNING: i32 = 0x40;
const wxFSW_EVENT_ERROR: i32 = 0x80;
const wxFSW_EVENT_ALL: i32 = wxFSW_EVENT_CREATE | wxFSW_EVENT_DELETE |
                         wxFSW_EVENT_RENAME | wxFSW_EVENT_MODIFY |
                         wxFSW_EVENT_ACCESS | wxFSW_EVENT_ATTRIB |
                         wxFSW_EVENT_WARNING | wxFSW_EVENT_ERROR;
//  ENUM: wxFSWWarningType
const wxFSW_WARNING_NONE: i32 = 0;
const wxFSW_WARNING_GENERAL: i32 = 0 + 1;
const wxFSW_WARNING_OVERFLOW: i32 = 0 + 2;

//  ENUM: wxAnimationDisposal
const wxANIM_UNSPECIFIED: i32 = -1;
const wxANIM_DONOTREMOVE: i32 = 0;
const wxANIM_TOBACKGROUND: i32 = 1;
const wxANIM_TOPREVIOUS: i32 = 2;

//  ENUM: wxZlibCompressionLevels
const wxZ_DEFAULT_COMPRESSION: i32 = -1;
const wxZ_NO_COMPRESSION: i32 = 0;
const wxZ_BEST_SPEED: i32 = 1;
const wxZ_BEST_COMPRESSION: i32 = 9;
//  ENUM: wxZLibFlags
const wxZLIB_NO_HEADER: i32 = 0;
const wxZLIB_ZLIB: i32 = 1;
const wxZLIB_GZIP: i32 = 2;
const wxZLIB_AUTO: i32 = 3;

// NODEF: wxDYNLIB_FUNCTION
//  ENUM: wxDynamicLibraryCategory
const wxDL_LIBRARY: i32 = 0;
const wxDL_MODULE: i32 = 0 + 1;
//  ENUM: wxPluginCategory
const wxDL_PLUGIN_GUI: i32 = 0;
const wxDL_PLUGIN_BASE: i32 = 0 + 1;

//  ENUM: wxLayoutDirection
const wxLayout_Default: i32 = 0;
const wxLayout_LeftToRight: i32 = 0 + 1;
const wxLayout_RightToLeft: i32 = 0 + 2;
//  ENUM: wxLocaleCategory
const wxLOCALE_CAT_NUMBER: i32 = 0;
const wxLOCALE_CAT_DATE: i32 = 0 + 1;
const wxLOCALE_CAT_MONEY: i32 = 0 + 2;
const wxLOCALE_CAT_DEFAULT: i32 = 0 + 3;
//  ENUM: wxLocaleInfo
const wxLOCALE_THOUSANDS_SEP: i32 = 0;
const wxLOCALE_DECIMAL_POINT: i32 = 0 + 1;
const wxLOCALE_SHORT_DATE_FMT: i32 = 0 + 2;
const wxLOCALE_LONG_DATE_FMT: i32 = 0 + 3;
const wxLOCALE_DATE_TIME_FMT: i32 = 0 + 4;
const wxLOCALE_TIME_FMT: i32 = 0 + 5;
//  ENUM: wxLocaleInitFlags
const wxLOCALE_DONT_LOAD_DEFAULT: i32 = 0x0000;
const wxLOCALE_LOAD_DEFAULT: i32 = 0x0001;

//  ENUM: @1
const typeCaption: i32 = 0;
const typeGripper: i32 = 0 + 1;
const typeDock: i32 = 0 + 2;
const typeDockSizer: i32 = 0 + 3;
const typePane: i32 = 0 + 4;
const typePaneSizer: i32 = 0 + 5;
const typeBackground: i32 = 0 + 6;
const typePaneBorder: i32 = 0 + 7;
const typePaneButton: i32 = 0 + 8;

const wxDIALOG_NO_PARENT: i32 = 0x00000020;
const wxDEFAULT_DIALOG_STYLE: i32 = (wxCAPTION | wxSYSTEM_MENU | wxCLOSE_BOX);
const wxDIALOG_ADAPTATION_NONE: i32 = 0;
const wxDIALOG_ADAPTATION_STANDARD_SIZER: i32 = 1;
const wxDIALOG_ADAPTATION_ANY_SIZER: i32 = 2;
const wxDIALOG_ADAPTATION_LOOSE_BUTTONS: i32 = 3;
//  ENUM: wxDialogLayoutAdaptationMode
const wxDIALOG_ADAPTATION_MODE_DEFAULT: i32 = 0;
const wxDIALOG_ADAPTATION_MODE_ENABLED: i32 = 1;
const wxDIALOG_ADAPTATION_MODE_DISABLED: i32 = 2;

//  ENUM: HTMLCursor
const HTMLCursor_Default: i32 = 0;
const HTMLCursor_Link: i32 = 0 + 1;
const HTMLCursor_Text: i32 = 0 + 2;

//  ENUM: Reason
const Reason_Mouse: i32 = 0;
const Reason_Unknown: i32 = 0 + 1;

const wxAC_NO_AUTORESIZE: i32 = (0x0010);
const wxAC_DEFAULT_STYLE: i32 = (wxBORDER_NONE);
//  ENUM: wxAnimationType
const wxANIMATION_TYPE_INVALID: i32 = 0;
const wxANIMATION_TYPE_GIF: i32 = 0 + 1;
const wxANIMATION_TYPE_ANI: i32 = 0 + 2;
const wxANIMATION_TYPE_ANY: i32 = 0 + 3;

const wxSPLASH_CENTRE_ON_PARENT: i32 = 0x01;
const wxSPLASH_CENTRE_ON_SCREEN: i32 = 0x02;
const wxSPLASH_NO_CENTRE: i32 = 0x00;
const wxSPLASH_TIMEOUT: i32 = 0x04;
const wxSPLASH_NO_TIMEOUT: i32 = 0x00;

const wxPREVIEW_PRINT: i32 = 1;
const wxPREVIEW_PREVIOUS: i32 = 2;
const wxPREVIEW_NEXT: i32 = 4;
const wxPREVIEW_ZOOM: i32 = 8;
const wxPREVIEW_FIRST: i32 = 16;
const wxPREVIEW_LAST: i32 = 32;
const wxPREVIEW_GOTO: i32 = 64;
const wxPREVIEW_DEFAULT: i32 =                             (wxPREVIEW_PREVIOUS|wxPREVIEW_NEXT|wxPREVIEW_ZOOM\
                            |wxPREVIEW_FIRST|wxPREVIEW_GOTO|wxPREVIEW_LAST);
const wxID_PREVIEW_CLOSE: i32 = 1;
const wxID_PREVIEW_NEXT: i32 = 2;
const wxID_PREVIEW_PREVIOUS: i32 = 3;
const wxID_PREVIEW_PRINT: i32 = 4;
const wxID_PREVIEW_ZOOM: i32 = 5;
const wxID_PREVIEW_FIRST: i32 = 6;
const wxID_PREVIEW_LAST: i32 = 7;
const wxID_PREVIEW_GOTO: i32 = 8;
const wxID_PREVIEW_ZOOM_IN: i32 = 9;
const wxID_PREVIEW_ZOOM_OUT: i32 = 10;
//  ENUM: wxPrinterError
const wxPRINTER_NO_ERROR: i32 = 0;
const wxPRINTER_CANCELLED: i32 = 0 + 1;
const wxPRINTER_ERROR: i32 = 0 + 2;
//  ENUM: wxPreviewFrameModalityKind
const wxPreviewFrame_AppModal: i32 = 0;
const wxPreviewFrame_WindowModal: i32 = 0 + 1;
const wxPreviewFrame_NonModal: i32 = 0 + 2;

// NODEF: wxGetVariantCast

const wxWIZARD_EX_HELPBUTTON: i32 = 0x00000010;
const wxWIZARD_VALIGN_TOP: i32 = 0x01;
const wxWIZARD_VALIGN_CENTRE: i32 = 0x02;
const wxWIZARD_VALIGN_BOTTOM: i32 = 0x04;
const wxWIZARD_HALIGN_LEFT: i32 = 0x08;
const wxWIZARD_HALIGN_CENTRE: i32 = 0x10;
const wxWIZARD_HALIGN_RIGHT: i32 = 0x20;
const wxWIZARD_TILE: i32 = 0x40;

//  ENUM: wxTextFileType
const wxTextFileType_None: i32 = 0;
const wxTextFileType_Unix: i32 = 0 + 1;
const wxTextFileType_Dos: i32 = 0 + 2;
const wxTextFileType_Mac: i32 = 0 + 3;
const wxTextFileType_Os2: i32 = 0 + 4;

const wxFD_DEFAULT_STYLE: i32 = wxFD_OPEN;
//  ENUM: @19
const wxFD_OPEN: i32 = 0x0001;
const wxFD_SAVE: i32 = 0x0002;
const wxFD_OVERWRITE_PROMPT: i32 = 0x0004;
const wxFD_NO_FOLLOW: i32 = 0x0008;
const wxFD_FILE_MUST_EXIST: i32 = 0x0010;
const wxFD_CHANGE_DIR: i32 = 0x0080;
const wxFD_PREVIEW: i32 = 0x0100;
const wxFD_MULTIPLE: i32 = 0x0200;
const wxFD_SHOW_HIDDEN: i32 = 0x0400;

//  ENUM: Direction
const Get: i32 = 0x01;
const Set: i32 = 0x02;
const Both: i32 = 0x03;

const wxHF_TOOLBAR: i32 = 0x0001;
const wxHF_CONTENTS: i32 = 0x0002;
const wxHF_INDEX: i32 = 0x0004;
const wxHF_SEARCH: i32 = 0x0008;
const wxHF_BOOKMARKS: i32 = 0x0010;
const wxHF_OPEN_FILES: i32 = 0x0020;
const wxHF_PRINT: i32 = 0x0040;
const wxHF_FLAT_TOOLBAR: i32 = 0x0080;
const wxHF_MERGE_BOOKS: i32 = 0x0100;
const wxHF_ICONS_BOOK: i32 = 0x0200;
const wxHF_ICONS_BOOK_CHAPTER: i32 = 0x0400;
const wxHF_ICONS_FOLDER: i32 = 0x0000;
const wxHF_DEFAULT_STYLE: i32 =                                      (wxHF_TOOLBAR | wxHF_CONTENTS | \
                                     wxHF_INDEX | wxHF_SEARCH | \
                                     wxHF_BOOKMARKS | wxHF_PRINT);

//  ENUM: Context
const Context_Current: i32 = 0;
const Context_Exception: i32 = 0 + 1;

const wxST_NO_AUTORESIZE: i32 = 0x0001;
const wxST_ELLIPSIZE_START: i32 = 0x0004;
const wxST_ELLIPSIZE_MIDDLE: i32 = 0x0008;
const wxST_ELLIPSIZE_END: i32 = 0x0010;

//  ENUM: wxSocketError
const wxSOCKET_NOERROR: i32 = 0;
const wxSOCKET_INVOP: i32 = 0 + 1;
const wxSOCKET_IOERR: i32 = 0 + 2;
const wxSOCKET_INVADDR: i32 = 0 + 3;
const wxSOCKET_INVSOCK: i32 = 0 + 4;
const wxSOCKET_NOHOST: i32 = 0 + 5;
const wxSOCKET_INVPORT: i32 = 0 + 6;
const wxSOCKET_WOULDBLOCK: i32 = 0 + 7;
const wxSOCKET_TIMEDOUT: i32 = 0 + 8;
const wxSOCKET_MEMERR: i32 = 0 + 9;
//  ENUM: wxSocketEventFlags
const wxSOCKET_INPUT: i32 = 0;
const wxSOCKET_OUTPUT: i32 = 0 + 1;
const wxSOCKET_CONNECTION: i32 = 0 + 2;
const wxSOCKET_LOST: i32 = 0 + 3;
//  ENUM: @46
const wxSOCKET_NONE: i32 = 0;
const wxSOCKET_NOWAIT: i32 = 1;
const wxSOCKET_WAITALL: i32 = 2;
const wxSOCKET_BLOCK: i32 = 4;
const wxSOCKET_REUSEADDR: i32 = 8;
const wxSOCKET_BROADCAST: i32 = 16;
const wxSOCKET_NOBIND: i32 = 32;
const wxSOCKET_NOWAIT_READ: i32 = 64;
const wxSOCKET_WAITALL_READ: i32 = 128;
const wxSOCKET_NOWAIT_WRITE: i32 = 256;
const wxSOCKET_WAITALL_WRITE: i32 = 512;

//  ENUM: wxGridSelectionModes
const wxGridSelectCells: i32 = 0;
const wxGridSelectRows: i32 = 0 + 1;
const wxGridSelectColumns: i32 = 0 + 2;
const wxGridSelectRowsOrColumns: i32 = 0 + 3;
const wxGridSelectNone: i32 = 0 + 4;
//  ENUM: CellSpan
const CellSpan_Inside: i32 = -1;
const CellSpan_None: i32 = 0;
const CellSpan_Main: i32 = 0 + 1;
//  ENUM: TabBehaviour
const Tab_Stop: i32 = 0;
const Tab_Wrap: i32 = 0 + 1;
const Tab_Leave: i32 = 0 + 2;

const wxSTC_INVALID_POSITION: i32 = -1;
const wxSTC_START: i32 = 2000;
const wxSTC_OPTIONAL_START: i32 = 3000;
const wxSTC_LEXER_START: i32 = 4000;
const wxSTC_WS_INVISIBLE: i32 = 0;
const wxSTC_WS_VISIBLEALWAYS: i32 = 1;
const wxSTC_WS_VISIBLEAFTERINDENT: i32 = 2;
const wxSTC_WS_VISIBLEONLYININDENT: i32 = 3;
const wxSTC_TD_LONGARROW: i32 = 0;
const wxSTC_TD_STRIKEOUT: i32 = 1;
const wxSTC_EOL_CRLF: i32 = 0;
const wxSTC_EOL_CR: i32 = 1;
const wxSTC_EOL_LF: i32 = 2;
const wxSTC_CP_UTF8: i32 = 65001;
const wxSTC_IME_WINDOWED: i32 = 0;
const wxSTC_IME_INLINE: i32 = 1;
const wxSTC_MARKER_MAX: i32 = 31;
const wxSTC_MARK_CIRCLE: i32 = 0;
const wxSTC_MARK_ROUNDRECT: i32 = 1;
const wxSTC_MARK_ARROW: i32 = 2;
const wxSTC_MARK_SMALLRECT: i32 = 3;
const wxSTC_MARK_SHORTARROW: i32 = 4;
const wxSTC_MARK_EMPTY: i32 = 5;
const wxSTC_MARK_ARROWDOWN: i32 = 6;
const wxSTC_MARK_MINUS: i32 = 7;
const wxSTC_MARK_PLUS: i32 = 8;
const wxSTC_MARK_VLINE: i32 = 9;
const wxSTC_MARK_LCORNER: i32 = 10;
const wxSTC_MARK_TCORNER: i32 = 11;
const wxSTC_MARK_BOXPLUS: i32 = 12;
const wxSTC_MARK_BOXPLUSCONNECTED: i32 = 13;
const wxSTC_MARK_BOXMINUS: i32 = 14;
const wxSTC_MARK_BOXMINUSCONNECTED: i32 = 15;
const wxSTC_MARK_LCORNERCURVE: i32 = 16;
const wxSTC_MARK_TCORNERCURVE: i32 = 17;
const wxSTC_MARK_CIRCLEPLUS: i32 = 18;
const wxSTC_MARK_CIRCLEPLUSCONNECTED: i32 = 19;
const wxSTC_MARK_CIRCLEMINUS: i32 = 20;
const wxSTC_MARK_CIRCLEMINUSCONNECTED: i32 = 21;
const wxSTC_MARK_BACKGROUND: i32 = 22;
const wxSTC_MARK_DOTDOTDOT: i32 = 23;
const wxSTC_MARK_ARROWS: i32 = 24;
const wxSTC_MARK_PIXMAP: i32 = 25;
const wxSTC_MARK_FULLRECT: i32 = 26;
const wxSTC_MARK_LEFTRECT: i32 = 27;
const wxSTC_MARK_AVAILABLE: i32 = 28;
const wxSTC_MARK_UNDERLINE: i32 = 29;
const wxSTC_MARK_RGBAIMAGE: i32 = 30;
const wxSTC_MARK_BOOKMARK: i32 = 31;
const wxSTC_MARK_CHARACTER: i32 = 10000;
const wxSTC_MARKNUM_FOLDEREND: i32 = 25;
const wxSTC_MARKNUM_FOLDEROPENMID: i32 = 26;
const wxSTC_MARKNUM_FOLDERMIDTAIL: i32 = 27;
const wxSTC_MARKNUM_FOLDERTAIL: i32 = 28;
const wxSTC_MARKNUM_FOLDERSUB: i32 = 29;
const wxSTC_MARKNUM_FOLDER: i32 = 30;
const wxSTC_MARKNUM_FOLDEROPEN: i32 = 31;
const wxSTC_MASK_FOLDERS: i32 = 0xFE000000;
const wxSTC_MAX_MARGIN: i32 = 4;
const wxSTC_MARGIN_SYMBOL: i32 = 0;
const wxSTC_MARGIN_NUMBER: i32 = 1;
const wxSTC_MARGIN_BACK: i32 = 2;
const wxSTC_MARGIN_FORE: i32 = 3;
const wxSTC_MARGIN_TEXT: i32 = 4;
const wxSTC_MARGIN_RTEXT: i32 = 5;
const wxSTC_MARGIN_COLOUR: i32 = 6;
const wxSTC_STYLE_DEFAULT: i32 = 32;
const wxSTC_STYLE_LINENUMBER: i32 = 33;
const wxSTC_STYLE_BRACELIGHT: i32 = 34;
const wxSTC_STYLE_BRACEBAD: i32 = 35;
const wxSTC_STYLE_CONTROLCHAR: i32 = 36;
const wxSTC_STYLE_INDENTGUIDE: i32 = 37;
const wxSTC_STYLE_CALLTIP: i32 = 38;
const wxSTC_STYLE_FOLDDISPLAYTEXT: i32 = 39;
const wxSTC_STYLE_LASTPREDEFINED: i32 = 39;
const wxSTC_STYLE_MAX: i32 = 255;
const wxSTC_CHARSET_ANSI: i32 = 0;
const wxSTC_CHARSET_DEFAULT: i32 = 1;
const wxSTC_CHARSET_BALTIC: i32 = 186;
const wxSTC_CHARSET_CHINESEBIG5: i32 = 136;
const wxSTC_CHARSET_EASTEUROPE: i32 = 238;
const wxSTC_CHARSET_GB2312: i32 = 134;
const wxSTC_CHARSET_GREEK: i32 = 161;
const wxSTC_CHARSET_HANGUL: i32 = 129;
const wxSTC_CHARSET_MAC: i32 = 77;
const wxSTC_CHARSET_OEM: i32 = 255;
const wxSTC_CHARSET_RUSSIAN: i32 = 204;
const wxSTC_CHARSET_OEM866: i32 = 866;
const wxSTC_CHARSET_CYRILLIC: i32 = 1251;
const wxSTC_CHARSET_SHIFTJIS: i32 = 128;
const wxSTC_CHARSET_SYMBOL: i32 = 2;
const wxSTC_CHARSET_TURKISH: i32 = 162;
const wxSTC_CHARSET_JOHAB: i32 = 130;
const wxSTC_CHARSET_HEBREW: i32 = 177;
const wxSTC_CHARSET_ARABIC: i32 = 178;
const wxSTC_CHARSET_VIETNAMESE: i32 = 163;
const wxSTC_CHARSET_THAI: i32 = 222;
const wxSTC_CHARSET_8859_15: i32 = 1000;
const wxSTC_CASE_MIXED: i32 = 0;
const wxSTC_CASE_UPPER: i32 = 1;
const wxSTC_CASE_LOWER: i32 = 2;
const wxSTC_CASE_CAMEL: i32 = 3;
const wxSTC_FONT_SIZE_MULTIPLIER: i32 = 100;
const wxSTC_WEIGHT_NORMAL: i32 = 400;
const wxSTC_WEIGHT_SEMIBOLD: i32 = 600;
const wxSTC_WEIGHT_BOLD: i32 = 700;
const wxSTC_INDIC_PLAIN: i32 = 0;
const wxSTC_INDIC_SQUIGGLE: i32 = 1;
const wxSTC_INDIC_TT: i32 = 2;
const wxSTC_INDIC_DIAGONAL: i32 = 3;
const wxSTC_INDIC_STRIKE: i32 = 4;
const wxSTC_INDIC_HIDDEN: i32 = 5;
const wxSTC_INDIC_BOX: i32 = 6;
const wxSTC_INDIC_ROUNDBOX: i32 = 7;
const wxSTC_INDIC_STRAIGHTBOX: i32 = 8;
const wxSTC_INDIC_DASH: i32 = 9;
const wxSTC_INDIC_DOTS: i32 = 10;
const wxSTC_INDIC_SQUIGGLELOW: i32 = 11;
const wxSTC_INDIC_DOTBOX: i32 = 12;
const wxSTC_INDIC_SQUIGGLEPIXMAP: i32 = 13;
const wxSTC_INDIC_COMPOSITIONTHICK: i32 = 14;
const wxSTC_INDIC_COMPOSITIONTHIN: i32 = 15;
const wxSTC_INDIC_FULLBOX: i32 = 16;
const wxSTC_INDIC_TEXTFORE: i32 = 17;
const wxSTC_INDIC_POINT: i32 = 18;
const wxSTC_INDIC_POINTCHARACTER: i32 = 19;
const wxSTC_INDIC_IME: i32 = 32;
const wxSTC_INDIC_IME_MAX: i32 = 35;
const wxSTC_INDIC_MAX: i32 = 35;
const wxSTC_INDIC_CONTAINER: i32 = 8;
const wxSTC_INDICVALUEBIT: i32 = 0x1000000;
const wxSTC_INDICVALUEMASK: i32 = 0xFFFFFF;
const wxSTC_INDICFLAG_VALUEFORE: i32 = 1;
const wxSTC_IV_NONE: i32 = 0;
const wxSTC_IV_REAL: i32 = 1;
const wxSTC_IV_LOOKFORWARD: i32 = 2;
const wxSTC_IV_LOOKBOTH: i32 = 3;
const wxSTC_PRINT_NORMAL: i32 = 0;
const wxSTC_PRINT_INVERTLIGHT: i32 = 1;
const wxSTC_PRINT_BLACKONWHITE: i32 = 2;
const wxSTC_PRINT_COLOURONWHITE: i32 = 3;
const wxSTC_PRINT_COLOURONWHITEDEFAULTBG: i32 = 4;
const wxSTC_FIND_WHOLEWORD: i32 = 0x2;
const wxSTC_FIND_MATCHCASE: i32 = 0x4;
const wxSTC_FIND_WORDSTART: i32 = 0x00100000;
const wxSTC_FIND_REGEXP: i32 = 0x00200000;
const wxSTC_FIND_POSIX: i32 = 0x00400000;
const wxSTC_FOLDLEVELBASE: i32 = 0x400;
const wxSTC_FOLDLEVELWHITEFLAG: i32 = 0x1000;
const wxSTC_FOLDLEVELHEADERFLAG: i32 = 0x2000;
const wxSTC_FOLDLEVELNUMBERMASK: i32 = 0x0FFF;
const wxSTC_FOLDDISPLAYTEXT_HIDDEN: i32 = 0;
const wxSTC_FOLDDISPLAYTEXT_STANDARD: i32 = 1;
const wxSTC_FOLDDISPLAYTEXT_BOXED: i32 = 2;
const wxSTC_FOLDACTION_CONTRACT: i32 = 0;
const wxSTC_FOLDACTION_EXPAND: i32 = 1;
const wxSTC_FOLDACTION_TOGGLE: i32 = 2;
const wxSTC_AUTOMATICFOLD_SHOW: i32 = 0x0001;
const wxSTC_AUTOMATICFOLD_CLICK: i32 = 0x0002;
const wxSTC_AUTOMATICFOLD_CHANGE: i32 = 0x0004;
const wxSTC_FOLDFLAG_LINEBEFORE_EXPANDED: i32 = 0x0002;
const wxSTC_FOLDFLAG_LINEBEFORE_CONTRACTED: i32 = 0x0004;
const wxSTC_FOLDFLAG_LINEAFTER_EXPANDED: i32 = 0x0008;
const wxSTC_FOLDFLAG_LINEAFTER_CONTRACTED: i32 = 0x0010;
const wxSTC_FOLDFLAG_LEVELNUMBERS: i32 = 0x0040;
const wxSTC_FOLDFLAG_LINESTATE: i32 = 0x0080;
const wxSTC_TIME_FOREVER: i32 = 10000000;
const wxSTC_IDLESTYLING_NONE: i32 = 0;
const wxSTC_IDLESTYLING_TOVISIBLE: i32 = 1;
const wxSTC_IDLESTYLING_AFTERVISIBLE: i32 = 2;
const wxSTC_IDLESTYLING_ALL: i32 = 3;
const wxSTC_WRAP_NONE: i32 = 0;
const wxSTC_WRAP_WORD: i32 = 1;
const wxSTC_WRAP_CHAR: i32 = 2;
const wxSTC_WRAP_WHITESPACE: i32 = 3;
const wxSTC_WRAPVISUALFLAG_NONE: i32 = 0x0000;
const wxSTC_WRAPVISUALFLAG_END: i32 = 0x0001;
const wxSTC_WRAPVISUALFLAG_START: i32 = 0x0002;
const wxSTC_WRAPVISUALFLAG_MARGIN: i32 = 0x0004;
const wxSTC_WRAPVISUALFLAGLOC_DEFAULT: i32 = 0x0000;
const wxSTC_WRAPVISUALFLAGLOC_END_BY_TEXT: i32 = 0x0001;
const wxSTC_WRAPVISUALFLAGLOC_START_BY_TEXT: i32 = 0x0002;
const wxSTC_WRAPINDENT_FIXED: i32 = 0;
const wxSTC_WRAPINDENT_SAME: i32 = 1;
const wxSTC_WRAPINDENT_INDENT: i32 = 2;
const wxSTC_CACHE_NONE: i32 = 0;
const wxSTC_CACHE_CARET: i32 = 1;
const wxSTC_CACHE_PAGE: i32 = 2;
const wxSTC_CACHE_DOCUMENT: i32 = 3;
const wxSTC_PHASES_ONE: i32 = 0;
const wxSTC_PHASES_TWO: i32 = 1;
const wxSTC_PHASES_MULTIPLE: i32 = 2;
const wxSTC_EFF_QUALITY_MASK: i32 = 0xF;
const wxSTC_EFF_QUALITY_DEFAULT: i32 = 0;
const wxSTC_EFF_QUALITY_NON_ANTIALIASED: i32 = 1;
const wxSTC_EFF_QUALITY_ANTIALIASED: i32 = 2;
const wxSTC_EFF_QUALITY_LCD_OPTIMIZED: i32 = 3;
const wxSTC_MULTIPASTE_ONCE: i32 = 0;
const wxSTC_MULTIPASTE_EACH: i32 = 1;
const wxSTC_EDGE_NONE: i32 = 0;
const wxSTC_EDGE_LINE: i32 = 1;
const wxSTC_EDGE_BACKGROUND: i32 = 2;
const wxSTC_EDGE_MULTILINE: i32 = 3;
const wxSTC_POPUP_NEVER: i32 = 0;
const wxSTC_POPUP_ALL: i32 = 1;
const wxSTC_POPUP_TEXT: i32 = 2;
const wxSTC_STATUS_OK: i32 = 0;
const wxSTC_STATUS_FAILURE: i32 = 1;
const wxSTC_STATUS_BADALLOC: i32 = 2;
const wxSTC_STATUS_WARN_START: i32 = 1000;
const wxSTC_STATUS_WARN_REGEX: i32 = 1001;
const wxSTC_CURSORNORMAL: i32 = -1;
const wxSTC_CURSORARROW: i32 = 2;
const wxSTC_CURSORWAIT: i32 = 4;
const wxSTC_CURSORREVERSEARROW: i32 = 7;
const wxSTC_VISIBLE_SLOP: i32 = 0x01;
const wxSTC_VISIBLE_STRICT: i32 = 0x04;
const wxSTC_CARET_SLOP: i32 = 0x01;
const wxSTC_CARET_STRICT: i32 = 0x04;
const wxSTC_CARET_JUMPS: i32 = 0x10;
const wxSTC_CARET_EVEN: i32 = 0x08;
const wxSTC_SEL_STREAM: i32 = 0;
const wxSTC_SEL_RECTANGLE: i32 = 1;
const wxSTC_SEL_LINES: i32 = 2;
const wxSTC_SEL_THIN: i32 = 3;
const wxSTC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE: i32 = 0;
const wxSTC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE: i32 = 1;
const wxSTC_MULTIAUTOC_ONCE: i32 = 0;
const wxSTC_MULTIAUTOC_EACH: i32 = 1;
const wxSTC_ORDER_PRESORTED: i32 = 0;
const wxSTC_ORDER_PERFORMSORT: i32 = 1;
const wxSTC_ORDER_CUSTOM: i32 = 2;
const wxSTC_CARETSTICKY_OFF: i32 = 0;
const wxSTC_CARETSTICKY_ON: i32 = 1;
const wxSTC_CARETSTICKY_WHITESPACE: i32 = 2;
const wxSTC_ALPHA_TRANSPARENT: i32 = 0;
const wxSTC_ALPHA_OPAQUE: i32 = 255;
const wxSTC_ALPHA_NOALPHA: i32 = 256;
const wxSTC_CARETSTYLE_INVISIBLE: i32 = 0;
const wxSTC_CARETSTYLE_LINE: i32 = 1;
const wxSTC_CARETSTYLE_BLOCK: i32 = 2;
const wxSTC_MARGINOPTION_NONE: i32 = 0;
const wxSTC_MARGINOPTION_SUBLINESELECT: i32 = 1;
const wxSTC_ANNOTATION_HIDDEN: i32 = 0;
const wxSTC_ANNOTATION_STANDARD: i32 = 1;
const wxSTC_ANNOTATION_BOXED: i32 = 2;
const wxSTC_ANNOTATION_INDENTED: i32 = 3;
const wxSTC_UNDO_MAY_COALESCE: i32 = 1;
const wxSTC_VS_NONE: i32 = 0;
const wxSTC_VS_RECTANGULARSELECTION: i32 = 1;
const wxSTC_VS_USERACCESSIBLE: i32 = 2;
const wxSTC_VS_NOWRAPLINESTART: i32 = 4;
const wxSTC_TECHNOLOGY_DEFAULT: i32 = 0;
const wxSTC_TECHNOLOGY_DIRECTWRITE: i32 = 1;
const wxSTC_LINE_END_TYPE_DEFAULT: i32 = 0;
const wxSTC_LINE_END_TYPE_UNICODE: i32 = 1;
const wxSTC_KEYWORDSET_MAX: i32 = 8;
const wxSTC_TYPE_BOOLEAN: i32 = 0;
const wxSTC_TYPE_INTEGER: i32 = 1;
const wxSTC_TYPE_STRING: i32 = 2;
const wxSTC_MOD_INSERTTEXT: i32 = 0x1;
const wxSTC_MOD_DELETETEXT: i32 = 0x2;
const wxSTC_MOD_CHANGESTYLE: i32 = 0x4;
const wxSTC_MOD_CHANGEFOLD: i32 = 0x8;
const wxSTC_PERFORMED_USER: i32 = 0x10;
const wxSTC_PERFORMED_UNDO: i32 = 0x20;
const wxSTC_PERFORMED_REDO: i32 = 0x40;
const wxSTC_MULTISTEPUNDOREDO: i32 = 0x80;
const wxSTC_LASTSTEPINUNDOREDO: i32 = 0x100;
const wxSTC_MOD_CHANGEMARKER: i32 = 0x200;
const wxSTC_MOD_BEFOREINSERT: i32 = 0x400;
const wxSTC_MOD_BEFOREDELETE: i32 = 0x800;
const wxSTC_MULTILINEUNDOREDO: i32 = 0x1000;
const wxSTC_STARTACTION: i32 = 0x2000;
const wxSTC_MOD_CHANGEINDICATOR: i32 = 0x4000;
const wxSTC_MOD_CHANGELINESTATE: i32 = 0x8000;
const wxSTC_MOD_CHANGEMARGIN: i32 = 0x10000;
const wxSTC_MOD_CHANGEANNOTATION: i32 = 0x20000;
const wxSTC_MOD_CONTAINER: i32 = 0x40000;
const wxSTC_MOD_LEXERSTATE: i32 = 0x80000;
const wxSTC_MOD_INSERTCHECK: i32 = 0x100000;
const wxSTC_MOD_CHANGETABSTOPS: i32 = 0x200000;
const wxSTC_MODEVENTMASKALL: i32 = 0x3FFFFF;
const wxSTC_UPDATE_CONTENT: i32 = 0x1;
const wxSTC_UPDATE_SELECTION: i32 = 0x2;
const wxSTC_UPDATE_V_SCROLL: i32 = 0x4;
const wxSTC_UPDATE_H_SCROLL: i32 = 0x8;
const wxSTC_KEY_DOWN: i32 = 300;
const wxSTC_KEY_UP: i32 = 301;
const wxSTC_KEY_LEFT: i32 = 302;
const wxSTC_KEY_RIGHT: i32 = 303;
const wxSTC_KEY_HOME: i32 = 304;
const wxSTC_KEY_END: i32 = 305;
const wxSTC_KEY_PRIOR: i32 = 306;
const wxSTC_KEY_NEXT: i32 = 307;
const wxSTC_KEY_DELETE: i32 = 308;
const wxSTC_KEY_INSERT: i32 = 309;
const wxSTC_KEY_ESCAPE: i32 = 7;
const wxSTC_KEY_BACK: i32 = 8;
const wxSTC_KEY_TAB: i32 = 9;
const wxSTC_KEY_RETURN: i32 = 13;
const wxSTC_KEY_ADD: i32 = 310;
const wxSTC_KEY_SUBTRACT: i32 = 311;
const wxSTC_KEY_DIVIDE: i32 = 312;
const wxSTC_KEY_WIN: i32 = 313;
const wxSTC_KEY_RWIN: i32 = 314;
const wxSTC_KEY_MENU: i32 = 315;
const wxSTC_KEYMOD_NORM: i32 = 0;
const wxSTC_KEYMOD_SHIFT: i32 = 1;
const wxSTC_KEYMOD_CTRL: i32 = 2;
const wxSTC_KEYMOD_ALT: i32 = 4;
const wxSTC_KEYMOD_SUPER: i32 = 8;
const wxSTC_KEYMOD_META: i32 = 16;
const wxSTC_AC_FILLUP: i32 = 1;
const wxSTC_AC_DOUBLECLICK: i32 = 2;
const wxSTC_AC_TAB: i32 = 3;
const wxSTC_AC_NEWLINE: i32 = 4;
const wxSTC_AC_COMMAND: i32 = 5;
const wxSTC_LEX_CONTAINER: i32 = 0;
const wxSTC_LEX_NULL: i32 = 1;
const wxSTC_LEX_PYTHON: i32 = 2;
const wxSTC_LEX_CPP: i32 = 3;
const wxSTC_LEX_HTML: i32 = 4;
const wxSTC_LEX_XML: i32 = 5;
const wxSTC_LEX_PERL: i32 = 6;
const wxSTC_LEX_SQL: i32 = 7;
const wxSTC_LEX_VB: i32 = 8;
const wxSTC_LEX_PROPERTIES: i32 = 9;
const wxSTC_LEX_ERRORLIST: i32 = 10;
const wxSTC_LEX_MAKEFILE: i32 = 11;
const wxSTC_LEX_BATCH: i32 = 12;
const wxSTC_LEX_XCODE: i32 = 13;
const wxSTC_LEX_LATEX: i32 = 14;
const wxSTC_LEX_LUA: i32 = 15;
const wxSTC_LEX_DIFF: i32 = 16;
const wxSTC_LEX_CONF: i32 = 17;
const wxSTC_LEX_PASCAL: i32 = 18;
const wxSTC_LEX_AVE: i32 = 19;
const wxSTC_LEX_ADA: i32 = 20;
const wxSTC_LEX_LISP: i32 = 21;
const wxSTC_LEX_RUBY: i32 = 22;
const wxSTC_LEX_EIFFEL: i32 = 23;
const wxSTC_LEX_EIFFELKW: i32 = 24;
const wxSTC_LEX_TCL: i32 = 25;
const wxSTC_LEX_NNCRONTAB: i32 = 26;
const wxSTC_LEX_BULLANT: i32 = 27;
const wxSTC_LEX_VBSCRIPT: i32 = 28;
const wxSTC_LEX_BAAN: i32 = 31;
const wxSTC_LEX_MATLAB: i32 = 32;
const wxSTC_LEX_SCRIPTOL: i32 = 33;
const wxSTC_LEX_ASM: i32 = 34;
const wxSTC_LEX_CPPNOCASE: i32 = 35;
const wxSTC_LEX_FORTRAN: i32 = 36;
const wxSTC_LEX_F77: i32 = 37;
const wxSTC_LEX_CSS: i32 = 38;
const wxSTC_LEX_POV: i32 = 39;
const wxSTC_LEX_LOUT: i32 = 40;
const wxSTC_LEX_ESCRIPT: i32 = 41;
const wxSTC_LEX_PS: i32 = 42;
const wxSTC_LEX_NSIS: i32 = 43;
const wxSTC_LEX_MMIXAL: i32 = 44;
const wxSTC_LEX_CLW: i32 = 45;
const wxSTC_LEX_CLWNOCASE: i32 = 46;
const wxSTC_LEX_LOT: i32 = 47;
const wxSTC_LEX_YAML: i32 = 48;
const wxSTC_LEX_TEX: i32 = 49;
const wxSTC_LEX_METAPOST: i32 = 50;
const wxSTC_LEX_POWERBASIC: i32 = 51;
const wxSTC_LEX_FORTH: i32 = 52;
const wxSTC_LEX_ERLANG: i32 = 53;
const wxSTC_LEX_OCTAVE: i32 = 54;
const wxSTC_LEX_MSSQL: i32 = 55;
const wxSTC_LEX_VERILOG: i32 = 56;
const wxSTC_LEX_KIX: i32 = 57;
const wxSTC_LEX_GUI4CLI: i32 = 58;
const wxSTC_LEX_SPECMAN: i32 = 59;
const wxSTC_LEX_AU3: i32 = 60;
const wxSTC_LEX_APDL: i32 = 61;
const wxSTC_LEX_BASH: i32 = 62;
const wxSTC_LEX_ASN1: i32 = 63;
const wxSTC_LEX_VHDL: i32 = 64;
const wxSTC_LEX_CAML: i32 = 65;
const wxSTC_LEX_BLITZBASIC: i32 = 66;
const wxSTC_LEX_PUREBASIC: i32 = 67;
const wxSTC_LEX_HASKELL: i32 = 68;
const wxSTC_LEX_PHPSCRIPT: i32 = 69;
const wxSTC_LEX_TADS3: i32 = 70;
const wxSTC_LEX_REBOL: i32 = 71;
const wxSTC_LEX_SMALLTALK: i32 = 72;
const wxSTC_LEX_FLAGSHIP: i32 = 73;
const wxSTC_LEX_CSOUND: i32 = 74;
const wxSTC_LEX_FREEBASIC: i32 = 75;
const wxSTC_LEX_INNOSETUP: i32 = 76;
const wxSTC_LEX_OPAL: i32 = 77;
const wxSTC_LEX_SPICE: i32 = 78;
const wxSTC_LEX_D: i32 = 79;
const wxSTC_LEX_CMAKE: i32 = 80;
const wxSTC_LEX_GAP: i32 = 81;
const wxSTC_LEX_PLM: i32 = 82;
const wxSTC_LEX_PROGRESS: i32 = 83;
const wxSTC_LEX_ABAQUS: i32 = 84;
const wxSTC_LEX_ASYMPTOTE: i32 = 85;
const wxSTC_LEX_R: i32 = 86;
const wxSTC_LEX_MAGIK: i32 = 87;
const wxSTC_LEX_POWERSHELL: i32 = 88;
const wxSTC_LEX_MYSQL: i32 = 89;
const wxSTC_LEX_PO: i32 = 90;
const wxSTC_LEX_TAL: i32 = 91;
const wxSTC_LEX_COBOL: i32 = 92;
const wxSTC_LEX_TACL: i32 = 93;
const wxSTC_LEX_SORCUS: i32 = 94;
const wxSTC_LEX_POWERPRO: i32 = 95;
const wxSTC_LEX_NIMROD: i32 = 96;
const wxSTC_LEX_SML: i32 = 97;
const wxSTC_LEX_MARKDOWN: i32 = 98;
const wxSTC_LEX_TXT2TAGS: i32 = 99;
const wxSTC_LEX_A68K: i32 = 100;
const wxSTC_LEX_MODULA: i32 = 101;
const wxSTC_LEX_COFFEESCRIPT: i32 = 102;
const wxSTC_LEX_TCMD: i32 = 103;
const wxSTC_LEX_AVS: i32 = 104;
const wxSTC_LEX_ECL: i32 = 105;
const wxSTC_LEX_OSCRIPT: i32 = 106;
const wxSTC_LEX_VISUALPROLOG: i32 = 107;
const wxSTC_LEX_LITERATEHASKELL: i32 = 108;
const wxSTC_LEX_STTXT: i32 = 109;
const wxSTC_LEX_KVIRC: i32 = 110;
const wxSTC_LEX_RUST: i32 = 111;
const wxSTC_LEX_DMAP: i32 = 112;
const wxSTC_LEX_AS: i32 = 113;
const wxSTC_LEX_DMIS: i32 = 114;
const wxSTC_LEX_REGISTRY: i32 = 115;
const wxSTC_LEX_BIBTEX: i32 = 116;
const wxSTC_LEX_SREC: i32 = 117;
const wxSTC_LEX_IHEX: i32 = 118;
const wxSTC_LEX_TEHEX: i32 = 119;
const wxSTC_LEX_JSON: i32 = 120;
const wxSTC_LEX_EDIFACT: i32 = 121;
const wxSTC_LEX_AUTOMATIC: i32 = 1000;
const wxSTC_P_DEFAULT: i32 = 0;
const wxSTC_P_COMMENTLINE: i32 = 1;
const wxSTC_P_NUMBER: i32 = 2;
const wxSTC_P_STRING: i32 = 3;
const wxSTC_P_CHARACTER: i32 = 4;
const wxSTC_P_WORD: i32 = 5;
const wxSTC_P_TRIPLE: i32 = 6;
const wxSTC_P_TRIPLEDOUBLE: i32 = 7;
const wxSTC_P_CLASSNAME: i32 = 8;
const wxSTC_P_DEFNAME: i32 = 9;
const wxSTC_P_OPERATOR: i32 = 10;
const wxSTC_P_IDENTIFIER: i32 = 11;
const wxSTC_P_COMMENTBLOCK: i32 = 12;
const wxSTC_P_STRINGEOL: i32 = 13;
const wxSTC_P_WORD2: i32 = 14;
const wxSTC_P_DECORATOR: i32 = 15;
const wxSTC_C_DEFAULT: i32 = 0;
const wxSTC_C_COMMENT: i32 = 1;
const wxSTC_C_COMMENTLINE: i32 = 2;
const wxSTC_C_COMMENTDOC: i32 = 3;
const wxSTC_C_NUMBER: i32 = 4;
const wxSTC_C_WORD: i32 = 5;
const wxSTC_C_STRING: i32 = 6;
const wxSTC_C_CHARACTER: i32 = 7;
const wxSTC_C_UUID: i32 = 8;
const wxSTC_C_PREPROCESSOR: i32 = 9;
const wxSTC_C_OPERATOR: i32 = 10;
const wxSTC_C_IDENTIFIER: i32 = 11;
const wxSTC_C_STRINGEOL: i32 = 12;
const wxSTC_C_VERBATIM: i32 = 13;
const wxSTC_C_REGEX: i32 = 14;
const wxSTC_C_COMMENTLINEDOC: i32 = 15;
const wxSTC_C_WORD2: i32 = 16;
const wxSTC_C_COMMENTDOCKEYWORD: i32 = 17;
const wxSTC_C_COMMENTDOCKEYWORDERROR: i32 = 18;
const wxSTC_C_GLOBALCLASS: i32 = 19;
const wxSTC_C_STRINGRAW: i32 = 20;
const wxSTC_C_TRIPLEVERBATIM: i32 = 21;
const wxSTC_C_HASHQUOTEDSTRING: i32 = 22;
const wxSTC_C_PREPROCESSORCOMMENT: i32 = 23;
const wxSTC_C_PREPROCESSORCOMMENTDOC: i32 = 24;
const wxSTC_C_USERLITERAL: i32 = 25;
const wxSTC_C_TASKMARKER: i32 = 26;
const wxSTC_C_ESCAPESEQUENCE: i32 = 27;
const wxSTC_D_DEFAULT: i32 = 0;
const wxSTC_D_COMMENT: i32 = 1;
const wxSTC_D_COMMENTLINE: i32 = 2;
const wxSTC_D_COMMENTDOC: i32 = 3;
const wxSTC_D_COMMENTNESTED: i32 = 4;
const wxSTC_D_NUMBER: i32 = 5;
const wxSTC_D_WORD: i32 = 6;
const wxSTC_D_WORD2: i32 = 7;
const wxSTC_D_WORD3: i32 = 8;
const wxSTC_D_TYPEDEF: i32 = 9;
const wxSTC_D_STRING: i32 = 10;
const wxSTC_D_STRINGEOL: i32 = 11;
const wxSTC_D_CHARACTER: i32 = 12;
const wxSTC_D_OPERATOR: i32 = 13;
const wxSTC_D_IDENTIFIER: i32 = 14;
const wxSTC_D_COMMENTLINEDOC: i32 = 15;
const wxSTC_D_COMMENTDOCKEYWORD: i32 = 16;
const wxSTC_D_COMMENTDOCKEYWORDERROR: i32 = 17;
const wxSTC_D_STRINGB: i32 = 18;
const wxSTC_D_STRINGR: i32 = 19;
const wxSTC_D_WORD5: i32 = 20;
const wxSTC_D_WORD6: i32 = 21;
const wxSTC_D_WORD7: i32 = 22;
const wxSTC_TCL_DEFAULT: i32 = 0;
const wxSTC_TCL_COMMENT: i32 = 1;
const wxSTC_TCL_COMMENTLINE: i32 = 2;
const wxSTC_TCL_NUMBER: i32 = 3;
const wxSTC_TCL_WORD_IN_QUOTE: i32 = 4;
const wxSTC_TCL_IN_QUOTE: i32 = 5;
const wxSTC_TCL_OPERATOR: i32 = 6;
const wxSTC_TCL_IDENTIFIER: i32 = 7;
const wxSTC_TCL_SUBSTITUTION: i32 = 8;
const wxSTC_TCL_SUB_BRACE: i32 = 9;
const wxSTC_TCL_MODIFIER: i32 = 10;
const wxSTC_TCL_EXPAND: i32 = 11;
const wxSTC_TCL_WORD: i32 = 12;
const wxSTC_TCL_WORD2: i32 = 13;
const wxSTC_TCL_WORD3: i32 = 14;
const wxSTC_TCL_WORD4: i32 = 15;
const wxSTC_TCL_WORD5: i32 = 16;
const wxSTC_TCL_WORD6: i32 = 17;
const wxSTC_TCL_WORD7: i32 = 18;
const wxSTC_TCL_WORD8: i32 = 19;
const wxSTC_TCL_COMMENT_BOX: i32 = 20;
const wxSTC_TCL_BLOCK_COMMENT: i32 = 21;
const wxSTC_H_DEFAULT: i32 = 0;
const wxSTC_H_TAG: i32 = 1;
const wxSTC_H_TAGUNKNOWN: i32 = 2;
const wxSTC_H_ATTRIBUTE: i32 = 3;
const wxSTC_H_ATTRIBUTEUNKNOWN: i32 = 4;
const wxSTC_H_NUMBER: i32 = 5;
const wxSTC_H_DOUBLESTRING: i32 = 6;
const wxSTC_H_SINGLESTRING: i32 = 7;
const wxSTC_H_OTHER: i32 = 8;
const wxSTC_H_COMMENT: i32 = 9;
const wxSTC_H_ENTITY: i32 = 10;
const wxSTC_H_TAGEND: i32 = 11;
const wxSTC_H_XMLSTART: i32 = 12;
const wxSTC_H_XMLEND: i32 = 13;
const wxSTC_H_SCRIPT: i32 = 14;
const wxSTC_H_ASP: i32 = 15;
const wxSTC_H_ASPAT: i32 = 16;
const wxSTC_H_CDATA: i32 = 17;
const wxSTC_H_QUESTION: i32 = 18;
const wxSTC_H_VALUE: i32 = 19;
const wxSTC_H_XCCOMMENT: i32 = 20;
const wxSTC_H_SGML_DEFAULT: i32 = 21;
const wxSTC_H_SGML_COMMAND: i32 = 22;
const wxSTC_H_SGML_1ST_PARAM: i32 = 23;
const wxSTC_H_SGML_DOUBLESTRING: i32 = 24;
const wxSTC_H_SGML_SIMPLESTRING: i32 = 25;
const wxSTC_H_SGML_ERROR: i32 = 26;
const wxSTC_H_SGML_SPECIAL: i32 = 27;
const wxSTC_H_SGML_ENTITY: i32 = 28;
const wxSTC_H_SGML_COMMENT: i32 = 29;
const wxSTC_H_SGML_1ST_PARAM_COMMENT: i32 = 30;
const wxSTC_H_SGML_BLOCK_DEFAULT: i32 = 31;
const wxSTC_HJ_START: i32 = 40;
const wxSTC_HJ_DEFAULT: i32 = 41;
const wxSTC_HJ_COMMENT: i32 = 42;
const wxSTC_HJ_COMMENTLINE: i32 = 43;
const wxSTC_HJ_COMMENTDOC: i32 = 44;
const wxSTC_HJ_NUMBER: i32 = 45;
const wxSTC_HJ_WORD: i32 = 46;
const wxSTC_HJ_KEYWORD: i32 = 47;
const wxSTC_HJ_DOUBLESTRING: i32 = 48;
const wxSTC_HJ_SINGLESTRING: i32 = 49;
const wxSTC_HJ_SYMBOLS: i32 = 50;
const wxSTC_HJ_STRINGEOL: i32 = 51;
const wxSTC_HJ_REGEX: i32 = 52;
const wxSTC_HJA_START: i32 = 55;
const wxSTC_HJA_DEFAULT: i32 = 56;
const wxSTC_HJA_COMMENT: i32 = 57;
const wxSTC_HJA_COMMENTLINE: i32 = 58;
const wxSTC_HJA_COMMENTDOC: i32 = 59;
const wxSTC_HJA_NUMBER: i32 = 60;
const wxSTC_HJA_WORD: i32 = 61;
const wxSTC_HJA_KEYWORD: i32 = 62;
const wxSTC_HJA_DOUBLESTRING: i32 = 63;
const wxSTC_HJA_SINGLESTRING: i32 = 64;
const wxSTC_HJA_SYMBOLS: i32 = 65;
const wxSTC_HJA_STRINGEOL: i32 = 66;
const wxSTC_HJA_REGEX: i32 = 67;
const wxSTC_HB_START: i32 = 70;
const wxSTC_HB_DEFAULT: i32 = 71;
const wxSTC_HB_COMMENTLINE: i32 = 72;
const wxSTC_HB_NUMBER: i32 = 73;
const wxSTC_HB_WORD: i32 = 74;
const wxSTC_HB_STRING: i32 = 75;
const wxSTC_HB_IDENTIFIER: i32 = 76;
const wxSTC_HB_STRINGEOL: i32 = 77;
const wxSTC_HBA_START: i32 = 80;
const wxSTC_HBA_DEFAULT: i32 = 81;
const wxSTC_HBA_COMMENTLINE: i32 = 82;
const wxSTC_HBA_NUMBER: i32 = 83;
const wxSTC_HBA_WORD: i32 = 84;
const wxSTC_HBA_STRING: i32 = 85;
const wxSTC_HBA_IDENTIFIER: i32 = 86;
const wxSTC_HBA_STRINGEOL: i32 = 87;
const wxSTC_HP_START: i32 = 90;
const wxSTC_HP_DEFAULT: i32 = 91;
const wxSTC_HP_COMMENTLINE: i32 = 92;
const wxSTC_HP_NUMBER: i32 = 93;
const wxSTC_HP_STRING: i32 = 94;
const wxSTC_HP_CHARACTER: i32 = 95;
const wxSTC_HP_WORD: i32 = 96;
const wxSTC_HP_TRIPLE: i32 = 97;
const wxSTC_HP_TRIPLEDOUBLE: i32 = 98;
const wxSTC_HP_CLASSNAME: i32 = 99;
const wxSTC_HP_DEFNAME: i32 = 100;
const wxSTC_HP_OPERATOR: i32 = 101;
const wxSTC_HP_IDENTIFIER: i32 = 102;
const wxSTC_HPHP_COMPLEX_VARIABLE: i32 = 104;
const wxSTC_HPA_START: i32 = 105;
const wxSTC_HPA_DEFAULT: i32 = 106;
const wxSTC_HPA_COMMENTLINE: i32 = 107;
const wxSTC_HPA_NUMBER: i32 = 108;
const wxSTC_HPA_STRING: i32 = 109;
const wxSTC_HPA_CHARACTER: i32 = 110;
const wxSTC_HPA_WORD: i32 = 111;
const wxSTC_HPA_TRIPLE: i32 = 112;
const wxSTC_HPA_TRIPLEDOUBLE: i32 = 113;
const wxSTC_HPA_CLASSNAME: i32 = 114;
const wxSTC_HPA_DEFNAME: i32 = 115;
const wxSTC_HPA_OPERATOR: i32 = 116;
const wxSTC_HPA_IDENTIFIER: i32 = 117;
const wxSTC_HPHP_DEFAULT: i32 = 118;
const wxSTC_HPHP_HSTRING: i32 = 119;
const wxSTC_HPHP_SIMPLESTRING: i32 = 120;
const wxSTC_HPHP_WORD: i32 = 121;
const wxSTC_HPHP_NUMBER: i32 = 122;
const wxSTC_HPHP_VARIABLE: i32 = 123;
const wxSTC_HPHP_COMMENT: i32 = 124;
const wxSTC_HPHP_COMMENTLINE: i32 = 125;
const wxSTC_HPHP_HSTRING_VARIABLE: i32 = 126;
const wxSTC_HPHP_OPERATOR: i32 = 127;
const wxSTC_PL_DEFAULT: i32 = 0;
const wxSTC_PL_ERROR: i32 = 1;
const wxSTC_PL_COMMENTLINE: i32 = 2;
const wxSTC_PL_POD: i32 = 3;
const wxSTC_PL_NUMBER: i32 = 4;
const wxSTC_PL_WORD: i32 = 5;
const wxSTC_PL_STRING: i32 = 6;
const wxSTC_PL_CHARACTER: i32 = 7;
const wxSTC_PL_PUNCTUATION: i32 = 8;
const wxSTC_PL_PREPROCESSOR: i32 = 9;
const wxSTC_PL_OPERATOR: i32 = 10;
const wxSTC_PL_IDENTIFIER: i32 = 11;
const wxSTC_PL_SCALAR: i32 = 12;
const wxSTC_PL_ARRAY: i32 = 13;
const wxSTC_PL_HASH: i32 = 14;
const wxSTC_PL_SYMBOLTABLE: i32 = 15;
const wxSTC_PL_VARIABLE_INDEXER: i32 = 16;
const wxSTC_PL_REGEX: i32 = 17;
const wxSTC_PL_REGSUBST: i32 = 18;
const wxSTC_PL_LONGQUOTE: i32 = 19;
const wxSTC_PL_BACKTICKS: i32 = 20;
const wxSTC_PL_DATASECTION: i32 = 21;
const wxSTC_PL_HERE_DELIM: i32 = 22;
const wxSTC_PL_HERE_Q: i32 = 23;
const wxSTC_PL_HERE_QQ: i32 = 24;
const wxSTC_PL_HERE_QX: i32 = 25;
const wxSTC_PL_STRING_Q: i32 = 26;
const wxSTC_PL_STRING_QQ: i32 = 27;
const wxSTC_PL_STRING_QX: i32 = 28;
const wxSTC_PL_STRING_QR: i32 = 29;
const wxSTC_PL_STRING_QW: i32 = 30;
const wxSTC_PL_POD_VERB: i32 = 31;
const wxSTC_PL_SUB_PROTOTYPE: i32 = 40;
const wxSTC_PL_FORMAT_IDENT: i32 = 41;
const wxSTC_PL_FORMAT: i32 = 42;
const wxSTC_PL_STRING_VAR: i32 = 43;
const wxSTC_PL_XLAT: i32 = 44;
const wxSTC_PL_REGEX_VAR: i32 = 54;
const wxSTC_PL_REGSUBST_VAR: i32 = 55;
const wxSTC_PL_BACKTICKS_VAR: i32 = 57;
const wxSTC_PL_HERE_QQ_VAR: i32 = 61;
const wxSTC_PL_HERE_QX_VAR: i32 = 62;
const wxSTC_PL_STRING_QQ_VAR: i32 = 64;
const wxSTC_PL_STRING_QX_VAR: i32 = 65;
const wxSTC_PL_STRING_QR_VAR: i32 = 66;
const wxSTC_RB_DEFAULT: i32 = 0;
const wxSTC_RB_ERROR: i32 = 1;
const wxSTC_RB_COMMENTLINE: i32 = 2;
const wxSTC_RB_POD: i32 = 3;
const wxSTC_RB_NUMBER: i32 = 4;
const wxSTC_RB_WORD: i32 = 5;
const wxSTC_RB_STRING: i32 = 6;
const wxSTC_RB_CHARACTER: i32 = 7;
const wxSTC_RB_CLASSNAME: i32 = 8;
const wxSTC_RB_DEFNAME: i32 = 9;
const wxSTC_RB_OPERATOR: i32 = 10;
const wxSTC_RB_IDENTIFIER: i32 = 11;
const wxSTC_RB_REGEX: i32 = 12;
const wxSTC_RB_GLOBAL: i32 = 13;
const wxSTC_RB_SYMBOL: i32 = 14;
const wxSTC_RB_MODULE_NAME: i32 = 15;
const wxSTC_RB_INSTANCE_VAR: i32 = 16;
const wxSTC_RB_CLASS_VAR: i32 = 17;
const wxSTC_RB_BACKTICKS: i32 = 18;
const wxSTC_RB_DATASECTION: i32 = 19;
const wxSTC_RB_HERE_DELIM: i32 = 20;
const wxSTC_RB_HERE_Q: i32 = 21;
const wxSTC_RB_HERE_QQ: i32 = 22;
const wxSTC_RB_HERE_QX: i32 = 23;
const wxSTC_RB_STRING_Q: i32 = 24;
const wxSTC_RB_STRING_QQ: i32 = 25;
const wxSTC_RB_STRING_QX: i32 = 26;
const wxSTC_RB_STRING_QR: i32 = 27;
const wxSTC_RB_STRING_QW: i32 = 28;
const wxSTC_RB_WORD_DEMOTED: i32 = 29;
const wxSTC_RB_STDIN: i32 = 30;
const wxSTC_RB_STDOUT: i32 = 31;
const wxSTC_RB_STDERR: i32 = 40;
const wxSTC_RB_UPPER_BOUND: i32 = 41;
const wxSTC_B_DEFAULT: i32 = 0;
const wxSTC_B_COMMENT: i32 = 1;
const wxSTC_B_NUMBER: i32 = 2;
const wxSTC_B_KEYWORD: i32 = 3;
const wxSTC_B_STRING: i32 = 4;
const wxSTC_B_PREPROCESSOR: i32 = 5;
const wxSTC_B_OPERATOR: i32 = 6;
const wxSTC_B_IDENTIFIER: i32 = 7;
const wxSTC_B_DATE: i32 = 8;
const wxSTC_B_STRINGEOL: i32 = 9;
const wxSTC_B_KEYWORD2: i32 = 10;
const wxSTC_B_KEYWORD3: i32 = 11;
const wxSTC_B_KEYWORD4: i32 = 12;
const wxSTC_B_CONSTANT: i32 = 13;
const wxSTC_B_ASM: i32 = 14;
const wxSTC_B_LABEL: i32 = 15;
const wxSTC_B_ERROR: i32 = 16;
const wxSTC_B_HEXNUMBER: i32 = 17;
const wxSTC_B_BINNUMBER: i32 = 18;
const wxSTC_B_COMMENTBLOCK: i32 = 19;
const wxSTC_B_DOCLINE: i32 = 20;
const wxSTC_B_DOCBLOCK: i32 = 21;
const wxSTC_B_DOCKEYWORD: i32 = 22;
const wxSTC_PROPS_DEFAULT: i32 = 0;
const wxSTC_PROPS_COMMENT: i32 = 1;
const wxSTC_PROPS_SECTION: i32 = 2;
const wxSTC_PROPS_ASSIGNMENT: i32 = 3;
const wxSTC_PROPS_DEFVAL: i32 = 4;
const wxSTC_PROPS_KEY: i32 = 5;
const wxSTC_L_DEFAULT: i32 = 0;
const wxSTC_L_COMMAND: i32 = 1;
const wxSTC_L_TAG: i32 = 2;
const wxSTC_L_MATH: i32 = 3;
const wxSTC_L_COMMENT: i32 = 4;
const wxSTC_L_TAG2: i32 = 5;
const wxSTC_L_MATH2: i32 = 6;
const wxSTC_L_COMMENT2: i32 = 7;
const wxSTC_L_VERBATIM: i32 = 8;
const wxSTC_L_SHORTCMD: i32 = 9;
const wxSTC_L_SPECIAL: i32 = 10;
const wxSTC_L_CMDOPT: i32 = 11;
const wxSTC_L_ERROR: i32 = 12;
const wxSTC_LUA_DEFAULT: i32 = 0;
const wxSTC_LUA_COMMENT: i32 = 1;
const wxSTC_LUA_COMMENTLINE: i32 = 2;
const wxSTC_LUA_COMMENTDOC: i32 = 3;
const wxSTC_LUA_NUMBER: i32 = 4;
const wxSTC_LUA_WORD: i32 = 5;
const wxSTC_LUA_STRING: i32 = 6;
const wxSTC_LUA_CHARACTER: i32 = 7;
const wxSTC_LUA_LITERALSTRING: i32 = 8;
const wxSTC_LUA_PREPROCESSOR: i32 = 9;
const wxSTC_LUA_OPERATOR: i32 = 10;
const wxSTC_LUA_IDENTIFIER: i32 = 11;
const wxSTC_LUA_STRINGEOL: i32 = 12;
const wxSTC_LUA_WORD2: i32 = 13;
const wxSTC_LUA_WORD3: i32 = 14;
const wxSTC_LUA_WORD4: i32 = 15;
const wxSTC_LUA_WORD5: i32 = 16;
const wxSTC_LUA_WORD6: i32 = 17;
const wxSTC_LUA_WORD7: i32 = 18;
const wxSTC_LUA_WORD8: i32 = 19;
const wxSTC_LUA_LABEL: i32 = 20;
const wxSTC_ERR_DEFAULT: i32 = 0;
const wxSTC_ERR_PYTHON: i32 = 1;
const wxSTC_ERR_GCC: i32 = 2;
const wxSTC_ERR_MS: i32 = 3;
const wxSTC_ERR_CMD: i32 = 4;
const wxSTC_ERR_BORLAND: i32 = 5;
const wxSTC_ERR_PERL: i32 = 6;
const wxSTC_ERR_NET: i32 = 7;
const wxSTC_ERR_LUA: i32 = 8;
const wxSTC_ERR_CTAG: i32 = 9;
const wxSTC_ERR_DIFF_CHANGED: i32 = 10;
const wxSTC_ERR_DIFF_ADDITION: i32 = 11;
const wxSTC_ERR_DIFF_DELETION: i32 = 12;
const wxSTC_ERR_DIFF_MESSAGE: i32 = 13;
const wxSTC_ERR_PHP: i32 = 14;
const wxSTC_ERR_ELF: i32 = 15;
const wxSTC_ERR_IFC: i32 = 16;
const wxSTC_ERR_IFORT: i32 = 17;
const wxSTC_ERR_ABSF: i32 = 18;
const wxSTC_ERR_TIDY: i32 = 19;
const wxSTC_ERR_JAVA_STACK: i32 = 20;
const wxSTC_ERR_VALUE: i32 = 21;
const wxSTC_ERR_GCC_INCLUDED_FROM: i32 = 22;
const wxSTC_ERR_ESCSEQ: i32 = 23;
const wxSTC_ERR_ESCSEQ_UNKNOWN: i32 = 24;
const wxSTC_ERR_ES_BLACK: i32 = 40;
const wxSTC_ERR_ES_RED: i32 = 41;
const wxSTC_ERR_ES_GREEN: i32 = 42;
const wxSTC_ERR_ES_BROWN: i32 = 43;
const wxSTC_ERR_ES_BLUE: i32 = 44;
const wxSTC_ERR_ES_MAGENTA: i32 = 45;
const wxSTC_ERR_ES_CYAN: i32 = 46;
const wxSTC_ERR_ES_GRAY: i32 = 47;
const wxSTC_ERR_ES_DARK_GRAY: i32 = 48;
const wxSTC_ERR_ES_BRIGHT_RED: i32 = 49;
const wxSTC_ERR_ES_BRIGHT_GREEN: i32 = 50;
const wxSTC_ERR_ES_YELLOW: i32 = 51;
const wxSTC_ERR_ES_BRIGHT_BLUE: i32 = 52;
const wxSTC_ERR_ES_BRIGHT_MAGENTA: i32 = 53;
const wxSTC_ERR_ES_BRIGHT_CYAN: i32 = 54;
const wxSTC_ERR_ES_WHITE: i32 = 55;
const wxSTC_BAT_DEFAULT: i32 = 0;
const wxSTC_BAT_COMMENT: i32 = 1;
const wxSTC_BAT_WORD: i32 = 2;
const wxSTC_BAT_LABEL: i32 = 3;
const wxSTC_BAT_HIDE: i32 = 4;
const wxSTC_BAT_COMMAND: i32 = 5;
const wxSTC_BAT_IDENTIFIER: i32 = 6;
const wxSTC_BAT_OPERATOR: i32 = 7;
const wxSTC_TCMD_DEFAULT: i32 = 0;
const wxSTC_TCMD_COMMENT: i32 = 1;
const wxSTC_TCMD_WORD: i32 = 2;
const wxSTC_TCMD_LABEL: i32 = 3;
const wxSTC_TCMD_HIDE: i32 = 4;
const wxSTC_TCMD_COMMAND: i32 = 5;
const wxSTC_TCMD_IDENTIFIER: i32 = 6;
const wxSTC_TCMD_OPERATOR: i32 = 7;
const wxSTC_TCMD_ENVIRONMENT: i32 = 8;
const wxSTC_TCMD_EXPANSION: i32 = 9;
const wxSTC_TCMD_CLABEL: i32 = 10;
const wxSTC_MAKE_DEFAULT: i32 = 0;
const wxSTC_MAKE_COMMENT: i32 = 1;
const wxSTC_MAKE_PREPROCESSOR: i32 = 2;
const wxSTC_MAKE_IDENTIFIER: i32 = 3;
const wxSTC_MAKE_OPERATOR: i32 = 4;
const wxSTC_MAKE_TARGET: i32 = 5;
const wxSTC_MAKE_IDEOL: i32 = 9;
const wxSTC_DIFF_DEFAULT: i32 = 0;
const wxSTC_DIFF_COMMENT: i32 = 1;
const wxSTC_DIFF_COMMAND: i32 = 2;
const wxSTC_DIFF_HEADER: i32 = 3;
const wxSTC_DIFF_POSITION: i32 = 4;
const wxSTC_DIFF_DELETED: i32 = 5;
const wxSTC_DIFF_ADDED: i32 = 6;
const wxSTC_DIFF_CHANGED: i32 = 7;
const wxSTC_CONF_DEFAULT: i32 = 0;
const wxSTC_CONF_COMMENT: i32 = 1;
const wxSTC_CONF_NUMBER: i32 = 2;
const wxSTC_CONF_IDENTIFIER: i32 = 3;
const wxSTC_CONF_EXTENSION: i32 = 4;
const wxSTC_CONF_PARAMETER: i32 = 5;
const wxSTC_CONF_STRING: i32 = 6;
const wxSTC_CONF_OPERATOR: i32 = 7;
const wxSTC_CONF_IP: i32 = 8;
const wxSTC_CONF_DIRECTIVE: i32 = 9;
const wxSTC_AVE_DEFAULT: i32 = 0;
const wxSTC_AVE_COMMENT: i32 = 1;
const wxSTC_AVE_NUMBER: i32 = 2;
const wxSTC_AVE_WORD: i32 = 3;
const wxSTC_AVE_STRING: i32 = 6;
const wxSTC_AVE_ENUM: i32 = 7;
const wxSTC_AVE_STRINGEOL: i32 = 8;
const wxSTC_AVE_IDENTIFIER: i32 = 9;
const wxSTC_AVE_OPERATOR: i32 = 10;
const wxSTC_AVE_WORD1: i32 = 11;
const wxSTC_AVE_WORD2: i32 = 12;
const wxSTC_AVE_WORD3: i32 = 13;
const wxSTC_AVE_WORD4: i32 = 14;
const wxSTC_AVE_WORD5: i32 = 15;
const wxSTC_AVE_WORD6: i32 = 16;
const wxSTC_ADA_DEFAULT: i32 = 0;
const wxSTC_ADA_WORD: i32 = 1;
const wxSTC_ADA_IDENTIFIER: i32 = 2;
const wxSTC_ADA_NUMBER: i32 = 3;
const wxSTC_ADA_DELIMITER: i32 = 4;
const wxSTC_ADA_CHARACTER: i32 = 5;
const wxSTC_ADA_CHARACTEREOL: i32 = 6;
const wxSTC_ADA_STRING: i32 = 7;
const wxSTC_ADA_STRINGEOL: i32 = 8;
const wxSTC_ADA_LABEL: i32 = 9;
const wxSTC_ADA_COMMENTLINE: i32 = 10;
const wxSTC_ADA_ILLEGAL: i32 = 11;
const wxSTC_BAAN_DEFAULT: i32 = 0;
const wxSTC_BAAN_COMMENT: i32 = 1;
const wxSTC_BAAN_COMMENTDOC: i32 = 2;
const wxSTC_BAAN_NUMBER: i32 = 3;
const wxSTC_BAAN_WORD: i32 = 4;
const wxSTC_BAAN_STRING: i32 = 5;
const wxSTC_BAAN_PREPROCESSOR: i32 = 6;
const wxSTC_BAAN_OPERATOR: i32 = 7;
const wxSTC_BAAN_IDENTIFIER: i32 = 8;
const wxSTC_BAAN_STRINGEOL: i32 = 9;
const wxSTC_BAAN_WORD2: i32 = 10;
const wxSTC_BAAN_WORD3: i32 = 11;
const wxSTC_BAAN_WORD4: i32 = 12;
const wxSTC_BAAN_WORD5: i32 = 13;
const wxSTC_BAAN_WORD6: i32 = 14;
const wxSTC_BAAN_WORD7: i32 = 15;
const wxSTC_BAAN_WORD8: i32 = 16;
const wxSTC_BAAN_WORD9: i32 = 17;
const wxSTC_BAAN_TABLEDEF: i32 = 18;
const wxSTC_BAAN_TABLESQL: i32 = 19;
const wxSTC_BAAN_FUNCTION: i32 = 20;
const wxSTC_BAAN_DOMDEF: i32 = 21;
const wxSTC_BAAN_FUNCDEF: i32 = 22;
const wxSTC_BAAN_OBJECTDEF: i32 = 23;
const wxSTC_BAAN_DEFINEDEF: i32 = 24;
const wxSTC_LISP_DEFAULT: i32 = 0;
const wxSTC_LISP_COMMENT: i32 = 1;
const wxSTC_LISP_NUMBER: i32 = 2;
const wxSTC_LISP_KEYWORD: i32 = 3;
const wxSTC_LISP_KEYWORD_KW: i32 = 4;
const wxSTC_LISP_SYMBOL: i32 = 5;
const wxSTC_LISP_STRING: i32 = 6;
const wxSTC_LISP_STRINGEOL: i32 = 8;
const wxSTC_LISP_IDENTIFIER: i32 = 9;
const wxSTC_LISP_OPERATOR: i32 = 10;
const wxSTC_LISP_SPECIAL: i32 = 11;
const wxSTC_LISP_MULTI_COMMENT: i32 = 12;
const wxSTC_EIFFEL_DEFAULT: i32 = 0;
const wxSTC_EIFFEL_COMMENTLINE: i32 = 1;
const wxSTC_EIFFEL_NUMBER: i32 = 2;
const wxSTC_EIFFEL_WORD: i32 = 3;
const wxSTC_EIFFEL_STRING: i32 = 4;
const wxSTC_EIFFEL_CHARACTER: i32 = 5;
const wxSTC_EIFFEL_OPERATOR: i32 = 6;
const wxSTC_EIFFEL_IDENTIFIER: i32 = 7;
const wxSTC_EIFFEL_STRINGEOL: i32 = 8;
const wxSTC_NNCRONTAB_DEFAULT: i32 = 0;
const wxSTC_NNCRONTAB_COMMENT: i32 = 1;
const wxSTC_NNCRONTAB_TASK: i32 = 2;
const wxSTC_NNCRONTAB_SECTION: i32 = 3;
const wxSTC_NNCRONTAB_KEYWORD: i32 = 4;
const wxSTC_NNCRONTAB_MODIFIER: i32 = 5;
const wxSTC_NNCRONTAB_ASTERISK: i32 = 6;
const wxSTC_NNCRONTAB_NUMBER: i32 = 7;
const wxSTC_NNCRONTAB_STRING: i32 = 8;
const wxSTC_NNCRONTAB_ENVIRONMENT: i32 = 9;
const wxSTC_NNCRONTAB_IDENTIFIER: i32 = 10;
const wxSTC_FORTH_DEFAULT: i32 = 0;
const wxSTC_FORTH_COMMENT: i32 = 1;
const wxSTC_FORTH_COMMENT_ML: i32 = 2;
const wxSTC_FORTH_IDENTIFIER: i32 = 3;
const wxSTC_FORTH_CONTROL: i32 = 4;
const wxSTC_FORTH_KEYWORD: i32 = 5;
const wxSTC_FORTH_DEFWORD: i32 = 6;
const wxSTC_FORTH_PREWORD1: i32 = 7;
const wxSTC_FORTH_PREWORD2: i32 = 8;
const wxSTC_FORTH_NUMBER: i32 = 9;
const wxSTC_FORTH_STRING: i32 = 10;
const wxSTC_FORTH_LOCALE: i32 = 11;
const wxSTC_MATLAB_DEFAULT: i32 = 0;
const wxSTC_MATLAB_COMMENT: i32 = 1;
const wxSTC_MATLAB_COMMAND: i32 = 2;
const wxSTC_MATLAB_NUMBER: i32 = 3;
const wxSTC_MATLAB_KEYWORD: i32 = 4;
const wxSTC_MATLAB_STRING: i32 = 5;
const wxSTC_MATLAB_OPERATOR: i32 = 6;
const wxSTC_MATLAB_IDENTIFIER: i32 = 7;
const wxSTC_MATLAB_DOUBLEQUOTESTRING: i32 = 8;
const wxSTC_SCRIPTOL_DEFAULT: i32 = 0;
const wxSTC_SCRIPTOL_WHITE: i32 = 1;
const wxSTC_SCRIPTOL_COMMENTLINE: i32 = 2;
const wxSTC_SCRIPTOL_PERSISTENT: i32 = 3;
const wxSTC_SCRIPTOL_CSTYLE: i32 = 4;
const wxSTC_SCRIPTOL_COMMENTBLOCK: i32 = 5;
const wxSTC_SCRIPTOL_NUMBER: i32 = 6;
const wxSTC_SCRIPTOL_STRING: i32 = 7;
const wxSTC_SCRIPTOL_CHARACTER: i32 = 8;
const wxSTC_SCRIPTOL_STRINGEOL: i32 = 9;
const wxSTC_SCRIPTOL_KEYWORD: i32 = 10;
const wxSTC_SCRIPTOL_OPERATOR: i32 = 11;
const wxSTC_SCRIPTOL_IDENTIFIER: i32 = 12;
const wxSTC_SCRIPTOL_TRIPLE: i32 = 13;
const wxSTC_SCRIPTOL_CLASSNAME: i32 = 14;
const wxSTC_SCRIPTOL_PREPROCESSOR: i32 = 15;
const wxSTC_ASM_DEFAULT: i32 = 0;
const wxSTC_ASM_COMMENT: i32 = 1;
const wxSTC_ASM_NUMBER: i32 = 2;
const wxSTC_ASM_STRING: i32 = 3;
const wxSTC_ASM_OPERATOR: i32 = 4;
const wxSTC_ASM_IDENTIFIER: i32 = 5;
const wxSTC_ASM_CPUINSTRUCTION: i32 = 6;
const wxSTC_ASM_MATHINSTRUCTION: i32 = 7;
const wxSTC_ASM_REGISTER: i32 = 8;
const wxSTC_ASM_DIRECTIVE: i32 = 9;
const wxSTC_ASM_DIRECTIVEOPERAND: i32 = 10;
const wxSTC_ASM_COMMENTBLOCK: i32 = 11;
const wxSTC_ASM_CHARACTER: i32 = 12;
const wxSTC_ASM_STRINGEOL: i32 = 13;
const wxSTC_ASM_EXTINSTRUCTION: i32 = 14;
const wxSTC_ASM_COMMENTDIRECTIVE: i32 = 15;
const wxSTC_F_DEFAULT: i32 = 0;
const wxSTC_F_COMMENT: i32 = 1;
const wxSTC_F_NUMBER: i32 = 2;
const wxSTC_F_STRING1: i32 = 3;
const wxSTC_F_STRING2: i32 = 4;
const wxSTC_F_STRINGEOL: i32 = 5;
const wxSTC_F_OPERATOR: i32 = 6;
const wxSTC_F_IDENTIFIER: i32 = 7;
const wxSTC_F_WORD: i32 = 8;
const wxSTC_F_WORD2: i32 = 9;
const wxSTC_F_WORD3: i32 = 10;
const wxSTC_F_PREPROCESSOR: i32 = 11;
const wxSTC_F_OPERATOR2: i32 = 12;
const wxSTC_F_LABEL: i32 = 13;
const wxSTC_F_CONTINUATION: i32 = 14;
const wxSTC_CSS_DEFAULT: i32 = 0;
const wxSTC_CSS_TAG: i32 = 1;
const wxSTC_CSS_CLASS: i32 = 2;
const wxSTC_CSS_PSEUDOCLASS: i32 = 3;
const wxSTC_CSS_UNKNOWN_PSEUDOCLASS: i32 = 4;
const wxSTC_CSS_OPERATOR: i32 = 5;
const wxSTC_CSS_IDENTIFIER: i32 = 6;
const wxSTC_CSS_UNKNOWN_IDENTIFIER: i32 = 7;
const wxSTC_CSS_VALUE: i32 = 8;
const wxSTC_CSS_COMMENT: i32 = 9;
const wxSTC_CSS_ID: i32 = 10;
const wxSTC_CSS_IMPORTANT: i32 = 11;
const wxSTC_CSS_DIRECTIVE: i32 = 12;
const wxSTC_CSS_DOUBLESTRING: i32 = 13;
const wxSTC_CSS_SINGLESTRING: i32 = 14;
const wxSTC_CSS_IDENTIFIER2: i32 = 15;
const wxSTC_CSS_ATTRIBUTE: i32 = 16;
const wxSTC_CSS_IDENTIFIER3: i32 = 17;
const wxSTC_CSS_PSEUDOELEMENT: i32 = 18;
const wxSTC_CSS_EXTENDED_IDENTIFIER: i32 = 19;
const wxSTC_CSS_EXTENDED_PSEUDOCLASS: i32 = 20;
const wxSTC_CSS_EXTENDED_PSEUDOELEMENT: i32 = 21;
const wxSTC_CSS_MEDIA: i32 = 22;
const wxSTC_CSS_VARIABLE: i32 = 23;
const wxSTC_POV_DEFAULT: i32 = 0;
const wxSTC_POV_COMMENT: i32 = 1;
const wxSTC_POV_COMMENTLINE: i32 = 2;
const wxSTC_POV_NUMBER: i32 = 3;
const wxSTC_POV_OPERATOR: i32 = 4;
const wxSTC_POV_IDENTIFIER: i32 = 5;
const wxSTC_POV_STRING: i32 = 6;
const wxSTC_POV_STRINGEOL: i32 = 7;
const wxSTC_POV_DIRECTIVE: i32 = 8;
const wxSTC_POV_BADDIRECTIVE: i32 = 9;
const wxSTC_POV_WORD2: i32 = 10;
const wxSTC_POV_WORD3: i32 = 11;
const wxSTC_POV_WORD4: i32 = 12;
const wxSTC_POV_WORD5: i32 = 13;
const wxSTC_POV_WORD6: i32 = 14;
const wxSTC_POV_WORD7: i32 = 15;
const wxSTC_POV_WORD8: i32 = 16;
const wxSTC_LOUT_DEFAULT: i32 = 0;
const wxSTC_LOUT_COMMENT: i32 = 1;
const wxSTC_LOUT_NUMBER: i32 = 2;
const wxSTC_LOUT_WORD: i32 = 3;
const wxSTC_LOUT_WORD2: i32 = 4;
const wxSTC_LOUT_WORD3: i32 = 5;
const wxSTC_LOUT_WORD4: i32 = 6;
const wxSTC_LOUT_STRING: i32 = 7;
const wxSTC_LOUT_OPERATOR: i32 = 8;
const wxSTC_LOUT_IDENTIFIER: i32 = 9;
const wxSTC_LOUT_STRINGEOL: i32 = 10;
const wxSTC_ESCRIPT_DEFAULT: i32 = 0;
const wxSTC_ESCRIPT_COMMENT: i32 = 1;
const wxSTC_ESCRIPT_COMMENTLINE: i32 = 2;
const wxSTC_ESCRIPT_COMMENTDOC: i32 = 3;
const wxSTC_ESCRIPT_NUMBER: i32 = 4;
const wxSTC_ESCRIPT_WORD: i32 = 5;
const wxSTC_ESCRIPT_STRING: i32 = 6;
const wxSTC_ESCRIPT_OPERATOR: i32 = 7;
const wxSTC_ESCRIPT_IDENTIFIER: i32 = 8;
const wxSTC_ESCRIPT_BRACE: i32 = 9;
const wxSTC_ESCRIPT_WORD2: i32 = 10;
const wxSTC_ESCRIPT_WORD3: i32 = 11;
const wxSTC_PS_DEFAULT: i32 = 0;
const wxSTC_PS_COMMENT: i32 = 1;
const wxSTC_PS_DSC_COMMENT: i32 = 2;
const wxSTC_PS_DSC_VALUE: i32 = 3;
const wxSTC_PS_NUMBER: i32 = 4;
const wxSTC_PS_NAME: i32 = 5;
const wxSTC_PS_KEYWORD: i32 = 6;
const wxSTC_PS_LITERAL: i32 = 7;
const wxSTC_PS_IMMEVAL: i32 = 8;
const wxSTC_PS_PAREN_ARRAY: i32 = 9;
const wxSTC_PS_PAREN_DICT: i32 = 10;
const wxSTC_PS_PAREN_PROC: i32 = 11;
const wxSTC_PS_TEXT: i32 = 12;
const wxSTC_PS_HEXSTRING: i32 = 13;
const wxSTC_PS_BASE85STRING: i32 = 14;
const wxSTC_PS_BADSTRINGCHAR: i32 = 15;
const wxSTC_NSIS_DEFAULT: i32 = 0;
const wxSTC_NSIS_COMMENT: i32 = 1;
const wxSTC_NSIS_STRINGDQ: i32 = 2;
const wxSTC_NSIS_STRINGLQ: i32 = 3;
const wxSTC_NSIS_STRINGRQ: i32 = 4;
const wxSTC_NSIS_FUNCTION: i32 = 5;
const wxSTC_NSIS_VARIABLE: i32 = 6;
const wxSTC_NSIS_LABEL: i32 = 7;
const wxSTC_NSIS_USERDEFINED: i32 = 8;
const wxSTC_NSIS_SECTIONDEF: i32 = 9;
const wxSTC_NSIS_SUBSECTIONDEF: i32 = 10;
const wxSTC_NSIS_IFDEFINEDEF: i32 = 11;
const wxSTC_NSIS_MACRODEF: i32 = 12;
const wxSTC_NSIS_STRINGVAR: i32 = 13;
const wxSTC_NSIS_NUMBER: i32 = 14;
const wxSTC_NSIS_SECTIONGROUP: i32 = 15;
const wxSTC_NSIS_PAGEEX: i32 = 16;
const wxSTC_NSIS_FUNCTIONDEF: i32 = 17;
const wxSTC_NSIS_COMMENTBOX: i32 = 18;
const wxSTC_MMIXAL_LEADWS: i32 = 0;
const wxSTC_MMIXAL_COMMENT: i32 = 1;
const wxSTC_MMIXAL_LABEL: i32 = 2;
const wxSTC_MMIXAL_OPCODE: i32 = 3;
const wxSTC_MMIXAL_OPCODE_PRE: i32 = 4;
const wxSTC_MMIXAL_OPCODE_VALID: i32 = 5;
const wxSTC_MMIXAL_OPCODE_UNKNOWN: i32 = 6;
const wxSTC_MMIXAL_OPCODE_POST: i32 = 7;
const wxSTC_MMIXAL_OPERANDS: i32 = 8;
const wxSTC_MMIXAL_NUMBER: i32 = 9;
const wxSTC_MMIXAL_REF: i32 = 10;
const wxSTC_MMIXAL_CHAR: i32 = 11;
const wxSTC_MMIXAL_STRING: i32 = 12;
const wxSTC_MMIXAL_REGISTER: i32 = 13;
const wxSTC_MMIXAL_HEX: i32 = 14;
const wxSTC_MMIXAL_OPERATOR: i32 = 15;
const wxSTC_MMIXAL_SYMBOL: i32 = 16;
const wxSTC_MMIXAL_INCLUDE: i32 = 17;
const wxSTC_CLW_DEFAULT: i32 = 0;
const wxSTC_CLW_LABEL: i32 = 1;
const wxSTC_CLW_COMMENT: i32 = 2;
const wxSTC_CLW_STRING: i32 = 3;
const wxSTC_CLW_USER_IDENTIFIER: i32 = 4;
const wxSTC_CLW_INTEGER_CONSTANT: i32 = 5;
const wxSTC_CLW_REAL_CONSTANT: i32 = 6;
const wxSTC_CLW_PICTURE_STRING: i32 = 7;
const wxSTC_CLW_KEYWORD: i32 = 8;
const wxSTC_CLW_COMPILER_DIRECTIVE: i32 = 9;
const wxSTC_CLW_RUNTIME_EXPRESSIONS: i32 = 10;
const wxSTC_CLW_BUILTIN_PROCEDURES_FUNCTION: i32 = 11;
const wxSTC_CLW_STRUCTURE_DATA_TYPE: i32 = 12;
const wxSTC_CLW_ATTRIBUTE: i32 = 13;
const wxSTC_CLW_STANDARD_EQUATE: i32 = 14;
const wxSTC_CLW_ERROR: i32 = 15;
const wxSTC_CLW_DEPRECATED: i32 = 16;
const wxSTC_LOT_DEFAULT: i32 = 0;
const wxSTC_LOT_HEADER: i32 = 1;
const wxSTC_LOT_BREAK: i32 = 2;
const wxSTC_LOT_SET: i32 = 3;
const wxSTC_LOT_PASS: i32 = 4;
const wxSTC_LOT_FAIL: i32 = 5;
const wxSTC_LOT_ABORT: i32 = 6;
const wxSTC_YAML_DEFAULT: i32 = 0;
const wxSTC_YAML_COMMENT: i32 = 1;
const wxSTC_YAML_IDENTIFIER: i32 = 2;
const wxSTC_YAML_KEYWORD: i32 = 3;
const wxSTC_YAML_NUMBER: i32 = 4;
const wxSTC_YAML_REFERENCE: i32 = 5;
const wxSTC_YAML_DOCUMENT: i32 = 6;
const wxSTC_YAML_TEXT: i32 = 7;
const wxSTC_YAML_ERROR: i32 = 8;
const wxSTC_YAML_OPERATOR: i32 = 9;
const wxSTC_TEX_DEFAULT: i32 = 0;
const wxSTC_TEX_SPECIAL: i32 = 1;
const wxSTC_TEX_GROUP: i32 = 2;
const wxSTC_TEX_SYMBOL: i32 = 3;
const wxSTC_TEX_COMMAND: i32 = 4;
const wxSTC_TEX_TEXT: i32 = 5;
const wxSTC_METAPOST_DEFAULT: i32 = 0;
const wxSTC_METAPOST_SPECIAL: i32 = 1;
const wxSTC_METAPOST_GROUP: i32 = 2;
const wxSTC_METAPOST_SYMBOL: i32 = 3;
const wxSTC_METAPOST_COMMAND: i32 = 4;
const wxSTC_METAPOST_TEXT: i32 = 5;
const wxSTC_METAPOST_EXTRA: i32 = 6;
const wxSTC_ERLANG_DEFAULT: i32 = 0;
const wxSTC_ERLANG_COMMENT: i32 = 1;
const wxSTC_ERLANG_VARIABLE: i32 = 2;
const wxSTC_ERLANG_NUMBER: i32 = 3;
const wxSTC_ERLANG_KEYWORD: i32 = 4;
const wxSTC_ERLANG_STRING: i32 = 5;
const wxSTC_ERLANG_OPERATOR: i32 = 6;
const wxSTC_ERLANG_ATOM: i32 = 7;
const wxSTC_ERLANG_FUNCTION_NAME: i32 = 8;
const wxSTC_ERLANG_CHARACTER: i32 = 9;
const wxSTC_ERLANG_MACRO: i32 = 10;
const wxSTC_ERLANG_RECORD: i32 = 11;
const wxSTC_ERLANG_PREPROC: i32 = 12;
const wxSTC_ERLANG_NODE_NAME: i32 = 13;
const wxSTC_ERLANG_COMMENT_FUNCTION: i32 = 14;
const wxSTC_ERLANG_COMMENT_MODULE: i32 = 15;
const wxSTC_ERLANG_COMMENT_DOC: i32 = 16;
const wxSTC_ERLANG_COMMENT_DOC_MACRO: i32 = 17;
const wxSTC_ERLANG_ATOM_QUOTED: i32 = 18;
const wxSTC_ERLANG_MACRO_QUOTED: i32 = 19;
const wxSTC_ERLANG_RECORD_QUOTED: i32 = 20;
const wxSTC_ERLANG_NODE_NAME_QUOTED: i32 = 21;
const wxSTC_ERLANG_BIFS: i32 = 22;
const wxSTC_ERLANG_MODULES: i32 = 23;
const wxSTC_ERLANG_MODULES_ATT: i32 = 24;
const wxSTC_ERLANG_UNKNOWN: i32 = 31;
const wxSTC_MSSQL_DEFAULT: i32 = 0;
const wxSTC_MSSQL_COMMENT: i32 = 1;
const wxSTC_MSSQL_LINE_COMMENT: i32 = 2;
const wxSTC_MSSQL_NUMBER: i32 = 3;
const wxSTC_MSSQL_STRING: i32 = 4;
const wxSTC_MSSQL_OPERATOR: i32 = 5;
const wxSTC_MSSQL_IDENTIFIER: i32 = 6;
const wxSTC_MSSQL_VARIABLE: i32 = 7;
const wxSTC_MSSQL_COLUMN_NAME: i32 = 8;
const wxSTC_MSSQL_STATEMENT: i32 = 9;
const wxSTC_MSSQL_DATATYPE: i32 = 10;
const wxSTC_MSSQL_SYSTABLE: i32 = 11;
const wxSTC_MSSQL_GLOBAL_VARIABLE: i32 = 12;
const wxSTC_MSSQL_FUNCTION: i32 = 13;
const wxSTC_MSSQL_STORED_PROCEDURE: i32 = 14;
const wxSTC_MSSQL_DEFAULT_PREF_DATATYPE: i32 = 15;
const wxSTC_MSSQL_COLUMN_NAME_2: i32 = 16;
const wxSTC_V_DEFAULT: i32 = 0;
const wxSTC_V_COMMENT: i32 = 1;
const wxSTC_V_COMMENTLINE: i32 = 2;
const wxSTC_V_COMMENTLINEBANG: i32 = 3;
const wxSTC_V_NUMBER: i32 = 4;
const wxSTC_V_WORD: i32 = 5;
const wxSTC_V_STRING: i32 = 6;
const wxSTC_V_WORD2: i32 = 7;
const wxSTC_V_WORD3: i32 = 8;
const wxSTC_V_PREPROCESSOR: i32 = 9;
const wxSTC_V_OPERATOR: i32 = 10;
const wxSTC_V_IDENTIFIER: i32 = 11;
const wxSTC_V_STRINGEOL: i32 = 12;
const wxSTC_V_USER: i32 = 19;
const wxSTC_V_COMMENT_WORD: i32 = 20;
const wxSTC_V_INPUT: i32 = 21;
const wxSTC_V_OUTPUT: i32 = 22;
const wxSTC_V_INOUT: i32 = 23;
const wxSTC_V_PORT_CONNECT: i32 = 24;
const wxSTC_KIX_DEFAULT: i32 = 0;
const wxSTC_KIX_COMMENT: i32 = 1;
const wxSTC_KIX_STRING1: i32 = 2;
const wxSTC_KIX_STRING2: i32 = 3;
const wxSTC_KIX_NUMBER: i32 = 4;
const wxSTC_KIX_VAR: i32 = 5;
const wxSTC_KIX_MACRO: i32 = 6;
const wxSTC_KIX_KEYWORD: i32 = 7;
const wxSTC_KIX_FUNCTIONS: i32 = 8;
const wxSTC_KIX_OPERATOR: i32 = 9;
const wxSTC_KIX_COMMENTSTREAM: i32 = 10;
const wxSTC_KIX_IDENTIFIER: i32 = 31;
const wxSTC_GC_DEFAULT: i32 = 0;
const wxSTC_GC_COMMENTLINE: i32 = 1;
const wxSTC_GC_COMMENTBLOCK: i32 = 2;
const wxSTC_GC_GLOBAL: i32 = 3;
const wxSTC_GC_EVENT: i32 = 4;
const wxSTC_GC_ATTRIBUTE: i32 = 5;
const wxSTC_GC_CONTROL: i32 = 6;
const wxSTC_GC_COMMAND: i32 = 7;
const wxSTC_GC_STRING: i32 = 8;
const wxSTC_GC_OPERATOR: i32 = 9;
const wxSTC_SN_DEFAULT: i32 = 0;
const wxSTC_SN_CODE: i32 = 1;
const wxSTC_SN_COMMENTLINE: i32 = 2;
const wxSTC_SN_COMMENTLINEBANG: i32 = 3;
const wxSTC_SN_NUMBER: i32 = 4;
const wxSTC_SN_WORD: i32 = 5;
const wxSTC_SN_STRING: i32 = 6;
const wxSTC_SN_WORD2: i32 = 7;
const wxSTC_SN_WORD3: i32 = 8;
const wxSTC_SN_PREPROCESSOR: i32 = 9;
const wxSTC_SN_OPERATOR: i32 = 10;
const wxSTC_SN_IDENTIFIER: i32 = 11;
const wxSTC_SN_STRINGEOL: i32 = 12;
const wxSTC_SN_REGEXTAG: i32 = 13;
const wxSTC_SN_SIGNAL: i32 = 14;
const wxSTC_SN_USER: i32 = 19;
const wxSTC_AU3_DEFAULT: i32 = 0;
const wxSTC_AU3_COMMENT: i32 = 1;
const wxSTC_AU3_COMMENTBLOCK: i32 = 2;
const wxSTC_AU3_NUMBER: i32 = 3;
const wxSTC_AU3_FUNCTION: i32 = 4;
const wxSTC_AU3_KEYWORD: i32 = 5;
const wxSTC_AU3_MACRO: i32 = 6;
const wxSTC_AU3_STRING: i32 = 7;
const wxSTC_AU3_OPERATOR: i32 = 8;
const wxSTC_AU3_VARIABLE: i32 = 9;
const wxSTC_AU3_SENT: i32 = 10;
const wxSTC_AU3_PREPROCESSOR: i32 = 11;
const wxSTC_AU3_SPECIAL: i32 = 12;
const wxSTC_AU3_EXPAND: i32 = 13;
const wxSTC_AU3_COMOBJ: i32 = 14;
const wxSTC_AU3_UDF: i32 = 15;
const wxSTC_APDL_DEFAULT: i32 = 0;
const wxSTC_APDL_COMMENT: i32 = 1;
const wxSTC_APDL_COMMENTBLOCK: i32 = 2;
const wxSTC_APDL_NUMBER: i32 = 3;
const wxSTC_APDL_STRING: i32 = 4;
const wxSTC_APDL_OPERATOR: i32 = 5;
const wxSTC_APDL_WORD: i32 = 6;
const wxSTC_APDL_PROCESSOR: i32 = 7;
const wxSTC_APDL_COMMAND: i32 = 8;
const wxSTC_APDL_SLASHCOMMAND: i32 = 9;
const wxSTC_APDL_STARCOMMAND: i32 = 10;
const wxSTC_APDL_ARGUMENT: i32 = 11;
const wxSTC_APDL_FUNCTION: i32 = 12;
const wxSTC_SH_DEFAULT: i32 = 0;
const wxSTC_SH_ERROR: i32 = 1;
const wxSTC_SH_COMMENTLINE: i32 = 2;
const wxSTC_SH_NUMBER: i32 = 3;
const wxSTC_SH_WORD: i32 = 4;
const wxSTC_SH_STRING: i32 = 5;
const wxSTC_SH_CHARACTER: i32 = 6;
const wxSTC_SH_OPERATOR: i32 = 7;
const wxSTC_SH_IDENTIFIER: i32 = 8;
const wxSTC_SH_SCALAR: i32 = 9;
const wxSTC_SH_PARAM: i32 = 10;
const wxSTC_SH_BACKTICKS: i32 = 11;
const wxSTC_SH_HERE_DELIM: i32 = 12;
const wxSTC_SH_HERE_Q: i32 = 13;
const wxSTC_ASN1_DEFAULT: i32 = 0;
const wxSTC_ASN1_COMMENT: i32 = 1;
const wxSTC_ASN1_IDENTIFIER: i32 = 2;
const wxSTC_ASN1_STRING: i32 = 3;
const wxSTC_ASN1_OID: i32 = 4;
const wxSTC_ASN1_SCALAR: i32 = 5;
const wxSTC_ASN1_KEYWORD: i32 = 6;
const wxSTC_ASN1_ATTRIBUTE: i32 = 7;
const wxSTC_ASN1_DESCRIPTOR: i32 = 8;
const wxSTC_ASN1_TYPE: i32 = 9;
const wxSTC_ASN1_OPERATOR: i32 = 10;
const wxSTC_VHDL_DEFAULT: i32 = 0;
const wxSTC_VHDL_COMMENT: i32 = 1;
const wxSTC_VHDL_COMMENTLINEBANG: i32 = 2;
const wxSTC_VHDL_NUMBER: i32 = 3;
const wxSTC_VHDL_STRING: i32 = 4;
const wxSTC_VHDL_OPERATOR: i32 = 5;
const wxSTC_VHDL_IDENTIFIER: i32 = 6;
const wxSTC_VHDL_STRINGEOL: i32 = 7;
const wxSTC_VHDL_KEYWORD: i32 = 8;
const wxSTC_VHDL_STDOPERATOR: i32 = 9;
const wxSTC_VHDL_ATTRIBUTE: i32 = 10;
const wxSTC_VHDL_STDFUNCTION: i32 = 11;
const wxSTC_VHDL_STDPACKAGE: i32 = 12;
const wxSTC_VHDL_STDTYPE: i32 = 13;
const wxSTC_VHDL_USERWORD: i32 = 14;
const wxSTC_VHDL_BLOCK_COMMENT: i32 = 15;
const wxSTC_CAML_DEFAULT: i32 = 0;
const wxSTC_CAML_IDENTIFIER: i32 = 1;
const wxSTC_CAML_TAGNAME: i32 = 2;
const wxSTC_CAML_KEYWORD: i32 = 3;
const wxSTC_CAML_KEYWORD2: i32 = 4;
const wxSTC_CAML_KEYWORD3: i32 = 5;
const wxSTC_CAML_LINENUM: i32 = 6;
const wxSTC_CAML_OPERATOR: i32 = 7;
const wxSTC_CAML_NUMBER: i32 = 8;
const wxSTC_CAML_CHAR: i32 = 9;
const wxSTC_CAML_WHITE: i32 = 10;
const wxSTC_CAML_STRING: i32 = 11;
const wxSTC_CAML_COMMENT: i32 = 12;
const wxSTC_CAML_COMMENT1: i32 = 13;
const wxSTC_CAML_COMMENT2: i32 = 14;
const wxSTC_CAML_COMMENT3: i32 = 15;
const wxSTC_HA_DEFAULT: i32 = 0;
const wxSTC_HA_IDENTIFIER: i32 = 1;
const wxSTC_HA_KEYWORD: i32 = 2;
const wxSTC_HA_NUMBER: i32 = 3;
const wxSTC_HA_STRING: i32 = 4;
const wxSTC_HA_CHARACTER: i32 = 5;
const wxSTC_HA_CLASS: i32 = 6;
const wxSTC_HA_MODULE: i32 = 7;
const wxSTC_HA_CAPITAL: i32 = 8;
const wxSTC_HA_DATA: i32 = 9;
const wxSTC_HA_IMPORT: i32 = 10;
const wxSTC_HA_OPERATOR: i32 = 11;
const wxSTC_HA_INSTANCE: i32 = 12;
const wxSTC_HA_COMMENTLINE: i32 = 13;
const wxSTC_HA_COMMENTBLOCK: i32 = 14;
const wxSTC_HA_COMMENTBLOCK2: i32 = 15;
const wxSTC_HA_COMMENTBLOCK3: i32 = 16;
const wxSTC_HA_PRAGMA: i32 = 17;
const wxSTC_HA_PREPROCESSOR: i32 = 18;
const wxSTC_HA_STRINGEOL: i32 = 19;
const wxSTC_HA_RESERVED_OPERATOR: i32 = 20;
const wxSTC_HA_LITERATE_COMMENT: i32 = 21;
const wxSTC_HA_LITERATE_CODEDELIM: i32 = 22;
const wxSTC_T3_DEFAULT: i32 = 0;
const wxSTC_T3_X_DEFAULT: i32 = 1;
const wxSTC_T3_PREPROCESSOR: i32 = 2;
const wxSTC_T3_BLOCK_COMMENT: i32 = 3;
const wxSTC_T3_LINE_COMMENT: i32 = 4;
const wxSTC_T3_OPERATOR: i32 = 5;
const wxSTC_T3_KEYWORD: i32 = 6;
const wxSTC_T3_NUMBER: i32 = 7;
const wxSTC_T3_IDENTIFIER: i32 = 8;
const wxSTC_T3_S_STRING: i32 = 9;
const wxSTC_T3_D_STRING: i32 = 10;
const wxSTC_T3_X_STRING: i32 = 11;
const wxSTC_T3_LIB_DIRECTIVE: i32 = 12;
const wxSTC_T3_MSG_PARAM: i32 = 13;
const wxSTC_T3_HTML_TAG: i32 = 14;
const wxSTC_T3_HTML_DEFAULT: i32 = 15;
const wxSTC_T3_HTML_STRING: i32 = 16;
const wxSTC_T3_USER1: i32 = 17;
const wxSTC_T3_USER2: i32 = 18;
const wxSTC_T3_USER3: i32 = 19;
const wxSTC_T3_BRACE: i32 = 20;
const wxSTC_REBOL_DEFAULT: i32 = 0;
const wxSTC_REBOL_COMMENTLINE: i32 = 1;
const wxSTC_REBOL_COMMENTBLOCK: i32 = 2;
const wxSTC_REBOL_PREFACE: i32 = 3;
const wxSTC_REBOL_OPERATOR: i32 = 4;
const wxSTC_REBOL_CHARACTER: i32 = 5;
const wxSTC_REBOL_QUOTEDSTRING: i32 = 6;
const wxSTC_REBOL_BRACEDSTRING: i32 = 7;
const wxSTC_REBOL_NUMBER: i32 = 8;
const wxSTC_REBOL_PAIR: i32 = 9;
const wxSTC_REBOL_TUPLE: i32 = 10;
const wxSTC_REBOL_BINARY: i32 = 11;
const wxSTC_REBOL_MONEY: i32 = 12;
const wxSTC_REBOL_ISSUE: i32 = 13;
const wxSTC_REBOL_TAG: i32 = 14;
const wxSTC_REBOL_FILE: i32 = 15;
const wxSTC_REBOL_EMAIL: i32 = 16;
const wxSTC_REBOL_URL: i32 = 17;
const wxSTC_REBOL_DATE: i32 = 18;
const wxSTC_REBOL_TIME: i32 = 19;
const wxSTC_REBOL_IDENTIFIER: i32 = 20;
const wxSTC_REBOL_WORD: i32 = 21;
const wxSTC_REBOL_WORD2: i32 = 22;
const wxSTC_REBOL_WORD3: i32 = 23;
const wxSTC_REBOL_WORD4: i32 = 24;
const wxSTC_REBOL_WORD5: i32 = 25;
const wxSTC_REBOL_WORD6: i32 = 26;
const wxSTC_REBOL_WORD7: i32 = 27;
const wxSTC_REBOL_WORD8: i32 = 28;
const wxSTC_SQL_DEFAULT: i32 = 0;
const wxSTC_SQL_COMMENT: i32 = 1;
const wxSTC_SQL_COMMENTLINE: i32 = 2;
const wxSTC_SQL_COMMENTDOC: i32 = 3;
const wxSTC_SQL_NUMBER: i32 = 4;
const wxSTC_SQL_WORD: i32 = 5;
const wxSTC_SQL_STRING: i32 = 6;
const wxSTC_SQL_CHARACTER: i32 = 7;
const wxSTC_SQL_SQLPLUS: i32 = 8;
const wxSTC_SQL_SQLPLUS_PROMPT: i32 = 9;
const wxSTC_SQL_OPERATOR: i32 = 10;
const wxSTC_SQL_IDENTIFIER: i32 = 11;
const wxSTC_SQL_SQLPLUS_COMMENT: i32 = 13;
const wxSTC_SQL_COMMENTLINEDOC: i32 = 15;
const wxSTC_SQL_WORD2: i32 = 16;
const wxSTC_SQL_COMMENTDOCKEYWORD: i32 = 17;
const wxSTC_SQL_COMMENTDOCKEYWORDERROR: i32 = 18;
const wxSTC_SQL_USER1: i32 = 19;
const wxSTC_SQL_USER2: i32 = 20;
const wxSTC_SQL_USER3: i32 = 21;
const wxSTC_SQL_USER4: i32 = 22;
const wxSTC_SQL_QUOTEDIDENTIFIER: i32 = 23;
const wxSTC_SQL_QOPERATOR: i32 = 24;
const wxSTC_ST_DEFAULT: i32 = 0;
const wxSTC_ST_STRING: i32 = 1;
const wxSTC_ST_NUMBER: i32 = 2;
const wxSTC_ST_COMMENT: i32 = 3;
const wxSTC_ST_SYMBOL: i32 = 4;
const wxSTC_ST_BINARY: i32 = 5;
const wxSTC_ST_BOOL: i32 = 6;
const wxSTC_ST_SELF: i32 = 7;
const wxSTC_ST_SUPER: i32 = 8;
const wxSTC_ST_NIL: i32 = 9;
const wxSTC_ST_GLOBAL: i32 = 10;
const wxSTC_ST_RETURN: i32 = 11;
const wxSTC_ST_SPECIAL: i32 = 12;
const wxSTC_ST_KWSEND: i32 = 13;
const wxSTC_ST_ASSIGN: i32 = 14;
const wxSTC_ST_CHARACTER: i32 = 15;
const wxSTC_ST_SPEC_SEL: i32 = 16;
const wxSTC_FS_DEFAULT: i32 = 0;
const wxSTC_FS_COMMENT: i32 = 1;
const wxSTC_FS_COMMENTLINE: i32 = 2;
const wxSTC_FS_COMMENTDOC: i32 = 3;
const wxSTC_FS_COMMENTLINEDOC: i32 = 4;
const wxSTC_FS_COMMENTDOCKEYWORD: i32 = 5;
const wxSTC_FS_COMMENTDOCKEYWORDERROR: i32 = 6;
const wxSTC_FS_KEYWORD: i32 = 7;
const wxSTC_FS_KEYWORD2: i32 = 8;
const wxSTC_FS_KEYWORD3: i32 = 9;
const wxSTC_FS_KEYWORD4: i32 = 10;
const wxSTC_FS_NUMBER: i32 = 11;
const wxSTC_FS_STRING: i32 = 12;
const wxSTC_FS_PREPROCESSOR: i32 = 13;
const wxSTC_FS_OPERATOR: i32 = 14;
const wxSTC_FS_IDENTIFIER: i32 = 15;
const wxSTC_FS_DATE: i32 = 16;
const wxSTC_FS_STRINGEOL: i32 = 17;
const wxSTC_FS_CONSTANT: i32 = 18;
const wxSTC_FS_WORDOPERATOR: i32 = 19;
const wxSTC_FS_DISABLEDCODE: i32 = 20;
const wxSTC_FS_DEFAULT_C: i32 = 21;
const wxSTC_FS_COMMENTDOC_C: i32 = 22;
const wxSTC_FS_COMMENTLINEDOC_C: i32 = 23;
const wxSTC_FS_KEYWORD_C: i32 = 24;
const wxSTC_FS_KEYWORD2_C: i32 = 25;
const wxSTC_FS_NUMBER_C: i32 = 26;
const wxSTC_FS_STRING_C: i32 = 27;
const wxSTC_FS_PREPROCESSOR_C: i32 = 28;
const wxSTC_FS_OPERATOR_C: i32 = 29;
const wxSTC_FS_IDENTIFIER_C: i32 = 30;
const wxSTC_FS_STRINGEOL_C: i32 = 31;
const wxSTC_CSOUND_DEFAULT: i32 = 0;
const wxSTC_CSOUND_COMMENT: i32 = 1;
const wxSTC_CSOUND_NUMBER: i32 = 2;
const wxSTC_CSOUND_OPERATOR: i32 = 3;
const wxSTC_CSOUND_INSTR: i32 = 4;
const wxSTC_CSOUND_IDENTIFIER: i32 = 5;
const wxSTC_CSOUND_OPCODE: i32 = 6;
const wxSTC_CSOUND_HEADERSTMT: i32 = 7;
const wxSTC_CSOUND_USERKEYWORD: i32 = 8;
const wxSTC_CSOUND_COMMENTBLOCK: i32 = 9;
const wxSTC_CSOUND_PARAM: i32 = 10;
const wxSTC_CSOUND_ARATE_VAR: i32 = 11;
const wxSTC_CSOUND_KRATE_VAR: i32 = 12;
const wxSTC_CSOUND_IRATE_VAR: i32 = 13;
const wxSTC_CSOUND_GLOBAL_VAR: i32 = 14;
const wxSTC_CSOUND_STRINGEOL: i32 = 15;
const wxSTC_INNO_DEFAULT: i32 = 0;
const wxSTC_INNO_COMMENT: i32 = 1;
const wxSTC_INNO_KEYWORD: i32 = 2;
const wxSTC_INNO_PARAMETER: i32 = 3;
const wxSTC_INNO_SECTION: i32 = 4;
const wxSTC_INNO_PREPROC: i32 = 5;
const wxSTC_INNO_INLINE_EXPANSION: i32 = 6;
const wxSTC_INNO_COMMENT_PASCAL: i32 = 7;
const wxSTC_INNO_KEYWORD_PASCAL: i32 = 8;
const wxSTC_INNO_KEYWORD_USER: i32 = 9;
const wxSTC_INNO_STRING_DOUBLE: i32 = 10;
const wxSTC_INNO_STRING_SINGLE: i32 = 11;
const wxSTC_INNO_IDENTIFIER: i32 = 12;
const wxSTC_OPAL_SPACE: i32 = 0;
const wxSTC_OPAL_COMMENT_BLOCK: i32 = 1;
const wxSTC_OPAL_COMMENT_LINE: i32 = 2;
const wxSTC_OPAL_INTEGER: i32 = 3;
const wxSTC_OPAL_KEYWORD: i32 = 4;
const wxSTC_OPAL_SORT: i32 = 5;
const wxSTC_OPAL_STRING: i32 = 6;
const wxSTC_OPAL_PAR: i32 = 7;
const wxSTC_OPAL_BOOL_CONST: i32 = 8;
const wxSTC_OPAL_DEFAULT: i32 = 32;
const wxSTC_SPICE_DEFAULT: i32 = 0;
const wxSTC_SPICE_IDENTIFIER: i32 = 1;
const wxSTC_SPICE_KEYWORD: i32 = 2;
const wxSTC_SPICE_KEYWORD2: i32 = 3;
const wxSTC_SPICE_KEYWORD3: i32 = 4;
const wxSTC_SPICE_NUMBER: i32 = 5;
const wxSTC_SPICE_DELIMITER: i32 = 6;
const wxSTC_SPICE_VALUE: i32 = 7;
const wxSTC_SPICE_COMMENTLINE: i32 = 8;
const wxSTC_CMAKE_DEFAULT: i32 = 0;
const wxSTC_CMAKE_COMMENT: i32 = 1;
const wxSTC_CMAKE_STRINGDQ: i32 = 2;
const wxSTC_CMAKE_STRINGLQ: i32 = 3;
const wxSTC_CMAKE_STRINGRQ: i32 = 4;
const wxSTC_CMAKE_COMMANDS: i32 = 5;
const wxSTC_CMAKE_PARAMETERS: i32 = 6;
const wxSTC_CMAKE_VARIABLE: i32 = 7;
const wxSTC_CMAKE_USERDEFINED: i32 = 8;
const wxSTC_CMAKE_WHILEDEF: i32 = 9;
const wxSTC_CMAKE_FOREACHDEF: i32 = 10;
const wxSTC_CMAKE_IFDEFINEDEF: i32 = 11;
const wxSTC_CMAKE_MACRODEF: i32 = 12;
const wxSTC_CMAKE_STRINGVAR: i32 = 13;
const wxSTC_CMAKE_NUMBER: i32 = 14;
const wxSTC_GAP_DEFAULT: i32 = 0;
const wxSTC_GAP_IDENTIFIER: i32 = 1;
const wxSTC_GAP_KEYWORD: i32 = 2;
const wxSTC_GAP_KEYWORD2: i32 = 3;
const wxSTC_GAP_KEYWORD3: i32 = 4;
const wxSTC_GAP_KEYWORD4: i32 = 5;
const wxSTC_GAP_STRING: i32 = 6;
const wxSTC_GAP_CHAR: i32 = 7;
const wxSTC_GAP_OPERATOR: i32 = 8;
const wxSTC_GAP_COMMENT: i32 = 9;
const wxSTC_GAP_NUMBER: i32 = 10;
const wxSTC_GAP_STRINGEOL: i32 = 11;
const wxSTC_PLM_DEFAULT: i32 = 0;
const wxSTC_PLM_COMMENT: i32 = 1;
const wxSTC_PLM_STRING: i32 = 2;
const wxSTC_PLM_NUMBER: i32 = 3;
const wxSTC_PLM_IDENTIFIER: i32 = 4;
const wxSTC_PLM_OPERATOR: i32 = 5;
const wxSTC_PLM_CONTROL: i32 = 6;
const wxSTC_PLM_KEYWORD: i32 = 7;
const wxSTC_ABL_DEFAULT: i32 = 0;
const wxSTC_ABL_NUMBER: i32 = 1;
const wxSTC_ABL_WORD: i32 = 2;
const wxSTC_ABL_STRING: i32 = 3;
const wxSTC_ABL_CHARACTER: i32 = 4;
const wxSTC_ABL_PREPROCESSOR: i32 = 5;
const wxSTC_ABL_OPERATOR: i32 = 6;
const wxSTC_ABL_IDENTIFIER: i32 = 7;
const wxSTC_ABL_BLOCK: i32 = 8;
const wxSTC_ABL_END: i32 = 9;
const wxSTC_ABL_COMMENT: i32 = 10;
const wxSTC_ABL_TASKMARKER: i32 = 11;
const wxSTC_ABL_LINECOMMENT: i32 = 12;
const wxSTC_ABAQUS_DEFAULT: i32 = 0;
const wxSTC_ABAQUS_COMMENT: i32 = 1;
const wxSTC_ABAQUS_COMMENTBLOCK: i32 = 2;
const wxSTC_ABAQUS_NUMBER: i32 = 3;
const wxSTC_ABAQUS_STRING: i32 = 4;
const wxSTC_ABAQUS_OPERATOR: i32 = 5;
const wxSTC_ABAQUS_WORD: i32 = 6;
const wxSTC_ABAQUS_PROCESSOR: i32 = 7;
const wxSTC_ABAQUS_COMMAND: i32 = 8;
const wxSTC_ABAQUS_SLASHCOMMAND: i32 = 9;
const wxSTC_ABAQUS_STARCOMMAND: i32 = 10;
const wxSTC_ABAQUS_ARGUMENT: i32 = 11;
const wxSTC_ABAQUS_FUNCTION: i32 = 12;
const wxSTC_ASY_DEFAULT: i32 = 0;
const wxSTC_ASY_COMMENT: i32 = 1;
const wxSTC_ASY_COMMENTLINE: i32 = 2;
const wxSTC_ASY_NUMBER: i32 = 3;
const wxSTC_ASY_WORD: i32 = 4;
const wxSTC_ASY_STRING: i32 = 5;
const wxSTC_ASY_CHARACTER: i32 = 6;
const wxSTC_ASY_OPERATOR: i32 = 7;
const wxSTC_ASY_IDENTIFIER: i32 = 8;
const wxSTC_ASY_STRINGEOL: i32 = 9;
const wxSTC_ASY_COMMENTLINEDOC: i32 = 10;
const wxSTC_ASY_WORD2: i32 = 11;
const wxSTC_R_DEFAULT: i32 = 0;
const wxSTC_R_COMMENT: i32 = 1;
const wxSTC_R_KWORD: i32 = 2;
const wxSTC_R_BASEKWORD: i32 = 3;
const wxSTC_R_OTHERKWORD: i32 = 4;
const wxSTC_R_NUMBER: i32 = 5;
const wxSTC_R_STRING: i32 = 6;
const wxSTC_R_STRING2: i32 = 7;
const wxSTC_R_OPERATOR: i32 = 8;
const wxSTC_R_IDENTIFIER: i32 = 9;
const wxSTC_R_INFIX: i32 = 10;
const wxSTC_R_INFIXEOL: i32 = 11;
const wxSTC_MAGIK_DEFAULT: i32 = 0;
const wxSTC_MAGIK_COMMENT: i32 = 1;
const wxSTC_MAGIK_HYPER_COMMENT: i32 = 16;
const wxSTC_MAGIK_STRING: i32 = 2;
const wxSTC_MAGIK_CHARACTER: i32 = 3;
const wxSTC_MAGIK_NUMBER: i32 = 4;
const wxSTC_MAGIK_IDENTIFIER: i32 = 5;
const wxSTC_MAGIK_OPERATOR: i32 = 6;
const wxSTC_MAGIK_FLOW: i32 = 7;
const wxSTC_MAGIK_CONTAINER: i32 = 8;
const wxSTC_MAGIK_BRACKET_BLOCK: i32 = 9;
const wxSTC_MAGIK_BRACE_BLOCK: i32 = 10;
const wxSTC_MAGIK_SQBRACKET_BLOCK: i32 = 11;
const wxSTC_MAGIK_UNKNOWN_KEYWORD: i32 = 12;
const wxSTC_MAGIK_KEYWORD: i32 = 13;
const wxSTC_MAGIK_PRAGMA: i32 = 14;
const wxSTC_MAGIK_SYMBOL: i32 = 15;
const wxSTC_POWERSHELL_DEFAULT: i32 = 0;
const wxSTC_POWERSHELL_COMMENT: i32 = 1;
const wxSTC_POWERSHELL_STRING: i32 = 2;
const wxSTC_POWERSHELL_CHARACTER: i32 = 3;
const wxSTC_POWERSHELL_NUMBER: i32 = 4;
const wxSTC_POWERSHELL_VARIABLE: i32 = 5;
const wxSTC_POWERSHELL_OPERATOR: i32 = 6;
const wxSTC_POWERSHELL_IDENTIFIER: i32 = 7;
const wxSTC_POWERSHELL_KEYWORD: i32 = 8;
const wxSTC_POWERSHELL_CMDLET: i32 = 9;
const wxSTC_POWERSHELL_ALIAS: i32 = 10;
const wxSTC_POWERSHELL_FUNCTION: i32 = 11;
const wxSTC_POWERSHELL_USER1: i32 = 12;
const wxSTC_POWERSHELL_COMMENTSTREAM: i32 = 13;
const wxSTC_POWERSHELL_HERE_STRING: i32 = 14;
const wxSTC_POWERSHELL_HERE_CHARACTER: i32 = 15;
const wxSTC_POWERSHELL_COMMENTDOCKEYWORD: i32 = 16;
const wxSTC_MYSQL_DEFAULT: i32 = 0;
const wxSTC_MYSQL_COMMENT: i32 = 1;
const wxSTC_MYSQL_COMMENTLINE: i32 = 2;
const wxSTC_MYSQL_VARIABLE: i32 = 3;
const wxSTC_MYSQL_SYSTEMVARIABLE: i32 = 4;
const wxSTC_MYSQL_KNOWNSYSTEMVARIABLE: i32 = 5;
const wxSTC_MYSQL_NUMBER: i32 = 6;
const wxSTC_MYSQL_MAJORKEYWORD: i32 = 7;
const wxSTC_MYSQL_KEYWORD: i32 = 8;
const wxSTC_MYSQL_DATABASEOBJECT: i32 = 9;
const wxSTC_MYSQL_PROCEDUREKEYWORD: i32 = 10;
const wxSTC_MYSQL_STRING: i32 = 11;
const wxSTC_MYSQL_SQSTRING: i32 = 12;
const wxSTC_MYSQL_DQSTRING: i32 = 13;
const wxSTC_MYSQL_OPERATOR: i32 = 14;
const wxSTC_MYSQL_FUNCTION: i32 = 15;
const wxSTC_MYSQL_IDENTIFIER: i32 = 16;
const wxSTC_MYSQL_QUOTEDIDENTIFIER: i32 = 17;
const wxSTC_MYSQL_USER1: i32 = 18;
const wxSTC_MYSQL_USER2: i32 = 19;
const wxSTC_MYSQL_USER3: i32 = 20;
const wxSTC_MYSQL_HIDDENCOMMAND: i32 = 21;
const wxSTC_MYSQL_PLACEHOLDER: i32 = 22;
const wxSTC_PO_DEFAULT: i32 = 0;
const wxSTC_PO_COMMENT: i32 = 1;
const wxSTC_PO_MSGID: i32 = 2;
const wxSTC_PO_MSGID_TEXT: i32 = 3;
const wxSTC_PO_MSGSTR: i32 = 4;
const wxSTC_PO_MSGSTR_TEXT: i32 = 5;
const wxSTC_PO_MSGCTXT: i32 = 6;
const wxSTC_PO_MSGCTXT_TEXT: i32 = 7;
const wxSTC_PO_FUZZY: i32 = 8;
const wxSTC_PO_PROGRAMMER_COMMENT: i32 = 9;
const wxSTC_PO_REFERENCE: i32 = 10;
const wxSTC_PO_FLAGS: i32 = 11;
const wxSTC_PO_MSGID_TEXT_EOL: i32 = 12;
const wxSTC_PO_MSGSTR_TEXT_EOL: i32 = 13;
const wxSTC_PO_MSGCTXT_TEXT_EOL: i32 = 14;
const wxSTC_PO_ERROR: i32 = 15;
const wxSTC_PAS_DEFAULT: i32 = 0;
const wxSTC_PAS_IDENTIFIER: i32 = 1;
const wxSTC_PAS_COMMENT: i32 = 2;
const wxSTC_PAS_COMMENT2: i32 = 3;
const wxSTC_PAS_COMMENTLINE: i32 = 4;
const wxSTC_PAS_PREPROCESSOR: i32 = 5;
const wxSTC_PAS_PREPROCESSOR2: i32 = 6;
const wxSTC_PAS_NUMBER: i32 = 7;
const wxSTC_PAS_HEXNUMBER: i32 = 8;
const wxSTC_PAS_WORD: i32 = 9;
const wxSTC_PAS_STRING: i32 = 10;
const wxSTC_PAS_STRINGEOL: i32 = 11;
const wxSTC_PAS_CHARACTER: i32 = 12;
const wxSTC_PAS_OPERATOR: i32 = 13;
const wxSTC_PAS_ASM: i32 = 14;
const wxSTC_SORCUS_DEFAULT: i32 = 0;
const wxSTC_SORCUS_COMMAND: i32 = 1;
const wxSTC_SORCUS_PARAMETER: i32 = 2;
const wxSTC_SORCUS_COMMENTLINE: i32 = 3;
const wxSTC_SORCUS_STRING: i32 = 4;
const wxSTC_SORCUS_STRINGEOL: i32 = 5;
const wxSTC_SORCUS_IDENTIFIER: i32 = 6;
const wxSTC_SORCUS_OPERATOR: i32 = 7;
const wxSTC_SORCUS_NUMBER: i32 = 8;
const wxSTC_SORCUS_CONSTANT: i32 = 9;
const wxSTC_POWERPRO_DEFAULT: i32 = 0;
const wxSTC_POWERPRO_COMMENTBLOCK: i32 = 1;
const wxSTC_POWERPRO_COMMENTLINE: i32 = 2;
const wxSTC_POWERPRO_NUMBER: i32 = 3;
const wxSTC_POWERPRO_WORD: i32 = 4;
const wxSTC_POWERPRO_WORD2: i32 = 5;
const wxSTC_POWERPRO_WORD3: i32 = 6;
const wxSTC_POWERPRO_WORD4: i32 = 7;
const wxSTC_POWERPRO_DOUBLEQUOTEDSTRING: i32 = 8;
const wxSTC_POWERPRO_SINGLEQUOTEDSTRING: i32 = 9;
const wxSTC_POWERPRO_LINECONTINUE: i32 = 10;
const wxSTC_POWERPRO_OPERATOR: i32 = 11;
const wxSTC_POWERPRO_IDENTIFIER: i32 = 12;
const wxSTC_POWERPRO_STRINGEOL: i32 = 13;
const wxSTC_POWERPRO_VERBATIM: i32 = 14;
const wxSTC_POWERPRO_ALTQUOTE: i32 = 15;
const wxSTC_POWERPRO_FUNCTION: i32 = 16;
const wxSTC_SML_DEFAULT: i32 = 0;
const wxSTC_SML_IDENTIFIER: i32 = 1;
const wxSTC_SML_TAGNAME: i32 = 2;
const wxSTC_SML_KEYWORD: i32 = 3;
const wxSTC_SML_KEYWORD2: i32 = 4;
const wxSTC_SML_KEYWORD3: i32 = 5;
const wxSTC_SML_LINENUM: i32 = 6;
const wxSTC_SML_OPERATOR: i32 = 7;
const wxSTC_SML_NUMBER: i32 = 8;
const wxSTC_SML_CHAR: i32 = 9;
const wxSTC_SML_STRING: i32 = 11;
const wxSTC_SML_COMMENT: i32 = 12;
const wxSTC_SML_COMMENT1: i32 = 13;
const wxSTC_SML_COMMENT2: i32 = 14;
const wxSTC_SML_COMMENT3: i32 = 15;
const wxSTC_MARKDOWN_DEFAULT: i32 = 0;
const wxSTC_MARKDOWN_LINE_BEGIN: i32 = 1;
const wxSTC_MARKDOWN_STRONG1: i32 = 2;
const wxSTC_MARKDOWN_STRONG2: i32 = 3;
const wxSTC_MARKDOWN_EM1: i32 = 4;
const wxSTC_MARKDOWN_EM2: i32 = 5;
const wxSTC_MARKDOWN_HEADER1: i32 = 6;
const wxSTC_MARKDOWN_HEADER2: i32 = 7;
const wxSTC_MARKDOWN_HEADER3: i32 = 8;
const wxSTC_MARKDOWN_HEADER4: i32 = 9;
const wxSTC_MARKDOWN_HEADER5: i32 = 10;
const wxSTC_MARKDOWN_HEADER6: i32 = 11;
const wxSTC_MARKDOWN_PRECHAR: i32 = 12;
const wxSTC_MARKDOWN_ULIST_ITEM: i32 = 13;
const wxSTC_MARKDOWN_OLIST_ITEM: i32 = 14;
const wxSTC_MARKDOWN_BLOCKQUOTE: i32 = 15;
const wxSTC_MARKDOWN_STRIKEOUT: i32 = 16;
const wxSTC_MARKDOWN_HRULE: i32 = 17;
const wxSTC_MARKDOWN_LINK: i32 = 18;
const wxSTC_MARKDOWN_CODE: i32 = 19;
const wxSTC_MARKDOWN_CODE2: i32 = 20;
const wxSTC_MARKDOWN_CODEBK: i32 = 21;
const wxSTC_TXT2TAGS_DEFAULT: i32 = 0;
const wxSTC_TXT2TAGS_LINE_BEGIN: i32 = 1;
const wxSTC_TXT2TAGS_STRONG1: i32 = 2;
const wxSTC_TXT2TAGS_STRONG2: i32 = 3;
const wxSTC_TXT2TAGS_EM1: i32 = 4;
const wxSTC_TXT2TAGS_EM2: i32 = 5;
const wxSTC_TXT2TAGS_HEADER1: i32 = 6;
const wxSTC_TXT2TAGS_HEADER2: i32 = 7;
const wxSTC_TXT2TAGS_HEADER3: i32 = 8;
const wxSTC_TXT2TAGS_HEADER4: i32 = 9;
const wxSTC_TXT2TAGS_HEADER5: i32 = 10;
const wxSTC_TXT2TAGS_HEADER6: i32 = 11;
const wxSTC_TXT2TAGS_PRECHAR: i32 = 12;
const wxSTC_TXT2TAGS_ULIST_ITEM: i32 = 13;
const wxSTC_TXT2TAGS_OLIST_ITEM: i32 = 14;
const wxSTC_TXT2TAGS_BLOCKQUOTE: i32 = 15;
const wxSTC_TXT2TAGS_STRIKEOUT: i32 = 16;
const wxSTC_TXT2TAGS_HRULE: i32 = 17;
const wxSTC_TXT2TAGS_LINK: i32 = 18;
const wxSTC_TXT2TAGS_CODE: i32 = 19;
const wxSTC_TXT2TAGS_CODE2: i32 = 20;
const wxSTC_TXT2TAGS_CODEBK: i32 = 21;
const wxSTC_TXT2TAGS_COMMENT: i32 = 22;
const wxSTC_TXT2TAGS_OPTION: i32 = 23;
const wxSTC_TXT2TAGS_PREPROC: i32 = 24;
const wxSTC_TXT2TAGS_POSTPROC: i32 = 25;
const wxSTC_A68K_DEFAULT: i32 = 0;
const wxSTC_A68K_COMMENT: i32 = 1;
const wxSTC_A68K_NUMBER_DEC: i32 = 2;
const wxSTC_A68K_NUMBER_BIN: i32 = 3;
const wxSTC_A68K_NUMBER_HEX: i32 = 4;
const wxSTC_A68K_STRING1: i32 = 5;
const wxSTC_A68K_OPERATOR: i32 = 6;
const wxSTC_A68K_CPUINSTRUCTION: i32 = 7;
const wxSTC_A68K_EXTINSTRUCTION: i32 = 8;
const wxSTC_A68K_REGISTER: i32 = 9;
const wxSTC_A68K_DIRECTIVE: i32 = 10;
const wxSTC_A68K_MACRO_ARG: i32 = 11;
const wxSTC_A68K_LABEL: i32 = 12;
const wxSTC_A68K_STRING2: i32 = 13;
const wxSTC_A68K_IDENTIFIER: i32 = 14;
const wxSTC_A68K_MACRO_DECLARATION: i32 = 15;
const wxSTC_A68K_COMMENT_WORD: i32 = 16;
const wxSTC_A68K_COMMENT_SPECIAL: i32 = 17;
const wxSTC_A68K_COMMENT_DOXYGEN: i32 = 18;
const wxSTC_MODULA_DEFAULT: i32 = 0;
const wxSTC_MODULA_COMMENT: i32 = 1;
const wxSTC_MODULA_DOXYCOMM: i32 = 2;
const wxSTC_MODULA_DOXYKEY: i32 = 3;
const wxSTC_MODULA_KEYWORD: i32 = 4;
const wxSTC_MODULA_RESERVED: i32 = 5;
const wxSTC_MODULA_NUMBER: i32 = 6;
const wxSTC_MODULA_BASENUM: i32 = 7;
const wxSTC_MODULA_FLOAT: i32 = 8;
const wxSTC_MODULA_STRING: i32 = 9;
const wxSTC_MODULA_STRSPEC: i32 = 10;
const wxSTC_MODULA_CHAR: i32 = 11;
const wxSTC_MODULA_CHARSPEC: i32 = 12;
const wxSTC_MODULA_PROC: i32 = 13;
const wxSTC_MODULA_PRAGMA: i32 = 14;
const wxSTC_MODULA_PRGKEY: i32 = 15;
const wxSTC_MODULA_OPERATOR: i32 = 16;
const wxSTC_MODULA_BADSTR: i32 = 17;
const wxSTC_COFFEESCRIPT_DEFAULT: i32 = 0;
const wxSTC_COFFEESCRIPT_COMMENT: i32 = 1;
const wxSTC_COFFEESCRIPT_COMMENTLINE: i32 = 2;
const wxSTC_COFFEESCRIPT_COMMENTDOC: i32 = 3;
const wxSTC_COFFEESCRIPT_NUMBER: i32 = 4;
const wxSTC_COFFEESCRIPT_WORD: i32 = 5;
const wxSTC_COFFEESCRIPT_STRING: i32 = 6;
const wxSTC_COFFEESCRIPT_CHARACTER: i32 = 7;
const wxSTC_COFFEESCRIPT_UUID: i32 = 8;
const wxSTC_COFFEESCRIPT_PREPROCESSOR: i32 = 9;
const wxSTC_COFFEESCRIPT_OPERATOR: i32 = 10;
const wxSTC_COFFEESCRIPT_IDENTIFIER: i32 = 11;
const wxSTC_COFFEESCRIPT_STRINGEOL: i32 = 12;
const wxSTC_COFFEESCRIPT_VERBATIM: i32 = 13;
const wxSTC_COFFEESCRIPT_REGEX: i32 = 14;
const wxSTC_COFFEESCRIPT_COMMENTLINEDOC: i32 = 15;
const wxSTC_COFFEESCRIPT_WORD2: i32 = 16;
const wxSTC_COFFEESCRIPT_COMMENTDOCKEYWORD: i32 = 17;
const wxSTC_COFFEESCRIPT_COMMENTDOCKEYWORDERROR: i32 = 18;
const wxSTC_COFFEESCRIPT_GLOBALCLASS: i32 = 19;
const wxSTC_COFFEESCRIPT_STRINGRAW: i32 = 20;
const wxSTC_COFFEESCRIPT_TRIPLEVERBATIM: i32 = 21;
const wxSTC_COFFEESCRIPT_COMMENTBLOCK: i32 = 22;
const wxSTC_COFFEESCRIPT_VERBOSE_REGEX: i32 = 23;
const wxSTC_COFFEESCRIPT_VERBOSE_REGEX_COMMENT: i32 = 24;
const wxSTC_COFFEESCRIPT_INSTANCEPROPERTY: i32 = 25;
const wxSTC_AVS_DEFAULT: i32 = 0;
const wxSTC_AVS_COMMENTBLOCK: i32 = 1;
const wxSTC_AVS_COMMENTBLOCKN: i32 = 2;
const wxSTC_AVS_COMMENTLINE: i32 = 3;
const wxSTC_AVS_NUMBER: i32 = 4;
const wxSTC_AVS_OPERATOR: i32 = 5;
const wxSTC_AVS_IDENTIFIER: i32 = 6;
const wxSTC_AVS_STRING: i32 = 7;
const wxSTC_AVS_TRIPLESTRING: i32 = 8;
const wxSTC_AVS_KEYWORD: i32 = 9;
const wxSTC_AVS_FILTER: i32 = 10;
const wxSTC_AVS_PLUGIN: i32 = 11;
const wxSTC_AVS_FUNCTION: i32 = 12;
const wxSTC_AVS_CLIPPROP: i32 = 13;
const wxSTC_AVS_USERDFN: i32 = 14;
const wxSTC_ECL_DEFAULT: i32 = 0;
const wxSTC_ECL_COMMENT: i32 = 1;
const wxSTC_ECL_COMMENTLINE: i32 = 2;
const wxSTC_ECL_NUMBER: i32 = 3;
const wxSTC_ECL_STRING: i32 = 4;
const wxSTC_ECL_WORD0: i32 = 5;
const wxSTC_ECL_OPERATOR: i32 = 6;
const wxSTC_ECL_CHARACTER: i32 = 7;
const wxSTC_ECL_UUID: i32 = 8;
const wxSTC_ECL_PREPROCESSOR: i32 = 9;
const wxSTC_ECL_UNKNOWN: i32 = 10;
const wxSTC_ECL_IDENTIFIER: i32 = 11;
const wxSTC_ECL_STRINGEOL: i32 = 12;
const wxSTC_ECL_VERBATIM: i32 = 13;
const wxSTC_ECL_REGEX: i32 = 14;
const wxSTC_ECL_COMMENTLINEDOC: i32 = 15;
const wxSTC_ECL_WORD1: i32 = 16;
const wxSTC_ECL_COMMENTDOCKEYWORD: i32 = 17;
const wxSTC_ECL_COMMENTDOCKEYWORDERROR: i32 = 18;
const wxSTC_ECL_WORD2: i32 = 19;
const wxSTC_ECL_WORD3: i32 = 20;
const wxSTC_ECL_WORD4: i32 = 21;
const wxSTC_ECL_WORD5: i32 = 22;
const wxSTC_ECL_COMMENTDOC: i32 = 23;
const wxSTC_ECL_ADDED: i32 = 24;
const wxSTC_ECL_DELETED: i32 = 25;
const wxSTC_ECL_CHANGED: i32 = 26;
const wxSTC_ECL_MOVED: i32 = 27;
const wxSTC_OSCRIPT_DEFAULT: i32 = 0;
const wxSTC_OSCRIPT_LINE_COMMENT: i32 = 1;
const wxSTC_OSCRIPT_BLOCK_COMMENT: i32 = 2;
const wxSTC_OSCRIPT_DOC_COMMENT: i32 = 3;
const wxSTC_OSCRIPT_PREPROCESSOR: i32 = 4;
const wxSTC_OSCRIPT_NUMBER: i32 = 5;
const wxSTC_OSCRIPT_SINGLEQUOTE_STRING: i32 = 6;
const wxSTC_OSCRIPT_DOUBLEQUOTE_STRING: i32 = 7;
const wxSTC_OSCRIPT_CONSTANT: i32 = 8;
const wxSTC_OSCRIPT_IDENTIFIER: i32 = 9;
const wxSTC_OSCRIPT_GLOBAL: i32 = 10;
const wxSTC_OSCRIPT_KEYWORD: i32 = 11;
const wxSTC_OSCRIPT_OPERATOR: i32 = 12;
const wxSTC_OSCRIPT_LABEL: i32 = 13;
const wxSTC_OSCRIPT_TYPE: i32 = 14;
const wxSTC_OSCRIPT_FUNCTION: i32 = 15;
const wxSTC_OSCRIPT_OBJECT: i32 = 16;
const wxSTC_OSCRIPT_PROPERTY: i32 = 17;
const wxSTC_OSCRIPT_METHOD: i32 = 18;
const wxSTC_VISUALPROLOG_DEFAULT: i32 = 0;
const wxSTC_VISUALPROLOG_KEY_MAJOR: i32 = 1;
const wxSTC_VISUALPROLOG_KEY_MINOR: i32 = 2;
const wxSTC_VISUALPROLOG_KEY_DIRECTIVE: i32 = 3;
const wxSTC_VISUALPROLOG_COMMENT_BLOCK: i32 = 4;
const wxSTC_VISUALPROLOG_COMMENT_LINE: i32 = 5;
const wxSTC_VISUALPROLOG_COMMENT_KEY: i32 = 6;
const wxSTC_VISUALPROLOG_COMMENT_KEY_ERROR: i32 = 7;
const wxSTC_VISUALPROLOG_IDENTIFIER: i32 = 8;
const wxSTC_VISUALPROLOG_VARIABLE: i32 = 9;
const wxSTC_VISUALPROLOG_ANONYMOUS: i32 = 10;
const wxSTC_VISUALPROLOG_NUMBER: i32 = 11;
const wxSTC_VISUALPROLOG_OPERATOR: i32 = 12;
const wxSTC_VISUALPROLOG_CHARACTER: i32 = 13;
const wxSTC_VISUALPROLOG_CHARACTER_TOO_MANY: i32 = 14;
const wxSTC_VISUALPROLOG_CHARACTER_ESCAPE_ERROR: i32 = 15;
const wxSTC_VISUALPROLOG_STRING: i32 = 16;
const wxSTC_VISUALPROLOG_STRING_ESCAPE: i32 = 17;
const wxSTC_VISUALPROLOG_STRING_ESCAPE_ERROR: i32 = 18;
const wxSTC_VISUALPROLOG_STRING_EOL_OPEN: i32 = 19;
const wxSTC_VISUALPROLOG_STRING_VERBATIM: i32 = 20;
const wxSTC_VISUALPROLOG_STRING_VERBATIM_SPECIAL: i32 = 21;
const wxSTC_VISUALPROLOG_STRING_VERBATIM_EOL: i32 = 22;
const wxSTC_STTXT_DEFAULT: i32 = 0;
const wxSTC_STTXT_COMMENT: i32 = 1;
const wxSTC_STTXT_COMMENTLINE: i32 = 2;
const wxSTC_STTXT_KEYWORD: i32 = 3;
const wxSTC_STTXT_TYPE: i32 = 4;
const wxSTC_STTXT_FUNCTION: i32 = 5;
const wxSTC_STTXT_FB: i32 = 6;
const wxSTC_STTXT_NUMBER: i32 = 7;
const wxSTC_STTXT_HEXNUMBER: i32 = 8;
const wxSTC_STTXT_PRAGMA: i32 = 9;
const wxSTC_STTXT_OPERATOR: i32 = 10;
const wxSTC_STTXT_CHARACTER: i32 = 11;
const wxSTC_STTXT_STRING1: i32 = 12;
const wxSTC_STTXT_STRING2: i32 = 13;
const wxSTC_STTXT_STRINGEOL: i32 = 14;
const wxSTC_STTXT_IDENTIFIER: i32 = 15;
const wxSTC_STTXT_DATETIME: i32 = 16;
const wxSTC_STTXT_VARS: i32 = 17;
const wxSTC_STTXT_PRAGMAS: i32 = 18;
const wxSTC_KVIRC_DEFAULT: i32 = 0;
const wxSTC_KVIRC_COMMENT: i32 = 1;
const wxSTC_KVIRC_COMMENTBLOCK: i32 = 2;
const wxSTC_KVIRC_STRING: i32 = 3;
const wxSTC_KVIRC_WORD: i32 = 4;
const wxSTC_KVIRC_KEYWORD: i32 = 5;
const wxSTC_KVIRC_FUNCTION_KEYWORD: i32 = 6;
const wxSTC_KVIRC_FUNCTION: i32 = 7;
const wxSTC_KVIRC_VARIABLE: i32 = 8;
const wxSTC_KVIRC_NUMBER: i32 = 9;
const wxSTC_KVIRC_OPERATOR: i32 = 10;
const wxSTC_KVIRC_STRING_FUNCTION: i32 = 11;
const wxSTC_KVIRC_STRING_VARIABLE: i32 = 12;
const wxSTC_RUST_DEFAULT: i32 = 0;
const wxSTC_RUST_COMMENTBLOCK: i32 = 1;
const wxSTC_RUST_COMMENTLINE: i32 = 2;
const wxSTC_RUST_COMMENTBLOCKDOC: i32 = 3;
const wxSTC_RUST_COMMENTLINEDOC: i32 = 4;
const wxSTC_RUST_NUMBER: i32 = 5;
const wxSTC_RUST_WORD: i32 = 6;
const wxSTC_RUST_WORD2: i32 = 7;
const wxSTC_RUST_WORD3: i32 = 8;
const wxSTC_RUST_WORD4: i32 = 9;
const wxSTC_RUST_WORD5: i32 = 10;
const wxSTC_RUST_WORD6: i32 = 11;
const wxSTC_RUST_WORD7: i32 = 12;
const wxSTC_RUST_STRING: i32 = 13;
const wxSTC_RUST_STRINGR: i32 = 14;
const wxSTC_RUST_CHARACTER: i32 = 15;
const wxSTC_RUST_OPERATOR: i32 = 16;
const wxSTC_RUST_IDENTIFIER: i32 = 17;
const wxSTC_RUST_LIFETIME: i32 = 18;
const wxSTC_RUST_MACRO: i32 = 19;
const wxSTC_RUST_LEXERROR: i32 = 20;
const wxSTC_RUST_BYTESTRING: i32 = 21;
const wxSTC_RUST_BYTESTRINGR: i32 = 22;
const wxSTC_RUST_BYTECHARACTER: i32 = 23;
const wxSTC_DMAP_DEFAULT: i32 = 0;
const wxSTC_DMAP_COMMENT: i32 = 1;
const wxSTC_DMAP_NUMBER: i32 = 2;
const wxSTC_DMAP_STRING1: i32 = 3;
const wxSTC_DMAP_STRING2: i32 = 4;
const wxSTC_DMAP_STRINGEOL: i32 = 5;
const wxSTC_DMAP_OPERATOR: i32 = 6;
const wxSTC_DMAP_IDENTIFIER: i32 = 7;
const wxSTC_DMAP_WORD: i32 = 8;
const wxSTC_DMAP_WORD2: i32 = 9;
const wxSTC_DMAP_WORD3: i32 = 10;
const wxSTC_DMIS_DEFAULT: i32 = 0;
const wxSTC_DMIS_COMMENT: i32 = 1;
const wxSTC_DMIS_STRING: i32 = 2;
const wxSTC_DMIS_NUMBER: i32 = 3;
const wxSTC_DMIS_KEYWORD: i32 = 4;
const wxSTC_DMIS_MAJORWORD: i32 = 5;
const wxSTC_DMIS_MINORWORD: i32 = 6;
const wxSTC_DMIS_UNSUPPORTED_MAJOR: i32 = 7;
const wxSTC_DMIS_UNSUPPORTED_MINOR: i32 = 8;
const wxSTC_DMIS_LABEL: i32 = 9;
const wxSTC_REG_DEFAULT: i32 = 0;
const wxSTC_REG_COMMENT: i32 = 1;
const wxSTC_REG_VALUENAME: i32 = 2;
const wxSTC_REG_STRING: i32 = 3;
const wxSTC_REG_HEXDIGIT: i32 = 4;
const wxSTC_REG_VALUETYPE: i32 = 5;
const wxSTC_REG_ADDEDKEY: i32 = 6;
const wxSTC_REG_DELETEDKEY: i32 = 7;
const wxSTC_REG_ESCAPED: i32 = 8;
const wxSTC_REG_KEYPATH_GUID: i32 = 9;
const wxSTC_REG_STRING_GUID: i32 = 10;
const wxSTC_REG_PARAMETER: i32 = 11;
const wxSTC_REG_OPERATOR: i32 = 12;
const wxSTC_BIBTEX_DEFAULT: i32 = 0;
const wxSTC_BIBTEX_ENTRY: i32 = 1;
const wxSTC_BIBTEX_UNKNOWN_ENTRY: i32 = 2;
const wxSTC_BIBTEX_KEY: i32 = 3;
const wxSTC_BIBTEX_PARAMETER: i32 = 4;
const wxSTC_BIBTEX_VALUE: i32 = 5;
const wxSTC_BIBTEX_COMMENT: i32 = 6;
const wxSTC_HEX_DEFAULT: i32 = 0;
const wxSTC_HEX_RECSTART: i32 = 1;
const wxSTC_HEX_RECTYPE: i32 = 2;
const wxSTC_HEX_RECTYPE_UNKNOWN: i32 = 3;
const wxSTC_HEX_BYTECOUNT: i32 = 4;
const wxSTC_HEX_BYTECOUNT_WRONG: i32 = 5;
const wxSTC_HEX_NOADDRESS: i32 = 6;
const wxSTC_HEX_DATAADDRESS: i32 = 7;
const wxSTC_HEX_RECCOUNT: i32 = 8;
const wxSTC_HEX_STARTADDRESS: i32 = 9;
const wxSTC_HEX_ADDRESSFIELD_UNKNOWN: i32 = 10;
const wxSTC_HEX_EXTENDEDADDRESS: i32 = 11;
const wxSTC_HEX_DATA_ODD: i32 = 12;
const wxSTC_HEX_DATA_EVEN: i32 = 13;
const wxSTC_HEX_DATA_UNKNOWN: i32 = 14;
const wxSTC_HEX_DATA_EMPTY: i32 = 15;
const wxSTC_HEX_CHECKSUM: i32 = 16;
const wxSTC_HEX_CHECKSUM_WRONG: i32 = 17;
const wxSTC_HEX_GARBAGE: i32 = 18;
const wxSTC_JSON_DEFAULT: i32 = 0;
const wxSTC_JSON_NUMBER: i32 = 1;
const wxSTC_JSON_STRING: i32 = 2;
const wxSTC_JSON_STRINGEOL: i32 = 3;
const wxSTC_JSON_PROPERTYNAME: i32 = 4;
const wxSTC_JSON_ESCAPESEQUENCE: i32 = 5;
const wxSTC_JSON_LINECOMMENT: i32 = 6;
const wxSTC_JSON_BLOCKCOMMENT: i32 = 7;
const wxSTC_JSON_OPERATOR: i32 = 8;
const wxSTC_JSON_URI: i32 = 9;
const wxSTC_JSON_COMPACTIRI: i32 = 10;
const wxSTC_JSON_KEYWORD: i32 = 11;
const wxSTC_JSON_LDKEYWORD: i32 = 12;
const wxSTC_JSON_ERROR: i32 = 13;
const wxSTC_EDI_DEFAULT: i32 = 0;
const wxSTC_EDI_SEGMENTSTART: i32 = 1;
const wxSTC_EDI_SEGMENTEND: i32 = 2;
const wxSTC_EDI_SEP_ELEMENT: i32 = 3;
const wxSTC_EDI_SEP_COMPOSITE: i32 = 4;
const wxSTC_EDI_SEP_RELEASE: i32 = 5;
const wxSTC_EDI_UNA: i32 = 6;
const wxSTC_EDI_UNH: i32 = 7;
const wxSTC_EDI_BADSEGMENT: i32 = 8;
const wxSTC_INDIC0_MASK: i32 = 0x20;
const wxSTC_INDIC1_MASK: i32 = 0x40;
const wxSTC_INDIC2_MASK: i32 = 0x80;
const wxSTC_INDICS_MASK: i32 = 0xE0;
const wxSTC_CMD_REDO: i32 = 2011;
const wxSTC_CMD_SELECTALL: i32 = 2013;
const wxSTC_CMD_UNDO: i32 = 2176;
const wxSTC_CMD_CUT: i32 = 2177;
const wxSTC_CMD_COPY: i32 = 2178;
const wxSTC_CMD_PASTE: i32 = 2179;
const wxSTC_CMD_CLEAR: i32 = 2180;
const wxSTC_CMD_LINEDOWN: i32 = 2300;
const wxSTC_CMD_LINEDOWNEXTEND: i32 = 2301;
const wxSTC_CMD_LINEUP: i32 = 2302;
const wxSTC_CMD_LINEUPEXTEND: i32 = 2303;
const wxSTC_CMD_CHARLEFT: i32 = 2304;
const wxSTC_CMD_CHARLEFTEXTEND: i32 = 2305;
const wxSTC_CMD_CHARRIGHT: i32 = 2306;
const wxSTC_CMD_CHARRIGHTEXTEND: i32 = 2307;
const wxSTC_CMD_WORDLEFT: i32 = 2308;
const wxSTC_CMD_WORDLEFTEXTEND: i32 = 2309;
const wxSTC_CMD_WORDRIGHT: i32 = 2310;
const wxSTC_CMD_WORDRIGHTEXTEND: i32 = 2311;
const wxSTC_CMD_HOME: i32 = 2312;
const wxSTC_CMD_HOMEEXTEND: i32 = 2313;
const wxSTC_CMD_LINEEND: i32 = 2314;
const wxSTC_CMD_LINEENDEXTEND: i32 = 2315;
const wxSTC_CMD_DOCUMENTSTART: i32 = 2316;
const wxSTC_CMD_DOCUMENTSTARTEXTEND: i32 = 2317;
const wxSTC_CMD_DOCUMENTEND: i32 = 2318;
const wxSTC_CMD_DOCUMENTENDEXTEND: i32 = 2319;
const wxSTC_CMD_PAGEUP: i32 = 2320;
const wxSTC_CMD_PAGEUPEXTEND: i32 = 2321;
const wxSTC_CMD_PAGEDOWN: i32 = 2322;
const wxSTC_CMD_PAGEDOWNEXTEND: i32 = 2323;
const wxSTC_CMD_EDITTOGGLEOVERTYPE: i32 = 2324;
const wxSTC_CMD_CANCEL: i32 = 2325;
const wxSTC_CMD_DELETEBACK: i32 = 2326;
const wxSTC_CMD_TAB: i32 = 2327;
const wxSTC_CMD_BACKTAB: i32 = 2328;
const wxSTC_CMD_NEWLINE: i32 = 2329;
const wxSTC_CMD_FORMFEED: i32 = 2330;
const wxSTC_CMD_VCHOME: i32 = 2331;
const wxSTC_CMD_VCHOMEEXTEND: i32 = 2332;
const wxSTC_CMD_ZOOMIN: i32 = 2333;
const wxSTC_CMD_ZOOMOUT: i32 = 2334;
const wxSTC_CMD_DELWORDLEFT: i32 = 2335;
const wxSTC_CMD_DELWORDRIGHT: i32 = 2336;
const wxSTC_CMD_DELWORDRIGHTEND: i32 = 2518;
const wxSTC_CMD_LINECUT: i32 = 2337;
const wxSTC_CMD_LINEDELETE: i32 = 2338;
const wxSTC_CMD_LINETRANSPOSE: i32 = 2339;
const wxSTC_CMD_LINEDUPLICATE: i32 = 2404;
const wxSTC_CMD_LOWERCASE: i32 = 2340;
const wxSTC_CMD_UPPERCASE: i32 = 2341;
const wxSTC_CMD_LINESCROLLDOWN: i32 = 2342;
const wxSTC_CMD_LINESCROLLUP: i32 = 2343;
const wxSTC_CMD_DELETEBACKNOTLINE: i32 = 2344;
const wxSTC_CMD_HOMEDISPLAY: i32 = 2345;
const wxSTC_CMD_HOMEDISPLAYEXTEND: i32 = 2346;
const wxSTC_CMD_LINEENDDISPLAY: i32 = 2347;
const wxSTC_CMD_LINEENDDISPLAYEXTEND: i32 = 2348;
const wxSTC_CMD_HOMEWRAP: i32 = 2349;
const wxSTC_CMD_HOMEWRAPEXTEND: i32 = 2450;
const wxSTC_CMD_LINEENDWRAP: i32 = 2451;
const wxSTC_CMD_LINEENDWRAPEXTEND: i32 = 2452;
const wxSTC_CMD_VCHOMEWRAP: i32 = 2453;
const wxSTC_CMD_VCHOMEWRAPEXTEND: i32 = 2454;
const wxSTC_CMD_LINECOPY: i32 = 2455;
const wxSTC_CMD_WORDPARTLEFT: i32 = 2390;
const wxSTC_CMD_WORDPARTLEFTEXTEND: i32 = 2391;
const wxSTC_CMD_WORDPARTRIGHT: i32 = 2392;
const wxSTC_CMD_WORDPARTRIGHTEXTEND: i32 = 2393;
const wxSTC_CMD_DELLINELEFT: i32 = 2395;
const wxSTC_CMD_DELLINERIGHT: i32 = 2396;
const wxSTC_CMD_PARADOWN: i32 = 2413;
const wxSTC_CMD_PARADOWNEXTEND: i32 = 2414;
const wxSTC_CMD_PARAUP: i32 = 2415;
const wxSTC_CMD_PARAUPEXTEND: i32 = 2416;
const wxSTC_CMD_LINEDOWNRECTEXTEND: i32 = 2426;
const wxSTC_CMD_LINEUPRECTEXTEND: i32 = 2427;
const wxSTC_CMD_CHARLEFTRECTEXTEND: i32 = 2428;
const wxSTC_CMD_CHARRIGHTRECTEXTEND: i32 = 2429;
const wxSTC_CMD_HOMERECTEXTEND: i32 = 2430;
const wxSTC_CMD_VCHOMERECTEXTEND: i32 = 2431;
const wxSTC_CMD_LINEENDRECTEXTEND: i32 = 2432;
const wxSTC_CMD_PAGEUPRECTEXTEND: i32 = 2433;
const wxSTC_CMD_PAGEDOWNRECTEXTEND: i32 = 2434;
const wxSTC_CMD_STUTTEREDPAGEUP: i32 = 2435;
const wxSTC_CMD_STUTTEREDPAGEUPEXTEND: i32 = 2436;
const wxSTC_CMD_STUTTEREDPAGEDOWN: i32 = 2437;
const wxSTC_CMD_STUTTEREDPAGEDOWNEXTEND: i32 = 2438;
const wxSTC_CMD_WORDLEFTEND: i32 = 2439;
const wxSTC_CMD_WORDLEFTENDEXTEND: i32 = 2440;
const wxSTC_CMD_WORDRIGHTEND: i32 = 2441;
const wxSTC_CMD_WORDRIGHTENDEXTEND: i32 = 2442;
const wxSTC_CMD_VERTICALCENTRECARET: i32 = 2619;
const wxSTC_CMD_MOVESELECTEDLINESUP: i32 = 2620;
const wxSTC_CMD_MOVESELECTEDLINESDOWN: i32 = 2621;
const wxSTC_CMD_SCROLLTOSTART: i32 = 2628;
const wxSTC_CMD_SCROLLTOEND: i32 = 2629;
const wxSTC_CMD_VCHOMEDISPLAY: i32 = 2652;
const wxSTC_CMD_VCHOMEDISPLAYEXTEND: i32 = 2653;

//  ENUM: @26
const wxID_HTML_PANEL: i32 = wxID_HIGHEST + 10;
const wxID_HTML_BACK: i32 = wxID_HIGHEST + 10 + 1;
const wxID_HTML_FORWARD: i32 = wxID_HIGHEST + 10 + 2;
const wxID_HTML_UPNODE: i32 = wxID_HIGHEST + 10 + 3;
const wxID_HTML_UP: i32 = wxID_HIGHEST + 10 + 4;
const wxID_HTML_DOWN: i32 = wxID_HIGHEST + 10 + 5;
const wxID_HTML_PRINT: i32 = wxID_HIGHEST + 10 + 6;
const wxID_HTML_OPENFILE: i32 = wxID_HIGHEST + 10 + 7;
const wxID_HTML_OPTIONS: i32 = wxID_HIGHEST + 10 + 8;
const wxID_HTML_BOOKMARKSLIST: i32 = wxID_HIGHEST + 10 + 9;
const wxID_HTML_BOOKMARKSADD: i32 = wxID_HIGHEST + 10 + 10;
const wxID_HTML_BOOKMARKSREMOVE: i32 = wxID_HIGHEST + 10 + 11;
const wxID_HTML_TREECTRL: i32 = wxID_HIGHEST + 10 + 12;
const wxID_HTML_INDEXPAGE: i32 = wxID_HIGHEST + 10 + 13;
const wxID_HTML_INDEXLIST: i32 = wxID_HIGHEST + 10 + 14;
const wxID_HTML_INDEXTEXT: i32 = wxID_HIGHEST + 10 + 15;
const wxID_HTML_INDEXBUTTON: i32 = wxID_HIGHEST + 10 + 16;
const wxID_HTML_INDEXBUTTONALL: i32 = wxID_HIGHEST + 10 + 17;
const wxID_HTML_NOTEBOOK: i32 = wxID_HIGHEST + 10 + 18;
const wxID_HTML_SEARCHPAGE: i32 = wxID_HIGHEST + 10 + 19;
const wxID_HTML_SEARCHTEXT: i32 = wxID_HIGHEST + 10 + 20;
const wxID_HTML_SEARCHLIST: i32 = wxID_HIGHEST + 10 + 21;
const wxID_HTML_SEARCHBUTTON: i32 = wxID_HIGHEST + 10 + 22;
const wxID_HTML_SEARCHCHOICE: i32 = wxID_HIGHEST + 10 + 23;
const wxID_HTML_COUNTINFO: i32 = wxID_HIGHEST + 10 + 24;

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
const wxEVENT_PROPAGATE_NONE: i32 = 0;
const wxEVENT_PROPAGATE_MAX: i32 = INT_MAX;
//  ENUM: wxEventCategory
const wxEVT_CATEGORY_UI: i32 = 1;
const wxEVT_CATEGORY_USER_INPUT: i32 = 2;
const wxEVT_CATEGORY_SOCKET: i32 = 4;
const wxEVT_CATEGORY_TIMER: i32 = 8;
const wxEVT_CATEGORY_THREAD: i32 = 16;
const wxEVT_CATEGORY_ALL: i32 =
        wxEVT_CATEGORY_UI|wxEVT_CATEGORY_USER_INPUT|wxEVT_CATEGORY_SOCKET| 
        wxEVT_CATEGORY_TIMER|wxEVT_CATEGORY_THREAD;
//  ENUM: wxKeyCategoryFlags
const WXK_CATEGORY_ARROW: i32 = 0;
const WXK_CATEGORY_PAGING: i32 = 0 + 1;
const WXK_CATEGORY_JUMP: i32 = 0 + 2;
const WXK_CATEGORY_TAB: i32 = 0 + 3;
const WXK_CATEGORY_CUT: i32 = 0 + 4;
const WXK_CATEGORY_NAVIGATION: i32 = 0 + 5;
//  ENUM: @14
const wxJOYSTICK1: i32 = 0;
const wxJOYSTICK2: i32 = 0 + 1;
//  ENUM: @15
const wxJOY_BUTTON_ANY: i32 = -1;
const wxJOY_BUTTON1: i32 = 1;
const wxJOY_BUTTON2: i32 = 2;
const wxJOY_BUTTON3: i32 = 4;
const wxJOY_BUTTON4: i32 = 8;
//  ENUM: wxUpdateUIMode
const wxUPDATE_UI_PROCESS_ALL: i32 = 0;
const wxUPDATE_UI_PROCESS_SPECIFIED: i32 = 0 + 1;
//  ENUM: wxMouseWheelAxis
const wxMOUSE_WHEEL_VERTICAL: i32 = 0;
const wxMOUSE_WHEEL_HORIZONTAL: i32 = 0 + 1;
//  ENUM: wxIdleMode
const wxIDLE_PROCESS_ALL: i32 = 0;
const wxIDLE_PROCESS_SPECIFIED: i32 = 0 + 1;

//  ENUM: wxRibbonBarOption
const wxRIBBON_BAR_SHOW_PAGE_LABELS: i32 = 0;
const wxRIBBON_BAR_SHOW_PAGE_ICONS: i32 = 0 + 1;
const wxRIBBON_BAR_FLOW_HORIZONTAL: i32 = 0 + 2;
const wxRIBBON_BAR_FLOW_VERTICAL: i32 = 0 + 3;
const wxRIBBON_BAR_SHOW_PANEL_EXT_BUTTONS: i32 = 0 + 4;
const wxRIBBON_BAR_SHOW_PANEL_MINIMISE_BUTTONS: i32 = 0 + 5;
const wxRIBBON_BAR_ALWAYS_SHOW_TABS: i32 = 0 + 6;
const wxRIBBON_BAR_SHOW_TOGGLE_BUTTON: i32 = 0 + 7;
const wxRIBBON_BAR_SHOW_HELP_BUTTON: i32 = 0 + 8;
const wxRIBBON_BAR_DEFAULT_STYLE: i32 = 0 + 9;
const wxRIBBON_BAR_FOLDBAR_STYLE: i32 = 0 + 10;
//  ENUM: wxRibbonDisplayMode
const wxRIBBON_BAR_PINNED: i32 = 0;
const wxRIBBON_BAR_MINIMIZED: i32 = 0 + 1;
const wxRIBBON_BAR_EXPANDED: i32 = 0 + 2;

//  ENUM: wxOperatingSystemId
const wxOS_UNKNOWN: i32 = 0;
const wxOS_MAC_OS: i32 = 1 << 0;
const wxOS_MAC_OSX_DARWIN: i32 = 1 << 1;
const wxOS_MAC: i32 = wxOS_MAC_OS|wxOS_MAC_OSX_DARWIN;
const wxOS_WINDOWS_NT: i32 = 1 << 3;
const wxOS_WINDOWS: i32 = wxOS_WINDOWS_NT;
const wxOS_UNIX_LINUX: i32 = 1 << 6;
const wxOS_UNIX_FREEBSD: i32 = 1 << 7;
const wxOS_UNIX_OPENBSD: i32 = 1 << 8;
const wxOS_UNIX_NETBSD: i32 = 1 << 9;
const wxOS_UNIX_SOLARIS: i32 = 1 << 10;
const wxOS_UNIX_AIX: i32 = 1 << 11;
const wxOS_UNIX_HPUX: i32 = 1 << 12;
const wxOS_UNIX: i32 = wxOS_UNIX_LINUX     |
                wxOS_UNIX_FREEBSD   |
                wxOS_UNIX_OPENBSD   |
                wxOS_UNIX_NETBSD    |
                wxOS_UNIX_SOLARIS   |
                wxOS_UNIX_AIX       |
                wxOS_UNIX_HPUX;
//  ENUM: wxPortId
const wxPORT_UNKNOWN: i32 = 0;
const wxPORT_BASE: i32 = 1 << 0;
const wxPORT_MSW: i32 = 1 << 1;
const wxPORT_MOTIF: i32 = 1 << 2;
const wxPORT_GTK: i32 = 1 << 3;
const wxPORT_DFB: i32 = 1 << 4;
const wxPORT_X11: i32 = 1 << 5;
const wxPORT_MAC: i32 = 1 << 7;
const wxPORT_COCOA: i32 = 1 << 8;
const wxPORT_QT: i32 = 1 << 10;
//  ENUM: wxBitness
const wxBITNESS_INVALID: i32 = -1;
const wxBITNESS_32: i32 = -1 + 1;
const wxBITNESS_64: i32 = -1 + 2;
const wxBITNESS_MAX: i32 = -1 + 3;
//  ENUM: wxArchitecture
const wxARCH_INVALID: i32 = -1;
const wxARCH_32: i32 = -1 + 1;
const wxARCH_64: i32 = -1 + 2;
const wxARCH_MAX: i32 = -1 + 3;
//  ENUM: wxEndianness
const wxENDIAN_INVALID: i32 = -1;
const wxENDIAN_BIG: i32 = -1 + 1;
const wxENDIAN_LITTLE: i32 = -1 + 2;
const wxENDIAN_PDP: i32 = -1 + 3;
const wxENDIAN_MAX: i32 = -1 + 4;

// NODEF: wxPLURAL
// NODEF: wxGETTEXT_IN_CONTEXT
// NODEF: wxGETTEXT_IN_CONTEXT_PLURAL
// NODEF: wxTRANSLATE

//  ENUM: wxMouseButton
const wxMOUSE_BTN_ANY: i32 = -1;
const wxMOUSE_BTN_NONE: i32 = 0;
const wxMOUSE_BTN_LEFT: i32 = 1;
const wxMOUSE_BTN_MIDDLE: i32 = 2;
const wxMOUSE_BTN_RIGHT: i32 = 3;
const wxMOUSE_BTN_AUX1: i32 = 4;
const wxMOUSE_BTN_AUX2: i32 = 5;
const wxMOUSE_BTN_MAX: i32 = 5 + 1;

const wxBK_DEFAULT: i32 = 0x0000;
const wxBK_TOP: i32 = 0x0010;
const wxBK_BOTTOM: i32 = 0x0020;
const wxBK_LEFT: i32 = 0x0040;
const wxBK_RIGHT: i32 = 0x0080;
const wxBK_ALIGN_MASK: i32 = (wxBK_TOP | wxBK_BOTTOM | wxBK_LEFT | wxBK_RIGHT);
//  SKIP: wxBookCtrl
//  ENUM: @2
const wxBK_HITTEST_NOWHERE: i32 = 1;
const wxBK_HITTEST_ONICON: i32 = 2;
const wxBK_HITTEST_ONLABEL: i32 = 4;
const wxBK_HITTEST_ONITEM: i32 = 16;
const wxBK_HITTEST_ONPAGE: i32 = 8;

//  ENUM: AccessMode
const Read: i32 = 0;
const Write: i32 = 0 + 1;
//  ENUM: StdKey
const HKCR: i32 = 0;
const HKCU: i32 = 0 + 1;
const HKLM: i32 = 0 + 2;
const HKUSR: i32 = 0 + 3;
const HKPD: i32 = 0 + 4;
const HKCC: i32 = 0 + 5;
const HKDD: i32 = 0 + 6;
const HKMAX: i32 = 0 + 7;
//  ENUM: ValueType
const Type_None: i32 = 0;
const Type_String: i32 = 0 + 1;
const Type_Expand_String: i32 = 0 + 2;
const Type_Binary: i32 = 0 + 3;
const Type_Dword: i32 = 0 + 4;
const Type_Dword_little_endian: i32 = 0 + 5;
const Type_Dword_big_endian: i32 = 0 + 6;
const Type_Link: i32 = 0 + 7;
const Type_Multi_String: i32 = 0 + 8;
const Type_Resource_list: i32 = 0 + 9;
const Type_Full_resource_descriptor: i32 = 0 + 10;
const Type_Resource_requirements_list: i32 = 0 + 11;
//  ENUM: WOW64ViewMode
const WOW64ViewMode_Default: i32 = 0;
const WOW64ViewMode_32: i32 = 0 + 1;
const WOW64ViewMode_64: i32 = 0 + 2;

const wxDEFAULT_FRAME_STYLE: i32 =                                (wxSYSTEM_MENU |          \
                               wxRESIZE_BORDER |        \
                               wxMINIMIZE_BOX |         \
                               wxMAXIMIZE_BOX |         \
                               wxCLOSE_BOX |            \
                               wxCAPTION |              \
                               wxCLIP_CHILDREN);
//  ENUM: @50
const wxUSER_ATTENTION_INFO: i32 = 1;
const wxUSER_ATTENTION_ERROR: i32 = 2;
//  ENUM: @51
const wxFULLSCREEN_NOMENUBAR: i32 = 0x0001;
const wxFULLSCREEN_NOTOOLBAR: i32 = 0x0002;
const wxFULLSCREEN_NOSTATUSBAR: i32 = 0x0004;
const wxFULLSCREEN_NOBORDER: i32 = 0x0008;
const wxFULLSCREEN_NOCAPTION: i32 = 0x0010;
const wxFULLSCREEN_ALL: i32 = wxFULLSCREEN_NOMENUBAR | wxFULLSCREEN_NOTOOLBAR |
                          wxFULLSCREEN_NOSTATUSBAR | wxFULLSCREEN_NOBORDER |
                          wxFULLSCREEN_NOCAPTION;

const wxEL_ALLOW_NEW: i32 = 0x0100;
const wxEL_ALLOW_EDIT: i32 = 0x0200;
const wxEL_ALLOW_DELETE: i32 = 0x0400;
const wxEL_NO_REORDER: i32 = 0x0800;
const wxEL_DEFAULT_STYLE: i32 = (wxEL_ALLOW_NEW | wxEL_ALLOW_EDIT | wxEL_ALLOW_DELETE);

const wxFC_DEFAULT_STYLE: i32 = wxFC_OPEN;
//  ENUM: @18
const wxFC_OPEN: i32 = 0x0001;
const wxFC_SAVE: i32 = 0x0002;
const wxFC_MULTIPLE: i32 = 0x0004;
const wxFC_NOSHOWHIDDEN: i32 = 0x0008;

const wxSL_HORIZONTAL: i32 = wxHORIZONTAL /* 0x0004 */;
const wxSL_VERTICAL: i32 = wxVERTICAL   /* 0x0008 */;
const wxSL_TICKS: i32 = 0x0010;
const wxSL_AUTOTICKS: i32 = wxSL_TICKS;
const wxSL_LEFT: i32 = 0x0040;
const wxSL_TOP: i32 = 0x0080;
const wxSL_RIGHT: i32 = 0x0100;
const wxSL_BOTTOM: i32 = 0x0200;
const wxSL_BOTH: i32 = 0x0400;
const wxSL_SELRANGE: i32 = 0x0800;
const wxSL_INVERSE: i32 = 0x1000;
const wxSL_MIN_MAX_LABELS: i32 = 0x2000;
const wxSL_VALUE_LABEL: i32 = 0x4000;
const wxSL_LABELS: i32 = (wxSL_MIN_MAX_LABELS|wxSL_VALUE_LABEL);

//  ENUM: wxRibbonGalleryButtonState
const wxRIBBON_GALLERY_BUTTON_NORMAL: i32 = 0;
const wxRIBBON_GALLERY_BUTTON_HOVERED: i32 = 0 + 1;
const wxRIBBON_GALLERY_BUTTON_ACTIVE: i32 = 0 + 2;
const wxRIBBON_GALLERY_BUTTON_DISABLED: i32 = 0 + 3;

//  ENUM: @22
const WX_GL_RGBA: i32 = 1;
const WX_GL_BUFFER_SIZE: i32 = 1 + 1;
const WX_GL_LEVEL: i32 = 1 + 2;
const WX_GL_DOUBLEBUFFER: i32 = 1 + 3;
const WX_GL_STEREO: i32 = 1 + 4;
const WX_GL_AUX_BUFFERS: i32 = 1 + 5;
const WX_GL_MIN_RED: i32 = 1 + 6;
const WX_GL_MIN_GREEN: i32 = 1 + 7;
const WX_GL_MIN_BLUE: i32 = 1 + 8;
const WX_GL_MIN_ALPHA: i32 = 1 + 9;
const WX_GL_DEPTH_SIZE: i32 = 1 + 10;
const WX_GL_STENCIL_SIZE: i32 = 1 + 11;
const WX_GL_MIN_ACCUM_RED: i32 = 1 + 12;
const WX_GL_MIN_ACCUM_GREEN: i32 = 1 + 13;
const WX_GL_MIN_ACCUM_BLUE: i32 = 1 + 14;
const WX_GL_MIN_ACCUM_ALPHA: i32 = 1 + 15;
const WX_GL_SAMPLE_BUFFERS: i32 = 1 + 16;
const WX_GL_SAMPLES: i32 = 1 + 17;
const WX_GL_FRAMEBUFFER_SRGB: i32 = 1 + 18;
const WX_GL_MAJOR_VERSION: i32 = 1 + 19;
const WX_GL_MINOR_VERSION: i32 = 1 + 20;
const WX_GL_CORE_PROFILE: i32 = 1 + 21;
const wx_GL_COMPAT_PROFILE: i32 = 1 + 22;
const WX_GL_FORWARD_COMPAT: i32 = 1 + 23;
const WX_GL_ES2: i32 = 1 + 24;
const WX_GL_DEBUG: i32 = 1 + 25;
const WX_GL_ROBUST_ACCESS: i32 = 1 + 26;
const WX_GL_NO_RESET_NOTIFY: i32 = 1 + 27;
const WX_GL_LOSE_ON_RESET: i32 = 1 + 28;
const WX_GL_RESET_ISOLATION: i32 = 1 + 29;
const WX_GL_RELEASE_FLUSH: i32 = 1 + 30;
const WX_GL_RELEASE_NONE: i32 = 1 + 31;

const wxACC_SELF: i32 = 0;
const wxACC_STATE_SYSTEM_ALERT_HIGH: i32 = 0x00000001;
const wxACC_STATE_SYSTEM_ALERT_MEDIUM: i32 = 0x00000002;
const wxACC_STATE_SYSTEM_ALERT_LOW: i32 = 0x00000004;
const wxACC_STATE_SYSTEM_ANIMATED: i32 = 0x00000008;
const wxACC_STATE_SYSTEM_BUSY: i32 = 0x00000010;
const wxACC_STATE_SYSTEM_CHECKED: i32 = 0x00000020;
const wxACC_STATE_SYSTEM_COLLAPSED: i32 = 0x00000040;
const wxACC_STATE_SYSTEM_DEFAULT: i32 = 0x00000080;
const wxACC_STATE_SYSTEM_EXPANDED: i32 = 0x00000100;
const wxACC_STATE_SYSTEM_EXTSELECTABLE: i32 = 0x00000200;
const wxACC_STATE_SYSTEM_FLOATING: i32 = 0x00000400;
const wxACC_STATE_SYSTEM_FOCUSABLE: i32 = 0x00000800;
const wxACC_STATE_SYSTEM_FOCUSED: i32 = 0x00001000;
const wxACC_STATE_SYSTEM_HOTTRACKED: i32 = 0x00002000;
const wxACC_STATE_SYSTEM_INVISIBLE: i32 = 0x00004000;
const wxACC_STATE_SYSTEM_MARQUEED: i32 = 0x00008000;
const wxACC_STATE_SYSTEM_MIXED: i32 = 0x00010000;
const wxACC_STATE_SYSTEM_MULTISELECTABLE: i32 = 0x00020000;
const wxACC_STATE_SYSTEM_OFFSCREEN: i32 = 0x00040000;
const wxACC_STATE_SYSTEM_PRESSED: i32 = 0x00080000;
const wxACC_STATE_SYSTEM_PROTECTED: i32 = 0x00100000;
const wxACC_STATE_SYSTEM_READONLY: i32 = 0x00200000;
const wxACC_STATE_SYSTEM_SELECTABLE: i32 = 0x00400000;
const wxACC_STATE_SYSTEM_SELECTED: i32 = 0x00800000;
const wxACC_STATE_SYSTEM_SELFVOICING: i32 = 0x01000000;
const wxACC_STATE_SYSTEM_UNAVAILABLE: i32 = 0x02000000;
const wxACC_EVENT_SYSTEM_SOUND: i32 = 0x0001;
const wxACC_EVENT_SYSTEM_ALERT: i32 = 0x0002;
const wxACC_EVENT_SYSTEM_FOREGROUND: i32 = 0x0003;
const wxACC_EVENT_SYSTEM_MENUSTART: i32 = 0x0004;
const wxACC_EVENT_SYSTEM_MENUEND: i32 = 0x0005;
const wxACC_EVENT_SYSTEM_MENUPOPUPSTART: i32 = 0x0006;
const wxACC_EVENT_SYSTEM_MENUPOPUPEND: i32 = 0x0007;
const wxACC_EVENT_SYSTEM_CAPTURESTART: i32 = 0x0008;
const wxACC_EVENT_SYSTEM_CAPTUREEND: i32 = 0x0009;
const wxACC_EVENT_SYSTEM_MOVESIZESTART: i32 = 0x000A;
const wxACC_EVENT_SYSTEM_MOVESIZEEND: i32 = 0x000B;
const wxACC_EVENT_SYSTEM_CONTEXTHELPSTART: i32 = 0x000C;
const wxACC_EVENT_SYSTEM_CONTEXTHELPEND: i32 = 0x000D;
const wxACC_EVENT_SYSTEM_DRAGDROPSTART: i32 = 0x000E;
const wxACC_EVENT_SYSTEM_DRAGDROPEND: i32 = 0x000F;
const wxACC_EVENT_SYSTEM_DIALOGSTART: i32 = 0x0010;
const wxACC_EVENT_SYSTEM_DIALOGEND: i32 = 0x0011;
const wxACC_EVENT_SYSTEM_SCROLLINGSTART: i32 = 0x0012;
const wxACC_EVENT_SYSTEM_SCROLLINGEND: i32 = 0x0013;
const wxACC_EVENT_SYSTEM_SWITCHSTART: i32 = 0x0014;
const wxACC_EVENT_SYSTEM_SWITCHEND: i32 = 0x0015;
const wxACC_EVENT_SYSTEM_MINIMIZESTART: i32 = 0x0016;
const wxACC_EVENT_SYSTEM_MINIMIZEEND: i32 = 0x0017;
const wxACC_EVENT_OBJECT_CREATE: i32 = 0x8000;
const wxACC_EVENT_OBJECT_DESTROY: i32 = 0x8001;
const wxACC_EVENT_OBJECT_SHOW: i32 = 0x8002;
const wxACC_EVENT_OBJECT_HIDE: i32 = 0x8003;
const wxACC_EVENT_OBJECT_REORDER: i32 = 0x8004;
const wxACC_EVENT_OBJECT_FOCUS: i32 = 0x8005;
const wxACC_EVENT_OBJECT_SELECTION: i32 = 0x8006;
const wxACC_EVENT_OBJECT_SELECTIONADD: i32 = 0x8007;
const wxACC_EVENT_OBJECT_SELECTIONREMOVE: i32 = 0x8008;
const wxACC_EVENT_OBJECT_SELECTIONWITHIN: i32 = 0x8009;
const wxACC_EVENT_OBJECT_STATECHANGE: i32 = 0x800A;
const wxACC_EVENT_OBJECT_LOCATIONCHANGE: i32 = 0x800B;
const wxACC_EVENT_OBJECT_NAMECHANGE: i32 = 0x800C;
const wxACC_EVENT_OBJECT_DESCRIPTIONCHANGE: i32 = 0x800D;
const wxACC_EVENT_OBJECT_VALUECHANGE: i32 = 0x800E;
const wxACC_EVENT_OBJECT_PARENTCHANGE: i32 = 0x800F;
const wxACC_EVENT_OBJECT_HELPCHANGE: i32 = 0x8010;
const wxACC_EVENT_OBJECT_DEFACTIONCHANGE: i32 = 0x8011;
const wxACC_EVENT_OBJECT_ACCELERATORCHANGE: i32 = 0x8012;
//  ENUM: wxAccStatus
const wxACC_FAIL: i32 = 0;
const wxACC_FALSE: i32 = 0 + 1;
const wxACC_OK: i32 = 0 + 2;
const wxACC_NOT_IMPLEMENTED: i32 = 0 + 3;
const wxACC_NOT_SUPPORTED: i32 = 0 + 4;
const wxACC_INVALID_ARG: i32 = 0 + 5;
//  ENUM: wxNavDir
const wxNAVDIR_FIRSTCHILD: i32 = 0;
const wxNAVDIR_LASTCHILD: i32 = 0 + 1;
const wxNAVDIR_DOWN: i32 = 0 + 2;
const wxNAVDIR_LEFT: i32 = 0 + 3;
const wxNAVDIR_NEXT: i32 = 0 + 4;
const wxNAVDIR_PREVIOUS: i32 = 0 + 5;
const wxNAVDIR_RIGHT: i32 = 0 + 6;
const wxNAVDIR_UP: i32 = 0 + 7;
//  ENUM: wxAccRole
const wxROLE_NONE: i32 = 0;
const wxROLE_SYSTEM_ALERT: i32 = 0 + 1;
const wxROLE_SYSTEM_ANIMATION: i32 = 0 + 2;
const wxROLE_SYSTEM_APPLICATION: i32 = 0 + 3;
const wxROLE_SYSTEM_BORDER: i32 = 0 + 4;
const wxROLE_SYSTEM_BUTTONDROPDOWN: i32 = 0 + 5;
const wxROLE_SYSTEM_BUTTONDROPDOWNGRID: i32 = 0 + 6;
const wxROLE_SYSTEM_BUTTONMENU: i32 = 0 + 7;
const wxROLE_SYSTEM_CARET: i32 = 0 + 8;
const wxROLE_SYSTEM_CELL: i32 = 0 + 9;
const wxROLE_SYSTEM_CHARACTER: i32 = 0 + 10;
const wxROLE_SYSTEM_CHART: i32 = 0 + 11;
const wxROLE_SYSTEM_CHECKBUTTON: i32 = 0 + 12;
const wxROLE_SYSTEM_CLIENT: i32 = 0 + 13;
const wxROLE_SYSTEM_CLOCK: i32 = 0 + 14;
const wxROLE_SYSTEM_COLUMN: i32 = 0 + 15;
const wxROLE_SYSTEM_COLUMNHEADER: i32 = 0 + 16;
const wxROLE_SYSTEM_COMBOBOX: i32 = 0 + 17;
const wxROLE_SYSTEM_CURSOR: i32 = 0 + 18;
const wxROLE_SYSTEM_DIAGRAM: i32 = 0 + 19;
const wxROLE_SYSTEM_DIAL: i32 = 0 + 20;
const wxROLE_SYSTEM_DIALOG: i32 = 0 + 21;
const wxROLE_SYSTEM_DOCUMENT: i32 = 0 + 22;
const wxROLE_SYSTEM_DROPLIST: i32 = 0 + 23;
const wxROLE_SYSTEM_EQUATION: i32 = 0 + 24;
const wxROLE_SYSTEM_GRAPHIC: i32 = 0 + 25;
const wxROLE_SYSTEM_GRIP: i32 = 0 + 26;
const wxROLE_SYSTEM_GROUPING: i32 = 0 + 27;
const wxROLE_SYSTEM_HELPBALLOON: i32 = 0 + 28;
const wxROLE_SYSTEM_HOTKEYFIELD: i32 = 0 + 29;
const wxROLE_SYSTEM_INDICATOR: i32 = 0 + 30;
const wxROLE_SYSTEM_LINK: i32 = 0 + 31;
const wxROLE_SYSTEM_LIST: i32 = 0 + 32;
const wxROLE_SYSTEM_LISTITEM: i32 = 0 + 33;
const wxROLE_SYSTEM_MENUBAR: i32 = 0 + 34;
const wxROLE_SYSTEM_MENUITEM: i32 = 0 + 35;
const wxROLE_SYSTEM_MENUPOPUP: i32 = 0 + 36;
const wxROLE_SYSTEM_OUTLINE: i32 = 0 + 37;
const wxROLE_SYSTEM_OUTLINEITEM: i32 = 0 + 38;
const wxROLE_SYSTEM_PAGETAB: i32 = 0 + 39;
const wxROLE_SYSTEM_PAGETABLIST: i32 = 0 + 40;
const wxROLE_SYSTEM_PANE: i32 = 0 + 41;
const wxROLE_SYSTEM_PROGRESSBAR: i32 = 0 + 42;
const wxROLE_SYSTEM_PROPERTYPAGE: i32 = 0 + 43;
const wxROLE_SYSTEM_PUSHBUTTON: i32 = 0 + 44;
const wxROLE_SYSTEM_RADIOBUTTON: i32 = 0 + 45;
const wxROLE_SYSTEM_ROW: i32 = 0 + 46;
const wxROLE_SYSTEM_ROWHEADER: i32 = 0 + 47;
const wxROLE_SYSTEM_SCROLLBAR: i32 = 0 + 48;
const wxROLE_SYSTEM_SEPARATOR: i32 = 0 + 49;
const wxROLE_SYSTEM_SLIDER: i32 = 0 + 50;
const wxROLE_SYSTEM_SOUND: i32 = 0 + 51;
const wxROLE_SYSTEM_SPINBUTTON: i32 = 0 + 52;
const wxROLE_SYSTEM_STATICTEXT: i32 = 0 + 53;
const wxROLE_SYSTEM_STATUSBAR: i32 = 0 + 54;
const wxROLE_SYSTEM_TABLE: i32 = 0 + 55;
const wxROLE_SYSTEM_TEXT: i32 = 0 + 56;
const wxROLE_SYSTEM_TITLEBAR: i32 = 0 + 57;
const wxROLE_SYSTEM_TOOLBAR: i32 = 0 + 58;
const wxROLE_SYSTEM_TOOLTIP: i32 = 0 + 59;
const wxROLE_SYSTEM_WHITESPACE: i32 = 0 + 60;
const wxROLE_SYSTEM_WINDOW: i32 = 0 + 61;
//  ENUM: wxAccObject
const wxOBJID_WINDOW: i32 =    0x00000000;
const wxOBJID_SYSMENU: i32 =   0xFFFFFFFF;
const wxOBJID_TITLEBAR: i32 =  0xFFFFFFFE;
const wxOBJID_MENU: i32 =      0xFFFFFFFD;
const wxOBJID_CLIENT: i32 =    0xFFFFFFFC;
const wxOBJID_VSCROLL: i32 =   0xFFFFFFFB;
const wxOBJID_HSCROLL: i32 =   0xFFFFFFFA;
const wxOBJID_SIZEGRIP: i32 =  0xFFFFFFF9;
const wxOBJID_CARET: i32 =     0xFFFFFFF8;
const wxOBJID_CURSOR: i32 =    0xFFFFFFF7;
const wxOBJID_ALERT: i32 =     0xFFFFFFF6;
const wxOBJID_SOUND: i32 =     0xFFFFFFF5;
//  ENUM: wxAccSelectionFlags
const wxACC_SEL_NONE: i32 = 0;
const wxACC_SEL_TAKEFOCUS: i32 = 1;
const wxACC_SEL_TAKESELECTION: i32 = 2;
const wxACC_SEL_EXTENDSELECTION: i32 = 4;
const wxACC_SEL_ADDSELECTION: i32 = 8;
const wxACC_SEL_REMOVESELECTION: i32 = 16;

//  ENUM: ResourceCat
const ResourceCat_None: i32 = 0;
const ResourceCat_Messages: i32 = 0 + 1;
//  ENUM: Dir
const Dir_Cache: i32 = 0;
const Dir_Documents: i32 = 0 + 1;
const Dir_Desktop: i32 = 0 + 2;
const Dir_Downloads: i32 = 0 + 3;
const Dir_Music: i32 = 0 + 4;
const Dir_Pictures: i32 = 0 + 5;
const Dir_Videos: i32 = 0 + 6;
//  ENUM: FileLayout
const FileLayout_Classic: i32 = 0;
const FileLayout_XDG: i32 = 0 + 1;
//  ENUM: ConfigFileConv
const ConfigFileConv_Dot: i32 = 0;
const ConfigFileConv_Ext: i32 = 0 + 1;

// NODEF: wxLongLongFmtSpec

const wxCLRP_USE_TEXTCTRL: i32 = (wxPB_USE_TEXTCTRL);
const wxCLRP_DEFAULT_STYLE: i32 = 0;
const wxCLRP_SHOW_LABEL: i32 = 0x0008;
const wxCLRP_SHOW_ALPHA: i32 = 0x0010;

//  ENUM: wxHtmlURLType
const wxHTML_URL_PAGE: i32 = 0;
const wxHTML_URL_IMAGE: i32 = 0 + 1;
const wxHTML_URL_OTHER: i32 = 0 + 2;

//  ENUM: NumericType
const Signed: i32 = 0;
const Unsigned: i32 = 0 + 1;
const Float: i32 = 0 + 2;

//  ENUM: wxToolBarToolStyle
const wxTOOL_STYLE_BUTTON: i32 = 1;
const wxTOOL_STYLE_SEPARATOR: i32 = 2;
const wxTOOL_STYLE_CONTROL: i32 = 2 + 1;
//  ENUM: @49
const wxTB_HORIZONTAL: i32 = wxHORIZONTAL;
const wxTB_TOP: i32 = wxTB_HORIZONTAL;
const wxTB_VERTICAL: i32 = wxVERTICAL;
const wxTB_LEFT: i32 = wxTB_VERTICAL;
const wxTB_FLAT: i32 = wxTB_VERTICAL + 1;
const wxTB_DOCKABLE: i32 = wxTB_VERTICAL + 2;
const wxTB_NOICONS: i32 = wxTB_VERTICAL + 3;
const wxTB_TEXT: i32 = wxTB_VERTICAL + 4;
const wxTB_NODIVIDER: i32 = wxTB_VERTICAL + 5;
const wxTB_NOALIGN: i32 = wxTB_VERTICAL + 6;
const wxTB_HORZ_LAYOUT: i32 = wxTB_VERTICAL + 7;
const wxTB_HORZ_TEXT: i32 = wxTB_HORZ_LAYOUT | wxTB_TEXT;
const wxTB_NO_TOOLTIPS: i32 = wxTB_HORZ_LAYOUT | wxTB_TEXT + 1;
const wxTB_BOTTOM: i32 = wxTB_HORZ_LAYOUT | wxTB_TEXT + 2;
const wxTB_RIGHT: i32 = wxTB_HORZ_LAYOUT | wxTB_TEXT + 3;
const wxTB_DEFAULT_STYLE: i32 = wxTB_HORIZONTAL;

//  ENUM: wxAuiNotebookOption
const wxAUI_NB_TOP: i32 = 1 << 0;
const wxAUI_NB_LEFT: i32 = 1 << 1;
const wxAUI_NB_RIGHT: i32 = 1 << 2;
const wxAUI_NB_BOTTOM: i32 = 1 << 3;
const wxAUI_NB_TAB_SPLIT: i32 = 1 << 4;
const wxAUI_NB_TAB_MOVE: i32 = 1 << 5;
const wxAUI_NB_TAB_EXTERNAL_MOVE: i32 = 1 << 6;
const wxAUI_NB_TAB_FIXED_WIDTH: i32 = 1 << 7;
const wxAUI_NB_SCROLL_BUTTONS: i32 = 1 << 8;
const wxAUI_NB_WINDOWLIST_BUTTON: i32 = 1 << 9;
const wxAUI_NB_CLOSE_BUTTON: i32 = 1 << 10;
const wxAUI_NB_CLOSE_ON_ACTIVE_TAB: i32 = 1 << 11;
const wxAUI_NB_CLOSE_ON_ALL_TABS: i32 = 1 << 12;
const wxAUI_NB_MIDDLE_CLICK_CLOSE: i32 = 1 << 13;
const wxAUI_NB_DEFAULT_STYLE: i32 = wxAUI_NB_TOP |
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
const wxCOND_NO_ERROR: i32 = 0;
const wxCOND_INVALID: i32 = 0 + 1;
const wxCOND_TIMEOUT: i32 = 0 + 2;
const wxCOND_MISC_ERROR: i32 = 0 + 3;
//  ENUM: wxCriticalSectionType
const wxCRITSEC_DEFAULT: i32 = 0;
const wxCRITSEC_NON_RECURSIVE: i32 = 0 + 1;
//  ENUM: wxThreadWait
const wxTHREAD_WAIT_BLOCK: i32 = 0;
const wxTHREAD_WAIT_YIELD: i32 = 0 + 1;
const wxTHREAD_WAIT_DEFAULT: i32 = wxTHREAD_WAIT_YIELD;
//  ENUM: wxThreadKind
const wxTHREAD_DETACHED: i32 = 0;
const wxTHREAD_JOINABLE: i32 = 0 + 1;
//  ENUM: wxThreadError
const wxTHREAD_NO_ERROR: i32 = 0;
const wxTHREAD_NO_RESOURCE: i32 = 0 + 1;
const wxTHREAD_RUNNING: i32 = 0 + 2;
const wxTHREAD_NOT_RUNNING: i32 = 0 + 3;
const wxTHREAD_KILLED: i32 = 0 + 4;
const wxTHREAD_MISC_ERROR: i32 = 0 + 5;
//  ENUM: wxSemaError
const wxSEMA_NO_ERROR: i32 = 0;
const wxSEMA_INVALID: i32 = 0 + 1;
const wxSEMA_BUSY: i32 = 0 + 2;
const wxSEMA_TIMEOUT: i32 = 0 + 3;
const wxSEMA_OVERFLOW: i32 = 0 + 4;
const wxSEMA_MISC_ERROR: i32 = 0 + 5;
//  ENUM: wxMutexType
const wxMUTEX_DEFAULT: i32 = 0;
const wxMUTEX_RECURSIVE: i32 = 0 + 1;
//  ENUM: wxMutexError
const wxMUTEX_NO_ERROR: i32 = 0;
const wxMUTEX_INVALID: i32 = 0 + 1;
const wxMUTEX_DEAD_LOCK: i32 = 0 + 2;
const wxMUTEX_BUSY: i32 = 0 + 3;
const wxMUTEX_UNLOCKED: i32 = 0 + 4;
const wxMUTEX_TIMEOUT: i32 = 0 + 5;
const wxMUTEX_MISC_ERROR: i32 = 0 + 6;

//  ENUM: wxMessageOutputFlags
const wxMSGOUT_PREFER_STDERR: i32 = 0;
const wxMSGOUT_PREFER_MSGBOX: i32 = 1;

const wxSTACKWALKER_MAX_DEPTH: i32 = (200);

const wxRE_READONLY: i32 = 0x0010;
const wxRE_MULTILINE: i32 = 0x0020;
const wxRE_CENTRE_CARET: i32 = 0x8000;
const wxRE_CENTER_CARET: i32 = wxRE_CENTRE_CARET;
const wxRICHTEXT_SHIFT_DOWN: i32 = 0x01;
const wxRICHTEXT_CTRL_DOWN: i32 = 0x02;
const wxRICHTEXT_ALT_DOWN: i32 = 0x04;
const wxRICHTEXT_EX_NO_GUIDELINES: i32 = 0x00000100;
//  SKIP: wxRICHTEXT_DEFAULT_OVERALL_SIZE
//  SKIP: wxRICHTEXT_DEFAULT_IMAGE_SIZE
const wxRICHTEXT_DEFAULT_SPACING: i32 = 3;
const wxRICHTEXT_DEFAULT_MARGIN: i32 = 3;
//  SKIP: wxRICHTEXT_DEFAULT_UNFOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_FOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_UNSELECTED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_TYPE_COLOUR
//  SKIP: wxRICHTEXT_DEFAULT_FOCUS_RECT_COLOUR
const wxRICHTEXT_DEFAULT_CARET_WIDTH: i32 = 2;
const wxRICHTEXT_DEFAULT_DELAYED_LAYOUT_THRESHOLD: i32 = 20000;
const wxRICHTEXT_DEFAULT_LAYOUT_INTERVAL: i32 = 50;
const wxRICHTEXT_DEFAULT_DELAYED_IMAGE_PROCESSING_INTERVAL: i32 = 200;
const wxID_RICHTEXT_PROPERTIES1: i32 = (wxID_HIGHEST + 1);
const wxID_RICHTEXT_PROPERTIES2: i32 = (wxID_HIGHEST + 2);
const wxID_RICHTEXT_PROPERTIES3: i32 = (wxID_HIGHEST + 3);
//  ENUM: wxRichTextCtrlSelectionState
const wxRichTextCtrlSelectionState_Normal: i32 = 0;
const wxRichTextCtrlSelectionState_CommonAncestor: i32 = 0 + 1;

const wxSIZE_AUTO_WIDTH: i32 = 0x0001;
const wxSIZE_AUTO_HEIGHT: i32 = 0x0002;
const wxSIZE_AUTO: i32 = (wxSIZE_AUTO_WIDTH|wxSIZE_AUTO_HEIGHT);
const wxSIZE_USE_EXISTING: i32 = 0x0000;
const wxSIZE_ALLOW_MINUS_ONE: i32 = 0x0004;
const wxSIZE_NO_ADJUSTMENTS: i32 = 0x0008;
const wxSIZE_FORCE: i32 = 0x0010;
const wxSIZE_FORCE_EVENT: i32 = 0x0020;
const wxVSCROLL: i32 = 0x80000000;
const wxHSCROLL: i32 = 0x40000000;
const wxCAPTION: i32 = 0x20000000;
const wxDOUBLE_BORDER: i32 = wxBORDER_DOUBLE;
const wxSUNKEN_BORDER: i32 = wxBORDER_SUNKEN;
const wxRAISED_BORDER: i32 = wxBORDER_RAISED;
const wxBORDER: i32 = wxBORDER_SIMPLE;
const wxSIMPLE_BORDER: i32 = wxBORDER_SIMPLE;
const wxSTATIC_BORDER: i32 = wxBORDER_STATIC;
const wxNO_BORDER: i32 = wxBORDER_NONE;
const wxALWAYS_SHOW_SB: i32 = 0x00800000;
const wxCLIP_CHILDREN: i32 = 0x00400000;
const wxCLIP_SIBLINGS: i32 = 0x20000000;
const wxTRANSPARENT_WINDOW: i32 = 0x00100000;
const wxTAB_TRAVERSAL: i32 = 0x00080000;
const wxWANTS_CHARS: i32 = 0x00040000;
const wxRETAINED: i32 = 0x00000000;
const wxBACKINGSTORE: i32 = wxRETAINED;
const wxPOPUP_WINDOW: i32 = 0x00020000;
const wxFULL_REPAINT_ON_RESIZE: i32 = 0x00010000;
const wxNO_FULL_REPAINT_ON_RESIZE: i32 = 0;
const wxWINDOW_STYLE_MASK: i32 =     (wxVSCROLL|wxHSCROLL|wxBORDER_MASK|wxALWAYS_SHOW_SB|wxCLIP_CHILDREN| \
     wxCLIP_SIBLINGS|wxTRANSPARENT_WINDOW|wxTAB_TRAVERSAL|wxWANTS_CHARS| \
     wxRETAINED|wxPOPUP_WINDOW|wxFULL_REPAINT_ON_RESIZE);
const wxWS_EX_BLOCK_EVENTS: i32 = 0x00000002;
const wxWS_EX_TRANSIENT: i32 = 0x00000004;
const wxWS_EX_THEMED_BACKGROUND: i32 = 0x00000008;
const wxWS_EX_PROCESS_IDLE: i32 = 0x00000010;
const wxWS_EX_PROCESS_UI_UPDATES: i32 = 0x00000020;
const wxFRAME_EX_METAL: i32 = 0x00000040;
const wxDIALOG_EX_METAL: i32 = 0x00000040;
const wxWS_EX_CONTEXTHELP: i32 = 0x00000080;
const wxFRAME_EX_CONTEXTHELP: i32 = wxWS_EX_CONTEXTHELP;
const wxDIALOG_EX_CONTEXTHELP: i32 = wxWS_EX_CONTEXTHELP;
const wxFRAME_DRAWER: i32 = 0x0020;
const wxFRAME_NO_WINDOW_MENU: i32 = 0x0100;
const wxMB_DOCKABLE: i32 = 0x0001;
const wxMENU_TEAROFF: i32 = 0x0001;
const wxCOLOURED: i32 = 0x0800;
const wxFIXED_LENGTH: i32 = 0x0400;
const wxLB_SORT: i32 = 0x0010;
const wxLB_SINGLE: i32 = 0x0020;
const wxLB_MULTIPLE: i32 = 0x0040;
const wxLB_EXTENDED: i32 = 0x0080;
const wxLB_NEEDED_SB: i32 = 0x0000;
const wxLB_OWNERDRAW: i32 = 0x0100;
const wxLB_ALWAYS_SB: i32 = 0x0200;
const wxLB_NO_SB: i32 = 0x0400;
const wxLB_HSCROLL: i32 = wxHSCROLL;
const wxLB_INT_HEIGHT: i32 = 0x0800;
const wxCB_SIMPLE: i32 = 0x0004;
const wxCB_SORT: i32 = 0x0008;
const wxCB_READONLY: i32 = 0x0010;
const wxCB_DROPDOWN: i32 = 0x0020;
const wxRA_LEFTTORIGHT: i32 = 0x0001;
const wxRA_TOPTOBOTTOM: i32 = 0x0002;
const wxRA_SPECIFY_COLS: i32 = wxHORIZONTAL;
const wxRA_SPECIFY_ROWS: i32 = wxVERTICAL;
const wxRA_HORIZONTAL: i32 = wxHORIZONTAL;
const wxRA_VERTICAL: i32 = wxVERTICAL;
const wxRB_GROUP: i32 = 0x0004;
const wxRB_SINGLE: i32 = 0x0008;
const wxSB_HORIZONTAL: i32 = wxHORIZONTAL;
const wxSB_VERTICAL: i32 = wxVERTICAL;
const wxSP_HORIZONTAL: i32 = wxHORIZONTAL /*  4 */;
const wxSP_VERTICAL: i32 = wxVERTICAL   /*  8 */;
const wxSP_ARROW_KEYS: i32 = 0x4000;
const wxSP_WRAP: i32 = 0x8000;
const wxTC_RIGHTJUSTIFY: i32 = 0x0010;
const wxTC_FIXEDWIDTH: i32 = 0x0020;
const wxTC_TOP: i32 = 0x0000    /*  default */;
const wxTC_LEFT: i32 = 0x0020;
const wxTC_RIGHT: i32 = 0x0040;
const wxTC_BOTTOM: i32 = 0x0080;
const wxTC_MULTILINE: i32 = 0x0200    /* == wxNB_MULTILINE */;
const wxTC_OWNERDRAW: i32 = 0x0400;
const wxBI_EXPAND: i32 = wxEXPAND;
const wxLI_HORIZONTAL: i32 = wxHORIZONTAL;
const wxLI_VERTICAL: i32 = wxVERTICAL;
const wxYES: i32 = 0x00000002;
const wxOK: i32 = 0x00000004;
const wxNO: i32 = 0x00000008;
const wxYES_NO: i32 = (wxYES | wxNO);
const wxCANCEL: i32 = 0x00000010;
const wxAPPLY: i32 = 0x00000020;
const wxCLOSE: i32 = 0x00000040;
const wxOK_DEFAULT: i32 = 0x00000000  /* has no effect (default) */;
const wxYES_DEFAULT: i32 = 0x00000000  /* has no effect (default) */;
const wxNO_DEFAULT: i32 = 0x00000080  /* only valid with wxYES_NO */;
const wxCANCEL_DEFAULT: i32 = 0x80000000  /* only valid with wxCANCEL */;
const wxICON_EXCLAMATION: i32 = 0x00000100;
const wxICON_HAND: i32 = 0x00000200;
const wxICON_WARNING: i32 = wxICON_EXCLAMATION;
const wxICON_ERROR: i32 = wxICON_HAND;
const wxICON_QUESTION: i32 = 0x00000400;
const wxICON_INFORMATION: i32 = 0x00000800;
const wxICON_STOP: i32 = wxICON_HAND;
const wxICON_ASTERISK: i32 = wxICON_INFORMATION;
const wxHELP: i32 = 0x00001000;
const wxFORWARD: i32 = 0x00002000;
const wxBACKWARD: i32 = 0x00004000;
const wxRESET: i32 = 0x00008000;
const wxMORE: i32 = 0x00010000;
const wxSETUP: i32 = 0x00020000;
const wxICON_NONE: i32 = 0x00040000;
const wxICON_AUTH_NEEDED: i32 = 0x00080000;
const wxICON_MASK: i32 =     (wxICON_EXCLAMATION|wxICON_HAND|wxICON_QUESTION|wxICON_INFORMATION|wxICON_NONE);
const wxNOT_FOUND: i32 = (-1);
const wxPRINT_QUALITY_HIGH: i32 = -1;
const wxPRINT_QUALITY_MEDIUM: i32 = -2;
const wxPRINT_QUALITY_LOW: i32 = -3;
const wxPRINT_QUALITY_DRAFT: i32 = -4;
const wxSTAY_ON_TOP: i32 = 0x8000;
const wxICONIZE: i32 = 0x4000;
const wxMINIMIZE: i32 = wxICONIZE;
const wxMAXIMIZE: i32 = 0x2000;
const wxCLOSE_BOX: i32 = 0x1000;
const wxSYSTEM_MENU: i32 = 0x0800;
const wxMINIMIZE_BOX: i32 = 0x0400;
const wxMAXIMIZE_BOX: i32 = 0x0200;
const wxTINY_CAPTION: i32 = 0x0080;
const wxRESIZE_BORDER: i32 = 0x0040;
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
const wxCENTRE: i32 = 0x0001;
const wxCENTER: i32 = wxCENTRE;
//  ENUM: wxOrientation
const wxHORIZONTAL: i32 = 0x0004;
const wxVERTICAL: i32 = 0x0008;
const wxBOTH: i32 = wxVERTICAL | wxHORIZONTAL;
const wxORIENTATION_MASK: i32 = wxBOTH;
//  ENUM: wxDirection
const wxLEFT: i32 = 0x0010;
const wxRIGHT: i32 = 0x0020;
const wxUP: i32 = 0x0040;
const wxDOWN: i32 = 0x0080;
const wxTOP: i32 = wxUP;
const wxBOTTOM: i32 = wxDOWN;
const wxNORTH: i32 = wxUP;
const wxSOUTH: i32 = wxDOWN;
const wxWEST: i32 = wxLEFT;
const wxEAST: i32 = wxRIGHT;
const wxALL: i32 = (wxUP | wxDOWN | wxRIGHT | wxLEFT);
const wxDIRECTION_MASK: i32 = wxALL;
//  ENUM: wxAlignment
const wxALIGN_INVALID: i32 = -1;
const wxALIGN_NOT: i32 = 0x0000;
const wxALIGN_CENTER_HORIZONTAL: i32 = 0x0100;
const wxALIGN_CENTRE_HORIZONTAL: i32 = wxALIGN_CENTER_HORIZONTAL;
const wxALIGN_LEFT: i32 = wxALIGN_NOT;
const wxALIGN_TOP: i32 = wxALIGN_NOT;
const wxALIGN_RIGHT: i32 = 0x0200;
const wxALIGN_BOTTOM: i32 = 0x0400;
const wxALIGN_CENTER_VERTICAL: i32 = 0x0800;
const wxALIGN_CENTRE_VERTICAL: i32 = wxALIGN_CENTER_VERTICAL;
const wxALIGN_CENTER: i32 = (wxALIGN_CENTER_HORIZONTAL | wxALIGN_CENTER_VERTICAL);
const wxALIGN_CENTRE: i32 = wxALIGN_CENTER;
const wxALIGN_MASK: i32 = 0x0f00;
//  ENUM: wxSizerFlagBits
const wxFIXED_MINSIZE: i32 = 0x8000;
const wxRESERVE_SPACE_EVEN_IF_HIDDEN: i32 = 0x0002;
const wxSIZER_FLAG_BITS_MASK: i32 = 0x8002;
//  ENUM: wxStretch
const wxSTRETCH_NOT: i32 = 0x0000;
const wxSHRINK: i32 = 0x1000;
const wxGROW: i32 = 0x2000;
const wxEXPAND: i32 = wxGROW;
const wxSHAPED: i32 = 0x4000;
const wxTILE: i32 = wxSHAPED | wxFIXED_MINSIZE;
const wxSTRETCH_MASK: i32 = 0x7000;
//  ENUM: wxBorder
const wxBORDER_DEFAULT: i32 = 0;
const wxBORDER_NONE: i32 = 0x00200000;
const wxBORDER_STATIC: i32 = 0x01000000;
const wxBORDER_SIMPLE: i32 = 0x02000000;
const wxBORDER_RAISED: i32 = 0x04000000;
const wxBORDER_SUNKEN: i32 = 0x08000000;
const wxBORDER_DOUBLE: i32 = 0x10000000;
const wxBORDER_THEME: i32 = wxBORDER_DOUBLE;
const wxBORDER_MASK: i32 = 0x1f200000;
//  ENUM: wxBackgroundStyle
const wxBG_STYLE_ERASE: i32 = 0;
const wxBG_STYLE_SYSTEM: i32 = 0 + 1;
const wxBG_STYLE_PAINT: i32 = 0 + 2;
const wxBG_STYLE_COLOUR: i32 = 0 + 3;
const wxBG_STYLE_TRANSPARENT: i32 = 0 + 4;
//  ENUM: wxStandardID
const wxID_AUTO_LOWEST: i32 = 0;
const wxID_AUTO_HIGHEST: i32 = 0 + 1;
const wxID_NONE: i32 = -3;
const wxID_SEPARATOR: i32 = -2;
const wxID_ANY: i32 = -1;
const wxID_LOWEST: i32 = 4999;
const wxID_OPEN: i32 = 4999 + 1;
const wxID_CLOSE: i32 = 4999 + 2;
const wxID_NEW: i32 = 4999 + 3;
const wxID_SAVE: i32 = 4999 + 4;
const wxID_SAVEAS: i32 = 4999 + 5;
const wxID_REVERT: i32 = 4999 + 6;
const wxID_EXIT: i32 = 4999 + 7;
const wxID_UNDO: i32 = 4999 + 8;
const wxID_REDO: i32 = 4999 + 9;
const wxID_HELP: i32 = 4999 + 10;
const wxID_PRINT: i32 = 4999 + 11;
const wxID_PRINT_SETUP: i32 = 4999 + 12;
const wxID_PAGE_SETUP: i32 = 4999 + 13;
const wxID_PREVIEW: i32 = 4999 + 14;
const wxID_ABOUT: i32 = 4999 + 15;
const wxID_HELP_CONTENTS: i32 = 4999 + 16;
const wxID_HELP_INDEX: i32 = 4999 + 17;
const wxID_HELP_SEARCH: i32 = 4999 + 18;
const wxID_HELP_COMMANDS: i32 = 4999 + 19;
const wxID_HELP_PROCEDURES: i32 = 4999 + 20;
const wxID_HELP_CONTEXT: i32 = 4999 + 21;
const wxID_CLOSE_ALL: i32 = 4999 + 22;
const wxID_PREFERENCES: i32 = 4999 + 23;
const wxID_EDIT: i32 = 5030;
const wxID_CUT: i32 = 5030 + 1;
const wxID_COPY: i32 = 5030 + 2;
const wxID_PASTE: i32 = 5030 + 3;
const wxID_CLEAR: i32 = 5030 + 4;
const wxID_FIND: i32 = 5030 + 5;
const wxID_DUPLICATE: i32 = 5030 + 6;
const wxID_SELECTALL: i32 = 5030 + 7;
const wxID_DELETE: i32 = 5030 + 8;
const wxID_REPLACE: i32 = 5030 + 9;
const wxID_REPLACE_ALL: i32 = 5030 + 10;
const wxID_PROPERTIES: i32 = 5030 + 11;
const wxID_VIEW_DETAILS: i32 = 5030 + 12;
const wxID_VIEW_LARGEICONS: i32 = 5030 + 13;
const wxID_VIEW_SMALLICONS: i32 = 5030 + 14;
const wxID_VIEW_LIST: i32 = 5030 + 15;
const wxID_VIEW_SORTDATE: i32 = 5030 + 16;
const wxID_VIEW_SORTNAME: i32 = 5030 + 17;
const wxID_VIEW_SORTSIZE: i32 = 5030 + 18;
const wxID_VIEW_SORTTYPE: i32 = 5030 + 19;
const wxID_FILE: i32 = 5050;
const wxID_FILE1: i32 = 5050 + 1;
const wxID_FILE2: i32 = 5050 + 2;
const wxID_FILE3: i32 = 5050 + 3;
const wxID_FILE4: i32 = 5050 + 4;
const wxID_FILE5: i32 = 5050 + 5;
const wxID_FILE6: i32 = 5050 + 6;
const wxID_FILE7: i32 = 5050 + 7;
const wxID_FILE8: i32 = 5050 + 8;
const wxID_FILE9: i32 = 5050 + 9;
const wxID_OK: i32 = 5100;
const wxID_CANCEL: i32 = 5100 + 1;
const wxID_APPLY: i32 = 5100 + 2;
const wxID_YES: i32 = 5100 + 3;
const wxID_NO: i32 = 5100 + 4;
const wxID_STATIC: i32 = 5100 + 5;
const wxID_FORWARD: i32 = 5100 + 6;
const wxID_BACKWARD: i32 = 5100 + 7;
const wxID_DEFAULT: i32 = 5100 + 8;
const wxID_MORE: i32 = 5100 + 9;
const wxID_SETUP: i32 = 5100 + 10;
const wxID_RESET: i32 = 5100 + 11;
const wxID_CONTEXT_HELP: i32 = 5100 + 12;
const wxID_YESTOALL: i32 = 5100 + 13;
const wxID_NOTOALL: i32 = 5100 + 14;
const wxID_ABORT: i32 = 5100 + 15;
const wxID_RETRY: i32 = 5100 + 16;
const wxID_IGNORE: i32 = 5100 + 17;
const wxID_ADD: i32 = 5100 + 18;
const wxID_REMOVE: i32 = 5100 + 19;
const wxID_UP: i32 = 5100 + 20;
const wxID_DOWN: i32 = 5100 + 21;
const wxID_HOME: i32 = 5100 + 22;
const wxID_REFRESH: i32 = 5100 + 23;
const wxID_STOP: i32 = 5100 + 24;
const wxID_INDEX: i32 = 5100 + 25;
const wxID_BOLD: i32 = 5100 + 26;
const wxID_ITALIC: i32 = 5100 + 27;
const wxID_JUSTIFY_CENTER: i32 = 5100 + 28;
const wxID_JUSTIFY_FILL: i32 = 5100 + 29;
const wxID_JUSTIFY_RIGHT: i32 = 5100 + 30;
const wxID_JUSTIFY_LEFT: i32 = 5100 + 31;
const wxID_UNDERLINE: i32 = 5100 + 32;
const wxID_INDENT: i32 = 5100 + 33;
const wxID_UNINDENT: i32 = 5100 + 34;
const wxID_ZOOM_100: i32 = 5100 + 35;
const wxID_ZOOM_FIT: i32 = 5100 + 36;
const wxID_ZOOM_IN: i32 = 5100 + 37;
const wxID_ZOOM_OUT: i32 = 5100 + 38;
const wxID_UNDELETE: i32 = 5100 + 39;
const wxID_REVERT_TO_SAVED: i32 = 5100 + 40;
const wxID_CDROM: i32 = 5100 + 41;
const wxID_CONVERT: i32 = 5100 + 42;
const wxID_EXECUTE: i32 = 5100 + 43;
const wxID_FLOPPY: i32 = 5100 + 44;
const wxID_HARDDISK: i32 = 5100 + 45;
const wxID_BOTTOM: i32 = 5100 + 46;
const wxID_FIRST: i32 = 5100 + 47;
const wxID_LAST: i32 = 5100 + 48;
const wxID_TOP: i32 = 5100 + 49;
const wxID_INFO: i32 = 5100 + 50;
const wxID_JUMP_TO: i32 = 5100 + 51;
const wxID_NETWORK: i32 = 5100 + 52;
const wxID_SELECT_COLOR: i32 = 5100 + 53;
const wxID_SELECT_FONT: i32 = 5100 + 54;
const wxID_SORT_ASCENDING: i32 = 5100 + 55;
const wxID_SORT_DESCENDING: i32 = 5100 + 56;
const wxID_SPELL_CHECK: i32 = 5100 + 57;
const wxID_STRIKETHROUGH: i32 = 5100 + 58;
const wxID_SYSTEM_MENU: i32 = 5200;
const wxID_CLOSE_FRAME: i32 = 5200 + 1;
const wxID_MOVE_FRAME: i32 = 5200 + 2;
const wxID_RESIZE_FRAME: i32 = 5200 + 3;
const wxID_MAXIMIZE_FRAME: i32 = 5200 + 4;
const wxID_ICONIZE_FRAME: i32 = 5200 + 5;
const wxID_RESTORE_FRAME: i32 = 5200 + 6;
const wxID_MDI_WINDOW_FIRST: i32 = 5230;
const wxID_MDI_WINDOW_CASCADE: i32 = wxID_MDI_WINDOW_FIRST;
const wxID_MDI_WINDOW_TILE_HORZ: i32 = wxID_MDI_WINDOW_FIRST + 1;
const wxID_MDI_WINDOW_TILE_VERT: i32 = wxID_MDI_WINDOW_FIRST + 2;
const wxID_MDI_WINDOW_ARRANGE_ICONS: i32 = wxID_MDI_WINDOW_FIRST + 3;
const wxID_MDI_WINDOW_PREV: i32 = wxID_MDI_WINDOW_FIRST + 4;
const wxID_MDI_WINDOW_NEXT: i32 = wxID_MDI_WINDOW_FIRST + 5;
const wxID_MDI_WINDOW_LAST: i32 = wxID_MDI_WINDOW_NEXT;
const wxID_FILEDLGG: i32 = 5900;
const wxID_FILECTRL: i32 = 5950;
const wxID_HIGHEST: i32 = 5999;
//  ENUM: wxItemKind
const wxITEM_SEPARATOR: i32 = -1;
const wxITEM_NORMAL: i32 = -1 + 1;
const wxITEM_CHECK: i32 = -1 + 2;
const wxITEM_RADIO: i32 = -1 + 3;
const wxITEM_DROPDOWN: i32 = -1 + 4;
const wxITEM_MAX: i32 = -1 + 5;
//  ENUM: wxHitTest
const wxHT_NOWHERE: i32 = 0;
const wxHT_SCROLLBAR_FIRST: i32 = wxHT_NOWHERE;
const wxHT_SCROLLBAR_ARROW_LINE_1: i32 = wxHT_NOWHERE + 1;
const wxHT_SCROLLBAR_ARROW_LINE_2: i32 = wxHT_NOWHERE + 2;
const wxHT_SCROLLBAR_ARROW_PAGE_1: i32 = wxHT_NOWHERE + 3;
const wxHT_SCROLLBAR_ARROW_PAGE_2: i32 = wxHT_NOWHERE + 4;
const wxHT_SCROLLBAR_THUMB: i32 = wxHT_NOWHERE + 5;
const wxHT_SCROLLBAR_BAR_1: i32 = wxHT_NOWHERE + 6;
const wxHT_SCROLLBAR_BAR_2: i32 = wxHT_NOWHERE + 7;
const wxHT_SCROLLBAR_LAST: i32 = wxHT_NOWHERE + 8;
const wxHT_WINDOW_OUTSIDE: i32 = wxHT_NOWHERE + 9;
const wxHT_WINDOW_INSIDE: i32 = wxHT_NOWHERE + 10;
const wxHT_WINDOW_VERT_SCROLLBAR: i32 = wxHT_NOWHERE + 11;
const wxHT_WINDOW_HORZ_SCROLLBAR: i32 = wxHT_NOWHERE + 12;
const wxHT_WINDOW_CORNER: i32 = wxHT_NOWHERE + 13;
const wxHT_MAX: i32 = wxHT_NOWHERE + 14;
//  ENUM: wxDataFormatId
const wxDF_INVALID: i32 =          0;
const wxDF_TEXT: i32 =             1;
const wxDF_BITMAP: i32 =           2;
const wxDF_METAFILE: i32 =         3;
const wxDF_SYLK: i32 =             4;
const wxDF_DIF: i32 =              5;
const wxDF_TIFF: i32 =             6;
const wxDF_OEMTEXT: i32 =          7;
const wxDF_DIB: i32 =              8;
const wxDF_PALETTE: i32 =          9;
const wxDF_PENDATA: i32 =          10;
const wxDF_RIFF: i32 =             11;
const wxDF_WAVE: i32 =             12;
const wxDF_UNICODETEXT: i32 =      13;
const wxDF_ENHMETAFILE: i32 =      14;
const wxDF_FILENAME: i32 =         15;
const wxDF_LOCALE: i32 =           16;
const wxDF_PRIVATE: i32 =          20;
const wxDF_HTML: i32 =             30;
const wxDF_PNG: i32 =              31;
const wxDF_MAX: i32 =              31 + 1;
//  ENUM: wxKeyCode
const WXK_NONE: i32 =    0;
const WXK_CONTROL_A: i32 = 1;
const WXK_CONTROL_B: i32 = 1 + 1;
const WXK_CONTROL_C: i32 = 1 + 2;
const WXK_CONTROL_D: i32 = 1 + 3;
const WXK_CONTROL_E: i32 = 1 + 4;
const WXK_CONTROL_F: i32 = 1 + 5;
const WXK_CONTROL_G: i32 = 1 + 6;
const WXK_CONTROL_H: i32 = 1 + 7;
const WXK_CONTROL_I: i32 = 1 + 8;
const WXK_CONTROL_J: i32 = 1 + 9;
const WXK_CONTROL_K: i32 = 1 + 10;
const WXK_CONTROL_L: i32 = 1 + 11;
const WXK_CONTROL_M: i32 = 1 + 12;
const WXK_CONTROL_N: i32 = 1 + 13;
const WXK_CONTROL_O: i32 = 1 + 14;
const WXK_CONTROL_P: i32 = 1 + 15;
const WXK_CONTROL_Q: i32 = 1 + 16;
const WXK_CONTROL_R: i32 = 1 + 17;
const WXK_CONTROL_S: i32 = 1 + 18;
const WXK_CONTROL_T: i32 = 1 + 19;
const WXK_CONTROL_U: i32 = 1 + 20;
const WXK_CONTROL_V: i32 = 1 + 21;
const WXK_CONTROL_W: i32 = 1 + 22;
const WXK_CONTROL_X: i32 = 1 + 23;
const WXK_CONTROL_Y: i32 = 1 + 24;
const WXK_CONTROL_Z: i32 = 1 + 25;
const WXK_BACK: i32 =    8;
const WXK_TAB: i32 =    9;
const WXK_RETURN: i32 =    13;
const WXK_ESCAPE: i32 =    27;
const WXK_SPACE: i32 =    32;
const WXK_DELETE: i32 =    127;
const WXK_START: i32 = 300;
const WXK_LBUTTON: i32 = 300 + 1;
const WXK_RBUTTON: i32 = 300 + 2;
const WXK_CANCEL: i32 = 300 + 3;
const WXK_MBUTTON: i32 = 300 + 4;
const WXK_CLEAR: i32 = 300 + 5;
const WXK_SHIFT: i32 = 300 + 6;
const WXK_ALT: i32 = 300 + 7;
const WXK_CONTROL: i32 = 300 + 8;
const WXK_RAW_CONTROL: i32 = 300 + 9;
const WXK_MENU: i32 = 300 + 10;
const WXK_PAUSE: i32 = 300 + 11;
const WXK_CAPITAL: i32 = 300 + 12;
const WXK_END: i32 = 300 + 13;
const WXK_HOME: i32 = 300 + 14;
const WXK_LEFT: i32 = 300 + 15;
const WXK_UP: i32 = 300 + 16;
const WXK_RIGHT: i32 = 300 + 17;
const WXK_DOWN: i32 = 300 + 18;
const WXK_SELECT: i32 = 300 + 19;
const WXK_PRINT: i32 = 300 + 20;
const WXK_EXECUTE: i32 = 300 + 21;
const WXK_SNAPSHOT: i32 = 300 + 22;
const WXK_INSERT: i32 = 300 + 23;
const WXK_HELP: i32 = 300 + 24;
const WXK_NUMPAD0: i32 = 300 + 25;
const WXK_NUMPAD1: i32 = 300 + 26;
const WXK_NUMPAD2: i32 = 300 + 27;
const WXK_NUMPAD3: i32 = 300 + 28;
const WXK_NUMPAD4: i32 = 300 + 29;
const WXK_NUMPAD5: i32 = 300 + 30;
const WXK_NUMPAD6: i32 = 300 + 31;
const WXK_NUMPAD7: i32 = 300 + 32;
const WXK_NUMPAD8: i32 = 300 + 33;
const WXK_NUMPAD9: i32 = 300 + 34;
const WXK_MULTIPLY: i32 = 300 + 35;
const WXK_ADD: i32 = 300 + 36;
const WXK_SEPARATOR: i32 = 300 + 37;
const WXK_SUBTRACT: i32 = 300 + 38;
const WXK_DECIMAL: i32 = 300 + 39;
const WXK_DIVIDE: i32 = 300 + 40;
const WXK_F1: i32 = 300 + 41;
const WXK_F2: i32 = 300 + 42;
const WXK_F3: i32 = 300 + 43;
const WXK_F4: i32 = 300 + 44;
const WXK_F5: i32 = 300 + 45;
const WXK_F6: i32 = 300 + 46;
const WXK_F7: i32 = 300 + 47;
const WXK_F8: i32 = 300 + 48;
const WXK_F9: i32 = 300 + 49;
const WXK_F10: i32 = 300 + 50;
const WXK_F11: i32 = 300 + 51;
const WXK_F12: i32 = 300 + 52;
const WXK_F13: i32 = 300 + 53;
const WXK_F14: i32 = 300 + 54;
const WXK_F15: i32 = 300 + 55;
const WXK_F16: i32 = 300 + 56;
const WXK_F17: i32 = 300 + 57;
const WXK_F18: i32 = 300 + 58;
const WXK_F19: i32 = 300 + 59;
const WXK_F20: i32 = 300 + 60;
const WXK_F21: i32 = 300 + 61;
const WXK_F22: i32 = 300 + 62;
const WXK_F23: i32 = 300 + 63;
const WXK_F24: i32 = 300 + 64;
const WXK_NUMLOCK: i32 = 300 + 65;
const WXK_SCROLL: i32 = 300 + 66;
const WXK_PAGEUP: i32 = 300 + 67;
const WXK_PAGEDOWN: i32 = 300 + 68;
const WXK_NUMPAD_SPACE: i32 = 300 + 69;
const WXK_NUMPAD_TAB: i32 = 300 + 70;
const WXK_NUMPAD_ENTER: i32 = 300 + 71;
const WXK_NUMPAD_F1: i32 = 300 + 72;
const WXK_NUMPAD_F2: i32 = 300 + 73;
const WXK_NUMPAD_F3: i32 = 300 + 74;
const WXK_NUMPAD_F4: i32 = 300 + 75;
const WXK_NUMPAD_HOME: i32 = 300 + 76;
const WXK_NUMPAD_LEFT: i32 = 300 + 77;
const WXK_NUMPAD_UP: i32 = 300 + 78;
const WXK_NUMPAD_RIGHT: i32 = 300 + 79;
const WXK_NUMPAD_DOWN: i32 = 300 + 80;
const WXK_NUMPAD_PAGEUP: i32 = 300 + 81;
const WXK_NUMPAD_PAGEDOWN: i32 = 300 + 82;
const WXK_NUMPAD_END: i32 = 300 + 83;
const WXK_NUMPAD_BEGIN: i32 = 300 + 84;
const WXK_NUMPAD_INSERT: i32 = 300 + 85;
const WXK_NUMPAD_DELETE: i32 = 300 + 86;
const WXK_NUMPAD_EQUAL: i32 = 300 + 87;
const WXK_NUMPAD_MULTIPLY: i32 = 300 + 88;
const WXK_NUMPAD_ADD: i32 = 300 + 89;
const WXK_NUMPAD_SEPARATOR: i32 = 300 + 90;
const WXK_NUMPAD_SUBTRACT: i32 = 300 + 91;
const WXK_NUMPAD_DECIMAL: i32 = 300 + 92;
const WXK_NUMPAD_DIVIDE: i32 = 300 + 93;
const WXK_WINDOWS_LEFT: i32 = 300 + 94;
const WXK_WINDOWS_RIGHT: i32 = 300 + 95;
const WXK_WINDOWS_MENU: i32 = 300 + 96;
const WXK_COMMAND: i32 = 300 + 97;
const WXK_SPECIAL1: i32 = 193;
const WXK_SPECIAL2: i32 = 193 + 1;
const WXK_SPECIAL3: i32 = 193 + 2;
const WXK_SPECIAL4: i32 = 193 + 3;
const WXK_SPECIAL5: i32 = 193 + 4;
const WXK_SPECIAL6: i32 = 193 + 5;
const WXK_SPECIAL7: i32 = 193 + 6;
const WXK_SPECIAL8: i32 = 193 + 7;
const WXK_SPECIAL9: i32 = 193 + 8;
const WXK_SPECIAL10: i32 = 193 + 9;
const WXK_SPECIAL11: i32 = 193 + 10;
const WXK_SPECIAL12: i32 = 193 + 11;
const WXK_SPECIAL13: i32 = 193 + 12;
const WXK_SPECIAL14: i32 = 193 + 13;
const WXK_SPECIAL15: i32 = 193 + 14;
const WXK_SPECIAL16: i32 = 193 + 15;
const WXK_SPECIAL17: i32 = 193 + 16;
const WXK_SPECIAL18: i32 = 193 + 17;
const WXK_SPECIAL19: i32 = 193 + 18;
const WXK_SPECIAL20: i32 = 193 + 19;
const WXK_BROWSER_BACK: i32 = 501;
const WXK_BROWSER_FORWARD: i32 = 501 + 1;
const WXK_BROWSER_REFRESH: i32 = 501 + 2;
const WXK_BROWSER_STOP: i32 = 501 + 3;
const WXK_BROWSER_SEARCH: i32 = 501 + 4;
const WXK_BROWSER_FAVORITES: i32 = 501 + 5;
const WXK_BROWSER_HOME: i32 = 501 + 6;
const WXK_VOLUME_MUTE: i32 = 501 + 7;
const WXK_VOLUME_DOWN: i32 = 501 + 8;
const WXK_VOLUME_UP: i32 = 501 + 9;
const WXK_MEDIA_NEXT_TRACK: i32 = 501 + 10;
const WXK_MEDIA_PREV_TRACK: i32 = 501 + 11;
const WXK_MEDIA_STOP: i32 = 501 + 12;
const WXK_MEDIA_PLAY_PAUSE: i32 = 501 + 13;
const WXK_LAUNCH_MAIL: i32 = 501 + 14;
const WXK_LAUNCH_APP1: i32 = 501 + 15;
const WXK_LAUNCH_APP2: i32 = 501 + 16;
//  ENUM: wxKeyModifier
const wxMOD_NONE: i32 = 0x0000;
const wxMOD_ALT: i32 = 0x0001;
const wxMOD_CONTROL: i32 = 0x0002;
const wxMOD_ALTGR: i32 = wxMOD_ALT | wxMOD_CONTROL;
const wxMOD_SHIFT: i32 = 0x0004;
const wxMOD_META: i32 = 0x0008;
const wxMOD_WIN: i32 = wxMOD_META;
const wxMOD_RAW_CONTROL: i32 = wxMOD_META + 1;
const wxMOD_CMD: i32 = wxMOD_CONTROL;
const wxMOD_ALL: i32 = 0xffff;
//  ENUM: wxPaperSize
const wxPAPER_10X11: i32 = 0;
const wxPAPER_10X14: i32 = 0 + 1;
const wxPAPER_11X17: i32 = 0 + 2;
const wxPAPER_12X11: i32 = 0 + 3;
const wxPAPER_15X11: i32 = 0 + 4;
const wxPAPER_9X11: i32 = 0 + 5;
const wxPAPER_A2: i32 = 0 + 6;
const wxPAPER_A3: i32 = 0 + 7;
const wxPAPER_A3_EXTRA: i32 = 0 + 8;
const wxPAPER_A3_EXTRA_TRANSVERSE: i32 = 0 + 9;
const wxPAPER_A3_ROTATED: i32 = 0 + 10;
const wxPAPER_A3_TRANSVERSE: i32 = 0 + 11;
const wxPAPER_A4: i32 = 0 + 12;
const wxPAPER_A4SMALL: i32 = 0 + 13;
const wxPAPER_A4_EXTRA: i32 = 0 + 14;
const wxPAPER_A4_PLUS: i32 = 0 + 15;
const wxPAPER_A4_ROTATED: i32 = 0 + 16;
const wxPAPER_A4_TRANSVERSE: i32 = 0 + 17;
const wxPAPER_A5: i32 = 0 + 18;
const wxPAPER_A5_EXTRA: i32 = 0 + 19;
const wxPAPER_A5_ROTATED: i32 = 0 + 20;
const wxPAPER_A5_TRANSVERSE: i32 = 0 + 21;
const wxPAPER_A6: i32 = 0 + 22;
const wxPAPER_A6_ROTATED: i32 = 0 + 23;
const wxPAPER_A_PLUS: i32 = 0 + 24;
const wxPAPER_B4: i32 = 0 + 25;
const wxPAPER_B4_JIS_ROTATED: i32 = 0 + 26;
const wxPAPER_B5: i32 = 0 + 27;
const wxPAPER_B5_EXTRA: i32 = 0 + 28;
const wxPAPER_B5_JIS_ROTATED: i32 = 0 + 29;
const wxPAPER_B5_TRANSVERSE: i32 = 0 + 30;
const wxPAPER_B6_JIS: i32 = 0 + 31;
const wxPAPER_B6_JIS_ROTATED: i32 = 0 + 32;
const wxPAPER_B_PLUS: i32 = 0 + 33;
const wxPAPER_CSHEET: i32 = 0 + 34;
const wxPAPER_DBL_JAPANESE_POSTCARD: i32 = 0 + 35;
const wxPAPER_DBL_JAPANESE_POSTCARD_ROTATED: i32 = 0 + 36;
const wxPAPER_DSHEET: i32 = 0 + 37;
const wxPAPER_ENV_10: i32 = 0 + 38;
const wxPAPER_ENV_11: i32 = 0 + 39;
const wxPAPER_ENV_12: i32 = 0 + 40;
const wxPAPER_ENV_14: i32 = 0 + 41;
const wxPAPER_ENV_9: i32 = 0 + 42;
const wxPAPER_ENV_B4: i32 = 0 + 43;
const wxPAPER_ENV_B5: i32 = 0 + 44;
const wxPAPER_ENV_B6: i32 = 0 + 45;
const wxPAPER_ENV_C3: i32 = 0 + 46;
const wxPAPER_ENV_C4: i32 = 0 + 47;
const wxPAPER_ENV_C5: i32 = 0 + 48;
const wxPAPER_ENV_C6: i32 = 0 + 49;
const wxPAPER_ENV_C65: i32 = 0 + 50;
const wxPAPER_ENV_DL: i32 = 0 + 51;
const wxPAPER_ENV_INVITE: i32 = 0 + 52;
const wxPAPER_ENV_ITALY: i32 = 0 + 53;
const wxPAPER_ENV_MONARCH: i32 = 0 + 54;
const wxPAPER_ENV_PERSONAL: i32 = 0 + 55;
const wxPAPER_ESHEET: i32 = 0 + 56;
const wxPAPER_EXECUTIVE: i32 = 0 + 57;
const wxPAPER_FANFOLD_LGL_GERMAN: i32 = 0 + 58;
const wxPAPER_FANFOLD_STD_GERMAN: i32 = 0 + 59;
const wxPAPER_FANFOLD_US: i32 = 0 + 60;
const wxPAPER_FOLIO: i32 = 0 + 61;
const wxPAPER_ISO_B4: i32 = 0 + 62;
const wxPAPER_JAPANESE_POSTCARD: i32 = 0 + 63;
const wxPAPER_JAPANESE_POSTCARD_ROTATED: i32 = 0 + 64;
const wxPAPER_JENV_CHOU3: i32 = 0 + 65;
const wxPAPER_JENV_CHOU3_ROTATED: i32 = 0 + 66;
const wxPAPER_JENV_CHOU4: i32 = 0 + 67;
const wxPAPER_JENV_CHOU4_ROTATED: i32 = 0 + 68;
const wxPAPER_JENV_KAKU2: i32 = 0 + 69;
const wxPAPER_JENV_KAKU2_ROTATED: i32 = 0 + 70;
const wxPAPER_JENV_KAKU3: i32 = 0 + 71;
const wxPAPER_JENV_KAKU3_ROTATED: i32 = 0 + 72;
const wxPAPER_JENV_YOU4: i32 = 0 + 73;
const wxPAPER_JENV_YOU4_ROTATED: i32 = 0 + 74;
const wxPAPER_LEDGER: i32 = 0 + 75;
const wxPAPER_LEGAL: i32 = 0 + 76;
const wxPAPER_LEGAL_EXTRA: i32 = 0 + 77;
const wxPAPER_LETTER: i32 = 0 + 78;
const wxPAPER_LETTERSMALL: i32 = 0 + 79;
const wxPAPER_LETTER_EXTRA: i32 = 0 + 80;
const wxPAPER_LETTER_EXTRA_TRANSVERSE: i32 = 0 + 81;
const wxPAPER_LETTER_PLUS: i32 = 0 + 82;
const wxPAPER_LETTER_ROTATED: i32 = 0 + 83;
const wxPAPER_LETTER_TRANSVERSE: i32 = 0 + 84;
const wxPAPER_NONE: i32 = 0 + 85;
const wxPAPER_NOTE: i32 = 0 + 86;
const wxPAPER_P16K: i32 = 0 + 87;
const wxPAPER_P16K_ROTATED: i32 = 0 + 88;
const wxPAPER_P32K: i32 = 0 + 89;
const wxPAPER_P32KBIG: i32 = 0 + 90;
const wxPAPER_P32KBIG_ROTATED: i32 = 0 + 91;
const wxPAPER_P32K_ROTATED: i32 = 0 + 92;
const wxPAPER_PENV_1: i32 = 0 + 93;
const wxPAPER_PENV_10: i32 = 0 + 94;
const wxPAPER_PENV_10_ROTATED: i32 = 0 + 95;
const wxPAPER_PENV_1_ROTATED: i32 = 0 + 96;
const wxPAPER_PENV_2: i32 = 0 + 97;
const wxPAPER_PENV_2_ROTATED: i32 = 0 + 98;
const wxPAPER_PENV_3: i32 = 0 + 99;
const wxPAPER_PENV_3_ROTATED: i32 = 0 + 100;
const wxPAPER_PENV_4: i32 = 0 + 101;
const wxPAPER_PENV_4_ROTATED: i32 = 0 + 102;
const wxPAPER_PENV_5: i32 = 0 + 103;
const wxPAPER_PENV_5_ROTATED: i32 = 0 + 104;
const wxPAPER_PENV_6: i32 = 0 + 105;
const wxPAPER_PENV_6_ROTATED: i32 = 0 + 106;
const wxPAPER_PENV_7: i32 = 0 + 107;
const wxPAPER_PENV_7_ROTATED: i32 = 0 + 108;
const wxPAPER_PENV_8: i32 = 0 + 109;
const wxPAPER_PENV_8_ROTATED: i32 = 0 + 110;
const wxPAPER_PENV_9: i32 = 0 + 111;
const wxPAPER_PENV_9_ROTATED: i32 = 0 + 112;
const wxPAPER_QUARTO: i32 = 0 + 113;
const wxPAPER_STATEMENT: i32 = 0 + 114;
const wxPAPER_TABLOID: i32 = 0 + 115;
const wxPAPER_TABLOID_EXTRA: i32 = 0 + 116;
//  ENUM: wxPrintOrientation
const wxPORTRAIT: i32 = 0;
const wxLANDSCAPE: i32 = 0 + 1;
//  ENUM: wxDuplexMode
const wxDUPLEX_SIMPLEX: i32 = 0;
const wxDUPLEX_HORIZONTAL: i32 = 0 + 1;
const wxDUPLEX_VERTICAL: i32 = 0 + 2;
//  ENUM: wxPrintMode
const wxPRINT_MODE_NONE: i32 =    0;
const wxPRINT_MODE_PREVIEW: i32 = 1;
const wxPRINT_MODE_FILE: i32 =    2;
const wxPRINT_MODE_PRINTER: i32 = 3;
const wxPRINT_MODE_STREAM: i32 =  4;
//  ENUM: wxUpdateUI
const wxUPDATE_UI_NONE: i32 = 0;
const wxUPDATE_UI_RECURSE: i32 = 0 + 1;
const wxUPDATE_UI_FROMIDLE: i32 = 0 + 2;

//  SKIP: wxPG_IT_CHILDREN
//  ENUM: wxPG_ITERATOR_FLAGS
const wxPG_ITERATE_PROPERTIES: i32 = wxPG_PROP_PROPERTY |
                          wxPG_PROP_MISC_PARENT |
                          wxPG_PROP_AGGREGATE |
                          wxPG_PROP_COLLAPSED |
                          wxPG_IT_CHILDREN(wxPG_PROP_MISC_PARENT) |
                          wxPG_IT_CHILDREN(wxPG_PROP_CATEGORY);
const wxPG_ITERATE_HIDDEN: i32 = wxPG_PROP_HIDDEN |
                      wxPG_IT_CHILDREN(wxPG_PROP_COLLAPSED);
const wxPG_ITERATE_FIXED_CHILDREN: i32 = wxPG_IT_CHILDREN(wxPG_PROP_AGGREGATE) |
                              wxPG_ITERATE_PROPERTIES;
const wxPG_ITERATE_CATEGORIES: i32 = wxPG_PROP_CATEGORY |
                          wxPG_IT_CHILDREN(wxPG_PROP_CATEGORY) |
                          wxPG_PROP_COLLAPSED;
const wxPG_ITERATE_ALL_PARENTS: i32 = wxPG_PROP_MISC_PARENT |
                           wxPG_PROP_AGGREGATE |
                           wxPG_PROP_CATEGORY;
const wxPG_ITERATE_ALL_PARENTS_RECURSIVELY: i32 = wxPG_ITERATE_ALL_PARENTS |
                                       wxPG_IT_CHILDREN(
                                                wxPG_ITERATE_ALL_PARENTS);
const wxPG_ITERATOR_FLAGS_ALL: i32 = wxPG_PROP_PROPERTY |
                          wxPG_PROP_MISC_PARENT |
                          wxPG_PROP_AGGREGATE |
                          wxPG_PROP_HIDDEN |
                          wxPG_PROP_CATEGORY |
                          wxPG_PROP_COLLAPSED;
const wxPG_ITERATOR_MASK_OP_ITEM: i32 = wxPG_ITERATOR_FLAGS_ALL;
const wxPG_ITERATOR_MASK_OP_PARENT: i32 = wxPG_ITERATOR_FLAGS_ALL;
const wxPG_ITERATE_VISIBLE: i32 = wxPG_ITERATE_PROPERTIES |
                       wxPG_PROP_CATEGORY |
                       wxPG_IT_CHILDREN(wxPG_PROP_AGGREGATE);
const wxPG_ITERATE_ALL: i32 = wxPG_ITERATE_VISIBLE |
                   wxPG_ITERATE_HIDDEN;
const wxPG_ITERATE_NORMAL: i32 = wxPG_ITERATE_PROPERTIES |
                      wxPG_ITERATE_HIDDEN;
const wxPG_ITERATE_DEFAULT: i32 = wxPG_ITERATE_NORMAL;

//  ENUM: wxLanguage
const wxLANGUAGE_DEFAULT: i32 = 0;
const wxLANGUAGE_UNKNOWN: i32 = 0 + 1;
const wxLANGUAGE_ABKHAZIAN: i32 = 0 + 2;
const wxLANGUAGE_AFAR: i32 = 0 + 3;
const wxLANGUAGE_AFRIKAANS: i32 = 0 + 4;
const wxLANGUAGE_ALBANIAN: i32 = 0 + 5;
const wxLANGUAGE_AMHARIC: i32 = 0 + 6;
const wxLANGUAGE_ARABIC: i32 = 0 + 7;
const wxLANGUAGE_ARABIC_ALGERIA: i32 = 0 + 8;
const wxLANGUAGE_ARABIC_BAHRAIN: i32 = 0 + 9;
const wxLANGUAGE_ARABIC_EGYPT: i32 = 0 + 10;
const wxLANGUAGE_ARABIC_IRAQ: i32 = 0 + 11;
const wxLANGUAGE_ARABIC_JORDAN: i32 = 0 + 12;
const wxLANGUAGE_ARABIC_KUWAIT: i32 = 0 + 13;
const wxLANGUAGE_ARABIC_LEBANON: i32 = 0 + 14;
const wxLANGUAGE_ARABIC_LIBYA: i32 = 0 + 15;
const wxLANGUAGE_ARABIC_MOROCCO: i32 = 0 + 16;
const wxLANGUAGE_ARABIC_OMAN: i32 = 0 + 17;
const wxLANGUAGE_ARABIC_QATAR: i32 = 0 + 18;
const wxLANGUAGE_ARABIC_SAUDI_ARABIA: i32 = 0 + 19;
const wxLANGUAGE_ARABIC_SUDAN: i32 = 0 + 20;
const wxLANGUAGE_ARABIC_SYRIA: i32 = 0 + 21;
const wxLANGUAGE_ARABIC_TUNISIA: i32 = 0 + 22;
const wxLANGUAGE_ARABIC_UAE: i32 = 0 + 23;
const wxLANGUAGE_ARABIC_YEMEN: i32 = 0 + 24;
const wxLANGUAGE_ARMENIAN: i32 = 0 + 25;
const wxLANGUAGE_ASSAMESE: i32 = 0 + 26;
const wxLANGUAGE_ASTURIAN: i32 = 0 + 27;
const wxLANGUAGE_AYMARA: i32 = 0 + 28;
const wxLANGUAGE_AZERI: i32 = 0 + 29;
const wxLANGUAGE_AZERI_CYRILLIC: i32 = 0 + 30;
const wxLANGUAGE_AZERI_LATIN: i32 = 0 + 31;
const wxLANGUAGE_BASHKIR: i32 = 0 + 32;
const wxLANGUAGE_BASQUE: i32 = 0 + 33;
const wxLANGUAGE_BELARUSIAN: i32 = 0 + 34;
const wxLANGUAGE_BENGALI: i32 = 0 + 35;
const wxLANGUAGE_BHUTANI: i32 = 0 + 36;
const wxLANGUAGE_BIHARI: i32 = 0 + 37;
const wxLANGUAGE_BISLAMA: i32 = 0 + 38;
const wxLANGUAGE_BOSNIAN: i32 = 0 + 39;
const wxLANGUAGE_BRETON: i32 = 0 + 40;
const wxLANGUAGE_BULGARIAN: i32 = 0 + 41;
const wxLANGUAGE_BURMESE: i32 = 0 + 42;
const wxLANGUAGE_CATALAN: i32 = 0 + 43;
const wxLANGUAGE_CHINESE: i32 = 0 + 44;
const wxLANGUAGE_CHINESE_SIMPLIFIED: i32 = 0 + 45;
const wxLANGUAGE_CHINESE_TRADITIONAL: i32 = 0 + 46;
const wxLANGUAGE_CHINESE_HONGKONG: i32 = 0 + 47;
const wxLANGUAGE_CHINESE_MACAU: i32 = 0 + 48;
const wxLANGUAGE_CHINESE_SINGAPORE: i32 = 0 + 49;
const wxLANGUAGE_CHINESE_TAIWAN: i32 = 0 + 50;
const wxLANGUAGE_CORSICAN: i32 = 0 + 51;
const wxLANGUAGE_CROATIAN: i32 = 0 + 52;
const wxLANGUAGE_CZECH: i32 = 0 + 53;
const wxLANGUAGE_DANISH: i32 = 0 + 54;
const wxLANGUAGE_DUTCH: i32 = 0 + 55;
const wxLANGUAGE_DUTCH_BELGIAN: i32 = 0 + 56;
const wxLANGUAGE_ENGLISH: i32 = 0 + 57;
const wxLANGUAGE_ENGLISH_UK: i32 = 0 + 58;
const wxLANGUAGE_ENGLISH_US: i32 = 0 + 59;
const wxLANGUAGE_ENGLISH_AUSTRALIA: i32 = 0 + 60;
const wxLANGUAGE_ENGLISH_BELIZE: i32 = 0 + 61;
const wxLANGUAGE_ENGLISH_BOTSWANA: i32 = 0 + 62;
const wxLANGUAGE_ENGLISH_CANADA: i32 = 0 + 63;
const wxLANGUAGE_ENGLISH_CARIBBEAN: i32 = 0 + 64;
const wxLANGUAGE_ENGLISH_DENMARK: i32 = 0 + 65;
const wxLANGUAGE_ENGLISH_EIRE: i32 = 0 + 66;
const wxLANGUAGE_ENGLISH_ISRAEL: i32 = 0 + 67;
const wxLANGUAGE_ENGLISH_JAMAICA: i32 = 0 + 68;
const wxLANGUAGE_ENGLISH_NEW_ZEALAND: i32 = 0 + 69;
const wxLANGUAGE_ENGLISH_PHILIPPINES: i32 = 0 + 70;
const wxLANGUAGE_ENGLISH_SOUTH_AFRICA: i32 = 0 + 71;
const wxLANGUAGE_ENGLISH_TRINIDAD: i32 = 0 + 72;
const wxLANGUAGE_ENGLISH_ZIMBABWE: i32 = 0 + 73;
const wxLANGUAGE_ESPERANTO: i32 = 0 + 74;
const wxLANGUAGE_ESTONIAN: i32 = 0 + 75;
const wxLANGUAGE_FAEROESE: i32 = 0 + 76;
const wxLANGUAGE_FARSI: i32 = 0 + 77;
const wxLANGUAGE_FIJI: i32 = 0 + 78;
const wxLANGUAGE_FINNISH: i32 = 0 + 79;
const wxLANGUAGE_FRENCH: i32 = 0 + 80;
const wxLANGUAGE_FRENCH_BELGIAN: i32 = 0 + 81;
const wxLANGUAGE_FRENCH_CANADIAN: i32 = 0 + 82;
const wxLANGUAGE_FRENCH_LUXEMBOURG: i32 = 0 + 83;
const wxLANGUAGE_FRENCH_MONACO: i32 = 0 + 84;
const wxLANGUAGE_FRENCH_SWISS: i32 = 0 + 85;
const wxLANGUAGE_FRISIAN: i32 = 0 + 86;
const wxLANGUAGE_GALICIAN: i32 = 0 + 87;
const wxLANGUAGE_GEORGIAN: i32 = 0 + 88;
const wxLANGUAGE_GERMAN: i32 = 0 + 89;
const wxLANGUAGE_GERMAN_AUSTRIAN: i32 = 0 + 90;
const wxLANGUAGE_GERMAN_BELGIUM: i32 = 0 + 91;
const wxLANGUAGE_GERMAN_LIECHTENSTEIN: i32 = 0 + 92;
const wxLANGUAGE_GERMAN_LUXEMBOURG: i32 = 0 + 93;
const wxLANGUAGE_GERMAN_SWISS: i32 = 0 + 94;
const wxLANGUAGE_GREEK: i32 = 0 + 95;
const wxLANGUAGE_GREENLANDIC: i32 = 0 + 96;
const wxLANGUAGE_GUARANI: i32 = 0 + 97;
const wxLANGUAGE_GUJARATI: i32 = 0 + 98;
const wxLANGUAGE_HAUSA: i32 = 0 + 99;
const wxLANGUAGE_HEBREW: i32 = 0 + 100;
const wxLANGUAGE_HINDI: i32 = 0 + 101;
const wxLANGUAGE_HUNGARIAN: i32 = 0 + 102;
const wxLANGUAGE_ICELANDIC: i32 = 0 + 103;
const wxLANGUAGE_INDONESIAN: i32 = 0 + 104;
const wxLANGUAGE_INTERLINGUA: i32 = 0 + 105;
const wxLANGUAGE_INTERLINGUE: i32 = 0 + 106;
const wxLANGUAGE_INUKTITUT: i32 = 0 + 107;
const wxLANGUAGE_INUPIAK: i32 = 0 + 108;
const wxLANGUAGE_IRISH: i32 = 0 + 109;
const wxLANGUAGE_ITALIAN: i32 = 0 + 110;
const wxLANGUAGE_ITALIAN_SWISS: i32 = 0 + 111;
const wxLANGUAGE_JAPANESE: i32 = 0 + 112;
const wxLANGUAGE_JAVANESE: i32 = 0 + 113;
const wxLANGUAGE_KABYLE: i32 = 0 + 114;
const wxLANGUAGE_KANNADA: i32 = 0 + 115;
const wxLANGUAGE_KASHMIRI: i32 = 0 + 116;
const wxLANGUAGE_KASHMIRI_INDIA: i32 = 0 + 117;
const wxLANGUAGE_KAZAKH: i32 = 0 + 118;
const wxLANGUAGE_KERNEWEK: i32 = 0 + 119;
const wxLANGUAGE_KHMER: i32 = 0 + 120;
const wxLANGUAGE_KINYARWANDA: i32 = 0 + 121;
const wxLANGUAGE_KIRGHIZ: i32 = 0 + 122;
const wxLANGUAGE_KIRUNDI: i32 = 0 + 123;
const wxLANGUAGE_KONKANI: i32 = 0 + 124;
const wxLANGUAGE_KOREAN: i32 = 0 + 125;
const wxLANGUAGE_KURDISH: i32 = 0 + 126;
const wxLANGUAGE_LAOTHIAN: i32 = 0 + 127;
const wxLANGUAGE_LATIN: i32 = 0 + 128;
const wxLANGUAGE_LATVIAN: i32 = 0 + 129;
const wxLANGUAGE_LINGALA: i32 = 0 + 130;
const wxLANGUAGE_LITHUANIAN: i32 = 0 + 131;
const wxLANGUAGE_MACEDONIAN: i32 = 0 + 132;
const wxLANGUAGE_MALAGASY: i32 = 0 + 133;
const wxLANGUAGE_MALAY: i32 = 0 + 134;
const wxLANGUAGE_MALAYALAM: i32 = 0 + 135;
const wxLANGUAGE_MALAY_BRUNEI_DARUSSALAM: i32 = 0 + 136;
const wxLANGUAGE_MALAY_MALAYSIA: i32 = 0 + 137;
const wxLANGUAGE_MALTESE: i32 = 0 + 138;
const wxLANGUAGE_MANIPURI: i32 = 0 + 139;
const wxLANGUAGE_MAORI: i32 = 0 + 140;
const wxLANGUAGE_MARATHI: i32 = 0 + 141;
const wxLANGUAGE_MOLDAVIAN: i32 = 0 + 142;
const wxLANGUAGE_MONGOLIAN: i32 = 0 + 143;
const wxLANGUAGE_NAURU: i32 = 0 + 144;
const wxLANGUAGE_NEPALI: i32 = 0 + 145;
const wxLANGUAGE_NEPALI_INDIA: i32 = 0 + 146;
const wxLANGUAGE_NORWEGIAN_BOKMAL: i32 = 0 + 147;
const wxLANGUAGE_NORWEGIAN_NYNORSK: i32 = 0 + 148;
const wxLANGUAGE_OCCITAN: i32 = 0 + 149;
const wxLANGUAGE_ORIYA: i32 = 0 + 150;
const wxLANGUAGE_OROMO: i32 = 0 + 151;
const wxLANGUAGE_PASHTO: i32 = 0 + 152;
const wxLANGUAGE_POLISH: i32 = 0 + 153;
const wxLANGUAGE_PORTUGUESE: i32 = 0 + 154;
const wxLANGUAGE_PORTUGUESE_BRAZILIAN: i32 = 0 + 155;
const wxLANGUAGE_PUNJABI: i32 = 0 + 156;
const wxLANGUAGE_QUECHUA: i32 = 0 + 157;
const wxLANGUAGE_RHAETO_ROMANCE: i32 = 0 + 158;
const wxLANGUAGE_ROMANIAN: i32 = 0 + 159;
const wxLANGUAGE_RUSSIAN: i32 = 0 + 160;
const wxLANGUAGE_RUSSIAN_UKRAINE: i32 = 0 + 161;
const wxLANGUAGE_SAMI: i32 = 0 + 162;
const wxLANGUAGE_SAMOAN: i32 = 0 + 163;
const wxLANGUAGE_SANGHO: i32 = 0 + 164;
const wxLANGUAGE_SANSKRIT: i32 = 0 + 165;
const wxLANGUAGE_SCOTS_GAELIC: i32 = 0 + 166;
const wxLANGUAGE_SERBIAN: i32 = 0 + 167;
const wxLANGUAGE_SERBIAN_CYRILLIC: i32 = 0 + 168;
const wxLANGUAGE_SERBIAN_LATIN: i32 = 0 + 169;
const wxLANGUAGE_SERBO_CROATIAN: i32 = 0 + 170;
const wxLANGUAGE_SESOTHO: i32 = 0 + 171;
const wxLANGUAGE_SETSWANA: i32 = 0 + 172;
const wxLANGUAGE_SHONA: i32 = 0 + 173;
const wxLANGUAGE_SINDHI: i32 = 0 + 174;
const wxLANGUAGE_SINHALESE: i32 = 0 + 175;
const wxLANGUAGE_SISWATI: i32 = 0 + 176;
const wxLANGUAGE_SLOVAK: i32 = 0 + 177;
const wxLANGUAGE_SLOVENIAN: i32 = 0 + 178;
const wxLANGUAGE_SOMALI: i32 = 0 + 179;
const wxLANGUAGE_SPANISH: i32 = 0 + 180;
const wxLANGUAGE_SPANISH_ARGENTINA: i32 = 0 + 181;
const wxLANGUAGE_SPANISH_BOLIVIA: i32 = 0 + 182;
const wxLANGUAGE_SPANISH_CHILE: i32 = 0 + 183;
const wxLANGUAGE_SPANISH_COLOMBIA: i32 = 0 + 184;
const wxLANGUAGE_SPANISH_COSTA_RICA: i32 = 0 + 185;
const wxLANGUAGE_SPANISH_DOMINICAN_REPUBLIC: i32 = 0 + 186;
const wxLANGUAGE_SPANISH_ECUADOR: i32 = 0 + 187;
const wxLANGUAGE_SPANISH_EL_SALVADOR: i32 = 0 + 188;
const wxLANGUAGE_SPANISH_GUATEMALA: i32 = 0 + 189;
const wxLANGUAGE_SPANISH_HONDURAS: i32 = 0 + 190;
const wxLANGUAGE_SPANISH_MEXICAN: i32 = 0 + 191;
const wxLANGUAGE_SPANISH_MODERN: i32 = 0 + 192;
const wxLANGUAGE_SPANISH_NICARAGUA: i32 = 0 + 193;
const wxLANGUAGE_SPANISH_PANAMA: i32 = 0 + 194;
const wxLANGUAGE_SPANISH_PARAGUAY: i32 = 0 + 195;
const wxLANGUAGE_SPANISH_PERU: i32 = 0 + 196;
const wxLANGUAGE_SPANISH_PUERTO_RICO: i32 = 0 + 197;
const wxLANGUAGE_SPANISH_URUGUAY: i32 = 0 + 198;
const wxLANGUAGE_SPANISH_US: i32 = 0 + 199;
const wxLANGUAGE_SPANISH_VENEZUELA: i32 = 0 + 200;
const wxLANGUAGE_SUNDANESE: i32 = 0 + 201;
const wxLANGUAGE_SWAHILI: i32 = 0 + 202;
const wxLANGUAGE_SWEDISH: i32 = 0 + 203;
const wxLANGUAGE_SWEDISH_FINLAND: i32 = 0 + 204;
const wxLANGUAGE_TAGALOG: i32 = 0 + 205;
const wxLANGUAGE_TAJIK: i32 = 0 + 206;
const wxLANGUAGE_TAMIL: i32 = 0 + 207;
const wxLANGUAGE_TATAR: i32 = 0 + 208;
const wxLANGUAGE_TELUGU: i32 = 0 + 209;
const wxLANGUAGE_THAI: i32 = 0 + 210;
const wxLANGUAGE_TIBETAN: i32 = 0 + 211;
const wxLANGUAGE_TIGRINYA: i32 = 0 + 212;
const wxLANGUAGE_TONGA: i32 = 0 + 213;
const wxLANGUAGE_TSONGA: i32 = 0 + 214;
const wxLANGUAGE_TURKISH: i32 = 0 + 215;
const wxLANGUAGE_TURKMEN: i32 = 0 + 216;
const wxLANGUAGE_TWI: i32 = 0 + 217;
const wxLANGUAGE_UIGHUR: i32 = 0 + 218;
const wxLANGUAGE_UKRAINIAN: i32 = 0 + 219;
const wxLANGUAGE_URDU: i32 = 0 + 220;
const wxLANGUAGE_URDU_INDIA: i32 = 0 + 221;
const wxLANGUAGE_URDU_PAKISTAN: i32 = 0 + 222;
const wxLANGUAGE_UZBEK: i32 = 0 + 223;
const wxLANGUAGE_UZBEK_CYRILLIC: i32 = 0 + 224;
const wxLANGUAGE_UZBEK_LATIN: i32 = 0 + 225;
const wxLANGUAGE_VALENCIAN: i32 = 0 + 226;
const wxLANGUAGE_VIETNAMESE: i32 = 0 + 227;
const wxLANGUAGE_VOLAPUK: i32 = 0 + 228;
const wxLANGUAGE_WELSH: i32 = 0 + 229;
const wxLANGUAGE_WOLOF: i32 = 0 + 230;
const wxLANGUAGE_XHOSA: i32 = 0 + 231;
const wxLANGUAGE_YIDDISH: i32 = 0 + 232;
const wxLANGUAGE_YORUBA: i32 = 0 + 233;
const wxLANGUAGE_ZHUANG: i32 = 0 + 234;
const wxLANGUAGE_ZULU: i32 = 0 + 235;
const wxLANGUAGE_USER_DEFINED: i32 = 0 + 236;
const wxLANGUAGE_CAMBODIAN: i32 = wxLANGUAGE_KHMER;

const wxDVC_DEFAULT_RENDERER_SIZE: i32 = 20;
const wxDVC_DEFAULT_WIDTH: i32 = 80;
const wxDVC_TOGGLE_DEFAULT_WIDTH: i32 = 30;
const wxDVC_DEFAULT_MINWIDTH: i32 = 30;
const wxDVR_DEFAULT_ALIGNMENT: i32 = -1;
const wxDV_SINGLE: i32 = 0x0000;
const wxDV_MULTIPLE: i32 = 0x0001;
const wxDV_NO_HEADER: i32 = 0x0002;
const wxDV_HORIZ_RULES: i32 = 0x0004;
const wxDV_VERT_RULES: i32 = 0x0008;
const wxDV_ROW_LINES: i32 = 0x0010;
const wxDV_VARIABLE_LINE_HEIGHT: i32 = 0x0020;
//  ENUM: wxDataViewCellMode
const wxDATAVIEW_CELL_INERT: i32 = 0;
const wxDATAVIEW_CELL_ACTIVATABLE: i32 = 0 + 1;
const wxDATAVIEW_CELL_EDITABLE: i32 = 0 + 2;
//  ENUM: wxDataViewCellRenderState
const wxDATAVIEW_CELL_SELECTED: i32 = 1;
const wxDATAVIEW_CELL_PRELIT: i32 = 2;
const wxDATAVIEW_CELL_INSENSITIVE: i32 = 4;
const wxDATAVIEW_CELL_FOCUSED: i32 = 8;
//  ENUM: wxDataViewColumnFlags
const wxDATAVIEW_COL_RESIZABLE: i32 = 1;
const wxDATAVIEW_COL_SORTABLE: i32 = 2;
const wxDATAVIEW_COL_REORDERABLE: i32 = 4;
const wxDATAVIEW_COL_HIDDEN: i32 = 8;

//  ENUM: wxBrushStyle
const wxBRUSHSTYLE_INVALID: i32 = -1;
const wxBRUSHSTYLE_SOLID: i32 = wxSOLID;
const wxBRUSHSTYLE_TRANSPARENT: i32 = wxTRANSPARENT;
const wxBRUSHSTYLE_STIPPLE_MASK_OPAQUE: i32 = wxSTIPPLE_MASK_OPAQUE;
const wxBRUSHSTYLE_STIPPLE_MASK: i32 = wxSTIPPLE_MASK;
const wxBRUSHSTYLE_STIPPLE: i32 = wxSTIPPLE;
const wxBRUSHSTYLE_BDIAGONAL_HATCH: i32 = wxSTIPPLE + 1;
const wxBRUSHSTYLE_CROSSDIAG_HATCH: i32 = wxSTIPPLE + 2;
const wxBRUSHSTYLE_FDIAGONAL_HATCH: i32 = wxSTIPPLE + 3;
const wxBRUSHSTYLE_CROSS_HATCH: i32 = wxSTIPPLE + 4;
const wxBRUSHSTYLE_HORIZONTAL_HATCH: i32 = wxSTIPPLE + 5;
const wxBRUSHSTYLE_VERTICAL_HATCH: i32 = wxSTIPPLE + 6;
const wxBRUSHSTYLE_FIRST_HATCH: i32 = wxSTIPPLE + 7;
const wxBRUSHSTYLE_LAST_HATCH: i32 = wxSTIPPLE + 8;

//  ENUM: wxURLError
const wxURL_NOERR: i32 = 0;
const wxURL_SNTXERR: i32 = 0 + 1;
const wxURL_NOPROTO: i32 = 0 + 2;
const wxURL_NOHOST: i32 = 0 + 3;
const wxURL_NOPATH: i32 = 0 + 4;
const wxURL_CONNERR: i32 = 0 + 5;
const wxURL_PROTOERR: i32 = 0 + 6;

const wxLB_DEFAULT: i32 = wxBK_DEFAULT;
const wxLB_TOP: i32 = wxBK_TOP;
const wxLB_BOTTOM: i32 = wxBK_BOTTOM;
const wxLB_LEFT: i32 = wxBK_LEFT;
const wxLB_RIGHT: i32 = wxBK_RIGHT;
const wxLB_ALIGN_MASK: i32 = wxBK_ALIGN_MASK;

//  SKIP: wxPG_NULL_BITMAP
//  SKIP: wxPG_INVALID_VALUE
const wxPG_BASE_OCT: i32 = 8L;
const wxPG_BASE_DEC: i32 = 10L;
const wxPG_BASE_HEX: i32 = 16L;
const wxPG_BASE_HEXL: i32 = 32L;
const wxPG_PREFIX_NONE: i32 = 0L;
const wxPG_PREFIX_0x: i32 = 1L;
const wxPG_PREFIX_DOLLAR_SIGN: i32 = 2L;
//  ENUM: wxPG_GETPROPERTYVALUES_FLAGS
const wxPG_DONT_RECURSE: i32 = 0x00000000;
const wxPG_KEEP_STRUCTURE: i32 = 0x00000010;
const wxPG_RECURSE: i32 = 0x00000020;
const wxPG_INC_ATTRIBUTES: i32 = 0x00000040;
const wxPG_RECURSE_STARTS: i32 = 0x00000080;
const wxPG_FORCE: i32 = 0x00000100;
const wxPG_SORT_TOP_LEVEL_ONLY: i32 = 0x00000200;
//  ENUM: wxPG_MISC_ARG_FLAGS
const wxPG_FULL_VALUE: i32 = 0x00000001;
const wxPG_REPORT_ERROR: i32 = 0x00000002;
const wxPG_PROPERTY_SPECIFIC: i32 = 0x00000004;
const wxPG_EDITABLE_VALUE: i32 = 0x00000008;
const wxPG_COMPOSITE_FRAGMENT: i32 = 0x00000010;
const wxPG_UNEDITABLE_COMPOSITE_FRAGMENT: i32 = 0x00000020;
const wxPG_VALUE_IS_CURRENT: i32 = 0x00000040;
const wxPG_PROGRAMMATIC_VALUE: i32 = 0x00000080;
//  ENUM: wxPG_SETVALUE_FLAGS
const wxPG_SETVAL_REFRESH_EDITOR: i32 = 0x0001;
const wxPG_SETVAL_AGGREGATED: i32 = 0x0002;
const wxPG_SETVAL_FROM_PARENT: i32 = 0x0004;
const wxPG_SETVAL_BY_USER: i32 = 0x0008;

//  ENUM: wxTextValidatorStyle
const wxFILTER_NONE: i32 = 0;
const wxFILTER_EMPTY: i32 = 0 + 1;
const wxFILTER_ASCII: i32 = 0 + 2;
const wxFILTER_ALPHA: i32 = 0 + 3;
const wxFILTER_ALPHANUMERIC: i32 = 0 + 4;
const wxFILTER_DIGITS: i32 = 0 + 5;
const wxFILTER_NUMERIC: i32 = 0 + 6;
const wxFILTER_INCLUDE_LIST: i32 = 0 + 7;
const wxFILTER_INCLUDE_CHAR_LIST: i32 = 0 + 8;
const wxFILTER_EXCLUDE_LIST: i32 = 0 + 9;
const wxFILTER_EXCLUDE_CHAR_LIST: i32 = 0 + 10;
const wxFILTER_XDIGITS: i32 = 0 + 11;
const wxFILTER_SPACE: i32 = 0 + 12;

//  ENUM: EditableStateFlags
const SelectionState: i32 = 0x01;
const ExpandedState: i32 = 0x02;
const ScrollPosState: i32 = 0x04;
const PageState: i32 = 0x08;
const SplitterPosState: i32 = 0x10;
const DescBoxState: i32 = 0x20;
const AllStates: i32 = SelectionState |
                           ExpandedState |
                           ScrollPosState |
                           PageState |
                           SplitterPosState |
                           DescBoxState;

//  ENUM: wxAttrKind
const Any: i32 = 0;
const Cell: i32 = 0 + 1;
const Row: i32 = 0 + 2;
const Col: i32 = 0 + 3;
const Default: i32 = 0 + 4;
const Merged: i32 = 0 + 5;

//  ENUM: wxShowEffect
const wxSHOW_EFFECT_NONE: i32 = 0;
const wxSHOW_EFFECT_ROLL_TO_LEFT: i32 = 0 + 1;
const wxSHOW_EFFECT_ROLL_TO_RIGHT: i32 = 0 + 2;
const wxSHOW_EFFECT_ROLL_TO_TOP: i32 = 0 + 3;
const wxSHOW_EFFECT_ROLL_TO_BOTTOM: i32 = 0 + 4;
const wxSHOW_EFFECT_SLIDE_TO_LEFT: i32 = 0 + 5;
const wxSHOW_EFFECT_SLIDE_TO_RIGHT: i32 = 0 + 6;
const wxSHOW_EFFECT_SLIDE_TO_TOP: i32 = 0 + 7;
const wxSHOW_EFFECT_SLIDE_TO_BOTTOM: i32 = 0 + 8;
const wxSHOW_EFFECT_BLEND: i32 = 0 + 9;
const wxSHOW_EFFECT_EXPAND: i32 = 0 + 10;
const wxSHOW_EFFECT_MAX: i32 = 0 + 11;
//  ENUM: @55
const wxTOUCH_NONE: i32 = 0;
const wxTOUCH_VERTICAL_PAN_GESTURE: i32 = 0 + 1;
const wxTOUCH_HORIZONTAL_PAN_GESTURE: i32 = 0 + 2;
const wxTOUCH_PAN_GESTURES: i32 = 0 + 3;
const wxTOUCH_ZOOM_GESTURE: i32 = 0 + 4;
const wxTOUCH_ROTATE_GESTURE: i32 = 0 + 5;
const wxTOUCH_PRESS_GESTURES: i32 = 0 + 6;
const wxTOUCH_ALL_GESTURES: i32 = 0 + 7;
//  ENUM: @56
const wxSEND_EVENT_POST: i32 = 1;
//  ENUM: wxWindowVariant
const wxWINDOW_VARIANT_NORMAL: i32 = 0;
const wxWINDOW_VARIANT_SMALL: i32 = 0 + 1;
const wxWINDOW_VARIANT_MINI: i32 = 0 + 2;
const wxWINDOW_VARIANT_LARGE: i32 = 0 + 3;
const wxWINDOW_VARIANT_MAX: i32 = 0 + 4;

//  ENUM: @7
const wxCC_SPECIAL_DCLICK: i32 = 0x0100;
const wxCC_STD_BUTTON: i32 = 0x0200;

//  ENUM: @41
const wxRE_EXTENDED: i32 = 0;
const wxRE_ADVANCED: i32 = 1;
const wxRE_BASIC: i32 = 2;
const wxRE_ICASE: i32 = 4;
const wxRE_NOSUB: i32 = 8;
const wxRE_NEWLINE: i32 = 16;
const wxRE_DEFAULT: i32 = wxRE_EXTENDED;
//  ENUM: @42
const wxRE_NOTBOL: i32 = 32;
const wxRE_NOTEOL: i32 = 64;

//  ENUM: EntryType
const Type_Unknown: i32 = 0;
const Type_String: i32 = 0 + 1;
const Type_Boolean: i32 = 0 + 2;
const Type_Integer: i32 = 0 + 3;
const Type_Float: i32 = 0 + 4;

//  ENUM: wxOwnerDrawnComboBoxPaintingFlags
const wxODCB_PAINTING_CONTROL: i32 = 0x0001;
const wxODCB_PAINTING_SELECTED: i32 = 0x0002;
//  ENUM: @39
const wxODCB_DCLICK_CYCLES: i32 = wxCC_SPECIAL_DCLICK;
const wxODCB_STD_CONTROL_PAINT: i32 = 0x1000;

//  ENUM: wxOutCode
const wxInside: i32 = 0x00;
const wxOutLeft: i32 = 0x01;
const wxOutRight: i32 = 0x02;
const wxOutTop: i32 = 0x08;
const wxOutBottom: i32 = 0x04;

//  ENUM: @38
const Timeout_Auto: i32 = -1;
const Timeout_Never: i32 = 0;

//  ENUM: @58
const wxEXTEND_LAST_ON_EACH_LINE: i32 = 0;
const wxREMOVE_LEADING_SPACES: i32 = 0 + 1;
const wxWRAPSIZER_DEFAULT_FLAGS: i32 = 0 + 2;

//  ENUM: wxIPCFormat
const wxIPC_INVALID: i32 =     0;
const wxIPC_TEXT: i32 =        1;
const wxIPC_BITMAP: i32 =      2;
const wxIPC_METAFILE: i32 =    3;
const wxIPC_SYLK: i32 =        4;
const wxIPC_DIF: i32 =         5;
const wxIPC_TIFF: i32 =        6;
const wxIPC_OEMTEXT: i32 =     7;
const wxIPC_DIB: i32 =         8;
const wxIPC_PALETTE: i32 =     9;
const wxIPC_PENDATA: i32 =     10;
const wxIPC_RIFF: i32 =        11;
const wxIPC_WAVE: i32 =        12;
const wxIPC_UTF16TEXT: i32 =   13;
const wxIPC_ENHMETAFILE: i32 = 14;
const wxIPC_FILENAME: i32 =    15;
const wxIPC_LOCALE: i32 =      16;
const wxIPC_UTF8TEXT: i32 =    17;
const wxIPC_UTF32TEXT: i32 =   18;
const wxIPC_UNICODETEXT: i32 = wxIPC_UTF16TEXT;
const wxIPC_PRIVATE: i32 =     20;
const wxIPC_INVALID: i32 =          0;
const wxIPC_TEXT: i32 =             1;
const wxIPC_BITMAP: i32 =           2;
const wxIPC_METAFILE: i32 =         3;
const wxIPC_SYLK: i32 =             4;
const wxIPC_DIF: i32 =              5;
const wxIPC_TIFF: i32 =             6;
const wxIPC_OEMTEXT: i32 =          7;
const wxIPC_DIB: i32 =              8;
const wxIPC_PALETTE: i32 =          9;
const wxIPC_PENDATA: i32 =          10;
const wxIPC_RIFF: i32 =             11;
const wxIPC_WAVE: i32 =             12;
const wxIPC_UTF16TEXT: i32 =        13;
const wxIPC_ENHMETAFILE: i32 =      14;
const wxIPC_FILENAME: i32 =         15;
const wxIPC_LOCALE: i32 =           16;
const wxIPC_UTF8TEXT: i32 =         17;
const wxIPC_UTF32TEXT: i32 =        18;
const wxIPC_UNICODETEXT: i32 =        18 + 1;
const wxIPC_PRIVATE: i32 =          20;

const wxXML_NO_INDENTATION: i32 = (-1);
//  ENUM: wxXmlNodeType
const wxXML_ELEMENT_NODE: i32 =  1;
const wxXML_ATTRIBUTE_NODE: i32 =  2;
const wxXML_TEXT_NODE: i32 =  3;
const wxXML_CDATA_SECTION_NODE: i32 =  4;
const wxXML_ENTITY_REF_NODE: i32 =  5;
const wxXML_ENTITY_NODE: i32 =  6;
const wxXML_PI_NODE: i32 =  7;
const wxXML_COMMENT_NODE: i32 =  8;
const wxXML_DOCUMENT_NODE: i32 =  9;
const wxXML_DOCUMENT_TYPE_NODE: i32 = 10;
const wxXML_DOCUMENT_FRAG_NODE: i32 = 11;
const wxXML_NOTATION_NODE: i32 = 12;
const wxXML_HTML_DOCUMENT_NODE: i32 = 13;
//  ENUM: wxXmlDocumentLoadFlag
const wxXMLDOC_NONE: i32 = 0;
const wxXMLDOC_KEEP_WHITESPACE_NODES: i32 = 0 + 1;

//  ENUM: wxIPCFormat
const wxIPC_INVALID: i32 =     0;
const wxIPC_TEXT: i32 =        1;
const wxIPC_BITMAP: i32 =      2;
const wxIPC_METAFILE: i32 =    3;
const wxIPC_SYLK: i32 =        4;
const wxIPC_DIF: i32 =         5;
const wxIPC_TIFF: i32 =        6;
const wxIPC_OEMTEXT: i32 =     7;
const wxIPC_DIB: i32 =         8;
const wxIPC_PALETTE: i32 =     9;
const wxIPC_PENDATA: i32 =     10;
const wxIPC_RIFF: i32 =        11;
const wxIPC_WAVE: i32 =        12;
const wxIPC_UTF16TEXT: i32 =   13;
const wxIPC_ENHMETAFILE: i32 = 14;
const wxIPC_FILENAME: i32 =    15;
const wxIPC_LOCALE: i32 =      16;
const wxIPC_UTF8TEXT: i32 =    17;
const wxIPC_UTF32TEXT: i32 =   18;
const wxIPC_UNICODETEXT: i32 = wxIPC_UTF16TEXT;
const wxIPC_PRIVATE: i32 =     20;
const wxIPC_INVALID: i32 =          0;
const wxIPC_TEXT: i32 =             1;
const wxIPC_BITMAP: i32 =           2;
const wxIPC_METAFILE: i32 =         3;
const wxIPC_SYLK: i32 =             4;
const wxIPC_DIF: i32 =              5;
const wxIPC_TIFF: i32 =             6;
const wxIPC_OEMTEXT: i32 =          7;
const wxIPC_DIB: i32 =              8;
const wxIPC_PALETTE: i32 =          9;
const wxIPC_PENDATA: i32 =          10;
const wxIPC_RIFF: i32 =             11;
const wxIPC_WAVE: i32 =             12;
const wxIPC_UTF16TEXT: i32 =        13;
const wxIPC_ENHMETAFILE: i32 =      14;
const wxIPC_FILENAME: i32 =         15;
const wxIPC_LOCALE: i32 =           16;
const wxIPC_UTF8TEXT: i32 =         17;
const wxIPC_UTF32TEXT: i32 =        18;
const wxIPC_UNICODETEXT: i32 =        18 + 1;
const wxIPC_PRIVATE: i32 =          20;

//  ENUM: @9
const ShowBelow: i32 = 0x0000;
const ShowAbove: i32 = 0x0001;
const CanDeferShow: i32 = 0x0002;

const wxHL_CONTEXTMENU: i32 = 0x0001;
const wxHL_ALIGN_LEFT: i32 = 0x0002;
const wxHL_ALIGN_RIGHT: i32 = 0x0004;
const wxHL_ALIGN_CENTRE: i32 = 0x0008;
const wxHL_DEFAULT_STYLE: i32 = (wxHL_CONTEXTMENU|wxNO_BORDER|wxHL_ALIGN_CENTRE);

const wxPD_CAN_ABORT: i32 = 0x0001;
const wxPD_APP_MODAL: i32 = 0x0002;
const wxPD_AUTO_HIDE: i32 = 0x0004;
const wxPD_ELAPSED_TIME: i32 = 0x0008;
const wxPD_ESTIMATED_TIME: i32 = 0x0010;
const wxPD_SMOOTH: i32 = 0x0020;
const wxPD_REMAINING_TIME: i32 = 0x0040;
const wxPD_CAN_SKIP: i32 = 0x0080;

const wxPG_COLOUR_WEB_BASE: i32 = 0x10000;
const wxPG_COLOUR_CUSTOM: i32 = 0xFFFFFF;
const wxPG_COLOUR_UNSPECIFIED: i32 = (wxPG_COLOUR_CUSTOM+1);
const wxPG_PROP_TRANSLATE_CUSTOM: i32 = wxPG_PROP_CLASS_SPECIFIC_1;

const wxIMAGE_OPTION_QUALITY: &str = "quality";
const wxIMAGE_OPTION_FILENAME: &str = "FileName";
const wxIMAGE_OPTION_RESOLUTION: &str = "Resolution";
const wxIMAGE_OPTION_RESOLUTIONX: &str = "ResolutionX";
const wxIMAGE_OPTION_RESOLUTIONY: &str = "ResolutionY";
const wxIMAGE_OPTION_RESOLUTIONUNIT: &str = "ResolutionUnit";
const wxIMAGE_OPTION_MAX_WIDTH: &str = "MaxWidth";
const wxIMAGE_OPTION_MAX_HEIGHT: &str = "MaxHeight";
const wxIMAGE_OPTION_ORIGINAL_WIDTH: &str = "OriginalWidth";
const wxIMAGE_OPTION_ORIGINAL_HEIGHT: &str = "OriginalHeight";
const wxIMAGE_OPTION_BMP_FORMAT: &str = "wxBMP_FORMAT";
const wxIMAGE_OPTION_CUR_HOTSPOT_X: &str = "HotSpotX";
const wxIMAGE_OPTION_CUR_HOTSPOT_Y: &str = "HotSpotY";
const wxIMAGE_OPTION_GIF_COMMENT: &str = "GifComment";
const wxIMAGE_OPTION_GIF_TRANSPARENCY: &str = "Transparency";
const wxIMAGE_OPTION_GIF_TRANSPARENCY_HIGHLIGHT: &str = "Highlight";
const wxIMAGE_OPTION_GIF_TRANSPARENCY_UNCHANGED: &str = "Unchanged";
const wxIMAGE_OPTION_PNG_FORMAT: &str = "PngFormat";
const wxIMAGE_OPTION_PNG_BITDEPTH: &str = "PngBitDepth";
const wxIMAGE_OPTION_PNG_FILTER: &str = "PngF";
const wxIMAGE_OPTION_PNG_COMPRESSION_LEVEL: &str = "PngZL";
const wxIMAGE_OPTION_PNG_COMPRESSION_MEM_LEVEL: &str = "PngZM";
const wxIMAGE_OPTION_PNG_COMPRESSION_STRATEGY: &str = "PngZS";
const wxIMAGE_OPTION_PNG_COMPRESSION_BUFFER_SIZE: &str = "PngZB";
const wxIMAGE_OPTION_TIFF_BITSPERSAMPLE: &str = "BitsPerSample";
const wxIMAGE_OPTION_TIFF_SAMPLESPERPIXEL: &str = "SamplesPerPixel";
const wxIMAGE_OPTION_TIFF_COMPRESSION: &str = "Compression";
const wxIMAGE_OPTION_TIFF_PHOTOMETRIC: &str = "Photometric";
const wxIMAGE_OPTION_TIFF_IMAGEDESCRIPTOR: &str = "ImageDescriptor";
//  ENUM: wxImageResolution
const wxIMAGE_RESOLUTION_NONE: i32 = 0;
const wxIMAGE_RESOLUTION_INCHES: i32 = 1;
const wxIMAGE_RESOLUTION_CM: i32 = 2;
//  ENUM: wxImageResizeQuality
const wxIMAGE_QUALITY_NEAREST: i32 = 0;
const wxIMAGE_QUALITY_BILINEAR: i32 = 0 + 1;
const wxIMAGE_QUALITY_BICUBIC: i32 = 0 + 2;
const wxIMAGE_QUALITY_BOX_AVERAGE: i32 = 0 + 3;
const wxIMAGE_QUALITY_NORMAL: i32 = 0 + 4;
const wxIMAGE_QUALITY_HIGH: i32 = 0 + 5;
//  ENUM: wxImageAlphaBlendMode
const wxIMAGE_ALPHA_BLEND_OVER: i32 = 0;
const wxIMAGE_ALPHA_BLEND_COMPOSE: i32 = 1;
//  ENUM: wxImagePNGType
const wxPNG_TYPE_COLOUR: i32 = 0;
const wxPNG_TYPE_GREY: i32 = 2;
const wxPNG_TYPE_GREY_RED: i32 = 3;
const wxPNG_TYPE_PALETTE: i32 = 4;
//  ENUM: @30
const wxBMP_24BPP: i32 = 24;
const wxBMP_8BPP: i32 =  8;
const wxBMP_8BPP_GREY: i32 =  9;
const wxBMP_8BPP_GRAY: i32 =  wxBMP_8BPP_GREY;
const wxBMP_8BPP_RED: i32 = 10;
const wxBMP_8BPP_PALETTE: i32 = 11;
const wxBMP_4BPP: i32 =  4;
const wxBMP_1BPP: i32 =  1;
const wxBMP_1BPP_BW: i32 =  2;

//  ENUM: wxAuiPaneDockArtSetting
const wxAUI_DOCKART_SASH_SIZE: i32 = 0;
const wxAUI_DOCKART_CAPTION_SIZE: i32 = 1;
const wxAUI_DOCKART_GRIPPER_SIZE: i32 = 2;
const wxAUI_DOCKART_PANE_BORDER_SIZE: i32 = 3;
const wxAUI_DOCKART_PANE_BUTTON_SIZE: i32 = 4;
const wxAUI_DOCKART_BACKGROUND_COLOUR: i32 = 5;
const wxAUI_DOCKART_SASH_COLOUR: i32 = 6;
const wxAUI_DOCKART_ACTIVE_CAPTION_COLOUR: i32 = 7;
const wxAUI_DOCKART_ACTIVE_CAPTION_GRADIENT_COLOUR: i32 = 8;
const wxAUI_DOCKART_INACTIVE_CAPTION_COLOUR: i32 = 9;
const wxAUI_DOCKART_INACTIVE_CAPTION_GRADIENT_COLOUR: i32 = 10;
const wxAUI_DOCKART_ACTIVE_CAPTION_TEXT_COLOUR: i32 = 11;
const wxAUI_DOCKART_INACTIVE_CAPTION_TEXT_COLOUR: i32 = 12;
const wxAUI_DOCKART_BORDER_COLOUR: i32 = 13;
const wxAUI_DOCKART_GRIPPER_COLOUR: i32 = 14;
const wxAUI_DOCKART_CAPTION_FONT: i32 = 15;
const wxAUI_DOCKART_GRADIENT_TYPE: i32 = 16;
//  ENUM: wxAuiPaneDockArtGradients
const wxAUI_GRADIENT_NONE: i32 = 0;
const wxAUI_GRADIENT_VERTICAL: i32 = 1;
const wxAUI_GRADIENT_HORIZONTAL: i32 = 2;
//  ENUM: wxAuiPaneButtonState
const wxAUI_BUTTON_STATE_NORMAL: i32 = 0;
const wxAUI_BUTTON_STATE_HOVER: i32 = 1 << 1;
const wxAUI_BUTTON_STATE_PRESSED: i32 = 1 << 2;
const wxAUI_BUTTON_STATE_DISABLED: i32 = 1 << 3;
const wxAUI_BUTTON_STATE_HIDDEN: i32 = 1 << 4;
const wxAUI_BUTTON_STATE_CHECKED: i32 = 1 << 5;
//  ENUM: wxAuiButtonId
const wxAUI_BUTTON_CLOSE: i32 = 101;
const wxAUI_BUTTON_MAXIMIZE_RESTORE: i32 = 102;
const wxAUI_BUTTON_MINIMIZE: i32 = 103;
const wxAUI_BUTTON_PIN: i32 = 104;
const wxAUI_BUTTON_OPTIONS: i32 = 105;
const wxAUI_BUTTON_WINDOWLIST: i32 = 106;
const wxAUI_BUTTON_LEFT: i32 = 107;
const wxAUI_BUTTON_RIGHT: i32 = 108;
const wxAUI_BUTTON_UP: i32 = 109;
const wxAUI_BUTTON_DOWN: i32 = 110;
const wxAUI_BUTTON_CUSTOM1: i32 = 201;
const wxAUI_BUTTON_CUSTOM2: i32 = 202;
const wxAUI_BUTTON_CUSTOM3: i32 = 203;

//  ENUM: wxEOL
const wxEOL_NATIVE: i32 = 0;
const wxEOL_UNIX: i32 = 0 + 1;
const wxEOL_MAC: i32 = 0 + 2;
const wxEOL_DOS: i32 = 0 + 3;

//  ENUM: wxDirTraverseResult
const wxDIR_IGNORE: i32 = -1;
const wxDIR_STOP: i32 = -1 + 1;
const wxDIR_CONTINUE: i32 = -1 + 2;
//  ENUM: wxDirFlags
const wxDIR_FILES: i32 = 0x0001;
const wxDIR_DIRS: i32 = 0x0002;
const wxDIR_HIDDEN: i32 = 0x0004;
const wxDIR_DOTDOT: i32 = 0x0008;
const wxDIR_NO_FOLLOW: i32 = 0x0010;
const wxDIR_DEFAULT: i32 = wxDIR_FILES | wxDIR_DIRS | wxDIR_HIDDEN;

const wxBITMAP_SCREEN_DEPTH: i32 = (-1);

//  ENUM: wxRibbonPanelOption
const wxRIBBON_PANEL_NO_AUTO_MINIMISE: i32 = 0;
const wxRIBBON_PANEL_EXT_BUTTON: i32 = 0 + 1;
const wxRIBBON_PANEL_MINIMISE_BUTTON: i32 = 0 + 2;
const wxRIBBON_PANEL_STRETCH: i32 = 0 + 3;
const wxRIBBON_PANEL_FLEXIBLE: i32 = 0 + 4;
const wxRIBBON_PANEL_DEFAULT_STYLE: i32 = 0 + 5;

//  ENUM: State
const State_Idle: i32 = 0;
const State_Unauthorized: i32 = 0 + 1;
const State_Active: i32 = 0 + 2;
const State_Completed: i32 = 0 + 3;
const State_Failed: i32 = 0 + 4;
const State_Cancelled: i32 = 0 + 5;
//  ENUM: Storage
const Storage_Memory: i32 = 0;
const Storage_File: i32 = 0 + 1;
const Storage_None: i32 = 0 + 2;

//  ENUM: wxAuiToolBarStyle
const wxAUI_TB_TEXT: i32 = 1 << 0;
const wxAUI_TB_NO_TOOLTIPS: i32 = 1 << 1;
const wxAUI_TB_NO_AUTORESIZE: i32 = 1 << 2;
const wxAUI_TB_GRIPPER: i32 = 1 << 3;
const wxAUI_TB_OVERFLOW: i32 = 1 << 4;
const wxAUI_TB_VERTICAL: i32 = 1 << 5;
const wxAUI_TB_HORZ_LAYOUT: i32 = 1 << 6;
const wxAUI_TB_HORIZONTAL: i32 = 1 << 7;
const wxAUI_TB_PLAIN_BACKGROUND: i32 = 1 << 8;
const wxAUI_TB_HORZ_TEXT: i32 = (wxAUI_TB_HORZ_LAYOUT | wxAUI_TB_TEXT);
const wxAUI_ORIENTATION_MASK: i32 = (wxAUI_TB_VERTICAL | wxAUI_TB_HORIZONTAL);
const wxAUI_TB_DEFAULT_STYLE: i32 = 0;
//  ENUM: wxAuiToolBarArtSetting
const wxAUI_TBART_SEPARATOR_SIZE: i32 = 0;
const wxAUI_TBART_GRIPPER_SIZE: i32 = 1;
const wxAUI_TBART_OVERFLOW_SIZE: i32 = 2

    
    wxAUI_TBART_DROPDOWN_SIZE = 3;
//  ENUM: wxAuiToolBarToolTextOrientation
const wxAUI_TBTOOL_TEXT_LEFT: i32 = 0;
const wxAUI_TBTOOL_TEXT_RIGHT: i32 = 1;
const wxAUI_TBTOOL_TEXT_TOP: i32 = 2;
const wxAUI_TBTOOL_TEXT_BOTTOM: i32 = 3;

//  ENUM: Origin
const Origin_Unknown: i32 = 0;
const Origin_Keyboard: i32 = 0 + 1;
const Origin_HelpButton: i32 = 0 + 2;


//  ENUM: Source
const Source_Server: i32 = 0;
const Source_Proxy: i32 = 0 + 1;

const wxCHOICE_WIDTH: i32 = 150;
const wxCHOICE_HEIGHT: i32 = 200;
const wxCHOICEDLG_STYLE: i32 = (wxDEFAULT_DIALOG_STYLE | wxOK | wxCANCEL | wxCENTRE | wxRESIZE_BORDER);

//  ENUM: wxAuiToolBarStyle
const wxAUI_TB_TEXT: i32 = 1 << 0;
const wxAUI_TB_NO_TOOLTIPS: i32 = 1 << 1;
const wxAUI_TB_NO_AUTORESIZE: i32 = 1 << 2;
const wxAUI_TB_GRIPPER: i32 = 1 << 3;
const wxAUI_TB_OVERFLOW: i32 = 1 << 4;
const wxAUI_TB_VERTICAL: i32 = 1 << 5;
const wxAUI_TB_HORZ_LAYOUT: i32 = 1 << 6;
const wxAUI_TB_HORIZONTAL: i32 = 1 << 7;
const wxAUI_TB_PLAIN_BACKGROUND: i32 = 1 << 8;
const wxAUI_TB_HORZ_TEXT: i32 = (wxAUI_TB_HORZ_LAYOUT | wxAUI_TB_TEXT);
const wxAUI_ORIENTATION_MASK: i32 = (wxAUI_TB_VERTICAL | wxAUI_TB_HORIZONTAL);
const wxAUI_TB_DEFAULT_STYLE: i32 = 0;
//  ENUM: wxAuiToolBarArtSetting
const wxAUI_TBART_SEPARATOR_SIZE: i32 = 0;
const wxAUI_TBART_GRIPPER_SIZE: i32 = 1;
const wxAUI_TBART_OVERFLOW_SIZE: i32 = 2

    
    wxAUI_TBART_DROPDOWN_SIZE = 3;
//  ENUM: wxAuiToolBarToolTextOrientation
const wxAUI_TBTOOL_TEXT_LEFT: i32 = 0;
const wxAUI_TBTOOL_TEXT_RIGHT: i32 = 1;
const wxAUI_TBTOOL_TEXT_TOP: i32 = 2;
const wxAUI_TBTOOL_TEXT_BOTTOM: i32 = 3;
//  ENUM: wxAuiPaneDockArtSetting
const wxAUI_DOCKART_SASH_SIZE: i32 = 0;
const wxAUI_DOCKART_CAPTION_SIZE: i32 = 1;
const wxAUI_DOCKART_GRIPPER_SIZE: i32 = 2;
const wxAUI_DOCKART_PANE_BORDER_SIZE: i32 = 3;
const wxAUI_DOCKART_PANE_BUTTON_SIZE: i32 = 4;
const wxAUI_DOCKART_BACKGROUND_COLOUR: i32 = 5;
const wxAUI_DOCKART_SASH_COLOUR: i32 = 6;
const wxAUI_DOCKART_ACTIVE_CAPTION_COLOUR: i32 = 7;
const wxAUI_DOCKART_ACTIVE_CAPTION_GRADIENT_COLOUR: i32 = 8;
const wxAUI_DOCKART_INACTIVE_CAPTION_COLOUR: i32 = 9;
const wxAUI_DOCKART_INACTIVE_CAPTION_GRADIENT_COLOUR: i32 = 10;
const wxAUI_DOCKART_ACTIVE_CAPTION_TEXT_COLOUR: i32 = 11;
const wxAUI_DOCKART_INACTIVE_CAPTION_TEXT_COLOUR: i32 = 12;
const wxAUI_DOCKART_BORDER_COLOUR: i32 = 13;
const wxAUI_DOCKART_GRIPPER_COLOUR: i32 = 14;
const wxAUI_DOCKART_CAPTION_FONT: i32 = 15;
const wxAUI_DOCKART_GRADIENT_TYPE: i32 = 16;

//  ENUM: wxRibbonArtSetting
const wxRIBBON_ART_TAB_SEPARATION_SIZE: i32 = 0;
const wxRIBBON_ART_PAGE_BORDER_LEFT_SIZE: i32 = 0 + 1;
const wxRIBBON_ART_PAGE_BORDER_TOP_SIZE: i32 = 0 + 2;
const wxRIBBON_ART_PAGE_BORDER_RIGHT_SIZE: i32 = 0 + 3;
const wxRIBBON_ART_PAGE_BORDER_BOTTOM_SIZE: i32 = 0 + 4;
const wxRIBBON_ART_PANEL_X_SEPARATION_SIZE: i32 = 0 + 5;
const wxRIBBON_ART_PANEL_Y_SEPARATION_SIZE: i32 = 0 + 6;
const wxRIBBON_ART_TOOL_GROUP_SEPARATION_SIZE: i32 = 0 + 7;
const wxRIBBON_ART_GALLERY_BITMAP_PADDING_LEFT_SIZE: i32 = 0 + 8;
const wxRIBBON_ART_GALLERY_BITMAP_PADDING_RIGHT_SIZE: i32 = 0 + 9;
const wxRIBBON_ART_GALLERY_BITMAP_PADDING_TOP_SIZE: i32 = 0 + 10;
const wxRIBBON_ART_GALLERY_BITMAP_PADDING_BOTTOM_SIZE: i32 = 0 + 11;
const wxRIBBON_ART_PANEL_LABEL_FONT: i32 = 0 + 12;
const wxRIBBON_ART_BUTTON_BAR_LABEL_FONT: i32 = 0 + 13;
const wxRIBBON_ART_TAB_LABEL_FONT: i32 = 0 + 14;
const wxRIBBON_ART_BUTTON_BAR_LABEL_COLOUR: i32 = 0 + 15;
const wxRIBBON_ART_BUTTON_BAR_LABEL_DISABLED_COLOUR: i32 = 0 + 16;
const wxRIBBON_ART_BUTTON_BAR_HOVER_BORDER_COLOUR: i32 = 0 + 17;
const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_COLOUR: i32 = 0 + 18;
const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 19;
const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_COLOUR: i32 = 0 + 20;
const wxRIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 21;
const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BORDER_COLOUR: i32 = 0 + 22;
const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_COLOUR: i32 = 0 + 23;
const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 24;
const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_COLOUR: i32 = 0 + 25;
const wxRIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 26;
const wxRIBBON_ART_GALLERY_BORDER_COLOUR: i32 = 0 + 27;
const wxRIBBON_ART_GALLERY_HOVER_BACKGROUND_COLOUR: i32 = 0 + 28;
const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_COLOUR: i32 = 0 + 29;
const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 30;
const wxRIBBON_ART_GALLERY_BUTTON_BACKGROUND_TOP_COLOUR: i32 = 0 + 31;
const wxRIBBON_ART_GALLERY_BUTTON_FACE_COLOUR: i32 = 0 + 32;
const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_COLOUR: i32 = 0 + 33;
const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 34;
const wxRIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_TOP_COLOUR: i32 = 0 + 35;
const wxRIBBON_ART_GALLERY_BUTTON_HOVER_FACE_COLOUR: i32 = 0 + 36;
const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_COLOUR: i32 = 0 + 37;
const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 38;
const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_TOP_COLOUR: i32 = 0 + 39;
const wxRIBBON_ART_GALLERY_BUTTON_ACTIVE_FACE_COLOUR: i32 = 0 + 40;
const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_COLOUR: i32 = 0 + 41;
const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 42;
const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_TOP_COLOUR: i32 = 0 + 43;
const wxRIBBON_ART_GALLERY_BUTTON_DISABLED_FACE_COLOUR: i32 = 0 + 44;
const wxRIBBON_ART_GALLERY_ITEM_BORDER_COLOUR: i32 = 0 + 45;
const wxRIBBON_ART_TAB_LABEL_COLOUR: i32 = 0 + 46;
const wxRIBBON_ART_TAB_ACTIVE_LABEL_COLOUR: i32 = 0 + 47;
const wxRIBBON_ART_TAB_HOVER_LABEL_COLOUR: i32 = 0 + 48;
const wxRIBBON_ART_TAB_SEPARATOR_COLOUR: i32 = 0 + 49;
const wxRIBBON_ART_TAB_SEPARATOR_GRADIENT_COLOUR: i32 = 0 + 50;
const wxRIBBON_ART_TAB_CTRL_BACKGROUND_COLOUR: i32 = 0 + 51;
const wxRIBBON_ART_TAB_CTRL_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 52;
const wxRIBBON_ART_TAB_HOVER_BACKGROUND_TOP_COLOUR: i32 = 0 + 53;
const wxRIBBON_ART_TAB_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 54;
const wxRIBBON_ART_TAB_HOVER_BACKGROUND_COLOUR: i32 = 0 + 55;
const wxRIBBON_ART_TAB_HOVER_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 56;
const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_COLOUR: i32 = 0 + 57;
const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 58;
const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_COLOUR: i32 = 0 + 59;
const wxRIBBON_ART_TAB_ACTIVE_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 60;
const wxRIBBON_ART_TAB_BORDER_COLOUR: i32 = 0 + 61;
const wxRIBBON_ART_PANEL_BORDER_COLOUR: i32 = 0 + 62;
const wxRIBBON_ART_PANEL_BORDER_GRADIENT_COLOUR: i32 = 0 + 63;
const wxRIBBON_ART_PANEL_HOVER_BORDER_COLOUR: i32 = 0 + 64;
const wxRIBBON_ART_PANEL_HOVER_BORDER_GRADIENT_COLOUR: i32 = 0 + 65;
const wxRIBBON_ART_PANEL_MINIMISED_BORDER_COLOUR: i32 = 0 + 66;
const wxRIBBON_ART_PANEL_MINIMISED_BORDER_GRADIENT_COLOUR: i32 = 0 + 67;
const wxRIBBON_ART_PANEL_LABEL_BACKGROUND_COLOUR: i32 = 0 + 68;
const wxRIBBON_ART_PANEL_LABEL_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 69;
const wxRIBBON_ART_PANEL_LABEL_COLOUR: i32 = 0 + 70;
const wxRIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_COLOUR: i32 = 0 + 71;
const wxRIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 72;
const wxRIBBON_ART_PANEL_HOVER_LABEL_COLOUR: i32 = 0 + 73;
const wxRIBBON_ART_PANEL_MINIMISED_LABEL_COLOUR: i32 = 0 + 74;
const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_COLOUR: i32 = 0 + 75;
const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 76;
const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_COLOUR: i32 = 0 + 77;
const wxRIBBON_ART_PANEL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 78;
const wxRIBBON_ART_PAGE_BORDER_COLOUR: i32 = 0 + 79;
const wxRIBBON_ART_PAGE_BACKGROUND_TOP_COLOUR: i32 = 0 + 80;
const wxRIBBON_ART_PAGE_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 81;
const wxRIBBON_ART_PAGE_BACKGROUND_COLOUR: i32 = 0 + 82;
const wxRIBBON_ART_PAGE_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 83;
const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_COLOUR: i32 = 0 + 84;
const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 85;
const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_COLOUR: i32 = 0 + 86;
const wxRIBBON_ART_PAGE_HOVER_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 87;
const wxRIBBON_ART_TOOLBAR_BORDER_COLOUR: i32 = 0 + 88;
const wxRIBBON_ART_TOOLBAR_HOVER_BORDER_COLOUR: i32 = 0 + 89;
const wxRIBBON_ART_TOOLBAR_FACE_COLOUR: i32 = 0 + 90;
const wxRIBBON_ART_TOOL_BACKGROUND_TOP_COLOUR: i32 = 0 + 91;
const wxRIBBON_ART_TOOL_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 92;
const wxRIBBON_ART_TOOL_BACKGROUND_COLOUR: i32 = 0 + 93;
const wxRIBBON_ART_TOOL_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 94;
const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_COLOUR: i32 = 0 + 95;
const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 96;
const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_COLOUR: i32 = 0 + 97;
const wxRIBBON_ART_TOOL_HOVER_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 98;
const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_COLOUR: i32 = 0 + 99;
const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: i32 = 0 + 100;
const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_COLOUR: i32 = 0 + 101;
const wxRIBBON_ART_TOOL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: i32 = 0 + 102;
const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_COLOUR: i32 = 0 + 103;
const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_GRADIENT_COLOUR: i32 = 0 + 104;
const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_TOP_COLOUR: i32 = 0 + 105;
const wxRIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_GRADIENT_TOP_COLOUR: i32 = 0 + 106;
//  ENUM: wxRibbonScrollButtonStyle
const wxRIBBON_SCROLL_BTN_LEFT: i32 = 0;
const wxRIBBON_SCROLL_BTN_RIGHT: i32 = 1;
const wxRIBBON_SCROLL_BTN_UP: i32 = 2;
const wxRIBBON_SCROLL_BTN_DOWN: i32 = 3;
const wxRIBBON_SCROLL_BTN_DIRECTION_MASK: i32 = 3;
const wxRIBBON_SCROLL_BTN_NORMAL: i32 = 0;
const wxRIBBON_SCROLL_BTN_HOVERED: i32 = 4;
const wxRIBBON_SCROLL_BTN_ACTIVE: i32 = 8;
const wxRIBBON_SCROLL_BTN_STATE_MASK: i32 = 12;
const wxRIBBON_SCROLL_BTN_FOR_OTHER: i32 = 0;
const wxRIBBON_SCROLL_BTN_FOR_TABS: i32 = 16;
const wxRIBBON_SCROLL_BTN_FOR_PAGE: i32 = 32;
const wxRIBBON_SCROLL_BTN_FOR_MASK: i32 = 48;
//  ENUM: wxRibbonButtonKind
const wxRIBBON_BUTTON_NORMAL: i32 = 1 << 0;
const wxRIBBON_BUTTON_DROPDOWN: i32 = 1 << 1;
const wxRIBBON_BUTTON_HYBRID: i32 = wxRIBBON_BUTTON_NORMAL | wxRIBBON_BUTTON_DROPDOWN;
const wxRIBBON_BUTTON_TOGGLE: i32 = 1 << 2;

//  ENUM: wxWebViewZoom
const wxWEBVIEW_ZOOM_TINY: i32 = 0;
const wxWEBVIEW_ZOOM_SMALL: i32 = 0 + 1;
const wxWEBVIEW_ZOOM_MEDIUM: i32 = 0 + 2;
const wxWEBVIEW_ZOOM_LARGE: i32 = 0 + 3;
const wxWEBVIEW_ZOOM_LARGEST: i32 = 0 + 4;
//  ENUM: wxWebViewZoomType
const wxWEBVIEW_ZOOM_TYPE_LAYOUT: i32 = 0;
const wxWEBVIEW_ZOOM_TYPE_TEXT: i32 = 0 + 1;
//  ENUM: wxWebViewNavigationError
const wxWEBVIEW_NAV_ERR_CONNECTION: i32 = 0;
const wxWEBVIEW_NAV_ERR_CERTIFICATE: i32 = 0 + 1;
const wxWEBVIEW_NAV_ERR_AUTH: i32 = 0 + 2;
const wxWEBVIEW_NAV_ERR_SECURITY: i32 = 0 + 3;
const wxWEBVIEW_NAV_ERR_NOT_FOUND: i32 = 0 + 4;
const wxWEBVIEW_NAV_ERR_REQUEST: i32 = 0 + 5;
const wxWEBVIEW_NAV_ERR_USER_CANCELLED: i32 = 0 + 6;
const wxWEBVIEW_NAV_ERR_OTHER: i32 = 0 + 7;
//  ENUM: wxWebViewReloadFlags
const wxWEBVIEW_RELOAD_DEFAULT: i32 = 0;
const wxWEBVIEW_RELOAD_NO_CACHE: i32 = 0 + 1;
//  ENUM: wxWebViewFindFlags
const wxWEBVIEW_FIND_WRAP: i32 =             0x0001;
const wxWEBVIEW_FIND_ENTIRE_WORD: i32 =      0x0002;
const wxWEBVIEW_FIND_MATCH_CASE: i32 =       0x0004;
const wxWEBVIEW_FIND_HIGHLIGHT_RESULT: i32 = 0x0008;
const wxWEBVIEW_FIND_BACKWARDS: i32 =        0x0010;
const wxWEBVIEW_FIND_DEFAULT: i32 =          0;
//  ENUM: wxWebViewNavigationActionFlags
const wxWEBVIEW_NAV_ACTION_NONE: i32 = 0;
const wxWEBVIEW_NAV_ACTION_USER: i32 = 0 + 1;
const wxWEBVIEW_NAV_ACTION_OTHER: i32 = 0 + 2;
//  ENUM: wxWebViewUserScriptInjectionTime
const wxWEBVIEW_INJECT_AT_DOCUMENT_START: i32 = 0;
const wxWEBVIEW_INJECT_AT_DOCUMENT_END: i32 = 0 + 1;
//  ENUM: wxWebViewIE_EmulationLevel
const wxWEBVIEWIE_EMU_DEFAULT: i32 =    0;
const wxWEBVIEWIE_EMU_IE7: i32 =        7000;
const wxWEBVIEWIE_EMU_IE8: i32 =        8000;
const wxWEBVIEWIE_EMU_IE8_FORCE: i32 =  8888;
const wxWEBVIEWIE_EMU_IE9: i32 =        9000;
const wxWEBVIEWIE_EMU_IE9_FORCE: i32 =  9999;
const wxWEBVIEWIE_EMU_IE10: i32 =       10000;
const wxWEBVIEWIE_EMU_IE10_FORCE: i32 = 10001;
const wxWEBVIEWIE_EMU_IE11: i32 =       11000;
const wxWEBVIEWIE_EMU_IE11_FORCE: i32 = 11001;

//  ENUM: ConversionFlags
const Escape: i32 = 0x01;
const QuoteStrings: i32 = 0x02;

const wxTR_NO_BUTTONS: i32 = 0x0000;
const wxTR_HAS_BUTTONS: i32 = 0x0001;
const wxTR_NO_LINES: i32 = 0x0004;
const wxTR_LINES_AT_ROOT: i32 = 0x0008;
const wxTR_TWIST_BUTTONS: i32 = 0x0010;
const wxTR_SINGLE: i32 = 0x0000;
const wxTR_MULTIPLE: i32 = 0x0020;
const wxTR_HAS_VARIABLE_ROW_HEIGHT: i32 = 0x0080;
const wxTR_EDIT_LABELS: i32 = 0x0200;
const wxTR_ROW_LINES: i32 = 0x0400;
const wxTR_HIDE_ROOT: i32 = 0x0800;
const wxTR_FULL_ROW_HIGHLIGHT: i32 = 0x2000;
const wxTR_DEFAULT_STYLE: i32 = (wxTR_HAS_BUTTONS | wxTR_LINES_AT_ROOT);
//  ENUM: wxTreeItemIcon
const wxTreeItemIcon_Normal: i32 = 0;
const wxTreeItemIcon_Selected: i32 = 0 + 1;
const wxTreeItemIcon_Expanded: i32 = 0 + 2;
const wxTreeItemIcon_SelectedExpanded: i32 = 0 + 3;
const wxTreeItemIcon_Max: i32 = 0 + 4;

// NODEF: wxTheClipboard

//  ENUM: wxFSVolumeFlags
const wxFS_VOL_MOUNTED: i32 = 0x0001;
const wxFS_VOL_REMOVABLE: i32 = 0x0002;
const wxFS_VOL_READONLY: i32 = 0x0004;
const wxFS_VOL_REMOTE: i32 = 0x0008;
//  ENUM: wxFSVolumeKind
const wxFS_VOL_FLOPPY: i32 = 0;
const wxFS_VOL_DISK: i32 = 0 + 1;
const wxFS_VOL_CDROM: i32 = 0 + 2;
const wxFS_VOL_DVDROM: i32 = 0 + 3;
const wxFS_VOL_NETWORK: i32 = 0 + 4;
const wxFS_VOL_OTHER: i32 = 0 + 5;
const wxFS_VOL_MAX: i32 = 0 + 6;
//  ENUM: wxFSIconType
const wxFS_VOL_ICO_SMALL: i32 = 0;
const wxFS_VOL_ICO_LARGE: i32 = 0 + 1;
const wxFS_VOL_ICO_SEL_SMALL: i32 = 0 + 2;
const wxFS_VOL_ICO_SEL_LARGE: i32 = 0 + 3;
const wxFS_VOL_ICO_MAX: i32 = 0 + 4;

//  ENUM: wxAutomationInstanceFlags
const wxAutomationInstance_UseExistingOnly: i32 = 0;
const wxAutomationInstance_CreateIfNeeded: i32 = 1;
const wxAutomationInstance_SilentIfNone: i32 = 2;

//  ENUM: wxEdge
const wxLeft: i32 = 0;
const wxTop: i32 = 0 + 1;
const wxRight: i32 = 0 + 2;
const wxBottom: i32 = 0 + 3;
const wxWidth: i32 = 0 + 4;
const wxHeight: i32 = 0 + 5;
const wxCentre: i32 = 0 + 6;
const wxCenter: i32 = wxCentre;
const wxCentreX: i32 = wxCentre + 1;
const wxCentreY: i32 = wxCentre + 2;
//  ENUM: wxRelationship
const wxUnconstrained: i32 = 0;
const wxAsIs: i32 = 0 + 1;
const wxPercentOf: i32 = 0 + 2;
const wxAbove: i32 = 0 + 3;
const wxBelow: i32 = 0 + 4;
const wxLeftOf: i32 = 0 + 5;
const wxRightOf: i32 = 0 + 6;
const wxSameAs: i32 = 0 + 7;
const wxAbsolute: i32 = 0 + 8;

//  ENUM: @23
const wxCOL_WIDTH_DEFAULT: i32 = -1;
const wxCOL_WIDTH_AUTOSIZE: i32 = -2;
//  ENUM: @24
const wxCOL_RESIZABLE: i32 = 1;
const wxCOL_SORTABLE: i32 = 2;
const wxCOL_REORDERABLE: i32 = 4;
const wxCOL_HIDDEN: i32 = 8;
const wxCOL_DEFAULT_FLAGS: i32 = wxCOL_RESIZABLE | wxCOL_REORDERABLE;

const wxCHK_2STATE: i32 = 0x4000;
const wxCHK_3STATE: i32 = 0x1000;
const wxCHK_ALLOW_3RD_STATE_FOR_USER: i32 = 0x2000;
//  ENUM: wxCheckBoxState
const wxCHK_UNCHECKED: i32 = 0;
const wxCHK_CHECKED: i32 = 0 + 1;
const wxCHK_UNDETERMINED: i32 = 0 + 2;

//  ENUM: @43
const wxCONTROL_NONE: i32 = 0x00000000;
const wxCONTROL_DISABLED: i32 = 0x00000001;
const wxCONTROL_FOCUSED: i32 = 0x00000002;
const wxCONTROL_PRESSED: i32 = 0x00000004;
const wxCONTROL_SPECIAL: i32 = 0x00000008;
const wxCONTROL_ISDEFAULT: i32 = wxCONTROL_SPECIAL;
const wxCONTROL_ISSUBMENU: i32 = wxCONTROL_SPECIAL;
const wxCONTROL_EXPANDED: i32 = wxCONTROL_SPECIAL;
const wxCONTROL_SIZEGRIP: i32 = wxCONTROL_SPECIAL;
const wxCONTROL_FLAT: i32 = wxCONTROL_SPECIAL;
const wxCONTROL_CELL: i32 = wxCONTROL_SPECIAL;
const wxCONTROL_CURRENT: i32 = 0x00000010;
const wxCONTROL_SELECTED: i32 = 0x00000020;
const wxCONTROL_CHECKED: i32 = 0x00000040;
const wxCONTROL_CHECKABLE: i32 = 0x00000080;
const wxCONTROL_UNDETERMINED: i32 = wxCONTROL_CHECKABLE;
//  ENUM: wxTitleBarButton
const wxTITLEBAR_BUTTON_CLOSE: i32 = 0x01000000;
const wxTITLEBAR_BUTTON_MAXIMIZE: i32 = 0x02000000;
const wxTITLEBAR_BUTTON_ICONIZE: i32 = 0x04000000;
const wxTITLEBAR_BUTTON_RESTORE: i32 = 0x08000000;
const wxTITLEBAR_BUTTON_HELP: i32 = 0x10000000;
//  ENUM: wxHeaderSortIconType
const wxHDR_SORT_ICON_NONE: i32 = 0;
const wxHDR_SORT_ICON_UP: i32 = 0 + 1;
const wxHDR_SORT_ICON_DOWN: i32 = 0 + 2;

//  ENUM: @11
const wxDP_DEFAULT: i32 = 0;
const wxDP_SPIN: i32 = 1;
const wxDP_DROPDOWN: i32 = 2;
const wxDP_SHOWCENTURY: i32 = 4;
const wxDP_ALLOWNONE: i32 = 8;

//  ENUM: @6
const NUM_CUSTOM: i32 = 16;

// NODEF: wxT
// NODEF: wxT_2
// NODEF: wxS
// NODEF: _T

//  ENUM: wxNavigationKeyEventFlags
const IsBackward: i32 = 0x0000;
const IsForward: i32 = 0x0001;
const WinChange: i32 = 0x0002;
const FromTab: i32 = 0x0004;

const wxTBK_BUTTONBAR: i32 = 0x0100;
const wxTBK_HORZ_LAYOUT: i32 = 0x8000;

//  ENUM: @57
const NO_IMAGE: i32 = -1;

//  ENUM: wxSystemFont
const wxSYS_OEM_FIXED_FONT: i32 = 10;
const wxSYS_ANSI_FIXED_FONT: i32 = 10 + 1;
const wxSYS_ANSI_VAR_FONT: i32 = 10 + 2;
const wxSYS_SYSTEM_FONT: i32 = 10 + 3;
const wxSYS_DEVICE_DEFAULT_FONT: i32 = 10 + 4;
const wxSYS_DEFAULT_GUI_FONT: i32 = 10 + 5;
//  ENUM: wxSystemColour
const wxSYS_COLOUR_SCROLLBAR: i32 = 0;
const wxSYS_COLOUR_DESKTOP: i32 = 0 + 1;
const wxSYS_COLOUR_ACTIVECAPTION: i32 = 0 + 2;
const wxSYS_COLOUR_INACTIVECAPTION: i32 = 0 + 3;
const wxSYS_COLOUR_MENU: i32 = 0 + 4;
const wxSYS_COLOUR_WINDOW: i32 = 0 + 5;
const wxSYS_COLOUR_WINDOWFRAME: i32 = 0 + 6;
const wxSYS_COLOUR_MENUTEXT: i32 = 0 + 7;
const wxSYS_COLOUR_WINDOWTEXT: i32 = 0 + 8;
const wxSYS_COLOUR_CAPTIONTEXT: i32 = 0 + 9;
const wxSYS_COLOUR_ACTIVEBORDER: i32 = 0 + 10;
const wxSYS_COLOUR_INACTIVEBORDER: i32 = 0 + 11;
const wxSYS_COLOUR_APPWORKSPACE: i32 = 0 + 12;
const wxSYS_COLOUR_HIGHLIGHT: i32 = 0 + 13;
const wxSYS_COLOUR_HIGHLIGHTTEXT: i32 = 0 + 14;
const wxSYS_COLOUR_BTNFACE: i32 = 0 + 15;
const wxSYS_COLOUR_BTNSHADOW: i32 = 0 + 16;
const wxSYS_COLOUR_GRAYTEXT: i32 = 0 + 17;
const wxSYS_COLOUR_BTNTEXT: i32 = 0 + 18;
const wxSYS_COLOUR_INACTIVECAPTIONTEXT: i32 = 0 + 19;
const wxSYS_COLOUR_BTNHIGHLIGHT: i32 = 0 + 20;
const wxSYS_COLOUR_3DDKSHADOW: i32 = 0 + 21;
const wxSYS_COLOUR_3DLIGHT: i32 = 0 + 22;
const wxSYS_COLOUR_INFOTEXT: i32 = 0 + 23;
const wxSYS_COLOUR_INFOBK: i32 = 0 + 24;
const wxSYS_COLOUR_LISTBOX: i32 = 0 + 25;
const wxSYS_COLOUR_HOTLIGHT: i32 = 0 + 26;
const wxSYS_COLOUR_GRADIENTACTIVECAPTION: i32 = 0 + 27;
const wxSYS_COLOUR_GRADIENTINACTIVECAPTION: i32 = 0 + 28;
const wxSYS_COLOUR_MENUHILIGHT: i32 = 0 + 29;
const wxSYS_COLOUR_MENUBAR: i32 = 0 + 30;
const wxSYS_COLOUR_LISTBOXTEXT: i32 = 0 + 31;
const wxSYS_COLOUR_LISTBOXHIGHLIGHTTEXT: i32 = 0 + 32;
const wxSYS_COLOUR_BACKGROUND: i32 = wxSYS_COLOUR_DESKTOP;
const wxSYS_COLOUR_3DFACE: i32 = wxSYS_COLOUR_BTNFACE;
const wxSYS_COLOUR_3DSHADOW: i32 = wxSYS_COLOUR_BTNSHADOW;
const wxSYS_COLOUR_BTNHILIGHT: i32 = wxSYS_COLOUR_BTNHIGHLIGHT;
const wxSYS_COLOUR_3DHIGHLIGHT: i32 = wxSYS_COLOUR_BTNHIGHLIGHT;
const wxSYS_COLOUR_3DHILIGHT: i32 = wxSYS_COLOUR_BTNHIGHLIGHT;
const wxSYS_COLOUR_FRAMEBK: i32 = wxSYS_COLOUR_BTNFACE;
//  ENUM: wxSystemMetric
const wxSYS_MOUSE_BUTTONS: i32 = 0;
const wxSYS_BORDER_X: i32 = 0 + 1;
const wxSYS_BORDER_Y: i32 = 0 + 2;
const wxSYS_CURSOR_X: i32 = 0 + 3;
const wxSYS_CURSOR_Y: i32 = 0 + 4;
const wxSYS_DCLICK_X: i32 = 0 + 5;
const wxSYS_DCLICK_Y: i32 = 0 + 6;
const wxSYS_DRAG_X: i32 = 0 + 7;
const wxSYS_DRAG_Y: i32 = 0 + 8;
const wxSYS_EDGE_X: i32 = 0 + 9;
const wxSYS_EDGE_Y: i32 = 0 + 10;
const wxSYS_HSCROLL_ARROW_X: i32 = 0 + 11;
const wxSYS_HSCROLL_ARROW_Y: i32 = 0 + 12;
const wxSYS_HTHUMB_X: i32 = 0 + 13;
const wxSYS_ICON_X: i32 = 0 + 14;
const wxSYS_ICON_Y: i32 = 0 + 15;
const wxSYS_ICONSPACING_X: i32 = 0 + 16;
const wxSYS_ICONSPACING_Y: i32 = 0 + 17;
const wxSYS_WINDOWMIN_X: i32 = 0 + 18;
const wxSYS_WINDOWMIN_Y: i32 = 0 + 19;
const wxSYS_SCREEN_X: i32 = 0 + 20;
const wxSYS_SCREEN_Y: i32 = 0 + 21;
const wxSYS_FRAMESIZE_X: i32 = 0 + 22;
const wxSYS_FRAMESIZE_Y: i32 = 0 + 23;
const wxSYS_SMALLICON_X: i32 = 0 + 24;
const wxSYS_SMALLICON_Y: i32 = 0 + 25;
const wxSYS_HSCROLL_Y: i32 = 0 + 26;
const wxSYS_VSCROLL_X: i32 = 0 + 27;
const wxSYS_VSCROLL_ARROW_X: i32 = 0 + 28;
const wxSYS_VSCROLL_ARROW_Y: i32 = 0 + 29;
const wxSYS_VTHUMB_Y: i32 = 0 + 30;
const wxSYS_CAPTION_Y: i32 = 0 + 31;
const wxSYS_MENU_Y: i32 = 0 + 32;
const wxSYS_NETWORK_PRESENT: i32 = 0 + 33;
const wxSYS_PENWINDOWS_PRESENT: i32 = 0 + 34;
const wxSYS_SHOW_SOUNDS: i32 = 0 + 35;
const wxSYS_SWAP_BUTTONS: i32 = 0 + 36;
const wxSYS_DCLICK_MSEC: i32 = 0 + 37;
const wxSYS_CARET_ON_MSEC: i32 = 0 + 38;
const wxSYS_CARET_OFF_MSEC: i32 = 0 + 39;
const wxSYS_CARET_TIMEOUT_MSEC: i32 = 0 + 40;
//  ENUM: wxSystemFeature
const wxSYS_CAN_DRAW_FRAME_DECORATIONS: i32 = 1;
const wxSYS_CAN_ICONIZE_FRAME: i32 = 1 + 1;
const wxSYS_TABLET_PRESENT: i32 = 1 + 2;
//  ENUM: wxSystemScreenType
const wxSYS_SCREEN_NONE: i32 = 0;
const wxSYS_SCREEN_TINY: i32 = 0 + 1;
const wxSYS_SCREEN_PDA: i32 = 0 + 2;
const wxSYS_SCREEN_SMALL: i32 = 0 + 3;
const wxSYS_SCREEN_DESKTOP: i32 = 0 + 4;

//  ENUM: wxRegionContain
const wxOutRegion: i32 = 0;
const wxPartRegion: i32 = 1;
const wxInRegion: i32 = 2;

const wxFLP_OPEN: i32 = 0x0400;
const wxFLP_SAVE: i32 = 0x0800;
const wxFLP_OVERWRITE_PROMPT: i32 = 0x1000;
const wxFLP_FILE_MUST_EXIST: i32 = 0x2000;
const wxFLP_CHANGE_DIR: i32 = 0x4000;
const wxFLP_SMALL: i32 = wxPB_SMALL;
const wxFLP_USE_TEXTCTRL: i32 = (wxPB_USE_TEXTCTRL);
const wxFLP_DEFAULT_STYLE: i32 = (wxFLP_OPEN|wxFLP_FILE_MUST_EXIST);
const wxDIRP_DIR_MUST_EXIST: i32 = 0x0008;
const wxDIRP_CHANGE_DIR: i32 = 0x0010;
const wxDIRP_SMALL: i32 = wxPB_SMALL;
const wxDIRP_USE_TEXTCTRL: i32 = (wxPB_USE_TEXTCTRL);
const wxDIRP_DEFAULT_STYLE: i32 = (wxDIRP_DIR_MUST_EXIST);

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
const wxC2S_NAME: i32 = 1;
const wxC2S_CSS_SYNTAX: i32 = 2;
const wxC2S_HTML_SYNTAX: i32 = 4;

//  ENUM: wxRichTextOddEvenPage
const wxRICHTEXT_PAGE_ODD: i32 = 0;
const wxRICHTEXT_PAGE_EVEN: i32 = 0 + 1;
const wxRICHTEXT_PAGE_ALL: i32 = 0 + 2;
//  ENUM: wxRichTextPageLocation
const wxRICHTEXT_PAGE_LEFT: i32 = 0;
const wxRICHTEXT_PAGE_CENTRE: i32 = 0 + 1;
const wxRICHTEXT_PAGE_RIGHT: i32 = 0 + 2;

//  ENUM: @45
const Option_AllowPixelFontSize: i32 = 0x0001;

const wxPU_CONTAINS_CONTROLS: i32 = 0x0001;

const wxIMAGELIST_DRAW_NORMAL: i32 = 0x0001;
const wxIMAGELIST_DRAW_TRANSPARENT: i32 = 0x0002;
const wxIMAGELIST_DRAW_SELECTED: i32 = 0x0004;
const wxIMAGELIST_DRAW_FOCUSED: i32 = 0x0008;
//  ENUM: @31
const wxIMAGE_LIST_NORMAL: i32 = 0;
const wxIMAGE_LIST_SMALL: i32 = 0 + 1;
const wxIMAGE_LIST_STATE: i32 = 0 + 2;

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

const wxGRID_AUTOSIZE: i32 = (-1);
//  ENUM: wxGridCellFloatFormat
const wxGRID_FLOAT_FORMAT_FIXED: i32 = 0x0010;
const wxGRID_FLOAT_FORMAT_SCIENTIFIC: i32 = 0x0020;
const wxGRID_FLOAT_FORMAT_COMPACT: i32 = 0x0040;
const wxGRID_FLOAT_FORMAT_UPPER: i32 = 0x0080;
const wxGRID_FLOAT_FORMAT_DEFAULT: i32 = wxGRID_FLOAT_FORMAT_FIXED;
//  ENUM: wxGridTableRequest
const wxGRIDTABLE_NOTIFY_ROWS_INSERTED: i32 = 0;
const wxGRIDTABLE_NOTIFY_ROWS_APPENDED: i32 = 0 + 1;
const wxGRIDTABLE_NOTIFY_ROWS_DELETED: i32 = 0 + 2;
const wxGRIDTABLE_NOTIFY_COLS_INSERTED: i32 = 0 + 3;
const wxGRIDTABLE_NOTIFY_COLS_APPENDED: i32 = 0 + 4;
const wxGRIDTABLE_NOTIFY_COLS_DELETED: i32 = 0 + 5;
//  ENUM: wxGridRenderStyle
const wxGRID_DRAW_ROWS_HEADER: i32 = 0x001;
const wxGRID_DRAW_COLS_HEADER: i32 = 0x002;
const wxGRID_DRAW_CELL_LINES: i32 = 0x004;
const wxGRID_DRAW_BOX_RECT: i32 = 0x008;
const wxGRID_DRAW_SELECTION: i32 = 0x010;
const wxGRID_DRAW_DEFAULT: i32 = wxGRID_DRAW_ROWS_HEADER |
                          wxGRID_DRAW_COLS_HEADER |
                          wxGRID_DRAW_CELL_LINES |
                          wxGRID_DRAW_BOX_RECT;

// NODEF: wxCHECK_VERSION
// NODEF: wxCHECK_VERSION_FULL

//  SKIP: wxInvalidDateTime

const wxTIMER_CONTINUOUS: bool = false;
const wxTIMER_ONE_SHOT: bool = true;

// NODEF: wxDISABLE_DEBUG_LOGGING_IN_RELEASE_BUILD
//  ENUM: wxLogLevelValues
const wxLOG_FatalError: i32 = 0;
const wxLOG_Error: i32 = 0 + 1;
const wxLOG_Warning: i32 = 0 + 2;
const wxLOG_Message: i32 = 0 + 3;
const wxLOG_Status: i32 = 0 + 4;
const wxLOG_Info: i32 = 0 + 5;
const wxLOG_Debug: i32 = 0 + 6;
const wxLOG_Trace: i32 = 0 + 7;
const wxLOG_Progress: i32 = 0 + 8;
const wxLOG_User: i32 = 100;
const wxLOG_Max: i32 = 10000;

//  ENUM: @25
const wxHD_ALLOW_REORDER: i32 = 0x0001;
const wxHD_ALLOW_HIDE: i32 = 0x0002;
const wxHD_BITMAP_ON_RIGHT: i32 = 0x0004;
const wxHD_DEFAULT_STYLE: i32 = wxHD_ALLOW_REORDER;

//  ENUM: wxSignal
const wxSIGNONE: i32 = 0;
const wxSIGHUP: i32 = 0 + 1;
const wxSIGINT: i32 = 0 + 2;
const wxSIGQUIT: i32 = 0 + 3;
const wxSIGILL: i32 = 0 + 4;
const wxSIGTRAP: i32 = 0 + 5;
const wxSIGABRT: i32 = 0 + 6;
const wxSIGEMT: i32 = 0 + 7;
const wxSIGFPE: i32 = 0 + 8;
const wxSIGKILL: i32 = 0 + 9;
const wxSIGBUS: i32 = 0 + 10;
const wxSIGSEGV: i32 = 0 + 11;
const wxSIGSYS: i32 = 0 + 12;
const wxSIGPIPE: i32 = 0 + 13;
const wxSIGALRM: i32 = 0 + 14;
const wxSIGTERM: i32 = 0 + 15;
//  ENUM: wxKillError
const wxKILL_OK: i32 = 0;
const wxKILL_BAD_SIGNAL: i32 = 0 + 1;
const wxKILL_ACCESS_DENIED: i32 = 0 + 2;
const wxKILL_NO_PROCESS: i32 = 0 + 3;
const wxKILL_ERROR: i32 = 0 + 4;
//  ENUM: wxKillFlags
const wxKILL_NOCHILDREN: i32 = 0;
const wxKILL_CHILDREN: i32 = 1;
//  ENUM: wxShutdownFlags
const wxSHUTDOWN_FORCE: i32 = 1;
const wxSHUTDOWN_POWEROFF: i32 = 2;
const wxSHUTDOWN_REBOOT: i32 = 4;
const wxSHUTDOWN_LOGOFF: i32 = 8;
//  ENUM: @53
const wxStrip_Mnemonics: i32 = 1;
const wxStrip_Accel: i32 = 2;
const wxStrip_CJKMnemonics: i32 = 4;
const wxStrip_All: i32 = wxStrip_Mnemonics | wxStrip_Accel;
const wxStrip_Menu: i32 = wxStrip_All | wxStrip_CJKMnemonics;
//  ENUM: @54
const wxEXEC_ASYNC: i32 = 0;
const wxEXEC_SYNC: i32 = 1;
const wxEXEC_SHOW_CONSOLE: i32 = 2;
const wxEXEC_MAKE_GROUP_LEADER: i32 = 4;
const wxEXEC_NODISABLE: i32 = 8;
const wxEXEC_NOEVENTS: i32 = 16;
const wxEXEC_HIDE_CONSOLE: i32 = 32;
const wxEXEC_BLOCK: i32 = wxEXEC_SYNC | wxEXEC_NOEVENTS;

//  ENUM: wxFlexSizerGrowMode
const wxFLEX_GROWMODE_NONE: i32 = 0;
const wxFLEX_GROWMODE_SPECIFIED: i32 = 0 + 1;
const wxFLEX_GROWMODE_ALL: i32 = 0 + 2;

const wxCP_DEFAULT_STYLE: i32 = (wxTAB_TRAVERSAL | wxNO_BORDER);
const wxCP_NO_TLW_RESIZE: i32 = (0x0002);

//  ENUM: wxFindReplaceFlags
const wxFR_DOWN: i32 = 1;
const wxFR_WHOLEWORD: i32 = 2;
const wxFR_MATCHCASE: i32 = 4;
//  ENUM: wxFindReplaceDialogStyles
const wxFR_REPLACEDIALOG: i32 = 1;
const wxFR_NOUPDOWN: i32 = 2;
const wxFR_NOMATCHCASE: i32 = 4;
const wxFR_NOWHOLEWORD: i32 = 8;

//  ENUM: wxBOM
const wxBOM_Unknown: i32 = -1;
const wxBOM_None: i32 = -1 + 1;
const wxBOM_UTF32BE: i32 = -1 + 2;
const wxBOM_UTF32LE: i32 = -1 + 3;
const wxBOM_UTF16BE: i32 = -1 + 4;
const wxBOM_UTF16LE: i32 = -1 + 5;
const wxBOM_UTF8: i32 = -1 + 6;

//  ENUM: @48
const wxTP_DEFAULT: i32 = 0;

//  ENUM: @8
const MovableButton: i32 = 0x0001;
const BitmapButton: i32 = 0x0002;
const ButtonSpacing: i32 = 0x0004;
const TextIndent: i32 = 0x0008;
const PaintControl: i32 = 0x0010;
const PaintWritable: i32 = 0x0020;
const Borderless: i32 = 0x0040;
const All: i32 = MovableButton | BitmapButton | ButtonSpacing |
                          TextIndent | PaintControl | PaintWritable |
                          Borderless;

//  ENUM: wxZipMethod
const wxZIP_METHOD_STORE: i32 = 0;
const wxZIP_METHOD_SHRINK: i32 = 0 + 1;
const wxZIP_METHOD_REDUCE1: i32 = 0 + 2;
const wxZIP_METHOD_REDUCE2: i32 = 0 + 3;
const wxZIP_METHOD_REDUCE3: i32 = 0 + 4;
const wxZIP_METHOD_REDUCE4: i32 = 0 + 5;
const wxZIP_METHOD_IMPLODE: i32 = 0 + 6;
const wxZIP_METHOD_TOKENIZE: i32 = 0 + 7;
const wxZIP_METHOD_DEFLATE: i32 = 0 + 8;
const wxZIP_METHOD_DEFLATE64: i32 = 0 + 9;
const wxZIP_METHOD_BZIP2: i32 = 12;
const wxZIP_METHOD_DEFAULT: i32 = 0xffff;
//  ENUM: wxZipSystem
const wxZIP_SYSTEM_MSDOS: i32 = 0;
const wxZIP_SYSTEM_AMIGA: i32 = 0 + 1;
const wxZIP_SYSTEM_OPENVMS: i32 = 0 + 2;
const wxZIP_SYSTEM_UNIX: i32 = 0 + 3;
const wxZIP_SYSTEM_VM_CMS: i32 = 0 + 4;
const wxZIP_SYSTEM_ATARI_ST: i32 = 0 + 5;
const wxZIP_SYSTEM_OS2_HPFS: i32 = 0 + 6;
const wxZIP_SYSTEM_MACINTOSH: i32 = 0 + 7;
const wxZIP_SYSTEM_Z_SYSTEM: i32 = 0 + 8;
const wxZIP_SYSTEM_CPM: i32 = 0 + 9;
const wxZIP_SYSTEM_WINDOWS_NTFS: i32 = 0 + 10;
const wxZIP_SYSTEM_MVS: i32 = 0 + 11;
const wxZIP_SYSTEM_VSE: i32 = 0 + 12;
const wxZIP_SYSTEM_ACORN_RISC: i32 = 0 + 13;
const wxZIP_SYSTEM_VFAT: i32 = 0 + 14;
const wxZIP_SYSTEM_ALTERNATE_MVS: i32 = 0 + 15;
const wxZIP_SYSTEM_BEOS: i32 = 0 + 16;
const wxZIP_SYSTEM_TANDEM: i32 = 0 + 17;
const wxZIP_SYSTEM_OS_400: i32 = 0 + 18;
//  ENUM: wxZipAttributes
const wxZIP_A_RDONLY: i32 = 0x01;
const wxZIP_A_HIDDEN: i32 = 0x02;
const wxZIP_A_SYSTEM: i32 = 0x04;
const wxZIP_A_SUBDIR: i32 = 0x10;
const wxZIP_A_ARCH: i32 = 0x20;
const wxZIP_A_MASK: i32 = 0x37;
//  ENUM: wxZipFlags
const wxZIP_ENCRYPTED: i32 = 0x0001;
const wxZIP_DEFLATE_NORMAL: i32 = 0x0000;
const wxZIP_DEFLATE_EXTRA: i32 = 0x0002;
const wxZIP_DEFLATE_FAST: i32 = 0x0004;
const wxZIP_DEFLATE_SUPERFAST: i32 = 0x0006;
const wxZIP_DEFLATE_MASK: i32 = 0x0006;
const wxZIP_SUMS_FOLLOW: i32 = 0x0008;
const wxZIP_ENHANCED: i32 = 0x0010;
const wxZIP_PATCH: i32 = 0x0020;
const wxZIP_STRONG_ENC: i32 = 0x0040;
const wxZIP_UNUSED: i32 = 0x0F80;
const wxZIP_RESERVED: i32 = 0xF000;
//  ENUM: wxZipArchiveFormat
const wxZIP_FORMAT_DEFAULT: i32 = 0;
const wxZIP_FORMAT_ZIP64: i32 = 0 + 1;

const wxDEFAULT_DELIMITERS: &str = " \t\r\n";
//  ENUM: wxStringTokenizerMode
const wxTOKEN_INVALID: i32 = -1;
const wxTOKEN_DEFAULT: i32 = -1 + 1;
const wxTOKEN_RET_EMPTY: i32 = -1 + 2;
const wxTOKEN_RET_EMPTY_ALL: i32 = -1 + 3;
const wxTOKEN_RET_DELIMS: i32 = -1 + 4;
const wxTOKEN_STRTOK: i32 = -1 + 5;

//  ENUM: wxTipKind
const wxTipKind_None: i32 = 0;
const wxTipKind_TopLeft: i32 = 0 + 1;
const wxTipKind_Top: i32 = 0 + 2;
const wxTipKind_TopRight: i32 = 0 + 3;
const wxTipKind_BottomLeft: i32 = 0 + 4;
const wxTipKind_Bottom: i32 = 0 + 5;
const wxTipKind_BottomRight: i32 = 0 + 6;
const wxTipKind_Auto: i32 = 0 + 7;

//  ENUM: wxFontFamily
const wxFONTFAMILY_DEFAULT: i32 = wxDEFAULT;
const wxFONTFAMILY_DECORATIVE: i32 = wxDECORATIVE;
const wxFONTFAMILY_ROMAN: i32 = wxROMAN;
const wxFONTFAMILY_SCRIPT: i32 = wxSCRIPT;
const wxFONTFAMILY_SWISS: i32 = wxSWISS;
const wxFONTFAMILY_MODERN: i32 = wxMODERN;
const wxFONTFAMILY_TELETYPE: i32 = wxTELETYPE;
const wxFONTFAMILY_MAX: i32 = wxTELETYPE + 1;
const wxFONTFAMILY_UNKNOWN: i32 = wxFONTFAMILY_MAX;
//  ENUM: wxFontStyle
const wxFONTSTYLE_NORMAL: i32 = wxNORMAL;
const wxFONTSTYLE_ITALIC: i32 = wxITALIC;
const wxFONTSTYLE_SLANT: i32 = wxSLANT;
const wxFONTSTYLE_MAX: i32 = wxSLANT + 1;
//  ENUM: wxFontWeight
const wxFONTWEIGHT_INVALID: i32 = 0;
const wxFONTWEIGHT_THIN: i32 = 100;
const wxFONTWEIGHT_EXTRALIGHT: i32 = 200;
const wxFONTWEIGHT_LIGHT: i32 = 300;
const wxFONTWEIGHT_NORMAL: i32 = 400;
const wxFONTWEIGHT_MEDIUM: i32 = 500;
const wxFONTWEIGHT_SEMIBOLD: i32 = 600;
const wxFONTWEIGHT_BOLD: i32 = 700;
const wxFONTWEIGHT_EXTRABOLD: i32 = 800;
const wxFONTWEIGHT_HEAVY: i32 = 900;
const wxFONTWEIGHT_EXTRAHEAVY: i32 = 1000;
const wxFONTWEIGHT_MAX: i32 = wxFONTWEIGHT_EXTRAHEAVY;
//  ENUM: wxFontSymbolicSize
const wxFONTSIZE_XX_SMALL: i32 = -3;
const wxFONTSIZE_X_SMALL: i32 = -3 + 1;
const wxFONTSIZE_SMALL: i32 = -3 + 2;
const wxFONTSIZE_MEDIUM: i32 = -3 + 3;
const wxFONTSIZE_LARGE: i32 = -3 + 4;
const wxFONTSIZE_X_LARGE: i32 = -3 + 5;
const wxFONTSIZE_XX_LARGE: i32 = -3 + 6;
//  ENUM: wxFontFlag
const wxFONTFLAG_DEFAULT: i32 = 0;
const wxFONTFLAG_ITALIC: i32 = 1 << 0;
const wxFONTFLAG_SLANT: i32 = 1 << 1;
const wxFONTFLAG_LIGHT: i32 = 1 << 2;
const wxFONTFLAG_BOLD: i32 = 1 << 3;
const wxFONTFLAG_ANTIALIASED: i32 = 1 << 4;
const wxFONTFLAG_NOT_ANTIALIASED: i32 = 1 << 5;
const wxFONTFLAG_UNDERLINED: i32 = 1 << 6;
const wxFONTFLAG_STRIKETHROUGH: i32 = 1 << 7;
const wxFONTFLAG_MASK: i32 = wxFONTFLAG_ITALIC             |
                      wxFONTFLAG_SLANT              |
                      wxFONTFLAG_LIGHT              |
                      wxFONTFLAG_BOLD               |
                      wxFONTFLAG_ANTIALIASED        |
                      wxFONTFLAG_NOT_ANTIALIASED    |
                      wxFONTFLAG_UNDERLINED         |
                      wxFONTFLAG_STRIKETHROUGH;
//  ENUM: wxFontEncoding
const wxFONTENCODING_SYSTEM: i32 = -1;
const wxFONTENCODING_DEFAULT: i32 = -1 + 1;
const wxFONTENCODING_ISO8859_1: i32 = -1 + 2;
const wxFONTENCODING_ISO8859_2: i32 = -1 + 3;
const wxFONTENCODING_ISO8859_3: i32 = -1 + 4;
const wxFONTENCODING_ISO8859_4: i32 = -1 + 5;
const wxFONTENCODING_ISO8859_5: i32 = -1 + 6;
const wxFONTENCODING_ISO8859_6: i32 = -1 + 7;
const wxFONTENCODING_ISO8859_7: i32 = -1 + 8;
const wxFONTENCODING_ISO8859_8: i32 = -1 + 9;
const wxFONTENCODING_ISO8859_9: i32 = -1 + 10;
const wxFONTENCODING_ISO8859_10: i32 = -1 + 11;
const wxFONTENCODING_ISO8859_11: i32 = -1 + 12;
const wxFONTENCODING_ISO8859_12: i32 = -1 + 13;
const wxFONTENCODING_ISO8859_13: i32 = -1 + 14;
const wxFONTENCODING_ISO8859_14: i32 = -1 + 15;
const wxFONTENCODING_ISO8859_15: i32 = -1 + 16;
const wxFONTENCODING_ISO8859_MAX: i32 = -1 + 17;
const wxFONTENCODING_KOI8: i32 = -1 + 18;
const wxFONTENCODING_KOI8_U: i32 = -1 + 19;
const wxFONTENCODING_ALTERNATIVE: i32 = -1 + 20;
const wxFONTENCODING_BULGARIAN: i32 = -1 + 21;
const wxFONTENCODING_CP437: i32 = -1 + 22;
const wxFONTENCODING_CP850: i32 = -1 + 23;
const wxFONTENCODING_CP852: i32 = -1 + 24;
const wxFONTENCODING_CP855: i32 = -1 + 25;
const wxFONTENCODING_CP866: i32 = -1 + 26;
const wxFONTENCODING_CP874: i32 = -1 + 27;
const wxFONTENCODING_CP932: i32 = -1 + 28;
const wxFONTENCODING_CP936: i32 = -1 + 29;
const wxFONTENCODING_CP949: i32 = -1 + 30;
const wxFONTENCODING_CP950: i32 = -1 + 31;
const wxFONTENCODING_CP1250: i32 = -1 + 32;
const wxFONTENCODING_CP1251: i32 = -1 + 33;
const wxFONTENCODING_CP1252: i32 = -1 + 34;
const wxFONTENCODING_CP1253: i32 = -1 + 35;
const wxFONTENCODING_CP1254: i32 = -1 + 36;
const wxFONTENCODING_CP1255: i32 = -1 + 37;
const wxFONTENCODING_CP1256: i32 = -1 + 38;
const wxFONTENCODING_CP1257: i32 = -1 + 39;
const wxFONTENCODING_CP1258: i32 = -1 + 40;
const wxFONTENCODING_CP1361: i32 = -1 + 41;
const wxFONTENCODING_CP12_MAX: i32 = -1 + 42;
const wxFONTENCODING_UTF7: i32 = -1 + 43;
const wxFONTENCODING_UTF8: i32 = -1 + 44;
const wxFONTENCODING_EUC_JP: i32 = -1 + 45;
const wxFONTENCODING_UTF16BE: i32 = -1 + 46;
const wxFONTENCODING_UTF16LE: i32 = -1 + 47;
const wxFONTENCODING_UTF32BE: i32 = -1 + 48;
const wxFONTENCODING_UTF32LE: i32 = -1 + 49;
const wxFONTENCODING_MACROMAN: i32 = -1 + 50;
const wxFONTENCODING_MACJAPANESE: i32 = -1 + 51;
const wxFONTENCODING_MACCHINESETRAD: i32 = -1 + 52;
const wxFONTENCODING_MACKOREAN: i32 = -1 + 53;
const wxFONTENCODING_MACARABIC: i32 = -1 + 54;
const wxFONTENCODING_MACHEBREW: i32 = -1 + 55;
const wxFONTENCODING_MACGREEK: i32 = -1 + 56;
const wxFONTENCODING_MACCYRILLIC: i32 = -1 + 57;
const wxFONTENCODING_MACDEVANAGARI: i32 = -1 + 58;
const wxFONTENCODING_MACGURMUKHI: i32 = -1 + 59;
const wxFONTENCODING_MACGUJARATI: i32 = -1 + 60;
const wxFONTENCODING_MACORIYA: i32 = -1 + 61;
const wxFONTENCODING_MACBENGALI: i32 = -1 + 62;
const wxFONTENCODING_MACTAMIL: i32 = -1 + 63;
const wxFONTENCODING_MACTELUGU: i32 = -1 + 64;
const wxFONTENCODING_MACKANNADA: i32 = -1 + 65;
const wxFONTENCODING_MACMALAJALAM: i32 = -1 + 66;
const wxFONTENCODING_MACSINHALESE: i32 = -1 + 67;
const wxFONTENCODING_MACBURMESE: i32 = -1 + 68;
const wxFONTENCODING_MACKHMER: i32 = -1 + 69;
const wxFONTENCODING_MACTHAI: i32 = -1 + 70;
const wxFONTENCODING_MACLAOTIAN: i32 = -1 + 71;
const wxFONTENCODING_MACGEORGIAN: i32 = -1 + 72;
const wxFONTENCODING_MACARMENIAN: i32 = -1 + 73;
const wxFONTENCODING_MACCHINESESIMP: i32 = -1 + 74;
const wxFONTENCODING_MACTIBETAN: i32 = -1 + 75;
const wxFONTENCODING_MACMONGOLIAN: i32 = -1 + 76;
const wxFONTENCODING_MACETHIOPIC: i32 = -1 + 77;
const wxFONTENCODING_MACCENTRALEUR: i32 = -1 + 78;
const wxFONTENCODING_MACVIATNAMESE: i32 = -1 + 79;
const wxFONTENCODING_MACARABICEXT: i32 = -1 + 80;
const wxFONTENCODING_MACSYMBOL: i32 = -1 + 81;
const wxFONTENCODING_MACDINGBATS: i32 = -1 + 82;
const wxFONTENCODING_MACTURKISH: i32 = -1 + 83;
const wxFONTENCODING_MACCROATIAN: i32 = -1 + 84;
const wxFONTENCODING_MACICELANDIC: i32 = -1 + 85;
const wxFONTENCODING_MACROMANIAN: i32 = -1 + 86;
const wxFONTENCODING_MACCELTIC: i32 = -1 + 87;
const wxFONTENCODING_MACGAELIC: i32 = -1 + 88;
const wxFONTENCODING_MACKEYBOARD: i32 = -1 + 89;
const wxFONTENCODING_ISO2022_JP: i32 = -1 + 90;
const wxFONTENCODING_MAX: i32 = -1 + 91;
const wxFONTENCODING_MACMIN: i32 = wxFONTENCODING_MACROMAN;
const wxFONTENCODING_MACMAX: i32 = wxFONTENCODING_MACKEYBOARD;
const wxFONTENCODING_UTF16: i32 = wxFONTENCODING_MACKEYBOARD + 1;
const wxFONTENCODING_UTF32: i32 = wxFONTENCODING_MACKEYBOARD + 2;
const wxFONTENCODING_UNICODE: i32 = wxFONTENCODING_MACKEYBOARD + 3;
const wxFONTENCODING_GB2312: i32 = wxFONTENCODING_CP936;
const wxFONTENCODING_BIG5: i32 = wxFONTENCODING_CP950;
const wxFONTENCODING_SHIFT_JIS: i32 = wxFONTENCODING_CP932;
const wxFONTENCODING_EUC_KR: i32 = wxFONTENCODING_CP949;
const wxFONTENCODING_JOHAB: i32 = wxFONTENCODING_CP1361;
const wxFONTENCODING_VIETNAMESE: i32 = wxFONTENCODING_CP1258;

//  ENUM: wxURIHostType
const wxURI_REGNAME: i32 = 0;
const wxURI_IPV4ADDRESS: i32 = 0 + 1;
const wxURI_IPV6ADDRESS: i32 = 0 + 2;
const wxURI_IPVFUTURE: i32 = 0 + 3;

//  ENUM: wxFileHistoryMenuPathStyle
const wxFH_PATH_SHOW_IF_DIFFERENT: i32 = 0;
const wxFH_PATH_SHOW_NEVER: i32 = 0 + 1;
const wxFH_PATH_SHOW_ALWAYS: i32 = 0 + 2;

// NODEF: wxDROP_ICON
//  ENUM: @13
const wxDrag_CopyOnly: i32 = 0;
const wxDrag_AllowMove: i32 = 1;
const wxDrag_DefaultMove: i32 = 3;
//  ENUM: wxDragResult
const wxDragError: i32 = 0;
const wxDragNone: i32 = 0 + 1;
const wxDragCopy: i32 = 0 + 2;
const wxDragMove: i32 = 0 + 3;
const wxDragLink: i32 = 0 + 4;
const wxDragCancel: i32 = 0 + 5;

//  ENUM: @12
const wxDIRCTRL_DIR_ONLY: i32 = 0x0010;
const wxDIRCTRL_SELECT_FIRST: i32 = 0x0020;
const wxDIRCTRL_SHOW_FILTERS: i32 = 0x0040;
const wxDIRCTRL_3D_INTERNAL: i32 = 0x0080;
const wxDIRCTRL_EDIT_LABELS: i32 = 0x0100;
const wxDIRCTRL_MULTIPLE: i32 = 0x0200;
const wxDIRCTRL_DEFAULT_STYLE: i32 = wxDIRCTRL_3D_INTERNAL;

//  ENUM: Origin
const Program: i32 = 0;
const Key: i32 = 0 + 1;
const Mouse: i32 = 0 + 2;

const wxFRAME_SHAPED: i32 = 0x0010;

//  ENUM: wxAuiManagerDock
const wxAUI_DOCK_NONE: i32 = 0;
const wxAUI_DOCK_TOP: i32 = 1;
const wxAUI_DOCK_RIGHT: i32 = 2;
const wxAUI_DOCK_BOTTOM: i32 = 3;
const wxAUI_DOCK_LEFT: i32 = 4;
const wxAUI_DOCK_CENTER: i32 = 5;
const wxAUI_DOCK_CENTRE: i32 = wxAUI_DOCK_CENTER;
//  ENUM: wxAuiManagerOption
const wxAUI_MGR_ALLOW_FLOATING: i32 = 1 << 0;
const wxAUI_MGR_ALLOW_ACTIVE_PANE: i32 = 1 << 1;
const wxAUI_MGR_TRANSPARENT_DRAG: i32 = 1 << 2;
const wxAUI_MGR_TRANSPARENT_HINT: i32 = 1 << 3;
const wxAUI_MGR_VENETIAN_BLINDS_HINT: i32 = 1 << 4;
const wxAUI_MGR_RECTANGLE_HINT: i32 = 1 << 5;
const wxAUI_MGR_HINT_FADE: i32 = 1 << 6;
const wxAUI_MGR_NO_VENETIAN_BLINDS_FADE: i32 = 1 << 7;
const wxAUI_MGR_LIVE_RESIZE: i32 = 1 << 8;
const wxAUI_MGR_DEFAULT: i32 = wxAUI_MGR_ALLOW_FLOATING |
                        wxAUI_MGR_TRANSPARENT_HINT |
                        wxAUI_MGR_HINT_FADE |
                        wxAUI_MGR_NO_VENETIAN_BLINDS_FADE;

const wxTextEntryDialogStyle: i32 = (wxOK | wxCANCEL | wxCENTRE);

const wxHLB_DEFAULT_STYLE: i32 = wxBORDER_SUNKEN;
const wxHLB_MULTIPLE: i32 = wxLB_MULTIPLE;

//  ENUM: TransferMode
const NONE: i32 = 0;
const ASCII: i32 = 0 + 1;
const BINARY: i32 = 0 + 2;

const wxPG_DEFAULT_STYLE: i32 = (0);
const wxPGMAN_DEFAULT_STYLE: i32 = (0);
//  SKIP: wxPGVFBFlags
//  ENUM: wxPG_WINDOW_STYLES
const wxPG_AUTO_SORT: i32 = 0x00000010;
const wxPG_HIDE_CATEGORIES: i32 = 0x00000020;
const wxPG_ALPHABETIC_MODE: i32 = (wxPG_HIDE_CATEGORIES|wxPG_AUTO_SORT);
const wxPG_BOLD_MODIFIED: i32 = 0x00000040;
const wxPG_SPLITTER_AUTO_CENTER: i32 = 0x00000080;
const wxPG_TOOLTIPS: i32 = 0x00000100;
const wxPG_HIDE_MARGIN: i32 = 0x00000200;
const wxPG_STATIC_SPLITTER: i32 = 0x00000400;
const wxPG_STATIC_LAYOUT: i32 = (wxPG_HIDE_MARGIN|wxPG_STATIC_SPLITTER);
const wxPG_LIMITED_EDITING: i32 = 0x00000800;
const wxPG_TOOLBAR: i32 = 0x00001000;
const wxPG_DESCRIPTION: i32 = 0x00002000;
const wxPG_NO_INTERNAL_BORDER: i32 = 0x00004000;
const wxPG_WINDOW_STYLE_MASK: i32 = wxPG_AUTO_SORT|wxPG_HIDE_CATEGORIES|wxPG_BOLD_MODIFIED|
                         wxPG_SPLITTER_AUTO_CENTER|wxPG_TOOLTIPS|wxPG_HIDE_MARGIN|
                         wxPG_STATIC_SPLITTER|wxPG_LIMITED_EDITING|wxPG_TOOLBAR|
                         wxPG_DESCRIPTION|wxPG_NO_INTERNAL_BORDER;
//  ENUM: wxPG_EX_WINDOW_STYLES
const wxPG_EX_INIT_NOCAT: i32 = 0x00001000;
const wxPG_EX_NO_FLAT_TOOLBAR: i32 = 0x00002000;
const wxPG_EX_MODE_BUTTONS: i32 = 0x00008000;
const wxPG_EX_HELP_AS_TOOLTIPS: i32 = 0x00010000;
const wxPG_EX_NATIVE_DOUBLE_BUFFERING: i32 = 0x00080000;
const wxPG_EX_AUTO_UNSPECIFIED_VALUES: i32 = 0x00200000;
const wxPG_EX_WRITEONLY_BUILTIN_ATTRIBUTES: i32 = 0x00400000;
const wxPG_EX_HIDE_PAGE_BUTTONS: i32 = 0x01000000;
const wxPG_EX_MULTIPLE_SELECTION: i32 = 0x02000000;
const wxPG_EX_ENABLE_TLP_TRACKING: i32 = 0x04000000;
const wxPG_EX_NO_TOOLBAR_DIVIDER: i32 = 0x04000000;
const wxPG_EX_TOOLBAR_SEPARATOR: i32 = 0x08000000;
const wxPG_EX_ALWAYS_ALLOW_FOCUS: i32 = 0x00100000;
const wxPG_EX_WINDOW_PG_STYLE_MASK: i32 = wxPG_EX_INIT_NOCAT|wxPG_EX_HELP_AS_TOOLTIPS|wxPG_EX_NATIVE_DOUBLE_BUFFERING|
                               wxPG_EX_AUTO_UNSPECIFIED_VALUES|wxPG_EX_WRITEONLY_BUILTIN_ATTRIBUTES|
                               wxPG_EX_MULTIPLE_SELECTION|wxPG_EX_ENABLE_TLP_TRACKING|wxPG_EX_ALWAYS_ALLOW_FOCUS;
const wxPG_EX_WINDOW_PGMAN_STYLE_MASK: i32 = wxPG_EX_NO_FLAT_TOOLBAR|wxPG_EX_MODE_BUTTONS|wxPG_EX_HIDE_PAGE_BUTTONS|
                                  wxPG_EX_NO_TOOLBAR_DIVIDER|wxPG_EX_TOOLBAR_SEPARATOR;
const wxPG_EX_WINDOW_STYLE_MASK: i32 = wxPG_EX_WINDOW_PG_STYLE_MASK|wxPG_EX_WINDOW_PGMAN_STYLE_MASK;
//  ENUM: wxPG_VALIDATION_FAILURE_BEHAVIOR_FLAGS
const wxPG_VFB_STAY_IN_PROPERTY: i32 = 0x01;
const wxPG_VFB_BEEP: i32 = 0x02;
const wxPG_VFB_MARK_CELL: i32 = 0x04;
const wxPG_VFB_SHOW_MESSAGE: i32 = 0x08;
const wxPG_VFB_SHOW_MESSAGEBOX: i32 = 0x10;
const wxPG_VFB_SHOW_MESSAGE_ON_STATUSBAR: i32 = 0x20;
const wxPG_VFB_DEFAULT: i32 = wxPG_VFB_MARK_CELL |
                                      wxPG_VFB_SHOW_MESSAGEBOX;
//  ENUM: wxPG_KEYBOARD_ACTIONS
const wxPG_ACTION_INVALID: i32 = 0;
const wxPG_ACTION_NEXT_PROPERTY: i32 = 0 + 1;
const wxPG_ACTION_PREV_PROPERTY: i32 = 0 + 2;
const wxPG_ACTION_EXPAND_PROPERTY: i32 = 0 + 3;
const wxPG_ACTION_COLLAPSE_PROPERTY: i32 = 0 + 4;
const wxPG_ACTION_CANCEL_EDIT: i32 = 0 + 5;
const wxPG_ACTION_EDIT: i32 = 0 + 6;
const wxPG_ACTION_PRESS_BUTTON: i32 = 0 + 7;
const wxPG_ACTION_MAX: i32 = 0 + 8;

//  ENUM: wxXmlResourceFlags
const wxXRC_USE_LOCALE: i32 = 1;
const wxXRC_NO_SUBCLASSING: i32 = 2;
const wxXRC_NO_RELOADING: i32 = 4;
const wxXRC_USE_ENVVARS: i32 = 8;

// NODEF: wxCHECK_GCC_VERSION
// NODEF: wxCHECK_SUNCC_VERSION
// NODEF: wxCHECK_VISUALC_VERSION
// NODEF: wxCHECK_W32API_VERSION

//  ENUM: wxStockLabelQueryFlag
const wxSTOCK_NOFLAGS: i32 = 0;
const wxSTOCK_WITH_MNEMONIC: i32 = 1;
const wxSTOCK_WITH_ACCELERATOR: i32 = 2;
const wxSTOCK_WITHOUT_ELLIPSIS: i32 = 4;
const wxSTOCK_FOR_BUTTON: i32 = wxSTOCK_WITHOUT_ELLIPSIS | wxSTOCK_WITH_MNEMONIC;

//  ENUM: @48
const wxTP_DEFAULT: i32 = 0;

const wxRICHTEXT_FIXED_WIDTH: i32 = 0x01;
const wxRICHTEXT_FIXED_HEIGHT: i32 = 0x02;
const wxRICHTEXT_VARIABLE_WIDTH: i32 = 0x04;
const wxRICHTEXT_VARIABLE_HEIGHT: i32 = 0x08;
const wxRICHTEXT_LAYOUT_SPECIFIED_RECT: i32 = 0x10;
const wxRICHTEXT_DRAW_IGNORE_CACHE: i32 = 0x01;
const wxRICHTEXT_DRAW_SELECTED: i32 = 0x02;
const wxRICHTEXT_DRAW_PRINT: i32 = 0x04;
const wxRICHTEXT_DRAW_GUIDELINES: i32 = 0x08;
const wxRICHTEXT_FORMATTED: i32 = 0x01;
const wxRICHTEXT_UNFORMATTED: i32 = 0x02;
const wxRICHTEXT_CACHE_SIZE: i32 = 0x04;
const wxRICHTEXT_HEIGHT_ONLY: i32 = 0x08;
const wxRICHTEXT_SETSTYLE_NONE: i32 = 0x00;
const wxRICHTEXT_SETSTYLE_WITH_UNDO: i32 = 0x01;
const wxRICHTEXT_SETSTYLE_OPTIMIZE: i32 = 0x02;
const wxRICHTEXT_SETSTYLE_PARAGRAPHS_ONLY: i32 = 0x04;
const wxRICHTEXT_SETSTYLE_CHARACTERS_ONLY: i32 = 0x08;
const wxRICHTEXT_SETSTYLE_RENUMBER: i32 = 0x10;
const wxRICHTEXT_SETSTYLE_SPECIFY_LEVEL: i32 = 0x20;
const wxRICHTEXT_SETSTYLE_RESET: i32 = 0x40;
const wxRICHTEXT_SETSTYLE_REMOVE: i32 = 0x80;
const wxRICHTEXT_SETPROPERTIES_NONE: i32 = 0x00;
const wxRICHTEXT_SETPROPERTIES_WITH_UNDO: i32 = 0x01;
const wxRICHTEXT_SETPROPERTIES_PARAGRAPHS_ONLY: i32 = 0x02;
const wxRICHTEXT_SETPROPERTIES_CHARACTERS_ONLY: i32 = 0x04;
const wxRICHTEXT_SETPROPERTIES_RESET: i32 = 0x08;
const wxRICHTEXT_SETPROPERTIES_REMOVE: i32 = 0x10;
const wxRICHTEXT_INSERT_NONE: i32 = 0x00;
const wxRICHTEXT_INSERT_WITH_PREVIOUS_PARAGRAPH_STYLE: i32 = 0x01;
const wxRICHTEXT_INSERT_INTERACTIVE: i32 = 0x02;
const wxTEXT_ATTR_KEEP_FIRST_PARA_STYLE: i32 = 0x10000000;
const wxSCRIPT_MUL_FACTOR: f32 = 1.5;
//  SKIP: wxRICHTEXT_ALL
//  SKIP: wxRICHTEXT_NONE
//  SKIP: wxRICHTEXT_NO_SELECTION
const wxRICHTEXT_HANDLER_INCLUDE_STYLESHEET: i32 = 0x0001;
const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_MEMORY: i32 = 0x0010;
const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_FILES: i32 = 0x0020;
const wxRICHTEXT_HANDLER_SAVE_IMAGES_TO_BASE64: i32 = 0x0040;
const wxRICHTEXT_HANDLER_NO_HEADER_FOOTER: i32 = 0x0080;
const wxRICHTEXT_HANDLER_CONVERT_FACENAMES: i32 = 0x0100;
const wxRICHTEXT_HANDLER_USE_CSS: i32 = 0x1000;
//  ENUM: wxRichTextFileType
const wxRICHTEXT_TYPE_ANY: i32 = 0;
const wxRICHTEXT_TYPE_TEXT: i32 = 0 + 1;
const wxRICHTEXT_TYPE_XML: i32 = 0 + 2;
const wxRICHTEXT_TYPE_HTML: i32 = 0 + 3;
const wxRICHTEXT_TYPE_RTF: i32 = 0 + 4;
const wxRICHTEXT_TYPE_PDF: i32 = 0 + 5;
//  ENUM: wxRichTextHitTestFlags
const wxRICHTEXT_HITTEST_NONE: i32 =    0x01;
const wxRICHTEXT_HITTEST_BEFORE: i32 =  0x02;
const wxRICHTEXT_HITTEST_AFTER: i32 =   0x04;
const wxRICHTEXT_HITTEST_ON: i32 =      0x08;
const wxRICHTEXT_HITTEST_OUTSIDE: i32 = 0x10;
const wxRICHTEXT_HITTEST_NO_NESTED_OBJECTS: i32 = 0x20;
const wxRICHTEXT_HITTEST_NO_FLOATING_OBJECTS: i32 = 0x40;
const wxRICHTEXT_HITTEST_HONOUR_ATOMIC: i32 = 0x80;
//  ENUM: wxTextBoxAttrFlags
const wxTEXT_BOX_ATTR_FLOAT: i32 = 0x00000001;
const wxTEXT_BOX_ATTR_CLEAR: i32 = 0x00000002;
const wxTEXT_BOX_ATTR_COLLAPSE_BORDERS: i32 = 0x00000004;
const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT: i32 = 0x00000008;
const wxTEXT_BOX_ATTR_BOX_STYLE_NAME: i32 = 0x00000010;
const wxTEXT_BOX_ATTR_WHITESPACE: i32 = 0x00000020;
const wxTEXT_BOX_ATTR_CORNER_RADIUS: i32 = 0x00000040;
//  ENUM: wxTextAttrValueFlags
const wxTEXT_ATTR_VALUE_VALID: i32 = 0x1000;
const wxTEXT_ATTR_VALUE_VALID_MASK: i32 = 0x1000;
//  ENUM: wxTextAttrUnits
const wxTEXT_ATTR_UNITS_TENTHS_MM: i32 = 0x0001;
const wxTEXT_ATTR_UNITS_PIXELS: i32 = 0x0002;
const wxTEXT_ATTR_UNITS_PERCENTAGE: i32 = 0x0004;
const wxTEXT_ATTR_UNITS_POINTS: i32 = 0x0008;
const wxTEXT_ATTR_UNITS_HUNDREDTHS_POINT: i32 = 0x0100;
const wxTEXT_ATTR_UNITS_MASK: i32 = 0x010F;
//  ENUM: wxTextBoxAttrPosition
const wxTEXT_BOX_ATTR_POSITION_STATIC: i32 = 0x0000;
const wxTEXT_BOX_ATTR_POSITION_RELATIVE: i32 = 0x0010;
const wxTEXT_BOX_ATTR_POSITION_ABSOLUTE: i32 = 0x0020;
const wxTEXT_BOX_ATTR_POSITION_FIXED: i32 = 0x0040;
const wxTEXT_BOX_ATTR_POSITION_MASK: i32 = 0x00F0;
//  ENUM: wxTextAttrBorderStyle
const wxTEXT_BOX_ATTR_BORDER_NONE: i32 = 0;
const wxTEXT_BOX_ATTR_BORDER_SOLID: i32 = 1;
const wxTEXT_BOX_ATTR_BORDER_DOTTED: i32 = 2;
const wxTEXT_BOX_ATTR_BORDER_DASHED: i32 = 3;
const wxTEXT_BOX_ATTR_BORDER_DOUBLE: i32 = 4;
const wxTEXT_BOX_ATTR_BORDER_GROOVE: i32 = 5;
const wxTEXT_BOX_ATTR_BORDER_RIDGE: i32 = 6;
const wxTEXT_BOX_ATTR_BORDER_INSET: i32 = 7;
const wxTEXT_BOX_ATTR_BORDER_OUTSET: i32 = 8;
//  ENUM: wxTextAttrBorderFlags
const wxTEXT_BOX_ATTR_BORDER_STYLE: i32 = 0x0001;
const wxTEXT_BOX_ATTR_BORDER_COLOUR: i32 = 0x0002;
//  ENUM: wxTextAttrBorderWidth
const wxTEXT_BOX_ATTR_BORDER_THIN: i32 = -1;
const wxTEXT_BOX_ATTR_BORDER_MEDIUM: i32 = -2;
const wxTEXT_BOX_ATTR_BORDER_THICK: i32 = -3;
//  ENUM: wxTextBoxAttrFloatStyle
const wxTEXT_BOX_ATTR_FLOAT_NONE: i32 = 0;
const wxTEXT_BOX_ATTR_FLOAT_LEFT: i32 = 1;
const wxTEXT_BOX_ATTR_FLOAT_RIGHT: i32 = 2;
//  ENUM: wxTextBoxAttrClearStyle
const wxTEXT_BOX_ATTR_CLEAR_NONE: i32 = 0;
const wxTEXT_BOX_ATTR_CLEAR_LEFT: i32 = 1;
const wxTEXT_BOX_ATTR_CLEAR_RIGHT: i32 = 2;
const wxTEXT_BOX_ATTR_CLEAR_BOTH: i32 = 3;
//  ENUM: wxTextBoxAttrCollapseMode
const wxTEXT_BOX_ATTR_COLLAPSE_NONE: i32 = 0;
const wxTEXT_BOX_ATTR_COLLAPSE_FULL: i32 = 1;
//  ENUM: wxTextBoxAttrVerticalAlignment
const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_NONE: i32 =       0;
const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_TOP: i32 =       1;
const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_CENTRE: i32 =     2;
const wxTEXT_BOX_ATTR_VERTICAL_ALIGNMENT_BOTTOM: i32 =    3;
//  ENUM: wxTextBoxAttrWhitespaceMode
const wxTEXT_BOX_ATTR_WHITESPACE_NONE: i32 = 0;
const wxTEXT_BOX_ATTR_WHITESPACE_NORMAL: i32 = 1;
const wxTEXT_BOX_ATTR_WHITESPACE_NO_WRAP: i32 = 2;
const wxTEXT_BOX_ATTR_WHITESPACE_PREFORMATTED: i32 = 3;
const wxTEXT_BOX_ATTR_WHITESPACE_PREFORMATTED_LINE: i32 = 4;
const wxTEXT_BOX_ATTR_WHITESPACE_PREFORMATTED_WRAP: i32 = 5;
//  ENUM: wxRichTextCommandId
const wxRICHTEXT_INSERT: i32 = 0;
const wxRICHTEXT_DELETE: i32 = 0 + 1;
const wxRICHTEXT_CHANGE_ATTRIBUTES: i32 = 0 + 2;
const wxRICHTEXT_CHANGE_STYLE: i32 = 0 + 3;
const wxRICHTEXT_CHANGE_OBJECT: i32 = 0 + 4;

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
const wxPATH_NATIVE: i32 = 0;
const wxPATH_UNIX: i32 = 0 + 1;
const wxPATH_BEOS: i32 = wxPATH_UNIX;
const wxPATH_MAC: i32 = wxPATH_UNIX + 1;
const wxPATH_DOS: i32 = wxPATH_UNIX + 2;
const wxPATH_WIN: i32 = wxPATH_DOS;
const wxPATH_OS2: i32 = wxPATH_DOS;
const wxPATH_VMS: i32 = wxPATH_DOS + 1;
const wxPATH_MAX: i32 = wxPATH_DOS + 2;
//  ENUM: wxSizeConvention
const wxSIZE_CONV_TRADITIONAL: i32 = 0;
const wxSIZE_CONV_IEC: i32 = 0 + 1;
const wxSIZE_CONV_SI: i32 = 0 + 2;
//  ENUM: wxPathNormalize
const wxPATH_NORM_ENV_VARS: i32 = 0x0001;
const wxPATH_NORM_DOTS: i32 = 0x0002;
const wxPATH_NORM_TILDE: i32 = 0x0004;
const wxPATH_NORM_CASE: i32 = 0x0008;
const wxPATH_NORM_ABSOLUTE: i32 = 0x0010;
const wxPATH_NORM_LONG: i32 =     0x0020;
const wxPATH_NORM_SHORTCUT: i32 = 0x0040;
const wxPATH_NORM_ALL: i32 = 0x00ff & ~wxPATH_NORM_CASE;
//  ENUM: @20
const wxPATH_RMDIR_FULL: i32 = 1;
const wxPATH_RMDIR_RECURSIVE: i32 = 2;
//  ENUM: @21
const wxFILE_EXISTS_REGULAR: i32 = 0x0001;
const wxFILE_EXISTS_DIR: i32 = 0x0002;
const wxFILE_EXISTS_SYMLINK: i32 = 0x1004;
const wxFILE_EXISTS_DEVICE: i32 = 0x0008;
const wxFILE_EXISTS_FIFO: i32 = 0x0016;
const wxFILE_EXISTS_SOCKET: i32 = 0x0032;
const wxFILE_EXISTS_NO_FOLLOW: i32 = 0x1000
    wxFILE_EXISTS_ANY       = 0x1FFF;

//  ENUM: wxTaskBarIconType
const wxTBI_DOCK: i32 = 0;
const wxTBI_CUSTOM_STATUSITEM: i32 = 0 + 1;
const wxTBI_DEFAULT_TYPE: i32 = 0 + 2;

//  ENUM: wxAcceleratorEntryFlags
const wxACCEL_NORMAL: i32 = 0;
const wxACCEL_ALT: i32 = 0 + 1;
const wxACCEL_CTRL: i32 = 0 + 2;
const wxACCEL_SHIFT: i32 = 0 + 3;
const wxACCEL_RAW_CTRL: i32 = 0 + 4;
const wxACCEL_CMD: i32 = 0 + 5;

//  ENUM: wxMessageQueueError
const wxMSGQUEUE_NO_ERROR: i32 = 0;
const wxMSGQUEUE_TIMEOUT: i32 = 0 + 1;
const wxMSGQUEUE_MISC_ERROR: i32 = 0 + 2;

//  ENUM: BufMode
const read: i32 = 0;
const write: i32 = 0 + 1;
const read_write: i32 = 0 + 2;

//  ENUM: Style
const Style_None: i32 = 0x00;
const Style_WithThousandsSep: i32 = 0x01;
const Style_NoTrailingZeroes: i32 = 0x02;

//  ENUM: wxFileSystemOpenFlags
const wxFS_READ: i32 = 1;
const wxFS_SEEKABLE: i32 = 4;

const wxLC_VRULES: i32 = 0x0001;
const wxLC_HRULES: i32 = 0x0002;
const wxLC_ICON: i32 = 0x0004;
const wxLC_SMALL_ICON: i32 = 0x0008;
const wxLC_LIST: i32 = 0x0010;
const wxLC_REPORT: i32 = 0x0020;
const wxLC_ALIGN_TOP: i32 = 0x0040;
const wxLC_ALIGN_LEFT: i32 = 0x0080;
const wxLC_AUTOARRANGE: i32 = 0x0100;
const wxLC_VIRTUAL: i32 = 0x0200;
const wxLC_EDIT_LABELS: i32 = 0x0400;
const wxLC_NO_HEADER: i32 = 0x0800;
const wxLC_NO_SORT_HEADER: i32 = 0x1000;
const wxLC_SINGLE_SEL: i32 = 0x2000;
const wxLC_SORT_ASCENDING: i32 = 0x4000;
const wxLC_SORT_DESCENDING: i32 = 0x8000;
const wxLC_MASK_TYPE: i32 = (wxLC_ICON | wxLC_SMALL_ICON | wxLC_LIST | wxLC_REPORT);
const wxLC_MASK_ALIGN: i32 = (wxLC_ALIGN_TOP | wxLC_ALIGN_LEFT);
const wxLC_MASK_SORT: i32 = (wxLC_SORT_ASCENDING | wxLC_SORT_DESCENDING);
const wxLIST_MASK_STATE: i32 = 0x0001;
const wxLIST_MASK_TEXT: i32 = 0x0002;
const wxLIST_MASK_IMAGE: i32 = 0x0004;
const wxLIST_MASK_DATA: i32 = 0x0008;
const wxLIST_SET_ITEM: i32 = 0x0010;
const wxLIST_MASK_WIDTH: i32 = 0x0020;
const wxLIST_MASK_FORMAT: i32 = 0x0040;
const wxLIST_STATE_DONTCARE: i32 = 0x0000;
const wxLIST_STATE_DROPHILITED: i32 = 0x0001;
const wxLIST_STATE_FOCUSED: i32 = 0x0002;
const wxLIST_STATE_SELECTED: i32 = 0x0004;
const wxLIST_STATE_CUT: i32 = 0x0008;
const wxLIST_HITTEST_ABOVE: i32 = 0x0001;
const wxLIST_HITTEST_BELOW: i32 = 0x0002;
const wxLIST_HITTEST_NOWHERE: i32 = 0x0004;
const wxLIST_HITTEST_ONITEMICON: i32 = 0x0020;
const wxLIST_HITTEST_ONITEMLABEL: i32 = 0x0080;
const wxLIST_HITTEST_ONITEMSTATEICON: i32 = 0x0200;
const wxLIST_HITTEST_TOLEFT: i32 = 0x0400;
const wxLIST_HITTEST_TORIGHT: i32 = 0x0800;
const wxLIST_HITTEST_ONITEM: i32 = (wxLIST_HITTEST_ONITEMICON | wxLIST_HITTEST_ONITEMLABEL | wxLIST_HITTEST_ONITEMSTATEICON);
const wxLIST_GETSUBITEMRECT_WHOLEITEM: i32 = -1l;
//  ENUM: @32
const wxLIST_NEXT_ABOVE: i32 = 0;
const wxLIST_NEXT_ALL: i32 = 0 + 1;
const wxLIST_NEXT_BELOW: i32 = 0 + 2;
const wxLIST_NEXT_LEFT: i32 = 0 + 3;
const wxLIST_NEXT_RIGHT: i32 = 0 + 4;
//  ENUM: @33
const wxLIST_ALIGN_DEFAULT: i32 = 0;
const wxLIST_ALIGN_LEFT: i32 = 0 + 1;
const wxLIST_ALIGN_TOP: i32 = 0 + 2;
const wxLIST_ALIGN_SNAP_TO_GRID: i32 = 0 + 3;
//  ENUM: wxListColumnFormat
const wxLIST_FORMAT_LEFT: i32 = 0;
const wxLIST_FORMAT_RIGHT: i32 = 0 + 1;
const wxLIST_FORMAT_CENTRE: i32 = 0 + 2;
const wxLIST_FORMAT_CENTER: i32 = wxLIST_FORMAT_CENTRE;
//  ENUM: @34
const wxLIST_AUTOSIZE: i32 = -1;
const wxLIST_AUTOSIZE_USEHEADER: i32 = -2;
//  ENUM: @35
const wxLIST_RECT_BOUNDS: i32 = 0;
const wxLIST_RECT_ICON: i32 = 0 + 1;
const wxLIST_RECT_LABEL: i32 = 0 + 2;
//  ENUM: @36
const wxLIST_FIND_UP: i32 = 0;
const wxLIST_FIND_DOWN: i32 = 0 + 1;
const wxLIST_FIND_LEFT: i32 = 0 + 2;
const wxLIST_FIND_RIGHT: i32 = 0 + 3;

//  ENUM: @29
const FALLBACK_NONE: i32 = 0;
const FALLBACK_SYSTEM: i32 = 1;
const FALLBACK_NEAREST_LARGER: i32 = 2;

// NODEF: wxCHANGE_UMASK
//  ENUM: wxPosixPermissions
const wxS_IRUSR: i32 = 00400;
const wxS_IWUSR: i32 = 00200;
const wxS_IXUSR: i32 = 00100;
const wxS_IRGRP: i32 = 00040;
const wxS_IWGRP: i32 = 00020;
const wxS_IXGRP: i32 = 00010;
const wxS_IROTH: i32 = 00004;
const wxS_IWOTH: i32 = 00002;
const wxS_IXOTH: i32 = 00001;
const wxPOSIX_USER_READ: i32 = wxS_IRUSR;
const wxPOSIX_USER_WRITE: i32 = wxS_IWUSR;
const wxPOSIX_USER_EXECUTE: i32 = wxS_IXUSR;
const wxPOSIX_GROUP_READ: i32 = wxS_IRGRP;
const wxPOSIX_GROUP_WRITE: i32 = wxS_IWGRP;
const wxPOSIX_GROUP_EXECUTE: i32 = wxS_IXGRP;
const wxPOSIX_OTHERS_READ: i32 = wxS_IROTH;
const wxPOSIX_OTHERS_WRITE: i32 = wxS_IWOTH;
const wxPOSIX_OTHERS_EXECUTE: i32 = wxS_IXOTH;
const wxS_DEFAULT: i32 = (wxPOSIX_USER_READ | wxPOSIX_USER_WRITE | 
                   wxPOSIX_GROUP_READ | wxPOSIX_GROUP_WRITE | 
                   wxPOSIX_OTHERS_READ | wxPOSIX_OTHERS_WRITE);
const wxS_DIR_DEFAULT: i32 = (wxPOSIX_USER_READ | wxPOSIX_USER_WRITE | wxPOSIX_USER_EXECUTE | 
                       wxPOSIX_GROUP_READ | wxPOSIX_GROUP_WRITE | wxPOSIX_GROUP_EXECUTE | 
                       wxPOSIX_OTHERS_READ | wxPOSIX_OTHERS_WRITE | wxPOSIX_OTHERS_EXECUTE);
//  ENUM: wxSeekMode
const wxFromStart: i32 = 0;
const wxFromCurrent: i32 = 0 + 1;
const wxFromEnd: i32 = 0 + 2;
//  ENUM: wxFileKind
const wxFILE_KIND_UNKNOWN: i32 = 0;
const wxFILE_KIND_DISK: i32 = 0 + 1;
const wxFILE_KIND_TERMINAL: i32 = 0 + 2;
const wxFILE_KIND_PIPE: i32 = 0 + 3;

//  ENUM: Kind
const Kind_General: i32 = 0;
const Kind_Advanced: i32 = 0 + 1;

//  ENUM: wxOleConvertVariantFlags
const wxOleConvertVariant_Default: i32 = 0;
const wxOleConvertVariant_ReturnSafeArrays: i32 = 1;

const wxRICHTEXT_ORGANISER_DELETE_STYLES: i32 = 0x0001;
const wxRICHTEXT_ORGANISER_CREATE_STYLES: i32 = 0x0002;
const wxRICHTEXT_ORGANISER_APPLY_STYLES: i32 = 0x0004;
const wxRICHTEXT_ORGANISER_EDIT_STYLES: i32 = 0x0008;
const wxRICHTEXT_ORGANISER_RENAME_STYLES: i32 = 0x0010;
const wxRICHTEXT_ORGANISER_OK_CANCEL: i32 = 0x0020;
const wxRICHTEXT_ORGANISER_RENUMBER: i32 = 0x0040;
const wxRICHTEXT_ORGANISER_SHOW_CHARACTER: i32 = 0x0100;
const wxRICHTEXT_ORGANISER_SHOW_PARAGRAPH: i32 = 0x0200;
const wxRICHTEXT_ORGANISER_SHOW_LIST: i32 = 0x0400;
const wxRICHTEXT_ORGANISER_SHOW_BOX: i32 = 0x0800;
const wxRICHTEXT_ORGANISER_SHOW_ALL: i32 = 0x1000;
const wxRICHTEXT_ORGANISER_ORGANISE: i32 = (wxRICHTEXT_ORGANISER_SHOW_ALL|wxRICHTEXT_ORGANISER_DELETE_STYLES|wxRICHTEXT_ORGANISER_CREATE_STYLES|wxRICHTEXT_ORGANISER_APPLY_STYLES|wxRICHTEXT_ORGANISER_EDIT_STYLES|wxRICHTEXT_ORGANISER_RENAME_STYLES);
const wxRICHTEXT_ORGANISER_BROWSE: i32 = (wxRICHTEXT_ORGANISER_SHOW_ALL|wxRICHTEXT_ORGANISER_OK_CANCEL);
const wxRICHTEXT_ORGANISER_BROWSE_NUMBERING: i32 = (wxRICHTEXT_ORGANISER_SHOW_LIST|wxRICHTEXT_ORGANISER_OK_CANCEL|wxRICHTEXT_ORGANISER_RENUMBER);

const wxTE_NO_VSCROLL: i32 = 0x0002;
const wxTE_READONLY: i32 = 0x0010;
const wxTE_MULTILINE: i32 = 0x0020;
const wxTE_PROCESS_TAB: i32 = 0x0040;
const wxTE_LEFT: i32 = 0x0000;
const wxTE_CENTER: i32 = wxALIGN_CENTER_HORIZONTAL;
const wxTE_RIGHT: i32 = wxALIGN_RIGHT;
const wxTE_CENTRE: i32 = wxTE_CENTER;
const wxTE_RICH: i32 = 0x0080;
const wxTE_PROCESS_ENTER: i32 = 0x0400;
const wxTE_PASSWORD: i32 = 0x0800;
const wxTE_AUTO_URL: i32 = 0x1000;
const wxTE_NOHIDESEL: i32 = 0x2000;
const wxTE_DONTWRAP: i32 = wxHSCROLL;
const wxTE_CHARWRAP: i32 = 0x4000;
const wxTE_WORDWRAP: i32 = 0x0001;
const wxTE_BESTWRAP: i32 = 0x0000;
const wxTE_RICH2: i32 = 0x8000;
const wxTEXT_TYPE_ANY: i32 = 0;
//  ENUM: wxTextAttrAlignment
const wxTEXT_ALIGNMENT_DEFAULT: i32 = 0;
const wxTEXT_ALIGNMENT_LEFT: i32 = 0 + 1;
const wxTEXT_ALIGNMENT_CENTRE: i32 = 0 + 2;
const wxTEXT_ALIGNMENT_CENTER: i32 = wxTEXT_ALIGNMENT_CENTRE;
const wxTEXT_ALIGNMENT_RIGHT: i32 = wxTEXT_ALIGNMENT_CENTRE + 1;
const wxTEXT_ALIGNMENT_JUSTIFIED: i32 = wxTEXT_ALIGNMENT_CENTRE + 2;
//  ENUM: wxTextAttrFlags
const wxTEXT_ATTR_TEXT_COLOUR: i32 = 0x00000001;
const wxTEXT_ATTR_BACKGROUND_COLOUR: i32 = 0x00000002;
const wxTEXT_ATTR_FONT_FACE: i32 = 0x00000004;
const wxTEXT_ATTR_FONT_POINT_SIZE: i32 = 0x00000008;
const wxTEXT_ATTR_FONT_PIXEL_SIZE: i32 = 0x10000000;
const wxTEXT_ATTR_FONT_WEIGHT: i32 = 0x00000010;
const wxTEXT_ATTR_FONT_ITALIC: i32 = 0x00000020;
const wxTEXT_ATTR_FONT_UNDERLINE: i32 = 0x00000040;
const wxTEXT_ATTR_FONT_STRIKETHROUGH: i32 = 0x08000000;
const wxTEXT_ATTR_FONT_ENCODING: i32 = 0x02000000;
const wxTEXT_ATTR_FONT_FAMILY: i32 = 0x04000000;
const wxTEXT_ATTR_FONT_SIZE: i32 = 
        ( wxTEXT_ATTR_FONT_POINT_SIZE | wxTEXT_ATTR_FONT_PIXEL_SIZE );
const wxTEXT_ATTR_FONT: i32 = 
        ( wxTEXT_ATTR_FONT_FACE | wxTEXT_ATTR_FONT_SIZE | wxTEXT_ATTR_FONT_WEIGHT | 
            wxTEXT_ATTR_FONT_ITALIC | wxTEXT_ATTR_FONT_UNDERLINE | wxTEXT_ATTR_FONT_STRIKETHROUGH | wxTEXT_ATTR_FONT_ENCODING | wxTEXT_ATTR_FONT_FAMILY );
const wxTEXT_ATTR_ALIGNMENT: i32 = 0x00000080;
const wxTEXT_ATTR_LEFT_INDENT: i32 = 0x00000100;
const wxTEXT_ATTR_RIGHT_INDENT: i32 = 0x00000200;
const wxTEXT_ATTR_TABS: i32 = 0x00000400;
const wxTEXT_ATTR_PARA_SPACING_AFTER: i32 = 0x00000800;
const wxTEXT_ATTR_PARA_SPACING_BEFORE: i32 = 0x00001000;
const wxTEXT_ATTR_LINE_SPACING: i32 = 0x00002000;
const wxTEXT_ATTR_CHARACTER_STYLE_NAME: i32 = 0x00004000;
const wxTEXT_ATTR_PARAGRAPH_STYLE_NAME: i32 = 0x00008000;
const wxTEXT_ATTR_LIST_STYLE_NAME: i32 = 0x00010000;
const wxTEXT_ATTR_BULLET_STYLE: i32 = 0x00020000;
const wxTEXT_ATTR_BULLET_NUMBER: i32 = 0x00040000;
const wxTEXT_ATTR_BULLET_TEXT: i32 = 0x00080000;
const wxTEXT_ATTR_BULLET_NAME: i32 = 0x00100000;
const wxTEXT_ATTR_BULLET: i32 = 
        ( wxTEXT_ATTR_BULLET_STYLE | wxTEXT_ATTR_BULLET_NUMBER | wxTEXT_ATTR_BULLET_TEXT | 
          wxTEXT_ATTR_BULLET_NAME );
const wxTEXT_ATTR_URL: i32 = 0x00200000;
const wxTEXT_ATTR_PAGE_BREAK: i32 = 0x00400000;
const wxTEXT_ATTR_EFFECTS: i32 = 0x00800000;
const wxTEXT_ATTR_OUTLINE_LEVEL: i32 = 0x01000000;
const wxTEXT_ATTR_AVOID_PAGE_BREAK_BEFORE: i32 = 0x20000000;
const wxTEXT_ATTR_AVOID_PAGE_BREAK_AFTER: i32 =  0x40000000;
const wxTEXT_ATTR_CHARACTER: i32 = 
        (wxTEXT_ATTR_FONT|wxTEXT_ATTR_EFFECTS| 
            wxTEXT_ATTR_BACKGROUND_COLOUR|wxTEXT_ATTR_TEXT_COLOUR|wxTEXT_ATTR_CHARACTER_STYLE_NAME|wxTEXT_ATTR_URL);
const wxTEXT_ATTR_PARAGRAPH: i32 = 
        (wxTEXT_ATTR_ALIGNMENT|wxTEXT_ATTR_LEFT_INDENT|wxTEXT_ATTR_RIGHT_INDENT|wxTEXT_ATTR_TABS|
            wxTEXT_ATTR_PARA_SPACING_BEFORE|wxTEXT_ATTR_PARA_SPACING_AFTER|wxTEXT_ATTR_LINE_SPACING|
            wxTEXT_ATTR_BULLET|wxTEXT_ATTR_PARAGRAPH_STYLE_NAME|wxTEXT_ATTR_LIST_STYLE_NAME|wxTEXT_ATTR_OUTLINE_LEVEL|
            wxTEXT_ATTR_PAGE_BREAK|wxTEXT_ATTR_AVOID_PAGE_BREAK_BEFORE|wxTEXT_ATTR_AVOID_PAGE_BREAK_AFTER);
const wxTEXT_ATTR_ALL: i32 = (wxTEXT_ATTR_CHARACTER|wxTEXT_ATTR_PARAGRAPH);
//  ENUM: wxTextAttrBulletStyle
const wxTEXT_ATTR_BULLET_STYLE_NONE: i32 = 0x00000000;
const wxTEXT_ATTR_BULLET_STYLE_ARABIC: i32 = 0x00000001;
const wxTEXT_ATTR_BULLET_STYLE_LETTERS_UPPER: i32 = 0x00000002;
const wxTEXT_ATTR_BULLET_STYLE_LETTERS_LOWER: i32 = 0x00000004;
const wxTEXT_ATTR_BULLET_STYLE_ROMAN_UPPER: i32 = 0x00000008;
const wxTEXT_ATTR_BULLET_STYLE_ROMAN_LOWER: i32 = 0x00000010;
const wxTEXT_ATTR_BULLET_STYLE_SYMBOL: i32 = 0x00000020;
const wxTEXT_ATTR_BULLET_STYLE_BITMAP: i32 = 0x00000040;
const wxTEXT_ATTR_BULLET_STYLE_PARENTHESES: i32 = 0x00000080;
const wxTEXT_ATTR_BULLET_STYLE_PERIOD: i32 = 0x00000100;
const wxTEXT_ATTR_BULLET_STYLE_STANDARD: i32 = 0x00000200;
const wxTEXT_ATTR_BULLET_STYLE_RIGHT_PARENTHESIS: i32 = 0x00000400;
const wxTEXT_ATTR_BULLET_STYLE_OUTLINE: i32 = 0x00000800;
const wxTEXT_ATTR_BULLET_STYLE_ALIGN_LEFT: i32 = 0x00000000;
const wxTEXT_ATTR_BULLET_STYLE_ALIGN_RIGHT: i32 = 0x00001000;
const wxTEXT_ATTR_BULLET_STYLE_ALIGN_CENTRE: i32 = 0x00002000;
const wxTEXT_ATTR_BULLET_STYLE_CONTINUATION: i32 = 0x00004000;
//  ENUM: wxTextAttrEffects
const wxTEXT_ATTR_EFFECT_NONE: i32 = 0x00000000;
const wxTEXT_ATTR_EFFECT_CAPITALS: i32 = 0x00000001;
const wxTEXT_ATTR_EFFECT_SMALL_CAPITALS: i32 = 0x00000002;
const wxTEXT_ATTR_EFFECT_STRIKETHROUGH: i32 = 0x00000004;
const wxTEXT_ATTR_EFFECT_DOUBLE_STRIKETHROUGH: i32 = 0x00000008;
const wxTEXT_ATTR_EFFECT_SHADOW: i32 = 0x00000010;
const wxTEXT_ATTR_EFFECT_EMBOSS: i32 = 0x00000020;
const wxTEXT_ATTR_EFFECT_OUTLINE: i32 = 0x00000040;
const wxTEXT_ATTR_EFFECT_ENGRAVE: i32 = 0x00000080;
const wxTEXT_ATTR_EFFECT_SUPERSCRIPT: i32 = 0x00000100;
const wxTEXT_ATTR_EFFECT_SUBSCRIPT: i32 = 0x00000200;
const wxTEXT_ATTR_EFFECT_RTL: i32 = 0x00000400;
const wxTEXT_ATTR_EFFECT_SUPPRESS_HYPHENATION: i32 = 0x00001000;
//  ENUM: wxTextAttrLineSpacing
const wxTEXT_ATTR_LINE_SPACING_NORMAL: i32 = 10;
const wxTEXT_ATTR_LINE_SPACING_HALF: i32 = 15;
const wxTEXT_ATTR_LINE_SPACING_TWICE: i32 = 20;
//  ENUM: wxTextAttrUnderlineType
const wxTEXT_ATTR_UNDERLINE_NONE: i32 = 0;
const wxTEXT_ATTR_UNDERLINE_SOLID: i32 = 0 + 1;
const wxTEXT_ATTR_UNDERLINE_DOUBLE: i32 = 0 + 2;
const wxTEXT_ATTR_UNDERLINE_SPECIAL: i32 = 0 + 3;
//  ENUM: wxTextCtrlHitTestResult
const wxTE_HT_UNKNOWN: i32 = -2;
const wxTE_HT_BEFORE: i32 = -2 + 1;
const wxTE_HT_ON_TEXT: i32 = -2 + 2;
const wxTE_HT_BELOW: i32 = -2 + 3;
const wxTE_HT_BEYOND: i32 = -2 + 4;

//  ENUM: ScaleMode
const Scale_None: i32 = 0;
const Scale_Fill: i32 = 0 + 1;
const Scale_AspectFit: i32 = 0 + 2;
const Scale_AspectFill: i32 = 0 + 3;

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
const wxPOWER_SOCKET: i32 = 0;
const wxPOWER_BATTERY: i32 = 0 + 1;
const wxPOWER_UNKNOWN: i32 = 0 + 2;
//  ENUM: wxBatteryState
const wxBATTERY_NORMAL_STATE: i32 = 0;
const wxBATTERY_LOW_STATE: i32 = 0 + 1;
const wxBATTERY_CRITICAL_STATE: i32 = 0 + 2;
const wxBATTERY_SHUTDOWN_STATE: i32 = 0 + 3;
const wxBATTERY_UNKNOWN_STATE: i32 = 0 + 4;
//  ENUM: wxPowerResourceKind
const wxPOWER_RESOURCE_SCREEN: i32 = 0;
const wxPOWER_RESOURCE_SYSTEM: i32 = 0 + 1;

//  ENUM: wxRibbonButtonBarButtonState
const wxRIBBON_BUTTONBAR_BUTTON_SMALL: i32 = 0 << 0;
const wxRIBBON_BUTTONBAR_BUTTON_MEDIUM: i32 = 1 << 0;
const wxRIBBON_BUTTONBAR_BUTTON_LARGE: i32 = 2 << 0;
const wxRIBBON_BUTTONBAR_BUTTON_SIZE_MASK: i32 = 3 << 0;
const wxRIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED: i32 = 1 << 3;
const wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED: i32 = 1 << 4;
const wxRIBBON_BUTTONBAR_BUTTON_HOVER_MASK: i32 = wxRIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED | wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED;
const wxRIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE: i32 = 1 << 5;
const wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE: i32 = 1 << 6;
const wxRIBBON_BUTTONBAR_BUTTON_ACTIVE_MASK: i32 = wxRIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE | wxRIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE;
const wxRIBBON_BUTTONBAR_BUTTON_DISABLED: i32 = 1 << 7;
const wxRIBBON_BUTTONBAR_BUTTON_TOGGLED: i32 = 1 << 8;
const wxRIBBON_BUTTONBAR_BUTTON_STATE_MASK: i32 = 0x1F8;

//  ENUM: wxRasterOperationMode
const wxCLEAR: i32 = 0;
const wxXOR: i32 = 0 + 1;
const wxINVERT: i32 = 0 + 2;
const wxOR_REVERSE: i32 = 0 + 3;
const wxAND_REVERSE: i32 = 0 + 4;
const wxCOPY: i32 = 0 + 5;
const wxAND: i32 = 0 + 6;
const wxAND_INVERT: i32 = 0 + 7;
const wxNO_OP: i32 = 0 + 8;
const wxNOR: i32 = 0 + 9;
const wxEQUIV: i32 = 0 + 10;
const wxSRC_INVERT: i32 = 0 + 11;
const wxOR_INVERT: i32 = 0 + 12;
const wxNAND: i32 = 0 + 13;
const wxOR: i32 = 0 + 14;
const wxSET: i32 = 0 + 15;
//  ENUM: wxFloodFillStyle
const wxFLOOD_SURFACE: i32 = 1;
const wxFLOOD_BORDER: i32 = 1 + 1;
//  ENUM: wxMappingMode
const wxMM_TEXT: i32 = 1;
const wxMM_METRIC: i32 = 1 + 1;
const wxMM_LOMETRIC: i32 = 1 + 2;
const wxMM_TWIPS: i32 = 1 + 3;
const wxMM_POINTS: i32 = 1 + 4;

//  ENUM: wxTaskBarButtonState
const wxTASKBAR_BUTTON_NO_PROGRESS: i32 = 0;
const wxTASKBAR_BUTTON_INDETERMINATE: i32 = 1;
const wxTASKBAR_BUTTON_NORMAL: i32 = 2;
const wxTASKBAR_BUTTON_ERROR: i32 = 4;
const wxTASKBAR_BUTTON_PAUSED: i32 = 8;
//  ENUM: wxTaskBarJumpListItemType
const wxTASKBAR_JUMP_LIST_SEPARATOR: i32 = 0;
const wxTASKBAR_JUMP_LIST_TASK: i32 = 0 + 1;
const wxTASKBAR_JUMP_LIST_DESTINATION: i32 = 0 + 2;

// NODEF: wxCONCAT
// NODEF: wxCONCAT3
// NODEF: wxCONCAT4
// NODEF: wxCONCAT5
// NODEF: wxSTRINGIZE
// NODEF: wxSTRINGIZE_T
// NODEF: __WXFUNCTION__

const wxICON_SCREEN_DEPTH: i32 = (-1);

//  ENUM: wxPenStyle
const wxPENSTYLE_INVALID: i32 = -1;
const wxPENSTYLE_SOLID: i32 = -1 + 1;
const wxPENSTYLE_DOT: i32 = -1 + 2;
const wxPENSTYLE_LONG_DASH: i32 = -1 + 3;
const wxPENSTYLE_SHORT_DASH: i32 = -1 + 4;
const wxPENSTYLE_DOT_DASH: i32 = -1 + 5;
const wxPENSTYLE_USER_DASH: i32 = -1 + 6;
const wxPENSTYLE_TRANSPARENT: i32 = -1 + 7;
const wxPENSTYLE_STIPPLE_MASK_OPAQUE: i32 = -1 + 8;
const wxPENSTYLE_STIPPLE_MASK: i32 = -1 + 9;
const wxPENSTYLE_STIPPLE: i32 = -1 + 10;
const wxPENSTYLE_BDIAGONAL_HATCH: i32 = -1 + 11;
const wxPENSTYLE_CROSSDIAG_HATCH: i32 = -1 + 12;
const wxPENSTYLE_FDIAGONAL_HATCH: i32 = -1 + 13;
const wxPENSTYLE_CROSS_HATCH: i32 = -1 + 14;
const wxPENSTYLE_HORIZONTAL_HATCH: i32 = -1 + 15;
const wxPENSTYLE_VERTICAL_HATCH: i32 = -1 + 16;
const wxPENSTYLE_FIRST_HATCH: i32 = -1 + 17;
const wxPENSTYLE_LAST_HATCH: i32 = -1 + 18;
//  ENUM: wxPenQuality
const wxPEN_QUALITY_DEFAULT: i32 = 0;
const wxPEN_QUALITY_LOW: i32 = 0 + 1;
const wxPEN_QUALITY_HIGH: i32 = 0 + 2;
//  ENUM: wxPenJoin
const wxJOIN_INVALID: i32 = -1;
const wxJOIN_BEVEL: i32 = 120;
const wxJOIN_MITER: i32 = 120 + 1;
const wxJOIN_ROUND: i32 = 120 + 2;
//  ENUM: wxPenCap
const wxCAP_INVALID: i32 = -1;
const wxCAP_ROUND: i32 = 130;
const wxCAP_PROJECTING: i32 = 130 + 1;
const wxCAP_BUTT: i32 = 130 + 2;

//  ENUM: wxRichTextStyleType
const wxRICHTEXT_STYLE_ALL: i32 = 0;
const wxRICHTEXT_STYLE_PARAGRAPH: i32 = 0 + 1;
const wxRICHTEXT_STYLE_CHARACTER: i32 = 0 + 2;
const wxRICHTEXT_STYLE_LIST: i32 = 0 + 3;
const wxRICHTEXT_STYLE_BOX: i32 = 0 + 4;

const wxSOUND_SYNC: i32 = 0;
const wxSOUND_ASYNC: i32 = 1;
const wxSOUND_LOOP: i32 = 2;

const wxID_HTML_HELPFRAME: i32 = (wxID_HIGHEST + 1);
const wxHF_EMBEDDED: i32 = 0x00008000;
const wxHF_DIALOG: i32 = 0x00010000;
const wxHF_FRAME: i32 = 0x00020000;
const wxHF_MODAL: i32 = 0x00040000;


const wxFNTP_FONTDESC_AS_LABEL: i32 = 0x0008;
const wxFNTP_USEFONT_FOR_LABEL: i32 = 0x0010;
const wxFONTBTN_DEFAULT_STYLE: i32 = (wxFNTP_FONTDESC_AS_LABEL | wxFNTP_USEFONT_FOR_LABEL);
const wxFNTP_USE_TEXTCTRL: i32 = (wxPB_USE_TEXTCTRL);
const wxFNTP_DEFAULT_STYLE: i32 = (wxFNTP_FONTDESC_AS_LABEL|wxFNTP_USEFONT_FOR_LABEL);

//  ENUM: wxNumValidatorStyle
const wxNUM_VAL_DEFAULT: i32 = 0;
const wxNUM_VAL_THOUSANDS_SEPARATOR: i32 = 1;
const wxNUM_VAL_ZERO_AS_BLANK: i32 = 2;
const wxNUM_VAL_NO_TRAILING_ZEROES: i32 = 2 + 1;

//  ENUM: wxAntialiasMode
const wxANTIALIAS_NONE: i32 = 0;
const wxANTIALIAS_DEFAULT: i32 = 0 + 1;
//  ENUM: wxInterpolationQuality
const wxINTERPOLATION_DEFAULT: i32 = 0;
const wxINTERPOLATION_NONE: i32 = 0 + 1;
const wxINTERPOLATION_FAST: i32 = 0 + 2;
const wxINTERPOLATION_GOOD: i32 = 0 + 3;
const wxINTERPOLATION_BEST: i32 = 0 + 4;
//  ENUM: wxCompositionMode
const wxCOMPOSITION_INVALID: i32 = -1;
const wxCOMPOSITION_CLEAR: i32 = -1 + 1;
const wxCOMPOSITION_SOURCE: i32 = -1 + 2;
const wxCOMPOSITION_OVER: i32 = -1 + 3;
const wxCOMPOSITION_IN: i32 = -1 + 4;
const wxCOMPOSITION_OUT: i32 = -1 + 5;
const wxCOMPOSITION_ATOP: i32 = -1 + 6;
const wxCOMPOSITION_DEST: i32 = -1 + 7;
const wxCOMPOSITION_DEST_OVER: i32 = -1 + 8;
const wxCOMPOSITION_DEST_IN: i32 = -1 + 9;
const wxCOMPOSITION_DEST_OUT: i32 = -1 + 10;
const wxCOMPOSITION_DEST_ATOP: i32 = -1 + 11;
const wxCOMPOSITION_XOR: i32 = -1 + 12;
const wxCOMPOSITION_ADD: i32 = -1 + 13;
//  ENUM: wxGradientType
const wxGRADIENT_NONE: i32 = 0;
const wxGRADIENT_LINEAR: i32 = 0 + 1;
const wxGRADIENT_RADIAL: i32 = 0 + 2;

//  ENUM: @40
const Selected: i32 = 0x00010000;
const ChoicePopup: i32 = 0x00020000;
const Control: i32 = 0x00040000;
const Disabled: i32 = 0x00080000;
const DontUseCellFgCol: i32 = 0x00100000;
const DontUseCellBgCol: i32 = 0x00200000;
const DontUseCellColours: i32 = DontUseCellFgCol |
                              DontUseCellBgCol;

//  ENUM: wxMessageQueueError
const wxMSGQUEUE_NO_ERROR: i32 = 0;
const wxMSGQUEUE_TIMEOUT: i32 = 0 + 1;
const wxMSGQUEUE_MISC_ERROR: i32 = 0 + 2;

// NODEF: wxFORCE_LINK_THIS_MODULE
// NODEF: wxFORCE_LINK_MODULE

//  ENUM: @44
const wxRICHTEXT_FIELD_STYLE_COMPOSITE: i32 = 0x01;
const wxRICHTEXT_FIELD_STYLE_RECTANGLE: i32 = 0x02;
const wxRICHTEXT_FIELD_STYLE_NO_BORDER: i32 = 0x04;
const wxRICHTEXT_FIELD_STYLE_START_TAG: i32 = 0x08;
const wxRICHTEXT_FIELD_STYLE_END_TAG: i32 = 0x10;

//  ENUM: wxMediaState
const wxMEDIASTATE_STOPPED: i32 = 0;
const wxMEDIASTATE_PAUSED: i32 = 0 + 1;
const wxMEDIASTATE_PLAYING: i32 = 0 + 2;
//  ENUM: wxMediaCtrlPlayerControls
const wxMEDIACTRLPLAYERCONTROLS_NONE: i32 =   0;
const wxMEDIACTRLPLAYERCONTROLS_STEP: i32 =   1 << 0;
const wxMEDIACTRLPLAYERCONTROLS_VOLUME: i32 =   1 << 1;
const wxMEDIACTRLPLAYERCONTROLS_DEFAULT: i32 =
                    wxMEDIACTRLPLAYERCONTROLS_STEP |
                    wxMEDIACTRLPLAYERCONTROLS_VOLUME;

// NODEF: wxBITMAP
// NODEF: wxBITMAP_PNG
// NODEF: wxBITMAP_PNG_FROM_DATA
// NODEF: wxICON
//  ENUM: wxBitmapType
const wxBITMAP_TYPE_INVALID: i32 = 0;
const wxBITMAP_TYPE_BMP: i32 = 0 + 1;
const wxBITMAP_TYPE_BMP_RESOURCE: i32 = 0 + 2;
const wxBITMAP_TYPE_RESOURCE: i32 = wxBITMAP_TYPE_BMP_RESOURCE;
const wxBITMAP_TYPE_ICO: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 1;
const wxBITMAP_TYPE_ICO_RESOURCE: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 2;
const wxBITMAP_TYPE_CUR: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 3;
const wxBITMAP_TYPE_CUR_RESOURCE: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 4;
const wxBITMAP_TYPE_XBM: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 5;
const wxBITMAP_TYPE_XBM_DATA: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 6;
const wxBITMAP_TYPE_XPM: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 7;
const wxBITMAP_TYPE_XPM_DATA: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 8;
const wxBITMAP_TYPE_TIFF: i32 = wxBITMAP_TYPE_BMP_RESOURCE + 9;
const wxBITMAP_TYPE_TIF: i32 = wxBITMAP_TYPE_TIFF;
const wxBITMAP_TYPE_TIFF_RESOURCE: i32 = wxBITMAP_TYPE_TIFF + 1;
const wxBITMAP_TYPE_TIF_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE;
const wxBITMAP_TYPE_GIF: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 1;
const wxBITMAP_TYPE_GIF_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 2;
const wxBITMAP_TYPE_PNG: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 3;
const wxBITMAP_TYPE_PNG_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 4;
const wxBITMAP_TYPE_JPEG: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 5;
const wxBITMAP_TYPE_JPEG_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 6;
const wxBITMAP_TYPE_PNM: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 7;
const wxBITMAP_TYPE_PNM_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 8;
const wxBITMAP_TYPE_PCX: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 9;
const wxBITMAP_TYPE_PCX_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 10;
const wxBITMAP_TYPE_PICT: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 11;
const wxBITMAP_TYPE_PICT_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 12;
const wxBITMAP_TYPE_ICON: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 13;
const wxBITMAP_TYPE_ICON_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 14;
const wxBITMAP_TYPE_ANI: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 15;
const wxBITMAP_TYPE_IFF: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 16;
const wxBITMAP_TYPE_TGA: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 17;
const wxBITMAP_TYPE_MACCURSOR: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 18;
const wxBITMAP_TYPE_MACCURSOR_RESOURCE: i32 = wxBITMAP_TYPE_TIFF_RESOURCE + 19;
const wxBITMAP_TYPE_ANY: i32 = 50;
//  ENUM: wxPolygonFillMode
const wxODDEVEN_RULE: i32 = 1;
const wxWINDING_RULE: i32 = 1 + 1;
//  ENUM: wxStockCursor
const wxCURSOR_NONE: i32 = 0;
const wxCURSOR_ARROW: i32 = 0 + 1;
const wxCURSOR_RIGHT_ARROW: i32 = 0 + 2;
const wxCURSOR_BULLSEYE: i32 = 0 + 3;
const wxCURSOR_CHAR: i32 = 0 + 4;
const wxCURSOR_CROSS: i32 = 0 + 5;
const wxCURSOR_HAND: i32 = 0 + 6;
const wxCURSOR_IBEAM: i32 = 0 + 7;
const wxCURSOR_LEFT_BUTTON: i32 = 0 + 8;
const wxCURSOR_MAGNIFIER: i32 = 0 + 9;
const wxCURSOR_MIDDLE_BUTTON: i32 = 0 + 10;
const wxCURSOR_NO_ENTRY: i32 = 0 + 11;
const wxCURSOR_PAINT_BRUSH: i32 = 0 + 12;
const wxCURSOR_PENCIL: i32 = 0 + 13;
const wxCURSOR_POINT_LEFT: i32 = 0 + 14;
const wxCURSOR_POINT_RIGHT: i32 = 0 + 15;
const wxCURSOR_QUESTION_ARROW: i32 = 0 + 16;
const wxCURSOR_RIGHT_BUTTON: i32 = 0 + 17;
const wxCURSOR_SIZENESW: i32 = 0 + 18;
const wxCURSOR_SIZENS: i32 = 0 + 19;
const wxCURSOR_SIZENWSE: i32 = 0 + 20;
const wxCURSOR_SIZEWE: i32 = 0 + 21;
const wxCURSOR_SIZING: i32 = 0 + 22;
const wxCURSOR_SPRAYCAN: i32 = 0 + 23;
const wxCURSOR_WAIT: i32 = 0 + 24;
const wxCURSOR_WATCH: i32 = 0 + 25;
const wxCURSOR_BLANK: i32 = 0 + 26;
const wxCURSOR_DEFAULT: i32 = 0 + 27;
const wxCURSOR_COPY_ARROW: i32 = 0 + 28;
const wxCURSOR_CROSS_REVERSE: i32 = 0 + 29;
const wxCURSOR_DOUBLE_ARROW: i32 = 0 + 30;
const wxCURSOR_BASED_ARROW_UP: i32 = 0 + 31;
const wxCURSOR_BASED_ARROW_DOWN: i32 = 0 + 32;
const wxCURSOR_ARROWWAIT: i32 = 0 + 33;
const wxCURSOR_MAX: i32 = 0 + 34;
//  ENUM: wxEllipsizeFlags
const wxELLIPSIZE_FLAGS_NONE: i32 = 0;
const wxELLIPSIZE_FLAGS_PROCESS_MNEMONICS: i32 = 1;
const wxELLIPSIZE_FLAGS_EXPAND_TABS: i32 = 2;
const wxELLIPSIZE_FLAGS_DEFAULT: i32 = wxELLIPSIZE_FLAGS_PROCESS_MNEMONICS|
                                wxELLIPSIZE_FLAGS_EXPAND_TABS;
//  ENUM: wxEllipsizeMode
const wxELLIPSIZE_NONE: i32 = 0;
const wxELLIPSIZE_START: i32 = 0 + 1;
const wxELLIPSIZE_MIDDLE: i32 = 0 + 2;
const wxELLIPSIZE_END: i32 = 0 + 3;

const wxRICHTEXT_FORMAT_STYLE_EDITOR: i32 = 0x0001;
const wxRICHTEXT_FORMAT_FONT: i32 = 0x0002;
const wxRICHTEXT_FORMAT_TABS: i32 = 0x0004;
const wxRICHTEXT_FORMAT_BULLETS: i32 = 0x0008;
const wxRICHTEXT_FORMAT_INDENTS_SPACING: i32 = 0x0010;


//  ENUM: wxNumValidatorStyle
const wxNUM_VAL_DEFAULT: i32 = 0;
const wxNUM_VAL_THOUSANDS_SEPARATOR: i32 = 1;
const wxNUM_VAL_ZERO_AS_BLANK: i32 = 2;
const wxNUM_VAL_NO_TRAILING_ZEROES: i32 = 2 + 1;

const wxGA_HORIZONTAL: i32 = wxHORIZONTAL;
const wxGA_VERTICAL: i32 = wxVERTICAL;
const wxGA_PROGRESS: i32 = 0x0010;
const wxGA_SMOOTH: i32 = 0x0020;
const wxGA_TEXT: i32 = 0x0040;

const wxHTML_ALIGN_LEFT: i32 = 0x0000;
const wxHTML_ALIGN_RIGHT: i32 = 0x0002;
const wxHTML_ALIGN_JUSTIFY: i32 = 0x0010;
const wxHTML_ALIGN_TOP: i32 = 0x0004;
const wxHTML_ALIGN_BOTTOM: i32 = 0x0008;
const wxHTML_ALIGN_CENTER: i32 = 0x0001;
const wxHTML_CLR_FOREGROUND: i32 = 0x0001;
const wxHTML_CLR_BACKGROUND: i32 = 0x0002;
const wxHTML_CLR_TRANSPARENT_BACKGROUND: i32 = 0x0004;
const wxHTML_UNITS_PIXELS: i32 = 0x0001;
const wxHTML_UNITS_PERCENT: i32 = 0x0002;
const wxHTML_INDENT_LEFT: i32 = 0x0010;
const wxHTML_INDENT_RIGHT: i32 = 0x0020;
const wxHTML_INDENT_TOP: i32 = 0x0040;
const wxHTML_INDENT_BOTTOM: i32 = 0x0080;
const wxHTML_INDENT_HORIZONTAL: i32 = (wxHTML_INDENT_LEFT | wxHTML_INDENT_RIGHT);
const wxHTML_INDENT_VERTICAL: i32 = (wxHTML_INDENT_TOP | wxHTML_INDENT_BOTTOM);
const wxHTML_INDENT_ALL: i32 = (wxHTML_INDENT_VERTICAL | wxHTML_INDENT_HORIZONTAL);
const wxHTML_COND_ISANCHOR: i32 = 1;
const wxHTML_COND_ISIMAGEMAP: i32 = 2;
const wxHTML_COND_USER: i32 = 10000;

//  ENUM: wxPropertySheetDialogFlags
const wxPROPSHEET_DEFAULT: i32 = 0x0001;
const wxPROPSHEET_NOTEBOOK: i32 = 0x0002;
const wxPROPSHEET_TOOLBOOK: i32 = 0x0004;
const wxPROPSHEET_CHOICEBOOK: i32 = 0x0008;
const wxPROPSHEET_LISTBOOK: i32 = 0x0010;
const wxPROPSHEET_BUTTONTOOLBOOK: i32 = 0x0020;
const wxPROPSHEET_TREEBOOK: i32 = 0x0040;
const wxPROPSHEET_SHRINKTOFIT: i32 = 0x0100;

//  SKIP: wxTLS_TYPE
// NODEF: wxTLS_VALUE
// NODEF: wxTLS_PTR

const wxPG_PROP_PASSWORD: i32 = wxPG_PROP_CLASS_SPECIFIC_2;
const wxPG_PROP_STATIC_CHOICES: i32 = wxPG_PROP_CLASS_SPECIFIC_1;
const wxPG_PROP_SHOW_FULL_FILENAME: i32 = wxPG_PROP_CLASS_SPECIFIC_1;
const wxPG_PROP_ACTIVE_BTN: i32 = wxPG_PROP_CLASS_SPECIFIC_1;
const wxPG_PROP_USE_CHECKBOX: i32 = wxPG_PROP_CLASS_SPECIFIC_1;
const wxPG_PROP_USE_DCC: i32 = wxPG_PROP_CLASS_SPECIFIC_2;
const wxAEDIALOG_STYLE: i32 =     (wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER | wxOK | wxCANCEL | wxCENTRE);
//  ENUM: wxPGNumericValidationConstants
const wxPG_PROPERTY_VALIDATION_ERROR_MESSAGE: i32 = 0;
const wxPG_PROPERTY_VALIDATION_SATURATE: i32 = 1;
const wxPG_PROPERTY_VALIDATION_WRAP: i32 = 2;

//  ENUM: wxCmdLineEntryFlags
const wxCMD_LINE_OPTION_MANDATORY: i32 = 0x01;
const wxCMD_LINE_PARAM_OPTIONAL: i32 = 0x02;
const wxCMD_LINE_PARAM_MULTIPLE: i32 = 0x04;
const wxCMD_LINE_OPTION_HELP: i32 = 0x08;
const wxCMD_LINE_NEEDS_SEPARATOR: i32 = 0x10;
const wxCMD_LINE_SWITCH_NEGATABLE: i32 = 0x20;
const wxCMD_LINE_HIDDEN: i32 = 0x40;
//  ENUM: wxCmdLineParamType
const wxCMD_LINE_VAL_STRING: i32 = 0;
const wxCMD_LINE_VAL_NUMBER: i32 = 0 + 1;
const wxCMD_LINE_VAL_DATE: i32 = 0 + 2;
const wxCMD_LINE_VAL_DOUBLE: i32 = 0 + 3;
const wxCMD_LINE_VAL_NONE: i32 = 0 + 4;
//  ENUM: wxCmdLineEntryType
const wxCMD_LINE_SWITCH: i32 = 0;
const wxCMD_LINE_OPTION: i32 = 0 + 1;
const wxCMD_LINE_PARAM: i32 = 0 + 2;
const wxCMD_LINE_USAGE_TEXT: i32 = 0 + 3;
const wxCMD_LINE_NONE: i32 = 0 + 4;
//  ENUM: wxCmdLineSwitchState
const wxCMD_SWITCH_OFF: i32 = 0;
const wxCMD_SWITCH_ON: i32 = 0 + 1;
//  ENUM: wxCmdLineSplitType
const wxCMD_LINE_SPLIT_DOS: i32 = 0;
const wxCMD_LINE_SPLIT_UNIX: i32 = 0 + 1;

const wxBU_LEFT: i32 = 0x0040;
const wxBU_TOP: i32 = 0x0080;
const wxBU_RIGHT: i32 = 0x0100;
const wxBU_BOTTOM: i32 = 0x0200;
const wxBU_ALIGN_MASK: i32 = ( wxBU_LEFT | wxBU_TOP | wxBU_RIGHT | wxBU_BOTTOM );
const wxBU_EXACTFIT: i32 = 0x0001;
const wxBU_NOTEXT: i32 = 0x0002;
const wxBU_AUTODRAW: i32 = 0x0004;

//  ENUM: wxTarType
const wxTAR_REGTYPE: i32 = '0';
const wxTAR_LNKTYPE: i32 = '1';
const wxTAR_SYMTYPE: i32 = '2';
const wxTAR_CHRTYPE: i32 = '3';
const wxTAR_BLKTYPE: i32 = '4';
const wxTAR_DIRTYPE: i32 = '5';
const wxTAR_FIFOTYPE: i32 = '6';
const wxTAR_CONTTYPE: i32 = '7';
//  ENUM: wxTarFormat
const wxTAR_USTAR: i32 = 0;
const wxTAR_PAX: i32 = 0 + 1;

const wxHELP_NETSCAPE: i32 = 1;
//  ENUM: wxHelpSearchMode
const wxHELP_SEARCH_INDEX: i32 = 0;
const wxHELP_SEARCH_ALL: i32 = 0 + 1;

//  ENUM: @3
const NO_IMAGE: i32 = -1;

//  ENUM: @0
const WX_ANY_VALUE_BUFFER_SIZE: i32 = 16;

const wxCHB_DEFAULT: i32 = wxBK_DEFAULT;
const wxCHB_TOP: i32 = wxBK_TOP;
const wxCHB_BOTTOM: i32 = wxBK_BOTTOM;
const wxCHB_LEFT: i32 = wxBK_LEFT;
const wxCHB_RIGHT: i32 = wxBK_RIGHT;
const wxCHB_ALIGN_MASK: i32 = wxBK_ALIGN_MASK;

const wxDD_CHANGE_DIR: i32 = 0x0100;
const wxDD_DIR_MUST_EXIST: i32 = 0x0200;
const wxDD_MULTIPLE: i32 = 0x0400;
const wxDD_SHOW_HIDDEN: i32 = 0x0001;
const wxDD_NEW_DIR_BUTTON: i32 = 0;
const wxDD_DEFAULT_STYLE: i32 = (wxDEFAULT_DIALOG_STYLE|wxRESIZE_BORDER);

const wxPB_USE_TEXTCTRL: i32 = 0x0002;
const wxPB_SMALL: i32 = 0x8000;

//  ENUM: TZ
const Local: i32 = 0;
const GMT_12: i32 = 0 + 1;
const GMT_11: i32 = 0 + 2;
const GMT_10: i32 = 0 + 3;
const GMT_9: i32 = 0 + 4;
const GMT_8: i32 = 0 + 5;
const GMT_7: i32 = 0 + 6;
const GMT_6: i32 = 0 + 7;
const GMT_5: i32 = 0 + 8;
const GMT_4: i32 = 0 + 9;
const GMT_3: i32 = 0 + 10;
const GMT_2: i32 = 0 + 11;
const GMT_1: i32 = 0 + 12;
const GMT0: i32 = 0 + 13;
const GMT1: i32 = 0 + 14;
const GMT2: i32 = 0 + 15;
const GMT3: i32 = 0 + 16;
const GMT4: i32 = 0 + 17;
const GMT5: i32 = 0 + 18;
const GMT6: i32 = 0 + 19;
const GMT7: i32 = 0 + 20;
const GMT8: i32 = 0 + 21;
const GMT9: i32 = 0 + 22;
const GMT10: i32 = 0 + 23;
const GMT11: i32 = 0 + 24;
const GMT12: i32 = 0 + 25;
const GMT13: i32 = 0 + 26;
const WET: i32 = GMT0;
const WEST: i32 = GMT1;
const CET: i32 = GMT1;
const CEST: i32 = GMT2;
const EET: i32 = GMT2;
const EEST: i32 = GMT3;
const MSK: i32 = GMT3;
const MSD: i32 = GMT4;
const AST: i32 = GMT_4;
const ADT: i32 = GMT_3;
const EST: i32 = GMT_5;
const EDT: i32 = GMT_4;
const CST: i32 = GMT_6;
const CDT: i32 = GMT_5;
const MST: i32 = GMT_7;
const MDT: i32 = GMT_6;
const PST: i32 = GMT_8;
const PDT: i32 = GMT_7;
const HST: i32 = GMT_10;
const AKST: i32 = GMT_9;
const AKDT: i32 = GMT_8;
const A_WST: i32 = GMT8;
const A_CST: i32 = GMT13 + 1;
const A_EST: i32 = GMT10;
const A_ESST: i32 = GMT11;
const NZST: i32 = GMT12;
const NZDT: i32 = GMT13;
const UTC: i32 = GMT0;
//  ENUM: Calendar
const Gregorian: i32 = 0;
const Julian: i32 = 0 + 1;
//  ENUM: Country
const Country_Unknown: i32 = 0;
const Country_Default: i32 = 0 + 1;
const Country_WesternEurope_Start: i32 = 0 + 2;
const Country_EEC: i32 = Country_WesternEurope_Start;
const France: i32 = Country_WesternEurope_Start + 1;
const Germany: i32 = Country_WesternEurope_Start + 2;
const UK: i32 = Country_WesternEurope_Start + 3;
const Country_WesternEurope_End: i32 = UK;
const Russia: i32 = UK + 1;
const USA: i32 = UK + 2;
//  ENUM: Month
const Jan: i32 = 0;
const Feb: i32 = 0 + 1;
const Mar: i32 = 0 + 2;
const Apr: i32 = 0 + 3;
const May: i32 = 0 + 4;
const Jun: i32 = 0 + 5;
const Jul: i32 = 0 + 6;
const Aug: i32 = 0 + 7;
const Sep: i32 = 0 + 8;
const Oct: i32 = 0 + 9;
const Nov: i32 = 0 + 10;
const Dec: i32 = 0 + 11;
const Inv_Month: i32 = 0 + 12;
//  ENUM: WeekDay
const Sun: i32 = 0;
const Mon: i32 = 0 + 1;
const Tue: i32 = 0 + 2;
const Wed: i32 = 0 + 3;
const Thu: i32 = 0 + 4;
const Fri: i32 = 0 + 5;
const Sat: i32 = 0 + 6;
const Inv_WeekDay: i32 = 0 + 7;
//  ENUM: Year
const Inv_Year: i32 = SHRT_MIN;
//  ENUM: NameFlags
const Name_Full: i32 = 0x01;
const Name_Abbr: i32 = 0x02;
//  ENUM: WeekFlags
const Default_First: i32 = 0;
const Monday_First: i32 = 0 + 1;
const Sunday_First: i32 = 0 + 2;

//  ENUM: @28
const wxPAGE_ODD: i32 = 0;
const wxPAGE_EVEN: i32 = 0 + 1;
const wxPAGE_ALL: i32 = 0 + 2;

//  SKIP: wxTreeListEventHandler
//  ENUM: @52
const wxTL_SINGLE: i32 = 0x0000;
const wxTL_MULTIPLE: i32 = 0x0001;
const wxTL_CHECKBOX: i32 = 0x0002;
const wxTL_3STATE: i32 = 0x0004;
const wxTL_USER_3STATE: i32 = 0x0008;
const wxTL_NO_HEADER: i32 = 0x0010;
const wxTL_DEFAULT_STYLE: i32 = wxTL_SINGLE;
const wxTL_STYLE_MASK: i32 = wxTL_SINGLE |
                          wxTL_MULTIPLE |
                          wxTL_CHECKBOX |
                          wxTL_3STATE |
                          wxTL_USER_3STATE;

