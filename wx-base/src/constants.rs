#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// FIXME: workaround for windows (LLP64)
#![allow(overflowing_literals)]

use std::os::raw::{c_int, c_long};

use crate::manual::*;

//  ENUM: Reason
pub const Reason_Mouse: c_int = 0;
pub const Reason_Unknown: c_int = 0 + 1;

//  ENUM: ConversionFlags
pub const Escape: c_int = 0x01;
pub const QuoteStrings: c_int = 0x02;

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

//  ENUM: @3
pub const NO_IMAGE: c_int = -1;

//  ENUM: @6
pub const NUM_CUSTOM: c_int = 16;

//  ENUM: @9
pub const ShowBelow: c_int = 0x0000;
pub const ShowAbove: c_int = 0x0001;
pub const CanDeferShow: c_int = 0x0002;

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

//  ENUM: EntryType
pub const Type_Unknown: c_int = 0;
pub const Type_String: c_int = 0 + 1;
pub const Type_Boolean: c_int = 0 + 2;
pub const Type_Integer: c_int = 0 + 3;
pub const Type_Float: c_int = 0 + 4;

//  ENUM: Direction
pub const Get: c_int = 0x01;
pub const Set: c_int = 0x02;
pub const Both: c_int = 0x03;

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

//  ENUM: Context
pub const Context_Current: c_int = 0;
pub const Context_Exception: c_int = 0 + 1;

//  ENUM: @16
pub const Event_Skip: c_int = -1;
pub const Event_Ignore: c_int = 0;
pub const Event_Processed: c_int = 1;

//  ENUM: OpenMode
pub const read: c_int = 0;
pub const write: c_int = 0 + 1;
pub const read_write: c_int = 0 + 2;
pub const write_append: c_int = 0 + 3;
pub const write_excl: c_int = 0 + 4;
//  ENUM: @17
pub const fd_invalid: c_int = -1;
pub const fd_stdin: c_int = -1 + 1;
pub const fd_stdout: c_int = -1 + 2;
pub const fd_stderr: c_int = -1 + 3;

//  ENUM: TransferMode
pub const NONE: c_int = 0;
pub const ASCII: c_int = 0 + 1;
pub const BINARY: c_int = 0 + 2;

//  ENUM: wxGridSelectionModes
pub const GridSelectCells: c_int = 0;
pub const GridSelectRows: c_int = 0 + 1;
pub const GridSelectColumns: c_int = 0 + 2;
pub const GridSelectRowsOrColumns: c_int = 0 + 3;
pub const GridSelectNone: c_int = 0 + 4;
//  ENUM: CellSpan
pub const CellSpan_Inside: c_int = -1;
pub const CellSpan_None: c_int = 0;
pub const CellSpan_Main: c_int = 0 + 1;
//  ENUM: TabBehaviour
pub const Tab_Stop: c_int = 0;
pub const Tab_Wrap: c_int = 0 + 1;
pub const Tab_Leave: c_int = 0 + 2;

//  ENUM: Origin
pub const Program: c_int = 0;
pub const Key: c_int = 0 + 1;
pub const Mouse: c_int = 0 + 2;

//  ENUM: wxAttrKind
pub const Any: c_int = 0;
pub const Cell: c_int = 0 + 1;
pub const Row: c_int = 0 + 2;
pub const Col: c_int = 0 + 3;
pub const Default: c_int = 0 + 4;
pub const Merged: c_int = 0 + 5;

//  ENUM: Origin
pub const Origin_Unknown: c_int = 0;
pub const Origin_Keyboard: c_int = 0 + 1;
pub const Origin_HelpButton: c_int = 0 + 2;

//  ENUM: PromptMode
pub const Prompt_Never: c_int = 0;
pub const Prompt_Once: c_int = 0 + 1;
pub const Prompt_Always: c_int = 0 + 2;

//  ENUM: HTMLCursor
pub const HTMLCursor_Default: c_int = 0;
pub const HTMLCursor_Link: c_int = 0 + 1;
pub const HTMLCursor_Text: c_int = 0 + 2;

//  ENUM: @29
pub const FALLBACK_NONE: c_int = 0;
pub const FALLBACK_SYSTEM: c_int = 1;
pub const FALLBACK_NEAREST_LARGER: c_int = 2;

//  ENUM: wxNavigationKeyEventFlags
pub const IsBackward: c_int = 0x0000;
pub const IsForward: c_int = 0x0001;
pub const WinChange: c_int = 0x0002;
pub const FromTab: c_int = 0x0004;

//  ENUM: @38
pub const Timeout_Auto: c_int = -1;
pub const Timeout_Never: c_int = 0;

//  ENUM: Style
pub const Style_None: c_int = 0x00;
pub const Style_WithThousandsSep: c_int = 0x01;
pub const Style_NoTrailingZeroes: c_int = 0x02;

//  ENUM: NumericType
pub const Signed: c_int = 0;
pub const Unsigned: c_int = 0 + 1;
pub const Float: c_int = 0 + 2;

//  ENUM: @40
pub const Selected: c_int = 0x00010000;
pub const ChoicePopup: c_int = 0x00020000;
pub const Control: c_int = 0x00040000;
pub const Disabled: c_int = 0x00080000;
pub const DontUseCellFgCol: c_int = 0x00100000;
pub const DontUseCellBgCol: c_int = 0x00200000;
pub const DontUseCellColours: c_int = DontUseCellFgCol |
                              DontUseCellBgCol;

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

//  ENUM: @44
pub const RICHTEXT_FIELD_STYLE_COMPOSITE: c_int = 0x01;
pub const RICHTEXT_FIELD_STYLE_RECTANGLE: c_int = 0x02;
pub const RICHTEXT_FIELD_STYLE_NO_BORDER: c_int = 0x04;
pub const RICHTEXT_FIELD_STYLE_START_TAG: c_int = 0x08;
pub const RICHTEXT_FIELD_STYLE_END_TAG: c_int = 0x10;

//  ENUM: @45
pub const Option_AllowPixelFontSize: c_int = 0x0001;

//  ENUM: wxRichTextStyleType
pub const RICHTEXT_STYLE_ALL: c_int = 0;
pub const RICHTEXT_STYLE_PARAGRAPH: c_int = 0 + 1;
pub const RICHTEXT_STYLE_CHARACTER: c_int = 0 + 2;
pub const RICHTEXT_STYLE_LIST: c_int = 0 + 3;
pub const RICHTEXT_STYLE_BOX: c_int = 0 + 4;

//  ENUM: ResourceCat
pub const ResourceCat_None: c_int = 0;
pub const ResourceCat_Messages: c_int = 0 + 1;
//  ENUM: Dir
pub const Dir_Cache: c_int = 0;
pub const Dir_Documents: c_int = 0 + 1;
pub const Dir_Desktop: c_int = 0 + 2;
pub const Dir_Downloads: c_int = 0 + 3;
pub const Dir_Music: c_int = 0 + 4;
pub const Dir_Pictures: c_int = 0 + 5;
pub const Dir_Videos: c_int = 0 + 6;
//  ENUM: FileLayout
pub const FileLayout_Classic: c_int = 0;
pub const FileLayout_XDG: c_int = 0 + 1;
//  ENUM: ConfigFileConv
pub const ConfigFileConv_Dot: c_int = 0;
pub const ConfigFileConv_Ext: c_int = 0 + 1;

//  ENUM: ScaleMode
pub const Scale_None: c_int = 0;
pub const Scale_Fill: c_int = 0 + 1;
pub const Scale_AspectFit: c_int = 0 + 2;
pub const Scale_AspectFill: c_int = 0 + 3;

//  ENUM: Kind
pub const Kind_General: c_int = 0;
pub const Kind_Advanced: c_int = 0 + 1;

//  ENUM: BufMode

//  ENUM: Source
pub const Source_Server: c_int = 0;
pub const Source_Proxy: c_int = 0 + 1;

//  ENUM: State
pub const State_Idle: c_int = 0;
pub const State_Unauthorized: c_int = 0 + 1;
pub const State_Active: c_int = 0 + 2;
pub const State_Completed: c_int = 0 + 3;
pub const State_Failed: c_int = 0 + 4;
pub const State_Cancelled: c_int = 0 + 5;
//  ENUM: Storage
pub const Storage_Memory: c_int = 0;
pub const Storage_File: c_int = 0 + 1;
pub const Storage_None: c_int = 0 + 2;

//  ENUM: @57

// NODEF: wxDECLARE_APP
// NODEF: wxIMPLEMENT_APP
//  SKIP: wxDISABLE_DEBUG_SUPPORT

pub const BITMAP_SCREEN_DEPTH: c_int = (-1);

pub const BK_DEFAULT: c_int = 0x0000;
pub const BK_TOP: c_int = 0x0010;
pub const BK_BOTTOM: c_int = 0x0020;
pub const BK_LEFT: c_int = 0x0040;
pub const BK_RIGHT: c_int = 0x0080;
pub const BK_ALIGN_MASK: c_int = (BK_TOP | BK_BOTTOM | BK_LEFT | BK_RIGHT);
//  SKIP: wxBookCtrl
//  ENUM: @2
pub const BK_HITTEST_NOWHERE: c_int = 1;
pub const BK_HITTEST_ONICON: c_int = 2;
pub const BK_HITTEST_ONLABEL: c_int = 4;
pub const BK_HITTEST_ONITEM: c_int = 16;
pub const BK_HITTEST_ONPAGE: c_int = 8;

//  ENUM: @10
pub const CONFIG_USE_LOCAL_FILE: c_int = 1;
pub const CONFIG_USE_GLOBAL_FILE: c_int = 2;
pub const CONFIG_USE_RELATIVE_PATH: c_int = 4;
pub const CONFIG_USE_NO_ESCAPE_CHARACTERS: c_int = 8;
pub const CONFIG_USE_SUBDIR: c_int = 16;

//  SKIP: wxInvalidDateTime

//  ENUM: wxRasterOperationMode
pub const CLEAR: c_int = 0;
pub const XOR: c_int = 0 + 1;
pub const INVERT: c_int = 0 + 2;
pub const OR_REVERSE: c_int = 0 + 3;
pub const AND_REVERSE: c_int = 0 + 4;
pub const COPY: c_int = 0 + 5;
pub const AND: c_int = 0 + 6;
pub const AND_INVERT: c_int = 0 + 7;
pub const NO_OP: c_int = 0 + 8;
pub const NOR: c_int = 0 + 9;
pub const EQUIV: c_int = 0 + 10;
pub const SRC_INVERT: c_int = 0 + 11;
pub const OR_INVERT: c_int = 0 + 12;
pub const NAND: c_int = 0 + 13;
pub const OR: c_int = 0 + 14;
pub const SET: c_int = 0 + 15;
//  ENUM: wxFloodFillStyle
pub const FLOOD_SURFACE: c_int = 1;
pub const FLOOD_BORDER: c_int = 1 + 1;
//  ENUM: wxMappingMode
pub const MM_TEXT: c_int = 1;
pub const MM_METRIC: c_int = 1 + 1;
pub const MM_LOMETRIC: c_int = 1 + 2;
pub const MM_TWIPS: c_int = 1 + 3;
pub const MM_POINTS: c_int = 1 + 4;

pub const DIALOG_NO_PARENT: c_int = 0x00000020;
pub const DEFAULT_DIALOG_STYLE: c_long = (CAPTION | SYSTEM_MENU | CLOSE_BOX);
pub const DIALOG_ADAPTATION_NONE: c_int = 0;
pub const DIALOG_ADAPTATION_STANDARD_SIZER: c_int = 1;
pub const DIALOG_ADAPTATION_ANY_SIZER: c_int = 2;
pub const DIALOG_ADAPTATION_LOOSE_BUTTONS: c_int = 3;
//  ENUM: wxDialogLayoutAdaptationMode
pub const DIALOG_ADAPTATION_MODE_DEFAULT: c_int = 0;
pub const DIALOG_ADAPTATION_MODE_ENABLED: c_int = 1;
pub const DIALOG_ADAPTATION_MODE_DISABLED: c_int = 2;

// NODEF: wxDROP_ICON
//  ENUM: @13
pub const Drag_CopyOnly: c_int = 0;
pub const Drag_AllowMove: c_int = 1;
pub const Drag_DefaultMove: c_int = 3;
//  ENUM: wxDragResult
pub const DragError: c_int = 0;
pub const DragNone: c_int = 0 + 1;
pub const DragCopy: c_int = 0 + 2;
pub const DragMove: c_int = 0 + 3;
pub const DragLink: c_int = 0 + 4;
pub const DragCancel: c_int = 0 + 5;

//  ENUM: wxFontFamily
pub const FONTFAMILY_DEFAULT: c_int = DEFAULT;
pub const FONTFAMILY_DECORATIVE: c_int = DECORATIVE;
pub const FONTFAMILY_ROMAN: c_int = ROMAN;
pub const FONTFAMILY_SCRIPT: c_int = SCRIPT;
pub const FONTFAMILY_SWISS: c_int = SWISS;
pub const FONTFAMILY_MODERN: c_int = MODERN;
pub const FONTFAMILY_TELETYPE: c_int = TELETYPE;
pub const FONTFAMILY_MAX: c_int = TELETYPE + 1;
pub const FONTFAMILY_UNKNOWN: c_int = FONTFAMILY_MAX;
//  ENUM: wxFontStyle
pub const FONTSTYLE_NORMAL: c_int = NORMAL;
pub const FONTSTYLE_ITALIC: c_int = ITALIC;
pub const FONTSTYLE_SLANT: c_int = SLANT;
pub const FONTSTYLE_MAX: c_int = SLANT + 1;
//  ENUM: wxFontWeight
pub const FONTWEIGHT_INVALID: c_int = 0;
pub const FONTWEIGHT_THIN: c_int = 100;
pub const FONTWEIGHT_EXTRALIGHT: c_int = 200;
pub const FONTWEIGHT_LIGHT: c_int = 300;
pub const FONTWEIGHT_NORMAL: c_int = 400;
pub const FONTWEIGHT_MEDIUM: c_int = 500;
pub const FONTWEIGHT_SEMIBOLD: c_int = 600;
pub const FONTWEIGHT_BOLD: c_int = 700;
pub const FONTWEIGHT_EXTRABOLD: c_int = 800;
pub const FONTWEIGHT_HEAVY: c_int = 900;
pub const FONTWEIGHT_EXTRAHEAVY: c_int = 1000;
pub const FONTWEIGHT_MAX: c_int = FONTWEIGHT_EXTRAHEAVY;
//  ENUM: wxFontSymbolicSize
pub const FONTSIZE_XX_SMALL: c_int = -3;
pub const FONTSIZE_X_SMALL: c_int = -3 + 1;
pub const FONTSIZE_SMALL: c_int = -3 + 2;
pub const FONTSIZE_MEDIUM: c_int = -3 + 3;
pub const FONTSIZE_LARGE: c_int = -3 + 4;
pub const FONTSIZE_X_LARGE: c_int = -3 + 5;
pub const FONTSIZE_XX_LARGE: c_int = -3 + 6;
//  ENUM: wxFontFlag
pub const FONTFLAG_DEFAULT: c_int = 0;
pub const FONTFLAG_ITALIC: c_int = 1 << 0;
pub const FONTFLAG_SLANT: c_int = 1 << 1;
pub const FONTFLAG_LIGHT: c_int = 1 << 2;
pub const FONTFLAG_BOLD: c_int = 1 << 3;
pub const FONTFLAG_ANTIALIASED: c_int = 1 << 4;
pub const FONTFLAG_NOT_ANTIALIASED: c_int = 1 << 5;
pub const FONTFLAG_UNDERLINED: c_int = 1 << 6;
pub const FONTFLAG_STRIKETHROUGH: c_int = 1 << 7;
pub const FONTFLAG_MASK: c_int = FONTFLAG_ITALIC             |
                      FONTFLAG_SLANT              |
                      FONTFLAG_LIGHT              |
                      FONTFLAG_BOLD               |
                      FONTFLAG_ANTIALIASED        |
                      FONTFLAG_NOT_ANTIALIASED    |
                      FONTFLAG_UNDERLINED         |
                      FONTFLAG_STRIKETHROUGH;
//  ENUM: wxFontEncoding
pub const FONTENCODING_SYSTEM: c_int = -1;
pub const FONTENCODING_DEFAULT: c_int = -1 + 1;
pub const FONTENCODING_ISO8859_1: c_int = -1 + 2;
pub const FONTENCODING_ISO8859_2: c_int = -1 + 3;
pub const FONTENCODING_ISO8859_3: c_int = -1 + 4;
pub const FONTENCODING_ISO8859_4: c_int = -1 + 5;
pub const FONTENCODING_ISO8859_5: c_int = -1 + 6;
pub const FONTENCODING_ISO8859_6: c_int = -1 + 7;
pub const FONTENCODING_ISO8859_7: c_int = -1 + 8;
pub const FONTENCODING_ISO8859_8: c_int = -1 + 9;
pub const FONTENCODING_ISO8859_9: c_int = -1 + 10;
pub const FONTENCODING_ISO8859_10: c_int = -1 + 11;
pub const FONTENCODING_ISO8859_11: c_int = -1 + 12;
pub const FONTENCODING_ISO8859_12: c_int = -1 + 13;
pub const FONTENCODING_ISO8859_13: c_int = -1 + 14;
pub const FONTENCODING_ISO8859_14: c_int = -1 + 15;
pub const FONTENCODING_ISO8859_15: c_int = -1 + 16;
pub const FONTENCODING_ISO8859_MAX: c_int = -1 + 17;
pub const FONTENCODING_KOI8: c_int = -1 + 18;
pub const FONTENCODING_KOI8_U: c_int = -1 + 19;
pub const FONTENCODING_ALTERNATIVE: c_int = -1 + 20;
pub const FONTENCODING_BULGARIAN: c_int = -1 + 21;
pub const FONTENCODING_CP437: c_int = -1 + 22;
pub const FONTENCODING_CP850: c_int = -1 + 23;
pub const FONTENCODING_CP852: c_int = -1 + 24;
pub const FONTENCODING_CP855: c_int = -1 + 25;
pub const FONTENCODING_CP866: c_int = -1 + 26;
pub const FONTENCODING_CP874: c_int = -1 + 27;
pub const FONTENCODING_CP932: c_int = -1 + 28;
pub const FONTENCODING_CP936: c_int = -1 + 29;
pub const FONTENCODING_CP949: c_int = -1 + 30;
pub const FONTENCODING_CP950: c_int = -1 + 31;
pub const FONTENCODING_CP1250: c_int = -1 + 32;
pub const FONTENCODING_CP1251: c_int = -1 + 33;
pub const FONTENCODING_CP1252: c_int = -1 + 34;
pub const FONTENCODING_CP1253: c_int = -1 + 35;
pub const FONTENCODING_CP1254: c_int = -1 + 36;
pub const FONTENCODING_CP1255: c_int = -1 + 37;
pub const FONTENCODING_CP1256: c_int = -1 + 38;
pub const FONTENCODING_CP1257: c_int = -1 + 39;
pub const FONTENCODING_CP1258: c_int = -1 + 40;
pub const FONTENCODING_CP1361: c_int = -1 + 41;
pub const FONTENCODING_CP12_MAX: c_int = -1 + 42;
pub const FONTENCODING_UTF7: c_int = -1 + 43;
pub const FONTENCODING_UTF8: c_int = -1 + 44;
pub const FONTENCODING_EUC_JP: c_int = -1 + 45;
pub const FONTENCODING_UTF16BE: c_int = -1 + 46;
pub const FONTENCODING_UTF16LE: c_int = -1 + 47;
pub const FONTENCODING_UTF32BE: c_int = -1 + 48;
pub const FONTENCODING_UTF32LE: c_int = -1 + 49;
pub const FONTENCODING_MACROMAN: c_int = -1 + 50;
pub const FONTENCODING_MACJAPANESE: c_int = -1 + 51;
pub const FONTENCODING_MACCHINESETRAD: c_int = -1 + 52;
pub const FONTENCODING_MACKOREAN: c_int = -1 + 53;
pub const FONTENCODING_MACARABIC: c_int = -1 + 54;
pub const FONTENCODING_MACHEBREW: c_int = -1 + 55;
pub const FONTENCODING_MACGREEK: c_int = -1 + 56;
pub const FONTENCODING_MACCYRILLIC: c_int = -1 + 57;
pub const FONTENCODING_MACDEVANAGARI: c_int = -1 + 58;
pub const FONTENCODING_MACGURMUKHI: c_int = -1 + 59;
pub const FONTENCODING_MACGUJARATI: c_int = -1 + 60;
pub const FONTENCODING_MACORIYA: c_int = -1 + 61;
pub const FONTENCODING_MACBENGALI: c_int = -1 + 62;
pub const FONTENCODING_MACTAMIL: c_int = -1 + 63;
pub const FONTENCODING_MACTELUGU: c_int = -1 + 64;
pub const FONTENCODING_MACKANNADA: c_int = -1 + 65;
pub const FONTENCODING_MACMALAJALAM: c_int = -1 + 66;
pub const FONTENCODING_MACSINHALESE: c_int = -1 + 67;
pub const FONTENCODING_MACBURMESE: c_int = -1 + 68;
pub const FONTENCODING_MACKHMER: c_int = -1 + 69;
pub const FONTENCODING_MACTHAI: c_int = -1 + 70;
pub const FONTENCODING_MACLAOTIAN: c_int = -1 + 71;
pub const FONTENCODING_MACGEORGIAN: c_int = -1 + 72;
pub const FONTENCODING_MACARMENIAN: c_int = -1 + 73;
pub const FONTENCODING_MACCHINESESIMP: c_int = -1 + 74;
pub const FONTENCODING_MACTIBETAN: c_int = -1 + 75;
pub const FONTENCODING_MACMONGOLIAN: c_int = -1 + 76;
pub const FONTENCODING_MACETHIOPIC: c_int = -1 + 77;
pub const FONTENCODING_MACCENTRALEUR: c_int = -1 + 78;
pub const FONTENCODING_MACVIATNAMESE: c_int = -1 + 79;
pub const FONTENCODING_MACARABICEXT: c_int = -1 + 80;
pub const FONTENCODING_MACSYMBOL: c_int = -1 + 81;
pub const FONTENCODING_MACDINGBATS: c_int = -1 + 82;
pub const FONTENCODING_MACTURKISH: c_int = -1 + 83;
pub const FONTENCODING_MACCROATIAN: c_int = -1 + 84;
pub const FONTENCODING_MACICELANDIC: c_int = -1 + 85;
pub const FONTENCODING_MACROMANIAN: c_int = -1 + 86;
pub const FONTENCODING_MACCELTIC: c_int = -1 + 87;
pub const FONTENCODING_MACGAELIC: c_int = -1 + 88;
pub const FONTENCODING_MACKEYBOARD: c_int = -1 + 89;
pub const FONTENCODING_ISO2022_JP: c_int = -1 + 90;
pub const FONTENCODING_MAX: c_int = -1 + 91;
pub const FONTENCODING_MACMIN: c_int = FONTENCODING_MACROMAN;
pub const FONTENCODING_MACMAX: c_int = FONTENCODING_MACKEYBOARD;
pub const FONTENCODING_UTF16: c_int = FONTENCODING_MACKEYBOARD + 1;
pub const FONTENCODING_UTF32: c_int = FONTENCODING_MACKEYBOARD + 2;
pub const FONTENCODING_UNICODE: c_int = FONTENCODING_MACKEYBOARD + 3;
pub const FONTENCODING_GB2312: c_int = FONTENCODING_CP936;
pub const FONTENCODING_BIG5: c_int = FONTENCODING_CP950;
pub const FONTENCODING_SHIFT_JIS: c_int = FONTENCODING_CP932;
pub const FONTENCODING_EUC_KR: c_int = FONTENCODING_CP949;
pub const FONTENCODING_JOHAB: c_int = FONTENCODING_CP1361;
pub const FONTENCODING_VIETNAMESE: c_int = FONTENCODING_CP1258;

pub const GRID_AUTOSIZE: c_int = (-1);
//  ENUM: wxGridCellFloatFormat
pub const GRID_FLOAT_FORMAT_FIXED: c_int = 0x0010;
pub const GRID_FLOAT_FORMAT_SCIENTIFIC: c_int = 0x0020;
pub const GRID_FLOAT_FORMAT_COMPACT: c_int = 0x0040;
pub const GRID_FLOAT_FORMAT_UPPER: c_int = 0x0080;
pub const GRID_FLOAT_FORMAT_DEFAULT: c_int = GRID_FLOAT_FORMAT_FIXED;
//  ENUM: wxGridTableRequest
pub const GRIDTABLE_NOTIFY_ROWS_INSERTED: c_int = 0;
pub const GRIDTABLE_NOTIFY_ROWS_APPENDED: c_int = 0 + 1;
pub const GRIDTABLE_NOTIFY_ROWS_DELETED: c_int = 0 + 2;
pub const GRIDTABLE_NOTIFY_COLS_INSERTED: c_int = 0 + 3;
pub const GRIDTABLE_NOTIFY_COLS_APPENDED: c_int = 0 + 4;
pub const GRIDTABLE_NOTIFY_COLS_DELETED: c_int = 0 + 5;
//  ENUM: wxGridRenderStyle
pub const GRID_DRAW_ROWS_HEADER: c_int = 0x001;
pub const GRID_DRAW_COLS_HEADER: c_int = 0x002;
pub const GRID_DRAW_CELL_LINES: c_int = 0x004;
pub const GRID_DRAW_BOX_RECT: c_int = 0x008;
pub const GRID_DRAW_SELECTION: c_int = 0x010;
pub const GRID_DRAW_DEFAULT: c_int = GRID_DRAW_ROWS_HEADER |
                          GRID_DRAW_COLS_HEADER |
                          GRID_DRAW_CELL_LINES |
                          GRID_DRAW_BOX_RECT;

pub const LC_VRULES: c_int = 0x0001;
pub const LC_HRULES: c_int = 0x0002;
pub const LC_ICON: c_int = 0x0004;
pub const LC_SMALL_ICON: c_int = 0x0008;
pub const LC_LIST: c_int = 0x0010;
pub const LC_REPORT: c_int = 0x0020;
pub const LC_ALIGN_TOP: c_int = 0x0040;
pub const LC_ALIGN_LEFT: c_int = 0x0080;
pub const LC_AUTOARRANGE: c_int = 0x0100;
pub const LC_VIRTUAL: c_int = 0x0200;
pub const LC_EDIT_LABELS: c_int = 0x0400;
pub const LC_NO_HEADER: c_int = 0x0800;
pub const LC_NO_SORT_HEADER: c_int = 0x1000;
pub const LC_SINGLE_SEL: c_int = 0x2000;
pub const LC_SORT_ASCENDING: c_int = 0x4000;
pub const LC_SORT_DESCENDING: c_int = 0x8000;
pub const LC_MASK_TYPE: c_int = (LC_ICON | LC_SMALL_ICON | LC_LIST | LC_REPORT);
pub const LC_MASK_ALIGN: c_int = (LC_ALIGN_TOP | LC_ALIGN_LEFT);
pub const LC_MASK_SORT: c_int = (LC_SORT_ASCENDING | LC_SORT_DESCENDING);
pub const LIST_MASK_STATE: c_int = 0x0001;
pub const LIST_MASK_TEXT: c_int = 0x0002;
pub const LIST_MASK_IMAGE: c_int = 0x0004;
pub const LIST_MASK_DATA: c_int = 0x0008;
pub const LIST_SET_ITEM: c_int = 0x0010;
pub const LIST_MASK_WIDTH: c_int = 0x0020;
pub const LIST_MASK_FORMAT: c_int = 0x0040;
pub const LIST_STATE_DONTCARE: c_int = 0x0000;
pub const LIST_STATE_DROPHILITED: c_int = 0x0001;
pub const LIST_STATE_FOCUSED: c_int = 0x0002;
pub const LIST_STATE_SELECTED: c_int = 0x0004;
pub const LIST_STATE_CUT: c_int = 0x0008;
pub const LIST_HITTEST_ABOVE: c_int = 0x0001;
pub const LIST_HITTEST_BELOW: c_int = 0x0002;
pub const LIST_HITTEST_NOWHERE: c_int = 0x0004;
pub const LIST_HITTEST_ONITEMICON: c_int = 0x0020;
pub const LIST_HITTEST_ONITEMLABEL: c_int = 0x0080;
pub const LIST_HITTEST_ONITEMSTATEICON: c_int = 0x0200;
pub const LIST_HITTEST_TOLEFT: c_int = 0x0400;
pub const LIST_HITTEST_TORIGHT: c_int = 0x0800;
pub const LIST_HITTEST_ONITEM: c_int = (LIST_HITTEST_ONITEMICON | LIST_HITTEST_ONITEMLABEL | LIST_HITTEST_ONITEMSTATEICON);
pub const LIST_GETSUBITEMRECT_WHOLEITEM: c_int = -1;
//  ENUM: @32
pub const LIST_NEXT_ABOVE: c_int = 0;
pub const LIST_NEXT_ALL: c_int = 0 + 1;
pub const LIST_NEXT_BELOW: c_int = 0 + 2;
pub const LIST_NEXT_LEFT: c_int = 0 + 3;
pub const LIST_NEXT_RIGHT: c_int = 0 + 4;
//  ENUM: @33
pub const LIST_ALIGN_DEFAULT: c_int = 0;
pub const LIST_ALIGN_LEFT: c_int = 0 + 1;
pub const LIST_ALIGN_TOP: c_int = 0 + 2;
pub const LIST_ALIGN_SNAP_TO_GRID: c_int = 0 + 3;
//  ENUM: wxListColumnFormat
pub const LIST_FORMAT_LEFT: c_int = 0;
pub const LIST_FORMAT_RIGHT: c_int = 0 + 1;
pub const LIST_FORMAT_CENTRE: c_int = 0 + 2;
pub const LIST_FORMAT_CENTER: c_int = LIST_FORMAT_CENTRE;
//  ENUM: @34
pub const LIST_AUTOSIZE: c_int = -1;
pub const LIST_AUTOSIZE_USEHEADER: c_int = -2;
//  ENUM: @35
pub const LIST_RECT_BOUNDS: c_int = 0;
pub const LIST_RECT_ICON: c_int = 0 + 1;
pub const LIST_RECT_LABEL: c_int = 0 + 2;
//  ENUM: @36
pub const LIST_FIND_UP: c_int = 0;
pub const LIST_FIND_DOWN: c_int = 0 + 1;
pub const LIST_FIND_LEFT: c_int = 0 + 2;
pub const LIST_FIND_RIGHT: c_int = 0 + 3;

// NODEF: wxDISABLE_DEBUG_LOGGING_IN_RELEASE_BUILD
//  ENUM: wxLogLevelValues
pub const LOG_FatalError: c_int = 0;
pub const LOG_Error: c_int = 0 + 1;
pub const LOG_Warning: c_int = 0 + 2;
pub const LOG_Message: c_int = 0 + 3;
pub const LOG_Status: c_int = 0 + 4;
pub const LOG_Info: c_int = 0 + 5;
pub const LOG_Debug: c_int = 0 + 6;
pub const LOG_Trace: c_int = 0 + 7;
pub const LOG_Progress: c_int = 0 + 8;
pub const LOG_User: c_int = 100;
pub const LOG_Max: c_int = 10000;

pub const PG_DEFAULT_STYLE: c_int = (0);
pub const PGMAN_DEFAULT_STYLE: c_int = (0);
//  SKIP: wxPGVFBFlags
//  ENUM: wxPG_WINDOW_STYLES
pub const PG_AUTO_SORT: c_int = 0x00000010;
pub const PG_HIDE_CATEGORIES: c_int = 0x00000020;
pub const PG_ALPHABETIC_MODE: c_int = (PG_HIDE_CATEGORIES|PG_AUTO_SORT);
pub const PG_BOLD_MODIFIED: c_int = 0x00000040;
pub const PG_SPLITTER_AUTO_CENTER: c_int = 0x00000080;
pub const PG_TOOLTIPS: c_int = 0x00000100;
pub const PG_HIDE_MARGIN: c_int = 0x00000200;
pub const PG_STATIC_SPLITTER: c_int = 0x00000400;
pub const PG_STATIC_LAYOUT: c_int = (PG_HIDE_MARGIN|PG_STATIC_SPLITTER);
pub const PG_LIMITED_EDITING: c_int = 0x00000800;
pub const PG_TOOLBAR: c_int = 0x00001000;
pub const PG_DESCRIPTION: c_int = 0x00002000;
pub const PG_NO_INTERNAL_BORDER: c_int = 0x00004000;
pub const PG_WINDOW_STYLE_MASK: c_int = PG_AUTO_SORT|PG_HIDE_CATEGORIES|PG_BOLD_MODIFIED|
                         PG_SPLITTER_AUTO_CENTER|PG_TOOLTIPS|PG_HIDE_MARGIN|
                         PG_STATIC_SPLITTER|PG_LIMITED_EDITING|PG_TOOLBAR|
                         PG_DESCRIPTION|PG_NO_INTERNAL_BORDER;
//  ENUM: wxPG_EX_WINDOW_STYLES
pub const PG_EX_INIT_NOCAT: c_int = 0x00001000;
pub const PG_EX_NO_FLAT_TOOLBAR: c_int = 0x00002000;
pub const PG_EX_MODE_BUTTONS: c_int = 0x00008000;
pub const PG_EX_HELP_AS_TOOLTIPS: c_int = 0x00010000;
pub const PG_EX_NATIVE_DOUBLE_BUFFERING: c_int = 0x00080000;
pub const PG_EX_AUTO_UNSPECIFIED_VALUES: c_int = 0x00200000;
pub const PG_EX_WRITEONLY_BUILTIN_ATTRIBUTES: c_int = 0x00400000;
pub const PG_EX_HIDE_PAGE_BUTTONS: c_int = 0x01000000;
pub const PG_EX_MULTIPLE_SELECTION: c_int = 0x02000000;
pub const PG_EX_ENABLE_TLP_TRACKING: c_int = 0x04000000;
pub const PG_EX_NO_TOOLBAR_DIVIDER: c_int = 0x04000000;
pub const PG_EX_TOOLBAR_SEPARATOR: c_int = 0x08000000;
pub const PG_EX_ALWAYS_ALLOW_FOCUS: c_int = 0x00100000;
pub const PG_EX_WINDOW_PG_STYLE_MASK: c_int = PG_EX_INIT_NOCAT|PG_EX_HELP_AS_TOOLTIPS|PG_EX_NATIVE_DOUBLE_BUFFERING|
                               PG_EX_AUTO_UNSPECIFIED_VALUES|PG_EX_WRITEONLY_BUILTIN_ATTRIBUTES|
                               PG_EX_MULTIPLE_SELECTION|PG_EX_ENABLE_TLP_TRACKING|PG_EX_ALWAYS_ALLOW_FOCUS;
pub const PG_EX_WINDOW_PGMAN_STYLE_MASK: c_int = PG_EX_NO_FLAT_TOOLBAR|PG_EX_MODE_BUTTONS|PG_EX_HIDE_PAGE_BUTTONS|
                                  PG_EX_NO_TOOLBAR_DIVIDER|PG_EX_TOOLBAR_SEPARATOR;
pub const PG_EX_WINDOW_STYLE_MASK: c_int = PG_EX_WINDOW_PG_STYLE_MASK|PG_EX_WINDOW_PGMAN_STYLE_MASK;
//  ENUM: wxPG_VALIDATION_FAILURE_BEHAVIOR_FLAGS
pub const PG_VFB_STAY_IN_PROPERTY: c_int = 0x01;
pub const PG_VFB_BEEP: c_int = 0x02;
pub const PG_VFB_MARK_CELL: c_int = 0x04;
pub const PG_VFB_SHOW_MESSAGE: c_int = 0x08;
pub const PG_VFB_SHOW_MESSAGEBOX: c_int = 0x10;
pub const PG_VFB_SHOW_MESSAGE_ON_STATUSBAR: c_int = 0x20;
pub const PG_VFB_DEFAULT: c_int = PG_VFB_MARK_CELL |
                                      PG_VFB_SHOW_MESSAGEBOX;
//  ENUM: wxPG_KEYBOARD_ACTIONS
pub const PG_ACTION_INVALID: c_int = 0;
pub const PG_ACTION_NEXT_PROPERTY: c_int = 0 + 1;
pub const PG_ACTION_PREV_PROPERTY: c_int = 0 + 2;
pub const PG_ACTION_EXPAND_PROPERTY: c_int = 0 + 3;
pub const PG_ACTION_COLLAPSE_PROPERTY: c_int = 0 + 4;
pub const PG_ACTION_CANCEL_EDIT: c_int = 0 + 5;
pub const PG_ACTION_EDIT: c_int = 0 + 6;
pub const PG_ACTION_PRESS_BUTTON: c_int = 0 + 7;
pub const PG_ACTION_MAX: c_int = 0 + 8;

pub const RE_READONLY: c_int = 0x0010;
pub const RE_MULTILINE: c_int = 0x0020;
pub const RE_CENTRE_CARET: c_int = 0x8000;
pub const RE_CENTER_CARET: c_int = RE_CENTRE_CARET;
pub const RICHTEXT_SHIFT_DOWN: c_int = 0x01;
pub const RICHTEXT_CTRL_DOWN: c_int = 0x02;
pub const RICHTEXT_ALT_DOWN: c_int = 0x04;
pub const RICHTEXT_EX_NO_GUIDELINES: c_int = 0x00000100;
//  SKIP: wxRICHTEXT_DEFAULT_OVERALL_SIZE
//  SKIP: wxRICHTEXT_DEFAULT_IMAGE_SIZE
pub const RICHTEXT_DEFAULT_SPACING: c_int = 3;
pub const RICHTEXT_DEFAULT_MARGIN: c_int = 3;
//  SKIP: wxRICHTEXT_DEFAULT_UNFOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_FOCUSSED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_UNSELECTED_BACKGROUND
//  SKIP: wxRICHTEXT_DEFAULT_TYPE_COLOUR
//  SKIP: wxRICHTEXT_DEFAULT_FOCUS_RECT_COLOUR
pub const RICHTEXT_DEFAULT_CARET_WIDTH: c_int = 2;
pub const RICHTEXT_DEFAULT_DELAYED_LAYOUT_THRESHOLD: c_int = 20000;
pub const RICHTEXT_DEFAULT_LAYOUT_INTERVAL: c_int = 50;
pub const RICHTEXT_DEFAULT_DELAYED_IMAGE_PROCESSING_INTERVAL: c_int = 200;
pub const ID_RICHTEXT_PROPERTIES1: c_int = (ID_HIGHEST + 1);
pub const ID_RICHTEXT_PROPERTIES2: c_int = (ID_HIGHEST + 2);
pub const ID_RICHTEXT_PROPERTIES3: c_int = (ID_HIGHEST + 3);
//  ENUM: wxRichTextCtrlSelectionState
pub const RichTextCtrlSelectionState_Normal: c_int = 0;
pub const RichTextCtrlSelectionState_CommonAncestor: c_int = 0 + 1;

//  ENUM: wxFlexSizerGrowMode
pub const FLEX_GROWMODE_NONE: c_int = 0;
pub const FLEX_GROWMODE_SPECIFIED: c_int = 0 + 1;
pub const FLEX_GROWMODE_ALL: c_int = 0 + 2;

//  ENUM: wxStreamError
pub const STREAM_NO_ERROR: c_int = 0;
pub const STREAM_EOF: c_int = 0 + 1;
pub const STREAM_WRITE_ERROR: c_int = 0 + 2;
pub const STREAM_READ_ERROR: c_int = 0 + 3;
//  ENUM: wxStreamProtocolType
pub const STREAM_PROTOCOL: c_int = 0;
pub const STREAM_MIMETYPE: c_int = 0 + 1;
pub const STREAM_ENCODING: c_int = 0 + 2;
pub const STREAM_FILEEXT: c_int = 0 + 3;

// NODEF: wxCRIT_SECT_DECLARE
// NODEF: wxCRIT_SECT_DECLARE_MEMBER
// NODEF: wxCRIT_SECT_LOCKER
// NODEF: wxCRITICAL_SECTION
// NODEF: wxLEAVE_CRIT_SECT
// NODEF: wxENTER_CRIT_SECT
//  ENUM: wxCondError
pub const COND_NO_ERROR: c_int = 0;
pub const COND_INVALID: c_int = 0 + 1;
pub const COND_TIMEOUT: c_int = 0 + 2;
pub const COND_MISC_ERROR: c_int = 0 + 3;
//  ENUM: wxCriticalSectionType
pub const CRITSEC_DEFAULT: c_int = 0;
pub const CRITSEC_NON_RECURSIVE: c_int = 0 + 1;
//  ENUM: wxThreadWait
pub const THREAD_WAIT_BLOCK: c_int = 0;
pub const THREAD_WAIT_YIELD: c_int = 0 + 1;
pub const THREAD_WAIT_DEFAULT: c_int = THREAD_WAIT_YIELD;
//  ENUM: wxThreadKind
pub const THREAD_DETACHED: c_int = 0;
pub const THREAD_JOINABLE: c_int = 0 + 1;
//  ENUM: wxThreadError
pub const THREAD_NO_ERROR: c_int = 0;
pub const THREAD_NO_RESOURCE: c_int = 0 + 1;
pub const THREAD_RUNNING: c_int = 0 + 2;
pub const THREAD_NOT_RUNNING: c_int = 0 + 3;
pub const THREAD_KILLED: c_int = 0 + 4;
pub const THREAD_MISC_ERROR: c_int = 0 + 5;
//  ENUM: wxSemaError
pub const SEMA_NO_ERROR: c_int = 0;
pub const SEMA_INVALID: c_int = 0 + 1;
pub const SEMA_BUSY: c_int = 0 + 2;
pub const SEMA_TIMEOUT: c_int = 0 + 3;
pub const SEMA_OVERFLOW: c_int = 0 + 4;
pub const SEMA_MISC_ERROR: c_int = 0 + 5;
//  ENUM: wxMutexType
pub const MUTEX_DEFAULT: c_int = 0;
pub const MUTEX_RECURSIVE: c_int = 0 + 1;
//  ENUM: wxMutexError
pub const MUTEX_NO_ERROR: c_int = 0;
pub const MUTEX_INVALID: c_int = 0 + 1;
pub const MUTEX_DEAD_LOCK: c_int = 0 + 2;
pub const MUTEX_BUSY: c_int = 0 + 3;
pub const MUTEX_UNLOCKED: c_int = 0 + 4;
pub const MUTEX_TIMEOUT: c_int = 0 + 5;
pub const MUTEX_MISC_ERROR: c_int = 0 + 6;

//  ENUM: wxToolBarToolStyle
pub const TOOL_STYLE_BUTTON: c_int = 1;
pub const TOOL_STYLE_SEPARATOR: c_int = 2;
pub const TOOL_STYLE_CONTROL: c_int = 2 + 1;
//  ENUM: @49
pub const TB_HORIZONTAL: c_int = HORIZONTAL;
pub const TB_TOP: c_int = TB_HORIZONTAL;
pub const TB_VERTICAL: c_int = VERTICAL;
pub const TB_LEFT: c_int = TB_VERTICAL;
pub const TB_FLAT: c_int = TB_VERTICAL + 1;
pub const TB_DOCKABLE: c_int = TB_VERTICAL + 2;
pub const TB_NOICONS: c_int = TB_VERTICAL + 3;
pub const TB_TEXT: c_int = TB_VERTICAL + 4;
pub const TB_NODIVIDER: c_int = TB_VERTICAL + 5;
pub const TB_NOALIGN: c_int = TB_VERTICAL + 6;
pub const TB_HORZ_LAYOUT: c_int = TB_VERTICAL + 7;
pub const TB_HORZ_TEXT: c_int = TB_HORZ_LAYOUT | TB_TEXT;
pub const TB_NO_TOOLTIPS: c_int = TB_HORZ_LAYOUT | TB_TEXT + 1;
pub const TB_BOTTOM: c_int = TB_HORZ_LAYOUT | TB_TEXT + 2;
pub const TB_RIGHT: c_int = TB_HORZ_LAYOUT | TB_TEXT + 3;
pub const TB_DEFAULT_STYLE: c_int = TB_HORIZONTAL;

//  ENUM: wxAcceleratorEntryFlags
pub const ACCEL_NORMAL: c_int = 0;
pub const ACCEL_ALT: c_int = 0 + 1;
pub const ACCEL_CTRL: c_int = 0 + 2;
pub const ACCEL_SHIFT: c_int = 0 + 3;
pub const ACCEL_RAW_CTRL: c_int = 0 + 4;
pub const ACCEL_CMD: c_int = 0 + 5;

pub const ACC_SELF: c_int = 0;
pub const ACC_STATE_SYSTEM_ALERT_HIGH: c_int = 0x00000001;
pub const ACC_STATE_SYSTEM_ALERT_MEDIUM: c_int = 0x00000002;
pub const ACC_STATE_SYSTEM_ALERT_LOW: c_int = 0x00000004;
pub const ACC_STATE_SYSTEM_ANIMATED: c_int = 0x00000008;
pub const ACC_STATE_SYSTEM_BUSY: c_int = 0x00000010;
pub const ACC_STATE_SYSTEM_CHECKED: c_int = 0x00000020;
pub const ACC_STATE_SYSTEM_COLLAPSED: c_int = 0x00000040;
pub const ACC_STATE_SYSTEM_DEFAULT: c_int = 0x00000080;
pub const ACC_STATE_SYSTEM_EXPANDED: c_int = 0x00000100;
pub const ACC_STATE_SYSTEM_EXTSELECTABLE: c_int = 0x00000200;
pub const ACC_STATE_SYSTEM_FLOATING: c_int = 0x00000400;
pub const ACC_STATE_SYSTEM_FOCUSABLE: c_int = 0x00000800;
pub const ACC_STATE_SYSTEM_FOCUSED: c_int = 0x00001000;
pub const ACC_STATE_SYSTEM_HOTTRACKED: c_int = 0x00002000;
pub const ACC_STATE_SYSTEM_INVISIBLE: c_int = 0x00004000;
pub const ACC_STATE_SYSTEM_MARQUEED: c_int = 0x00008000;
pub const ACC_STATE_SYSTEM_MIXED: c_int = 0x00010000;
pub const ACC_STATE_SYSTEM_MULTISELECTABLE: c_int = 0x00020000;
pub const ACC_STATE_SYSTEM_OFFSCREEN: c_int = 0x00040000;
pub const ACC_STATE_SYSTEM_PRESSED: c_int = 0x00080000;
pub const ACC_STATE_SYSTEM_PROTECTED: c_int = 0x00100000;
pub const ACC_STATE_SYSTEM_READONLY: c_int = 0x00200000;
pub const ACC_STATE_SYSTEM_SELECTABLE: c_int = 0x00400000;
pub const ACC_STATE_SYSTEM_SELECTED: c_int = 0x00800000;
pub const ACC_STATE_SYSTEM_SELFVOICING: c_int = 0x01000000;
pub const ACC_STATE_SYSTEM_UNAVAILABLE: c_int = 0x02000000;
pub const ACC_EVENT_SYSTEM_SOUND: c_int = 0x0001;
pub const ACC_EVENT_SYSTEM_ALERT: c_int = 0x0002;
pub const ACC_EVENT_SYSTEM_FOREGROUND: c_int = 0x0003;
pub const ACC_EVENT_SYSTEM_MENUSTART: c_int = 0x0004;
pub const ACC_EVENT_SYSTEM_MENUEND: c_int = 0x0005;
pub const ACC_EVENT_SYSTEM_MENUPOPUPSTART: c_int = 0x0006;
pub const ACC_EVENT_SYSTEM_MENUPOPUPEND: c_int = 0x0007;
pub const ACC_EVENT_SYSTEM_CAPTURESTART: c_int = 0x0008;
pub const ACC_EVENT_SYSTEM_CAPTUREEND: c_int = 0x0009;
pub const ACC_EVENT_SYSTEM_MOVESIZESTART: c_int = 0x000A;
pub const ACC_EVENT_SYSTEM_MOVESIZEEND: c_int = 0x000B;
pub const ACC_EVENT_SYSTEM_CONTEXTHELPSTART: c_int = 0x000C;
pub const ACC_EVENT_SYSTEM_CONTEXTHELPEND: c_int = 0x000D;
pub const ACC_EVENT_SYSTEM_DRAGDROPSTART: c_int = 0x000E;
pub const ACC_EVENT_SYSTEM_DRAGDROPEND: c_int = 0x000F;
pub const ACC_EVENT_SYSTEM_DIALOGSTART: c_int = 0x0010;
pub const ACC_EVENT_SYSTEM_DIALOGEND: c_int = 0x0011;
pub const ACC_EVENT_SYSTEM_SCROLLINGSTART: c_int = 0x0012;
pub const ACC_EVENT_SYSTEM_SCROLLINGEND: c_int = 0x0013;
pub const ACC_EVENT_SYSTEM_SWITCHSTART: c_int = 0x0014;
pub const ACC_EVENT_SYSTEM_SWITCHEND: c_int = 0x0015;
pub const ACC_EVENT_SYSTEM_MINIMIZESTART: c_int = 0x0016;
pub const ACC_EVENT_SYSTEM_MINIMIZEEND: c_int = 0x0017;
pub const ACC_EVENT_OBJECT_CREATE: c_int = 0x8000;
pub const ACC_EVENT_OBJECT_DESTROY: c_int = 0x8001;
pub const ACC_EVENT_OBJECT_SHOW: c_int = 0x8002;
pub const ACC_EVENT_OBJECT_HIDE: c_int = 0x8003;
pub const ACC_EVENT_OBJECT_REORDER: c_int = 0x8004;
pub const ACC_EVENT_OBJECT_FOCUS: c_int = 0x8005;
pub const ACC_EVENT_OBJECT_SELECTION: c_int = 0x8006;
pub const ACC_EVENT_OBJECT_SELECTIONADD: c_int = 0x8007;
pub const ACC_EVENT_OBJECT_SELECTIONREMOVE: c_int = 0x8008;
pub const ACC_EVENT_OBJECT_SELECTIONWITHIN: c_int = 0x8009;
pub const ACC_EVENT_OBJECT_STATECHANGE: c_int = 0x800A;
pub const ACC_EVENT_OBJECT_LOCATIONCHANGE: c_int = 0x800B;
pub const ACC_EVENT_OBJECT_NAMECHANGE: c_int = 0x800C;
pub const ACC_EVENT_OBJECT_DESCRIPTIONCHANGE: c_int = 0x800D;
pub const ACC_EVENT_OBJECT_VALUECHANGE: c_int = 0x800E;
pub const ACC_EVENT_OBJECT_PARENTCHANGE: c_int = 0x800F;
pub const ACC_EVENT_OBJECT_HELPCHANGE: c_int = 0x8010;
pub const ACC_EVENT_OBJECT_DEFACTIONCHANGE: c_int = 0x8011;
pub const ACC_EVENT_OBJECT_ACCELERATORCHANGE: c_int = 0x8012;
//  ENUM: wxAccStatus
pub const ACC_FAIL: c_int = 0;
pub const ACC_FALSE: c_int = 0 + 1;
pub const ACC_OK: c_int = 0 + 2;
pub const ACC_NOT_IMPLEMENTED: c_int = 0 + 3;
pub const ACC_NOT_SUPPORTED: c_int = 0 + 4;
pub const ACC_INVALID_ARG: c_int = 0 + 5;
//  ENUM: wxNavDir
pub const NAVDIR_FIRSTCHILD: c_int = 0;
pub const NAVDIR_LASTCHILD: c_int = 0 + 1;
pub const NAVDIR_DOWN: c_int = 0 + 2;
pub const NAVDIR_LEFT: c_int = 0 + 3;
pub const NAVDIR_NEXT: c_int = 0 + 4;
pub const NAVDIR_PREVIOUS: c_int = 0 + 5;
pub const NAVDIR_RIGHT: c_int = 0 + 6;
pub const NAVDIR_UP: c_int = 0 + 7;
//  ENUM: wxAccRole
pub const ROLE_NONE: c_int = 0;
pub const ROLE_SYSTEM_ALERT: c_int = 0 + 1;
pub const ROLE_SYSTEM_ANIMATION: c_int = 0 + 2;
pub const ROLE_SYSTEM_APPLICATION: c_int = 0 + 3;
pub const ROLE_SYSTEM_BORDER: c_int = 0 + 4;
pub const ROLE_SYSTEM_BUTTONDROPDOWN: c_int = 0 + 5;
pub const ROLE_SYSTEM_BUTTONDROPDOWNGRID: c_int = 0 + 6;
pub const ROLE_SYSTEM_BUTTONMENU: c_int = 0 + 7;
pub const ROLE_SYSTEM_CARET: c_int = 0 + 8;
pub const ROLE_SYSTEM_CELL: c_int = 0 + 9;
pub const ROLE_SYSTEM_CHARACTER: c_int = 0 + 10;
pub const ROLE_SYSTEM_CHART: c_int = 0 + 11;
pub const ROLE_SYSTEM_CHECKBUTTON: c_int = 0 + 12;
pub const ROLE_SYSTEM_CLIENT: c_int = 0 + 13;
pub const ROLE_SYSTEM_CLOCK: c_int = 0 + 14;
pub const ROLE_SYSTEM_COLUMN: c_int = 0 + 15;
pub const ROLE_SYSTEM_COLUMNHEADER: c_int = 0 + 16;
pub const ROLE_SYSTEM_COMBOBOX: c_int = 0 + 17;
pub const ROLE_SYSTEM_CURSOR: c_int = 0 + 18;
pub const ROLE_SYSTEM_DIAGRAM: c_int = 0 + 19;
pub const ROLE_SYSTEM_DIAL: c_int = 0 + 20;
pub const ROLE_SYSTEM_DIALOG: c_int = 0 + 21;
pub const ROLE_SYSTEM_DOCUMENT: c_int = 0 + 22;
pub const ROLE_SYSTEM_DROPLIST: c_int = 0 + 23;
pub const ROLE_SYSTEM_EQUATION: c_int = 0 + 24;
pub const ROLE_SYSTEM_GRAPHIC: c_int = 0 + 25;
pub const ROLE_SYSTEM_GRIP: c_int = 0 + 26;
pub const ROLE_SYSTEM_GROUPING: c_int = 0 + 27;
pub const ROLE_SYSTEM_HELPBALLOON: c_int = 0 + 28;
pub const ROLE_SYSTEM_HOTKEYFIELD: c_int = 0 + 29;
pub const ROLE_SYSTEM_INDICATOR: c_int = 0 + 30;
pub const ROLE_SYSTEM_LINK: c_int = 0 + 31;
pub const ROLE_SYSTEM_LIST: c_int = 0 + 32;
pub const ROLE_SYSTEM_LISTITEM: c_int = 0 + 33;
pub const ROLE_SYSTEM_MENUBAR: c_int = 0 + 34;
pub const ROLE_SYSTEM_MENUITEM: c_int = 0 + 35;
pub const ROLE_SYSTEM_MENUPOPUP: c_int = 0 + 36;
pub const ROLE_SYSTEM_OUTLINE: c_int = 0 + 37;
pub const ROLE_SYSTEM_OUTLINEITEM: c_int = 0 + 38;
pub const ROLE_SYSTEM_PAGETAB: c_int = 0 + 39;
pub const ROLE_SYSTEM_PAGETABLIST: c_int = 0 + 40;
pub const ROLE_SYSTEM_PANE: c_int = 0 + 41;
pub const ROLE_SYSTEM_PROGRESSBAR: c_int = 0 + 42;
pub const ROLE_SYSTEM_PROPERTYPAGE: c_int = 0 + 43;
pub const ROLE_SYSTEM_PUSHBUTTON: c_int = 0 + 44;
pub const ROLE_SYSTEM_RADIOBUTTON: c_int = 0 + 45;
pub const ROLE_SYSTEM_ROW: c_int = 0 + 46;
pub const ROLE_SYSTEM_ROWHEADER: c_int = 0 + 47;
pub const ROLE_SYSTEM_SCROLLBAR: c_int = 0 + 48;
pub const ROLE_SYSTEM_SEPARATOR: c_int = 0 + 49;
pub const ROLE_SYSTEM_SLIDER: c_int = 0 + 50;
pub const ROLE_SYSTEM_SOUND: c_int = 0 + 51;
pub const ROLE_SYSTEM_SPINBUTTON: c_int = 0 + 52;
pub const ROLE_SYSTEM_STATICTEXT: c_int = 0 + 53;
pub const ROLE_SYSTEM_STATUSBAR: c_int = 0 + 54;
pub const ROLE_SYSTEM_TABLE: c_int = 0 + 55;
pub const ROLE_SYSTEM_TEXT: c_int = 0 + 56;
pub const ROLE_SYSTEM_TITLEBAR: c_int = 0 + 57;
pub const ROLE_SYSTEM_TOOLBAR: c_int = 0 + 58;
pub const ROLE_SYSTEM_TOOLTIP: c_int = 0 + 59;
pub const ROLE_SYSTEM_WHITESPACE: c_int = 0 + 60;
pub const ROLE_SYSTEM_WINDOW: c_int = 0 + 61;
//  ENUM: wxAccObject
pub const OBJID_WINDOW: c_long =    0x00000000;
pub const OBJID_SYSMENU: c_long =   0xFFFFFFFF;
pub const OBJID_TITLEBAR: c_long =  0xFFFFFFFE;
pub const OBJID_MENU: c_long =      0xFFFFFFFD;
pub const OBJID_CLIENT: c_long =    0xFFFFFFFC;
pub const OBJID_VSCROLL: c_long =   0xFFFFFFFB;
pub const OBJID_HSCROLL: c_long =   0xFFFFFFFA;
pub const OBJID_SIZEGRIP: c_long =  0xFFFFFFF9;
pub const OBJID_CARET: c_long =     0xFFFFFFF8;
pub const OBJID_CURSOR: c_long =    0xFFFFFFF7;
pub const OBJID_ALERT: c_long =     0xFFFFFFF6;
pub const OBJID_SOUND: c_long =     0xFFFFFFF5;
//  ENUM: wxAccSelectionFlags
pub const ACC_SEL_NONE: c_int = 0;
pub const ACC_SEL_TAKEFOCUS: c_int = 1;
pub const ACC_SEL_TAKESELECTION: c_int = 2;
pub const ACC_SEL_EXTENDSELECTION: c_int = 4;
pub const ACC_SEL_ADDSELECTION: c_int = 8;
pub const ACC_SEL_REMOVESELECTION: c_int = 16;

pub const AC_NO_AUTORESIZE: c_int = (0x0010);
pub const AC_DEFAULT_STYLE: c_long = (BORDER_NONE);
//  ENUM: wxAnimationType
pub const ANIMATION_TYPE_INVALID: c_int = 0;
pub const ANIMATION_TYPE_GIF: c_int = 0 + 1;
pub const ANIMATION_TYPE_ANI: c_int = 0 + 2;
pub const ANIMATION_TYPE_ANY: c_int = 0 + 3;

//  ENUM: wxAnimationDisposal
pub const ANIM_UNSPECIFIED: c_int = -1;
pub const ANIM_DONOTREMOVE: c_int = 0;
pub const ANIM_TOBACKGROUND: c_int = 1;
pub const ANIM_TOPREVIOUS: c_int = 2;

//  ENUM: @0
pub const WX_ANY_VALUE_BUFFER_SIZE: c_int = 16;

pub const BU_LEFT: c_int = 0x0040;
pub const BU_TOP: c_int = 0x0080;
pub const BU_RIGHT: c_int = 0x0100;
pub const BU_BOTTOM: c_int = 0x0200;
pub const BU_ALIGN_MASK: c_int = ( BU_LEFT | BU_TOP | BU_RIGHT | BU_BOTTOM );
pub const BU_EXACTFIT: c_int = 0x0001;
pub const BU_NOTEXT: c_int = 0x0002;
pub const BU_AUTODRAW: c_int = 0x0004;

//  ENUM: wxAuiToolBarStyle
pub const AUI_TB_TEXT: c_int = 1 << 0;
pub const AUI_TB_NO_TOOLTIPS: c_int = 1 << 1;
pub const AUI_TB_NO_AUTORESIZE: c_int = 1 << 2;
pub const AUI_TB_GRIPPER: c_int = 1 << 3;
pub const AUI_TB_OVERFLOW: c_int = 1 << 4;
pub const AUI_TB_VERTICAL: c_int = 1 << 5;
pub const AUI_TB_HORZ_LAYOUT: c_int = 1 << 6;
pub const AUI_TB_HORIZONTAL: c_int = 1 << 7;
pub const AUI_TB_PLAIN_BACKGROUND: c_int = 1 << 8;
pub const AUI_TB_HORZ_TEXT: c_int = (AUI_TB_HORZ_LAYOUT | AUI_TB_TEXT);
pub const AUI_ORIENTATION_MASK: c_int = (AUI_TB_VERTICAL | AUI_TB_HORIZONTAL);
pub const AUI_TB_DEFAULT_STYLE: c_int = 0;
//  ENUM: wxAuiToolBarArtSetting
pub const AUI_TBART_SEPARATOR_SIZE: c_int = 0;
pub const AUI_TBART_GRIPPER_SIZE: c_int = 1;
//  SKIP: wxAUI_TBART_OVERFLOW_SIZE
//  ENUM: wxAuiToolBarToolTextOrientation
pub const AUI_TBTOOL_TEXT_LEFT: c_int = 0;
pub const AUI_TBTOOL_TEXT_RIGHT: c_int = 1;
pub const AUI_TBTOOL_TEXT_TOP: c_int = 2;
pub const AUI_TBTOOL_TEXT_BOTTOM: c_int = 3;

//  ENUM: wxAuiNotebookOption
pub const AUI_NB_TOP: c_int = 1 << 0;
pub const AUI_NB_LEFT: c_int = 1 << 1;
pub const AUI_NB_RIGHT: c_int = 1 << 2;
pub const AUI_NB_BOTTOM: c_int = 1 << 3;
pub const AUI_NB_TAB_SPLIT: c_int = 1 << 4;
pub const AUI_NB_TAB_MOVE: c_int = 1 << 5;
pub const AUI_NB_TAB_EXTERNAL_MOVE: c_int = 1 << 6;
pub const AUI_NB_TAB_FIXED_WIDTH: c_int = 1 << 7;
pub const AUI_NB_SCROLL_BUTTONS: c_int = 1 << 8;
pub const AUI_NB_WINDOWLIST_BUTTON: c_int = 1 << 9;
pub const AUI_NB_CLOSE_BUTTON: c_int = 1 << 10;
pub const AUI_NB_CLOSE_ON_ACTIVE_TAB: c_int = 1 << 11;
pub const AUI_NB_CLOSE_ON_ALL_TABS: c_int = 1 << 12;
pub const AUI_NB_MIDDLE_CLICK_CLOSE: c_int = 1 << 13;
pub const AUI_NB_DEFAULT_STYLE: c_int = AUI_NB_TOP |
                             AUI_NB_TAB_SPLIT |
                             AUI_NB_TAB_MOVE |
                             AUI_NB_SCROLL_BUTTONS |
                             AUI_NB_CLOSE_ON_ACTIVE_TAB |
                             AUI_NB_MIDDLE_CLICK_CLOSE;

//  ENUM: wxAuiPaneDockArtSetting
pub const AUI_DOCKART_SASH_SIZE: c_int = 0;
pub const AUI_DOCKART_CAPTION_SIZE: c_int = 1;
pub const AUI_DOCKART_GRIPPER_SIZE: c_int = 2;
pub const AUI_DOCKART_PANE_BORDER_SIZE: c_int = 3;
pub const AUI_DOCKART_PANE_BUTTON_SIZE: c_int = 4;
pub const AUI_DOCKART_BACKGROUND_COLOUR: c_int = 5;
pub const AUI_DOCKART_SASH_COLOUR: c_int = 6;
pub const AUI_DOCKART_ACTIVE_CAPTION_COLOUR: c_int = 7;
pub const AUI_DOCKART_ACTIVE_CAPTION_GRADIENT_COLOUR: c_int = 8;
pub const AUI_DOCKART_INACTIVE_CAPTION_COLOUR: c_int = 9;
pub const AUI_DOCKART_INACTIVE_CAPTION_GRADIENT_COLOUR: c_int = 10;
pub const AUI_DOCKART_ACTIVE_CAPTION_TEXT_COLOUR: c_int = 11;
pub const AUI_DOCKART_INACTIVE_CAPTION_TEXT_COLOUR: c_int = 12;
pub const AUI_DOCKART_BORDER_COLOUR: c_int = 13;
pub const AUI_DOCKART_GRIPPER_COLOUR: c_int = 14;
pub const AUI_DOCKART_CAPTION_FONT: c_int = 15;
pub const AUI_DOCKART_GRADIENT_TYPE: c_int = 16;
//  ENUM: wxAuiPaneDockArtGradients
pub const AUI_GRADIENT_NONE: c_int = 0;
pub const AUI_GRADIENT_VERTICAL: c_int = 1;
pub const AUI_GRADIENT_HORIZONTAL: c_int = 2;
//  ENUM: wxAuiPaneButtonState
pub const AUI_BUTTON_STATE_NORMAL: c_int = 0;
pub const AUI_BUTTON_STATE_HOVER: c_int = 1 << 1;
pub const AUI_BUTTON_STATE_PRESSED: c_int = 1 << 2;
pub const AUI_BUTTON_STATE_DISABLED: c_int = 1 << 3;
pub const AUI_BUTTON_STATE_HIDDEN: c_int = 1 << 4;
pub const AUI_BUTTON_STATE_CHECKED: c_int = 1 << 5;
//  ENUM: wxAuiButtonId
pub const AUI_BUTTON_CLOSE: c_int = 101;
pub const AUI_BUTTON_MAXIMIZE_RESTORE: c_int = 102;
pub const AUI_BUTTON_MINIMIZE: c_int = 103;
pub const AUI_BUTTON_PIN: c_int = 104;
pub const AUI_BUTTON_OPTIONS: c_int = 105;
pub const AUI_BUTTON_WINDOWLIST: c_int = 106;
pub const AUI_BUTTON_LEFT: c_int = 107;
pub const AUI_BUTTON_RIGHT: c_int = 108;
pub const AUI_BUTTON_UP: c_int = 109;
pub const AUI_BUTTON_DOWN: c_int = 110;
pub const AUI_BUTTON_CUSTOM1: c_int = 201;
pub const AUI_BUTTON_CUSTOM2: c_int = 202;
pub const AUI_BUTTON_CUSTOM3: c_int = 203;

//  ENUM: wxAuiManagerDock
pub const AUI_DOCK_NONE: c_int = 0;
pub const AUI_DOCK_TOP: c_int = 1;
pub const AUI_DOCK_RIGHT: c_int = 2;
pub const AUI_DOCK_BOTTOM: c_int = 3;
pub const AUI_DOCK_LEFT: c_int = 4;
pub const AUI_DOCK_CENTER: c_int = 5;
pub const AUI_DOCK_CENTRE: c_int = AUI_DOCK_CENTER;
//  ENUM: wxAuiManagerOption
pub const AUI_MGR_ALLOW_FLOATING: c_int = 1 << 0;
pub const AUI_MGR_ALLOW_ACTIVE_PANE: c_int = 1 << 1;
pub const AUI_MGR_TRANSPARENT_DRAG: c_int = 1 << 2;
pub const AUI_MGR_TRANSPARENT_HINT: c_int = 1 << 3;
pub const AUI_MGR_VENETIAN_BLINDS_HINT: c_int = 1 << 4;
pub const AUI_MGR_RECTANGLE_HINT: c_int = 1 << 5;
pub const AUI_MGR_HINT_FADE: c_int = 1 << 6;
pub const AUI_MGR_NO_VENETIAN_BLINDS_FADE: c_int = 1 << 7;
pub const AUI_MGR_LIVE_RESIZE: c_int = 1 << 8;
pub const AUI_MGR_DEFAULT: c_int = AUI_MGR_ALLOW_FLOATING |
                        AUI_MGR_TRANSPARENT_HINT |
                        AUI_MGR_HINT_FADE |
                        AUI_MGR_NO_VENETIAN_BLINDS_FADE;

//  ENUM: wxBase64DecodeMode
pub const Base64DecodeMode_Strict: c_int = 0;
pub const Base64DecodeMode_SkipWS: c_int = 0 + 1;
pub const Base64DecodeMode_Relaxed: c_int = 0 + 2;

//  ENUM: wxBrushStyle
pub const BRUSHSTYLE_INVALID: c_int = -1;
pub const BRUSHSTYLE_SOLID: c_int = SOLID;
pub const BRUSHSTYLE_TRANSPARENT: c_int = TRANSPARENT;
pub const BRUSHSTYLE_STIPPLE_MASK_OPAQUE: c_int = STIPPLE_MASK_OPAQUE;
pub const BRUSHSTYLE_STIPPLE_MASK: c_int = STIPPLE_MASK;
pub const BRUSHSTYLE_STIPPLE: c_int = STIPPLE;
pub const BRUSHSTYLE_BDIAGONAL_HATCH: c_int = STIPPLE + 1;
pub const BRUSHSTYLE_CROSSDIAG_HATCH: c_int = STIPPLE + 2;
pub const BRUSHSTYLE_FDIAGONAL_HATCH: c_int = STIPPLE + 3;
pub const BRUSHSTYLE_CROSS_HATCH: c_int = STIPPLE + 4;
pub const BRUSHSTYLE_HORIZONTAL_HATCH: c_int = STIPPLE + 5;
pub const BRUSHSTYLE_VERTICAL_HATCH: c_int = STIPPLE + 6;
pub const BRUSHSTYLE_FIRST_HATCH: c_int = STIPPLE + 7;
pub const BRUSHSTYLE_LAST_HATCH: c_int = STIPPLE + 8;

//  ENUM: @4
pub const CAL_SUNDAY_FIRST: c_int = 0x0080;
pub const CAL_MONDAY_FIRST: c_int = 0x0001;
pub const CAL_SHOW_HOLIDAYS: c_int = 0x0002;
pub const CAL_NO_YEAR_CHANGE: c_int = 0x0004;
pub const CAL_NO_MONTH_CHANGE: c_int = 0x000c;
pub const CAL_SEQUENTIAL_MONTH_SELECTION: c_int = 0x0010;
pub const CAL_SHOW_SURROUNDING_WEEKS: c_int = 0x0020;
pub const CAL_SHOW_WEEK_NUMBERS: c_int = 0x0040;
//  ENUM: wxCalendarDateBorder
pub const CAL_BORDER_NONE: c_int = 0;
pub const CAL_BORDER_SQUARE: c_int = 0 + 1;
pub const CAL_BORDER_ROUND: c_int = 0 + 2;
//  ENUM: wxCalendarHitTestResult
pub const CAL_HITTEST_NOWHERE: c_int = 0;
pub const CAL_HITTEST_HEADER: c_int = 0 + 1;
pub const CAL_HITTEST_DAY: c_int = 0 + 2;
pub const CAL_HITTEST_INCMONTH: c_int = 0 + 3;
pub const CAL_HITTEST_DECMONTH: c_int = 0 + 4;
pub const CAL_HITTEST_SURROUNDING_WEEK: c_int = 0 + 5;
pub const CAL_HITTEST_WEEK: c_int = 0 + 6;

// NODEF: wxT
// NODEF: wxT_2
// NODEF: wxS
// NODEF: _T

pub const CHK_2STATE: c_int = 0x4000;
pub const CHK_3STATE: c_int = 0x1000;
pub const CHK_ALLOW_3RD_STATE_FOR_USER: c_int = 0x2000;
//  ENUM: wxCheckBoxState
pub const CHK_UNCHECKED: c_int = 0;
pub const CHK_CHECKED: c_int = 0 + 1;
pub const CHK_UNDETERMINED: c_int = 0 + 2;

pub const CHOICE_WIDTH: c_int = 150;
pub const CHOICE_HEIGHT: c_int = 200;
pub const CHOICEDLG_STYLE: c_long = (DEFAULT_DIALOG_STYLE | OK | CANCEL | CENTRE | RESIZE_BORDER);

pub const CHB_DEFAULT: c_int = BK_DEFAULT;
pub const CHB_TOP: c_int = BK_TOP;
pub const CHB_BOTTOM: c_int = BK_BOTTOM;
pub const CHB_LEFT: c_int = BK_LEFT;
pub const CHB_RIGHT: c_int = BK_RIGHT;
pub const CHB_ALIGN_MASK: c_int = BK_ALIGN_MASK;

// NODEF: wxTheClipboard

pub const CLRP_USE_TEXTCTRL: c_int = (PB_USE_TEXTCTRL);
pub const CLRP_DEFAULT_STYLE: c_int = 0;
pub const CLRP_SHOW_LABEL: c_int = 0x0008;
pub const CLRP_SHOW_ALPHA: c_int = 0x0010;

//  ENUM: wxCmdLineEntryFlags
pub const CMD_LINE_OPTION_MANDATORY: c_int = 0x01;
pub const CMD_LINE_PARAM_OPTIONAL: c_int = 0x02;
pub const CMD_LINE_PARAM_MULTIPLE: c_int = 0x04;
pub const CMD_LINE_OPTION_HELP: c_int = 0x08;
pub const CMD_LINE_NEEDS_SEPARATOR: c_int = 0x10;
pub const CMD_LINE_SWITCH_NEGATABLE: c_int = 0x20;
pub const CMD_LINE_HIDDEN: c_int = 0x40;
//  ENUM: wxCmdLineParamType
pub const CMD_LINE_VAL_STRING: c_int = 0;
pub const CMD_LINE_VAL_NUMBER: c_int = 0 + 1;
pub const CMD_LINE_VAL_DATE: c_int = 0 + 2;
pub const CMD_LINE_VAL_DOUBLE: c_int = 0 + 3;
pub const CMD_LINE_VAL_NONE: c_int = 0 + 4;
//  ENUM: wxCmdLineEntryType
pub const CMD_LINE_SWITCH: c_int = 0;
pub const CMD_LINE_OPTION: c_int = 0 + 1;
pub const CMD_LINE_PARAM: c_int = 0 + 2;
pub const CMD_LINE_USAGE_TEXT: c_int = 0 + 3;
pub const CMD_LINE_NONE: c_int = 0 + 4;
//  ENUM: wxCmdLineSwitchState
pub const CMD_SWITCH_OFF: c_int = 0;
pub const CMD_SWITCH_ON: c_int = 0 + 1;
//  ENUM: wxCmdLineSplitType
pub const CMD_LINE_SPLIT_DOS: c_int = 0;
pub const CMD_LINE_SPLIT_UNIX: c_int = 0 + 1;

//  ENUM: wxPrintBin
pub const PRINTBIN_DEFAULT: c_int = 0;
pub const PRINTBIN_ONLYONE: c_int = 0 + 1;
pub const PRINTBIN_LOWER: c_int = 0 + 2;
pub const PRINTBIN_MIDDLE: c_int = 0 + 3;
pub const PRINTBIN_MANUAL: c_int = 0 + 4;
pub const PRINTBIN_ENVELOPE: c_int = 0 + 5;
pub const PRINTBIN_ENVMANUAL: c_int = 0 + 6;
pub const PRINTBIN_AUTO: c_int = 0 + 7;
pub const PRINTBIN_TRACTOR: c_int = 0 + 8;
pub const PRINTBIN_SMALLFMT: c_int = 0 + 9;
pub const PRINTBIN_LARGEFMT: c_int = 0 + 10;
pub const PRINTBIN_LARGECAPACITY: c_int = 0 + 11;
pub const PRINTBIN_CASSETTE: c_int = 0 + 12;
pub const PRINTBIN_FORMSOURCE: c_int = 0 + 13;
pub const PRINTBIN_USER: c_int = 0 + 14;

pub const CP_DEFAULT_STYLE: c_long = (TAB_TRAVERSAL | NO_BORDER);
pub const CP_NO_TLW_RESIZE: c_int = (0x0002);

//  ENUM: @5
pub const C2S_NAME: c_int = 1;
pub const C2S_CSS_SYNTAX: c_int = 2;
pub const C2S_HTML_SYNTAX: c_int = 4;

//  ENUM: @7
pub const CC_SPECIAL_DCLICK: c_int = 0x0100;
pub const CC_STD_BUTTON: c_int = 0x0200;

//  ENUM: wxBOM
pub const BOM_Unknown: c_int = -1;
pub const BOM_None: c_int = -1 + 1;
pub const BOM_UTF32BE: c_int = -1 + 2;
pub const BOM_UTF32LE: c_int = -1 + 3;
pub const BOM_UTF16BE: c_int = -1 + 4;
pub const BOM_UTF16LE: c_int = -1 + 5;
pub const BOM_UTF8: c_int = -1 + 6;

// NODEF: wxCONCAT
// NODEF: wxCONCAT3
// NODEF: wxCONCAT4
// NODEF: wxCONCAT5
// NODEF: wxSTRINGIZE
// NODEF: wxSTRINGIZE_T
// NODEF: __WXFUNCTION__

pub const DVC_DEFAULT_RENDERER_SIZE: c_int = 20;
pub const DVC_DEFAULT_WIDTH: c_int = 80;
pub const DVC_TOGGLE_DEFAULT_WIDTH: c_int = 30;
pub const DVC_DEFAULT_MINWIDTH: c_int = 30;
pub const DVR_DEFAULT_ALIGNMENT: c_int = -1;
pub const DV_SINGLE: c_int = 0x0000;
pub const DV_MULTIPLE: c_int = 0x0001;
pub const DV_NO_HEADER: c_int = 0x0002;
pub const DV_HORIZ_RULES: c_int = 0x0004;
pub const DV_VERT_RULES: c_int = 0x0008;
pub const DV_ROW_LINES: c_int = 0x0010;
pub const DV_VARIABLE_LINE_HEIGHT: c_int = 0x0020;
//  ENUM: wxDataViewCellMode
pub const DATAVIEW_CELL_INERT: c_int = 0;
pub const DATAVIEW_CELL_ACTIVATABLE: c_int = 0 + 1;
pub const DATAVIEW_CELL_EDITABLE: c_int = 0 + 2;
//  ENUM: wxDataViewCellRenderState
pub const DATAVIEW_CELL_SELECTED: c_int = 1;
pub const DATAVIEW_CELL_PRELIT: c_int = 2;
pub const DATAVIEW_CELL_INSENSITIVE: c_int = 4;
pub const DATAVIEW_CELL_FOCUSED: c_int = 8;
//  ENUM: wxDataViewColumnFlags
pub const DATAVIEW_COL_RESIZABLE: c_int = 1;
pub const DATAVIEW_COL_SORTABLE: c_int = 2;
pub const DATAVIEW_COL_REORDERABLE: c_int = 4;
pub const DATAVIEW_COL_HIDDEN: c_int = 8;

//  ENUM: @11
pub const DP_DEFAULT: c_int = 0;
pub const DP_SPIN: c_int = 1;
pub const DP_DROPDOWN: c_int = 2;
pub const DP_SHOWCENTURY: c_int = 4;
pub const DP_ALLOWNONE: c_int = 8;

pub const BUFFER_VIRTUAL_AREA: c_int = 0x01;
pub const BUFFER_CLIENT_AREA: c_int = 0x02;
pub const BUFFER_USES_SHARED_BUFFER: c_int = 0x04;

//  ENUM: wxSVGShapeRenderingMode
pub const SVG_SHAPE_RENDERING_AUTO: c_int = 0;
pub const SVG_SHAPE_RENDERING_OPTIMIZE_SPEED: c_int = 0 + 1;
pub const SVG_SHAPE_RENDERING_CRISP_EDGES: c_int = 0 + 2;
pub const SVG_SHAPE_RENDERING_GEOMETRIC_PRECISION: c_int = 0 + 3;
pub const SVG_SHAPE_RENDERING_OPTIMISE_SPEED: c_int = SVG_SHAPE_RENDERING_OPTIMIZE_SPEED;

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

pub const SIZE_AUTO_WIDTH: c_int = 0x0001;
pub const SIZE_AUTO_HEIGHT: c_int = 0x0002;
pub const SIZE_AUTO: c_int = (SIZE_AUTO_WIDTH|SIZE_AUTO_HEIGHT);
pub const SIZE_USE_EXISTING: c_int = 0x0000;
pub const SIZE_ALLOW_MINUS_ONE: c_int = 0x0004;
pub const SIZE_NO_ADJUSTMENTS: c_int = 0x0008;
pub const SIZE_FORCE: c_int = 0x0010;
pub const SIZE_FORCE_EVENT: c_int = 0x0020;
pub const VSCROLL: c_long = 0x80000000;
pub const HSCROLL: c_long = 0x40000000;
pub const CAPTION: c_long = 0x20000000;
pub const DOUBLE_BORDER: c_long = BORDER_DOUBLE;
pub const SUNKEN_BORDER: c_long = BORDER_SUNKEN;
pub const RAISED_BORDER: c_long = BORDER_RAISED;
pub const BORDER: c_long = BORDER_SIMPLE;
pub const SIMPLE_BORDER: c_long = BORDER_SIMPLE;
pub const STATIC_BORDER: c_long = BORDER_STATIC;
pub const NO_BORDER: c_long = BORDER_NONE;
pub const ALWAYS_SHOW_SB: c_long = 0x00800000;
pub const CLIP_CHILDREN: c_long = 0x00400000;
pub const CLIP_SIBLINGS: c_long = 0x20000000;
pub const TRANSPARENT_WINDOW: c_long = 0x00100000;
pub const TAB_TRAVERSAL: c_long = 0x00080000;
pub const WANTS_CHARS: c_long = 0x00040000;
pub const RETAINED: c_long = 0x00000000;
pub const BACKINGSTORE: c_long = RETAINED;
pub const POPUP_WINDOW: c_long = 0x00020000;
pub const FULL_REPAINT_ON_RESIZE: c_long = 0x00010000;
pub const NO_FULL_REPAINT_ON_RESIZE: c_int = 0;
pub const WINDOW_STYLE_MASK: c_long = (VSCROLL|HSCROLL|BORDER_MASK|ALWAYS_SHOW_SB|CLIP_CHILDREN| CLIP_SIBLINGS|TRANSPARENT_WINDOW|TAB_TRAVERSAL|WANTS_CHARS| RETAINED|POPUP_WINDOW|FULL_REPAINT_ON_RESIZE);
pub const WS_EX_BLOCK_EVENTS: c_int = 0x00000002;
pub const WS_EX_TRANSIENT: c_int = 0x00000004;
pub const WS_EX_THEMED_BACKGROUND: c_int = 0x00000008;
pub const WS_EX_PROCESS_IDLE: c_int = 0x00000010;
pub const WS_EX_PROCESS_UI_UPDATES: c_int = 0x00000020;
pub const FRAME_EX_METAL: c_int = 0x00000040;
pub const DIALOG_EX_METAL: c_int = 0x00000040;
pub const WS_EX_CONTEXTHELP: c_int = 0x00000080;
pub const FRAME_EX_CONTEXTHELP: c_int = WS_EX_CONTEXTHELP;
pub const DIALOG_EX_CONTEXTHELP: c_int = WS_EX_CONTEXTHELP;
pub const FRAME_DRAWER: c_int = 0x0020;
pub const FRAME_NO_WINDOW_MENU: c_int = 0x0100;
pub const MB_DOCKABLE: c_int = 0x0001;
pub const MENU_TEAROFF: c_int = 0x0001;
pub const COLOURED: c_int = 0x0800;
pub const FIXED_LENGTH: c_int = 0x0400;
pub const LB_SORT: c_int = 0x0010;
pub const LB_SINGLE: c_int = 0x0020;
pub const LB_MULTIPLE: c_int = 0x0040;
pub const LB_EXTENDED: c_int = 0x0080;
pub const LB_NEEDED_SB: c_int = 0x0000;
pub const LB_OWNERDRAW: c_int = 0x0100;
pub const LB_ALWAYS_SB: c_int = 0x0200;
pub const LB_NO_SB: c_int = 0x0400;
pub const LB_HSCROLL: c_long = HSCROLL;
pub const LB_INT_HEIGHT: c_int = 0x0800;
pub const CB_SIMPLE: c_int = 0x0004;
pub const CB_SORT: c_int = 0x0008;
pub const CB_READONLY: c_int = 0x0010;
pub const CB_DROPDOWN: c_int = 0x0020;
pub const RA_LEFTTORIGHT: c_int = 0x0001;
pub const RA_TOPTOBOTTOM: c_int = 0x0002;
pub const RA_SPECIFY_COLS: c_int = HORIZONTAL;
pub const RA_SPECIFY_ROWS: c_int = VERTICAL;
pub const RA_HORIZONTAL: c_int = HORIZONTAL;
pub const RA_VERTICAL: c_int = VERTICAL;
pub const RB_GROUP: c_int = 0x0004;
pub const RB_SINGLE: c_int = 0x0008;
pub const SB_HORIZONTAL: c_int = HORIZONTAL;
pub const SB_VERTICAL: c_int = VERTICAL;
pub const SP_HORIZONTAL: c_int = HORIZONTAL /*  4 */;
pub const SP_VERTICAL: c_int = VERTICAL   /*  8 */;
pub const SP_ARROW_KEYS: c_int = 0x4000;
pub const SP_WRAP: c_int = 0x8000;
pub const TC_RIGHTJUSTIFY: c_int = 0x0010;
pub const TC_FIXEDWIDTH: c_int = 0x0020;
pub const TC_TOP: c_int = 0x0000    /*  default */;
pub const TC_LEFT: c_int = 0x0020;
pub const TC_RIGHT: c_int = 0x0040;
pub const TC_BOTTOM: c_int = 0x0080;
pub const TC_MULTILINE: c_int = 0x0200    /* == NB_MULTILINE */;
pub const TC_OWNERDRAW: c_int = 0x0400;
pub const BI_EXPAND: c_int = EXPAND;
pub const LI_HORIZONTAL: c_int = HORIZONTAL;
pub const LI_VERTICAL: c_int = VERTICAL;
pub const YES: c_int = 0x00000002;
pub const OK: c_long = 0x00000004;
pub const NO: c_int = 0x00000008;
pub const YES_NO: c_int = (YES | NO);
pub const CANCEL: c_long = 0x00000010;
pub const APPLY: c_int = 0x00000020;
pub const CLOSE: c_int = 0x00000040;
pub const OK_DEFAULT: c_int = 0x00000000  /* has no effect (default) */;
pub const YES_DEFAULT: c_int = 0x00000000  /* has no effect (default) */;
pub const NO_DEFAULT: c_int = 0x00000080  /* only valid with YES_NO */;
pub const CANCEL_DEFAULT: c_long = 0x80000000  /* only valid with CANCEL */;
pub const ICON_EXCLAMATION: c_int = 0x00000100;
pub const ICON_HAND: c_int = 0x00000200;
pub const ICON_WARNING: c_int = ICON_EXCLAMATION;
pub const ICON_ERROR: c_int = ICON_HAND;
pub const ICON_QUESTION: c_int = 0x00000400;
pub const ICON_INFORMATION: c_int = 0x00000800;
pub const ICON_STOP: c_int = ICON_HAND;
pub const ICON_ASTERISK: c_int = ICON_INFORMATION;
pub const HELP: c_int = 0x00001000;
pub const FORWARD: c_int = 0x00002000;
pub const BACKWARD: c_int = 0x00004000;
pub const RESET: c_int = 0x00008000;
pub const MORE: c_int = 0x00010000;
pub const SETUP: c_int = 0x00020000;
pub const ICON_NONE: c_int = 0x00040000;
pub const ICON_AUTH_NEEDED: c_int = 0x00080000;
pub const ICON_MASK: c_int = (ICON_EXCLAMATION|ICON_HAND|ICON_QUESTION|ICON_INFORMATION|ICON_NONE);
pub const NOT_FOUND: c_int = (-1);
pub const PRINT_QUALITY_HIGH: c_int = -1;
pub const PRINT_QUALITY_MEDIUM: c_int = -2;
pub const PRINT_QUALITY_LOW: c_int = -3;
pub const PRINT_QUALITY_DRAFT: c_int = -4;
pub const STAY_ON_TOP: c_int = 0x8000;
pub const ICONIZE: c_int = 0x4000;
pub const MINIMIZE: c_int = ICONIZE;
pub const MAXIMIZE: c_int = 0x2000;
pub const CLOSE_BOX: c_long = 0x1000;
pub const SYSTEM_MENU: c_long = 0x0800;
pub const MINIMIZE_BOX: c_long = 0x0400;
pub const MAXIMIZE_BOX: c_long = 0x0200;
pub const TINY_CAPTION: c_int = 0x0080;
pub const RESIZE_BORDER: c_long = 0x0040;
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
pub const CENTRE: c_long = 0x0001;
pub const CENTER: c_long = CENTRE;
//  ENUM: wxOrientation
pub const HORIZONTAL: c_int = 0x0004;
pub const VERTICAL: c_int = 0x0008;
pub const BOTH: c_int = VERTICAL | HORIZONTAL;
pub const ORIENTATION_MASK: c_int = BOTH;
//  ENUM: wxDirection
pub const LEFT: c_int = 0x0010;
pub const RIGHT: c_int = 0x0020;
pub const UP: c_int = 0x0040;
pub const DOWN: c_int = 0x0080;
pub const TOP: c_int = UP;
pub const BOTTOM: c_int = DOWN;
pub const NORTH: c_int = UP;
pub const SOUTH: c_int = DOWN;
//  SKIP: wxWEST
pub const EAST: c_int = RIGHT;
pub const ALL: c_int = (UP | DOWN | RIGHT | LEFT);
pub const DIRECTION_MASK: c_int = ALL;
//  ENUM: wxAlignment
pub const ALIGN_INVALID: c_int = -1;
pub const ALIGN_NOT: c_int = 0x0000;
pub const ALIGN_CENTER_HORIZONTAL: c_int = 0x0100;
pub const ALIGN_CENTRE_HORIZONTAL: c_int = ALIGN_CENTER_HORIZONTAL;
pub const ALIGN_LEFT: c_int = ALIGN_NOT;
pub const ALIGN_TOP: c_int = ALIGN_NOT;
pub const ALIGN_RIGHT: c_int = 0x0200;
pub const ALIGN_BOTTOM: c_int = 0x0400;
pub const ALIGN_CENTER_VERTICAL: c_int = 0x0800;
pub const ALIGN_CENTRE_VERTICAL: c_int = ALIGN_CENTER_VERTICAL;
pub const ALIGN_CENTER: c_int = (ALIGN_CENTER_HORIZONTAL | ALIGN_CENTER_VERTICAL);
pub const ALIGN_CENTRE: c_int = ALIGN_CENTER;
pub const ALIGN_MASK: c_int = 0x0f00;
//  ENUM: wxSizerFlagBits
pub const FIXED_MINSIZE: c_int = 0x8000;
pub const RESERVE_SPACE_EVEN_IF_HIDDEN: c_int = 0x0002;
pub const SIZER_FLAG_BITS_MASK: c_int = 0x8002;
//  ENUM: wxStretch
pub const STRETCH_NOT: c_int = 0x0000;
pub const SHRINK: c_int = 0x1000;
pub const GROW: c_int = 0x2000;
pub const EXPAND: c_int = GROW;
pub const SHAPED: c_int = 0x4000;
pub const TILE: c_int = SHAPED | FIXED_MINSIZE;
pub const STRETCH_MASK: c_int = 0x7000;
//  ENUM: wxBorder
pub const BORDER_DEFAULT: c_long = 0;
pub const BORDER_NONE: c_long = 0x00200000;
pub const BORDER_STATIC: c_long = 0x01000000;
pub const BORDER_SIMPLE: c_long = 0x02000000;
pub const BORDER_RAISED: c_long = 0x04000000;
pub const BORDER_SUNKEN: c_long = 0x08000000;
pub const BORDER_DOUBLE: c_long = 0x10000000;
pub const BORDER_THEME: c_long = BORDER_DOUBLE;
pub const BORDER_MASK: c_long = 0x1f200000;
//  ENUM: wxBackgroundStyle
pub const BG_STYLE_ERASE: c_int = 0;
pub const BG_STYLE_SYSTEM: c_int = 0 + 1;
pub const BG_STYLE_PAINT: c_int = 0 + 2;
pub const BG_STYLE_COLOUR: c_int = 0 + 3;
pub const BG_STYLE_TRANSPARENT: c_int = 0 + 4;
//  ENUM: wxStandardID
pub const ID_AUTO_LOWEST: c_int = 0;
pub const ID_AUTO_HIGHEST: c_int = 0 + 1;
pub const ID_NONE: c_int = -3;
pub const ID_SEPARATOR: c_int = -2;
pub const ID_ANY: c_int = -1;
pub const ID_LOWEST: c_int = 4999;
pub const ID_OPEN: c_int = 4999 + 1;
pub const ID_CLOSE: c_int = 4999 + 2;
pub const ID_NEW: c_int = 4999 + 3;
pub const ID_SAVE: c_int = 4999 + 4;
pub const ID_SAVEAS: c_int = 4999 + 5;
pub const ID_REVERT: c_int = 4999 + 6;
pub const ID_EXIT: c_int = 4999 + 7;
pub const ID_UNDO: c_int = 4999 + 8;
pub const ID_REDO: c_int = 4999 + 9;
pub const ID_HELP: c_int = 4999 + 10;
pub const ID_PRINT: c_int = 4999 + 11;
pub const ID_PRINT_SETUP: c_int = 4999 + 12;
pub const ID_PAGE_SETUP: c_int = 4999 + 13;
pub const ID_PREVIEW: c_int = 4999 + 14;
pub const ID_ABOUT: c_int = 4999 + 15;
pub const ID_HELP_CONTENTS: c_int = 4999 + 16;
pub const ID_HELP_INDEX: c_int = 4999 + 17;
pub const ID_HELP_SEARCH: c_int = 4999 + 18;
pub const ID_HELP_COMMANDS: c_int = 4999 + 19;
pub const ID_HELP_PROCEDURES: c_int = 4999 + 20;
pub const ID_HELP_CONTEXT: c_int = 4999 + 21;
pub const ID_CLOSE_ALL: c_int = 4999 + 22;
pub const ID_PREFERENCES: c_int = 4999 + 23;
pub const ID_EDIT: c_int = 5030;
pub const ID_CUT: c_int = 5030 + 1;
pub const ID_COPY: c_int = 5030 + 2;
pub const ID_PASTE: c_int = 5030 + 3;
pub const ID_CLEAR: c_int = 5030 + 4;
pub const ID_FIND: c_int = 5030 + 5;
pub const ID_DUPLICATE: c_int = 5030 + 6;
pub const ID_SELECTALL: c_int = 5030 + 7;
pub const ID_DELETE: c_int = 5030 + 8;
pub const ID_REPLACE: c_int = 5030 + 9;
pub const ID_REPLACE_ALL: c_int = 5030 + 10;
pub const ID_PROPERTIES: c_int = 5030 + 11;
pub const ID_VIEW_DETAILS: c_int = 5030 + 12;
pub const ID_VIEW_LARGEICONS: c_int = 5030 + 13;
pub const ID_VIEW_SMALLICONS: c_int = 5030 + 14;
pub const ID_VIEW_LIST: c_int = 5030 + 15;
pub const ID_VIEW_SORTDATE: c_int = 5030 + 16;
pub const ID_VIEW_SORTNAME: c_int = 5030 + 17;
pub const ID_VIEW_SORTSIZE: c_int = 5030 + 18;
pub const ID_VIEW_SORTTYPE: c_int = 5030 + 19;
pub const ID_FILE: c_int = 5050;
pub const ID_FILE1: c_int = 5050 + 1;
pub const ID_FILE2: c_int = 5050 + 2;
pub const ID_FILE3: c_int = 5050 + 3;
pub const ID_FILE4: c_int = 5050 + 4;
pub const ID_FILE5: c_int = 5050 + 5;
pub const ID_FILE6: c_int = 5050 + 6;
pub const ID_FILE7: c_int = 5050 + 7;
pub const ID_FILE8: c_int = 5050 + 8;
pub const ID_FILE9: c_int = 5050 + 9;
pub const ID_OK: c_int = 5100;
pub const ID_CANCEL: c_int = 5100 + 1;
pub const ID_APPLY: c_int = 5100 + 2;
pub const ID_YES: c_int = 5100 + 3;
pub const ID_NO: c_int = 5100 + 4;
pub const ID_STATIC: c_int = 5100 + 5;
pub const ID_FORWARD: c_int = 5100 + 6;
pub const ID_BACKWARD: c_int = 5100 + 7;
pub const ID_DEFAULT: c_int = 5100 + 8;
pub const ID_MORE: c_int = 5100 + 9;
pub const ID_SETUP: c_int = 5100 + 10;
pub const ID_RESET: c_int = 5100 + 11;
pub const ID_CONTEXT_HELP: c_int = 5100 + 12;
pub const ID_YESTOALL: c_int = 5100 + 13;
pub const ID_NOTOALL: c_int = 5100 + 14;
pub const ID_ABORT: c_int = 5100 + 15;
pub const ID_RETRY: c_int = 5100 + 16;
pub const ID_IGNORE: c_int = 5100 + 17;
pub const ID_ADD: c_int = 5100 + 18;
pub const ID_REMOVE: c_int = 5100 + 19;
pub const ID_UP: c_int = 5100 + 20;
pub const ID_DOWN: c_int = 5100 + 21;
pub const ID_HOME: c_int = 5100 + 22;
pub const ID_REFRESH: c_int = 5100 + 23;
pub const ID_STOP: c_int = 5100 + 24;
pub const ID_INDEX: c_int = 5100 + 25;
pub const ID_BOLD: c_int = 5100 + 26;
pub const ID_ITALIC: c_int = 5100 + 27;
pub const ID_JUSTIFY_CENTER: c_int = 5100 + 28;
pub const ID_JUSTIFY_FILL: c_int = 5100 + 29;
pub const ID_JUSTIFY_RIGHT: c_int = 5100 + 30;
pub const ID_JUSTIFY_LEFT: c_int = 5100 + 31;
pub const ID_UNDERLINE: c_int = 5100 + 32;
pub const ID_INDENT: c_int = 5100 + 33;
pub const ID_UNINDENT: c_int = 5100 + 34;
pub const ID_ZOOM_100: c_int = 5100 + 35;
pub const ID_ZOOM_FIT: c_int = 5100 + 36;
pub const ID_ZOOM_IN: c_int = 5100 + 37;
pub const ID_ZOOM_OUT: c_int = 5100 + 38;
pub const ID_UNDELETE: c_int = 5100 + 39;
pub const ID_REVERT_TO_SAVED: c_int = 5100 + 40;
pub const ID_CDROM: c_int = 5100 + 41;
pub const ID_CONVERT: c_int = 5100 + 42;
pub const ID_EXECUTE: c_int = 5100 + 43;
pub const ID_FLOPPY: c_int = 5100 + 44;
pub const ID_HARDDISK: c_int = 5100 + 45;
pub const ID_BOTTOM: c_int = 5100 + 46;
pub const ID_FIRST: c_int = 5100 + 47;
pub const ID_LAST: c_int = 5100 + 48;
pub const ID_TOP: c_int = 5100 + 49;
pub const ID_INFO: c_int = 5100 + 50;
pub const ID_JUMP_TO: c_int = 5100 + 51;
pub const ID_NETWORK: c_int = 5100 + 52;
pub const ID_SELECT_COLOR: c_int = 5100 + 53;
pub const ID_SELECT_FONT: c_int = 5100 + 54;
pub const ID_SORT_ASCENDING: c_int = 5100 + 55;
pub const ID_SORT_DESCENDING: c_int = 5100 + 56;
pub const ID_SPELL_CHECK: c_int = 5100 + 57;
pub const ID_STRIKETHROUGH: c_int = 5100 + 58;
pub const ID_SYSTEM_MENU: c_int = 5200;
pub const ID_CLOSE_FRAME: c_int = 5200 + 1;
pub const ID_MOVE_FRAME: c_int = 5200 + 2;
pub const ID_RESIZE_FRAME: c_int = 5200 + 3;
pub const ID_MAXIMIZE_FRAME: c_int = 5200 + 4;
pub const ID_ICONIZE_FRAME: c_int = 5200 + 5;
pub const ID_RESTORE_FRAME: c_int = 5200 + 6;
pub const ID_MDI_WINDOW_FIRST: c_int = 5230;
pub const ID_MDI_WINDOW_CASCADE: c_int = ID_MDI_WINDOW_FIRST;
pub const ID_MDI_WINDOW_TILE_HORZ: c_int = ID_MDI_WINDOW_FIRST + 1;
pub const ID_MDI_WINDOW_TILE_VERT: c_int = ID_MDI_WINDOW_FIRST + 2;
pub const ID_MDI_WINDOW_ARRANGE_ICONS: c_int = ID_MDI_WINDOW_FIRST + 3;
pub const ID_MDI_WINDOW_PREV: c_int = ID_MDI_WINDOW_FIRST + 4;
pub const ID_MDI_WINDOW_NEXT: c_int = ID_MDI_WINDOW_FIRST + 5;
pub const ID_MDI_WINDOW_LAST: c_int = ID_MDI_WINDOW_NEXT;
pub const ID_FILEDLGG: c_int = 5900;
pub const ID_FILECTRL: c_int = 5950;
pub const ID_HIGHEST: c_int = 5999;
//  ENUM: wxItemKind
pub const ITEM_SEPARATOR: c_int = -1;
pub const ITEM_NORMAL: c_int = -1 + 1;
pub const ITEM_CHECK: c_int = -1 + 2;
pub const ITEM_RADIO: c_int = -1 + 3;
pub const ITEM_DROPDOWN: c_int = -1 + 4;
pub const ITEM_MAX: c_int = -1 + 5;
//  ENUM: wxHitTest
pub const HT_NOWHERE: c_int = 0;
pub const HT_SCROLLBAR_FIRST: c_int = HT_NOWHERE;
pub const HT_SCROLLBAR_ARROW_LINE_1: c_int = HT_NOWHERE + 1;
pub const HT_SCROLLBAR_ARROW_LINE_2: c_int = HT_NOWHERE + 2;
pub const HT_SCROLLBAR_ARROW_PAGE_1: c_int = HT_NOWHERE + 3;
pub const HT_SCROLLBAR_ARROW_PAGE_2: c_int = HT_NOWHERE + 4;
pub const HT_SCROLLBAR_THUMB: c_int = HT_NOWHERE + 5;
pub const HT_SCROLLBAR_BAR_1: c_int = HT_NOWHERE + 6;
pub const HT_SCROLLBAR_BAR_2: c_int = HT_NOWHERE + 7;
pub const HT_SCROLLBAR_LAST: c_int = HT_NOWHERE + 8;
pub const HT_WINDOW_OUTSIDE: c_int = HT_NOWHERE + 9;
pub const HT_WINDOW_INSIDE: c_int = HT_NOWHERE + 10;
pub const HT_WINDOW_VERT_SCROLLBAR: c_int = HT_NOWHERE + 11;
pub const HT_WINDOW_HORZ_SCROLLBAR: c_int = HT_NOWHERE + 12;
pub const HT_WINDOW_CORNER: c_int = HT_NOWHERE + 13;
pub const HT_MAX: c_int = HT_NOWHERE + 14;
//  ENUM: wxDataFormatId
pub const DF_INVALID: c_int =          0;
pub const DF_TEXT: c_int =             1;
pub const DF_BITMAP: c_int =           2;
pub const DF_METAFILE: c_int =         3;
pub const DF_SYLK: c_int =             4;
pub const DF_DIF: c_int =              5;
pub const DF_TIFF: c_int =             6;
pub const DF_OEMTEXT: c_int =          7;
pub const DF_DIB: c_int =              8;
pub const DF_PALETTE: c_int =          9;
pub const DF_PENDATA: c_int =          10;
pub const DF_RIFF: c_int =             11;
pub const DF_WAVE: c_int =             12;
pub const DF_UNICODETEXT: c_int =      13;
pub const DF_ENHMETAFILE: c_int =      14;
pub const DF_FILENAME: c_int =         15;
pub const DF_LOCALE: c_int =           16;
pub const DF_PRIVATE: c_int =          20;
pub const DF_HTML: c_int =             30;
pub const DF_PNG: c_int =              31;
pub const DF_MAX: c_int =              31 + 1;
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
pub const WXK_BROWSER_BACK: c_int = 501;
pub const WXK_BROWSER_FORWARD: c_int = 501 + 1;
pub const WXK_BROWSER_REFRESH: c_int = 501 + 2;
pub const WXK_BROWSER_STOP: c_int = 501 + 3;
pub const WXK_BROWSER_SEARCH: c_int = 501 + 4;
pub const WXK_BROWSER_FAVORITES: c_int = 501 + 5;
pub const WXK_BROWSER_HOME: c_int = 501 + 6;
pub const WXK_VOLUME_MUTE: c_int = 501 + 7;
pub const WXK_VOLUME_DOWN: c_int = 501 + 8;
pub const WXK_VOLUME_UP: c_int = 501 + 9;
pub const WXK_MEDIA_NEXT_TRACK: c_int = 501 + 10;
pub const WXK_MEDIA_PREV_TRACK: c_int = 501 + 11;
pub const WXK_MEDIA_STOP: c_int = 501 + 12;
pub const WXK_MEDIA_PLAY_PAUSE: c_int = 501 + 13;
pub const WXK_LAUNCH_MAIL: c_int = 501 + 14;
pub const WXK_LAUNCH_APP1: c_int = 501 + 15;
pub const WXK_LAUNCH_APP2: c_int = 501 + 16;
//  ENUM: wxKeyModifier
pub const MOD_NONE: c_int = 0x0000;
pub const MOD_ALT: c_int = 0x0001;
pub const MOD_CONTROL: c_int = 0x0002;
pub const MOD_ALTGR: c_int = MOD_ALT | MOD_CONTROL;
pub const MOD_SHIFT: c_int = 0x0004;
pub const MOD_META: c_int = 0x0008;
pub const MOD_WIN: c_int = MOD_META;
pub const MOD_RAW_CONTROL: c_int = MOD_META + 1;
pub const MOD_CMD: c_int = MOD_CONTROL;
pub const MOD_ALL: c_int = 0xffff;
//  ENUM: wxPaperSize
pub const PAPER_10X11: c_int = 0;
pub const PAPER_10X14: c_int = 0 + 1;
pub const PAPER_11X17: c_int = 0 + 2;
pub const PAPER_12X11: c_int = 0 + 3;
pub const PAPER_15X11: c_int = 0 + 4;
pub const PAPER_9X11: c_int = 0 + 5;
pub const PAPER_A2: c_int = 0 + 6;
pub const PAPER_A3: c_int = 0 + 7;
pub const PAPER_A3_EXTRA: c_int = 0 + 8;
pub const PAPER_A3_EXTRA_TRANSVERSE: c_int = 0 + 9;
pub const PAPER_A3_ROTATED: c_int = 0 + 10;
pub const PAPER_A3_TRANSVERSE: c_int = 0 + 11;
pub const PAPER_A4: c_int = 0 + 12;
pub const PAPER_A4SMALL: c_int = 0 + 13;
pub const PAPER_A4_EXTRA: c_int = 0 + 14;
pub const PAPER_A4_PLUS: c_int = 0 + 15;
pub const PAPER_A4_ROTATED: c_int = 0 + 16;
pub const PAPER_A4_TRANSVERSE: c_int = 0 + 17;
pub const PAPER_A5: c_int = 0 + 18;
pub const PAPER_A5_EXTRA: c_int = 0 + 19;
pub const PAPER_A5_ROTATED: c_int = 0 + 20;
pub const PAPER_A5_TRANSVERSE: c_int = 0 + 21;
pub const PAPER_A6: c_int = 0 + 22;
pub const PAPER_A6_ROTATED: c_int = 0 + 23;
pub const PAPER_A_PLUS: c_int = 0 + 24;
pub const PAPER_B4: c_int = 0 + 25;
pub const PAPER_B4_JIS_ROTATED: c_int = 0 + 26;
pub const PAPER_B5: c_int = 0 + 27;
pub const PAPER_B5_EXTRA: c_int = 0 + 28;
pub const PAPER_B5_JIS_ROTATED: c_int = 0 + 29;
pub const PAPER_B5_TRANSVERSE: c_int = 0 + 30;
pub const PAPER_B6_JIS: c_int = 0 + 31;
pub const PAPER_B6_JIS_ROTATED: c_int = 0 + 32;
pub const PAPER_B_PLUS: c_int = 0 + 33;
pub const PAPER_CSHEET: c_int = 0 + 34;
pub const PAPER_DBL_JAPANESE_POSTCARD: c_int = 0 + 35;
pub const PAPER_DBL_JAPANESE_POSTCARD_ROTATED: c_int = 0 + 36;
pub const PAPER_DSHEET: c_int = 0 + 37;
pub const PAPER_ENV_10: c_int = 0 + 38;
pub const PAPER_ENV_11: c_int = 0 + 39;
pub const PAPER_ENV_12: c_int = 0 + 40;
pub const PAPER_ENV_14: c_int = 0 + 41;
pub const PAPER_ENV_9: c_int = 0 + 42;
pub const PAPER_ENV_B4: c_int = 0 + 43;
pub const PAPER_ENV_B5: c_int = 0 + 44;
pub const PAPER_ENV_B6: c_int = 0 + 45;
pub const PAPER_ENV_C3: c_int = 0 + 46;
pub const PAPER_ENV_C4: c_int = 0 + 47;
pub const PAPER_ENV_C5: c_int = 0 + 48;
pub const PAPER_ENV_C6: c_int = 0 + 49;
pub const PAPER_ENV_C65: c_int = 0 + 50;
pub const PAPER_ENV_DL: c_int = 0 + 51;
pub const PAPER_ENV_INVITE: c_int = 0 + 52;
pub const PAPER_ENV_ITALY: c_int = 0 + 53;
pub const PAPER_ENV_MONARCH: c_int = 0 + 54;
pub const PAPER_ENV_PERSONAL: c_int = 0 + 55;
pub const PAPER_ESHEET: c_int = 0 + 56;
pub const PAPER_EXECUTIVE: c_int = 0 + 57;
pub const PAPER_FANFOLD_LGL_GERMAN: c_int = 0 + 58;
pub const PAPER_FANFOLD_STD_GERMAN: c_int = 0 + 59;
pub const PAPER_FANFOLD_US: c_int = 0 + 60;
pub const PAPER_FOLIO: c_int = 0 + 61;
pub const PAPER_ISO_B4: c_int = 0 + 62;
pub const PAPER_JAPANESE_POSTCARD: c_int = 0 + 63;
pub const PAPER_JAPANESE_POSTCARD_ROTATED: c_int = 0 + 64;
pub const PAPER_JENV_CHOU3: c_int = 0 + 65;
pub const PAPER_JENV_CHOU3_ROTATED: c_int = 0 + 66;
pub const PAPER_JENV_CHOU4: c_int = 0 + 67;
pub const PAPER_JENV_CHOU4_ROTATED: c_int = 0 + 68;
pub const PAPER_JENV_KAKU2: c_int = 0 + 69;
pub const PAPER_JENV_KAKU2_ROTATED: c_int = 0 + 70;
pub const PAPER_JENV_KAKU3: c_int = 0 + 71;
pub const PAPER_JENV_KAKU3_ROTATED: c_int = 0 + 72;
pub const PAPER_JENV_YOU4: c_int = 0 + 73;
pub const PAPER_JENV_YOU4_ROTATED: c_int = 0 + 74;
pub const PAPER_LEDGER: c_int = 0 + 75;
pub const PAPER_LEGAL: c_int = 0 + 76;
pub const PAPER_LEGAL_EXTRA: c_int = 0 + 77;
pub const PAPER_LETTER: c_int = 0 + 78;
pub const PAPER_LETTERSMALL: c_int = 0 + 79;
pub const PAPER_LETTER_EXTRA: c_int = 0 + 80;
pub const PAPER_LETTER_EXTRA_TRANSVERSE: c_int = 0 + 81;
pub const PAPER_LETTER_PLUS: c_int = 0 + 82;
pub const PAPER_LETTER_ROTATED: c_int = 0 + 83;
pub const PAPER_LETTER_TRANSVERSE: c_int = 0 + 84;
pub const PAPER_NONE: c_int = 0 + 85;
pub const PAPER_NOTE: c_int = 0 + 86;
pub const PAPER_P16K: c_int = 0 + 87;
pub const PAPER_P16K_ROTATED: c_int = 0 + 88;
pub const PAPER_P32K: c_int = 0 + 89;
pub const PAPER_P32KBIG: c_int = 0 + 90;
pub const PAPER_P32KBIG_ROTATED: c_int = 0 + 91;
pub const PAPER_P32K_ROTATED: c_int = 0 + 92;
pub const PAPER_PENV_1: c_int = 0 + 93;
pub const PAPER_PENV_10: c_int = 0 + 94;
pub const PAPER_PENV_10_ROTATED: c_int = 0 + 95;
pub const PAPER_PENV_1_ROTATED: c_int = 0 + 96;
pub const PAPER_PENV_2: c_int = 0 + 97;
pub const PAPER_PENV_2_ROTATED: c_int = 0 + 98;
pub const PAPER_PENV_3: c_int = 0 + 99;
pub const PAPER_PENV_3_ROTATED: c_int = 0 + 100;
pub const PAPER_PENV_4: c_int = 0 + 101;
pub const PAPER_PENV_4_ROTATED: c_int = 0 + 102;
pub const PAPER_PENV_5: c_int = 0 + 103;
pub const PAPER_PENV_5_ROTATED: c_int = 0 + 104;
pub const PAPER_PENV_6: c_int = 0 + 105;
pub const PAPER_PENV_6_ROTATED: c_int = 0 + 106;
pub const PAPER_PENV_7: c_int = 0 + 107;
pub const PAPER_PENV_7_ROTATED: c_int = 0 + 108;
pub const PAPER_PENV_8: c_int = 0 + 109;
pub const PAPER_PENV_8_ROTATED: c_int = 0 + 110;
pub const PAPER_PENV_9: c_int = 0 + 111;
pub const PAPER_PENV_9_ROTATED: c_int = 0 + 112;
pub const PAPER_QUARTO: c_int = 0 + 113;
pub const PAPER_STATEMENT: c_int = 0 + 114;
pub const PAPER_TABLOID: c_int = 0 + 115;
pub const PAPER_TABLOID_EXTRA: c_int = 0 + 116;
//  ENUM: wxPrintOrientation
pub const PORTRAIT: c_int = 0;
pub const LANDSCAPE: c_int = 0 + 1;
//  ENUM: wxDuplexMode
pub const DUPLEX_SIMPLEX: c_int = 0;
pub const DUPLEX_HORIZONTAL: c_int = 0 + 1;
pub const DUPLEX_VERTICAL: c_int = 0 + 2;
//  ENUM: wxPrintMode
pub const PRINT_MODE_NONE: c_int =    0;
pub const PRINT_MODE_PREVIEW: c_int = 1;
pub const PRINT_MODE_FILE: c_int =    2;
pub const PRINT_MODE_PRINTER: c_int = 3;
pub const PRINT_MODE_STREAM: c_int =  4;
//  ENUM: wxUpdateUI
pub const UPDATE_UI_NONE: c_int = 0;
pub const UPDATE_UI_RECURSE: c_int = 0 + 1;
pub const UPDATE_UI_FROMIDLE: c_int = 0 + 2;

//  ENUM: wxDirTraverseResult
pub const DIR_IGNORE: c_int = -1;
pub const DIR_STOP: c_int = -1 + 1;
pub const DIR_CONTINUE: c_int = -1 + 2;
//  ENUM: wxDirFlags
pub const DIR_FILES: c_int = 0x0001;
pub const DIR_DIRS: c_int = 0x0002;
pub const DIR_HIDDEN: c_int = 0x0004;
pub const DIR_DOTDOT: c_int = 0x0008;
pub const DIR_NO_FOLLOW: c_int = 0x0010;
pub const DIR_DEFAULT: c_int = DIR_FILES | DIR_DIRS | DIR_HIDDEN;

//  ENUM: @12
pub const DIRCTRL_DIR_ONLY: c_int = 0x0010;
pub const DIRCTRL_SELECT_FIRST: c_int = 0x0020;
pub const DIRCTRL_SHOW_FILTERS: c_int = 0x0040;
pub const DIRCTRL_3D_INTERNAL: c_int = 0x0080;
pub const DIRCTRL_EDIT_LABELS: c_int = 0x0100;
pub const DIRCTRL_MULTIPLE: c_int = 0x0200;
pub const DIRCTRL_DEFAULT_STYLE: c_int = DIRCTRL_3D_INTERNAL;

pub const DD_CHANGE_DIR: c_int = 0x0100;
pub const DD_DIR_MUST_EXIST: c_int = 0x0200;
pub const DD_MULTIPLE: c_int = 0x0400;
pub const DD_SHOW_HIDDEN: c_int = 0x0001;
pub const DD_NEW_DIR_BUTTON: c_int = 0;
pub const DD_DEFAULT_STYLE: c_long = (DEFAULT_DIALOG_STYLE|RESIZE_BORDER);

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

// NODEF: wxDYNLIB_FUNCTION
//  ENUM: wxDynamicLibraryCategory
pub const DL_LIBRARY: c_int = 0;
pub const DL_MODULE: c_int = 0 + 1;
//  ENUM: wxPluginCategory
pub const DL_PLUGIN_GUI: c_int = 0;
pub const DL_PLUGIN_BASE: c_int = 0 + 1;

pub const EL_ALLOW_NEW: c_int = 0x0100;
pub const EL_ALLOW_EDIT: c_int = 0x0200;
pub const EL_ALLOW_DELETE: c_int = 0x0400;
pub const EL_NO_REORDER: c_int = 0x0800;
pub const EL_DEFAULT_STYLE: c_int = (EL_ALLOW_NEW | EL_ALLOW_EDIT | EL_ALLOW_DELETE);

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
pub const EVENT_PROPAGATE_NONE: c_int = 0;
//  SKIP: wxEVENT_PROPAGATE_MAX
//  ENUM: wxEventCategory
pub const EVT_CATEGORY_UI: c_int = 1;
pub const EVT_CATEGORY_USER_INPUT: c_int = 2;
pub const EVT_CATEGORY_SOCKET: c_int = 4;
pub const EVT_CATEGORY_TIMER: c_int = 8;
pub const EVT_CATEGORY_THREAD: c_int = 16;
pub const EVT_CATEGORY_ALL: c_int =
        EVT_CATEGORY_UI|EVT_CATEGORY_USER_INPUT|EVT_CATEGORY_SOCKET| 
        EVT_CATEGORY_TIMER|EVT_CATEGORY_THREAD;
//  ENUM: wxKeyCategoryFlags
pub const WXK_CATEGORY_ARROW: c_int = 0;
pub const WXK_CATEGORY_PAGING: c_int = 0 + 1;
pub const WXK_CATEGORY_JUMP: c_int = 0 + 2;
pub const WXK_CATEGORY_TAB: c_int = 0 + 3;
pub const WXK_CATEGORY_CUT: c_int = 0 + 4;
pub const WXK_CATEGORY_NAVIGATION: c_int = 0 + 5;
//  ENUM: @14
pub const JOYSTICK1: c_int = 0;
pub const JOYSTICK2: c_int = 0 + 1;
//  ENUM: @15
pub const JOY_BUTTON_ANY: c_int = -1;
pub const JOY_BUTTON1: c_int = 1;
pub const JOY_BUTTON2: c_int = 2;
pub const JOY_BUTTON3: c_int = 4;
pub const JOY_BUTTON4: c_int = 8;
//  ENUM: wxUpdateUIMode
pub const UPDATE_UI_PROCESS_ALL: c_int = 0;
pub const UPDATE_UI_PROCESS_SPECIFIED: c_int = 0 + 1;
//  ENUM: wxMouseWheelAxis
pub const MOUSE_WHEEL_VERTICAL: c_int = 0;
pub const MOUSE_WHEEL_HORIZONTAL: c_int = 0 + 1;
//  ENUM: wxIdleMode
pub const IDLE_PROCESS_ALL: c_int = 0;
pub const IDLE_PROCESS_SPECIFIED: c_int = 0 + 1;

//  ENUM: wxFindReplaceFlags
pub const FR_DOWN: c_int = 1;
pub const FR_WHOLEWORD: c_int = 2;
pub const FR_MATCHCASE: c_int = 4;
//  ENUM: wxFindReplaceDialogStyles
pub const FR_REPLACEDIALOG: c_int = 1;
pub const FR_NOUPDOWN: c_int = 2;
pub const FR_NOMATCHCASE: c_int = 4;
pub const FR_NOWHOLEWORD: c_int = 8;

pub const FC_DEFAULT_STYLE: c_int = FC_OPEN;
//  ENUM: @18
pub const FC_OPEN: c_int = 0x0001;
pub const FC_SAVE: c_int = 0x0002;
pub const FC_MULTIPLE: c_int = 0x0004;
pub const FC_NOSHOWHIDDEN: c_int = 0x0008;

pub const FD_DEFAULT_STYLE: c_int = FD_OPEN;
//  ENUM: @19
pub const FD_OPEN: c_int = 0x0001;
pub const FD_SAVE: c_int = 0x0002;
pub const FD_OVERWRITE_PROMPT: c_int = 0x0004;
pub const FD_NO_FOLLOW: c_int = 0x0008;
pub const FD_FILE_MUST_EXIST: c_int = 0x0010;
pub const FD_CHANGE_DIR: c_int = 0x0080;
pub const FD_PREVIEW: c_int = 0x0100;
pub const FD_MULTIPLE: c_int = 0x0200;
pub const FD_SHOW_HIDDEN: c_int = 0x0400;

// NODEF: wxCHANGE_UMASK
//  ENUM: wxPosixPermissions
pub const S_IRUSR: c_int = 00400;
pub const S_IWUSR: c_int = 00200;
pub const S_IXUSR: c_int = 00100;
pub const S_IRGRP: c_int = 00040;
pub const S_IWGRP: c_int = 00020;
pub const S_IXGRP: c_int = 00010;
pub const S_IROTH: c_int = 00004;
pub const S_IWOTH: c_int = 00002;
pub const S_IXOTH: c_int = 00001;
pub const POSIX_USER_READ: c_int = S_IRUSR;
pub const POSIX_USER_WRITE: c_int = S_IWUSR;
pub const POSIX_USER_EXECUTE: c_int = S_IXUSR;
pub const POSIX_GROUP_READ: c_int = S_IRGRP;
pub const POSIX_GROUP_WRITE: c_int = S_IWGRP;
pub const POSIX_GROUP_EXECUTE: c_int = S_IXGRP;
pub const POSIX_OTHERS_READ: c_int = S_IROTH;
pub const POSIX_OTHERS_WRITE: c_int = S_IWOTH;
pub const POSIX_OTHERS_EXECUTE: c_int = S_IXOTH;
pub const S_DEFAULT: c_int = (POSIX_USER_READ | POSIX_USER_WRITE | 
                   POSIX_GROUP_READ | POSIX_GROUP_WRITE | 
                   POSIX_OTHERS_READ | POSIX_OTHERS_WRITE);
pub const S_DIR_DEFAULT: c_int = (POSIX_USER_READ | POSIX_USER_WRITE | POSIX_USER_EXECUTE | 
                       POSIX_GROUP_READ | POSIX_GROUP_WRITE | POSIX_GROUP_EXECUTE | 
                       POSIX_OTHERS_READ | POSIX_OTHERS_WRITE | POSIX_OTHERS_EXECUTE);
//  ENUM: wxSeekMode
pub const FromStart: c_int = 0;
pub const FromCurrent: c_int = 0 + 1;
pub const FromEnd: c_int = 0 + 2;
//  ENUM: wxFileKind
pub const FILE_KIND_UNKNOWN: c_int = 0;
pub const FILE_KIND_DISK: c_int = 0 + 1;
pub const FILE_KIND_TERMINAL: c_int = 0 + 2;
pub const FILE_KIND_PIPE: c_int = 0 + 3;

//  ENUM: wxFileHistoryMenuPathStyle
pub const FH_PATH_SHOW_IF_DIFFERENT: c_int = 0;
pub const FH_PATH_SHOW_NEVER: c_int = 0 + 1;
pub const FH_PATH_SHOW_ALWAYS: c_int = 0 + 2;

//  ENUM: wxPathFormat
pub const PATH_NATIVE: c_int = 0;
pub const PATH_UNIX: c_int = 0 + 1;
pub const PATH_BEOS: c_int = PATH_UNIX;
pub const PATH_MAC: c_int = PATH_UNIX + 1;
pub const PATH_DOS: c_int = PATH_UNIX + 2;
pub const PATH_WIN: c_int = PATH_DOS;
pub const PATH_OS2: c_int = PATH_DOS;
pub const PATH_VMS: c_int = PATH_DOS + 1;
pub const PATH_MAX: c_int = PATH_DOS + 2;
//  ENUM: wxSizeConvention
pub const SIZE_CONV_TRADITIONAL: c_int = 0;
pub const SIZE_CONV_IEC: c_int = 0 + 1;
pub const SIZE_CONV_SI: c_int = 0 + 2;
//  ENUM: wxPathNormalize
pub const PATH_NORM_ENV_VARS: c_int = 0x0001;
pub const PATH_NORM_DOTS: c_int = 0x0002;
pub const PATH_NORM_TILDE: c_int = 0x0004;
pub const PATH_NORM_CASE: c_int = 0x0008;
pub const PATH_NORM_ABSOLUTE: c_int = 0x0010;
pub const PATH_NORM_LONG: c_int =     0x0020;
pub const PATH_NORM_SHORTCUT: c_int = 0x0040;
pub const PATH_NORM_ALL: c_int = 0x00ff & !PATH_NORM_CASE;
//  ENUM: @20
pub const PATH_RMDIR_FULL: c_int = 1;
pub const PATH_RMDIR_RECURSIVE: c_int = 2;
//  ENUM: @21
pub const FILE_EXISTS_REGULAR: c_int = 0x0001;
pub const FILE_EXISTS_DIR: c_int = 0x0002;
pub const FILE_EXISTS_SYMLINK: c_int = 0x1004;
pub const FILE_EXISTS_DEVICE: c_int = 0x0008;
pub const FILE_EXISTS_FIFO: c_int = 0x0016;
pub const FILE_EXISTS_SOCKET: c_int = 0x0032;
//  SKIP: wxFILE_EXISTS_NO_FOLLOW

pub const FLP_OPEN: c_int = 0x0400;
pub const FLP_SAVE: c_int = 0x0800;
pub const FLP_OVERWRITE_PROMPT: c_int = 0x1000;
pub const FLP_FILE_MUST_EXIST: c_int = 0x2000;
pub const FLP_CHANGE_DIR: c_int = 0x4000;
pub const FLP_SMALL: c_int = PB_SMALL;
pub const FLP_USE_TEXTCTRL: c_int = (PB_USE_TEXTCTRL);
pub const FLP_DEFAULT_STYLE: c_int = (FLP_OPEN|FLP_FILE_MUST_EXIST);
pub const DIRP_DIR_MUST_EXIST: c_int = 0x0008;
pub const DIRP_CHANGE_DIR: c_int = 0x0010;
pub const DIRP_SMALL: c_int = PB_SMALL;
pub const DIRP_USE_TEXTCTRL: c_int = (PB_USE_TEXTCTRL);
pub const DIRP_DEFAULT_STYLE: c_int = (DIRP_DIR_MUST_EXIST);

//  ENUM: wxFileSystemOpenFlags
pub const FS_READ: c_int = 1;
pub const FS_SEEKABLE: c_int = 4;

pub const FNTP_FONTDESC_AS_LABEL: c_int = 0x0008;
pub const FNTP_USEFONT_FOR_LABEL: c_int = 0x0010;
pub const FONTBTN_DEFAULT_STYLE: c_int = (FNTP_FONTDESC_AS_LABEL | FNTP_USEFONT_FOR_LABEL);
pub const FNTP_USE_TEXTCTRL: c_int = (PB_USE_TEXTCTRL);
pub const FNTP_DEFAULT_STYLE: c_int = (FNTP_FONTDESC_AS_LABEL|FNTP_USEFONT_FOR_LABEL);

pub const FRAME_NO_TASKBAR: c_int = 0x0002;
pub const FRAME_TOOL_WINDOW: c_int = 0x0004;
pub const FRAME_FLOAT_ON_PARENT: c_int = 0x0008;

//  ENUM: wxFSWFlags
pub const FSW_EVENT_CREATE: c_int = 0x01;
pub const FSW_EVENT_DELETE: c_int = 0x02;
pub const FSW_EVENT_RENAME: c_int = 0x04;
pub const FSW_EVENT_MODIFY: c_int = 0x08;
pub const FSW_EVENT_ACCESS: c_int = 0x10;
pub const FSW_EVENT_ATTRIB: c_int = 0x20;
pub const FSW_EVENT_UNMOUNT: c_int = 0x2000;
pub const FSW_EVENT_WARNING: c_int = 0x40;
pub const FSW_EVENT_ERROR: c_int = 0x80;
pub const FSW_EVENT_ALL: c_int = FSW_EVENT_CREATE | FSW_EVENT_DELETE |
                         FSW_EVENT_RENAME | FSW_EVENT_MODIFY |
                         FSW_EVENT_ACCESS | FSW_EVENT_ATTRIB |
                         FSW_EVENT_WARNING | FSW_EVENT_ERROR;
//  ENUM: wxFSWWarningType
pub const FSW_WARNING_NONE: c_int = 0;
pub const FSW_WARNING_GENERAL: c_int = 0 + 1;
pub const FSW_WARNING_OVERFLOW: c_int = 0 + 2;

pub const GA_HORIZONTAL: c_int = HORIZONTAL;
pub const GA_VERTICAL: c_int = VERTICAL;
pub const GA_PROGRESS: c_int = 0x0010;
pub const GA_SMOOTH: c_int = 0x0020;
pub const GA_TEXT: c_int = 0x0040;

// NODEF: wxBITMAP
// NODEF: wxBITMAP_PNG
// NODEF: wxBITMAP_PNG_FROM_DATA
// NODEF: wxICON
//  ENUM: wxBitmapType
pub const BITMAP_TYPE_INVALID: c_int = 0;
pub const BITMAP_TYPE_BMP: c_int = 0 + 1;
pub const BITMAP_TYPE_BMP_RESOURCE: c_int = 0 + 2;
pub const BITMAP_TYPE_RESOURCE: c_int = BITMAP_TYPE_BMP_RESOURCE;
pub const BITMAP_TYPE_ICO: c_int = BITMAP_TYPE_BMP_RESOURCE + 1;
pub const BITMAP_TYPE_ICO_RESOURCE: c_int = BITMAP_TYPE_BMP_RESOURCE + 2;
pub const BITMAP_TYPE_CUR: c_int = BITMAP_TYPE_BMP_RESOURCE + 3;
pub const BITMAP_TYPE_CUR_RESOURCE: c_int = BITMAP_TYPE_BMP_RESOURCE + 4;
pub const BITMAP_TYPE_XBM: c_int = BITMAP_TYPE_BMP_RESOURCE + 5;
pub const BITMAP_TYPE_XBM_DATA: c_int = BITMAP_TYPE_BMP_RESOURCE + 6;
pub const BITMAP_TYPE_XPM: c_int = BITMAP_TYPE_BMP_RESOURCE + 7;
pub const BITMAP_TYPE_XPM_DATA: c_int = BITMAP_TYPE_BMP_RESOURCE + 8;
pub const BITMAP_TYPE_TIFF: c_int = BITMAP_TYPE_BMP_RESOURCE + 9;
pub const BITMAP_TYPE_TIF: c_int = BITMAP_TYPE_TIFF;
pub const BITMAP_TYPE_TIFF_RESOURCE: c_int = BITMAP_TYPE_TIFF + 1;
pub const BITMAP_TYPE_TIF_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE;
pub const BITMAP_TYPE_GIF: c_int = BITMAP_TYPE_TIFF_RESOURCE + 1;
pub const BITMAP_TYPE_GIF_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 2;
pub const BITMAP_TYPE_PNG: c_int = BITMAP_TYPE_TIFF_RESOURCE + 3;
pub const BITMAP_TYPE_PNG_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 4;
pub const BITMAP_TYPE_JPEG: c_int = BITMAP_TYPE_TIFF_RESOURCE + 5;
pub const BITMAP_TYPE_JPEG_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 6;
pub const BITMAP_TYPE_PNM: c_int = BITMAP_TYPE_TIFF_RESOURCE + 7;
pub const BITMAP_TYPE_PNM_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 8;
pub const BITMAP_TYPE_PCX: c_int = BITMAP_TYPE_TIFF_RESOURCE + 9;
pub const BITMAP_TYPE_PCX_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 10;
pub const BITMAP_TYPE_PICT: c_int = BITMAP_TYPE_TIFF_RESOURCE + 11;
pub const BITMAP_TYPE_PICT_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 12;
pub const BITMAP_TYPE_ICON: c_int = BITMAP_TYPE_TIFF_RESOURCE + 13;
pub const BITMAP_TYPE_ICON_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 14;
pub const BITMAP_TYPE_ANI: c_int = BITMAP_TYPE_TIFF_RESOURCE + 15;
pub const BITMAP_TYPE_IFF: c_int = BITMAP_TYPE_TIFF_RESOURCE + 16;
pub const BITMAP_TYPE_TGA: c_int = BITMAP_TYPE_TIFF_RESOURCE + 17;
pub const BITMAP_TYPE_MACCURSOR: c_int = BITMAP_TYPE_TIFF_RESOURCE + 18;
pub const BITMAP_TYPE_MACCURSOR_RESOURCE: c_int = BITMAP_TYPE_TIFF_RESOURCE + 19;
pub const BITMAP_TYPE_ANY: c_int = 50;
//  ENUM: wxPolygonFillMode
pub const ODDEVEN_RULE: c_int = 1;
pub const WINDING_RULE: c_int = 1 + 1;
//  ENUM: wxStockCursor
pub const CURSOR_NONE: c_int = 0;
pub const CURSOR_ARROW: c_int = 0 + 1;
pub const CURSOR_RIGHT_ARROW: c_int = 0 + 2;
pub const CURSOR_BULLSEYE: c_int = 0 + 3;
pub const CURSOR_CHAR: c_int = 0 + 4;
pub const CURSOR_CROSS: c_int = 0 + 5;
pub const CURSOR_HAND: c_int = 0 + 6;
pub const CURSOR_IBEAM: c_int = 0 + 7;
pub const CURSOR_LEFT_BUTTON: c_int = 0 + 8;
pub const CURSOR_MAGNIFIER: c_int = 0 + 9;
pub const CURSOR_MIDDLE_BUTTON: c_int = 0 + 10;
pub const CURSOR_NO_ENTRY: c_int = 0 + 11;
pub const CURSOR_PAINT_BRUSH: c_int = 0 + 12;
pub const CURSOR_PENCIL: c_int = 0 + 13;
pub const CURSOR_POINT_LEFT: c_int = 0 + 14;
pub const CURSOR_POINT_RIGHT: c_int = 0 + 15;
pub const CURSOR_QUESTION_ARROW: c_int = 0 + 16;
pub const CURSOR_RIGHT_BUTTON: c_int = 0 + 17;
pub const CURSOR_SIZENESW: c_int = 0 + 18;
pub const CURSOR_SIZENS: c_int = 0 + 19;
pub const CURSOR_SIZENWSE: c_int = 0 + 20;
pub const CURSOR_SIZEWE: c_int = 0 + 21;
pub const CURSOR_SIZING: c_int = 0 + 22;
pub const CURSOR_SPRAYCAN: c_int = 0 + 23;
pub const CURSOR_WAIT: c_int = 0 + 24;
pub const CURSOR_WATCH: c_int = 0 + 25;
pub const CURSOR_BLANK: c_int = 0 + 26;
pub const CURSOR_DEFAULT: c_int = 0 + 27;
pub const CURSOR_COPY_ARROW: c_int = 0 + 28;
pub const CURSOR_CROSS_REVERSE: c_int = 0 + 29;
pub const CURSOR_DOUBLE_ARROW: c_int = 0 + 30;
pub const CURSOR_BASED_ARROW_UP: c_int = 0 + 31;
pub const CURSOR_BASED_ARROW_DOWN: c_int = 0 + 32;
pub const CURSOR_ARROWWAIT: c_int = 0 + 33;
pub const CURSOR_MAX: c_int = 0 + 34;
//  ENUM: wxEllipsizeFlags
pub const ELLIPSIZE_FLAGS_NONE: c_int = 0;
pub const ELLIPSIZE_FLAGS_PROCESS_MNEMONICS: c_int = 1;
pub const ELLIPSIZE_FLAGS_EXPAND_TABS: c_int = 2;
pub const ELLIPSIZE_FLAGS_DEFAULT: c_int = ELLIPSIZE_FLAGS_PROCESS_MNEMONICS|
                                ELLIPSIZE_FLAGS_EXPAND_TABS;
//  ENUM: wxEllipsizeMode
pub const ELLIPSIZE_NONE: c_int = 0;
pub const ELLIPSIZE_START: c_int = 0 + 1;
pub const ELLIPSIZE_MIDDLE: c_int = 0 + 2;
pub const ELLIPSIZE_END: c_int = 0 + 3;

//  ENUM: wxOutCode
pub const Inside: c_int = 0x00;
pub const OutLeft: c_int = 0x01;
pub const OutRight: c_int = 0x02;
pub const OutTop: c_int = 0x08;
pub const OutBottom: c_int = 0x04;

//  ENUM: @22
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
pub const WX_GL_FRAMEBUFFER_SRGB: c_int = 1 + 18;
pub const WX_GL_MAJOR_VERSION: c_int = 1 + 19;
pub const WX_GL_MINOR_VERSION: c_int = 1 + 20;
pub const WX_GL_CORE_PROFILE: c_int = 1 + 21;
pub const wx_GL_COMPAT_PROFILE: c_int = 1 + 22;
pub const WX_GL_FORWARD_COMPAT: c_int = 1 + 23;
pub const WX_GL_ES2: c_int = 1 + 24;
pub const WX_GL_DEBUG: c_int = 1 + 25;
pub const WX_GL_ROBUST_ACCESS: c_int = 1 + 26;
pub const WX_GL_NO_RESET_NOTIFY: c_int = 1 + 27;
pub const WX_GL_LOSE_ON_RESET: c_int = 1 + 28;
pub const WX_GL_RESET_ISOLATION: c_int = 1 + 29;
pub const WX_GL_RELEASE_FLUSH: c_int = 1 + 30;
pub const WX_GL_RELEASE_NONE: c_int = 1 + 31;

//  ENUM: wxAntialiasMode
pub const ANTIALIAS_NONE: c_int = 0;
pub const ANTIALIAS_DEFAULT: c_int = 0 + 1;
//  ENUM: wxInterpolationQuality
pub const INTERPOLATION_DEFAULT: c_int = 0;
pub const INTERPOLATION_NONE: c_int = 0 + 1;
pub const INTERPOLATION_FAST: c_int = 0 + 2;
pub const INTERPOLATION_GOOD: c_int = 0 + 3;
pub const INTERPOLATION_BEST: c_int = 0 + 4;
//  ENUM: wxCompositionMode
pub const COMPOSITION_INVALID: c_int = -1;
pub const COMPOSITION_CLEAR: c_int = -1 + 1;
pub const COMPOSITION_SOURCE: c_int = -1 + 2;
pub const COMPOSITION_OVER: c_int = -1 + 3;
pub const COMPOSITION_IN: c_int = -1 + 4;
pub const COMPOSITION_OUT: c_int = -1 + 5;
pub const COMPOSITION_ATOP: c_int = -1 + 6;
pub const COMPOSITION_DEST: c_int = -1 + 7;
pub const COMPOSITION_DEST_OVER: c_int = -1 + 8;
pub const COMPOSITION_DEST_IN: c_int = -1 + 9;
pub const COMPOSITION_DEST_OUT: c_int = -1 + 10;
pub const COMPOSITION_DEST_ATOP: c_int = -1 + 11;
pub const COMPOSITION_XOR: c_int = -1 + 12;
pub const COMPOSITION_ADD: c_int = -1 + 13;
//  ENUM: wxGradientType
pub const GRADIENT_NONE: c_int = 0;
pub const GRADIENT_LINEAR: c_int = 0 + 1;
pub const GRADIENT_RADIAL: c_int = 0 + 2;

//  ENUM: @23
pub const COL_WIDTH_DEFAULT: c_int = -1;
pub const COL_WIDTH_AUTOSIZE: c_int = -2;
//  ENUM: @24
pub const COL_RESIZABLE: c_int = 1;
pub const COL_SORTABLE: c_int = 2;
pub const COL_REORDERABLE: c_int = 4;
pub const COL_HIDDEN: c_int = 8;
pub const COL_DEFAULT_FLAGS: c_int = COL_RESIZABLE | COL_REORDERABLE;

//  ENUM: @25
pub const HD_ALLOW_REORDER: c_int = 0x0001;
pub const HD_ALLOW_HIDE: c_int = 0x0002;
pub const HD_BITMAP_ON_RIGHT: c_int = 0x0004;
pub const HD_DEFAULT_STYLE: c_int = HD_ALLOW_REORDER;

pub const HELP_NETSCAPE: c_int = 1;
//  ENUM: wxHelpSearchMode
pub const HELP_SEARCH_INDEX: c_int = 0;
pub const HELP_SEARCH_ALL: c_int = 0 + 1;

pub const ID_HTML_HELPFRAME: c_int = (ID_HIGHEST + 1);
pub const HF_EMBEDDED: c_int = 0x00008000;
pub const HF_DIALOG: c_int = 0x00010000;
pub const HF_FRAME: c_int = 0x00020000;
pub const HF_MODAL: c_int = 0x00040000;

pub const HF_TOOLBAR: c_int = 0x0001;
pub const HF_CONTENTS: c_int = 0x0002;
pub const HF_INDEX: c_int = 0x0004;
pub const HF_SEARCH: c_int = 0x0008;
pub const HF_BOOKMARKS: c_int = 0x0010;
pub const HF_OPEN_FILES: c_int = 0x0020;
pub const HF_PRINT: c_int = 0x0040;
pub const HF_FLAT_TOOLBAR: c_int = 0x0080;
pub const HF_MERGE_BOOKS: c_int = 0x0100;
pub const HF_ICONS_BOOK: c_int = 0x0200;
pub const HF_ICONS_BOOK_CHAPTER: c_int = 0x0400;
pub const HF_ICONS_FOLDER: c_int = 0x0000;
pub const HF_DEFAULT_STYLE: c_int = (HF_TOOLBAR | HF_CONTENTS | HF_INDEX | HF_SEARCH | HF_BOOKMARKS | HF_PRINT);

//  ENUM: @26
pub const ID_HTML_PANEL: c_int = ID_HIGHEST + 10;
pub const ID_HTML_BACK: c_int = ID_HIGHEST + 10 + 1;
pub const ID_HTML_FORWARD: c_int = ID_HIGHEST + 10 + 2;
pub const ID_HTML_UPNODE: c_int = ID_HIGHEST + 10 + 3;
pub const ID_HTML_UP: c_int = ID_HIGHEST + 10 + 4;
pub const ID_HTML_DOWN: c_int = ID_HIGHEST + 10 + 5;
pub const ID_HTML_PRINT: c_int = ID_HIGHEST + 10 + 6;
pub const ID_HTML_OPENFILE: c_int = ID_HIGHEST + 10 + 7;
pub const ID_HTML_OPTIONS: c_int = ID_HIGHEST + 10 + 8;
pub const ID_HTML_BOOKMARKSLIST: c_int = ID_HIGHEST + 10 + 9;
pub const ID_HTML_BOOKMARKSADD: c_int = ID_HIGHEST + 10 + 10;
pub const ID_HTML_BOOKMARKSREMOVE: c_int = ID_HIGHEST + 10 + 11;
pub const ID_HTML_TREECTRL: c_int = ID_HIGHEST + 10 + 12;
pub const ID_HTML_INDEXPAGE: c_int = ID_HIGHEST + 10 + 13;
pub const ID_HTML_INDEXLIST: c_int = ID_HIGHEST + 10 + 14;
pub const ID_HTML_INDEXTEXT: c_int = ID_HIGHEST + 10 + 15;
pub const ID_HTML_INDEXBUTTON: c_int = ID_HIGHEST + 10 + 16;
pub const ID_HTML_INDEXBUTTONALL: c_int = ID_HIGHEST + 10 + 17;
pub const ID_HTML_NOTEBOOK: c_int = ID_HIGHEST + 10 + 18;
pub const ID_HTML_SEARCHPAGE: c_int = ID_HIGHEST + 10 + 19;
pub const ID_HTML_SEARCHTEXT: c_int = ID_HIGHEST + 10 + 20;
pub const ID_HTML_SEARCHLIST: c_int = ID_HIGHEST + 10 + 21;
pub const ID_HTML_SEARCHBUTTON: c_int = ID_HIGHEST + 10 + 22;
pub const ID_HTML_SEARCHCHOICE: c_int = ID_HIGHEST + 10 + 23;
pub const ID_HTML_COUNTINFO: c_int = ID_HIGHEST + 10 + 24;

//  ENUM: wxHtmlSelectionState
pub const HTML_SEL_OUT: c_int = 0;
pub const HTML_SEL_IN: c_int = 0 + 1;
pub const HTML_SEL_CHANGING: c_int = 0 + 2;
//  ENUM: @27
pub const HTML_FIND_EXACT: c_int = 1;
pub const HTML_FIND_NEAREST_BEFORE: c_int = 2;
pub const HTML_FIND_NEAREST_AFTER: c_int = 4;
//  ENUM: wxHtmlScriptMode
pub const HTML_SCRIPT_NORMAL: c_int = 0;
pub const HTML_SCRIPT_SUB: c_int = 0 + 1;
pub const HTML_SCRIPT_SUP: c_int = 0 + 2;

pub const HTML_ALIGN_LEFT: c_int = 0x0000;
pub const HTML_ALIGN_RIGHT: c_int = 0x0002;
pub const HTML_ALIGN_JUSTIFY: c_int = 0x0010;
pub const HTML_ALIGN_TOP: c_int = 0x0004;
pub const HTML_ALIGN_BOTTOM: c_int = 0x0008;
pub const HTML_ALIGN_CENTER: c_int = 0x0001;
pub const HTML_CLR_FOREGROUND: c_int = 0x0001;
pub const HTML_CLR_BACKGROUND: c_int = 0x0002;
pub const HTML_CLR_TRANSPARENT_BACKGROUND: c_int = 0x0004;
pub const HTML_UNITS_PIXELS: c_int = 0x0001;
pub const HTML_UNITS_PERCENT: c_int = 0x0002;
pub const HTML_INDENT_LEFT: c_int = 0x0010;
pub const HTML_INDENT_RIGHT: c_int = 0x0020;
pub const HTML_INDENT_TOP: c_int = 0x0040;
pub const HTML_INDENT_BOTTOM: c_int = 0x0080;
pub const HTML_INDENT_HORIZONTAL: c_int = (HTML_INDENT_LEFT | HTML_INDENT_RIGHT);
pub const HTML_INDENT_VERTICAL: c_int = (HTML_INDENT_TOP | HTML_INDENT_BOTTOM);
pub const HTML_INDENT_ALL: c_int = (HTML_INDENT_VERTICAL | HTML_INDENT_HORIZONTAL);
pub const HTML_COND_ISANCHOR: c_int = 1;
pub const HTML_COND_ISIMAGEMAP: c_int = 2;
pub const HTML_COND_USER: c_int = 10000;

//  ENUM: wxHtmlURLType
pub const HTML_URL_PAGE: c_int = 0;
pub const HTML_URL_IMAGE: c_int = 0 + 1;
pub const HTML_URL_OTHER: c_int = 0 + 2;

pub const HW_SCROLLBAR_NEVER: c_int = 0x0002;
pub const HW_SCROLLBAR_AUTO: c_int = 0x0004;
pub const HW_NO_SELECTION: c_int = 0x0008;
pub const HW_DEFAULT_STYLE: c_int = HW_SCROLLBAR_AUTO;
//  ENUM: wxHtmlOpeningStatus
pub const HTML_OPEN: c_int = 0;
pub const HTML_BLOCK: c_int = 0 + 1;
pub const HTML_REDIRECT: c_int = 0 + 2;

//  ENUM: @28
pub const PAGE_ODD: c_int = 0;
pub const PAGE_EVEN: c_int = 0 + 1;
pub const PAGE_ALL: c_int = 0 + 2;

pub const HLB_DEFAULT_STYLE: c_long = BORDER_SUNKEN;
pub const HLB_MULTIPLE: c_int = LB_MULTIPLE;

pub const HL_CONTEXTMENU: c_long = 0x0001;
pub const HL_ALIGN_LEFT: c_int = 0x0002;
pub const HL_ALIGN_RIGHT: c_int = 0x0004;
pub const HL_ALIGN_CENTRE: c_long = 0x0008;
pub const HL_DEFAULT_STYLE: c_long = (HL_CONTEXTMENU|NO_BORDER|HL_ALIGN_CENTRE);

pub const ICON_SCREEN_DEPTH: c_int = (-1);

pub const IMAGE_OPTION_QUALITY: &str = "quality";
pub const IMAGE_OPTION_FILENAME: &str = "FileName";
pub const IMAGE_OPTION_RESOLUTION: &str = "Resolution";
pub const IMAGE_OPTION_RESOLUTIONX: &str = "ResolutionX";
pub const IMAGE_OPTION_RESOLUTIONY: &str = "ResolutionY";
pub const IMAGE_OPTION_RESOLUTIONUNIT: &str = "ResolutionUnit";
pub const IMAGE_OPTION_MAX_WIDTH: &str = "MaxWidth";
pub const IMAGE_OPTION_MAX_HEIGHT: &str = "MaxHeight";
pub const IMAGE_OPTION_ORIGINAL_WIDTH: &str = "OriginalWidth";
pub const IMAGE_OPTION_ORIGINAL_HEIGHT: &str = "OriginalHeight";
pub const IMAGE_OPTION_BMP_FORMAT: &str = "wxBMP_FORMAT";
pub const IMAGE_OPTION_CUR_HOTSPOT_X: &str = "HotSpotX";
pub const IMAGE_OPTION_CUR_HOTSPOT_Y: &str = "HotSpotY";
pub const IMAGE_OPTION_GIF_COMMENT: &str = "GifComment";
pub const IMAGE_OPTION_GIF_TRANSPARENCY: &str = "Transparency";
pub const IMAGE_OPTION_GIF_TRANSPARENCY_HIGHLIGHT: &str = "Highlight";
pub const IMAGE_OPTION_GIF_TRANSPARENCY_UNCHANGED: &str = "Unchanged";
pub const IMAGE_OPTION_PNG_FORMAT: &str = "PngFormat";
pub const IMAGE_OPTION_PNG_BITDEPTH: &str = "PngBitDepth";
pub const IMAGE_OPTION_PNG_FILTER: &str = "PngF";
pub const IMAGE_OPTION_PNG_COMPRESSION_LEVEL: &str = "PngZL";
pub const IMAGE_OPTION_PNG_COMPRESSION_MEM_LEVEL: &str = "PngZM";
pub const IMAGE_OPTION_PNG_COMPRESSION_STRATEGY: &str = "PngZS";
pub const IMAGE_OPTION_PNG_COMPRESSION_BUFFER_SIZE: &str = "PngZB";
pub const IMAGE_OPTION_TIFF_BITSPERSAMPLE: &str = "BitsPerSample";
pub const IMAGE_OPTION_TIFF_SAMPLESPERPIXEL: &str = "SamplesPerPixel";
pub const IMAGE_OPTION_TIFF_COMPRESSION: &str = "Compression";
pub const IMAGE_OPTION_TIFF_PHOTOMETRIC: &str = "Photometric";
pub const IMAGE_OPTION_TIFF_IMAGEDESCRIPTOR: &str = "ImageDescriptor";
//  ENUM: wxImageResolution
pub const IMAGE_RESOLUTION_NONE: c_int = 0;
pub const IMAGE_RESOLUTION_INCHES: c_int = 1;
pub const IMAGE_RESOLUTION_CM: c_int = 2;
//  ENUM: wxImageResizeQuality
pub const IMAGE_QUALITY_NEAREST: c_int = 0;
pub const IMAGE_QUALITY_BILINEAR: c_int = 0 + 1;
pub const IMAGE_QUALITY_BICUBIC: c_int = 0 + 2;
pub const IMAGE_QUALITY_BOX_AVERAGE: c_int = 0 + 3;
pub const IMAGE_QUALITY_NORMAL: c_int = 0 + 4;
pub const IMAGE_QUALITY_HIGH: c_int = 0 + 5;
//  ENUM: wxImageAlphaBlendMode
pub const IMAGE_ALPHA_BLEND_OVER: c_int = 0;
pub const IMAGE_ALPHA_BLEND_COMPOSE: c_int = 1;
//  ENUM: wxImagePNGType
pub const PNG_TYPE_COLOUR: c_int = 0;
pub const PNG_TYPE_GREY: c_int = 2;
pub const PNG_TYPE_GREY_RED: c_int = 3;
pub const PNG_TYPE_PALETTE: c_int = 4;
//  ENUM: @30
pub const BMP_24BPP: c_int = 24;
pub const BMP_8BPP: c_int =  8;
pub const BMP_8BPP_GREY: c_int =  9;
pub const BMP_8BPP_GRAY: c_int =  BMP_8BPP_GREY;
pub const BMP_8BPP_RED: c_int = 10;
pub const BMP_8BPP_PALETTE: c_int = 11;
pub const BMP_4BPP: c_int =  4;
pub const BMP_1BPP: c_int =  1;
pub const BMP_1BPP_BW: c_int =  2;


pub const IMAGELIST_DRAW_NORMAL: c_int = 0x0001;
pub const IMAGELIST_DRAW_TRANSPARENT: c_int = 0x0002;
pub const IMAGELIST_DRAW_SELECTED: c_int = 0x0004;
pub const IMAGELIST_DRAW_FOCUSED: c_int = 0x0008;
//  ENUM: @31
pub const IMAGE_LIST_NORMAL: c_int = 0;
pub const IMAGE_LIST_SMALL: c_int = 0 + 1;
pub const IMAGE_LIST_STATE: c_int = 0 + 2;



//  ENUM: wxLayoutDirection
pub const Layout_Default: c_int = 0;
pub const Layout_LeftToRight: c_int = 0 + 1;
pub const Layout_RightToLeft: c_int = 0 + 2;
//  ENUM: wxLocaleCategory
pub const LOCALE_CAT_NUMBER: c_int = 0;
pub const LOCALE_CAT_DATE: c_int = 0 + 1;
pub const LOCALE_CAT_MONEY: c_int = 0 + 2;
pub const LOCALE_CAT_DEFAULT: c_int = 0 + 3;
//  ENUM: wxLocaleInfo
pub const LOCALE_THOUSANDS_SEP: c_int = 0;
pub const LOCALE_DECIMAL_POINT: c_int = 0 + 1;
pub const LOCALE_SHORT_DATE_FMT: c_int = 0 + 2;
pub const LOCALE_LONG_DATE_FMT: c_int = 0 + 3;
pub const LOCALE_DATE_TIME_FMT: c_int = 0 + 4;
pub const LOCALE_TIME_FMT: c_int = 0 + 5;
//  ENUM: wxLocaleInitFlags
pub const LOCALE_DONT_LOAD_DEFAULT: c_int = 0x0000;
pub const LOCALE_LOAD_DEFAULT: c_int = 0x0001;

//  ENUM: wxIPCFormat
pub const IPC_INVALID: c_int =     0;
pub const IPC_TEXT: c_int =        1;
pub const IPC_BITMAP: c_int =      2;
pub const IPC_METAFILE: c_int =    3;
pub const IPC_SYLK: c_int =        4;
pub const IPC_DIF: c_int =         5;
pub const IPC_TIFF: c_int =        6;
pub const IPC_OEMTEXT: c_int =     7;
pub const IPC_DIB: c_int =         8;
pub const IPC_PALETTE: c_int =     9;
pub const IPC_PENDATA: c_int =     10;
pub const IPC_RIFF: c_int =        11;
pub const IPC_WAVE: c_int =        12;
pub const IPC_UTF16TEXT: c_int =   13;
pub const IPC_ENHMETAFILE: c_int = 14;
pub const IPC_FILENAME: c_int =    15;
pub const IPC_LOCALE: c_int =      16;
pub const IPC_UTF8TEXT: c_int =    17;
pub const IPC_UTF32TEXT: c_int =   18;
pub const IPC_UNICODETEXT: c_int = IPC_UTF16TEXT;
pub const IPC_PRIVATE: c_int =     20;

//  ENUM: wxLanguage
pub const LANGUAGE_DEFAULT: c_int = 0;
pub const LANGUAGE_UNKNOWN: c_int = 0 + 1;
pub const LANGUAGE_ABKHAZIAN: c_int = 0 + 2;
pub const LANGUAGE_AFAR: c_int = 0 + 3;
pub const LANGUAGE_AFRIKAANS: c_int = 0 + 4;
pub const LANGUAGE_ALBANIAN: c_int = 0 + 5;
pub const LANGUAGE_AMHARIC: c_int = 0 + 6;
pub const LANGUAGE_ARABIC: c_int = 0 + 7;
pub const LANGUAGE_ARABIC_ALGERIA: c_int = 0 + 8;
pub const LANGUAGE_ARABIC_BAHRAIN: c_int = 0 + 9;
pub const LANGUAGE_ARABIC_EGYPT: c_int = 0 + 10;
pub const LANGUAGE_ARABIC_IRAQ: c_int = 0 + 11;
pub const LANGUAGE_ARABIC_JORDAN: c_int = 0 + 12;
pub const LANGUAGE_ARABIC_KUWAIT: c_int = 0 + 13;
pub const LANGUAGE_ARABIC_LEBANON: c_int = 0 + 14;
pub const LANGUAGE_ARABIC_LIBYA: c_int = 0 + 15;
pub const LANGUAGE_ARABIC_MOROCCO: c_int = 0 + 16;
pub const LANGUAGE_ARABIC_OMAN: c_int = 0 + 17;
pub const LANGUAGE_ARABIC_QATAR: c_int = 0 + 18;
pub const LANGUAGE_ARABIC_SAUDI_ARABIA: c_int = 0 + 19;
pub const LANGUAGE_ARABIC_SUDAN: c_int = 0 + 20;
pub const LANGUAGE_ARABIC_SYRIA: c_int = 0 + 21;
pub const LANGUAGE_ARABIC_TUNISIA: c_int = 0 + 22;
pub const LANGUAGE_ARABIC_UAE: c_int = 0 + 23;
pub const LANGUAGE_ARABIC_YEMEN: c_int = 0 + 24;
pub const LANGUAGE_ARMENIAN: c_int = 0 + 25;
pub const LANGUAGE_ASSAMESE: c_int = 0 + 26;
pub const LANGUAGE_ASTURIAN: c_int = 0 + 27;
pub const LANGUAGE_AYMARA: c_int = 0 + 28;
pub const LANGUAGE_AZERI: c_int = 0 + 29;
pub const LANGUAGE_AZERI_CYRILLIC: c_int = 0 + 30;
pub const LANGUAGE_AZERI_LATIN: c_int = 0 + 31;
pub const LANGUAGE_BASHKIR: c_int = 0 + 32;
pub const LANGUAGE_BASQUE: c_int = 0 + 33;
pub const LANGUAGE_BELARUSIAN: c_int = 0 + 34;
pub const LANGUAGE_BENGALI: c_int = 0 + 35;
pub const LANGUAGE_BHUTANI: c_int = 0 + 36;
pub const LANGUAGE_BIHARI: c_int = 0 + 37;
pub const LANGUAGE_BISLAMA: c_int = 0 + 38;
pub const LANGUAGE_BOSNIAN: c_int = 0 + 39;
pub const LANGUAGE_BRETON: c_int = 0 + 40;
pub const LANGUAGE_BULGARIAN: c_int = 0 + 41;
pub const LANGUAGE_BURMESE: c_int = 0 + 42;
pub const LANGUAGE_CATALAN: c_int = 0 + 43;
pub const LANGUAGE_CHINESE: c_int = 0 + 44;
pub const LANGUAGE_CHINESE_SIMPLIFIED: c_int = 0 + 45;
pub const LANGUAGE_CHINESE_TRADITIONAL: c_int = 0 + 46;
pub const LANGUAGE_CHINESE_HONGKONG: c_int = 0 + 47;
pub const LANGUAGE_CHINESE_MACAU: c_int = 0 + 48;
pub const LANGUAGE_CHINESE_SINGAPORE: c_int = 0 + 49;
pub const LANGUAGE_CHINESE_TAIWAN: c_int = 0 + 50;
pub const LANGUAGE_CORSICAN: c_int = 0 + 51;
pub const LANGUAGE_CROATIAN: c_int = 0 + 52;
pub const LANGUAGE_CZECH: c_int = 0 + 53;
pub const LANGUAGE_DANISH: c_int = 0 + 54;
pub const LANGUAGE_DUTCH: c_int = 0 + 55;
pub const LANGUAGE_DUTCH_BELGIAN: c_int = 0 + 56;
pub const LANGUAGE_ENGLISH: c_int = 0 + 57;
pub const LANGUAGE_ENGLISH_UK: c_int = 0 + 58;
pub const LANGUAGE_ENGLISH_US: c_int = 0 + 59;
pub const LANGUAGE_ENGLISH_AUSTRALIA: c_int = 0 + 60;
pub const LANGUAGE_ENGLISH_BELIZE: c_int = 0 + 61;
pub const LANGUAGE_ENGLISH_BOTSWANA: c_int = 0 + 62;
pub const LANGUAGE_ENGLISH_CANADA: c_int = 0 + 63;
pub const LANGUAGE_ENGLISH_CARIBBEAN: c_int = 0 + 64;
pub const LANGUAGE_ENGLISH_DENMARK: c_int = 0 + 65;
pub const LANGUAGE_ENGLISH_EIRE: c_int = 0 + 66;
pub const LANGUAGE_ENGLISH_ISRAEL: c_int = 0 + 67;
pub const LANGUAGE_ENGLISH_JAMAICA: c_int = 0 + 68;
pub const LANGUAGE_ENGLISH_NEW_ZEALAND: c_int = 0 + 69;
pub const LANGUAGE_ENGLISH_PHILIPPINES: c_int = 0 + 70;
pub const LANGUAGE_ENGLISH_SOUTH_AFRICA: c_int = 0 + 71;
pub const LANGUAGE_ENGLISH_TRINIDAD: c_int = 0 + 72;
pub const LANGUAGE_ENGLISH_ZIMBABWE: c_int = 0 + 73;
pub const LANGUAGE_ESPERANTO: c_int = 0 + 74;
pub const LANGUAGE_ESTONIAN: c_int = 0 + 75;
pub const LANGUAGE_FAEROESE: c_int = 0 + 76;
pub const LANGUAGE_FARSI: c_int = 0 + 77;
pub const LANGUAGE_FIJI: c_int = 0 + 78;
pub const LANGUAGE_FINNISH: c_int = 0 + 79;
pub const LANGUAGE_FRENCH: c_int = 0 + 80;
pub const LANGUAGE_FRENCH_BELGIAN: c_int = 0 + 81;
pub const LANGUAGE_FRENCH_CANADIAN: c_int = 0 + 82;
pub const LANGUAGE_FRENCH_LUXEMBOURG: c_int = 0 + 83;
pub const LANGUAGE_FRENCH_MONACO: c_int = 0 + 84;
pub const LANGUAGE_FRENCH_SWISS: c_int = 0 + 85;
pub const LANGUAGE_FRISIAN: c_int = 0 + 86;
pub const LANGUAGE_GALICIAN: c_int = 0 + 87;
pub const LANGUAGE_GEORGIAN: c_int = 0 + 88;
pub const LANGUAGE_GERMAN: c_int = 0 + 89;
pub const LANGUAGE_GERMAN_AUSTRIAN: c_int = 0 + 90;
pub const LANGUAGE_GERMAN_BELGIUM: c_int = 0 + 91;
pub const LANGUAGE_GERMAN_LIECHTENSTEIN: c_int = 0 + 92;
pub const LANGUAGE_GERMAN_LUXEMBOURG: c_int = 0 + 93;
pub const LANGUAGE_GERMAN_SWISS: c_int = 0 + 94;
pub const LANGUAGE_GREEK: c_int = 0 + 95;
pub const LANGUAGE_GREENLANDIC: c_int = 0 + 96;
pub const LANGUAGE_GUARANI: c_int = 0 + 97;
pub const LANGUAGE_GUJARATI: c_int = 0 + 98;
pub const LANGUAGE_HAUSA: c_int = 0 + 99;
pub const LANGUAGE_HEBREW: c_int = 0 + 100;
pub const LANGUAGE_HINDI: c_int = 0 + 101;
pub const LANGUAGE_HUNGARIAN: c_int = 0 + 102;
pub const LANGUAGE_ICELANDIC: c_int = 0 + 103;
pub const LANGUAGE_INDONESIAN: c_int = 0 + 104;
pub const LANGUAGE_INTERLINGUA: c_int = 0 + 105;
pub const LANGUAGE_INTERLINGUE: c_int = 0 + 106;
pub const LANGUAGE_INUKTITUT: c_int = 0 + 107;
pub const LANGUAGE_INUPIAK: c_int = 0 + 108;
pub const LANGUAGE_IRISH: c_int = 0 + 109;
pub const LANGUAGE_ITALIAN: c_int = 0 + 110;
pub const LANGUAGE_ITALIAN_SWISS: c_int = 0 + 111;
pub const LANGUAGE_JAPANESE: c_int = 0 + 112;
pub const LANGUAGE_JAVANESE: c_int = 0 + 113;
pub const LANGUAGE_KABYLE: c_int = 0 + 114;
pub const LANGUAGE_KANNADA: c_int = 0 + 115;
pub const LANGUAGE_KASHMIRI: c_int = 0 + 116;
pub const LANGUAGE_KASHMIRI_INDIA: c_int = 0 + 117;
pub const LANGUAGE_KAZAKH: c_int = 0 + 118;
pub const LANGUAGE_KERNEWEK: c_int = 0 + 119;
pub const LANGUAGE_KHMER: c_int = 0 + 120;
pub const LANGUAGE_KINYARWANDA: c_int = 0 + 121;
pub const LANGUAGE_KIRGHIZ: c_int = 0 + 122;
pub const LANGUAGE_KIRUNDI: c_int = 0 + 123;
pub const LANGUAGE_KONKANI: c_int = 0 + 124;
pub const LANGUAGE_KOREAN: c_int = 0 + 125;
pub const LANGUAGE_KURDISH: c_int = 0 + 126;
pub const LANGUAGE_LAOTHIAN: c_int = 0 + 127;
pub const LANGUAGE_LATIN: c_int = 0 + 128;
pub const LANGUAGE_LATVIAN: c_int = 0 + 129;
pub const LANGUAGE_LINGALA: c_int = 0 + 130;
pub const LANGUAGE_LITHUANIAN: c_int = 0 + 131;
pub const LANGUAGE_MACEDONIAN: c_int = 0 + 132;
pub const LANGUAGE_MALAGASY: c_int = 0 + 133;
pub const LANGUAGE_MALAY: c_int = 0 + 134;
pub const LANGUAGE_MALAYALAM: c_int = 0 + 135;
pub const LANGUAGE_MALAY_BRUNEI_DARUSSALAM: c_int = 0 + 136;
pub const LANGUAGE_MALAY_MALAYSIA: c_int = 0 + 137;
pub const LANGUAGE_MALTESE: c_int = 0 + 138;
pub const LANGUAGE_MANIPURI: c_int = 0 + 139;
pub const LANGUAGE_MAORI: c_int = 0 + 140;
pub const LANGUAGE_MARATHI: c_int = 0 + 141;
pub const LANGUAGE_MOLDAVIAN: c_int = 0 + 142;
pub const LANGUAGE_MONGOLIAN: c_int = 0 + 143;
pub const LANGUAGE_NAURU: c_int = 0 + 144;
pub const LANGUAGE_NEPALI: c_int = 0 + 145;
pub const LANGUAGE_NEPALI_INDIA: c_int = 0 + 146;
pub const LANGUAGE_NORWEGIAN_BOKMAL: c_int = 0 + 147;
pub const LANGUAGE_NORWEGIAN_NYNORSK: c_int = 0 + 148;
pub const LANGUAGE_OCCITAN: c_int = 0 + 149;
pub const LANGUAGE_ORIYA: c_int = 0 + 150;
pub const LANGUAGE_OROMO: c_int = 0 + 151;
pub const LANGUAGE_PASHTO: c_int = 0 + 152;
pub const LANGUAGE_POLISH: c_int = 0 + 153;
pub const LANGUAGE_PORTUGUESE: c_int = 0 + 154;
pub const LANGUAGE_PORTUGUESE_BRAZILIAN: c_int = 0 + 155;
pub const LANGUAGE_PUNJABI: c_int = 0 + 156;
pub const LANGUAGE_QUECHUA: c_int = 0 + 157;
pub const LANGUAGE_RHAETO_ROMANCE: c_int = 0 + 158;
pub const LANGUAGE_ROMANIAN: c_int = 0 + 159;
pub const LANGUAGE_RUSSIAN: c_int = 0 + 160;
pub const LANGUAGE_RUSSIAN_UKRAINE: c_int = 0 + 161;
pub const LANGUAGE_SAMI: c_int = 0 + 162;
pub const LANGUAGE_SAMOAN: c_int = 0 + 163;
pub const LANGUAGE_SANGHO: c_int = 0 + 164;
pub const LANGUAGE_SANSKRIT: c_int = 0 + 165;
pub const LANGUAGE_SCOTS_GAELIC: c_int = 0 + 166;
pub const LANGUAGE_SERBIAN: c_int = 0 + 167;
pub const LANGUAGE_SERBIAN_CYRILLIC: c_int = 0 + 168;
pub const LANGUAGE_SERBIAN_LATIN: c_int = 0 + 169;
pub const LANGUAGE_SERBO_CROATIAN: c_int = 0 + 170;
pub const LANGUAGE_SESOTHO: c_int = 0 + 171;
pub const LANGUAGE_SETSWANA: c_int = 0 + 172;
pub const LANGUAGE_SHONA: c_int = 0 + 173;
pub const LANGUAGE_SINDHI: c_int = 0 + 174;
pub const LANGUAGE_SINHALESE: c_int = 0 + 175;
pub const LANGUAGE_SISWATI: c_int = 0 + 176;
pub const LANGUAGE_SLOVAK: c_int = 0 + 177;
pub const LANGUAGE_SLOVENIAN: c_int = 0 + 178;
pub const LANGUAGE_SOMALI: c_int = 0 + 179;
pub const LANGUAGE_SPANISH: c_int = 0 + 180;
pub const LANGUAGE_SPANISH_ARGENTINA: c_int = 0 + 181;
pub const LANGUAGE_SPANISH_BOLIVIA: c_int = 0 + 182;
pub const LANGUAGE_SPANISH_CHILE: c_int = 0 + 183;
pub const LANGUAGE_SPANISH_COLOMBIA: c_int = 0 + 184;
pub const LANGUAGE_SPANISH_COSTA_RICA: c_int = 0 + 185;
pub const LANGUAGE_SPANISH_DOMINICAN_REPUBLIC: c_int = 0 + 186;
pub const LANGUAGE_SPANISH_ECUADOR: c_int = 0 + 187;
pub const LANGUAGE_SPANISH_EL_SALVADOR: c_int = 0 + 188;
pub const LANGUAGE_SPANISH_GUATEMALA: c_int = 0 + 189;
pub const LANGUAGE_SPANISH_HONDURAS: c_int = 0 + 190;
pub const LANGUAGE_SPANISH_MEXICAN: c_int = 0 + 191;
pub const LANGUAGE_SPANISH_MODERN: c_int = 0 + 192;
pub const LANGUAGE_SPANISH_NICARAGUA: c_int = 0 + 193;
pub const LANGUAGE_SPANISH_PANAMA: c_int = 0 + 194;
pub const LANGUAGE_SPANISH_PARAGUAY: c_int = 0 + 195;
pub const LANGUAGE_SPANISH_PERU: c_int = 0 + 196;
pub const LANGUAGE_SPANISH_PUERTO_RICO: c_int = 0 + 197;
pub const LANGUAGE_SPANISH_URUGUAY: c_int = 0 + 198;
pub const LANGUAGE_SPANISH_US: c_int = 0 + 199;
pub const LANGUAGE_SPANISH_VENEZUELA: c_int = 0 + 200;
pub const LANGUAGE_SUNDANESE: c_int = 0 + 201;
pub const LANGUAGE_SWAHILI: c_int = 0 + 202;
pub const LANGUAGE_SWEDISH: c_int = 0 + 203;
pub const LANGUAGE_SWEDISH_FINLAND: c_int = 0 + 204;
pub const LANGUAGE_TAGALOG: c_int = 0 + 205;
pub const LANGUAGE_TAJIK: c_int = 0 + 206;
pub const LANGUAGE_TAMIL: c_int = 0 + 207;
pub const LANGUAGE_TATAR: c_int = 0 + 208;
pub const LANGUAGE_TELUGU: c_int = 0 + 209;
pub const LANGUAGE_THAI: c_int = 0 + 210;
pub const LANGUAGE_TIBETAN: c_int = 0 + 211;
pub const LANGUAGE_TIGRINYA: c_int = 0 + 212;
pub const LANGUAGE_TONGA: c_int = 0 + 213;
pub const LANGUAGE_TSONGA: c_int = 0 + 214;
pub const LANGUAGE_TURKISH: c_int = 0 + 215;
pub const LANGUAGE_TURKMEN: c_int = 0 + 216;
pub const LANGUAGE_TWI: c_int = 0 + 217;
pub const LANGUAGE_UIGHUR: c_int = 0 + 218;
pub const LANGUAGE_UKRAINIAN: c_int = 0 + 219;
pub const LANGUAGE_URDU: c_int = 0 + 220;
pub const LANGUAGE_URDU_INDIA: c_int = 0 + 221;
pub const LANGUAGE_URDU_PAKISTAN: c_int = 0 + 222;
pub const LANGUAGE_UZBEK: c_int = 0 + 223;
pub const LANGUAGE_UZBEK_CYRILLIC: c_int = 0 + 224;
pub const LANGUAGE_UZBEK_LATIN: c_int = 0 + 225;
pub const LANGUAGE_VALENCIAN: c_int = 0 + 226;
pub const LANGUAGE_VIETNAMESE: c_int = 0 + 227;
pub const LANGUAGE_VOLAPUK: c_int = 0 + 228;
pub const LANGUAGE_WELSH: c_int = 0 + 229;
pub const LANGUAGE_WOLOF: c_int = 0 + 230;
pub const LANGUAGE_XHOSA: c_int = 0 + 231;
pub const LANGUAGE_YIDDISH: c_int = 0 + 232;
pub const LANGUAGE_YORUBA: c_int = 0 + 233;
pub const LANGUAGE_ZHUANG: c_int = 0 + 234;
pub const LANGUAGE_ZULU: c_int = 0 + 235;
pub const LANGUAGE_USER_DEFINED: c_int = 0 + 236;
pub const LANGUAGE_CAMBODIAN: c_int = LANGUAGE_KHMER;

//  ENUM: wxEdge
pub const Left: c_int = 0;
pub const Top: c_int = 0 + 1;
pub const Right: c_int = 0 + 2;
pub const Bottom: c_int = 0 + 3;
pub const Width: c_int = 0 + 4;
pub const Height: c_int = 0 + 5;
pub const Centre: c_int = 0 + 6;
pub const Center: c_int = Centre;
pub const CentreX: c_int = Centre + 1;
pub const CentreY: c_int = Centre + 2;
//  ENUM: wxRelationship
pub const Unconstrained: c_int = 0;
pub const AsIs: c_int = 0 + 1;
pub const PercentOf: c_int = 0 + 2;
pub const Above: c_int = 0 + 3;
pub const Below: c_int = 0 + 4;
pub const LeftOf: c_int = 0 + 5;
pub const RightOf: c_int = 0 + 6;
pub const SameAs: c_int = 0 + 7;
pub const Absolute: c_int = 0 + 8;

//  ENUM: wxLayoutOrientation
pub const LAYOUT_HORIZONTAL: c_int = 0;
pub const LAYOUT_VERTICAL: c_int = 0 + 1;
//  ENUM: wxLayoutAlignment
pub const LAYOUT_NONE: c_int = 0;
pub const LAYOUT_TOP: c_int = 0 + 1;
pub const LAYOUT_LEFT: c_int = 0 + 2;
pub const LAYOUT_RIGHT: c_int = 0 + 3;
pub const LAYOUT_BOTTOM: c_int = 0 + 4;

// NODEF: wxFORCE_LINK_THIS_MODULE
// NODEF: wxFORCE_LINK_MODULE

pub const LB_DEFAULT: c_int = BK_DEFAULT;
pub const LB_TOP: c_int = BK_TOP;
pub const LB_BOTTOM: c_int = BK_BOTTOM;
pub const LB_LEFT: c_int = BK_LEFT;
pub const LB_RIGHT: c_int = BK_RIGHT;
pub const LB_ALIGN_MASK: c_int = BK_ALIGN_MASK;

// NODEF: wxLongLongFmtSpec

//  ENUM: wxMediaState
pub const MEDIASTATE_STOPPED: c_int = 0;
pub const MEDIASTATE_PAUSED: c_int = 0 + 1;
pub const MEDIASTATE_PLAYING: c_int = 0 + 2;
//  ENUM: wxMediaCtrlPlayerControls
pub const MEDIACTRLPLAYERCONTROLS_NONE: c_int =   0;
pub const MEDIACTRLPLAYERCONTROLS_STEP: c_int =   1 << 0;
pub const MEDIACTRLPLAYERCONTROLS_VOLUME: c_int =   1 << 1;
pub const MEDIACTRLPLAYERCONTROLS_DEFAULT: c_int =
                    MEDIACTRLPLAYERCONTROLS_STEP |
                    MEDIACTRLPLAYERCONTROLS_VOLUME;

// NODEF: WXTRACE
// NODEF: WXTRACELEVEL

//  ENUM: wxMouseButton
pub const MOUSE_BTN_ANY: c_int = -1;
pub const MOUSE_BTN_NONE: c_int = 0;
pub const MOUSE_BTN_LEFT: c_int = 1;
pub const MOUSE_BTN_MIDDLE: c_int = 2;
pub const MOUSE_BTN_RIGHT: c_int = 3;
pub const MOUSE_BTN_AUX1: c_int = 4;
pub const MOUSE_BTN_AUX2: c_int = 5;
pub const MOUSE_BTN_MAX: c_int = 5 + 1;

//  ENUM: wxMessageOutputFlags
pub const MSGOUT_PREFER_STDERR: c_int = 0;
pub const MSGOUT_PREFER_MSGBOX: c_int = 1;

//  ENUM: wxMessageQueueError
pub const MSGQUEUE_NO_ERROR: c_int = 0;
pub const MSGQUEUE_TIMEOUT: c_int = 0 + 1;
pub const MSGQUEUE_MISC_ERROR: c_int = 0 + 2;

//  ENUM: wxAutomationInstanceFlags
pub const AutomationInstance_UseExistingOnly: c_int = 0;
pub const AutomationInstance_CreateIfNeeded: c_int = 1;
pub const AutomationInstance_SilentIfNone: c_int = 2;

//  ENUM: wxOleConvertVariantFlags
pub const OleConvertVariant_Default: c_int = 0;
pub const OleConvertVariant_ReturnSafeArrays: c_int = 1;

pub const FRAME_SHAPED: c_int = 0x0010;

pub const NB_DEFAULT: c_int = BK_DEFAULT;
pub const NB_TOP: c_int = BK_TOP;
pub const NB_BOTTOM: c_int = BK_BOTTOM;
pub const NB_LEFT: c_int = BK_LEFT;
pub const NB_RIGHT: c_int = BK_RIGHT;
pub const NB_FIXEDWIDTH: c_int = 0x0100;
pub const NB_MULTILINE: c_int = 0x0200;
pub const NB_NOPAGETHEME: c_int = 0x0400;
//  ENUM: @37
pub const NB_HITTEST_NOWHERE: c_int = BK_HITTEST_NOWHERE;
pub const NB_HITTEST_ONICON: c_int = BK_HITTEST_ONICON;
pub const NB_HITTEST_ONLABEL: c_int = BK_HITTEST_ONLABEL;
pub const NB_HITTEST_ONITEM: c_int = BK_HITTEST_ONITEM;
pub const NB_HITTEST_ONPAGE: c_int = BK_HITTEST_ONPAGE;

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

//  ENUM: wxOwnerDrawnComboBoxPaintingFlags
pub const ODCB_PAINTING_CONTROL: c_int = 0x0001;
pub const ODCB_PAINTING_SELECTED: c_int = 0x0002;
//  ENUM: @39
pub const ODCB_DCLICK_CYCLES: c_int = CC_SPECIAL_DCLICK;
pub const ODCB_STD_CONTROL_PAINT: c_int = 0x1000;

//  ENUM: wxPenStyle
pub const PENSTYLE_INVALID: c_int = -1;
pub const PENSTYLE_SOLID: c_int = -1 + 1;
pub const PENSTYLE_DOT: c_int = -1 + 2;
pub const PENSTYLE_LONG_DASH: c_int = -1 + 3;
pub const PENSTYLE_SHORT_DASH: c_int = -1 + 4;
pub const PENSTYLE_DOT_DASH: c_int = -1 + 5;
pub const PENSTYLE_USER_DASH: c_int = -1 + 6;
pub const PENSTYLE_TRANSPARENT: c_int = -1 + 7;
pub const PENSTYLE_STIPPLE_MASK_OPAQUE: c_int = -1 + 8;
pub const PENSTYLE_STIPPLE_MASK: c_int = -1 + 9;
pub const PENSTYLE_STIPPLE: c_int = -1 + 10;
pub const PENSTYLE_BDIAGONAL_HATCH: c_int = -1 + 11;
pub const PENSTYLE_CROSSDIAG_HATCH: c_int = -1 + 12;
pub const PENSTYLE_FDIAGONAL_HATCH: c_int = -1 + 13;
pub const PENSTYLE_CROSS_HATCH: c_int = -1 + 14;
pub const PENSTYLE_HORIZONTAL_HATCH: c_int = -1 + 15;
pub const PENSTYLE_VERTICAL_HATCH: c_int = -1 + 16;
pub const PENSTYLE_FIRST_HATCH: c_int = -1 + 17;
pub const PENSTYLE_LAST_HATCH: c_int = -1 + 18;
//  ENUM: wxPenQuality
pub const PEN_QUALITY_DEFAULT: c_int = 0;
pub const PEN_QUALITY_LOW: c_int = 0 + 1;
pub const PEN_QUALITY_HIGH: c_int = 0 + 2;
//  ENUM: wxPenJoin
pub const JOIN_INVALID: c_int = -1;
pub const JOIN_BEVEL: c_int = 120;
pub const JOIN_MITER: c_int = 120 + 1;
pub const JOIN_ROUND: c_int = 120 + 2;
//  ENUM: wxPenCap
pub const CAP_INVALID: c_int = -1;
pub const CAP_ROUND: c_int = 130;
pub const CAP_PROJECTING: c_int = 130 + 1;
pub const CAP_BUTT: c_int = 130 + 2;

pub const PB_USE_TEXTCTRL: c_int = 0x0002;
pub const PB_SMALL: c_int = 0x8000;

// NODEF: wxCHECK_GCC_VERSION
// NODEF: wxCHECK_SUNCC_VERSION
// NODEF: wxCHECK_VISUALC_VERSION
// NODEF: wxCHECK_W32API_VERSION

//  ENUM: wxOperatingSystemId
pub const OS_UNKNOWN: c_int = 0;
pub const OS_MAC_OS: c_int = 1 << 0;
pub const OS_MAC_OSX_DARWIN: c_int = 1 << 1;
pub const OS_MAC: c_int = OS_MAC_OS|OS_MAC_OSX_DARWIN;
pub const OS_WINDOWS_NT: c_int = 1 << 3;
pub const OS_WINDOWS: c_int = OS_WINDOWS_NT;
pub const OS_UNIX_LINUX: c_int = 1 << 6;
pub const OS_UNIX_FREEBSD: c_int = 1 << 7;
pub const OS_UNIX_OPENBSD: c_int = 1 << 8;
pub const OS_UNIX_NETBSD: c_int = 1 << 9;
pub const OS_UNIX_SOLARIS: c_int = 1 << 10;
pub const OS_UNIX_AIX: c_int = 1 << 11;
pub const OS_UNIX_HPUX: c_int = 1 << 12;
pub const OS_UNIX: c_int = OS_UNIX_LINUX     |
                OS_UNIX_FREEBSD   |
                OS_UNIX_OPENBSD   |
                OS_UNIX_NETBSD    |
                OS_UNIX_SOLARIS   |
                OS_UNIX_AIX       |
                OS_UNIX_HPUX;
//  ENUM: wxPortId
pub const PORT_UNKNOWN: c_int = 0;
pub const PORT_BASE: c_int = 1 << 0;
pub const PORT_MSW: c_int = 1 << 1;
pub const PORT_MOTIF: c_int = 1 << 2;
pub const PORT_GTK: c_int = 1 << 3;
pub const PORT_DFB: c_int = 1 << 4;
pub const PORT_X11: c_int = 1 << 5;
pub const PORT_MAC: c_int = 1 << 7;
pub const PORT_COCOA: c_int = 1 << 8;
pub const PORT_QT: c_int = 1 << 10;
//  ENUM: wxBitness
pub const BITNESS_INVALID: c_int = -1;
pub const BITNESS_32: c_int = -1 + 1;
pub const BITNESS_64: c_int = -1 + 2;
pub const BITNESS_MAX: c_int = -1 + 3;
//  ENUM: wxArchitecture
pub const ARCH_INVALID: c_int = -1;
pub const ARCH_32: c_int = -1 + 1;
pub const ARCH_64: c_int = -1 + 2;
pub const ARCH_MAX: c_int = -1 + 3;
//  ENUM: wxEndianness
pub const ENDIAN_INVALID: c_int = -1;
pub const ENDIAN_BIG: c_int = -1 + 1;
pub const ENDIAN_LITTLE: c_int = -1 + 2;
pub const ENDIAN_PDP: c_int = -1 + 3;
pub const ENDIAN_MAX: c_int = -1 + 4;

pub const PU_CONTAINS_CONTROLS: c_int = 0x0001;

//  ENUM: wxPowerType
pub const POWER_SOCKET: c_int = 0;
pub const POWER_BATTERY: c_int = 0 + 1;
pub const POWER_UNKNOWN: c_int = 0 + 2;
//  ENUM: wxBatteryState
pub const BATTERY_NORMAL_STATE: c_int = 0;
pub const BATTERY_LOW_STATE: c_int = 0 + 1;
pub const BATTERY_CRITICAL_STATE: c_int = 0 + 2;
pub const BATTERY_SHUTDOWN_STATE: c_int = 0 + 3;
pub const BATTERY_UNKNOWN_STATE: c_int = 0 + 4;
//  ENUM: wxPowerResourceKind
pub const POWER_RESOURCE_SCREEN: c_int = 0;
pub const POWER_RESOURCE_SYSTEM: c_int = 0 + 1;

pub const PREVIEW_PRINT: c_int = 1;
pub const PREVIEW_PREVIOUS: c_int = 2;
pub const PREVIEW_NEXT: c_int = 4;
pub const PREVIEW_ZOOM: c_int = 8;
pub const PREVIEW_FIRST: c_int = 16;
pub const PREVIEW_LAST: c_int = 32;
pub const PREVIEW_GOTO: c_int = 64;
pub const PREVIEW_DEFAULT: c_int = (PREVIEW_PREVIOUS|PREVIEW_NEXT|PREVIEW_ZOOM|PREVIEW_FIRST|PREVIEW_GOTO|PREVIEW_LAST);
pub const ID_PREVIEW_CLOSE: c_int = 1;
pub const ID_PREVIEW_NEXT: c_int = 2;
pub const ID_PREVIEW_PREVIOUS: c_int = 3;
pub const ID_PREVIEW_PRINT: c_int = 4;
pub const ID_PREVIEW_ZOOM: c_int = 5;
pub const ID_PREVIEW_FIRST: c_int = 6;
pub const ID_PREVIEW_LAST: c_int = 7;
pub const ID_PREVIEW_GOTO: c_int = 8;
pub const ID_PREVIEW_ZOOM_IN: c_int = 9;
pub const ID_PREVIEW_ZOOM_OUT: c_int = 10;
//  ENUM: wxPrinterError
pub const PRINTER_NO_ERROR: c_int = 0;
pub const PRINTER_CANCELLED: c_int = 0 + 1;
pub const PRINTER_ERROR: c_int = 0 + 2;
//  ENUM: wxPreviewFrameModalityKind
pub const PreviewFrame_AppModal: c_int = 0;
pub const PreviewFrame_WindowModal: c_int = 0 + 1;
pub const PreviewFrame_NonModal: c_int = 0 + 2;

pub const PD_CAN_ABORT: c_int = 0x0001;
pub const PD_APP_MODAL: c_int = 0x0002;
pub const PD_AUTO_HIDE: c_int = 0x0004;
pub const PD_ELAPSED_TIME: c_int = 0x0008;
pub const PD_ESTIMATED_TIME: c_int = 0x0010;
pub const PD_SMOOTH: c_int = 0x0020;
pub const PD_REMAINING_TIME: c_int = 0x0040;
pub const PD_CAN_SKIP: c_int = 0x0080;

//  ENUM: wxPropertySheetDialogFlags
pub const PROPSHEET_DEFAULT: c_int = 0x0001;
pub const PROPSHEET_NOTEBOOK: c_int = 0x0002;
pub const PROPSHEET_TOOLBOOK: c_int = 0x0004;
pub const PROPSHEET_CHOICEBOOK: c_int = 0x0008;
pub const PROPSHEET_LISTBOOK: c_int = 0x0010;
pub const PROPSHEET_BUTTONTOOLBOOK: c_int = 0x0020;
pub const PROPSHEET_TREEBOOK: c_int = 0x0040;
pub const PROPSHEET_SHRINKTOFIT: c_int = 0x0100;

pub const PG_COLOUR_WEB_BASE: c_int = 0x10000;
pub const PG_COLOUR_CUSTOM: c_int = 0xFFFFFF;
pub const PG_COLOUR_UNSPECIFIED: c_int = (PG_COLOUR_CUSTOM+1);
pub const PG_PROP_TRANSLATE_CUSTOM: c_long = PG_PROP_CLASS_SPECIFIC_1;

pub const PG_ATTR_DEFAULT_VALUE: &str = "DefaultValue";
pub const PG_ATTR_MIN: &str = "Min";
pub const PG_ATTR_MAX: &str = "Max";
pub const PG_ATTR_UNITS: &str = "Units";
pub const PG_ATTR_HINT: &str = "Hint";
pub const PG_ATTR_AUTOCOMPLETE: &str = "AutoComplete";
pub const PG_BOOL_USE_CHECKBOX: &str = "UseCheckbox";
pub const PG_BOOL_USE_DOUBLE_CLICK_CYCLING: &str = "UseDClickCycling";
pub const PG_FLOAT_PRECISION: &str = "Precision";
pub const PG_STRING_PASSWORD: &str = "Password";
pub const PG_UINT_BASE: &str = "Base";
pub const PG_UINT_PREFIX: &str = "Prefix";
pub const PG_DIALOG_TITLE: &str = "DialogTitle";
pub const PG_FILE_WILDCARD: &str = "Wildcard";
pub const PG_FILE_SHOW_FULL_PATH: &str = "ShowFullPath";
pub const PG_FILE_SHOW_RELATIVE_PATH: &str = "ShowRelativePath";
pub const PG_FILE_INITIAL_PATH: &str = "InitialPath";
pub const PG_FILE_DIALOG_STYLE: &str = "DialogStyle";
pub const PG_ARRAY_DELIMITER: &str = "Delimiter";
pub const PG_DATE_FORMAT: &str = "DateFormat";
pub const PG_DATE_PICKER_STYLE: &str = "PickerStyle";
pub const PG_ATTR_SPINCTRL_STEP: &str = "Step";
pub const PG_ATTR_SPINCTRL_WRAP: &str = "Wrap";
pub const PG_ATTR_SPINCTRL_MOTION: &str = "MotionSpin";
pub const PG_ATTR_MULTICHOICE_USERSTRINGMODE: &str = "UserStringMode";
pub const PG_COLOUR_ALLOW_CUSTOM: &str = "AllowCustom";
pub const PG_COLOUR_HAS_ALPHA: &str = "HasAlpha";
pub const PG_PROP_MAX: c_long = PG_PROP_AUTO_UNSPECIFIED;
//  SKIP: wxPG_PROP_PARENTAL_FLAGS
pub const PG_STRING_STORED_FLAGS: c_long = (PG_PROP_DISABLED|PG_PROP_HIDDEN|PG_PROP_NOEDITOR|PG_PROP_COLLAPSED);
//  SKIP: wxNullProperty
//  SKIP: wxPGChoicesEmptyData
//  ENUM: wxPGPropertyFlags
pub const PG_PROP_MODIFIED: c_long = 0x0001;
pub const PG_PROP_DISABLED: c_long = 0x0002;
pub const PG_PROP_HIDDEN: c_long = 0x0004;
pub const PG_PROP_CUSTOMIMAGE: c_long = 0x0008;
pub const PG_PROP_NOEDITOR: c_long = 0x0010;
pub const PG_PROP_COLLAPSED: c_long = 0x0020;
pub const PG_PROP_INVALID_VALUE: c_long = 0x0040;
pub const PG_PROP_WAS_MODIFIED: c_long = 0x0200;
pub const PG_PROP_AGGREGATE: c_long = 0x0400;
pub const PG_PROP_CHILDREN_ARE_COPIES: c_long = 0x0800;
pub const PG_PROP_PROPERTY: c_long = 0x1000;
pub const PG_PROP_CATEGORY: c_long = 0x2000;
pub const PG_PROP_MISC_PARENT: c_long = 0x4000;
pub const PG_PROP_READONLY: c_long = 0x8000;
pub const PG_PROP_COMPOSED_VALUE: c_long = 0x00010000;
pub const PG_PROP_USES_COMMON_VALUE: c_long = 0x00020000;
pub const PG_PROP_AUTO_UNSPECIFIED: c_long = 0x00040000;
pub const PG_PROP_CLASS_SPECIFIC_1: c_long = 0x00080000;
pub const PG_PROP_CLASS_SPECIFIC_2: c_long = 0x00100000;
//  SKIP: wxPG_PROP_BEING_DELETED

//  SKIP: wxPG_LABEL
pub const PG_LABEL_STRING: &str = "@!";
//  SKIP: wxPG_NULL_BITMAP
//  SKIP: wxPG_COLOUR_BLACK
//  SKIP: wxPG_COLOUR
//  SKIP: wxPG_DEFAULT_IMAGE_SIZE
//  SKIP: wxPG_INVALID_VALUE
pub const PG_BASE_OCT: c_int = 8;
pub const PG_BASE_DEC: c_int = 10;
pub const PG_BASE_HEX: c_int = 16;
pub const PG_BASE_HEXL: c_int = 32;
pub const PG_PREFIX_NONE: c_int = 0;
pub const PG_PREFIX_0x: c_int = 1;
pub const PG_PREFIX_DOLLAR_SIGN: c_int = 2;
//  ENUM: wxPG_GETPROPERTYVALUES_FLAGS
pub const PG_DONT_RECURSE: c_int = 0x00000000;
pub const PG_KEEP_STRUCTURE: c_int = 0x00000010;
pub const PG_RECURSE: c_int = 0x00000020;
pub const PG_INC_ATTRIBUTES: c_int = 0x00000040;
pub const PG_RECURSE_STARTS: c_int = 0x00000080;
pub const PG_FORCE: c_int = 0x00000100;
pub const PG_SORT_TOP_LEVEL_ONLY: c_int = 0x00000200;
//  ENUM: wxPG_MISC_ARG_FLAGS
pub const PG_FULL_VALUE: c_int = 0x00000001;
pub const PG_REPORT_ERROR: c_int = 0x00000002;
pub const PG_PROPERTY_SPECIFIC: c_int = 0x00000004;
pub const PG_EDITABLE_VALUE: c_int = 0x00000008;
pub const PG_COMPOSITE_FRAGMENT: c_int = 0x00000010;
pub const PG_UNEDITABLE_COMPOSITE_FRAGMENT: c_int = 0x00000020;
pub const PG_VALUE_IS_CURRENT: c_int = 0x00000040;
pub const PG_PROGRAMMATIC_VALUE: c_int = 0x00000080;
//  ENUM: wxPG_SETVALUE_FLAGS
pub const PG_SETVAL_REFRESH_EDITOR: c_int = 0x0001;
pub const PG_SETVAL_AGGREGATED: c_int = 0x0002;
pub const PG_SETVAL_FROM_PARENT: c_int = 0x0004;
pub const PG_SETVAL_BY_USER: c_int = 0x0008;

//  ENUM: wxPG_PROPERTYVALUES_FLAGS

//  SKIP: wxPG_IT_CHILDREN
//  ENUM: wxPG_ITERATOR_FLAGS
pub const PG_ITERATE_PROPERTIES: c_long = PG_PROP_PROPERTY |
                          PG_PROP_MISC_PARENT |
                          PG_PROP_AGGREGATE |
                          PG_PROP_COLLAPSED |
                          PG_IT_CHILDREN(PG_PROP_MISC_PARENT) |
                          PG_IT_CHILDREN(PG_PROP_CATEGORY);
pub const PG_ITERATE_HIDDEN: c_long = PG_PROP_HIDDEN |
                      PG_IT_CHILDREN(PG_PROP_COLLAPSED);
pub const PG_ITERATE_FIXED_CHILDREN: c_long = PG_IT_CHILDREN(PG_PROP_AGGREGATE) |
                              PG_ITERATE_PROPERTIES;
pub const PG_ITERATE_CATEGORIES: c_long = PG_PROP_CATEGORY |
                          PG_IT_CHILDREN(PG_PROP_CATEGORY) |
                          PG_PROP_COLLAPSED;
pub const PG_ITERATE_ALL_PARENTS: c_long = PG_PROP_MISC_PARENT |
                           PG_PROP_AGGREGATE |
                           PG_PROP_CATEGORY;
pub const PG_ITERATE_ALL_PARENTS_RECURSIVELY: c_long = PG_ITERATE_ALL_PARENTS |
                                       PG_IT_CHILDREN(
                                                PG_ITERATE_ALL_PARENTS);
pub const PG_ITERATOR_FLAGS_ALL: c_long = PG_PROP_PROPERTY |
                          PG_PROP_MISC_PARENT |
                          PG_PROP_AGGREGATE |
                          PG_PROP_HIDDEN |
                          PG_PROP_CATEGORY |
                          PG_PROP_COLLAPSED;
pub const PG_ITERATOR_MASK_OP_ITEM: c_long = PG_ITERATOR_FLAGS_ALL;
pub const PG_ITERATOR_MASK_OP_PARENT: c_long = PG_ITERATOR_FLAGS_ALL;
pub const PG_ITERATE_VISIBLE: c_long = PG_ITERATE_PROPERTIES |
                       PG_PROP_CATEGORY |
                       PG_IT_CHILDREN(PG_PROP_AGGREGATE);
pub const PG_ITERATE_ALL: c_long = PG_ITERATE_VISIBLE |
                   PG_ITERATE_HIDDEN;
pub const PG_ITERATE_NORMAL: c_long = PG_ITERATE_PROPERTIES |
                      PG_ITERATE_HIDDEN;
pub const PG_ITERATE_DEFAULT: c_long = PG_ITERATE_NORMAL;

pub const PG_PROP_PASSWORD: c_long = PG_PROP_CLASS_SPECIFIC_2;
pub const PG_PROP_STATIC_CHOICES: c_long = PG_PROP_CLASS_SPECIFIC_1;
pub const PG_PROP_SHOW_FULL_FILENAME: c_long = PG_PROP_CLASS_SPECIFIC_1;
pub const PG_PROP_ACTIVE_BTN: c_long = PG_PROP_CLASS_SPECIFIC_1;
pub const PG_PROP_USE_CHECKBOX: c_long = PG_PROP_CLASS_SPECIFIC_1;
pub const PG_PROP_USE_DCC: c_long = PG_PROP_CLASS_SPECIFIC_2;
pub const AEDIALOG_STYLE: c_long = (DEFAULT_DIALOG_STYLE | RESIZE_BORDER | OK | CANCEL | CENTRE);
//  ENUM: wxPGNumericValidationConstants
pub const PG_PROPERTY_VALIDATION_ERROR_MESSAGE: c_int = 0;
pub const PG_PROPERTY_VALIDATION_SATURATE: c_int = 1;
pub const PG_PROPERTY_VALIDATION_WRAP: c_int = 2;

//  ENUM: wxProtocolError
pub const PROTO_NOERR: c_int = 0;
pub const PROTO_NETERR: c_int = 0 + 1;
pub const PROTO_PROTERR: c_int = 0 + 2;
pub const PROTO_CONNERR: c_int = 0 + 3;
pub const PROTO_INVVAL: c_int = 0 + 4;
pub const PROTO_NOHNDLR: c_int = 0 + 5;
pub const PROTO_NOFILE: c_int = 0 + 6;
pub const PROTO_ABRT: c_int = 0 + 7;
pub const PROTO_RCNCT: c_int = 0 + 8;
pub const PROTO_STREAMING: c_int = 0 + 9;

//  ENUM: @41
pub const RE_EXTENDED: c_int = 0;
pub const RE_ADVANCED: c_int = 1;
pub const RE_BASIC: c_int = 2;
pub const RE_ICASE: c_int = 4;
pub const RE_NOSUB: c_int = 8;
pub const RE_NEWLINE: c_int = 16;
pub const RE_DEFAULT: c_int = RE_EXTENDED;
//  ENUM: @42
pub const RE_NOTBOL: c_int = 32;
pub const RE_NOTEOL: c_int = 64;

//  ENUM: wxRegionContain
pub const OutRegion: c_int = 0;
pub const PartRegion: c_int = 1;
pub const InRegion: c_int = 2;

//  ENUM: @43
pub const CONTROL_NONE: c_int = 0x00000000;
pub const CONTROL_DISABLED: c_int = 0x00000001;
pub const CONTROL_FOCUSED: c_int = 0x00000002;
pub const CONTROL_PRESSED: c_int = 0x00000004;
pub const CONTROL_SPECIAL: c_int = 0x00000008;
pub const CONTROL_ISDEFAULT: c_int = CONTROL_SPECIAL;
pub const CONTROL_ISSUBMENU: c_int = CONTROL_SPECIAL;
pub const CONTROL_EXPANDED: c_int = CONTROL_SPECIAL;
pub const CONTROL_SIZEGRIP: c_int = CONTROL_SPECIAL;
pub const CONTROL_FLAT: c_int = CONTROL_SPECIAL;
pub const CONTROL_CELL: c_int = CONTROL_SPECIAL;
pub const CONTROL_CURRENT: c_int = 0x00000010;
pub const CONTROL_SELECTED: c_int = 0x00000020;
pub const CONTROL_CHECKED: c_int = 0x00000040;
pub const CONTROL_CHECKABLE: c_int = 0x00000080;
pub const CONTROL_UNDETERMINED: c_int = CONTROL_CHECKABLE;
//  ENUM: wxTitleBarButton
pub const TITLEBAR_BUTTON_CLOSE: c_int = 0x01000000;
pub const TITLEBAR_BUTTON_MAXIMIZE: c_int = 0x02000000;
pub const TITLEBAR_BUTTON_ICONIZE: c_int = 0x04000000;
pub const TITLEBAR_BUTTON_RESTORE: c_int = 0x08000000;
pub const TITLEBAR_BUTTON_HELP: c_int = 0x10000000;
//  ENUM: wxHeaderSortIconType
pub const HDR_SORT_ICON_NONE: c_int = 0;
pub const HDR_SORT_ICON_UP: c_int = 0 + 1;
pub const HDR_SORT_ICON_DOWN: c_int = 0 + 2;

//  ENUM: wxRibbonArtSetting
pub const RIBBON_ART_TAB_SEPARATION_SIZE: c_int = 0;
pub const RIBBON_ART_PAGE_BORDER_LEFT_SIZE: c_int = 0 + 1;
pub const RIBBON_ART_PAGE_BORDER_TOP_SIZE: c_int = 0 + 2;
pub const RIBBON_ART_PAGE_BORDER_RIGHT_SIZE: c_int = 0 + 3;
pub const RIBBON_ART_PAGE_BORDER_BOTTOM_SIZE: c_int = 0 + 4;
pub const RIBBON_ART_PANEL_X_SEPARATION_SIZE: c_int = 0 + 5;
pub const RIBBON_ART_PANEL_Y_SEPARATION_SIZE: c_int = 0 + 6;
pub const RIBBON_ART_TOOL_GROUP_SEPARATION_SIZE: c_int = 0 + 7;
pub const RIBBON_ART_GALLERY_BITMAP_PADDING_LEFT_SIZE: c_int = 0 + 8;
pub const RIBBON_ART_GALLERY_BITMAP_PADDING_RIGHT_SIZE: c_int = 0 + 9;
pub const RIBBON_ART_GALLERY_BITMAP_PADDING_TOP_SIZE: c_int = 0 + 10;
pub const RIBBON_ART_GALLERY_BITMAP_PADDING_BOTTOM_SIZE: c_int = 0 + 11;
pub const RIBBON_ART_PANEL_LABEL_FONT: c_int = 0 + 12;
pub const RIBBON_ART_BUTTON_BAR_LABEL_FONT: c_int = 0 + 13;
pub const RIBBON_ART_TAB_LABEL_FONT: c_int = 0 + 14;
pub const RIBBON_ART_BUTTON_BAR_LABEL_COLOUR: c_int = 0 + 15;
pub const RIBBON_ART_BUTTON_BAR_LABEL_DISABLED_COLOUR: c_int = 0 + 16;
pub const RIBBON_ART_BUTTON_BAR_HOVER_BORDER_COLOUR: c_int = 0 + 17;
pub const RIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 18;
pub const RIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 19;
pub const RIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_COLOUR: c_int = 0 + 20;
pub const RIBBON_ART_BUTTON_BAR_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 21;
pub const RIBBON_ART_BUTTON_BAR_ACTIVE_BORDER_COLOUR: c_int = 0 + 22;
pub const RIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 23;
pub const RIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 24;
pub const RIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 25;
pub const RIBBON_ART_BUTTON_BAR_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 26;
pub const RIBBON_ART_GALLERY_BORDER_COLOUR: c_int = 0 + 27;
pub const RIBBON_ART_GALLERY_HOVER_BACKGROUND_COLOUR: c_int = 0 + 28;
pub const RIBBON_ART_GALLERY_BUTTON_BACKGROUND_COLOUR: c_int = 0 + 29;
pub const RIBBON_ART_GALLERY_BUTTON_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 30;
pub const RIBBON_ART_GALLERY_BUTTON_BACKGROUND_TOP_COLOUR: c_int = 0 + 31;
pub const RIBBON_ART_GALLERY_BUTTON_FACE_COLOUR: c_int = 0 + 32;
pub const RIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_COLOUR: c_int = 0 + 33;
pub const RIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 34;
pub const RIBBON_ART_GALLERY_BUTTON_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 35;
pub const RIBBON_ART_GALLERY_BUTTON_HOVER_FACE_COLOUR: c_int = 0 + 36;
pub const RIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 37;
pub const RIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 38;
pub const RIBBON_ART_GALLERY_BUTTON_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 39;
pub const RIBBON_ART_GALLERY_BUTTON_ACTIVE_FACE_COLOUR: c_int = 0 + 40;
pub const RIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_COLOUR: c_int = 0 + 41;
pub const RIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 42;
pub const RIBBON_ART_GALLERY_BUTTON_DISABLED_BACKGROUND_TOP_COLOUR: c_int = 0 + 43;
pub const RIBBON_ART_GALLERY_BUTTON_DISABLED_FACE_COLOUR: c_int = 0 + 44;
pub const RIBBON_ART_GALLERY_ITEM_BORDER_COLOUR: c_int = 0 + 45;
pub const RIBBON_ART_TAB_LABEL_COLOUR: c_int = 0 + 46;
pub const RIBBON_ART_TAB_ACTIVE_LABEL_COLOUR: c_int = 0 + 47;
pub const RIBBON_ART_TAB_HOVER_LABEL_COLOUR: c_int = 0 + 48;
pub const RIBBON_ART_TAB_SEPARATOR_COLOUR: c_int = 0 + 49;
pub const RIBBON_ART_TAB_SEPARATOR_GRADIENT_COLOUR: c_int = 0 + 50;
pub const RIBBON_ART_TAB_CTRL_BACKGROUND_COLOUR: c_int = 0 + 51;
pub const RIBBON_ART_TAB_CTRL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 52;
pub const RIBBON_ART_TAB_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 53;
pub const RIBBON_ART_TAB_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 54;
pub const RIBBON_ART_TAB_HOVER_BACKGROUND_COLOUR: c_int = 0 + 55;
pub const RIBBON_ART_TAB_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 56;
pub const RIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 57;
pub const RIBBON_ART_TAB_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 58;
pub const RIBBON_ART_TAB_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 59;
pub const RIBBON_ART_TAB_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 60;
pub const RIBBON_ART_TAB_BORDER_COLOUR: c_int = 0 + 61;
pub const RIBBON_ART_PANEL_BORDER_COLOUR: c_int = 0 + 62;
pub const RIBBON_ART_PANEL_BORDER_GRADIENT_COLOUR: c_int = 0 + 63;
pub const RIBBON_ART_PANEL_HOVER_BORDER_COLOUR: c_int = 0 + 64;
pub const RIBBON_ART_PANEL_HOVER_BORDER_GRADIENT_COLOUR: c_int = 0 + 65;
pub const RIBBON_ART_PANEL_MINIMISED_BORDER_COLOUR: c_int = 0 + 66;
pub const RIBBON_ART_PANEL_MINIMISED_BORDER_GRADIENT_COLOUR: c_int = 0 + 67;
pub const RIBBON_ART_PANEL_LABEL_BACKGROUND_COLOUR: c_int = 0 + 68;
pub const RIBBON_ART_PANEL_LABEL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 69;
pub const RIBBON_ART_PANEL_LABEL_COLOUR: c_int = 0 + 70;
pub const RIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_COLOUR: c_int = 0 + 71;
pub const RIBBON_ART_PANEL_HOVER_LABEL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 72;
pub const RIBBON_ART_PANEL_HOVER_LABEL_COLOUR: c_int = 0 + 73;
pub const RIBBON_ART_PANEL_MINIMISED_LABEL_COLOUR: c_int = 0 + 74;
pub const RIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 75;
pub const RIBBON_ART_PANEL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 76;
pub const RIBBON_ART_PANEL_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 77;
pub const RIBBON_ART_PANEL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 78;
pub const RIBBON_ART_PAGE_BORDER_COLOUR: c_int = 0 + 79;
pub const RIBBON_ART_PAGE_BACKGROUND_TOP_COLOUR: c_int = 0 + 80;
pub const RIBBON_ART_PAGE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 81;
pub const RIBBON_ART_PAGE_BACKGROUND_COLOUR: c_int = 0 + 82;
pub const RIBBON_ART_PAGE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 83;
pub const RIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 84;
pub const RIBBON_ART_PAGE_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 85;
pub const RIBBON_ART_PAGE_HOVER_BACKGROUND_COLOUR: c_int = 0 + 86;
pub const RIBBON_ART_PAGE_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 87;
pub const RIBBON_ART_TOOLBAR_BORDER_COLOUR: c_int = 0 + 88;
pub const RIBBON_ART_TOOLBAR_HOVER_BORDER_COLOUR: c_int = 0 + 89;
pub const RIBBON_ART_TOOLBAR_FACE_COLOUR: c_int = 0 + 90;
pub const RIBBON_ART_TOOL_BACKGROUND_TOP_COLOUR: c_int = 0 + 91;
pub const RIBBON_ART_TOOL_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 92;
pub const RIBBON_ART_TOOL_BACKGROUND_COLOUR: c_int = 0 + 93;
pub const RIBBON_ART_TOOL_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 94;
pub const RIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_COLOUR: c_int = 0 + 95;
pub const RIBBON_ART_TOOL_HOVER_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 96;
pub const RIBBON_ART_TOOL_HOVER_BACKGROUND_COLOUR: c_int = 0 + 97;
pub const RIBBON_ART_TOOL_HOVER_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 98;
pub const RIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_COLOUR: c_int = 0 + 99;
pub const RIBBON_ART_TOOL_ACTIVE_BACKGROUND_TOP_GRADIENT_COLOUR: c_int = 0 + 100;
pub const RIBBON_ART_TOOL_ACTIVE_BACKGROUND_COLOUR: c_int = 0 + 101;
pub const RIBBON_ART_TOOL_ACTIVE_BACKGROUND_GRADIENT_COLOUR: c_int = 0 + 102;
pub const RIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_COLOUR: c_int = 0 + 103;
pub const RIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_GRADIENT_COLOUR: c_int = 0 + 104;
pub const RIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_TOP_COLOUR: c_int = 0 + 105;
pub const RIBBON_ART_BUTTON_BAR_LABEL_HIGHLIGHT_GRADIENT_TOP_COLOUR: c_int = 0 + 106;
//  ENUM: wxRibbonScrollButtonStyle
pub const RIBBON_SCROLL_BTN_LEFT: c_int = 0;
pub const RIBBON_SCROLL_BTN_RIGHT: c_int = 1;
pub const RIBBON_SCROLL_BTN_UP: c_int = 2;
pub const RIBBON_SCROLL_BTN_DOWN: c_int = 3;
pub const RIBBON_SCROLL_BTN_DIRECTION_MASK: c_int = 3;
pub const RIBBON_SCROLL_BTN_NORMAL: c_int = 0;
pub const RIBBON_SCROLL_BTN_HOVERED: c_int = 4;
pub const RIBBON_SCROLL_BTN_ACTIVE: c_int = 8;
pub const RIBBON_SCROLL_BTN_STATE_MASK: c_int = 12;
pub const RIBBON_SCROLL_BTN_FOR_OTHER: c_int = 0;
pub const RIBBON_SCROLL_BTN_FOR_TABS: c_int = 16;
pub const RIBBON_SCROLL_BTN_FOR_PAGE: c_int = 32;
pub const RIBBON_SCROLL_BTN_FOR_MASK: c_int = 48;
//  ENUM: wxRibbonButtonKind
pub const RIBBON_BUTTON_NORMAL: c_int = 1 << 0;
pub const RIBBON_BUTTON_DROPDOWN: c_int = 1 << 1;
pub const RIBBON_BUTTON_HYBRID: c_int = RIBBON_BUTTON_NORMAL | RIBBON_BUTTON_DROPDOWN;
pub const RIBBON_BUTTON_TOGGLE: c_int = 1 << 2;

//  ENUM: wxRibbonBarOption
pub const RIBBON_BAR_SHOW_PAGE_LABELS: c_int = 0;
pub const RIBBON_BAR_SHOW_PAGE_ICONS: c_int = 0 + 1;
pub const RIBBON_BAR_FLOW_HORIZONTAL: c_int = 0 + 2;
pub const RIBBON_BAR_FLOW_VERTICAL: c_int = 0 + 3;
pub const RIBBON_BAR_SHOW_PANEL_EXT_BUTTONS: c_int = 0 + 4;
pub const RIBBON_BAR_SHOW_PANEL_MINIMISE_BUTTONS: c_int = 0 + 5;
pub const RIBBON_BAR_ALWAYS_SHOW_TABS: c_int = 0 + 6;
pub const RIBBON_BAR_SHOW_TOGGLE_BUTTON: c_int = 0 + 7;
pub const RIBBON_BAR_SHOW_HELP_BUTTON: c_int = 0 + 8;
pub const RIBBON_BAR_DEFAULT_STYLE: c_int = 0 + 9;
pub const RIBBON_BAR_FOLDBAR_STYLE: c_int = 0 + 10;
//  ENUM: wxRibbonDisplayMode
pub const RIBBON_BAR_PINNED: c_int = 0;
pub const RIBBON_BAR_MINIMIZED: c_int = 0 + 1;
pub const RIBBON_BAR_EXPANDED: c_int = 0 + 2;

//  ENUM: wxRibbonButtonBarButtonState
pub const RIBBON_BUTTONBAR_BUTTON_SMALL: c_int = 0 << 0;
pub const RIBBON_BUTTONBAR_BUTTON_MEDIUM: c_int = 1 << 0;
pub const RIBBON_BUTTONBAR_BUTTON_LARGE: c_int = 2 << 0;
pub const RIBBON_BUTTONBAR_BUTTON_SIZE_MASK: c_int = 3 << 0;
pub const RIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED: c_int = 1 << 3;
pub const RIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED: c_int = 1 << 4;
pub const RIBBON_BUTTONBAR_BUTTON_HOVER_MASK: c_int = RIBBON_BUTTONBAR_BUTTON_NORMAL_HOVERED | RIBBON_BUTTONBAR_BUTTON_DROPDOWN_HOVERED;
pub const RIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE: c_int = 1 << 5;
pub const RIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE: c_int = 1 << 6;
pub const RIBBON_BUTTONBAR_BUTTON_ACTIVE_MASK: c_int = RIBBON_BUTTONBAR_BUTTON_NORMAL_ACTIVE | RIBBON_BUTTONBAR_BUTTON_DROPDOWN_ACTIVE;
pub const RIBBON_BUTTONBAR_BUTTON_DISABLED: c_int = 1 << 7;
pub const RIBBON_BUTTONBAR_BUTTON_TOGGLED: c_int = 1 << 8;
pub const RIBBON_BUTTONBAR_BUTTON_STATE_MASK: c_int = 0x1F8;

//  ENUM: wxRibbonGalleryButtonState
pub const RIBBON_GALLERY_BUTTON_NORMAL: c_int = 0;
pub const RIBBON_GALLERY_BUTTON_HOVERED: c_int = 0 + 1;
pub const RIBBON_GALLERY_BUTTON_ACTIVE: c_int = 0 + 2;
pub const RIBBON_GALLERY_BUTTON_DISABLED: c_int = 0 + 3;

//  ENUM: wxRibbonPanelOption
pub const RIBBON_PANEL_NO_AUTO_MINIMISE: c_int = 0;
pub const RIBBON_PANEL_EXT_BUTTON: c_int = 0 + 1;
pub const RIBBON_PANEL_MINIMISE_BUTTON: c_int = 0 + 2;
pub const RIBBON_PANEL_STRETCH: c_int = 0 + 3;
pub const RIBBON_PANEL_FLEXIBLE: c_int = 0 + 4;
pub const RIBBON_PANEL_DEFAULT_STYLE: c_int = 0 + 5;

pub const RICHTEXT_FIXED_WIDTH: c_int = 0x01;
pub const RICHTEXT_FIXED_HEIGHT: c_int = 0x02;
pub const RICHTEXT_VARIABLE_WIDTH: c_int = 0x04;
pub const RICHTEXT_VARIABLE_HEIGHT: c_int = 0x08;
pub const RICHTEXT_LAYOUT_SPECIFIED_RECT: c_int = 0x10;
pub const RICHTEXT_DRAW_IGNORE_CACHE: c_int = 0x01;
pub const RICHTEXT_DRAW_SELECTED: c_int = 0x02;
pub const RICHTEXT_DRAW_PRINT: c_int = 0x04;
pub const RICHTEXT_DRAW_GUIDELINES: c_int = 0x08;
pub const RICHTEXT_FORMATTED: c_int = 0x01;
pub const RICHTEXT_UNFORMATTED: c_int = 0x02;
pub const RICHTEXT_CACHE_SIZE: c_int = 0x04;
pub const RICHTEXT_HEIGHT_ONLY: c_int = 0x08;
pub const RICHTEXT_SETSTYLE_NONE: c_int = 0x00;
pub const RICHTEXT_SETSTYLE_WITH_UNDO: c_int = 0x01;
pub const RICHTEXT_SETSTYLE_OPTIMIZE: c_int = 0x02;
pub const RICHTEXT_SETSTYLE_PARAGRAPHS_ONLY: c_int = 0x04;
pub const RICHTEXT_SETSTYLE_CHARACTERS_ONLY: c_int = 0x08;
pub const RICHTEXT_SETSTYLE_RENUMBER: c_int = 0x10;
pub const RICHTEXT_SETSTYLE_SPECIFY_LEVEL: c_int = 0x20;
pub const RICHTEXT_SETSTYLE_RESET: c_int = 0x40;
pub const RICHTEXT_SETSTYLE_REMOVE: c_int = 0x80;
pub const RICHTEXT_SETPROPERTIES_NONE: c_int = 0x00;
pub const RICHTEXT_SETPROPERTIES_WITH_UNDO: c_int = 0x01;
pub const RICHTEXT_SETPROPERTIES_PARAGRAPHS_ONLY: c_int = 0x02;
pub const RICHTEXT_SETPROPERTIES_CHARACTERS_ONLY: c_int = 0x04;
pub const RICHTEXT_SETPROPERTIES_RESET: c_int = 0x08;
pub const RICHTEXT_SETPROPERTIES_REMOVE: c_int = 0x10;
pub const RICHTEXT_INSERT_NONE: c_int = 0x00;
pub const RICHTEXT_INSERT_WITH_PREVIOUS_PARAGRAPH_STYLE: c_int = 0x01;
pub const RICHTEXT_INSERT_INTERACTIVE: c_int = 0x02;
pub const TEXT_ATTR_KEEP_FIRST_PARA_STYLE: c_int = 0x10000000;
pub const SCRIPT_MUL_FACTOR: f32 = 1.5;
//  SKIP: wxRICHTEXT_ALL
//  SKIP: wxRICHTEXT_NONE
//  SKIP: wxRICHTEXT_NO_SELECTION
pub const RICHTEXT_HANDLER_INCLUDE_STYLESHEET: c_int = 0x0001;
pub const RICHTEXT_HANDLER_SAVE_IMAGES_TO_MEMORY: c_int = 0x0010;
pub const RICHTEXT_HANDLER_SAVE_IMAGES_TO_FILES: c_int = 0x0020;
pub const RICHTEXT_HANDLER_SAVE_IMAGES_TO_BASE64: c_int = 0x0040;
pub const RICHTEXT_HANDLER_NO_HEADER_FOOTER: c_int = 0x0080;
pub const RICHTEXT_HANDLER_CONVERT_FACENAMES: c_int = 0x0100;
pub const RICHTEXT_HANDLER_USE_CSS: c_int = 0x1000;
//  ENUM: wxRichTextFileType
pub const RICHTEXT_TYPE_ANY: c_int = 0;
pub const RICHTEXT_TYPE_TEXT: c_int = 0 + 1;
pub const RICHTEXT_TYPE_XML: c_int = 0 + 2;
pub const RICHTEXT_TYPE_HTML: c_int = 0 + 3;
pub const RICHTEXT_TYPE_RTF: c_int = 0 + 4;
pub const RICHTEXT_TYPE_PDF: c_int = 0 + 5;
//  ENUM: wxRichTextHitTestFlags
pub const RICHTEXT_HITTEST_NONE: c_int =    0x01;
pub const RICHTEXT_HITTEST_BEFORE: c_int =  0x02;
pub const RICHTEXT_HITTEST_AFTER: c_int =   0x04;
pub const RICHTEXT_HITTEST_ON: c_int =      0x08;
pub const RICHTEXT_HITTEST_OUTSIDE: c_int = 0x10;
pub const RICHTEXT_HITTEST_NO_NESTED_OBJECTS: c_int = 0x20;
pub const RICHTEXT_HITTEST_NO_FLOATING_OBJECTS: c_int = 0x40;
pub const RICHTEXT_HITTEST_HONOUR_ATOMIC: c_int = 0x80;
//  ENUM: wxTextBoxAttrFlags
pub const TEXT_BOX_ATTR_FLOAT: c_int = 0x00000001;
pub const TEXT_BOX_ATTR_CLEAR: c_int = 0x00000002;
pub const TEXT_BOX_ATTR_COLLAPSE_BORDERS: c_int = 0x00000004;
pub const TEXT_BOX_ATTR_VERTICAL_ALIGNMENT: c_int = 0x00000008;
pub const TEXT_BOX_ATTR_BOX_STYLE_NAME: c_int = 0x00000010;
pub const TEXT_BOX_ATTR_WHITESPACE: c_int = 0x00000020;
pub const TEXT_BOX_ATTR_CORNER_RADIUS: c_int = 0x00000040;
//  ENUM: wxTextAttrValueFlags
pub const TEXT_ATTR_VALUE_VALID: c_int = 0x1000;
pub const TEXT_ATTR_VALUE_VALID_MASK: c_int = 0x1000;
//  ENUM: wxTextAttrUnits
pub const TEXT_ATTR_UNITS_TENTHS_MM: c_int = 0x0001;
pub const TEXT_ATTR_UNITS_PIXELS: c_int = 0x0002;
pub const TEXT_ATTR_UNITS_PERCENTAGE: c_int = 0x0004;
pub const TEXT_ATTR_UNITS_POINTS: c_int = 0x0008;
pub const TEXT_ATTR_UNITS_HUNDREDTHS_POINT: c_int = 0x0100;
pub const TEXT_ATTR_UNITS_MASK: c_int = 0x010F;
//  ENUM: wxTextBoxAttrPosition
pub const TEXT_BOX_ATTR_POSITION_STATIC: c_int = 0x0000;
pub const TEXT_BOX_ATTR_POSITION_RELATIVE: c_int = 0x0010;
pub const TEXT_BOX_ATTR_POSITION_ABSOLUTE: c_int = 0x0020;
pub const TEXT_BOX_ATTR_POSITION_FIXED: c_int = 0x0040;
pub const TEXT_BOX_ATTR_POSITION_MASK: c_int = 0x00F0;
//  ENUM: wxTextAttrBorderStyle
pub const TEXT_BOX_ATTR_BORDER_NONE: c_int = 0;
pub const TEXT_BOX_ATTR_BORDER_SOLID: c_int = 1;
pub const TEXT_BOX_ATTR_BORDER_DOTTED: c_int = 2;
pub const TEXT_BOX_ATTR_BORDER_DASHED: c_int = 3;
pub const TEXT_BOX_ATTR_BORDER_DOUBLE: c_int = 4;
pub const TEXT_BOX_ATTR_BORDER_GROOVE: c_int = 5;
pub const TEXT_BOX_ATTR_BORDER_RIDGE: c_int = 6;
pub const TEXT_BOX_ATTR_BORDER_INSET: c_int = 7;
pub const TEXT_BOX_ATTR_BORDER_OUTSET: c_int = 8;
//  ENUM: wxTextAttrBorderFlags
pub const TEXT_BOX_ATTR_BORDER_STYLE: c_int = 0x0001;
pub const TEXT_BOX_ATTR_BORDER_COLOUR: c_int = 0x0002;
//  ENUM: wxTextAttrBorderWidth
pub const TEXT_BOX_ATTR_BORDER_THIN: c_int = -1;
pub const TEXT_BOX_ATTR_BORDER_MEDIUM: c_int = -2;
pub const TEXT_BOX_ATTR_BORDER_THICK: c_int = -3;
//  ENUM: wxTextBoxAttrFloatStyle
pub const TEXT_BOX_ATTR_FLOAT_NONE: c_int = 0;
pub const TEXT_BOX_ATTR_FLOAT_LEFT: c_int = 1;
pub const TEXT_BOX_ATTR_FLOAT_RIGHT: c_int = 2;
//  ENUM: wxTextBoxAttrClearStyle
pub const TEXT_BOX_ATTR_CLEAR_NONE: c_int = 0;
pub const TEXT_BOX_ATTR_CLEAR_LEFT: c_int = 1;
pub const TEXT_BOX_ATTR_CLEAR_RIGHT: c_int = 2;
pub const TEXT_BOX_ATTR_CLEAR_BOTH: c_int = 3;
//  ENUM: wxTextBoxAttrCollapseMode
pub const TEXT_BOX_ATTR_COLLAPSE_NONE: c_int = 0;
pub const TEXT_BOX_ATTR_COLLAPSE_FULL: c_int = 1;
//  ENUM: wxTextBoxAttrVerticalAlignment
pub const TEXT_BOX_ATTR_VERTICAL_ALIGNMENT_NONE: c_int =       0;
pub const TEXT_BOX_ATTR_VERTICAL_ALIGNMENT_TOP: c_int =       1;
pub const TEXT_BOX_ATTR_VERTICAL_ALIGNMENT_CENTRE: c_int =     2;
pub const TEXT_BOX_ATTR_VERTICAL_ALIGNMENT_BOTTOM: c_int =    3;
//  ENUM: wxTextBoxAttrWhitespaceMode
pub const TEXT_BOX_ATTR_WHITESPACE_NONE: c_int = 0;
pub const TEXT_BOX_ATTR_WHITESPACE_NORMAL: c_int = 1;
pub const TEXT_BOX_ATTR_WHITESPACE_NO_WRAP: c_int = 2;
pub const TEXT_BOX_ATTR_WHITESPACE_PREFORMATTED: c_int = 3;
pub const TEXT_BOX_ATTR_WHITESPACE_PREFORMATTED_LINE: c_int = 4;
pub const TEXT_BOX_ATTR_WHITESPACE_PREFORMATTED_WRAP: c_int = 5;
//  ENUM: wxRichTextCommandId
pub const RICHTEXT_INSERT: c_int = 0;
pub const RICHTEXT_DELETE: c_int = 0 + 1;
pub const RICHTEXT_CHANGE_ATTRIBUTES: c_int = 0 + 2;
pub const RICHTEXT_CHANGE_STYLE: c_int = 0 + 3;
pub const RICHTEXT_CHANGE_OBJECT: c_int = 0 + 4;

pub const RICHTEXT_FORMAT_STYLE_EDITOR: c_int = 0x0001;
pub const RICHTEXT_FORMAT_FONT: c_int = 0x0002;
pub const RICHTEXT_FORMAT_TABS: c_int = 0x0004;
pub const RICHTEXT_FORMAT_BULLETS: c_int = 0x0008;
pub const RICHTEXT_FORMAT_INDENTS_SPACING: c_int = 0x0010;

//  ENUM: wxRichTextOddEvenPage
pub const RICHTEXT_PAGE_ODD: c_int = 0;
pub const RICHTEXT_PAGE_EVEN: c_int = 0 + 1;
pub const RICHTEXT_PAGE_ALL: c_int = 0 + 2;
//  ENUM: wxRichTextPageLocation
pub const RICHTEXT_PAGE_LEFT: c_int = 0;
pub const RICHTEXT_PAGE_CENTRE: c_int = 0 + 1;
pub const RICHTEXT_PAGE_RIGHT: c_int = 0 + 2;

pub const RICHTEXT_ORGANISER_DELETE_STYLES: c_int = 0x0001;
pub const RICHTEXT_ORGANISER_CREATE_STYLES: c_int = 0x0002;
pub const RICHTEXT_ORGANISER_APPLY_STYLES: c_int = 0x0004;
pub const RICHTEXT_ORGANISER_EDIT_STYLES: c_int = 0x0008;
pub const RICHTEXT_ORGANISER_RENAME_STYLES: c_int = 0x0010;
pub const RICHTEXT_ORGANISER_OK_CANCEL: c_int = 0x0020;
pub const RICHTEXT_ORGANISER_RENUMBER: c_int = 0x0040;
pub const RICHTEXT_ORGANISER_SHOW_CHARACTER: c_int = 0x0100;
pub const RICHTEXT_ORGANISER_SHOW_PARAGRAPH: c_int = 0x0200;
pub const RICHTEXT_ORGANISER_SHOW_LIST: c_int = 0x0400;
pub const RICHTEXT_ORGANISER_SHOW_BOX: c_int = 0x0800;
pub const RICHTEXT_ORGANISER_SHOW_ALL: c_int = 0x1000;
pub const RICHTEXT_ORGANISER_ORGANISE: c_int = (RICHTEXT_ORGANISER_SHOW_ALL|RICHTEXT_ORGANISER_DELETE_STYLES|RICHTEXT_ORGANISER_CREATE_STYLES|RICHTEXT_ORGANISER_APPLY_STYLES|RICHTEXT_ORGANISER_EDIT_STYLES|RICHTEXT_ORGANISER_RENAME_STYLES);
pub const RICHTEXT_ORGANISER_BROWSE: c_int = (RICHTEXT_ORGANISER_SHOW_ALL|RICHTEXT_ORGANISER_OK_CANCEL);
pub const RICHTEXT_ORGANISER_BROWSE_NUMBERING: c_int = (RICHTEXT_ORGANISER_SHOW_LIST|RICHTEXT_ORGANISER_OK_CANCEL|RICHTEXT_ORGANISER_RENUMBER);

//  ENUM: wxTipKind
pub const TipKind_None: c_int = 0;
pub const TipKind_TopLeft: c_int = 0 + 1;
pub const TipKind_Top: c_int = 0 + 2;
pub const TipKind_TopRight: c_int = 0 + 3;
pub const TipKind_BottomLeft: c_int = 0 + 4;
pub const TipKind_Bottom: c_int = 0 + 5;
pub const TipKind_BottomRight: c_int = 0 + 6;
pub const TipKind_Auto: c_int = 0 + 7;

pub const SW_NOBORDER: c_int = 0x0000;
pub const SW_BORDER: c_int = 0x0020;
pub const SW_3DSASH: c_int = 0x0040;
pub const SW_3DBORDER: c_int = 0x0080;
pub const SW_3D: c_int = (SW_3DSASH | SW_3DBORDER);
//  ENUM: wxSashEdgePosition
pub const SASH_TOP: c_int = 0;
pub const SASH_RIGHT: c_int = 0 + 1;
pub const SASH_BOTTOM: c_int = 0 + 2;
pub const SASH_LEFT: c_int = 0 + 3;
pub const SASH_NONE: c_int = 100;
//  ENUM: wxSashDragStatus
pub const SASH_STATUS_OK: c_int = 0;
pub const SASH_STATUS_OUT_OF_RANGE: c_int = 0 + 1;

//  ENUM: wxIPCFormat

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

//  ENUM: wxScrollbarVisibility
pub const SHOW_SB_NEVER: c_int = -1;
pub const SHOW_SB_DEFAULT: c_int = -1 + 1;
pub const SHOW_SB_ALWAYS: c_int = -1 + 2;

//  ENUM: wxSystemFont
pub const SYS_OEM_FIXED_FONT: c_int = 10;
pub const SYS_ANSI_FIXED_FONT: c_int = 10 + 1;
pub const SYS_ANSI_VAR_FONT: c_int = 10 + 2;
pub const SYS_SYSTEM_FONT: c_int = 10 + 3;
pub const SYS_DEVICE_DEFAULT_FONT: c_int = 10 + 4;
pub const SYS_DEFAULT_GUI_FONT: c_int = 10 + 5;
//  ENUM: wxSystemColour
pub const SYS_COLOUR_SCROLLBAR: c_int = 0;
pub const SYS_COLOUR_DESKTOP: c_int = 0 + 1;
pub const SYS_COLOUR_ACTIVECAPTION: c_int = 0 + 2;
pub const SYS_COLOUR_INACTIVECAPTION: c_int = 0 + 3;
pub const SYS_COLOUR_MENU: c_int = 0 + 4;
pub const SYS_COLOUR_WINDOW: c_int = 0 + 5;
pub const SYS_COLOUR_WINDOWFRAME: c_int = 0 + 6;
pub const SYS_COLOUR_MENUTEXT: c_int = 0 + 7;
pub const SYS_COLOUR_WINDOWTEXT: c_int = 0 + 8;
pub const SYS_COLOUR_CAPTIONTEXT: c_int = 0 + 9;
pub const SYS_COLOUR_ACTIVEBORDER: c_int = 0 + 10;
pub const SYS_COLOUR_INACTIVEBORDER: c_int = 0 + 11;
pub const SYS_COLOUR_APPWORKSPACE: c_int = 0 + 12;
pub const SYS_COLOUR_HIGHLIGHT: c_int = 0 + 13;
pub const SYS_COLOUR_HIGHLIGHTTEXT: c_int = 0 + 14;
pub const SYS_COLOUR_BTNFACE: c_int = 0 + 15;
pub const SYS_COLOUR_BTNSHADOW: c_int = 0 + 16;
pub const SYS_COLOUR_GRAYTEXT: c_int = 0 + 17;
pub const SYS_COLOUR_BTNTEXT: c_int = 0 + 18;
pub const SYS_COLOUR_INACTIVECAPTIONTEXT: c_int = 0 + 19;
pub const SYS_COLOUR_BTNHIGHLIGHT: c_int = 0 + 20;
pub const SYS_COLOUR_3DDKSHADOW: c_int = 0 + 21;
pub const SYS_COLOUR_3DLIGHT: c_int = 0 + 22;
pub const SYS_COLOUR_INFOTEXT: c_int = 0 + 23;
pub const SYS_COLOUR_INFOBK: c_int = 0 + 24;
pub const SYS_COLOUR_LISTBOX: c_int = 0 + 25;
pub const SYS_COLOUR_HOTLIGHT: c_int = 0 + 26;
pub const SYS_COLOUR_GRADIENTACTIVECAPTION: c_int = 0 + 27;
pub const SYS_COLOUR_GRADIENTINACTIVECAPTION: c_int = 0 + 28;
pub const SYS_COLOUR_MENUHILIGHT: c_int = 0 + 29;
pub const SYS_COLOUR_MENUBAR: c_int = 0 + 30;
pub const SYS_COLOUR_LISTBOXTEXT: c_int = 0 + 31;
pub const SYS_COLOUR_LISTBOXHIGHLIGHTTEXT: c_int = 0 + 32;
pub const SYS_COLOUR_BACKGROUND: c_int = SYS_COLOUR_DESKTOP;
pub const SYS_COLOUR_3DFACE: c_int = SYS_COLOUR_BTNFACE;
pub const SYS_COLOUR_3DSHADOW: c_int = SYS_COLOUR_BTNSHADOW;
pub const SYS_COLOUR_BTNHILIGHT: c_int = SYS_COLOUR_BTNHIGHLIGHT;
pub const SYS_COLOUR_3DHIGHLIGHT: c_int = SYS_COLOUR_BTNHIGHLIGHT;
pub const SYS_COLOUR_3DHILIGHT: c_int = SYS_COLOUR_BTNHIGHLIGHT;
pub const SYS_COLOUR_FRAMEBK: c_int = SYS_COLOUR_BTNFACE;
//  ENUM: wxSystemMetric
pub const SYS_MOUSE_BUTTONS: c_int = 0;
pub const SYS_BORDER_X: c_int = 0 + 1;
pub const SYS_BORDER_Y: c_int = 0 + 2;
pub const SYS_CURSOR_X: c_int = 0 + 3;
pub const SYS_CURSOR_Y: c_int = 0 + 4;
pub const SYS_DCLICK_X: c_int = 0 + 5;
pub const SYS_DCLICK_Y: c_int = 0 + 6;
pub const SYS_DRAG_X: c_int = 0 + 7;
pub const SYS_DRAG_Y: c_int = 0 + 8;
pub const SYS_EDGE_X: c_int = 0 + 9;
pub const SYS_EDGE_Y: c_int = 0 + 10;
pub const SYS_HSCROLL_ARROW_X: c_int = 0 + 11;
pub const SYS_HSCROLL_ARROW_Y: c_int = 0 + 12;
pub const SYS_HTHUMB_X: c_int = 0 + 13;
pub const SYS_ICON_X: c_int = 0 + 14;
pub const SYS_ICON_Y: c_int = 0 + 15;
pub const SYS_ICONSPACING_X: c_int = 0 + 16;
pub const SYS_ICONSPACING_Y: c_int = 0 + 17;
pub const SYS_WINDOWMIN_X: c_int = 0 + 18;
pub const SYS_WINDOWMIN_Y: c_int = 0 + 19;
pub const SYS_SCREEN_X: c_int = 0 + 20;
pub const SYS_SCREEN_Y: c_int = 0 + 21;
pub const SYS_FRAMESIZE_X: c_int = 0 + 22;
pub const SYS_FRAMESIZE_Y: c_int = 0 + 23;
pub const SYS_SMALLICON_X: c_int = 0 + 24;
pub const SYS_SMALLICON_Y: c_int = 0 + 25;
pub const SYS_HSCROLL_Y: c_int = 0 + 26;
pub const SYS_VSCROLL_X: c_int = 0 + 27;
pub const SYS_VSCROLL_ARROW_X: c_int = 0 + 28;
pub const SYS_VSCROLL_ARROW_Y: c_int = 0 + 29;
pub const SYS_VTHUMB_Y: c_int = 0 + 30;
pub const SYS_CAPTION_Y: c_int = 0 + 31;
pub const SYS_MENU_Y: c_int = 0 + 32;
pub const SYS_NETWORK_PRESENT: c_int = 0 + 33;
pub const SYS_PENWINDOWS_PRESENT: c_int = 0 + 34;
pub const SYS_SHOW_SOUNDS: c_int = 0 + 35;
pub const SYS_SWAP_BUTTONS: c_int = 0 + 36;
pub const SYS_DCLICK_MSEC: c_int = 0 + 37;
pub const SYS_CARET_ON_MSEC: c_int = 0 + 38;
pub const SYS_CARET_OFF_MSEC: c_int = 0 + 39;
pub const SYS_CARET_TIMEOUT_MSEC: c_int = 0 + 40;
//  ENUM: wxSystemFeature
pub const SYS_CAN_DRAW_FRAME_DECORATIONS: c_int = 1;
pub const SYS_CAN_ICONIZE_FRAME: c_int = 1 + 1;
pub const SYS_TABLET_PRESENT: c_int = 1 + 2;
//  ENUM: wxSystemScreenType
pub const SYS_SCREEN_NONE: c_int = 0;
pub const SYS_SCREEN_TINY: c_int = 0 + 1;
pub const SYS_SCREEN_PDA: c_int = 0 + 2;
pub const SYS_SCREEN_SMALL: c_int = 0 + 3;
pub const SYS_SCREEN_DESKTOP: c_int = 0 + 4;

pub const SL_HORIZONTAL: c_int = HORIZONTAL /* 0x0004 */;
pub const SL_VERTICAL: c_int = VERTICAL   /* 0x0008 */;
pub const SL_TICKS: c_int = 0x0010;
pub const SL_AUTOTICKS: c_int = SL_TICKS;
pub const SL_LEFT: c_int = 0x0040;
pub const SL_TOP: c_int = 0x0080;
pub const SL_RIGHT: c_int = 0x0100;
pub const SL_BOTTOM: c_int = 0x0200;
pub const SL_BOTH: c_int = 0x0400;
pub const SL_SELRANGE: c_int = 0x0800;
pub const SL_INVERSE: c_int = 0x1000;
pub const SL_MIN_MAX_LABELS: c_int = 0x2000;
pub const SL_VALUE_LABEL: c_int = 0x4000;
pub const SL_LABELS: c_int = (SL_MIN_MAX_LABELS|SL_VALUE_LABEL);

//  ENUM: wxSocketError
pub const SOCKET_NOERROR: c_int = 0;
pub const SOCKET_INVOP: c_int = 0 + 1;
pub const SOCKET_IOERR: c_int = 0 + 2;
pub const SOCKET_INVADDR: c_int = 0 + 3;
pub const SOCKET_INVSOCK: c_int = 0 + 4;
pub const SOCKET_NOHOST: c_int = 0 + 5;
pub const SOCKET_INVPORT: c_int = 0 + 6;
pub const SOCKET_WOULDBLOCK: c_int = 0 + 7;
pub const SOCKET_TIMEDOUT: c_int = 0 + 8;
pub const SOCKET_MEMERR: c_int = 0 + 9;
//  ENUM: wxSocketEventFlags
pub const SOCKET_INPUT: c_int = 0;
pub const SOCKET_OUTPUT: c_int = 0 + 1;
pub const SOCKET_CONNECTION: c_int = 0 + 2;
pub const SOCKET_LOST: c_int = 0 + 3;
//  ENUM: @46
pub const SOCKET_NONE: c_int = 0;
pub const SOCKET_NOWAIT: c_int = 1;
pub const SOCKET_WAITALL: c_int = 2;
pub const SOCKET_BLOCK: c_int = 4;
pub const SOCKET_REUSEADDR: c_int = 8;
pub const SOCKET_BROADCAST: c_int = 16;
pub const SOCKET_NOBIND: c_int = 32;
pub const SOCKET_NOWAIT_READ: c_int = 64;
pub const SOCKET_WAITALL_READ: c_int = 128;
pub const SOCKET_NOWAIT_WRITE: c_int = 256;
pub const SOCKET_WAITALL_WRITE: c_int = 512;

pub const SOUND_SYNC: c_int = 0;
pub const SOUND_ASYNC: c_int = 1;
pub const SOUND_LOOP: c_int = 2;

pub const SPLASH_CENTRE_ON_PARENT: c_int = 0x01;
pub const SPLASH_CENTRE_ON_SCREEN: c_int = 0x02;
pub const SPLASH_NO_CENTRE: c_int = 0x00;
pub const SPLASH_TIMEOUT: c_int = 0x04;
pub const SPLASH_NO_TIMEOUT: c_int = 0x00;

pub const SP_NOBORDER: c_int = 0x0000;
pub const SP_THIN_SASH: c_int = 0x0000;
pub const SP_NOSASH: c_int = 0x0010;
pub const SP_PERMIT_UNSPLIT: c_int = 0x0040;
pub const SP_LIVE_UPDATE: c_int = 0x0080;
pub const SP_3DSASH: c_int = 0x0100;
pub const SP_3DBORDER: c_int = 0x0200;
pub const SP_NO_XP_THEME: c_int = 0x0400;
pub const SP_BORDER: c_int = SP_3DBORDER;
pub const SP_3D: c_int = (SP_3DBORDER | SP_3DSASH);
//  ENUM: wxSplitMode
pub const SPLIT_HORIZONTAL: c_int = 1;
pub const SPLIT_VERTICAL: c_int = 1 + 1;
//  ENUM: @47
pub const SPLIT_DRAG_NONE: c_int = 0;
pub const SPLIT_DRAG_DRAGGING: c_int = 0 + 1;
pub const SPLIT_DRAG_LEFT_DOWN: c_int = 0 + 2;

pub const STACKWALKER_MAX_DEPTH: c_int = (200);

pub const ST_NO_AUTORESIZE: c_int = 0x0001;
pub const ST_ELLIPSIZE_START: c_int = 0x0004;
pub const ST_ELLIPSIZE_MIDDLE: c_int = 0x0008;
pub const ST_ELLIPSIZE_END: c_int = 0x0010;

pub const STB_SIZEGRIP: c_long = 0x0010;
pub const STB_SHOW_TIPS: c_long = 0x0020;
pub const STB_ELLIPSIZE_START: c_int = 0x0040;
pub const STB_ELLIPSIZE_MIDDLE: c_int = 0x0080;
pub const STB_ELLIPSIZE_END: c_long = 0x0100;
pub const STB_DEFAULT_STYLE: c_long = (STB_SIZEGRIP|STB_ELLIPSIZE_END|STB_SHOW_TIPS|FULL_REPAINT_ON_RESIZE);
pub const SB_NORMAL: c_int = 0x0000;
pub const SB_FLAT: c_int = 0x0001;
pub const SB_RAISED: c_int = 0x0002;
pub const SB_SUNKEN: c_int = 0x0003;

pub const STC_INVALID_POSITION: c_int = -1;
pub const STC_START: c_int = 2000;
pub const STC_OPTIONAL_START: c_int = 3000;
pub const STC_LEXER_START: c_int = 4000;
pub const STC_WS_INVISIBLE: c_int = 0;
pub const STC_WS_VISIBLEALWAYS: c_int = 1;
pub const STC_WS_VISIBLEAFTERINDENT: c_int = 2;
pub const STC_WS_VISIBLEONLYININDENT: c_int = 3;
pub const STC_TD_LONGARROW: c_int = 0;
pub const STC_TD_STRIKEOUT: c_int = 1;
pub const STC_EOL_CRLF: c_int = 0;
pub const STC_EOL_CR: c_int = 1;
pub const STC_EOL_LF: c_int = 2;
pub const STC_CP_UTF8: c_int = 65001;
pub const STC_IME_WINDOWED: c_int = 0;
pub const STC_IME_INLINE: c_int = 1;
pub const STC_MARKER_MAX: c_int = 31;
pub const STC_MARK_CIRCLE: c_int = 0;
pub const STC_MARK_ROUNDRECT: c_int = 1;
pub const STC_MARK_ARROW: c_int = 2;
pub const STC_MARK_SMALLRECT: c_int = 3;
pub const STC_MARK_SHORTARROW: c_int = 4;
pub const STC_MARK_EMPTY: c_int = 5;
pub const STC_MARK_ARROWDOWN: c_int = 6;
pub const STC_MARK_MINUS: c_int = 7;
pub const STC_MARK_PLUS: c_int = 8;
pub const STC_MARK_VLINE: c_int = 9;
pub const STC_MARK_LCORNER: c_int = 10;
pub const STC_MARK_TCORNER: c_int = 11;
pub const STC_MARK_BOXPLUS: c_int = 12;
pub const STC_MARK_BOXPLUSCONNECTED: c_int = 13;
pub const STC_MARK_BOXMINUS: c_int = 14;
pub const STC_MARK_BOXMINUSCONNECTED: c_int = 15;
pub const STC_MARK_LCORNERCURVE: c_int = 16;
pub const STC_MARK_TCORNERCURVE: c_int = 17;
pub const STC_MARK_CIRCLEPLUS: c_int = 18;
pub const STC_MARK_CIRCLEPLUSCONNECTED: c_int = 19;
pub const STC_MARK_CIRCLEMINUS: c_int = 20;
pub const STC_MARK_CIRCLEMINUSCONNECTED: c_int = 21;
pub const STC_MARK_BACKGROUND: c_int = 22;
pub const STC_MARK_DOTDOTDOT: c_int = 23;
pub const STC_MARK_ARROWS: c_int = 24;
pub const STC_MARK_PIXMAP: c_int = 25;
pub const STC_MARK_FULLRECT: c_int = 26;
pub const STC_MARK_LEFTRECT: c_int = 27;
pub const STC_MARK_AVAILABLE: c_int = 28;
pub const STC_MARK_UNDERLINE: c_int = 29;
pub const STC_MARK_RGBAIMAGE: c_int = 30;
pub const STC_MARK_BOOKMARK: c_int = 31;
pub const STC_MARK_CHARACTER: c_int = 10000;
pub const STC_MARKNUM_FOLDEREND: c_int = 25;
pub const STC_MARKNUM_FOLDEROPENMID: c_int = 26;
pub const STC_MARKNUM_FOLDERMIDTAIL: c_int = 27;
pub const STC_MARKNUM_FOLDERTAIL: c_int = 28;
pub const STC_MARKNUM_FOLDERSUB: c_int = 29;
pub const STC_MARKNUM_FOLDER: c_int = 30;
pub const STC_MARKNUM_FOLDEROPEN: c_int = 31;
pub const STC_MASK_FOLDERS: c_long = 0xFE000000;
pub const STC_MAX_MARGIN: c_int = 4;
pub const STC_MARGIN_SYMBOL: c_int = 0;
pub const STC_MARGIN_NUMBER: c_int = 1;
pub const STC_MARGIN_BACK: c_int = 2;
pub const STC_MARGIN_FORE: c_int = 3;
pub const STC_MARGIN_TEXT: c_int = 4;
pub const STC_MARGIN_RTEXT: c_int = 5;
pub const STC_MARGIN_COLOUR: c_int = 6;
pub const STC_STYLE_DEFAULT: c_int = 32;
pub const STC_STYLE_LINENUMBER: c_int = 33;
pub const STC_STYLE_BRACELIGHT: c_int = 34;
pub const STC_STYLE_BRACEBAD: c_int = 35;
pub const STC_STYLE_CONTROLCHAR: c_int = 36;
pub const STC_STYLE_INDENTGUIDE: c_int = 37;
pub const STC_STYLE_CALLTIP: c_int = 38;
pub const STC_STYLE_FOLDDISPLAYTEXT: c_int = 39;
pub const STC_STYLE_LASTPREDEFINED: c_int = 39;
pub const STC_STYLE_MAX: c_int = 255;
pub const STC_CHARSET_ANSI: c_int = 0;
pub const STC_CHARSET_DEFAULT: c_int = 1;
pub const STC_CHARSET_BALTIC: c_int = 186;
pub const STC_CHARSET_CHINESEBIG5: c_int = 136;
pub const STC_CHARSET_EASTEUROPE: c_int = 238;
pub const STC_CHARSET_GB2312: c_int = 134;
pub const STC_CHARSET_GREEK: c_int = 161;
pub const STC_CHARSET_HANGUL: c_int = 129;
pub const STC_CHARSET_MAC: c_int = 77;
pub const STC_CHARSET_OEM: c_int = 255;
pub const STC_CHARSET_RUSSIAN: c_int = 204;
pub const STC_CHARSET_OEM866: c_int = 866;
pub const STC_CHARSET_CYRILLIC: c_int = 1251;
pub const STC_CHARSET_SHIFTJIS: c_int = 128;
pub const STC_CHARSET_SYMBOL: c_int = 2;
pub const STC_CHARSET_TURKISH: c_int = 162;
pub const STC_CHARSET_JOHAB: c_int = 130;
pub const STC_CHARSET_HEBREW: c_int = 177;
pub const STC_CHARSET_ARABIC: c_int = 178;
pub const STC_CHARSET_VIETNAMESE: c_int = 163;
pub const STC_CHARSET_THAI: c_int = 222;
pub const STC_CHARSET_8859_15: c_int = 1000;
pub const STC_CASE_MIXED: c_int = 0;
pub const STC_CASE_UPPER: c_int = 1;
pub const STC_CASE_LOWER: c_int = 2;
pub const STC_CASE_CAMEL: c_int = 3;
pub const STC_FONT_SIZE_MULTIPLIER: c_int = 100;
pub const STC_WEIGHT_NORMAL: c_int = 400;
pub const STC_WEIGHT_SEMIBOLD: c_int = 600;
pub const STC_WEIGHT_BOLD: c_int = 700;
pub const STC_INDIC_PLAIN: c_int = 0;
pub const STC_INDIC_SQUIGGLE: c_int = 1;
pub const STC_INDIC_TT: c_int = 2;
pub const STC_INDIC_DIAGONAL: c_int = 3;
pub const STC_INDIC_STRIKE: c_int = 4;
pub const STC_INDIC_HIDDEN: c_int = 5;
pub const STC_INDIC_BOX: c_int = 6;
pub const STC_INDIC_ROUNDBOX: c_int = 7;
pub const STC_INDIC_STRAIGHTBOX: c_int = 8;
pub const STC_INDIC_DASH: c_int = 9;
pub const STC_INDIC_DOTS: c_int = 10;
pub const STC_INDIC_SQUIGGLELOW: c_int = 11;
pub const STC_INDIC_DOTBOX: c_int = 12;
pub const STC_INDIC_SQUIGGLEPIXMAP: c_int = 13;
pub const STC_INDIC_COMPOSITIONTHICK: c_int = 14;
pub const STC_INDIC_COMPOSITIONTHIN: c_int = 15;
pub const STC_INDIC_FULLBOX: c_int = 16;
pub const STC_INDIC_TEXTFORE: c_int = 17;
pub const STC_INDIC_POINT: c_int = 18;
pub const STC_INDIC_POINTCHARACTER: c_int = 19;
pub const STC_INDIC_IME: c_int = 32;
pub const STC_INDIC_IME_MAX: c_int = 35;
pub const STC_INDIC_MAX: c_int = 35;
pub const STC_INDIC_CONTAINER: c_int = 8;
pub const STC_INDICVALUEBIT: c_int = 0x1000000;
pub const STC_INDICVALUEMASK: c_int = 0xFFFFFF;
pub const STC_INDICFLAG_VALUEFORE: c_int = 1;
pub const STC_IV_NONE: c_int = 0;
pub const STC_IV_REAL: c_int = 1;
pub const STC_IV_LOOKFORWARD: c_int = 2;
pub const STC_IV_LOOKBOTH: c_int = 3;
pub const STC_PRINT_NORMAL: c_int = 0;
pub const STC_PRINT_INVERTLIGHT: c_int = 1;
pub const STC_PRINT_BLACKONWHITE: c_int = 2;
pub const STC_PRINT_COLOURONWHITE: c_int = 3;
pub const STC_PRINT_COLOURONWHITEDEFAULTBG: c_int = 4;
pub const STC_FIND_WHOLEWORD: c_int = 0x2;
pub const STC_FIND_MATCHCASE: c_int = 0x4;
pub const STC_FIND_WORDSTART: c_int = 0x00100000;
pub const STC_FIND_REGEXP: c_int = 0x00200000;
pub const STC_FIND_POSIX: c_int = 0x00400000;
pub const STC_FOLDLEVELBASE: c_int = 0x400;
pub const STC_FOLDLEVELWHITEFLAG: c_int = 0x1000;
pub const STC_FOLDLEVELHEADERFLAG: c_int = 0x2000;
pub const STC_FOLDLEVELNUMBERMASK: c_int = 0x0FFF;
pub const STC_FOLDDISPLAYTEXT_HIDDEN: c_int = 0;
pub const STC_FOLDDISPLAYTEXT_STANDARD: c_int = 1;
pub const STC_FOLDDISPLAYTEXT_BOXED: c_int = 2;
pub const STC_FOLDACTION_CONTRACT: c_int = 0;
pub const STC_FOLDACTION_EXPAND: c_int = 1;
pub const STC_FOLDACTION_TOGGLE: c_int = 2;
pub const STC_AUTOMATICFOLD_SHOW: c_int = 0x0001;
pub const STC_AUTOMATICFOLD_CLICK: c_int = 0x0002;
pub const STC_AUTOMATICFOLD_CHANGE: c_int = 0x0004;
pub const STC_FOLDFLAG_LINEBEFORE_EXPANDED: c_int = 0x0002;
pub const STC_FOLDFLAG_LINEBEFORE_CONTRACTED: c_int = 0x0004;
pub const STC_FOLDFLAG_LINEAFTER_EXPANDED: c_int = 0x0008;
pub const STC_FOLDFLAG_LINEAFTER_CONTRACTED: c_int = 0x0010;
pub const STC_FOLDFLAG_LEVELNUMBERS: c_int = 0x0040;
pub const STC_FOLDFLAG_LINESTATE: c_int = 0x0080;
pub const STC_TIME_FOREVER: c_int = 10000000;
pub const STC_IDLESTYLING_NONE: c_int = 0;
pub const STC_IDLESTYLING_TOVISIBLE: c_int = 1;
pub const STC_IDLESTYLING_AFTERVISIBLE: c_int = 2;
pub const STC_IDLESTYLING_ALL: c_int = 3;
pub const STC_WRAP_NONE: c_int = 0;
pub const STC_WRAP_WORD: c_int = 1;
pub const STC_WRAP_CHAR: c_int = 2;
pub const STC_WRAP_WHITESPACE: c_int = 3;
pub const STC_WRAPVISUALFLAG_NONE: c_int = 0x0000;
pub const STC_WRAPVISUALFLAG_END: c_int = 0x0001;
pub const STC_WRAPVISUALFLAG_START: c_int = 0x0002;
pub const STC_WRAPVISUALFLAG_MARGIN: c_int = 0x0004;
pub const STC_WRAPVISUALFLAGLOC_DEFAULT: c_int = 0x0000;
pub const STC_WRAPVISUALFLAGLOC_END_BY_TEXT: c_int = 0x0001;
pub const STC_WRAPVISUALFLAGLOC_START_BY_TEXT: c_int = 0x0002;
pub const STC_WRAPINDENT_FIXED: c_int = 0;
pub const STC_WRAPINDENT_SAME: c_int = 1;
pub const STC_WRAPINDENT_INDENT: c_int = 2;
pub const STC_CACHE_NONE: c_int = 0;
pub const STC_CACHE_CARET: c_int = 1;
pub const STC_CACHE_PAGE: c_int = 2;
pub const STC_CACHE_DOCUMENT: c_int = 3;
pub const STC_PHASES_ONE: c_int = 0;
pub const STC_PHASES_TWO: c_int = 1;
pub const STC_PHASES_MULTIPLE: c_int = 2;
pub const STC_EFF_QUALITY_MASK: c_int = 0xF;
pub const STC_EFF_QUALITY_DEFAULT: c_int = 0;
pub const STC_EFF_QUALITY_NON_ANTIALIASED: c_int = 1;
pub const STC_EFF_QUALITY_ANTIALIASED: c_int = 2;
pub const STC_EFF_QUALITY_LCD_OPTIMIZED: c_int = 3;
pub const STC_MULTIPASTE_ONCE: c_int = 0;
pub const STC_MULTIPASTE_EACH: c_int = 1;
pub const STC_EDGE_NONE: c_int = 0;
pub const STC_EDGE_LINE: c_int = 1;
pub const STC_EDGE_BACKGROUND: c_int = 2;
pub const STC_EDGE_MULTILINE: c_int = 3;
pub const STC_POPUP_NEVER: c_int = 0;
pub const STC_POPUP_ALL: c_int = 1;
pub const STC_POPUP_TEXT: c_int = 2;
pub const STC_STATUS_OK: c_int = 0;
pub const STC_STATUS_FAILURE: c_int = 1;
pub const STC_STATUS_BADALLOC: c_int = 2;
pub const STC_STATUS_WARN_START: c_int = 1000;
pub const STC_STATUS_WARN_REGEX: c_int = 1001;
pub const STC_CURSORNORMAL: c_int = -1;
pub const STC_CURSORARROW: c_int = 2;
pub const STC_CURSORWAIT: c_int = 4;
pub const STC_CURSORREVERSEARROW: c_int = 7;
pub const STC_VISIBLE_SLOP: c_int = 0x01;
pub const STC_VISIBLE_STRICT: c_int = 0x04;
pub const STC_CARET_SLOP: c_int = 0x01;
pub const STC_CARET_STRICT: c_int = 0x04;
pub const STC_CARET_JUMPS: c_int = 0x10;
pub const STC_CARET_EVEN: c_int = 0x08;
pub const STC_SEL_STREAM: c_int = 0;
pub const STC_SEL_RECTANGLE: c_int = 1;
pub const STC_SEL_LINES: c_int = 2;
pub const STC_SEL_THIN: c_int = 3;
pub const STC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE: c_int = 0;
pub const STC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE: c_int = 1;
pub const STC_MULTIAUTOC_ONCE: c_int = 0;
pub const STC_MULTIAUTOC_EACH: c_int = 1;
pub const STC_ORDER_PRESORTED: c_int = 0;
pub const STC_ORDER_PERFORMSORT: c_int = 1;
pub const STC_ORDER_CUSTOM: c_int = 2;
pub const STC_CARETSTICKY_OFF: c_int = 0;
pub const STC_CARETSTICKY_ON: c_int = 1;
pub const STC_CARETSTICKY_WHITESPACE: c_int = 2;
pub const STC_ALPHA_TRANSPARENT: c_int = 0;
pub const STC_ALPHA_OPAQUE: c_int = 255;
pub const STC_ALPHA_NOALPHA: c_int = 256;
pub const STC_CARETSTYLE_INVISIBLE: c_int = 0;
pub const STC_CARETSTYLE_LINE: c_int = 1;
pub const STC_CARETSTYLE_BLOCK: c_int = 2;
pub const STC_MARGINOPTION_NONE: c_int = 0;
pub const STC_MARGINOPTION_SUBLINESELECT: c_int = 1;
pub const STC_ANNOTATION_HIDDEN: c_int = 0;
pub const STC_ANNOTATION_STANDARD: c_int = 1;
pub const STC_ANNOTATION_BOXED: c_int = 2;
pub const STC_ANNOTATION_INDENTED: c_int = 3;
pub const STC_UNDO_MAY_COALESCE: c_int = 1;
pub const STC_VS_NONE: c_int = 0;
pub const STC_VS_RECTANGULARSELECTION: c_int = 1;
pub const STC_VS_USERACCESSIBLE: c_int = 2;
pub const STC_VS_NOWRAPLINESTART: c_int = 4;
pub const STC_TECHNOLOGY_DEFAULT: c_int = 0;
pub const STC_TECHNOLOGY_DIRECTWRITE: c_int = 1;
pub const STC_LINE_END_TYPE_DEFAULT: c_int = 0;
pub const STC_LINE_END_TYPE_UNICODE: c_int = 1;
pub const STC_KEYWORDSET_MAX: c_int = 8;
pub const STC_TYPE_BOOLEAN: c_int = 0;
pub const STC_TYPE_INTEGER: c_int = 1;
pub const STC_TYPE_STRING: c_int = 2;
pub const STC_MOD_INSERTTEXT: c_int = 0x1;
pub const STC_MOD_DELETETEXT: c_int = 0x2;
pub const STC_MOD_CHANGESTYLE: c_int = 0x4;
pub const STC_MOD_CHANGEFOLD: c_int = 0x8;
pub const STC_PERFORMED_USER: c_int = 0x10;
pub const STC_PERFORMED_UNDO: c_int = 0x20;
pub const STC_PERFORMED_REDO: c_int = 0x40;
pub const STC_MULTISTEPUNDOREDO: c_int = 0x80;
pub const STC_LASTSTEPINUNDOREDO: c_int = 0x100;
pub const STC_MOD_CHANGEMARKER: c_int = 0x200;
pub const STC_MOD_BEFOREINSERT: c_int = 0x400;
pub const STC_MOD_BEFOREDELETE: c_int = 0x800;
pub const STC_MULTILINEUNDOREDO: c_int = 0x1000;
pub const STC_STARTACTION: c_int = 0x2000;
pub const STC_MOD_CHANGEINDICATOR: c_int = 0x4000;
pub const STC_MOD_CHANGELINESTATE: c_int = 0x8000;
pub const STC_MOD_CHANGEMARGIN: c_int = 0x10000;
pub const STC_MOD_CHANGEANNOTATION: c_int = 0x20000;
pub const STC_MOD_CONTAINER: c_int = 0x40000;
pub const STC_MOD_LEXERSTATE: c_int = 0x80000;
pub const STC_MOD_INSERTCHECK: c_int = 0x100000;
pub const STC_MOD_CHANGETABSTOPS: c_int = 0x200000;
pub const STC_MODEVENTMASKALL: c_int = 0x3FFFFF;
pub const STC_UPDATE_CONTENT: c_int = 0x1;
pub const STC_UPDATE_SELECTION: c_int = 0x2;
pub const STC_UPDATE_V_SCROLL: c_int = 0x4;
pub const STC_UPDATE_H_SCROLL: c_int = 0x8;
pub const STC_KEY_DOWN: c_int = 300;
pub const STC_KEY_UP: c_int = 301;
pub const STC_KEY_LEFT: c_int = 302;
pub const STC_KEY_RIGHT: c_int = 303;
pub const STC_KEY_HOME: c_int = 304;
pub const STC_KEY_END: c_int = 305;
pub const STC_KEY_PRIOR: c_int = 306;
pub const STC_KEY_NEXT: c_int = 307;
pub const STC_KEY_DELETE: c_int = 308;
pub const STC_KEY_INSERT: c_int = 309;
pub const STC_KEY_ESCAPE: c_int = 7;
pub const STC_KEY_BACK: c_int = 8;
pub const STC_KEY_TAB: c_int = 9;
pub const STC_KEY_RETURN: c_int = 13;
pub const STC_KEY_ADD: c_int = 310;
pub const STC_KEY_SUBTRACT: c_int = 311;
pub const STC_KEY_DIVIDE: c_int = 312;
pub const STC_KEY_WIN: c_int = 313;
pub const STC_KEY_RWIN: c_int = 314;
pub const STC_KEY_MENU: c_int = 315;
pub const STC_KEYMOD_NORM: c_int = 0;
pub const STC_KEYMOD_SHIFT: c_int = 1;
pub const STC_KEYMOD_CTRL: c_int = 2;
pub const STC_KEYMOD_ALT: c_int = 4;
pub const STC_KEYMOD_SUPER: c_int = 8;
pub const STC_KEYMOD_META: c_int = 16;
pub const STC_AC_FILLUP: c_int = 1;
pub const STC_AC_DOUBLECLICK: c_int = 2;
pub const STC_AC_TAB: c_int = 3;
pub const STC_AC_NEWLINE: c_int = 4;
pub const STC_AC_COMMAND: c_int = 5;
pub const STC_LEX_CONTAINER: c_int = 0;
pub const STC_LEX_NULL: c_int = 1;
pub const STC_LEX_PYTHON: c_int = 2;
pub const STC_LEX_CPP: c_int = 3;
pub const STC_LEX_HTML: c_int = 4;
pub const STC_LEX_XML: c_int = 5;
pub const STC_LEX_PERL: c_int = 6;
pub const STC_LEX_SQL: c_int = 7;
pub const STC_LEX_VB: c_int = 8;
pub const STC_LEX_PROPERTIES: c_int = 9;
pub const STC_LEX_ERRORLIST: c_int = 10;
pub const STC_LEX_MAKEFILE: c_int = 11;
pub const STC_LEX_BATCH: c_int = 12;
pub const STC_LEX_XCODE: c_int = 13;
pub const STC_LEX_LATEX: c_int = 14;
pub const STC_LEX_LUA: c_int = 15;
pub const STC_LEX_DIFF: c_int = 16;
pub const STC_LEX_CONF: c_int = 17;
pub const STC_LEX_PASCAL: c_int = 18;
pub const STC_LEX_AVE: c_int = 19;
pub const STC_LEX_ADA: c_int = 20;
pub const STC_LEX_LISP: c_int = 21;
pub const STC_LEX_RUBY: c_int = 22;
pub const STC_LEX_EIFFEL: c_int = 23;
pub const STC_LEX_EIFFELKW: c_int = 24;
pub const STC_LEX_TCL: c_int = 25;
pub const STC_LEX_NNCRONTAB: c_int = 26;
pub const STC_LEX_BULLANT: c_int = 27;
pub const STC_LEX_VBSCRIPT: c_int = 28;
pub const STC_LEX_BAAN: c_int = 31;
pub const STC_LEX_MATLAB: c_int = 32;
pub const STC_LEX_SCRIPTOL: c_int = 33;
pub const STC_LEX_ASM: c_int = 34;
pub const STC_LEX_CPPNOCASE: c_int = 35;
pub const STC_LEX_FORTRAN: c_int = 36;
pub const STC_LEX_F77: c_int = 37;
pub const STC_LEX_CSS: c_int = 38;
pub const STC_LEX_POV: c_int = 39;
pub const STC_LEX_LOUT: c_int = 40;
pub const STC_LEX_ESCRIPT: c_int = 41;
pub const STC_LEX_PS: c_int = 42;
pub const STC_LEX_NSIS: c_int = 43;
pub const STC_LEX_MMIXAL: c_int = 44;
pub const STC_LEX_CLW: c_int = 45;
pub const STC_LEX_CLWNOCASE: c_int = 46;
pub const STC_LEX_LOT: c_int = 47;
pub const STC_LEX_YAML: c_int = 48;
pub const STC_LEX_TEX: c_int = 49;
pub const STC_LEX_METAPOST: c_int = 50;
pub const STC_LEX_POWERBASIC: c_int = 51;
pub const STC_LEX_FORTH: c_int = 52;
pub const STC_LEX_ERLANG: c_int = 53;
pub const STC_LEX_OCTAVE: c_int = 54;
pub const STC_LEX_MSSQL: c_int = 55;
pub const STC_LEX_VERILOG: c_int = 56;
pub const STC_LEX_KIX: c_int = 57;
pub const STC_LEX_GUI4CLI: c_int = 58;
pub const STC_LEX_SPECMAN: c_int = 59;
pub const STC_LEX_AU3: c_int = 60;
pub const STC_LEX_APDL: c_int = 61;
pub const STC_LEX_BASH: c_int = 62;
pub const STC_LEX_ASN1: c_int = 63;
pub const STC_LEX_VHDL: c_int = 64;
pub const STC_LEX_CAML: c_int = 65;
pub const STC_LEX_BLITZBASIC: c_int = 66;
pub const STC_LEX_PUREBASIC: c_int = 67;
pub const STC_LEX_HASKELL: c_int = 68;
pub const STC_LEX_PHPSCRIPT: c_int = 69;
pub const STC_LEX_TADS3: c_int = 70;
pub const STC_LEX_REBOL: c_int = 71;
pub const STC_LEX_SMALLTALK: c_int = 72;
pub const STC_LEX_FLAGSHIP: c_int = 73;
pub const STC_LEX_CSOUND: c_int = 74;
pub const STC_LEX_FREEBASIC: c_int = 75;
pub const STC_LEX_INNOSETUP: c_int = 76;
pub const STC_LEX_OPAL: c_int = 77;
pub const STC_LEX_SPICE: c_int = 78;
pub const STC_LEX_D: c_int = 79;
pub const STC_LEX_CMAKE: c_int = 80;
pub const STC_LEX_GAP: c_int = 81;
pub const STC_LEX_PLM: c_int = 82;
pub const STC_LEX_PROGRESS: c_int = 83;
pub const STC_LEX_ABAQUS: c_int = 84;
pub const STC_LEX_ASYMPTOTE: c_int = 85;
pub const STC_LEX_R: c_int = 86;
pub const STC_LEX_MAGIK: c_int = 87;
pub const STC_LEX_POWERSHELL: c_int = 88;
pub const STC_LEX_MYSQL: c_int = 89;
pub const STC_LEX_PO: c_int = 90;
pub const STC_LEX_TAL: c_int = 91;
pub const STC_LEX_COBOL: c_int = 92;
pub const STC_LEX_TACL: c_int = 93;
pub const STC_LEX_SORCUS: c_int = 94;
pub const STC_LEX_POWERPRO: c_int = 95;
pub const STC_LEX_NIMROD: c_int = 96;
pub const STC_LEX_SML: c_int = 97;
pub const STC_LEX_MARKDOWN: c_int = 98;
pub const STC_LEX_TXT2TAGS: c_int = 99;
pub const STC_LEX_A68K: c_int = 100;
pub const STC_LEX_MODULA: c_int = 101;
pub const STC_LEX_COFFEESCRIPT: c_int = 102;
pub const STC_LEX_TCMD: c_int = 103;
pub const STC_LEX_AVS: c_int = 104;
pub const STC_LEX_ECL: c_int = 105;
pub const STC_LEX_OSCRIPT: c_int = 106;
pub const STC_LEX_VISUALPROLOG: c_int = 107;
pub const STC_LEX_LITERATEHASKELL: c_int = 108;
pub const STC_LEX_STTXT: c_int = 109;
pub const STC_LEX_KVIRC: c_int = 110;
pub const STC_LEX_RUST: c_int = 111;
pub const STC_LEX_DMAP: c_int = 112;
pub const STC_LEX_AS: c_int = 113;
pub const STC_LEX_DMIS: c_int = 114;
pub const STC_LEX_REGISTRY: c_int = 115;
pub const STC_LEX_BIBTEX: c_int = 116;
pub const STC_LEX_SREC: c_int = 117;
pub const STC_LEX_IHEX: c_int = 118;
pub const STC_LEX_TEHEX: c_int = 119;
pub const STC_LEX_JSON: c_int = 120;
pub const STC_LEX_EDIFACT: c_int = 121;
pub const STC_LEX_AUTOMATIC: c_int = 1000;
pub const STC_P_DEFAULT: c_int = 0;
pub const STC_P_COMMENTLINE: c_int = 1;
pub const STC_P_NUMBER: c_int = 2;
pub const STC_P_STRING: c_int = 3;
pub const STC_P_CHARACTER: c_int = 4;
pub const STC_P_WORD: c_int = 5;
pub const STC_P_TRIPLE: c_int = 6;
pub const STC_P_TRIPLEDOUBLE: c_int = 7;
pub const STC_P_CLASSNAME: c_int = 8;
pub const STC_P_DEFNAME: c_int = 9;
pub const STC_P_OPERATOR: c_int = 10;
pub const STC_P_IDENTIFIER: c_int = 11;
pub const STC_P_COMMENTBLOCK: c_int = 12;
pub const STC_P_STRINGEOL: c_int = 13;
pub const STC_P_WORD2: c_int = 14;
pub const STC_P_DECORATOR: c_int = 15;
pub const STC_C_DEFAULT: c_int = 0;
pub const STC_C_COMMENT: c_int = 1;
pub const STC_C_COMMENTLINE: c_int = 2;
pub const STC_C_COMMENTDOC: c_int = 3;
pub const STC_C_NUMBER: c_int = 4;
pub const STC_C_WORD: c_int = 5;
pub const STC_C_STRING: c_int = 6;
pub const STC_C_CHARACTER: c_int = 7;
pub const STC_C_UUID: c_int = 8;
pub const STC_C_PREPROCESSOR: c_int = 9;
pub const STC_C_OPERATOR: c_int = 10;
pub const STC_C_IDENTIFIER: c_int = 11;
pub const STC_C_STRINGEOL: c_int = 12;
pub const STC_C_VERBATIM: c_int = 13;
pub const STC_C_REGEX: c_int = 14;
pub const STC_C_COMMENTLINEDOC: c_int = 15;
pub const STC_C_WORD2: c_int = 16;
pub const STC_C_COMMENTDOCKEYWORD: c_int = 17;
pub const STC_C_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const STC_C_GLOBALCLASS: c_int = 19;
pub const STC_C_STRINGRAW: c_int = 20;
pub const STC_C_TRIPLEVERBATIM: c_int = 21;
pub const STC_C_HASHQUOTEDSTRING: c_int = 22;
pub const STC_C_PREPROCESSORCOMMENT: c_int = 23;
pub const STC_C_PREPROCESSORCOMMENTDOC: c_int = 24;
pub const STC_C_USERLITERAL: c_int = 25;
pub const STC_C_TASKMARKER: c_int = 26;
pub const STC_C_ESCAPESEQUENCE: c_int = 27;
pub const STC_D_DEFAULT: c_int = 0;
pub const STC_D_COMMENT: c_int = 1;
pub const STC_D_COMMENTLINE: c_int = 2;
pub const STC_D_COMMENTDOC: c_int = 3;
pub const STC_D_COMMENTNESTED: c_int = 4;
pub const STC_D_NUMBER: c_int = 5;
pub const STC_D_WORD: c_int = 6;
pub const STC_D_WORD2: c_int = 7;
pub const STC_D_WORD3: c_int = 8;
pub const STC_D_TYPEDEF: c_int = 9;
pub const STC_D_STRING: c_int = 10;
pub const STC_D_STRINGEOL: c_int = 11;
pub const STC_D_CHARACTER: c_int = 12;
pub const STC_D_OPERATOR: c_int = 13;
pub const STC_D_IDENTIFIER: c_int = 14;
pub const STC_D_COMMENTLINEDOC: c_int = 15;
pub const STC_D_COMMENTDOCKEYWORD: c_int = 16;
pub const STC_D_COMMENTDOCKEYWORDERROR: c_int = 17;
pub const STC_D_STRINGB: c_int = 18;
pub const STC_D_STRINGR: c_int = 19;
pub const STC_D_WORD5: c_int = 20;
pub const STC_D_WORD6: c_int = 21;
pub const STC_D_WORD7: c_int = 22;
pub const STC_TCL_DEFAULT: c_int = 0;
pub const STC_TCL_COMMENT: c_int = 1;
pub const STC_TCL_COMMENTLINE: c_int = 2;
pub const STC_TCL_NUMBER: c_int = 3;
pub const STC_TCL_WORD_IN_QUOTE: c_int = 4;
pub const STC_TCL_IN_QUOTE: c_int = 5;
pub const STC_TCL_OPERATOR: c_int = 6;
pub const STC_TCL_IDENTIFIER: c_int = 7;
pub const STC_TCL_SUBSTITUTION: c_int = 8;
pub const STC_TCL_SUB_BRACE: c_int = 9;
pub const STC_TCL_MODIFIER: c_int = 10;
pub const STC_TCL_EXPAND: c_int = 11;
pub const STC_TCL_WORD: c_int = 12;
pub const STC_TCL_WORD2: c_int = 13;
pub const STC_TCL_WORD3: c_int = 14;
pub const STC_TCL_WORD4: c_int = 15;
pub const STC_TCL_WORD5: c_int = 16;
pub const STC_TCL_WORD6: c_int = 17;
pub const STC_TCL_WORD7: c_int = 18;
pub const STC_TCL_WORD8: c_int = 19;
pub const STC_TCL_COMMENT_BOX: c_int = 20;
pub const STC_TCL_BLOCK_COMMENT: c_int = 21;
pub const STC_H_DEFAULT: c_int = 0;
pub const STC_H_TAG: c_int = 1;
pub const STC_H_TAGUNKNOWN: c_int = 2;
pub const STC_H_ATTRIBUTE: c_int = 3;
pub const STC_H_ATTRIBUTEUNKNOWN: c_int = 4;
pub const STC_H_NUMBER: c_int = 5;
pub const STC_H_DOUBLESTRING: c_int = 6;
pub const STC_H_SINGLESTRING: c_int = 7;
pub const STC_H_OTHER: c_int = 8;
pub const STC_H_COMMENT: c_int = 9;
pub const STC_H_ENTITY: c_int = 10;
pub const STC_H_TAGEND: c_int = 11;
pub const STC_H_XMLSTART: c_int = 12;
pub const STC_H_XMLEND: c_int = 13;
pub const STC_H_SCRIPT: c_int = 14;
pub const STC_H_ASP: c_int = 15;
pub const STC_H_ASPAT: c_int = 16;
pub const STC_H_CDATA: c_int = 17;
pub const STC_H_QUESTION: c_int = 18;
pub const STC_H_VALUE: c_int = 19;
pub const STC_H_XCCOMMENT: c_int = 20;
pub const STC_H_SGML_DEFAULT: c_int = 21;
pub const STC_H_SGML_COMMAND: c_int = 22;
pub const STC_H_SGML_1ST_PARAM: c_int = 23;
pub const STC_H_SGML_DOUBLESTRING: c_int = 24;
pub const STC_H_SGML_SIMPLESTRING: c_int = 25;
pub const STC_H_SGML_ERROR: c_int = 26;
pub const STC_H_SGML_SPECIAL: c_int = 27;
pub const STC_H_SGML_ENTITY: c_int = 28;
pub const STC_H_SGML_COMMENT: c_int = 29;
pub const STC_H_SGML_1ST_PARAM_COMMENT: c_int = 30;
pub const STC_H_SGML_BLOCK_DEFAULT: c_int = 31;
pub const STC_HJ_START: c_int = 40;
pub const STC_HJ_DEFAULT: c_int = 41;
pub const STC_HJ_COMMENT: c_int = 42;
pub const STC_HJ_COMMENTLINE: c_int = 43;
pub const STC_HJ_COMMENTDOC: c_int = 44;
pub const STC_HJ_NUMBER: c_int = 45;
pub const STC_HJ_WORD: c_int = 46;
pub const STC_HJ_KEYWORD: c_int = 47;
pub const STC_HJ_DOUBLESTRING: c_int = 48;
pub const STC_HJ_SINGLESTRING: c_int = 49;
pub const STC_HJ_SYMBOLS: c_int = 50;
pub const STC_HJ_STRINGEOL: c_int = 51;
pub const STC_HJ_REGEX: c_int = 52;
pub const STC_HJA_START: c_int = 55;
pub const STC_HJA_DEFAULT: c_int = 56;
pub const STC_HJA_COMMENT: c_int = 57;
pub const STC_HJA_COMMENTLINE: c_int = 58;
pub const STC_HJA_COMMENTDOC: c_int = 59;
pub const STC_HJA_NUMBER: c_int = 60;
pub const STC_HJA_WORD: c_int = 61;
pub const STC_HJA_KEYWORD: c_int = 62;
pub const STC_HJA_DOUBLESTRING: c_int = 63;
pub const STC_HJA_SINGLESTRING: c_int = 64;
pub const STC_HJA_SYMBOLS: c_int = 65;
pub const STC_HJA_STRINGEOL: c_int = 66;
pub const STC_HJA_REGEX: c_int = 67;
pub const STC_HB_START: c_int = 70;
pub const STC_HB_DEFAULT: c_int = 71;
pub const STC_HB_COMMENTLINE: c_int = 72;
pub const STC_HB_NUMBER: c_int = 73;
pub const STC_HB_WORD: c_int = 74;
pub const STC_HB_STRING: c_int = 75;
pub const STC_HB_IDENTIFIER: c_int = 76;
pub const STC_HB_STRINGEOL: c_int = 77;
pub const STC_HBA_START: c_int = 80;
pub const STC_HBA_DEFAULT: c_int = 81;
pub const STC_HBA_COMMENTLINE: c_int = 82;
pub const STC_HBA_NUMBER: c_int = 83;
pub const STC_HBA_WORD: c_int = 84;
pub const STC_HBA_STRING: c_int = 85;
pub const STC_HBA_IDENTIFIER: c_int = 86;
pub const STC_HBA_STRINGEOL: c_int = 87;
pub const STC_HP_START: c_int = 90;
pub const STC_HP_DEFAULT: c_int = 91;
pub const STC_HP_COMMENTLINE: c_int = 92;
pub const STC_HP_NUMBER: c_int = 93;
pub const STC_HP_STRING: c_int = 94;
pub const STC_HP_CHARACTER: c_int = 95;
pub const STC_HP_WORD: c_int = 96;
pub const STC_HP_TRIPLE: c_int = 97;
pub const STC_HP_TRIPLEDOUBLE: c_int = 98;
pub const STC_HP_CLASSNAME: c_int = 99;
pub const STC_HP_DEFNAME: c_int = 100;
pub const STC_HP_OPERATOR: c_int = 101;
pub const STC_HP_IDENTIFIER: c_int = 102;
pub const STC_HPHP_COMPLEX_VARIABLE: c_int = 104;
pub const STC_HPA_START: c_int = 105;
pub const STC_HPA_DEFAULT: c_int = 106;
pub const STC_HPA_COMMENTLINE: c_int = 107;
pub const STC_HPA_NUMBER: c_int = 108;
pub const STC_HPA_STRING: c_int = 109;
pub const STC_HPA_CHARACTER: c_int = 110;
pub const STC_HPA_WORD: c_int = 111;
pub const STC_HPA_TRIPLE: c_int = 112;
pub const STC_HPA_TRIPLEDOUBLE: c_int = 113;
pub const STC_HPA_CLASSNAME: c_int = 114;
pub const STC_HPA_DEFNAME: c_int = 115;
pub const STC_HPA_OPERATOR: c_int = 116;
pub const STC_HPA_IDENTIFIER: c_int = 117;
pub const STC_HPHP_DEFAULT: c_int = 118;
pub const STC_HPHP_HSTRING: c_int = 119;
pub const STC_HPHP_SIMPLESTRING: c_int = 120;
pub const STC_HPHP_WORD: c_int = 121;
pub const STC_HPHP_NUMBER: c_int = 122;
pub const STC_HPHP_VARIABLE: c_int = 123;
pub const STC_HPHP_COMMENT: c_int = 124;
pub const STC_HPHP_COMMENTLINE: c_int = 125;
pub const STC_HPHP_HSTRING_VARIABLE: c_int = 126;
pub const STC_HPHP_OPERATOR: c_int = 127;
pub const STC_PL_DEFAULT: c_int = 0;
pub const STC_PL_ERROR: c_int = 1;
pub const STC_PL_COMMENTLINE: c_int = 2;
pub const STC_PL_POD: c_int = 3;
pub const STC_PL_NUMBER: c_int = 4;
pub const STC_PL_WORD: c_int = 5;
pub const STC_PL_STRING: c_int = 6;
pub const STC_PL_CHARACTER: c_int = 7;
pub const STC_PL_PUNCTUATION: c_int = 8;
pub const STC_PL_PREPROCESSOR: c_int = 9;
pub const STC_PL_OPERATOR: c_int = 10;
pub const STC_PL_IDENTIFIER: c_int = 11;
pub const STC_PL_SCALAR: c_int = 12;
pub const STC_PL_ARRAY: c_int = 13;
pub const STC_PL_HASH: c_int = 14;
pub const STC_PL_SYMBOLTABLE: c_int = 15;
pub const STC_PL_VARIABLE_INDEXER: c_int = 16;
pub const STC_PL_REGEX: c_int = 17;
pub const STC_PL_REGSUBST: c_int = 18;
pub const STC_PL_LONGQUOTE: c_int = 19;
pub const STC_PL_BACKTICKS: c_int = 20;
pub const STC_PL_DATASECTION: c_int = 21;
pub const STC_PL_HERE_DELIM: c_int = 22;
pub const STC_PL_HERE_Q: c_int = 23;
pub const STC_PL_HERE_QQ: c_int = 24;
pub const STC_PL_HERE_QX: c_int = 25;
pub const STC_PL_STRING_Q: c_int = 26;
pub const STC_PL_STRING_QQ: c_int = 27;
pub const STC_PL_STRING_QX: c_int = 28;
pub const STC_PL_STRING_QR: c_int = 29;
pub const STC_PL_STRING_QW: c_int = 30;
pub const STC_PL_POD_VERB: c_int = 31;
pub const STC_PL_SUB_PROTOTYPE: c_int = 40;
pub const STC_PL_FORMAT_IDENT: c_int = 41;
pub const STC_PL_FORMAT: c_int = 42;
pub const STC_PL_STRING_VAR: c_int = 43;
pub const STC_PL_XLAT: c_int = 44;
pub const STC_PL_REGEX_VAR: c_int = 54;
pub const STC_PL_REGSUBST_VAR: c_int = 55;
pub const STC_PL_BACKTICKS_VAR: c_int = 57;
pub const STC_PL_HERE_QQ_VAR: c_int = 61;
pub const STC_PL_HERE_QX_VAR: c_int = 62;
pub const STC_PL_STRING_QQ_VAR: c_int = 64;
pub const STC_PL_STRING_QX_VAR: c_int = 65;
pub const STC_PL_STRING_QR_VAR: c_int = 66;
pub const STC_RB_DEFAULT: c_int = 0;
pub const STC_RB_ERROR: c_int = 1;
pub const STC_RB_COMMENTLINE: c_int = 2;
pub const STC_RB_POD: c_int = 3;
pub const STC_RB_NUMBER: c_int = 4;
pub const STC_RB_WORD: c_int = 5;
pub const STC_RB_STRING: c_int = 6;
pub const STC_RB_CHARACTER: c_int = 7;
pub const STC_RB_CLASSNAME: c_int = 8;
pub const STC_RB_DEFNAME: c_int = 9;
pub const STC_RB_OPERATOR: c_int = 10;
pub const STC_RB_IDENTIFIER: c_int = 11;
pub const STC_RB_REGEX: c_int = 12;
pub const STC_RB_GLOBAL: c_int = 13;
pub const STC_RB_SYMBOL: c_int = 14;
pub const STC_RB_MODULE_NAME: c_int = 15;
pub const STC_RB_INSTANCE_VAR: c_int = 16;
pub const STC_RB_CLASS_VAR: c_int = 17;
pub const STC_RB_BACKTICKS: c_int = 18;
pub const STC_RB_DATASECTION: c_int = 19;
pub const STC_RB_HERE_DELIM: c_int = 20;
pub const STC_RB_HERE_Q: c_int = 21;
pub const STC_RB_HERE_QQ: c_int = 22;
pub const STC_RB_HERE_QX: c_int = 23;
pub const STC_RB_STRING_Q: c_int = 24;
pub const STC_RB_STRING_QQ: c_int = 25;
pub const STC_RB_STRING_QX: c_int = 26;
pub const STC_RB_STRING_QR: c_int = 27;
pub const STC_RB_STRING_QW: c_int = 28;
pub const STC_RB_WORD_DEMOTED: c_int = 29;
pub const STC_RB_STDIN: c_int = 30;
pub const STC_RB_STDOUT: c_int = 31;
pub const STC_RB_STDERR: c_int = 40;
pub const STC_RB_UPPER_BOUND: c_int = 41;
pub const STC_B_DEFAULT: c_int = 0;
pub const STC_B_COMMENT: c_int = 1;
pub const STC_B_NUMBER: c_int = 2;
pub const STC_B_KEYWORD: c_int = 3;
pub const STC_B_STRING: c_int = 4;
pub const STC_B_PREPROCESSOR: c_int = 5;
pub const STC_B_OPERATOR: c_int = 6;
pub const STC_B_IDENTIFIER: c_int = 7;
pub const STC_B_DATE: c_int = 8;
pub const STC_B_STRINGEOL: c_int = 9;
pub const STC_B_KEYWORD2: c_int = 10;
pub const STC_B_KEYWORD3: c_int = 11;
pub const STC_B_KEYWORD4: c_int = 12;
pub const STC_B_CONSTANT: c_int = 13;
pub const STC_B_ASM: c_int = 14;
pub const STC_B_LABEL: c_int = 15;
pub const STC_B_ERROR: c_int = 16;
pub const STC_B_HEXNUMBER: c_int = 17;
pub const STC_B_BINNUMBER: c_int = 18;
pub const STC_B_COMMENTBLOCK: c_int = 19;
pub const STC_B_DOCLINE: c_int = 20;
pub const STC_B_DOCBLOCK: c_int = 21;
pub const STC_B_DOCKEYWORD: c_int = 22;
pub const STC_PROPS_DEFAULT: c_int = 0;
pub const STC_PROPS_COMMENT: c_int = 1;
pub const STC_PROPS_SECTION: c_int = 2;
pub const STC_PROPS_ASSIGNMENT: c_int = 3;
pub const STC_PROPS_DEFVAL: c_int = 4;
pub const STC_PROPS_KEY: c_int = 5;
pub const STC_L_DEFAULT: c_int = 0;
pub const STC_L_COMMAND: c_int = 1;
pub const STC_L_TAG: c_int = 2;
pub const STC_L_MATH: c_int = 3;
pub const STC_L_COMMENT: c_int = 4;
pub const STC_L_TAG2: c_int = 5;
pub const STC_L_MATH2: c_int = 6;
pub const STC_L_COMMENT2: c_int = 7;
pub const STC_L_VERBATIM: c_int = 8;
pub const STC_L_SHORTCMD: c_int = 9;
pub const STC_L_SPECIAL: c_int = 10;
pub const STC_L_CMDOPT: c_int = 11;
pub const STC_L_ERROR: c_int = 12;
pub const STC_LUA_DEFAULT: c_int = 0;
pub const STC_LUA_COMMENT: c_int = 1;
pub const STC_LUA_COMMENTLINE: c_int = 2;
pub const STC_LUA_COMMENTDOC: c_int = 3;
pub const STC_LUA_NUMBER: c_int = 4;
pub const STC_LUA_WORD: c_int = 5;
pub const STC_LUA_STRING: c_int = 6;
pub const STC_LUA_CHARACTER: c_int = 7;
pub const STC_LUA_LITERALSTRING: c_int = 8;
pub const STC_LUA_PREPROCESSOR: c_int = 9;
pub const STC_LUA_OPERATOR: c_int = 10;
pub const STC_LUA_IDENTIFIER: c_int = 11;
pub const STC_LUA_STRINGEOL: c_int = 12;
pub const STC_LUA_WORD2: c_int = 13;
pub const STC_LUA_WORD3: c_int = 14;
pub const STC_LUA_WORD4: c_int = 15;
pub const STC_LUA_WORD5: c_int = 16;
pub const STC_LUA_WORD6: c_int = 17;
pub const STC_LUA_WORD7: c_int = 18;
pub const STC_LUA_WORD8: c_int = 19;
pub const STC_LUA_LABEL: c_int = 20;
pub const STC_ERR_DEFAULT: c_int = 0;
pub const STC_ERR_PYTHON: c_int = 1;
pub const STC_ERR_GCC: c_int = 2;
pub const STC_ERR_MS: c_int = 3;
pub const STC_ERR_CMD: c_int = 4;
pub const STC_ERR_BORLAND: c_int = 5;
pub const STC_ERR_PERL: c_int = 6;
pub const STC_ERR_NET: c_int = 7;
pub const STC_ERR_LUA: c_int = 8;
pub const STC_ERR_CTAG: c_int = 9;
pub const STC_ERR_DIFF_CHANGED: c_int = 10;
pub const STC_ERR_DIFF_ADDITION: c_int = 11;
pub const STC_ERR_DIFF_DELETION: c_int = 12;
pub const STC_ERR_DIFF_MESSAGE: c_int = 13;
pub const STC_ERR_PHP: c_int = 14;
pub const STC_ERR_ELF: c_int = 15;
pub const STC_ERR_IFC: c_int = 16;
pub const STC_ERR_IFORT: c_int = 17;
pub const STC_ERR_ABSF: c_int = 18;
pub const STC_ERR_TIDY: c_int = 19;
pub const STC_ERR_JAVA_STACK: c_int = 20;
pub const STC_ERR_VALUE: c_int = 21;
pub const STC_ERR_GCC_INCLUDED_FROM: c_int = 22;
pub const STC_ERR_ESCSEQ: c_int = 23;
pub const STC_ERR_ESCSEQ_UNKNOWN: c_int = 24;
pub const STC_ERR_ES_BLACK: c_int = 40;
pub const STC_ERR_ES_RED: c_int = 41;
pub const STC_ERR_ES_GREEN: c_int = 42;
pub const STC_ERR_ES_BROWN: c_int = 43;
pub const STC_ERR_ES_BLUE: c_int = 44;
pub const STC_ERR_ES_MAGENTA: c_int = 45;
pub const STC_ERR_ES_CYAN: c_int = 46;
pub const STC_ERR_ES_GRAY: c_int = 47;
pub const STC_ERR_ES_DARK_GRAY: c_int = 48;
pub const STC_ERR_ES_BRIGHT_RED: c_int = 49;
pub const STC_ERR_ES_BRIGHT_GREEN: c_int = 50;
pub const STC_ERR_ES_YELLOW: c_int = 51;
pub const STC_ERR_ES_BRIGHT_BLUE: c_int = 52;
pub const STC_ERR_ES_BRIGHT_MAGENTA: c_int = 53;
pub const STC_ERR_ES_BRIGHT_CYAN: c_int = 54;
pub const STC_ERR_ES_WHITE: c_int = 55;
pub const STC_BAT_DEFAULT: c_int = 0;
pub const STC_BAT_COMMENT: c_int = 1;
pub const STC_BAT_WORD: c_int = 2;
pub const STC_BAT_LABEL: c_int = 3;
pub const STC_BAT_HIDE: c_int = 4;
pub const STC_BAT_COMMAND: c_int = 5;
pub const STC_BAT_IDENTIFIER: c_int = 6;
pub const STC_BAT_OPERATOR: c_int = 7;
pub const STC_TCMD_DEFAULT: c_int = 0;
pub const STC_TCMD_COMMENT: c_int = 1;
pub const STC_TCMD_WORD: c_int = 2;
pub const STC_TCMD_LABEL: c_int = 3;
pub const STC_TCMD_HIDE: c_int = 4;
pub const STC_TCMD_COMMAND: c_int = 5;
pub const STC_TCMD_IDENTIFIER: c_int = 6;
pub const STC_TCMD_OPERATOR: c_int = 7;
pub const STC_TCMD_ENVIRONMENT: c_int = 8;
pub const STC_TCMD_EXPANSION: c_int = 9;
pub const STC_TCMD_CLABEL: c_int = 10;
pub const STC_MAKE_DEFAULT: c_int = 0;
pub const STC_MAKE_COMMENT: c_int = 1;
pub const STC_MAKE_PREPROCESSOR: c_int = 2;
pub const STC_MAKE_IDENTIFIER: c_int = 3;
pub const STC_MAKE_OPERATOR: c_int = 4;
pub const STC_MAKE_TARGET: c_int = 5;
pub const STC_MAKE_IDEOL: c_int = 9;
pub const STC_DIFF_DEFAULT: c_int = 0;
pub const STC_DIFF_COMMENT: c_int = 1;
pub const STC_DIFF_COMMAND: c_int = 2;
pub const STC_DIFF_HEADER: c_int = 3;
pub const STC_DIFF_POSITION: c_int = 4;
pub const STC_DIFF_DELETED: c_int = 5;
pub const STC_DIFF_ADDED: c_int = 6;
pub const STC_DIFF_CHANGED: c_int = 7;
pub const STC_CONF_DEFAULT: c_int = 0;
pub const STC_CONF_COMMENT: c_int = 1;
pub const STC_CONF_NUMBER: c_int = 2;
pub const STC_CONF_IDENTIFIER: c_int = 3;
pub const STC_CONF_EXTENSION: c_int = 4;
pub const STC_CONF_PARAMETER: c_int = 5;
pub const STC_CONF_STRING: c_int = 6;
pub const STC_CONF_OPERATOR: c_int = 7;
pub const STC_CONF_IP: c_int = 8;
pub const STC_CONF_DIRECTIVE: c_int = 9;
pub const STC_AVE_DEFAULT: c_int = 0;
pub const STC_AVE_COMMENT: c_int = 1;
pub const STC_AVE_NUMBER: c_int = 2;
pub const STC_AVE_WORD: c_int = 3;
pub const STC_AVE_STRING: c_int = 6;
pub const STC_AVE_ENUM: c_int = 7;
pub const STC_AVE_STRINGEOL: c_int = 8;
pub const STC_AVE_IDENTIFIER: c_int = 9;
pub const STC_AVE_OPERATOR: c_int = 10;
pub const STC_AVE_WORD1: c_int = 11;
pub const STC_AVE_WORD2: c_int = 12;
pub const STC_AVE_WORD3: c_int = 13;
pub const STC_AVE_WORD4: c_int = 14;
pub const STC_AVE_WORD5: c_int = 15;
pub const STC_AVE_WORD6: c_int = 16;
pub const STC_ADA_DEFAULT: c_int = 0;
pub const STC_ADA_WORD: c_int = 1;
pub const STC_ADA_IDENTIFIER: c_int = 2;
pub const STC_ADA_NUMBER: c_int = 3;
pub const STC_ADA_DELIMITER: c_int = 4;
pub const STC_ADA_CHARACTER: c_int = 5;
pub const STC_ADA_CHARACTEREOL: c_int = 6;
pub const STC_ADA_STRING: c_int = 7;
pub const STC_ADA_STRINGEOL: c_int = 8;
pub const STC_ADA_LABEL: c_int = 9;
pub const STC_ADA_COMMENTLINE: c_int = 10;
pub const STC_ADA_ILLEGAL: c_int = 11;
pub const STC_BAAN_DEFAULT: c_int = 0;
pub const STC_BAAN_COMMENT: c_int = 1;
pub const STC_BAAN_COMMENTDOC: c_int = 2;
pub const STC_BAAN_NUMBER: c_int = 3;
pub const STC_BAAN_WORD: c_int = 4;
pub const STC_BAAN_STRING: c_int = 5;
pub const STC_BAAN_PREPROCESSOR: c_int = 6;
pub const STC_BAAN_OPERATOR: c_int = 7;
pub const STC_BAAN_IDENTIFIER: c_int = 8;
pub const STC_BAAN_STRINGEOL: c_int = 9;
pub const STC_BAAN_WORD2: c_int = 10;
pub const STC_BAAN_WORD3: c_int = 11;
pub const STC_BAAN_WORD4: c_int = 12;
pub const STC_BAAN_WORD5: c_int = 13;
pub const STC_BAAN_WORD6: c_int = 14;
pub const STC_BAAN_WORD7: c_int = 15;
pub const STC_BAAN_WORD8: c_int = 16;
pub const STC_BAAN_WORD9: c_int = 17;
pub const STC_BAAN_TABLEDEF: c_int = 18;
pub const STC_BAAN_TABLESQL: c_int = 19;
pub const STC_BAAN_FUNCTION: c_int = 20;
pub const STC_BAAN_DOMDEF: c_int = 21;
pub const STC_BAAN_FUNCDEF: c_int = 22;
pub const STC_BAAN_OBJECTDEF: c_int = 23;
pub const STC_BAAN_DEFINEDEF: c_int = 24;
pub const STC_LISP_DEFAULT: c_int = 0;
pub const STC_LISP_COMMENT: c_int = 1;
pub const STC_LISP_NUMBER: c_int = 2;
pub const STC_LISP_KEYWORD: c_int = 3;
pub const STC_LISP_KEYWORD_KW: c_int = 4;
pub const STC_LISP_SYMBOL: c_int = 5;
pub const STC_LISP_STRING: c_int = 6;
pub const STC_LISP_STRINGEOL: c_int = 8;
pub const STC_LISP_IDENTIFIER: c_int = 9;
pub const STC_LISP_OPERATOR: c_int = 10;
pub const STC_LISP_SPECIAL: c_int = 11;
pub const STC_LISP_MULTI_COMMENT: c_int = 12;
pub const STC_EIFFEL_DEFAULT: c_int = 0;
pub const STC_EIFFEL_COMMENTLINE: c_int = 1;
pub const STC_EIFFEL_NUMBER: c_int = 2;
pub const STC_EIFFEL_WORD: c_int = 3;
pub const STC_EIFFEL_STRING: c_int = 4;
pub const STC_EIFFEL_CHARACTER: c_int = 5;
pub const STC_EIFFEL_OPERATOR: c_int = 6;
pub const STC_EIFFEL_IDENTIFIER: c_int = 7;
pub const STC_EIFFEL_STRINGEOL: c_int = 8;
pub const STC_NNCRONTAB_DEFAULT: c_int = 0;
pub const STC_NNCRONTAB_COMMENT: c_int = 1;
pub const STC_NNCRONTAB_TASK: c_int = 2;
pub const STC_NNCRONTAB_SECTION: c_int = 3;
pub const STC_NNCRONTAB_KEYWORD: c_int = 4;
pub const STC_NNCRONTAB_MODIFIER: c_int = 5;
pub const STC_NNCRONTAB_ASTERISK: c_int = 6;
pub const STC_NNCRONTAB_NUMBER: c_int = 7;
pub const STC_NNCRONTAB_STRING: c_int = 8;
pub const STC_NNCRONTAB_ENVIRONMENT: c_int = 9;
pub const STC_NNCRONTAB_IDENTIFIER: c_int = 10;
pub const STC_FORTH_DEFAULT: c_int = 0;
pub const STC_FORTH_COMMENT: c_int = 1;
pub const STC_FORTH_COMMENT_ML: c_int = 2;
pub const STC_FORTH_IDENTIFIER: c_int = 3;
pub const STC_FORTH_CONTROL: c_int = 4;
pub const STC_FORTH_KEYWORD: c_int = 5;
pub const STC_FORTH_DEFWORD: c_int = 6;
pub const STC_FORTH_PREWORD1: c_int = 7;
pub const STC_FORTH_PREWORD2: c_int = 8;
pub const STC_FORTH_NUMBER: c_int = 9;
pub const STC_FORTH_STRING: c_int = 10;
pub const STC_FORTH_LOCALE: c_int = 11;
pub const STC_MATLAB_DEFAULT: c_int = 0;
pub const STC_MATLAB_COMMENT: c_int = 1;
pub const STC_MATLAB_COMMAND: c_int = 2;
pub const STC_MATLAB_NUMBER: c_int = 3;
pub const STC_MATLAB_KEYWORD: c_int = 4;
pub const STC_MATLAB_STRING: c_int = 5;
pub const STC_MATLAB_OPERATOR: c_int = 6;
pub const STC_MATLAB_IDENTIFIER: c_int = 7;
pub const STC_MATLAB_DOUBLEQUOTESTRING: c_int = 8;
pub const STC_SCRIPTOL_DEFAULT: c_int = 0;
pub const STC_SCRIPTOL_WHITE: c_int = 1;
pub const STC_SCRIPTOL_COMMENTLINE: c_int = 2;
pub const STC_SCRIPTOL_PERSISTENT: c_int = 3;
pub const STC_SCRIPTOL_CSTYLE: c_int = 4;
pub const STC_SCRIPTOL_COMMENTBLOCK: c_int = 5;
pub const STC_SCRIPTOL_NUMBER: c_int = 6;
pub const STC_SCRIPTOL_STRING: c_int = 7;
pub const STC_SCRIPTOL_CHARACTER: c_int = 8;
pub const STC_SCRIPTOL_STRINGEOL: c_int = 9;
pub const STC_SCRIPTOL_KEYWORD: c_int = 10;
pub const STC_SCRIPTOL_OPERATOR: c_int = 11;
pub const STC_SCRIPTOL_IDENTIFIER: c_int = 12;
pub const STC_SCRIPTOL_TRIPLE: c_int = 13;
pub const STC_SCRIPTOL_CLASSNAME: c_int = 14;
pub const STC_SCRIPTOL_PREPROCESSOR: c_int = 15;
pub const STC_ASM_DEFAULT: c_int = 0;
pub const STC_ASM_COMMENT: c_int = 1;
pub const STC_ASM_NUMBER: c_int = 2;
pub const STC_ASM_STRING: c_int = 3;
pub const STC_ASM_OPERATOR: c_int = 4;
pub const STC_ASM_IDENTIFIER: c_int = 5;
pub const STC_ASM_CPUINSTRUCTION: c_int = 6;
pub const STC_ASM_MATHINSTRUCTION: c_int = 7;
pub const STC_ASM_REGISTER: c_int = 8;
pub const STC_ASM_DIRECTIVE: c_int = 9;
pub const STC_ASM_DIRECTIVEOPERAND: c_int = 10;
pub const STC_ASM_COMMENTBLOCK: c_int = 11;
pub const STC_ASM_CHARACTER: c_int = 12;
pub const STC_ASM_STRINGEOL: c_int = 13;
pub const STC_ASM_EXTINSTRUCTION: c_int = 14;
pub const STC_ASM_COMMENTDIRECTIVE: c_int = 15;
pub const STC_F_DEFAULT: c_int = 0;
pub const STC_F_COMMENT: c_int = 1;
pub const STC_F_NUMBER: c_int = 2;
pub const STC_F_STRING1: c_int = 3;
pub const STC_F_STRING2: c_int = 4;
pub const STC_F_STRINGEOL: c_int = 5;
pub const STC_F_OPERATOR: c_int = 6;
pub const STC_F_IDENTIFIER: c_int = 7;
pub const STC_F_WORD: c_int = 8;
pub const STC_F_WORD2: c_int = 9;
pub const STC_F_WORD3: c_int = 10;
pub const STC_F_PREPROCESSOR: c_int = 11;
pub const STC_F_OPERATOR2: c_int = 12;
pub const STC_F_LABEL: c_int = 13;
pub const STC_F_CONTINUATION: c_int = 14;
pub const STC_CSS_DEFAULT: c_int = 0;
pub const STC_CSS_TAG: c_int = 1;
pub const STC_CSS_CLASS: c_int = 2;
pub const STC_CSS_PSEUDOCLASS: c_int = 3;
pub const STC_CSS_UNKNOWN_PSEUDOCLASS: c_int = 4;
pub const STC_CSS_OPERATOR: c_int = 5;
pub const STC_CSS_IDENTIFIER: c_int = 6;
pub const STC_CSS_UNKNOWN_IDENTIFIER: c_int = 7;
pub const STC_CSS_VALUE: c_int = 8;
pub const STC_CSS_COMMENT: c_int = 9;
pub const STC_CSS_ID: c_int = 10;
pub const STC_CSS_IMPORTANT: c_int = 11;
pub const STC_CSS_DIRECTIVE: c_int = 12;
pub const STC_CSS_DOUBLESTRING: c_int = 13;
pub const STC_CSS_SINGLESTRING: c_int = 14;
pub const STC_CSS_IDENTIFIER2: c_int = 15;
pub const STC_CSS_ATTRIBUTE: c_int = 16;
pub const STC_CSS_IDENTIFIER3: c_int = 17;
pub const STC_CSS_PSEUDOELEMENT: c_int = 18;
pub const STC_CSS_EXTENDED_IDENTIFIER: c_int = 19;
pub const STC_CSS_EXTENDED_PSEUDOCLASS: c_int = 20;
pub const STC_CSS_EXTENDED_PSEUDOELEMENT: c_int = 21;
pub const STC_CSS_MEDIA: c_int = 22;
pub const STC_CSS_VARIABLE: c_int = 23;
pub const STC_POV_DEFAULT: c_int = 0;
pub const STC_POV_COMMENT: c_int = 1;
pub const STC_POV_COMMENTLINE: c_int = 2;
pub const STC_POV_NUMBER: c_int = 3;
pub const STC_POV_OPERATOR: c_int = 4;
pub const STC_POV_IDENTIFIER: c_int = 5;
pub const STC_POV_STRING: c_int = 6;
pub const STC_POV_STRINGEOL: c_int = 7;
pub const STC_POV_DIRECTIVE: c_int = 8;
pub const STC_POV_BADDIRECTIVE: c_int = 9;
pub const STC_POV_WORD2: c_int = 10;
pub const STC_POV_WORD3: c_int = 11;
pub const STC_POV_WORD4: c_int = 12;
pub const STC_POV_WORD5: c_int = 13;
pub const STC_POV_WORD6: c_int = 14;
pub const STC_POV_WORD7: c_int = 15;
pub const STC_POV_WORD8: c_int = 16;
pub const STC_LOUT_DEFAULT: c_int = 0;
pub const STC_LOUT_COMMENT: c_int = 1;
pub const STC_LOUT_NUMBER: c_int = 2;
pub const STC_LOUT_WORD: c_int = 3;
pub const STC_LOUT_WORD2: c_int = 4;
pub const STC_LOUT_WORD3: c_int = 5;
pub const STC_LOUT_WORD4: c_int = 6;
pub const STC_LOUT_STRING: c_int = 7;
pub const STC_LOUT_OPERATOR: c_int = 8;
pub const STC_LOUT_IDENTIFIER: c_int = 9;
pub const STC_LOUT_STRINGEOL: c_int = 10;
pub const STC_ESCRIPT_DEFAULT: c_int = 0;
pub const STC_ESCRIPT_COMMENT: c_int = 1;
pub const STC_ESCRIPT_COMMENTLINE: c_int = 2;
pub const STC_ESCRIPT_COMMENTDOC: c_int = 3;
pub const STC_ESCRIPT_NUMBER: c_int = 4;
pub const STC_ESCRIPT_WORD: c_int = 5;
pub const STC_ESCRIPT_STRING: c_int = 6;
pub const STC_ESCRIPT_OPERATOR: c_int = 7;
pub const STC_ESCRIPT_IDENTIFIER: c_int = 8;
pub const STC_ESCRIPT_BRACE: c_int = 9;
pub const STC_ESCRIPT_WORD2: c_int = 10;
pub const STC_ESCRIPT_WORD3: c_int = 11;
pub const STC_PS_DEFAULT: c_int = 0;
pub const STC_PS_COMMENT: c_int = 1;
pub const STC_PS_DSC_COMMENT: c_int = 2;
pub const STC_PS_DSC_VALUE: c_int = 3;
pub const STC_PS_NUMBER: c_int = 4;
pub const STC_PS_NAME: c_int = 5;
pub const STC_PS_KEYWORD: c_int = 6;
pub const STC_PS_LITERAL: c_int = 7;
pub const STC_PS_IMMEVAL: c_int = 8;
pub const STC_PS_PAREN_ARRAY: c_int = 9;
pub const STC_PS_PAREN_DICT: c_int = 10;
pub const STC_PS_PAREN_PROC: c_int = 11;
pub const STC_PS_TEXT: c_int = 12;
pub const STC_PS_HEXSTRING: c_int = 13;
pub const STC_PS_BASE85STRING: c_int = 14;
pub const STC_PS_BADSTRINGCHAR: c_int = 15;
pub const STC_NSIS_DEFAULT: c_int = 0;
pub const STC_NSIS_COMMENT: c_int = 1;
pub const STC_NSIS_STRINGDQ: c_int = 2;
pub const STC_NSIS_STRINGLQ: c_int = 3;
pub const STC_NSIS_STRINGRQ: c_int = 4;
pub const STC_NSIS_FUNCTION: c_int = 5;
pub const STC_NSIS_VARIABLE: c_int = 6;
pub const STC_NSIS_LABEL: c_int = 7;
pub const STC_NSIS_USERDEFINED: c_int = 8;
pub const STC_NSIS_SECTIONDEF: c_int = 9;
pub const STC_NSIS_SUBSECTIONDEF: c_int = 10;
pub const STC_NSIS_IFDEFINEDEF: c_int = 11;
pub const STC_NSIS_MACRODEF: c_int = 12;
pub const STC_NSIS_STRINGVAR: c_int = 13;
pub const STC_NSIS_NUMBER: c_int = 14;
pub const STC_NSIS_SECTIONGROUP: c_int = 15;
pub const STC_NSIS_PAGEEX: c_int = 16;
pub const STC_NSIS_FUNCTIONDEF: c_int = 17;
pub const STC_NSIS_COMMENTBOX: c_int = 18;
pub const STC_MMIXAL_LEADWS: c_int = 0;
pub const STC_MMIXAL_COMMENT: c_int = 1;
pub const STC_MMIXAL_LABEL: c_int = 2;
pub const STC_MMIXAL_OPCODE: c_int = 3;
pub const STC_MMIXAL_OPCODE_PRE: c_int = 4;
pub const STC_MMIXAL_OPCODE_VALID: c_int = 5;
pub const STC_MMIXAL_OPCODE_UNKNOWN: c_int = 6;
pub const STC_MMIXAL_OPCODE_POST: c_int = 7;
pub const STC_MMIXAL_OPERANDS: c_int = 8;
pub const STC_MMIXAL_NUMBER: c_int = 9;
pub const STC_MMIXAL_REF: c_int = 10;
pub const STC_MMIXAL_CHAR: c_int = 11;
pub const STC_MMIXAL_STRING: c_int = 12;
pub const STC_MMIXAL_REGISTER: c_int = 13;
pub const STC_MMIXAL_HEX: c_int = 14;
pub const STC_MMIXAL_OPERATOR: c_int = 15;
pub const STC_MMIXAL_SYMBOL: c_int = 16;
pub const STC_MMIXAL_INCLUDE: c_int = 17;
pub const STC_CLW_DEFAULT: c_int = 0;
pub const STC_CLW_LABEL: c_int = 1;
pub const STC_CLW_COMMENT: c_int = 2;
pub const STC_CLW_STRING: c_int = 3;
pub const STC_CLW_USER_IDENTIFIER: c_int = 4;
pub const STC_CLW_INTEGER_CONSTANT: c_int = 5;
pub const STC_CLW_REAL_CONSTANT: c_int = 6;
pub const STC_CLW_PICTURE_STRING: c_int = 7;
pub const STC_CLW_KEYWORD: c_int = 8;
pub const STC_CLW_COMPILER_DIRECTIVE: c_int = 9;
pub const STC_CLW_RUNTIME_EXPRESSIONS: c_int = 10;
pub const STC_CLW_BUILTIN_PROCEDURES_FUNCTION: c_int = 11;
pub const STC_CLW_STRUCTURE_DATA_TYPE: c_int = 12;
pub const STC_CLW_ATTRIBUTE: c_int = 13;
pub const STC_CLW_STANDARD_EQUATE: c_int = 14;
pub const STC_CLW_ERROR: c_int = 15;
pub const STC_CLW_DEPRECATED: c_int = 16;
pub const STC_LOT_DEFAULT: c_int = 0;
pub const STC_LOT_HEADER: c_int = 1;
pub const STC_LOT_BREAK: c_int = 2;
pub const STC_LOT_SET: c_int = 3;
pub const STC_LOT_PASS: c_int = 4;
pub const STC_LOT_FAIL: c_int = 5;
pub const STC_LOT_ABORT: c_int = 6;
pub const STC_YAML_DEFAULT: c_int = 0;
pub const STC_YAML_COMMENT: c_int = 1;
pub const STC_YAML_IDENTIFIER: c_int = 2;
pub const STC_YAML_KEYWORD: c_int = 3;
pub const STC_YAML_NUMBER: c_int = 4;
pub const STC_YAML_REFERENCE: c_int = 5;
pub const STC_YAML_DOCUMENT: c_int = 6;
pub const STC_YAML_TEXT: c_int = 7;
pub const STC_YAML_ERROR: c_int = 8;
pub const STC_YAML_OPERATOR: c_int = 9;
pub const STC_TEX_DEFAULT: c_int = 0;
pub const STC_TEX_SPECIAL: c_int = 1;
pub const STC_TEX_GROUP: c_int = 2;
pub const STC_TEX_SYMBOL: c_int = 3;
pub const STC_TEX_COMMAND: c_int = 4;
pub const STC_TEX_TEXT: c_int = 5;
pub const STC_METAPOST_DEFAULT: c_int = 0;
pub const STC_METAPOST_SPECIAL: c_int = 1;
pub const STC_METAPOST_GROUP: c_int = 2;
pub const STC_METAPOST_SYMBOL: c_int = 3;
pub const STC_METAPOST_COMMAND: c_int = 4;
pub const STC_METAPOST_TEXT: c_int = 5;
pub const STC_METAPOST_EXTRA: c_int = 6;
pub const STC_ERLANG_DEFAULT: c_int = 0;
pub const STC_ERLANG_COMMENT: c_int = 1;
pub const STC_ERLANG_VARIABLE: c_int = 2;
pub const STC_ERLANG_NUMBER: c_int = 3;
pub const STC_ERLANG_KEYWORD: c_int = 4;
pub const STC_ERLANG_STRING: c_int = 5;
pub const STC_ERLANG_OPERATOR: c_int = 6;
pub const STC_ERLANG_ATOM: c_int = 7;
pub const STC_ERLANG_FUNCTION_NAME: c_int = 8;
pub const STC_ERLANG_CHARACTER: c_int = 9;
pub const STC_ERLANG_MACRO: c_int = 10;
pub const STC_ERLANG_RECORD: c_int = 11;
pub const STC_ERLANG_PREPROC: c_int = 12;
pub const STC_ERLANG_NODE_NAME: c_int = 13;
pub const STC_ERLANG_COMMENT_FUNCTION: c_int = 14;
pub const STC_ERLANG_COMMENT_MODULE: c_int = 15;
pub const STC_ERLANG_COMMENT_DOC: c_int = 16;
pub const STC_ERLANG_COMMENT_DOC_MACRO: c_int = 17;
pub const STC_ERLANG_ATOM_QUOTED: c_int = 18;
pub const STC_ERLANG_MACRO_QUOTED: c_int = 19;
pub const STC_ERLANG_RECORD_QUOTED: c_int = 20;
pub const STC_ERLANG_NODE_NAME_QUOTED: c_int = 21;
pub const STC_ERLANG_BIFS: c_int = 22;
pub const STC_ERLANG_MODULES: c_int = 23;
pub const STC_ERLANG_MODULES_ATT: c_int = 24;
pub const STC_ERLANG_UNKNOWN: c_int = 31;
pub const STC_MSSQL_DEFAULT: c_int = 0;
pub const STC_MSSQL_COMMENT: c_int = 1;
pub const STC_MSSQL_LINE_COMMENT: c_int = 2;
pub const STC_MSSQL_NUMBER: c_int = 3;
pub const STC_MSSQL_STRING: c_int = 4;
pub const STC_MSSQL_OPERATOR: c_int = 5;
pub const STC_MSSQL_IDENTIFIER: c_int = 6;
pub const STC_MSSQL_VARIABLE: c_int = 7;
pub const STC_MSSQL_COLUMN_NAME: c_int = 8;
pub const STC_MSSQL_STATEMENT: c_int = 9;
pub const STC_MSSQL_DATATYPE: c_int = 10;
pub const STC_MSSQL_SYSTABLE: c_int = 11;
pub const STC_MSSQL_GLOBAL_VARIABLE: c_int = 12;
pub const STC_MSSQL_FUNCTION: c_int = 13;
pub const STC_MSSQL_STORED_PROCEDURE: c_int = 14;
pub const STC_MSSQL_DEFAULT_PREF_DATATYPE: c_int = 15;
pub const STC_MSSQL_COLUMN_NAME_2: c_int = 16;
pub const STC_V_DEFAULT: c_int = 0;
pub const STC_V_COMMENT: c_int = 1;
pub const STC_V_COMMENTLINE: c_int = 2;
pub const STC_V_COMMENTLINEBANG: c_int = 3;
pub const STC_V_NUMBER: c_int = 4;
pub const STC_V_WORD: c_int = 5;
pub const STC_V_STRING: c_int = 6;
pub const STC_V_WORD2: c_int = 7;
pub const STC_V_WORD3: c_int = 8;
pub const STC_V_PREPROCESSOR: c_int = 9;
pub const STC_V_OPERATOR: c_int = 10;
pub const STC_V_IDENTIFIER: c_int = 11;
pub const STC_V_STRINGEOL: c_int = 12;
pub const STC_V_USER: c_int = 19;
pub const STC_V_COMMENT_WORD: c_int = 20;
pub const STC_V_INPUT: c_int = 21;
pub const STC_V_OUTPUT: c_int = 22;
pub const STC_V_INOUT: c_int = 23;
pub const STC_V_PORT_CONNECT: c_int = 24;
pub const STC_KIX_DEFAULT: c_int = 0;
pub const STC_KIX_COMMENT: c_int = 1;
pub const STC_KIX_STRING1: c_int = 2;
pub const STC_KIX_STRING2: c_int = 3;
pub const STC_KIX_NUMBER: c_int = 4;
pub const STC_KIX_VAR: c_int = 5;
pub const STC_KIX_MACRO: c_int = 6;
pub const STC_KIX_KEYWORD: c_int = 7;
pub const STC_KIX_FUNCTIONS: c_int = 8;
pub const STC_KIX_OPERATOR: c_int = 9;
pub const STC_KIX_COMMENTSTREAM: c_int = 10;
pub const STC_KIX_IDENTIFIER: c_int = 31;
pub const STC_GC_DEFAULT: c_int = 0;
pub const STC_GC_COMMENTLINE: c_int = 1;
pub const STC_GC_COMMENTBLOCK: c_int = 2;
pub const STC_GC_GLOBAL: c_int = 3;
pub const STC_GC_EVENT: c_int = 4;
pub const STC_GC_ATTRIBUTE: c_int = 5;
pub const STC_GC_CONTROL: c_int = 6;
pub const STC_GC_COMMAND: c_int = 7;
pub const STC_GC_STRING: c_int = 8;
pub const STC_GC_OPERATOR: c_int = 9;
pub const STC_SN_DEFAULT: c_int = 0;
pub const STC_SN_CODE: c_int = 1;
pub const STC_SN_COMMENTLINE: c_int = 2;
pub const STC_SN_COMMENTLINEBANG: c_int = 3;
pub const STC_SN_NUMBER: c_int = 4;
pub const STC_SN_WORD: c_int = 5;
pub const STC_SN_STRING: c_int = 6;
pub const STC_SN_WORD2: c_int = 7;
pub const STC_SN_WORD3: c_int = 8;
pub const STC_SN_PREPROCESSOR: c_int = 9;
pub const STC_SN_OPERATOR: c_int = 10;
pub const STC_SN_IDENTIFIER: c_int = 11;
pub const STC_SN_STRINGEOL: c_int = 12;
pub const STC_SN_REGEXTAG: c_int = 13;
pub const STC_SN_SIGNAL: c_int = 14;
pub const STC_SN_USER: c_int = 19;
pub const STC_AU3_DEFAULT: c_int = 0;
pub const STC_AU3_COMMENT: c_int = 1;
pub const STC_AU3_COMMENTBLOCK: c_int = 2;
pub const STC_AU3_NUMBER: c_int = 3;
pub const STC_AU3_FUNCTION: c_int = 4;
pub const STC_AU3_KEYWORD: c_int = 5;
pub const STC_AU3_MACRO: c_int = 6;
pub const STC_AU3_STRING: c_int = 7;
pub const STC_AU3_OPERATOR: c_int = 8;
pub const STC_AU3_VARIABLE: c_int = 9;
pub const STC_AU3_SENT: c_int = 10;
pub const STC_AU3_PREPROCESSOR: c_int = 11;
pub const STC_AU3_SPECIAL: c_int = 12;
pub const STC_AU3_EXPAND: c_int = 13;
pub const STC_AU3_COMOBJ: c_int = 14;
pub const STC_AU3_UDF: c_int = 15;
pub const STC_APDL_DEFAULT: c_int = 0;
pub const STC_APDL_COMMENT: c_int = 1;
pub const STC_APDL_COMMENTBLOCK: c_int = 2;
pub const STC_APDL_NUMBER: c_int = 3;
pub const STC_APDL_STRING: c_int = 4;
pub const STC_APDL_OPERATOR: c_int = 5;
pub const STC_APDL_WORD: c_int = 6;
pub const STC_APDL_PROCESSOR: c_int = 7;
pub const STC_APDL_COMMAND: c_int = 8;
pub const STC_APDL_SLASHCOMMAND: c_int = 9;
pub const STC_APDL_STARCOMMAND: c_int = 10;
pub const STC_APDL_ARGUMENT: c_int = 11;
pub const STC_APDL_FUNCTION: c_int = 12;
pub const STC_SH_DEFAULT: c_int = 0;
pub const STC_SH_ERROR: c_int = 1;
pub const STC_SH_COMMENTLINE: c_int = 2;
pub const STC_SH_NUMBER: c_int = 3;
pub const STC_SH_WORD: c_int = 4;
pub const STC_SH_STRING: c_int = 5;
pub const STC_SH_CHARACTER: c_int = 6;
pub const STC_SH_OPERATOR: c_int = 7;
pub const STC_SH_IDENTIFIER: c_int = 8;
pub const STC_SH_SCALAR: c_int = 9;
pub const STC_SH_PARAM: c_int = 10;
pub const STC_SH_BACKTICKS: c_int = 11;
pub const STC_SH_HERE_DELIM: c_int = 12;
pub const STC_SH_HERE_Q: c_int = 13;
pub const STC_ASN1_DEFAULT: c_int = 0;
pub const STC_ASN1_COMMENT: c_int = 1;
pub const STC_ASN1_IDENTIFIER: c_int = 2;
pub const STC_ASN1_STRING: c_int = 3;
pub const STC_ASN1_OID: c_int = 4;
pub const STC_ASN1_SCALAR: c_int = 5;
pub const STC_ASN1_KEYWORD: c_int = 6;
pub const STC_ASN1_ATTRIBUTE: c_int = 7;
pub const STC_ASN1_DESCRIPTOR: c_int = 8;
pub const STC_ASN1_TYPE: c_int = 9;
pub const STC_ASN1_OPERATOR: c_int = 10;
pub const STC_VHDL_DEFAULT: c_int = 0;
pub const STC_VHDL_COMMENT: c_int = 1;
pub const STC_VHDL_COMMENTLINEBANG: c_int = 2;
pub const STC_VHDL_NUMBER: c_int = 3;
pub const STC_VHDL_STRING: c_int = 4;
pub const STC_VHDL_OPERATOR: c_int = 5;
pub const STC_VHDL_IDENTIFIER: c_int = 6;
pub const STC_VHDL_STRINGEOL: c_int = 7;
pub const STC_VHDL_KEYWORD: c_int = 8;
pub const STC_VHDL_STDOPERATOR: c_int = 9;
pub const STC_VHDL_ATTRIBUTE: c_int = 10;
pub const STC_VHDL_STDFUNCTION: c_int = 11;
pub const STC_VHDL_STDPACKAGE: c_int = 12;
pub const STC_VHDL_STDTYPE: c_int = 13;
pub const STC_VHDL_USERWORD: c_int = 14;
pub const STC_VHDL_BLOCK_COMMENT: c_int = 15;
pub const STC_CAML_DEFAULT: c_int = 0;
pub const STC_CAML_IDENTIFIER: c_int = 1;
pub const STC_CAML_TAGNAME: c_int = 2;
pub const STC_CAML_KEYWORD: c_int = 3;
pub const STC_CAML_KEYWORD2: c_int = 4;
pub const STC_CAML_KEYWORD3: c_int = 5;
pub const STC_CAML_LINENUM: c_int = 6;
pub const STC_CAML_OPERATOR: c_int = 7;
pub const STC_CAML_NUMBER: c_int = 8;
pub const STC_CAML_CHAR: c_int = 9;
pub const STC_CAML_WHITE: c_int = 10;
pub const STC_CAML_STRING: c_int = 11;
pub const STC_CAML_COMMENT: c_int = 12;
pub const STC_CAML_COMMENT1: c_int = 13;
pub const STC_CAML_COMMENT2: c_int = 14;
pub const STC_CAML_COMMENT3: c_int = 15;
pub const STC_HA_DEFAULT: c_int = 0;
pub const STC_HA_IDENTIFIER: c_int = 1;
pub const STC_HA_KEYWORD: c_int = 2;
pub const STC_HA_NUMBER: c_int = 3;
pub const STC_HA_STRING: c_int = 4;
pub const STC_HA_CHARACTER: c_int = 5;
pub const STC_HA_CLASS: c_int = 6;
pub const STC_HA_MODULE: c_int = 7;
pub const STC_HA_CAPITAL: c_int = 8;
pub const STC_HA_DATA: c_int = 9;
pub const STC_HA_IMPORT: c_int = 10;
pub const STC_HA_OPERATOR: c_int = 11;
pub const STC_HA_INSTANCE: c_int = 12;
pub const STC_HA_COMMENTLINE: c_int = 13;
pub const STC_HA_COMMENTBLOCK: c_int = 14;
pub const STC_HA_COMMENTBLOCK2: c_int = 15;
pub const STC_HA_COMMENTBLOCK3: c_int = 16;
pub const STC_HA_PRAGMA: c_int = 17;
pub const STC_HA_PREPROCESSOR: c_int = 18;
pub const STC_HA_STRINGEOL: c_int = 19;
pub const STC_HA_RESERVED_OPERATOR: c_int = 20;
pub const STC_HA_LITERATE_COMMENT: c_int = 21;
pub const STC_HA_LITERATE_CODEDELIM: c_int = 22;
pub const STC_T3_DEFAULT: c_int = 0;
pub const STC_T3_X_DEFAULT: c_int = 1;
pub const STC_T3_PREPROCESSOR: c_int = 2;
pub const STC_T3_BLOCK_COMMENT: c_int = 3;
pub const STC_T3_LINE_COMMENT: c_int = 4;
pub const STC_T3_OPERATOR: c_int = 5;
pub const STC_T3_KEYWORD: c_int = 6;
pub const STC_T3_NUMBER: c_int = 7;
pub const STC_T3_IDENTIFIER: c_int = 8;
pub const STC_T3_S_STRING: c_int = 9;
pub const STC_T3_D_STRING: c_int = 10;
pub const STC_T3_X_STRING: c_int = 11;
pub const STC_T3_LIB_DIRECTIVE: c_int = 12;
pub const STC_T3_MSG_PARAM: c_int = 13;
pub const STC_T3_HTML_TAG: c_int = 14;
pub const STC_T3_HTML_DEFAULT: c_int = 15;
pub const STC_T3_HTML_STRING: c_int = 16;
pub const STC_T3_USER1: c_int = 17;
pub const STC_T3_USER2: c_int = 18;
pub const STC_T3_USER3: c_int = 19;
pub const STC_T3_BRACE: c_int = 20;
pub const STC_REBOL_DEFAULT: c_int = 0;
pub const STC_REBOL_COMMENTLINE: c_int = 1;
pub const STC_REBOL_COMMENTBLOCK: c_int = 2;
pub const STC_REBOL_PREFACE: c_int = 3;
pub const STC_REBOL_OPERATOR: c_int = 4;
pub const STC_REBOL_CHARACTER: c_int = 5;
pub const STC_REBOL_QUOTEDSTRING: c_int = 6;
pub const STC_REBOL_BRACEDSTRING: c_int = 7;
pub const STC_REBOL_NUMBER: c_int = 8;
pub const STC_REBOL_PAIR: c_int = 9;
pub const STC_REBOL_TUPLE: c_int = 10;
pub const STC_REBOL_BINARY: c_int = 11;
pub const STC_REBOL_MONEY: c_int = 12;
pub const STC_REBOL_ISSUE: c_int = 13;
pub const STC_REBOL_TAG: c_int = 14;
pub const STC_REBOL_FILE: c_int = 15;
pub const STC_REBOL_EMAIL: c_int = 16;
pub const STC_REBOL_URL: c_int = 17;
pub const STC_REBOL_DATE: c_int = 18;
pub const STC_REBOL_TIME: c_int = 19;
pub const STC_REBOL_IDENTIFIER: c_int = 20;
pub const STC_REBOL_WORD: c_int = 21;
pub const STC_REBOL_WORD2: c_int = 22;
pub const STC_REBOL_WORD3: c_int = 23;
pub const STC_REBOL_WORD4: c_int = 24;
pub const STC_REBOL_WORD5: c_int = 25;
pub const STC_REBOL_WORD6: c_int = 26;
pub const STC_REBOL_WORD7: c_int = 27;
pub const STC_REBOL_WORD8: c_int = 28;
pub const STC_SQL_DEFAULT: c_int = 0;
pub const STC_SQL_COMMENT: c_int = 1;
pub const STC_SQL_COMMENTLINE: c_int = 2;
pub const STC_SQL_COMMENTDOC: c_int = 3;
pub const STC_SQL_NUMBER: c_int = 4;
pub const STC_SQL_WORD: c_int = 5;
pub const STC_SQL_STRING: c_int = 6;
pub const STC_SQL_CHARACTER: c_int = 7;
pub const STC_SQL_SQLPLUS: c_int = 8;
pub const STC_SQL_SQLPLUS_PROMPT: c_int = 9;
pub const STC_SQL_OPERATOR: c_int = 10;
pub const STC_SQL_IDENTIFIER: c_int = 11;
pub const STC_SQL_SQLPLUS_COMMENT: c_int = 13;
pub const STC_SQL_COMMENTLINEDOC: c_int = 15;
pub const STC_SQL_WORD2: c_int = 16;
pub const STC_SQL_COMMENTDOCKEYWORD: c_int = 17;
pub const STC_SQL_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const STC_SQL_USER1: c_int = 19;
pub const STC_SQL_USER2: c_int = 20;
pub const STC_SQL_USER3: c_int = 21;
pub const STC_SQL_USER4: c_int = 22;
pub const STC_SQL_QUOTEDIDENTIFIER: c_int = 23;
pub const STC_SQL_QOPERATOR: c_int = 24;
pub const STC_ST_DEFAULT: c_int = 0;
pub const STC_ST_STRING: c_int = 1;
pub const STC_ST_NUMBER: c_int = 2;
pub const STC_ST_COMMENT: c_int = 3;
pub const STC_ST_SYMBOL: c_int = 4;
pub const STC_ST_BINARY: c_int = 5;
pub const STC_ST_BOOL: c_int = 6;
pub const STC_ST_SELF: c_int = 7;
pub const STC_ST_SUPER: c_int = 8;
pub const STC_ST_NIL: c_int = 9;
pub const STC_ST_GLOBAL: c_int = 10;
pub const STC_ST_RETURN: c_int = 11;
pub const STC_ST_SPECIAL: c_int = 12;
pub const STC_ST_KWSEND: c_int = 13;
pub const STC_ST_ASSIGN: c_int = 14;
pub const STC_ST_CHARACTER: c_int = 15;
pub const STC_ST_SPEC_SEL: c_int = 16;
pub const STC_FS_DEFAULT: c_int = 0;
pub const STC_FS_COMMENT: c_int = 1;
pub const STC_FS_COMMENTLINE: c_int = 2;
pub const STC_FS_COMMENTDOC: c_int = 3;
pub const STC_FS_COMMENTLINEDOC: c_int = 4;
pub const STC_FS_COMMENTDOCKEYWORD: c_int = 5;
pub const STC_FS_COMMENTDOCKEYWORDERROR: c_int = 6;
pub const STC_FS_KEYWORD: c_int = 7;
pub const STC_FS_KEYWORD2: c_int = 8;
pub const STC_FS_KEYWORD3: c_int = 9;
pub const STC_FS_KEYWORD4: c_int = 10;
pub const STC_FS_NUMBER: c_int = 11;
pub const STC_FS_STRING: c_int = 12;
pub const STC_FS_PREPROCESSOR: c_int = 13;
pub const STC_FS_OPERATOR: c_int = 14;
pub const STC_FS_IDENTIFIER: c_int = 15;
pub const STC_FS_DATE: c_int = 16;
pub const STC_FS_STRINGEOL: c_int = 17;
pub const STC_FS_CONSTANT: c_int = 18;
pub const STC_FS_WORDOPERATOR: c_int = 19;
pub const STC_FS_DISABLEDCODE: c_int = 20;
pub const STC_FS_DEFAULT_C: c_int = 21;
pub const STC_FS_COMMENTDOC_C: c_int = 22;
pub const STC_FS_COMMENTLINEDOC_C: c_int = 23;
pub const STC_FS_KEYWORD_C: c_int = 24;
pub const STC_FS_KEYWORD2_C: c_int = 25;
pub const STC_FS_NUMBER_C: c_int = 26;
pub const STC_FS_STRING_C: c_int = 27;
pub const STC_FS_PREPROCESSOR_C: c_int = 28;
pub const STC_FS_OPERATOR_C: c_int = 29;
pub const STC_FS_IDENTIFIER_C: c_int = 30;
pub const STC_FS_STRINGEOL_C: c_int = 31;
pub const STC_CSOUND_DEFAULT: c_int = 0;
pub const STC_CSOUND_COMMENT: c_int = 1;
pub const STC_CSOUND_NUMBER: c_int = 2;
pub const STC_CSOUND_OPERATOR: c_int = 3;
pub const STC_CSOUND_INSTR: c_int = 4;
pub const STC_CSOUND_IDENTIFIER: c_int = 5;
pub const STC_CSOUND_OPCODE: c_int = 6;
pub const STC_CSOUND_HEADERSTMT: c_int = 7;
pub const STC_CSOUND_USERKEYWORD: c_int = 8;
pub const STC_CSOUND_COMMENTBLOCK: c_int = 9;
pub const STC_CSOUND_PARAM: c_int = 10;
pub const STC_CSOUND_ARATE_VAR: c_int = 11;
pub const STC_CSOUND_KRATE_VAR: c_int = 12;
pub const STC_CSOUND_IRATE_VAR: c_int = 13;
pub const STC_CSOUND_GLOBAL_VAR: c_int = 14;
pub const STC_CSOUND_STRINGEOL: c_int = 15;
pub const STC_INNO_DEFAULT: c_int = 0;
pub const STC_INNO_COMMENT: c_int = 1;
pub const STC_INNO_KEYWORD: c_int = 2;
pub const STC_INNO_PARAMETER: c_int = 3;
pub const STC_INNO_SECTION: c_int = 4;
pub const STC_INNO_PREPROC: c_int = 5;
pub const STC_INNO_INLINE_EXPANSION: c_int = 6;
pub const STC_INNO_COMMENT_PASCAL: c_int = 7;
pub const STC_INNO_KEYWORD_PASCAL: c_int = 8;
pub const STC_INNO_KEYWORD_USER: c_int = 9;
pub const STC_INNO_STRING_DOUBLE: c_int = 10;
pub const STC_INNO_STRING_SINGLE: c_int = 11;
pub const STC_INNO_IDENTIFIER: c_int = 12;
pub const STC_OPAL_SPACE: c_int = 0;
pub const STC_OPAL_COMMENT_BLOCK: c_int = 1;
pub const STC_OPAL_COMMENT_LINE: c_int = 2;
pub const STC_OPAL_INTEGER: c_int = 3;
pub const STC_OPAL_KEYWORD: c_int = 4;
pub const STC_OPAL_SORT: c_int = 5;
pub const STC_OPAL_STRING: c_int = 6;
pub const STC_OPAL_PAR: c_int = 7;
pub const STC_OPAL_BOOL_CONST: c_int = 8;
pub const STC_OPAL_DEFAULT: c_int = 32;
pub const STC_SPICE_DEFAULT: c_int = 0;
pub const STC_SPICE_IDENTIFIER: c_int = 1;
pub const STC_SPICE_KEYWORD: c_int = 2;
pub const STC_SPICE_KEYWORD2: c_int = 3;
pub const STC_SPICE_KEYWORD3: c_int = 4;
pub const STC_SPICE_NUMBER: c_int = 5;
pub const STC_SPICE_DELIMITER: c_int = 6;
pub const STC_SPICE_VALUE: c_int = 7;
pub const STC_SPICE_COMMENTLINE: c_int = 8;
pub const STC_CMAKE_DEFAULT: c_int = 0;
pub const STC_CMAKE_COMMENT: c_int = 1;
pub const STC_CMAKE_STRINGDQ: c_int = 2;
pub const STC_CMAKE_STRINGLQ: c_int = 3;
pub const STC_CMAKE_STRINGRQ: c_int = 4;
pub const STC_CMAKE_COMMANDS: c_int = 5;
pub const STC_CMAKE_PARAMETERS: c_int = 6;
pub const STC_CMAKE_VARIABLE: c_int = 7;
pub const STC_CMAKE_USERDEFINED: c_int = 8;
pub const STC_CMAKE_WHILEDEF: c_int = 9;
pub const STC_CMAKE_FOREACHDEF: c_int = 10;
pub const STC_CMAKE_IFDEFINEDEF: c_int = 11;
pub const STC_CMAKE_MACRODEF: c_int = 12;
pub const STC_CMAKE_STRINGVAR: c_int = 13;
pub const STC_CMAKE_NUMBER: c_int = 14;
pub const STC_GAP_DEFAULT: c_int = 0;
pub const STC_GAP_IDENTIFIER: c_int = 1;
pub const STC_GAP_KEYWORD: c_int = 2;
pub const STC_GAP_KEYWORD2: c_int = 3;
pub const STC_GAP_KEYWORD3: c_int = 4;
pub const STC_GAP_KEYWORD4: c_int = 5;
pub const STC_GAP_STRING: c_int = 6;
pub const STC_GAP_CHAR: c_int = 7;
pub const STC_GAP_OPERATOR: c_int = 8;
pub const STC_GAP_COMMENT: c_int = 9;
pub const STC_GAP_NUMBER: c_int = 10;
pub const STC_GAP_STRINGEOL: c_int = 11;
pub const STC_PLM_DEFAULT: c_int = 0;
pub const STC_PLM_COMMENT: c_int = 1;
pub const STC_PLM_STRING: c_int = 2;
pub const STC_PLM_NUMBER: c_int = 3;
pub const STC_PLM_IDENTIFIER: c_int = 4;
pub const STC_PLM_OPERATOR: c_int = 5;
pub const STC_PLM_CONTROL: c_int = 6;
pub const STC_PLM_KEYWORD: c_int = 7;
pub const STC_ABL_DEFAULT: c_int = 0;
pub const STC_ABL_NUMBER: c_int = 1;
pub const STC_ABL_WORD: c_int = 2;
pub const STC_ABL_STRING: c_int = 3;
pub const STC_ABL_CHARACTER: c_int = 4;
pub const STC_ABL_PREPROCESSOR: c_int = 5;
pub const STC_ABL_OPERATOR: c_int = 6;
pub const STC_ABL_IDENTIFIER: c_int = 7;
pub const STC_ABL_BLOCK: c_int = 8;
pub const STC_ABL_END: c_int = 9;
pub const STC_ABL_COMMENT: c_int = 10;
pub const STC_ABL_TASKMARKER: c_int = 11;
pub const STC_ABL_LINECOMMENT: c_int = 12;
pub const STC_ABAQUS_DEFAULT: c_int = 0;
pub const STC_ABAQUS_COMMENT: c_int = 1;
pub const STC_ABAQUS_COMMENTBLOCK: c_int = 2;
pub const STC_ABAQUS_NUMBER: c_int = 3;
pub const STC_ABAQUS_STRING: c_int = 4;
pub const STC_ABAQUS_OPERATOR: c_int = 5;
pub const STC_ABAQUS_WORD: c_int = 6;
pub const STC_ABAQUS_PROCESSOR: c_int = 7;
pub const STC_ABAQUS_COMMAND: c_int = 8;
pub const STC_ABAQUS_SLASHCOMMAND: c_int = 9;
pub const STC_ABAQUS_STARCOMMAND: c_int = 10;
pub const STC_ABAQUS_ARGUMENT: c_int = 11;
pub const STC_ABAQUS_FUNCTION: c_int = 12;
pub const STC_ASY_DEFAULT: c_int = 0;
pub const STC_ASY_COMMENT: c_int = 1;
pub const STC_ASY_COMMENTLINE: c_int = 2;
pub const STC_ASY_NUMBER: c_int = 3;
pub const STC_ASY_WORD: c_int = 4;
pub const STC_ASY_STRING: c_int = 5;
pub const STC_ASY_CHARACTER: c_int = 6;
pub const STC_ASY_OPERATOR: c_int = 7;
pub const STC_ASY_IDENTIFIER: c_int = 8;
pub const STC_ASY_STRINGEOL: c_int = 9;
pub const STC_ASY_COMMENTLINEDOC: c_int = 10;
pub const STC_ASY_WORD2: c_int = 11;
pub const STC_R_DEFAULT: c_int = 0;
pub const STC_R_COMMENT: c_int = 1;
pub const STC_R_KWORD: c_int = 2;
pub const STC_R_BASEKWORD: c_int = 3;
pub const STC_R_OTHERKWORD: c_int = 4;
pub const STC_R_NUMBER: c_int = 5;
pub const STC_R_STRING: c_int = 6;
pub const STC_R_STRING2: c_int = 7;
pub const STC_R_OPERATOR: c_int = 8;
pub const STC_R_IDENTIFIER: c_int = 9;
pub const STC_R_INFIX: c_int = 10;
pub const STC_R_INFIXEOL: c_int = 11;
pub const STC_MAGIK_DEFAULT: c_int = 0;
pub const STC_MAGIK_COMMENT: c_int = 1;
pub const STC_MAGIK_HYPER_COMMENT: c_int = 16;
pub const STC_MAGIK_STRING: c_int = 2;
pub const STC_MAGIK_CHARACTER: c_int = 3;
pub const STC_MAGIK_NUMBER: c_int = 4;
pub const STC_MAGIK_IDENTIFIER: c_int = 5;
pub const STC_MAGIK_OPERATOR: c_int = 6;
pub const STC_MAGIK_FLOW: c_int = 7;
pub const STC_MAGIK_CONTAINER: c_int = 8;
pub const STC_MAGIK_BRACKET_BLOCK: c_int = 9;
pub const STC_MAGIK_BRACE_BLOCK: c_int = 10;
pub const STC_MAGIK_SQBRACKET_BLOCK: c_int = 11;
pub const STC_MAGIK_UNKNOWN_KEYWORD: c_int = 12;
pub const STC_MAGIK_KEYWORD: c_int = 13;
pub const STC_MAGIK_PRAGMA: c_int = 14;
pub const STC_MAGIK_SYMBOL: c_int = 15;
pub const STC_POWERSHELL_DEFAULT: c_int = 0;
pub const STC_POWERSHELL_COMMENT: c_int = 1;
pub const STC_POWERSHELL_STRING: c_int = 2;
pub const STC_POWERSHELL_CHARACTER: c_int = 3;
pub const STC_POWERSHELL_NUMBER: c_int = 4;
pub const STC_POWERSHELL_VARIABLE: c_int = 5;
pub const STC_POWERSHELL_OPERATOR: c_int = 6;
pub const STC_POWERSHELL_IDENTIFIER: c_int = 7;
pub const STC_POWERSHELL_KEYWORD: c_int = 8;
pub const STC_POWERSHELL_CMDLET: c_int = 9;
pub const STC_POWERSHELL_ALIAS: c_int = 10;
pub const STC_POWERSHELL_FUNCTION: c_int = 11;
pub const STC_POWERSHELL_USER1: c_int = 12;
pub const STC_POWERSHELL_COMMENTSTREAM: c_int = 13;
pub const STC_POWERSHELL_HERE_STRING: c_int = 14;
pub const STC_POWERSHELL_HERE_CHARACTER: c_int = 15;
pub const STC_POWERSHELL_COMMENTDOCKEYWORD: c_int = 16;
pub const STC_MYSQL_DEFAULT: c_int = 0;
pub const STC_MYSQL_COMMENT: c_int = 1;
pub const STC_MYSQL_COMMENTLINE: c_int = 2;
pub const STC_MYSQL_VARIABLE: c_int = 3;
pub const STC_MYSQL_SYSTEMVARIABLE: c_int = 4;
pub const STC_MYSQL_KNOWNSYSTEMVARIABLE: c_int = 5;
pub const STC_MYSQL_NUMBER: c_int = 6;
pub const STC_MYSQL_MAJORKEYWORD: c_int = 7;
pub const STC_MYSQL_KEYWORD: c_int = 8;
pub const STC_MYSQL_DATABASEOBJECT: c_int = 9;
pub const STC_MYSQL_PROCEDUREKEYWORD: c_int = 10;
pub const STC_MYSQL_STRING: c_int = 11;
pub const STC_MYSQL_SQSTRING: c_int = 12;
pub const STC_MYSQL_DQSTRING: c_int = 13;
pub const STC_MYSQL_OPERATOR: c_int = 14;
pub const STC_MYSQL_FUNCTION: c_int = 15;
pub const STC_MYSQL_IDENTIFIER: c_int = 16;
pub const STC_MYSQL_QUOTEDIDENTIFIER: c_int = 17;
pub const STC_MYSQL_USER1: c_int = 18;
pub const STC_MYSQL_USER2: c_int = 19;
pub const STC_MYSQL_USER3: c_int = 20;
pub const STC_MYSQL_HIDDENCOMMAND: c_int = 21;
pub const STC_MYSQL_PLACEHOLDER: c_int = 22;
pub const STC_PO_DEFAULT: c_int = 0;
pub const STC_PO_COMMENT: c_int = 1;
pub const STC_PO_MSGID: c_int = 2;
pub const STC_PO_MSGID_TEXT: c_int = 3;
pub const STC_PO_MSGSTR: c_int = 4;
pub const STC_PO_MSGSTR_TEXT: c_int = 5;
pub const STC_PO_MSGCTXT: c_int = 6;
pub const STC_PO_MSGCTXT_TEXT: c_int = 7;
pub const STC_PO_FUZZY: c_int = 8;
pub const STC_PO_PROGRAMMER_COMMENT: c_int = 9;
pub const STC_PO_REFERENCE: c_int = 10;
pub const STC_PO_FLAGS: c_int = 11;
pub const STC_PO_MSGID_TEXT_EOL: c_int = 12;
pub const STC_PO_MSGSTR_TEXT_EOL: c_int = 13;
pub const STC_PO_MSGCTXT_TEXT_EOL: c_int = 14;
pub const STC_PO_ERROR: c_int = 15;
pub const STC_PAS_DEFAULT: c_int = 0;
pub const STC_PAS_IDENTIFIER: c_int = 1;
pub const STC_PAS_COMMENT: c_int = 2;
pub const STC_PAS_COMMENT2: c_int = 3;
pub const STC_PAS_COMMENTLINE: c_int = 4;
pub const STC_PAS_PREPROCESSOR: c_int = 5;
pub const STC_PAS_PREPROCESSOR2: c_int = 6;
pub const STC_PAS_NUMBER: c_int = 7;
pub const STC_PAS_HEXNUMBER: c_int = 8;
pub const STC_PAS_WORD: c_int = 9;
pub const STC_PAS_STRING: c_int = 10;
pub const STC_PAS_STRINGEOL: c_int = 11;
pub const STC_PAS_CHARACTER: c_int = 12;
pub const STC_PAS_OPERATOR: c_int = 13;
pub const STC_PAS_ASM: c_int = 14;
pub const STC_SORCUS_DEFAULT: c_int = 0;
pub const STC_SORCUS_COMMAND: c_int = 1;
pub const STC_SORCUS_PARAMETER: c_int = 2;
pub const STC_SORCUS_COMMENTLINE: c_int = 3;
pub const STC_SORCUS_STRING: c_int = 4;
pub const STC_SORCUS_STRINGEOL: c_int = 5;
pub const STC_SORCUS_IDENTIFIER: c_int = 6;
pub const STC_SORCUS_OPERATOR: c_int = 7;
pub const STC_SORCUS_NUMBER: c_int = 8;
pub const STC_SORCUS_CONSTANT: c_int = 9;
pub const STC_POWERPRO_DEFAULT: c_int = 0;
pub const STC_POWERPRO_COMMENTBLOCK: c_int = 1;
pub const STC_POWERPRO_COMMENTLINE: c_int = 2;
pub const STC_POWERPRO_NUMBER: c_int = 3;
pub const STC_POWERPRO_WORD: c_int = 4;
pub const STC_POWERPRO_WORD2: c_int = 5;
pub const STC_POWERPRO_WORD3: c_int = 6;
pub const STC_POWERPRO_WORD4: c_int = 7;
pub const STC_POWERPRO_DOUBLEQUOTEDSTRING: c_int = 8;
pub const STC_POWERPRO_SINGLEQUOTEDSTRING: c_int = 9;
pub const STC_POWERPRO_LINECONTINUE: c_int = 10;
pub const STC_POWERPRO_OPERATOR: c_int = 11;
pub const STC_POWERPRO_IDENTIFIER: c_int = 12;
pub const STC_POWERPRO_STRINGEOL: c_int = 13;
pub const STC_POWERPRO_VERBATIM: c_int = 14;
pub const STC_POWERPRO_ALTQUOTE: c_int = 15;
pub const STC_POWERPRO_FUNCTION: c_int = 16;
pub const STC_SML_DEFAULT: c_int = 0;
pub const STC_SML_IDENTIFIER: c_int = 1;
pub const STC_SML_TAGNAME: c_int = 2;
pub const STC_SML_KEYWORD: c_int = 3;
pub const STC_SML_KEYWORD2: c_int = 4;
pub const STC_SML_KEYWORD3: c_int = 5;
pub const STC_SML_LINENUM: c_int = 6;
pub const STC_SML_OPERATOR: c_int = 7;
pub const STC_SML_NUMBER: c_int = 8;
pub const STC_SML_CHAR: c_int = 9;
pub const STC_SML_STRING: c_int = 11;
pub const STC_SML_COMMENT: c_int = 12;
pub const STC_SML_COMMENT1: c_int = 13;
pub const STC_SML_COMMENT2: c_int = 14;
pub const STC_SML_COMMENT3: c_int = 15;
pub const STC_MARKDOWN_DEFAULT: c_int = 0;
pub const STC_MARKDOWN_LINE_BEGIN: c_int = 1;
pub const STC_MARKDOWN_STRONG1: c_int = 2;
pub const STC_MARKDOWN_STRONG2: c_int = 3;
pub const STC_MARKDOWN_EM1: c_int = 4;
pub const STC_MARKDOWN_EM2: c_int = 5;
pub const STC_MARKDOWN_HEADER1: c_int = 6;
pub const STC_MARKDOWN_HEADER2: c_int = 7;
pub const STC_MARKDOWN_HEADER3: c_int = 8;
pub const STC_MARKDOWN_HEADER4: c_int = 9;
pub const STC_MARKDOWN_HEADER5: c_int = 10;
pub const STC_MARKDOWN_HEADER6: c_int = 11;
pub const STC_MARKDOWN_PRECHAR: c_int = 12;
pub const STC_MARKDOWN_ULIST_ITEM: c_int = 13;
pub const STC_MARKDOWN_OLIST_ITEM: c_int = 14;
pub const STC_MARKDOWN_BLOCKQUOTE: c_int = 15;
pub const STC_MARKDOWN_STRIKEOUT: c_int = 16;
pub const STC_MARKDOWN_HRULE: c_int = 17;
pub const STC_MARKDOWN_LINK: c_int = 18;
pub const STC_MARKDOWN_CODE: c_int = 19;
pub const STC_MARKDOWN_CODE2: c_int = 20;
pub const STC_MARKDOWN_CODEBK: c_int = 21;
pub const STC_TXT2TAGS_DEFAULT: c_int = 0;
pub const STC_TXT2TAGS_LINE_BEGIN: c_int = 1;
pub const STC_TXT2TAGS_STRONG1: c_int = 2;
pub const STC_TXT2TAGS_STRONG2: c_int = 3;
pub const STC_TXT2TAGS_EM1: c_int = 4;
pub const STC_TXT2TAGS_EM2: c_int = 5;
pub const STC_TXT2TAGS_HEADER1: c_int = 6;
pub const STC_TXT2TAGS_HEADER2: c_int = 7;
pub const STC_TXT2TAGS_HEADER3: c_int = 8;
pub const STC_TXT2TAGS_HEADER4: c_int = 9;
pub const STC_TXT2TAGS_HEADER5: c_int = 10;
pub const STC_TXT2TAGS_HEADER6: c_int = 11;
pub const STC_TXT2TAGS_PRECHAR: c_int = 12;
pub const STC_TXT2TAGS_ULIST_ITEM: c_int = 13;
pub const STC_TXT2TAGS_OLIST_ITEM: c_int = 14;
pub const STC_TXT2TAGS_BLOCKQUOTE: c_int = 15;
pub const STC_TXT2TAGS_STRIKEOUT: c_int = 16;
pub const STC_TXT2TAGS_HRULE: c_int = 17;
pub const STC_TXT2TAGS_LINK: c_int = 18;
pub const STC_TXT2TAGS_CODE: c_int = 19;
pub const STC_TXT2TAGS_CODE2: c_int = 20;
pub const STC_TXT2TAGS_CODEBK: c_int = 21;
pub const STC_TXT2TAGS_COMMENT: c_int = 22;
pub const STC_TXT2TAGS_OPTION: c_int = 23;
pub const STC_TXT2TAGS_PREPROC: c_int = 24;
pub const STC_TXT2TAGS_POSTPROC: c_int = 25;
pub const STC_A68K_DEFAULT: c_int = 0;
pub const STC_A68K_COMMENT: c_int = 1;
pub const STC_A68K_NUMBER_DEC: c_int = 2;
pub const STC_A68K_NUMBER_BIN: c_int = 3;
pub const STC_A68K_NUMBER_HEX: c_int = 4;
pub const STC_A68K_STRING1: c_int = 5;
pub const STC_A68K_OPERATOR: c_int = 6;
pub const STC_A68K_CPUINSTRUCTION: c_int = 7;
pub const STC_A68K_EXTINSTRUCTION: c_int = 8;
pub const STC_A68K_REGISTER: c_int = 9;
pub const STC_A68K_DIRECTIVE: c_int = 10;
pub const STC_A68K_MACRO_ARG: c_int = 11;
pub const STC_A68K_LABEL: c_int = 12;
pub const STC_A68K_STRING2: c_int = 13;
pub const STC_A68K_IDENTIFIER: c_int = 14;
pub const STC_A68K_MACRO_DECLARATION: c_int = 15;
pub const STC_A68K_COMMENT_WORD: c_int = 16;
pub const STC_A68K_COMMENT_SPECIAL: c_int = 17;
pub const STC_A68K_COMMENT_DOXYGEN: c_int = 18;
pub const STC_MODULA_DEFAULT: c_int = 0;
pub const STC_MODULA_COMMENT: c_int = 1;
pub const STC_MODULA_DOXYCOMM: c_int = 2;
pub const STC_MODULA_DOXYKEY: c_int = 3;
pub const STC_MODULA_KEYWORD: c_int = 4;
pub const STC_MODULA_RESERVED: c_int = 5;
pub const STC_MODULA_NUMBER: c_int = 6;
pub const STC_MODULA_BASENUM: c_int = 7;
pub const STC_MODULA_FLOAT: c_int = 8;
pub const STC_MODULA_STRING: c_int = 9;
pub const STC_MODULA_STRSPEC: c_int = 10;
pub const STC_MODULA_CHAR: c_int = 11;
pub const STC_MODULA_CHARSPEC: c_int = 12;
pub const STC_MODULA_PROC: c_int = 13;
pub const STC_MODULA_PRAGMA: c_int = 14;
pub const STC_MODULA_PRGKEY: c_int = 15;
pub const STC_MODULA_OPERATOR: c_int = 16;
pub const STC_MODULA_BADSTR: c_int = 17;
pub const STC_COFFEESCRIPT_DEFAULT: c_int = 0;
pub const STC_COFFEESCRIPT_COMMENT: c_int = 1;
pub const STC_COFFEESCRIPT_COMMENTLINE: c_int = 2;
pub const STC_COFFEESCRIPT_COMMENTDOC: c_int = 3;
pub const STC_COFFEESCRIPT_NUMBER: c_int = 4;
pub const STC_COFFEESCRIPT_WORD: c_int = 5;
pub const STC_COFFEESCRIPT_STRING: c_int = 6;
pub const STC_COFFEESCRIPT_CHARACTER: c_int = 7;
pub const STC_COFFEESCRIPT_UUID: c_int = 8;
pub const STC_COFFEESCRIPT_PREPROCESSOR: c_int = 9;
pub const STC_COFFEESCRIPT_OPERATOR: c_int = 10;
pub const STC_COFFEESCRIPT_IDENTIFIER: c_int = 11;
pub const STC_COFFEESCRIPT_STRINGEOL: c_int = 12;
pub const STC_COFFEESCRIPT_VERBATIM: c_int = 13;
pub const STC_COFFEESCRIPT_REGEX: c_int = 14;
pub const STC_COFFEESCRIPT_COMMENTLINEDOC: c_int = 15;
pub const STC_COFFEESCRIPT_WORD2: c_int = 16;
pub const STC_COFFEESCRIPT_COMMENTDOCKEYWORD: c_int = 17;
pub const STC_COFFEESCRIPT_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const STC_COFFEESCRIPT_GLOBALCLASS: c_int = 19;
pub const STC_COFFEESCRIPT_STRINGRAW: c_int = 20;
pub const STC_COFFEESCRIPT_TRIPLEVERBATIM: c_int = 21;
pub const STC_COFFEESCRIPT_COMMENTBLOCK: c_int = 22;
pub const STC_COFFEESCRIPT_VERBOSE_REGEX: c_int = 23;
pub const STC_COFFEESCRIPT_VERBOSE_REGEX_COMMENT: c_int = 24;
pub const STC_COFFEESCRIPT_INSTANCEPROPERTY: c_int = 25;
pub const STC_AVS_DEFAULT: c_int = 0;
pub const STC_AVS_COMMENTBLOCK: c_int = 1;
pub const STC_AVS_COMMENTBLOCKN: c_int = 2;
pub const STC_AVS_COMMENTLINE: c_int = 3;
pub const STC_AVS_NUMBER: c_int = 4;
pub const STC_AVS_OPERATOR: c_int = 5;
pub const STC_AVS_IDENTIFIER: c_int = 6;
pub const STC_AVS_STRING: c_int = 7;
pub const STC_AVS_TRIPLESTRING: c_int = 8;
pub const STC_AVS_KEYWORD: c_int = 9;
pub const STC_AVS_FILTER: c_int = 10;
pub const STC_AVS_PLUGIN: c_int = 11;
pub const STC_AVS_FUNCTION: c_int = 12;
pub const STC_AVS_CLIPPROP: c_int = 13;
pub const STC_AVS_USERDFN: c_int = 14;
pub const STC_ECL_DEFAULT: c_int = 0;
pub const STC_ECL_COMMENT: c_int = 1;
pub const STC_ECL_COMMENTLINE: c_int = 2;
pub const STC_ECL_NUMBER: c_int = 3;
pub const STC_ECL_STRING: c_int = 4;
pub const STC_ECL_WORD0: c_int = 5;
pub const STC_ECL_OPERATOR: c_int = 6;
pub const STC_ECL_CHARACTER: c_int = 7;
pub const STC_ECL_UUID: c_int = 8;
pub const STC_ECL_PREPROCESSOR: c_int = 9;
pub const STC_ECL_UNKNOWN: c_int = 10;
pub const STC_ECL_IDENTIFIER: c_int = 11;
pub const STC_ECL_STRINGEOL: c_int = 12;
pub const STC_ECL_VERBATIM: c_int = 13;
pub const STC_ECL_REGEX: c_int = 14;
pub const STC_ECL_COMMENTLINEDOC: c_int = 15;
pub const STC_ECL_WORD1: c_int = 16;
pub const STC_ECL_COMMENTDOCKEYWORD: c_int = 17;
pub const STC_ECL_COMMENTDOCKEYWORDERROR: c_int = 18;
pub const STC_ECL_WORD2: c_int = 19;
pub const STC_ECL_WORD3: c_int = 20;
pub const STC_ECL_WORD4: c_int = 21;
pub const STC_ECL_WORD5: c_int = 22;
pub const STC_ECL_COMMENTDOC: c_int = 23;
pub const STC_ECL_ADDED: c_int = 24;
pub const STC_ECL_DELETED: c_int = 25;
pub const STC_ECL_CHANGED: c_int = 26;
pub const STC_ECL_MOVED: c_int = 27;
pub const STC_OSCRIPT_DEFAULT: c_int = 0;
pub const STC_OSCRIPT_LINE_COMMENT: c_int = 1;
pub const STC_OSCRIPT_BLOCK_COMMENT: c_int = 2;
pub const STC_OSCRIPT_DOC_COMMENT: c_int = 3;
pub const STC_OSCRIPT_PREPROCESSOR: c_int = 4;
pub const STC_OSCRIPT_NUMBER: c_int = 5;
pub const STC_OSCRIPT_SINGLEQUOTE_STRING: c_int = 6;
pub const STC_OSCRIPT_DOUBLEQUOTE_STRING: c_int = 7;
pub const STC_OSCRIPT_CONSTANT: c_int = 8;
pub const STC_OSCRIPT_IDENTIFIER: c_int = 9;
pub const STC_OSCRIPT_GLOBAL: c_int = 10;
pub const STC_OSCRIPT_KEYWORD: c_int = 11;
pub const STC_OSCRIPT_OPERATOR: c_int = 12;
pub const STC_OSCRIPT_LABEL: c_int = 13;
pub const STC_OSCRIPT_TYPE: c_int = 14;
pub const STC_OSCRIPT_FUNCTION: c_int = 15;
pub const STC_OSCRIPT_OBJECT: c_int = 16;
pub const STC_OSCRIPT_PROPERTY: c_int = 17;
pub const STC_OSCRIPT_METHOD: c_int = 18;
pub const STC_VISUALPROLOG_DEFAULT: c_int = 0;
pub const STC_VISUALPROLOG_KEY_MAJOR: c_int = 1;
pub const STC_VISUALPROLOG_KEY_MINOR: c_int = 2;
pub const STC_VISUALPROLOG_KEY_DIRECTIVE: c_int = 3;
pub const STC_VISUALPROLOG_COMMENT_BLOCK: c_int = 4;
pub const STC_VISUALPROLOG_COMMENT_LINE: c_int = 5;
pub const STC_VISUALPROLOG_COMMENT_KEY: c_int = 6;
pub const STC_VISUALPROLOG_COMMENT_KEY_ERROR: c_int = 7;
pub const STC_VISUALPROLOG_IDENTIFIER: c_int = 8;
pub const STC_VISUALPROLOG_VARIABLE: c_int = 9;
pub const STC_VISUALPROLOG_ANONYMOUS: c_int = 10;
pub const STC_VISUALPROLOG_NUMBER: c_int = 11;
pub const STC_VISUALPROLOG_OPERATOR: c_int = 12;
pub const STC_VISUALPROLOG_CHARACTER: c_int = 13;
pub const STC_VISUALPROLOG_CHARACTER_TOO_MANY: c_int = 14;
pub const STC_VISUALPROLOG_CHARACTER_ESCAPE_ERROR: c_int = 15;
pub const STC_VISUALPROLOG_STRING: c_int = 16;
pub const STC_VISUALPROLOG_STRING_ESCAPE: c_int = 17;
pub const STC_VISUALPROLOG_STRING_ESCAPE_ERROR: c_int = 18;
pub const STC_VISUALPROLOG_STRING_EOL_OPEN: c_int = 19;
pub const STC_VISUALPROLOG_STRING_VERBATIM: c_int = 20;
pub const STC_VISUALPROLOG_STRING_VERBATIM_SPECIAL: c_int = 21;
pub const STC_VISUALPROLOG_STRING_VERBATIM_EOL: c_int = 22;
pub const STC_STTXT_DEFAULT: c_int = 0;
pub const STC_STTXT_COMMENT: c_int = 1;
pub const STC_STTXT_COMMENTLINE: c_int = 2;
pub const STC_STTXT_KEYWORD: c_int = 3;
pub const STC_STTXT_TYPE: c_int = 4;
pub const STC_STTXT_FUNCTION: c_int = 5;
pub const STC_STTXT_FB: c_int = 6;
pub const STC_STTXT_NUMBER: c_int = 7;
pub const STC_STTXT_HEXNUMBER: c_int = 8;
pub const STC_STTXT_PRAGMA: c_int = 9;
pub const STC_STTXT_OPERATOR: c_int = 10;
pub const STC_STTXT_CHARACTER: c_int = 11;
pub const STC_STTXT_STRING1: c_int = 12;
pub const STC_STTXT_STRING2: c_int = 13;
pub const STC_STTXT_STRINGEOL: c_int = 14;
pub const STC_STTXT_IDENTIFIER: c_int = 15;
pub const STC_STTXT_DATETIME: c_int = 16;
pub const STC_STTXT_VARS: c_int = 17;
pub const STC_STTXT_PRAGMAS: c_int = 18;
pub const STC_KVIRC_DEFAULT: c_int = 0;
pub const STC_KVIRC_COMMENT: c_int = 1;
pub const STC_KVIRC_COMMENTBLOCK: c_int = 2;
pub const STC_KVIRC_STRING: c_int = 3;
pub const STC_KVIRC_WORD: c_int = 4;
pub const STC_KVIRC_KEYWORD: c_int = 5;
pub const STC_KVIRC_FUNCTION_KEYWORD: c_int = 6;
pub const STC_KVIRC_FUNCTION: c_int = 7;
pub const STC_KVIRC_VARIABLE: c_int = 8;
pub const STC_KVIRC_NUMBER: c_int = 9;
pub const STC_KVIRC_OPERATOR: c_int = 10;
pub const STC_KVIRC_STRING_FUNCTION: c_int = 11;
pub const STC_KVIRC_STRING_VARIABLE: c_int = 12;
pub const STC_RUST_DEFAULT: c_int = 0;
pub const STC_RUST_COMMENTBLOCK: c_int = 1;
pub const STC_RUST_COMMENTLINE: c_int = 2;
pub const STC_RUST_COMMENTBLOCKDOC: c_int = 3;
pub const STC_RUST_COMMENTLINEDOC: c_int = 4;
pub const STC_RUST_NUMBER: c_int = 5;
pub const STC_RUST_WORD: c_int = 6;
pub const STC_RUST_WORD2: c_int = 7;
pub const STC_RUST_WORD3: c_int = 8;
pub const STC_RUST_WORD4: c_int = 9;
pub const STC_RUST_WORD5: c_int = 10;
pub const STC_RUST_WORD6: c_int = 11;
pub const STC_RUST_WORD7: c_int = 12;
pub const STC_RUST_STRING: c_int = 13;
pub const STC_RUST_STRINGR: c_int = 14;
pub const STC_RUST_CHARACTER: c_int = 15;
pub const STC_RUST_OPERATOR: c_int = 16;
pub const STC_RUST_IDENTIFIER: c_int = 17;
pub const STC_RUST_LIFETIME: c_int = 18;
pub const STC_RUST_MACRO: c_int = 19;
pub const STC_RUST_LEXERROR: c_int = 20;
pub const STC_RUST_BYTESTRING: c_int = 21;
pub const STC_RUST_BYTESTRINGR: c_int = 22;
pub const STC_RUST_BYTECHARACTER: c_int = 23;
pub const STC_DMAP_DEFAULT: c_int = 0;
pub const STC_DMAP_COMMENT: c_int = 1;
pub const STC_DMAP_NUMBER: c_int = 2;
pub const STC_DMAP_STRING1: c_int = 3;
pub const STC_DMAP_STRING2: c_int = 4;
pub const STC_DMAP_STRINGEOL: c_int = 5;
pub const STC_DMAP_OPERATOR: c_int = 6;
pub const STC_DMAP_IDENTIFIER: c_int = 7;
pub const STC_DMAP_WORD: c_int = 8;
pub const STC_DMAP_WORD2: c_int = 9;
pub const STC_DMAP_WORD3: c_int = 10;
pub const STC_DMIS_DEFAULT: c_int = 0;
pub const STC_DMIS_COMMENT: c_int = 1;
pub const STC_DMIS_STRING: c_int = 2;
pub const STC_DMIS_NUMBER: c_int = 3;
pub const STC_DMIS_KEYWORD: c_int = 4;
pub const STC_DMIS_MAJORWORD: c_int = 5;
pub const STC_DMIS_MINORWORD: c_int = 6;
pub const STC_DMIS_UNSUPPORTED_MAJOR: c_int = 7;
pub const STC_DMIS_UNSUPPORTED_MINOR: c_int = 8;
pub const STC_DMIS_LABEL: c_int = 9;
pub const STC_REG_DEFAULT: c_int = 0;
pub const STC_REG_COMMENT: c_int = 1;
pub const STC_REG_VALUENAME: c_int = 2;
pub const STC_REG_STRING: c_int = 3;
pub const STC_REG_HEXDIGIT: c_int = 4;
pub const STC_REG_VALUETYPE: c_int = 5;
pub const STC_REG_ADDEDKEY: c_int = 6;
pub const STC_REG_DELETEDKEY: c_int = 7;
pub const STC_REG_ESCAPED: c_int = 8;
pub const STC_REG_KEYPATH_GUID: c_int = 9;
pub const STC_REG_STRING_GUID: c_int = 10;
pub const STC_REG_PARAMETER: c_int = 11;
pub const STC_REG_OPERATOR: c_int = 12;
pub const STC_BIBTEX_DEFAULT: c_int = 0;
pub const STC_BIBTEX_ENTRY: c_int = 1;
pub const STC_BIBTEX_UNKNOWN_ENTRY: c_int = 2;
pub const STC_BIBTEX_KEY: c_int = 3;
pub const STC_BIBTEX_PARAMETER: c_int = 4;
pub const STC_BIBTEX_VALUE: c_int = 5;
pub const STC_BIBTEX_COMMENT: c_int = 6;
pub const STC_HEX_DEFAULT: c_int = 0;
pub const STC_HEX_RECSTART: c_int = 1;
pub const STC_HEX_RECTYPE: c_int = 2;
pub const STC_HEX_RECTYPE_UNKNOWN: c_int = 3;
pub const STC_HEX_BYTECOUNT: c_int = 4;
pub const STC_HEX_BYTECOUNT_WRONG: c_int = 5;
pub const STC_HEX_NOADDRESS: c_int = 6;
pub const STC_HEX_DATAADDRESS: c_int = 7;
pub const STC_HEX_RECCOUNT: c_int = 8;
pub const STC_HEX_STARTADDRESS: c_int = 9;
pub const STC_HEX_ADDRESSFIELD_UNKNOWN: c_int = 10;
pub const STC_HEX_EXTENDEDADDRESS: c_int = 11;
pub const STC_HEX_DATA_ODD: c_int = 12;
pub const STC_HEX_DATA_EVEN: c_int = 13;
pub const STC_HEX_DATA_UNKNOWN: c_int = 14;
pub const STC_HEX_DATA_EMPTY: c_int = 15;
pub const STC_HEX_CHECKSUM: c_int = 16;
pub const STC_HEX_CHECKSUM_WRONG: c_int = 17;
pub const STC_HEX_GARBAGE: c_int = 18;
pub const STC_JSON_DEFAULT: c_int = 0;
pub const STC_JSON_NUMBER: c_int = 1;
pub const STC_JSON_STRING: c_int = 2;
pub const STC_JSON_STRINGEOL: c_int = 3;
pub const STC_JSON_PROPERTYNAME: c_int = 4;
pub const STC_JSON_ESCAPESEQUENCE: c_int = 5;
pub const STC_JSON_LINECOMMENT: c_int = 6;
pub const STC_JSON_BLOCKCOMMENT: c_int = 7;
pub const STC_JSON_OPERATOR: c_int = 8;
pub const STC_JSON_URI: c_int = 9;
pub const STC_JSON_COMPACTIRI: c_int = 10;
pub const STC_JSON_KEYWORD: c_int = 11;
pub const STC_JSON_LDKEYWORD: c_int = 12;
pub const STC_JSON_ERROR: c_int = 13;
pub const STC_EDI_DEFAULT: c_int = 0;
pub const STC_EDI_SEGMENTSTART: c_int = 1;
pub const STC_EDI_SEGMENTEND: c_int = 2;
pub const STC_EDI_SEP_ELEMENT: c_int = 3;
pub const STC_EDI_SEP_COMPOSITE: c_int = 4;
pub const STC_EDI_SEP_RELEASE: c_int = 5;
pub const STC_EDI_UNA: c_int = 6;
pub const STC_EDI_UNH: c_int = 7;
pub const STC_EDI_BADSEGMENT: c_int = 8;
pub const STC_INDIC0_MASK: c_int = 0x20;
pub const STC_INDIC1_MASK: c_int = 0x40;
pub const STC_INDIC2_MASK: c_int = 0x80;
pub const STC_INDICS_MASK: c_int = 0xE0;
pub const STC_CMD_REDO: c_int = 2011;
pub const STC_CMD_SELECTALL: c_int = 2013;
pub const STC_CMD_UNDO: c_int = 2176;
pub const STC_CMD_CUT: c_int = 2177;
pub const STC_CMD_COPY: c_int = 2178;
pub const STC_CMD_PASTE: c_int = 2179;
pub const STC_CMD_CLEAR: c_int = 2180;
pub const STC_CMD_LINEDOWN: c_int = 2300;
pub const STC_CMD_LINEDOWNEXTEND: c_int = 2301;
pub const STC_CMD_LINEUP: c_int = 2302;
pub const STC_CMD_LINEUPEXTEND: c_int = 2303;
pub const STC_CMD_CHARLEFT: c_int = 2304;
pub const STC_CMD_CHARLEFTEXTEND: c_int = 2305;
pub const STC_CMD_CHARRIGHT: c_int = 2306;
pub const STC_CMD_CHARRIGHTEXTEND: c_int = 2307;
pub const STC_CMD_WORDLEFT: c_int = 2308;
pub const STC_CMD_WORDLEFTEXTEND: c_int = 2309;
pub const STC_CMD_WORDRIGHT: c_int = 2310;
pub const STC_CMD_WORDRIGHTEXTEND: c_int = 2311;
pub const STC_CMD_HOME: c_int = 2312;
pub const STC_CMD_HOMEEXTEND: c_int = 2313;
pub const STC_CMD_LINEEND: c_int = 2314;
pub const STC_CMD_LINEENDEXTEND: c_int = 2315;
pub const STC_CMD_DOCUMENTSTART: c_int = 2316;
pub const STC_CMD_DOCUMENTSTARTEXTEND: c_int = 2317;
pub const STC_CMD_DOCUMENTEND: c_int = 2318;
pub const STC_CMD_DOCUMENTENDEXTEND: c_int = 2319;
pub const STC_CMD_PAGEUP: c_int = 2320;
pub const STC_CMD_PAGEUPEXTEND: c_int = 2321;
pub const STC_CMD_PAGEDOWN: c_int = 2322;
pub const STC_CMD_PAGEDOWNEXTEND: c_int = 2323;
pub const STC_CMD_EDITTOGGLEOVERTYPE: c_int = 2324;
pub const STC_CMD_CANCEL: c_int = 2325;
pub const STC_CMD_DELETEBACK: c_int = 2326;
pub const STC_CMD_TAB: c_int = 2327;
pub const STC_CMD_BACKTAB: c_int = 2328;
pub const STC_CMD_NEWLINE: c_int = 2329;
pub const STC_CMD_FORMFEED: c_int = 2330;
pub const STC_CMD_VCHOME: c_int = 2331;
pub const STC_CMD_VCHOMEEXTEND: c_int = 2332;
pub const STC_CMD_ZOOMIN: c_int = 2333;
pub const STC_CMD_ZOOMOUT: c_int = 2334;
pub const STC_CMD_DELWORDLEFT: c_int = 2335;
pub const STC_CMD_DELWORDRIGHT: c_int = 2336;
pub const STC_CMD_DELWORDRIGHTEND: c_int = 2518;
pub const STC_CMD_LINECUT: c_int = 2337;
pub const STC_CMD_LINEDELETE: c_int = 2338;
pub const STC_CMD_LINETRANSPOSE: c_int = 2339;
pub const STC_CMD_LINEDUPLICATE: c_int = 2404;
pub const STC_CMD_LOWERCASE: c_int = 2340;
pub const STC_CMD_UPPERCASE: c_int = 2341;
pub const STC_CMD_LINESCROLLDOWN: c_int = 2342;
pub const STC_CMD_LINESCROLLUP: c_int = 2343;
pub const STC_CMD_DELETEBACKNOTLINE: c_int = 2344;
pub const STC_CMD_HOMEDISPLAY: c_int = 2345;
pub const STC_CMD_HOMEDISPLAYEXTEND: c_int = 2346;
pub const STC_CMD_LINEENDDISPLAY: c_int = 2347;
pub const STC_CMD_LINEENDDISPLAYEXTEND: c_int = 2348;
pub const STC_CMD_HOMEWRAP: c_int = 2349;
pub const STC_CMD_HOMEWRAPEXTEND: c_int = 2450;
pub const STC_CMD_LINEENDWRAP: c_int = 2451;
pub const STC_CMD_LINEENDWRAPEXTEND: c_int = 2452;
pub const STC_CMD_VCHOMEWRAP: c_int = 2453;
pub const STC_CMD_VCHOMEWRAPEXTEND: c_int = 2454;
pub const STC_CMD_LINECOPY: c_int = 2455;
pub const STC_CMD_WORDPARTLEFT: c_int = 2390;
pub const STC_CMD_WORDPARTLEFTEXTEND: c_int = 2391;
pub const STC_CMD_WORDPARTRIGHT: c_int = 2392;
pub const STC_CMD_WORDPARTRIGHTEXTEND: c_int = 2393;
pub const STC_CMD_DELLINELEFT: c_int = 2395;
pub const STC_CMD_DELLINERIGHT: c_int = 2396;
pub const STC_CMD_PARADOWN: c_int = 2413;
pub const STC_CMD_PARADOWNEXTEND: c_int = 2414;
pub const STC_CMD_PARAUP: c_int = 2415;
pub const STC_CMD_PARAUPEXTEND: c_int = 2416;
pub const STC_CMD_LINEDOWNRECTEXTEND: c_int = 2426;
pub const STC_CMD_LINEUPRECTEXTEND: c_int = 2427;
pub const STC_CMD_CHARLEFTRECTEXTEND: c_int = 2428;
pub const STC_CMD_CHARRIGHTRECTEXTEND: c_int = 2429;
pub const STC_CMD_HOMERECTEXTEND: c_int = 2430;
pub const STC_CMD_VCHOMERECTEXTEND: c_int = 2431;
pub const STC_CMD_LINEENDRECTEXTEND: c_int = 2432;
pub const STC_CMD_PAGEUPRECTEXTEND: c_int = 2433;
pub const STC_CMD_PAGEDOWNRECTEXTEND: c_int = 2434;
pub const STC_CMD_STUTTEREDPAGEUP: c_int = 2435;
pub const STC_CMD_STUTTEREDPAGEUPEXTEND: c_int = 2436;
pub const STC_CMD_STUTTEREDPAGEDOWN: c_int = 2437;
pub const STC_CMD_STUTTEREDPAGEDOWNEXTEND: c_int = 2438;
pub const STC_CMD_WORDLEFTEND: c_int = 2439;
pub const STC_CMD_WORDLEFTENDEXTEND: c_int = 2440;
pub const STC_CMD_WORDRIGHTEND: c_int = 2441;
pub const STC_CMD_WORDRIGHTENDEXTEND: c_int = 2442;
pub const STC_CMD_VERTICALCENTRECARET: c_int = 2619;
pub const STC_CMD_MOVESELECTEDLINESUP: c_int = 2620;
pub const STC_CMD_MOVESELECTEDLINESDOWN: c_int = 2621;
pub const STC_CMD_SCROLLTOSTART: c_int = 2628;
pub const STC_CMD_SCROLLTOEND: c_int = 2629;
pub const STC_CMD_VCHOMEDISPLAY: c_int = 2652;
pub const STC_CMD_VCHOMEDISPLAYEXTEND: c_int = 2653;

//  ENUM: wxStockLabelQueryFlag
pub const STOCK_NOFLAGS: c_int = 0;
pub const STOCK_WITH_MNEMONIC: c_int = 1;
pub const STOCK_WITH_ACCELERATOR: c_int = 2;
pub const STOCK_WITHOUT_ELLIPSIS: c_int = 4;
pub const STOCK_FOR_BUTTON: c_int = STOCK_WITHOUT_ELLIPSIS | STOCK_WITH_MNEMONIC;

//  ENUM: wxTarType
pub const TAR_REGTYPE: char = '0';
pub const TAR_LNKTYPE: char = '1';
pub const TAR_SYMTYPE: char = '2';
pub const TAR_CHRTYPE: char = '3';
pub const TAR_BLKTYPE: char = '4';
pub const TAR_DIRTYPE: char = '5';
pub const TAR_FIFOTYPE: char = '6';
pub const TAR_CONTTYPE: char = '7';
//  ENUM: wxTarFormat
pub const TAR_USTAR: c_int = 0;
pub const TAR_PAX: c_int = 0 + 1;

//  ENUM: wxTaskBarIconType
pub const TBI_DOCK: c_int = 0;
pub const TBI_CUSTOM_STATUSITEM: c_int = 0 + 1;
pub const TBI_DEFAULT_TYPE: c_int = 0 + 2;

//  ENUM: wxTaskBarButtonState
pub const TASKBAR_BUTTON_NO_PROGRESS: c_int = 0;
pub const TASKBAR_BUTTON_INDETERMINATE: c_int = 1;
pub const TASKBAR_BUTTON_NORMAL: c_int = 2;
pub const TASKBAR_BUTTON_ERROR: c_int = 4;
pub const TASKBAR_BUTTON_PAUSED: c_int = 8;
//  ENUM: wxTaskBarJumpListItemType
pub const TASKBAR_JUMP_LIST_SEPARATOR: c_int = 0;
pub const TASKBAR_JUMP_LIST_TASK: c_int = 0 + 1;
pub const TASKBAR_JUMP_LIST_DESTINATION: c_int = 0 + 2;

pub const TE_NO_VSCROLL: c_int = 0x0002;
pub const TE_READONLY: c_int = 0x0010;
pub const TE_MULTILINE: c_int = 0x0020;
pub const TE_PROCESS_TAB: c_int = 0x0040;
pub const TE_LEFT: c_int = 0x0000;
pub const TE_CENTER: c_int = ALIGN_CENTER_HORIZONTAL;
pub const TE_RIGHT: c_int = ALIGN_RIGHT;
pub const TE_CENTRE: c_int = TE_CENTER;
pub const TE_RICH: c_int = 0x0080;
pub const TE_PROCESS_ENTER: c_int = 0x0400;
pub const TE_PASSWORD: c_int = 0x0800;
pub const TE_AUTO_URL: c_int = 0x1000;
pub const TE_NOHIDESEL: c_int = 0x2000;
pub const TE_DONTWRAP: c_long = HSCROLL;
pub const TE_CHARWRAP: c_int = 0x4000;
pub const TE_WORDWRAP: c_int = 0x0001;
pub const TE_BESTWRAP: c_int = 0x0000;
pub const TE_RICH2: c_int = 0x8000;
pub const TEXT_TYPE_ANY: c_int = 0;
//  ENUM: wxTextAttrAlignment
pub const TEXT_ALIGNMENT_DEFAULT: c_int = 0;
pub const TEXT_ALIGNMENT_LEFT: c_int = 0 + 1;
pub const TEXT_ALIGNMENT_CENTRE: c_int = 0 + 2;
pub const TEXT_ALIGNMENT_CENTER: c_int = TEXT_ALIGNMENT_CENTRE;
pub const TEXT_ALIGNMENT_RIGHT: c_int = TEXT_ALIGNMENT_CENTRE + 1;
pub const TEXT_ALIGNMENT_JUSTIFIED: c_int = TEXT_ALIGNMENT_CENTRE + 2;
//  ENUM: wxTextAttrFlags
pub const TEXT_ATTR_TEXT_COLOUR: c_int = 0x00000001;
pub const TEXT_ATTR_BACKGROUND_COLOUR: c_int = 0x00000002;
pub const TEXT_ATTR_FONT_FACE: c_int = 0x00000004;
pub const TEXT_ATTR_FONT_POINT_SIZE: c_int = 0x00000008;
pub const TEXT_ATTR_FONT_PIXEL_SIZE: c_int = 0x10000000;
pub const TEXT_ATTR_FONT_WEIGHT: c_int = 0x00000010;
pub const TEXT_ATTR_FONT_ITALIC: c_int = 0x00000020;
pub const TEXT_ATTR_FONT_UNDERLINE: c_int = 0x00000040;
pub const TEXT_ATTR_FONT_STRIKETHROUGH: c_int = 0x08000000;
pub const TEXT_ATTR_FONT_ENCODING: c_int = 0x02000000;
pub const TEXT_ATTR_FONT_FAMILY: c_int = 0x04000000;
pub const TEXT_ATTR_FONT_SIZE: c_int = 
        ( TEXT_ATTR_FONT_POINT_SIZE | TEXT_ATTR_FONT_PIXEL_SIZE );
pub const TEXT_ATTR_FONT: c_int = 
        ( TEXT_ATTR_FONT_FACE | TEXT_ATTR_FONT_SIZE | TEXT_ATTR_FONT_WEIGHT | 
            TEXT_ATTR_FONT_ITALIC | TEXT_ATTR_FONT_UNDERLINE | TEXT_ATTR_FONT_STRIKETHROUGH | TEXT_ATTR_FONT_ENCODING | TEXT_ATTR_FONT_FAMILY );
pub const TEXT_ATTR_ALIGNMENT: c_int = 0x00000080;
pub const TEXT_ATTR_LEFT_INDENT: c_int = 0x00000100;
pub const TEXT_ATTR_RIGHT_INDENT: c_int = 0x00000200;
pub const TEXT_ATTR_TABS: c_int = 0x00000400;
pub const TEXT_ATTR_PARA_SPACING_AFTER: c_int = 0x00000800;
pub const TEXT_ATTR_PARA_SPACING_BEFORE: c_int = 0x00001000;
pub const TEXT_ATTR_LINE_SPACING: c_int = 0x00002000;
pub const TEXT_ATTR_CHARACTER_STYLE_NAME: c_int = 0x00004000;
pub const TEXT_ATTR_PARAGRAPH_STYLE_NAME: c_int = 0x00008000;
pub const TEXT_ATTR_LIST_STYLE_NAME: c_int = 0x00010000;
pub const TEXT_ATTR_BULLET_STYLE: c_int = 0x00020000;
pub const TEXT_ATTR_BULLET_NUMBER: c_int = 0x00040000;
pub const TEXT_ATTR_BULLET_TEXT: c_int = 0x00080000;
pub const TEXT_ATTR_BULLET_NAME: c_int = 0x00100000;
pub const TEXT_ATTR_BULLET: c_int = 
        ( TEXT_ATTR_BULLET_STYLE | TEXT_ATTR_BULLET_NUMBER | TEXT_ATTR_BULLET_TEXT | 
          TEXT_ATTR_BULLET_NAME );
pub const TEXT_ATTR_URL: c_int = 0x00200000;
pub const TEXT_ATTR_PAGE_BREAK: c_int = 0x00400000;
pub const TEXT_ATTR_EFFECTS: c_int = 0x00800000;
pub const TEXT_ATTR_OUTLINE_LEVEL: c_int = 0x01000000;
pub const TEXT_ATTR_AVOID_PAGE_BREAK_BEFORE: c_int = 0x20000000;
pub const TEXT_ATTR_AVOID_PAGE_BREAK_AFTER: c_int =  0x40000000;
pub const TEXT_ATTR_CHARACTER: c_int = 
        (TEXT_ATTR_FONT|TEXT_ATTR_EFFECTS| 
            TEXT_ATTR_BACKGROUND_COLOUR|TEXT_ATTR_TEXT_COLOUR|TEXT_ATTR_CHARACTER_STYLE_NAME|TEXT_ATTR_URL);
pub const TEXT_ATTR_PARAGRAPH: c_int = 
        (TEXT_ATTR_ALIGNMENT|TEXT_ATTR_LEFT_INDENT|TEXT_ATTR_RIGHT_INDENT|TEXT_ATTR_TABS|
            TEXT_ATTR_PARA_SPACING_BEFORE|TEXT_ATTR_PARA_SPACING_AFTER|TEXT_ATTR_LINE_SPACING|
            TEXT_ATTR_BULLET|TEXT_ATTR_PARAGRAPH_STYLE_NAME|TEXT_ATTR_LIST_STYLE_NAME|TEXT_ATTR_OUTLINE_LEVEL|
            TEXT_ATTR_PAGE_BREAK|TEXT_ATTR_AVOID_PAGE_BREAK_BEFORE|TEXT_ATTR_AVOID_PAGE_BREAK_AFTER);
pub const TEXT_ATTR_ALL: c_int = (TEXT_ATTR_CHARACTER|TEXT_ATTR_PARAGRAPH);
//  ENUM: wxTextAttrBulletStyle
pub const TEXT_ATTR_BULLET_STYLE_NONE: c_int = 0x00000000;
pub const TEXT_ATTR_BULLET_STYLE_ARABIC: c_int = 0x00000001;
pub const TEXT_ATTR_BULLET_STYLE_LETTERS_UPPER: c_int = 0x00000002;
pub const TEXT_ATTR_BULLET_STYLE_LETTERS_LOWER: c_int = 0x00000004;
pub const TEXT_ATTR_BULLET_STYLE_ROMAN_UPPER: c_int = 0x00000008;
pub const TEXT_ATTR_BULLET_STYLE_ROMAN_LOWER: c_int = 0x00000010;
pub const TEXT_ATTR_BULLET_STYLE_SYMBOL: c_int = 0x00000020;
pub const TEXT_ATTR_BULLET_STYLE_BITMAP: c_int = 0x00000040;
pub const TEXT_ATTR_BULLET_STYLE_PARENTHESES: c_int = 0x00000080;
pub const TEXT_ATTR_BULLET_STYLE_PERIOD: c_int = 0x00000100;
pub const TEXT_ATTR_BULLET_STYLE_STANDARD: c_int = 0x00000200;
pub const TEXT_ATTR_BULLET_STYLE_RIGHT_PARENTHESIS: c_int = 0x00000400;
pub const TEXT_ATTR_BULLET_STYLE_OUTLINE: c_int = 0x00000800;
pub const TEXT_ATTR_BULLET_STYLE_ALIGN_LEFT: c_int = 0x00000000;
pub const TEXT_ATTR_BULLET_STYLE_ALIGN_RIGHT: c_int = 0x00001000;
pub const TEXT_ATTR_BULLET_STYLE_ALIGN_CENTRE: c_int = 0x00002000;
pub const TEXT_ATTR_BULLET_STYLE_CONTINUATION: c_int = 0x00004000;
//  ENUM: wxTextAttrEffects
pub const TEXT_ATTR_EFFECT_NONE: c_int = 0x00000000;
pub const TEXT_ATTR_EFFECT_CAPITALS: c_int = 0x00000001;
pub const TEXT_ATTR_EFFECT_SMALL_CAPITALS: c_int = 0x00000002;
pub const TEXT_ATTR_EFFECT_STRIKETHROUGH: c_int = 0x00000004;
pub const TEXT_ATTR_EFFECT_DOUBLE_STRIKETHROUGH: c_int = 0x00000008;
pub const TEXT_ATTR_EFFECT_SHADOW: c_int = 0x00000010;
pub const TEXT_ATTR_EFFECT_EMBOSS: c_int = 0x00000020;
pub const TEXT_ATTR_EFFECT_OUTLINE: c_int = 0x00000040;
pub const TEXT_ATTR_EFFECT_ENGRAVE: c_int = 0x00000080;
pub const TEXT_ATTR_EFFECT_SUPERSCRIPT: c_int = 0x00000100;
pub const TEXT_ATTR_EFFECT_SUBSCRIPT: c_int = 0x00000200;
pub const TEXT_ATTR_EFFECT_RTL: c_int = 0x00000400;
pub const TEXT_ATTR_EFFECT_SUPPRESS_HYPHENATION: c_int = 0x00001000;
//  ENUM: wxTextAttrLineSpacing
pub const TEXT_ATTR_LINE_SPACING_NORMAL: c_int = 10;
pub const TEXT_ATTR_LINE_SPACING_HALF: c_int = 15;
pub const TEXT_ATTR_LINE_SPACING_TWICE: c_int = 20;
//  ENUM: wxTextAttrUnderlineType
pub const TEXT_ATTR_UNDERLINE_NONE: c_int = 0;
pub const TEXT_ATTR_UNDERLINE_SOLID: c_int = 0 + 1;
pub const TEXT_ATTR_UNDERLINE_DOUBLE: c_int = 0 + 2;
pub const TEXT_ATTR_UNDERLINE_SPECIAL: c_int = 0 + 3;
//  ENUM: wxTextCtrlHitTestResult
pub const TE_HT_UNKNOWN: c_int = -2;
pub const TE_HT_BEFORE: c_int = -2 + 1;
pub const TE_HT_ON_TEXT: c_int = -2 + 2;
pub const TE_HT_BELOW: c_int = -2 + 3;
pub const TE_HT_BEYOND: c_int = -2 + 4;

pub const TextEntryDialogStyle: c_long = (OK | CANCEL | CENTRE);

//  ENUM: wxTextFileType
pub const TextFileType_None: c_int = 0;
pub const TextFileType_Unix: c_int = 0 + 1;
pub const TextFileType_Dos: c_int = 0 + 2;
pub const TextFileType_Mac: c_int = 0 + 3;
pub const TextFileType_Os2: c_int = 0 + 4;

//  ENUM: @48
pub const TP_DEFAULT: c_int = 0;

pub const TIMER_CONTINUOUS: bool = false;
pub const TIMER_ONE_SHOT: bool = true;

//  SKIP: wxTLS_TYPE
// NODEF: wxTLS_VALUE
// NODEF: wxTLS_PTR

pub const DEFAULT_DELIMITERS: &str = " \t\r\n";
//  ENUM: wxStringTokenizerMode
pub const TOKEN_INVALID: c_int = -1;
pub const TOKEN_DEFAULT: c_int = -1 + 1;
pub const TOKEN_RET_EMPTY: c_int = -1 + 2;
pub const TOKEN_RET_EMPTY_ALL: c_int = -1 + 3;
pub const TOKEN_RET_DELIMS: c_int = -1 + 4;
pub const TOKEN_STRTOK: c_int = -1 + 5;

pub const TBK_BUTTONBAR: c_int = 0x0100;
pub const TBK_HORZ_LAYOUT: c_int = 0x8000;

pub const DEFAULT_FRAME_STYLE: c_long = (SYSTEM_MENU |          RESIZE_BORDER |        MINIMIZE_BOX |         MAXIMIZE_BOX |         CLOSE_BOX |            CAPTION |              CLIP_CHILDREN);
//  ENUM: @50
pub const USER_ATTENTION_INFO: c_int = 1;
pub const USER_ATTENTION_ERROR: c_int = 2;
//  ENUM: @51
pub const FULLSCREEN_NOMENUBAR: c_int = 0x0001;
pub const FULLSCREEN_NOTOOLBAR: c_int = 0x0002;
pub const FULLSCREEN_NOSTATUSBAR: c_int = 0x0004;
pub const FULLSCREEN_NOBORDER: c_int = 0x0008;
pub const FULLSCREEN_NOCAPTION: c_int = 0x0010;
pub const FULLSCREEN_ALL: c_int = FULLSCREEN_NOMENUBAR | FULLSCREEN_NOTOOLBAR |
                          FULLSCREEN_NOSTATUSBAR | FULLSCREEN_NOBORDER |
                          FULLSCREEN_NOCAPTION;

// NODEF: wxPLURAL
// NODEF: wxGETTEXT_IN_CONTEXT
// NODEF: wxGETTEXT_IN_CONTEXT_PLURAL
// NODEF: wxTRANSLATE

pub const TR_NO_BUTTONS: c_int = 0x0000;
pub const TR_HAS_BUTTONS: c_int = 0x0001;
pub const TR_NO_LINES: c_int = 0x0004;
pub const TR_LINES_AT_ROOT: c_int = 0x0008;
pub const TR_TWIST_BUTTONS: c_int = 0x0010;
pub const TR_SINGLE: c_int = 0x0000;
pub const TR_MULTIPLE: c_int = 0x0020;
pub const TR_HAS_VARIABLE_ROW_HEIGHT: c_int = 0x0080;
pub const TR_EDIT_LABELS: c_int = 0x0200;
pub const TR_ROW_LINES: c_int = 0x0400;
pub const TR_HIDE_ROOT: c_int = 0x0800;
pub const TR_FULL_ROW_HIGHLIGHT: c_int = 0x2000;
pub const TR_DEFAULT_STYLE: c_int = (TR_HAS_BUTTONS | TR_LINES_AT_ROOT);
//  ENUM: wxTreeItemIcon
pub const TreeItemIcon_Normal: c_int = 0;
pub const TreeItemIcon_Selected: c_int = 0 + 1;
pub const TreeItemIcon_Expanded: c_int = 0 + 2;
pub const TreeItemIcon_SelectedExpanded: c_int = 0 + 3;
pub const TreeItemIcon_Max: c_int = 0 + 4;

//  SKIP: wxTreeListEventHandler
//  ENUM: @52
pub const TL_SINGLE: c_int = 0x0000;
pub const TL_MULTIPLE: c_int = 0x0001;
pub const TL_CHECKBOX: c_int = 0x0002;
pub const TL_3STATE: c_int = 0x0004;
pub const TL_USER_3STATE: c_int = 0x0008;
pub const TL_NO_HEADER: c_int = 0x0010;
pub const TL_DEFAULT_STYLE: c_int = TL_SINGLE;
pub const TL_STYLE_MASK: c_int = TL_SINGLE |
                          TL_MULTIPLE |
                          TL_CHECKBOX |
                          TL_3STATE |
                          TL_USER_3STATE;

//  ENUM: wxEOL
pub const EOL_NATIVE: c_int = 0;
pub const EOL_UNIX: c_int = 0 + 1;
pub const EOL_MAC: c_int = 0 + 2;
pub const EOL_DOS: c_int = 0 + 3;

//  ENUM: wxURIHostType
pub const URI_REGNAME: c_int = 0;
pub const URI_IPV4ADDRESS: c_int = 0 + 1;
pub const URI_IPV6ADDRESS: c_int = 0 + 2;
pub const URI_IPVFUTURE: c_int = 0 + 3;

//  ENUM: wxURLError
pub const URL_NOERR: c_int = 0;
pub const URL_SNTXERR: c_int = 0 + 1;
pub const URL_NOPROTO: c_int = 0 + 2;
pub const URL_NOHOST: c_int = 0 + 3;
pub const URL_NOPATH: c_int = 0 + 4;
pub const URL_CONNERR: c_int = 0 + 5;
pub const URL_PROTOERR: c_int = 0 + 6;

//  ENUM: wxSignal
pub const SIGNONE: c_int = 0;
pub const SIGHUP: c_int = 0 + 1;
pub const SIGINT: c_int = 0 + 2;
pub const SIGQUIT: c_int = 0 + 3;
pub const SIGILL: c_int = 0 + 4;
pub const SIGTRAP: c_int = 0 + 5;
pub const SIGABRT: c_int = 0 + 6;
pub const SIGEMT: c_int = 0 + 7;
pub const SIGFPE: c_int = 0 + 8;
pub const SIGKILL: c_int = 0 + 9;
pub const SIGBUS: c_int = 0 + 10;
pub const SIGSEGV: c_int = 0 + 11;
pub const SIGSYS: c_int = 0 + 12;
pub const SIGPIPE: c_int = 0 + 13;
pub const SIGALRM: c_int = 0 + 14;
pub const SIGTERM: c_int = 0 + 15;
//  ENUM: wxKillError
pub const KILL_OK: c_int = 0;
pub const KILL_BAD_SIGNAL: c_int = 0 + 1;
pub const KILL_ACCESS_DENIED: c_int = 0 + 2;
pub const KILL_NO_PROCESS: c_int = 0 + 3;
pub const KILL_ERROR: c_int = 0 + 4;
//  ENUM: wxKillFlags
pub const KILL_NOCHILDREN: c_int = 0;
pub const KILL_CHILDREN: c_int = 1;
//  ENUM: wxShutdownFlags
pub const SHUTDOWN_FORCE: c_int = 1;
pub const SHUTDOWN_POWEROFF: c_int = 2;
pub const SHUTDOWN_REBOOT: c_int = 4;
pub const SHUTDOWN_LOGOFF: c_int = 8;
//  ENUM: @53
pub const Strip_Mnemonics: c_int = 1;
pub const Strip_Accel: c_int = 2;
pub const Strip_CJKMnemonics: c_int = 4;
pub const Strip_All: c_int = Strip_Mnemonics | Strip_Accel;
pub const Strip_Menu: c_int = Strip_All | Strip_CJKMnemonics;
//  ENUM: @54
pub const EXEC_ASYNC: c_int = 0;
pub const EXEC_SYNC: c_int = 1;
pub const EXEC_SHOW_CONSOLE: c_int = 2;
pub const EXEC_MAKE_GROUP_LEADER: c_int = 4;
pub const EXEC_NODISABLE: c_int = 8;
pub const EXEC_NOEVENTS: c_int = 16;
pub const EXEC_HIDE_CONSOLE: c_int = 32;
pub const EXEC_BLOCK: c_int = EXEC_SYNC | EXEC_NOEVENTS;

//  ENUM: wxNumValidatorStyle
pub const NUM_VAL_DEFAULT: c_int = 0;
pub const NUM_VAL_THOUSANDS_SEPARATOR: c_int = 1;
pub const NUM_VAL_ZERO_AS_BLANK: c_int = 2;
pub const NUM_VAL_NO_TRAILING_ZEROES: c_int = 2 + 1;

//  ENUM: wxTextValidatorStyle
pub const FILTER_NONE: c_int = 0;
pub const FILTER_EMPTY: c_int = 0 + 1;
pub const FILTER_ASCII: c_int = 0 + 2;
pub const FILTER_ALPHA: c_int = 0 + 3;
pub const FILTER_ALPHANUMERIC: c_int = 0 + 4;
pub const FILTER_DIGITS: c_int = 0 + 5;
pub const FILTER_NUMERIC: c_int = 0 + 6;
pub const FILTER_INCLUDE_LIST: c_int = 0 + 7;
pub const FILTER_INCLUDE_CHAR_LIST: c_int = 0 + 8;
pub const FILTER_EXCLUDE_LIST: c_int = 0 + 9;
pub const FILTER_EXCLUDE_CHAR_LIST: c_int = 0 + 10;
pub const FILTER_XDIGITS: c_int = 0 + 11;
pub const FILTER_SPACE: c_int = 0 + 12;

// NODEF: wxGetVariantCast

// NODEF: wxCHECK_VERSION
// NODEF: wxCHECK_VERSION_FULL

//  ENUM: wxFSVolumeFlags
pub const FS_VOL_MOUNTED: c_int = 0x0001;
pub const FS_VOL_REMOVABLE: c_int = 0x0002;
pub const FS_VOL_READONLY: c_int = 0x0004;
pub const FS_VOL_REMOTE: c_int = 0x0008;
//  ENUM: wxFSVolumeKind
pub const FS_VOL_FLOPPY: c_int = 0;
pub const FS_VOL_DISK: c_int = 0 + 1;
pub const FS_VOL_CDROM: c_int = 0 + 2;
pub const FS_VOL_DVDROM: c_int = 0 + 3;
pub const FS_VOL_NETWORK: c_int = 0 + 4;
pub const FS_VOL_OTHER: c_int = 0 + 5;
pub const FS_VOL_MAX: c_int = 0 + 6;
//  ENUM: wxFSIconType
pub const FS_VOL_ICO_SMALL: c_int = 0;
pub const FS_VOL_ICO_LARGE: c_int = 0 + 1;
pub const FS_VOL_ICO_SEL_SMALL: c_int = 0 + 2;
pub const FS_VOL_ICO_SEL_LARGE: c_int = 0 + 3;
pub const FS_VOL_ICO_MAX: c_int = 0 + 4;

//  ENUM: wxWebViewZoom
pub const WEBVIEW_ZOOM_TINY: c_int = 0;
pub const WEBVIEW_ZOOM_SMALL: c_int = 0 + 1;
pub const WEBVIEW_ZOOM_MEDIUM: c_int = 0 + 2;
pub const WEBVIEW_ZOOM_LARGE: c_int = 0 + 3;
pub const WEBVIEW_ZOOM_LARGEST: c_int = 0 + 4;
//  ENUM: wxWebViewZoomType
pub const WEBVIEW_ZOOM_TYPE_LAYOUT: c_int = 0;
pub const WEBVIEW_ZOOM_TYPE_TEXT: c_int = 0 + 1;
//  ENUM: wxWebViewNavigationError
pub const WEBVIEW_NAV_ERR_CONNECTION: c_int = 0;
pub const WEBVIEW_NAV_ERR_CERTIFICATE: c_int = 0 + 1;
pub const WEBVIEW_NAV_ERR_AUTH: c_int = 0 + 2;
pub const WEBVIEW_NAV_ERR_SECURITY: c_int = 0 + 3;
pub const WEBVIEW_NAV_ERR_NOT_FOUND: c_int = 0 + 4;
pub const WEBVIEW_NAV_ERR_REQUEST: c_int = 0 + 5;
pub const WEBVIEW_NAV_ERR_USER_CANCELLED: c_int = 0 + 6;
pub const WEBVIEW_NAV_ERR_OTHER: c_int = 0 + 7;
//  ENUM: wxWebViewReloadFlags
pub const WEBVIEW_RELOAD_DEFAULT: c_int = 0;
pub const WEBVIEW_RELOAD_NO_CACHE: c_int = 0 + 1;
//  ENUM: wxWebViewFindFlags
pub const WEBVIEW_FIND_WRAP: c_int =             0x0001;
pub const WEBVIEW_FIND_ENTIRE_WORD: c_int =      0x0002;
pub const WEBVIEW_FIND_MATCH_CASE: c_int =       0x0004;
pub const WEBVIEW_FIND_HIGHLIGHT_RESULT: c_int = 0x0008;
pub const WEBVIEW_FIND_BACKWARDS: c_int =        0x0010;
pub const WEBVIEW_FIND_DEFAULT: c_int =          0;
//  ENUM: wxWebViewNavigationActionFlags
pub const WEBVIEW_NAV_ACTION_NONE: c_int = 0;
pub const WEBVIEW_NAV_ACTION_USER: c_int = 0 + 1;
pub const WEBVIEW_NAV_ACTION_OTHER: c_int = 0 + 2;
//  ENUM: wxWebViewUserScriptInjectionTime
pub const WEBVIEW_INJECT_AT_DOCUMENT_START: c_int = 0;
pub const WEBVIEW_INJECT_AT_DOCUMENT_END: c_int = 0 + 1;
//  ENUM: wxWebViewIE_EmulationLevel
pub const WEBVIEWIE_EMU_DEFAULT: c_int =    0;
pub const WEBVIEWIE_EMU_IE7: c_int =        7000;
pub const WEBVIEWIE_EMU_IE8: c_int =        8000;
pub const WEBVIEWIE_EMU_IE8_FORCE: c_int =  8888;
pub const WEBVIEWIE_EMU_IE9: c_int =        9000;
pub const WEBVIEWIE_EMU_IE9_FORCE: c_int =  9999;
pub const WEBVIEWIE_EMU_IE10: c_int =       10000;
pub const WEBVIEWIE_EMU_IE10_FORCE: c_int = 10001;
pub const WEBVIEWIE_EMU_IE11: c_int =       11000;
pub const WEBVIEWIE_EMU_IE11_FORCE: c_int = 11001;

//  ENUM: wxShowEffect
pub const SHOW_EFFECT_NONE: c_int = 0;
pub const SHOW_EFFECT_ROLL_TO_LEFT: c_int = 0 + 1;
pub const SHOW_EFFECT_ROLL_TO_RIGHT: c_int = 0 + 2;
pub const SHOW_EFFECT_ROLL_TO_TOP: c_int = 0 + 3;
pub const SHOW_EFFECT_ROLL_TO_BOTTOM: c_int = 0 + 4;
pub const SHOW_EFFECT_SLIDE_TO_LEFT: c_int = 0 + 5;
pub const SHOW_EFFECT_SLIDE_TO_RIGHT: c_int = 0 + 6;
pub const SHOW_EFFECT_SLIDE_TO_TOP: c_int = 0 + 7;
pub const SHOW_EFFECT_SLIDE_TO_BOTTOM: c_int = 0 + 8;
pub const SHOW_EFFECT_BLEND: c_int = 0 + 9;
pub const SHOW_EFFECT_EXPAND: c_int = 0 + 10;
pub const SHOW_EFFECT_MAX: c_int = 0 + 11;
//  ENUM: @55
pub const TOUCH_NONE: c_int = 0;
pub const TOUCH_VERTICAL_PAN_GESTURE: c_int = 0 + 1;
pub const TOUCH_HORIZONTAL_PAN_GESTURE: c_int = 0 + 2;
pub const TOUCH_PAN_GESTURES: c_int = 0 + 3;
pub const TOUCH_ZOOM_GESTURE: c_int = 0 + 4;
pub const TOUCH_ROTATE_GESTURE: c_int = 0 + 5;
pub const TOUCH_PRESS_GESTURES: c_int = 0 + 6;
pub const TOUCH_ALL_GESTURES: c_int = 0 + 7;
//  ENUM: @56
pub const SEND_EVENT_POST: c_int = 1;
//  ENUM: wxWindowVariant
pub const WINDOW_VARIANT_NORMAL: c_int = 0;
pub const WINDOW_VARIANT_SMALL: c_int = 0 + 1;
pub const WINDOW_VARIANT_MINI: c_int = 0 + 2;
pub const WINDOW_VARIANT_LARGE: c_int = 0 + 3;
pub const WINDOW_VARIANT_MAX: c_int = 0 + 4;

pub const WIZARD_EX_HELPBUTTON: c_int = 0x00000010;
pub const WIZARD_VALIGN_TOP: c_int = 0x01;
pub const WIZARD_VALIGN_CENTRE: c_int = 0x02;
pub const WIZARD_VALIGN_BOTTOM: c_int = 0x04;
pub const WIZARD_HALIGN_LEFT: c_int = 0x08;
pub const WIZARD_HALIGN_CENTRE: c_int = 0x10;
pub const WIZARD_HALIGN_RIGHT: c_int = 0x20;
pub const WIZARD_TILE: c_int = 0x40;

//  ENUM: @58
pub const EXTEND_LAST_ON_EACH_LINE: c_int = 0;
pub const REMOVE_LEADING_SPACES: c_int = 0 + 1;
pub const WRAPSIZER_DEFAULT_FLAGS: c_int = 0 + 2;

pub const XML_NO_INDENTATION: c_int = (-1);
//  ENUM: wxXmlNodeType
pub const XML_ELEMENT_NODE: c_int =  1;
pub const XML_ATTRIBUTE_NODE: c_int =  2;
pub const XML_TEXT_NODE: c_int =  3;
pub const XML_CDATA_SECTION_NODE: c_int =  4;
pub const XML_ENTITY_REF_NODE: c_int =  5;
pub const XML_ENTITY_NODE: c_int =  6;
pub const XML_PI_NODE: c_int =  7;
pub const XML_COMMENT_NODE: c_int =  8;
pub const XML_DOCUMENT_NODE: c_int =  9;
pub const XML_DOCUMENT_TYPE_NODE: c_int = 10;
pub const XML_DOCUMENT_FRAG_NODE: c_int = 11;
pub const XML_NOTATION_NODE: c_int = 12;
pub const XML_HTML_DOCUMENT_NODE: c_int = 13;
//  ENUM: wxXmlDocumentLoadFlag
pub const XMLDOC_NONE: c_int = 0;
pub const XMLDOC_KEEP_WHITESPACE_NODES: c_int = 0 + 1;

//  ENUM: wxXmlResourceFlags
pub const XRC_USE_LOCALE: c_int = 1;
pub const XRC_NO_SUBCLASSING: c_int = 2;
pub const XRC_NO_RELOADING: c_int = 4;
pub const XRC_USE_ENVVARS: c_int = 8;

//  ENUM: wxZipMethod
pub const ZIP_METHOD_STORE: c_int = 0;
pub const ZIP_METHOD_SHRINK: c_int = 0 + 1;
pub const ZIP_METHOD_REDUCE1: c_int = 0 + 2;
pub const ZIP_METHOD_REDUCE2: c_int = 0 + 3;
pub const ZIP_METHOD_REDUCE3: c_int = 0 + 4;
pub const ZIP_METHOD_REDUCE4: c_int = 0 + 5;
pub const ZIP_METHOD_IMPLODE: c_int = 0 + 6;
pub const ZIP_METHOD_TOKENIZE: c_int = 0 + 7;
pub const ZIP_METHOD_DEFLATE: c_int = 0 + 8;
pub const ZIP_METHOD_DEFLATE64: c_int = 0 + 9;
pub const ZIP_METHOD_BZIP2: c_int = 12;
pub const ZIP_METHOD_DEFAULT: c_int = 0xffff;
//  ENUM: wxZipSystem
pub const ZIP_SYSTEM_MSDOS: c_int = 0;
pub const ZIP_SYSTEM_AMIGA: c_int = 0 + 1;
pub const ZIP_SYSTEM_OPENVMS: c_int = 0 + 2;
pub const ZIP_SYSTEM_UNIX: c_int = 0 + 3;
pub const ZIP_SYSTEM_VM_CMS: c_int = 0 + 4;
pub const ZIP_SYSTEM_ATARI_ST: c_int = 0 + 5;
pub const ZIP_SYSTEM_OS2_HPFS: c_int = 0 + 6;
pub const ZIP_SYSTEM_MACINTOSH: c_int = 0 + 7;
pub const ZIP_SYSTEM_Z_SYSTEM: c_int = 0 + 8;
pub const ZIP_SYSTEM_CPM: c_int = 0 + 9;
pub const ZIP_SYSTEM_WINDOWS_NTFS: c_int = 0 + 10;
pub const ZIP_SYSTEM_MVS: c_int = 0 + 11;
pub const ZIP_SYSTEM_VSE: c_int = 0 + 12;
pub const ZIP_SYSTEM_ACORN_RISC: c_int = 0 + 13;
pub const ZIP_SYSTEM_VFAT: c_int = 0 + 14;
pub const ZIP_SYSTEM_ALTERNATE_MVS: c_int = 0 + 15;
pub const ZIP_SYSTEM_BEOS: c_int = 0 + 16;
pub const ZIP_SYSTEM_TANDEM: c_int = 0 + 17;
pub const ZIP_SYSTEM_OS_400: c_int = 0 + 18;
//  ENUM: wxZipAttributes
pub const ZIP_A_RDONLY: c_int = 0x01;
pub const ZIP_A_HIDDEN: c_int = 0x02;
pub const ZIP_A_SYSTEM: c_int = 0x04;
pub const ZIP_A_SUBDIR: c_int = 0x10;
pub const ZIP_A_ARCH: c_int = 0x20;
pub const ZIP_A_MASK: c_int = 0x37;
//  ENUM: wxZipFlags
pub const ZIP_ENCRYPTED: c_int = 0x0001;
pub const ZIP_DEFLATE_NORMAL: c_int = 0x0000;
pub const ZIP_DEFLATE_EXTRA: c_int = 0x0002;
pub const ZIP_DEFLATE_FAST: c_int = 0x0004;
pub const ZIP_DEFLATE_SUPERFAST: c_int = 0x0006;
pub const ZIP_DEFLATE_MASK: c_int = 0x0006;
pub const ZIP_SUMS_FOLLOW: c_int = 0x0008;
pub const ZIP_ENHANCED: c_int = 0x0010;
pub const ZIP_PATCH: c_int = 0x0020;
pub const ZIP_STRONG_ENC: c_int = 0x0040;
pub const ZIP_UNUSED: c_int = 0x0F80;
pub const ZIP_RESERVED: c_int = 0xF000;
//  ENUM: wxZipArchiveFormat
pub const ZIP_FORMAT_DEFAULT: c_int = 0;
pub const ZIP_FORMAT_ZIP64: c_int = 0 + 1;

//  ENUM: wxZlibCompressionLevels
pub const Z_DEFAULT_COMPRESSION: c_int = -1;
pub const Z_NO_COMPRESSION: c_int = 0;
pub const Z_BEST_SPEED: c_int = 1;
pub const Z_BEST_COMPRESSION: c_int = 9;
//  ENUM: wxZLibFlags
pub const ZLIB_NO_HEADER: c_int = 0;
pub const ZLIB_ZLIB: c_int = 1;
pub const ZLIB_GZIP: c_int = 2;
pub const ZLIB_AUTO: c_int = 3;

//  ENUM: @48

//  ENUM: wxMessageQueueError

//  ENUM: wxNumValidatorStyle

//  ENUM: wxAuiToolBarStyle
//  ENUM: wxAuiToolBarArtSetting
//  ENUM: wxAuiToolBarToolTextOrientation
//  ENUM: wxAuiPaneDockArtSetting

