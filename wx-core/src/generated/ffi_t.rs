use super::*;

extern "C" {

    // wxTGAHandler
    pub fn wxTGAHandler_CLASSINFO() -> *mut c_void;
    pub fn wxTGAHandler_new() -> *mut c_void;

    // wxTIFFHandler
    pub fn wxTIFFHandler_CLASSINFO() -> *mut c_void;
    pub fn wxTIFFHandler_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTIFFHandler_GetLibraryVersionInfo() -> wxVersionInfo;

    // wxTaskBarIcon
    pub fn wxTaskBarIcon_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTaskBarIcon_new(icon_type: wxTaskBarIconType) -> *mut c_void;
    // DTOR: pub fn wxTaskBarIcon_~wxTaskBarIcon(self_: *mut c_void);
    pub fn wxTaskBarIcon_Destroy(self_: *mut c_void);
    pub fn wxTaskBarIcon_IsIconInstalled(self_: *const c_void) -> bool;
    pub fn wxTaskBarIcon_IsOk(self_: *const c_void) -> bool;
    pub fn wxTaskBarIcon_PopupMenu(self_: *mut c_void, menu: *mut c_void) -> bool;
    pub fn wxTaskBarIcon_RemoveIcon(self_: *mut c_void) -> bool;
    pub fn wxTaskBarIcon_SetIcon(
        self_: *mut c_void,
        icon: *const c_void,
        tooltip: *const c_void,
    ) -> bool;
    pub fn wxTaskBarIcon_IsAvailable() -> bool;

    // wxTaskBarIconEvent
    pub fn wxTaskBarIconEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTaskBarIconEvent_new(evt_type: wxEventType, tb_icon: *mut c_void) -> *mut c_void;

    // wxTextAttr
    pub fn wxTextAttr_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxTextAttr_GetAlignment(self_: *const c_void) -> wxTextAttrAlignment;
    pub fn wxTextAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetBulletFont(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetBulletName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetBulletNumber(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetBulletStyle(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetBulletText(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetCharacterStyleName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetFlags(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetFontAttributes(
        self_: *mut c_void,
        font: *const c_void,
        flags: c_int,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontEncoding(self_: *const c_void) -> wxFontEncoding;
    pub fn wxTextAttr_GetFontFaceName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontFamily(self_: *const c_void) -> wxFontFamily;
    pub fn wxTextAttr_GetFontSize(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontStyle(self_: *const c_void) -> wxFontStyle;
    pub fn wxTextAttr_GetFontUnderlined(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetUnderlineType(self_: *const c_void) -> wxTextAttrUnderlineType;
    pub fn wxTextAttr_GetUnderlineColour(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextAttr_GetFontWeight(self_: *const c_void) -> wxFontWeight;
    pub fn wxTextAttr_GetLeftIndent(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetLeftSubIndent(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetLineSpacing(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetListStyleName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetOutlineLevel(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetParagraphSpacingAfter(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetParagraphSpacingBefore(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetParagraphStyleName(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetRightIndent(self_: *const c_void) -> c_long;
    pub fn wxTextAttr_GetTabs(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_GetTextEffectFlags(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetTextEffects(self_: *const c_void) -> c_int;
    pub fn wxTextAttr_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_HasAlignment(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletNumber(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletStyle(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasBulletText(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasCharacterStyleName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFlag(self_: *const c_void, flag: c_long) -> bool;
    pub fn wxTextAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontEncoding(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontFaceName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontFamily(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontItalic(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontSize(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontPointSize(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontPixelSize(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontUnderlined(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasFontWeight(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasLeftIndent(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasLineSpacing(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasListStyleName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasOutlineLevel(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasPageBreak(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasParagraphSpacingAfter(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasParagraphSpacingBefore(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasParagraphStyleName(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasRightIndent(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasTabs(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasTextColour(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasTextEffects(self_: *const c_void) -> bool;
    pub fn wxTextAttr_HasURL(self_: *const c_void) -> bool;
    pub fn wxTextAttr_IsCharacterStyle(self_: *const c_void) -> bool;
    pub fn wxTextAttr_IsDefault(self_: *const c_void) -> bool;
    pub fn wxTextAttr_IsParagraphStyle(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextAttr_SetAlignment(self_: *mut c_void, alignment: wxTextAttrAlignment);
    pub fn wxTextAttr_SetBackgroundColour(self_: *mut c_void, col_back: *const c_void);
    pub fn wxTextAttr_SetBulletFont(self_: *mut c_void, font: *const c_void);
    pub fn wxTextAttr_SetBulletName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetBulletNumber(self_: *mut c_void, n: c_int);
    pub fn wxTextAttr_SetBulletStyle(self_: *mut c_void, style: c_int);
    pub fn wxTextAttr_SetBulletText(self_: *mut c_void, text: *const c_void);
    pub fn wxTextAttr_SetCharacterStyleName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetFlags(self_: *mut c_void, flags: c_long);
    pub fn wxTextAttr_SetFont(self_: *mut c_void, font: *const c_void, flags: c_int);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontEncoding(self_: *mut c_void, encoding: wxFontEncoding);
    pub fn wxTextAttr_SetFontFaceName(self_: *mut c_void, face_name: *const c_void);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontFamily(self_: *mut c_void, family: wxFontFamily);
    pub fn wxTextAttr_SetFontSize(self_: *mut c_void, point_size: c_int);
    pub fn wxTextAttr_SetFontPointSize(self_: *mut c_void, point_size: c_int);
    pub fn wxTextAttr_SetFontPixelSize(self_: *mut c_void, pixel_size: c_int);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontStyle(self_: *mut c_void, font_style: wxFontStyle);
    pub fn wxTextAttr_SetFontUnderlined(self_: *mut c_void, underlined: bool);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontUnderlined1(self_: *mut c_void, type_: wxTextAttrUnderlineType, colour: *const c_void);
    // NOT_SUPPORTED: pub fn wxTextAttr_SetFontWeight(self_: *mut c_void, font_weight: wxFontWeight);
    pub fn wxTextAttr_SetLeftIndent(self_: *mut c_void, indent: c_int, sub_indent: c_int);
    pub fn wxTextAttr_SetLineSpacing(self_: *mut c_void, spacing: c_int);
    pub fn wxTextAttr_SetListStyleName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetOutlineLevel(self_: *mut c_void, level: c_int);
    pub fn wxTextAttr_SetPageBreak(self_: *mut c_void, page_break: bool);
    pub fn wxTextAttr_SetParagraphSpacingAfter(self_: *mut c_void, spacing: c_int);
    pub fn wxTextAttr_SetParagraphSpacingBefore(self_: *mut c_void, spacing: c_int);
    pub fn wxTextAttr_SetParagraphStyleName(self_: *mut c_void, name: *const c_void);
    pub fn wxTextAttr_SetRightIndent(self_: *mut c_void, indent: c_int);
    pub fn wxTextAttr_SetTabs(self_: *mut c_void, tabs: *const c_void);
    pub fn wxTextAttr_SetTextColour(self_: *mut c_void, col_text: *const c_void);
    pub fn wxTextAttr_SetTextEffectFlags(self_: *mut c_void, flags: c_int);
    pub fn wxTextAttr_SetTextEffects(self_: *mut c_void, effects: c_int);
    pub fn wxTextAttr_SetURL(self_: *mut c_void, url: *const c_void);
    // BLOCKED: pub fn wxTextAttr_operator=(self_: *mut c_void, attr: *const c_void);
    pub fn wxTextAttr_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextAttr_new1(col_text: *const c_void, col_back: *const c_void, font: *const c_void, alignment: wxTextAttrAlignment) -> *mut c_void;
    pub fn wxTextAttr_new2(attr: *const c_void) -> *mut c_void;
    pub fn wxTextAttr_Apply(
        self_: *mut c_void,
        style: *const c_void,
        compare_with: *const c_void,
    ) -> bool;
    pub fn wxTextAttr_Merge(self_: *mut c_void, overlay: *const c_void);
    pub fn wxTextAttr_EqPartial(self_: *const c_void, attr: *const c_void, weak_test: bool)
        -> bool;
    pub fn wxTextAttr_Merge1(base: *const c_void, overlay: *const c_void) -> *mut c_void;

    // wxTextCtrl
    pub fn wxTextCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTextCtrl_OSXEnableNewLineReplacement(self_: *mut c_void, enable: bool);
    // BLOCKED: pub fn wxTextCtrl_operator<<(self_: *mut c_void, s: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxTextCtrl_operator<<1(self_: *mut c_void, i: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxTextCtrl_operator<<2(self_: *mut c_void, i: c_long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextCtrl_operator<<3(self_: *mut c_void, f: float) -> *mut c_void;
    // BLOCKED: pub fn wxTextCtrl_operator<<4(self_: *mut c_void, d: c_double) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextCtrl_operator<<5(self_: *mut c_void, c: char) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTextCtrl_operator<<6(self_: *mut c_void, c: wchar_t) -> *mut c_void;
    pub fn wxTextCtrl_new() -> *mut c_void;
    pub fn wxTextCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTextCtrl_~wxTextCtrl(self_: *mut c_void);
    pub fn wxTextCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxTextCtrl_DiscardEdits(self_: *mut c_void);
    pub fn wxTextCtrl_EmptyUndoBuffer(self_: *mut c_void);
    pub fn wxTextCtrl_EmulateKeyPress(self_: *mut c_void, event: *const c_void) -> bool;
    pub fn wxTextCtrl_EnableProofCheck(self_: *mut c_void, options: *const c_void) -> bool;
    pub fn wxTextCtrl_GetDefaultStyle(self_: *const c_void) -> *mut c_void;
    pub fn wxTextCtrl_GetLineLength(self_: *const c_void, line_no: c_long) -> c_int;
    pub fn wxTextCtrl_GetLineText(self_: *const c_void, line_no: c_long) -> *mut c_void;
    pub fn wxTextCtrl_GetNumberOfLines(self_: *const c_void) -> c_int;
    pub fn wxTextCtrl_GetStyle(self_: *mut c_void, position: c_long, style: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextCtrl_HitTest(self_: *const c_void, pt: *const c_void, pos: *mut c_void) -> wxTextCtrlHitTestResult;
    // NOT_SUPPORTED: pub fn wxTextCtrl_HitTest1(self_: *const c_void, pt: *const c_void, col: *mut c_void, row: *mut c_void) -> wxTextCtrlHitTestResult;
    pub fn wxTextCtrl_IsModified(self_: *const c_void) -> bool;
    pub fn wxTextCtrl_IsMultiLine(self_: *const c_void) -> bool;
    pub fn wxTextCtrl_IsSingleLine(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTextCtrl_GetProofCheckOptions(self_: *mut c_void) -> wxTextProofOptions;
    pub fn wxTextCtrl_LoadFile(
        self_: *mut c_void,
        filename: *const c_void,
        file_type: c_int,
    ) -> bool;
    pub fn wxTextCtrl_MarkDirty(self_: *mut c_void);
    pub fn wxTextCtrl_OnDropFiles(self_: *mut c_void, event: *mut c_void);
    pub fn wxTextCtrl_PositionToXY(
        self_: *const c_void,
        pos: c_long,
        x: *mut c_void,
        y: *mut c_void,
    ) -> bool;
    pub fn wxTextCtrl_PositionToCoords(self_: *const c_void, pos: c_long) -> *mut c_void;
    pub fn wxTextCtrl_SaveFile(
        self_: *mut c_void,
        filename: *const c_void,
        file_type: c_int,
    ) -> bool;
    pub fn wxTextCtrl_SetDefaultStyle(self_: *mut c_void, style: *const c_void) -> bool;
    pub fn wxTextCtrl_SetModified(self_: *mut c_void, modified: bool);
    pub fn wxTextCtrl_SetStyle(
        self_: *mut c_void,
        start: c_long,
        end: c_long,
        style: *const c_void,
    ) -> bool;
    pub fn wxTextCtrl_ShowPosition(self_: *mut c_void, pos: c_long);
    pub fn wxTextCtrl_XYToPosition(self_: *const c_void, x: c_long, y: c_long) -> c_long;
    // Mix-in(s) to wxTextCtrl
    pub fn wxTextCtrl_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxTextDataObject
    pub fn wxTextDataObject_delete(self_: *mut c_void);
    pub fn wxTextDataObject_new(text: *const c_void) -> *mut c_void;
    pub fn wxTextDataObject_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxTextDataObject_GetTextLength(self_: *const c_void) -> usize;
    // BLOCKED: pub fn wxTextDataObject_GetFormat(self_: *const c_void) -> *mut c_void;
    pub fn wxTextDataObject_SetText(self_: *mut c_void, str_text: *const c_void);

    // wxTextDropTarget
    pub fn wxTextDropTarget_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxTextDropTarget_new() -> *mut c_void;
    pub fn wxTextDropTarget_OnDropText(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        data: *const c_void,
    ) -> bool;

    // wxTextEntry
    pub fn wxTextEntry_delete(self_: *mut c_void);
    pub fn wxTextEntry_AppendText(self_: *mut c_void, text: *const c_void);
    pub fn wxTextEntry_AutoComplete(self_: *mut c_void, choices: *const c_void) -> bool;
    pub fn wxTextEntry_AutoComplete1(self_: *mut c_void, completer: *mut c_void) -> bool;
    pub fn wxTextEntry_AutoCompleteFileNames(self_: *mut c_void) -> bool;
    pub fn wxTextEntry_AutoCompleteDirectories(self_: *mut c_void) -> bool;
    pub fn wxTextEntry_CanCopy(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanCut(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanPaste(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanRedo(self_: *const c_void) -> bool;
    pub fn wxTextEntry_CanUndo(self_: *const c_void) -> bool;
    pub fn wxTextEntry_ChangeValue(self_: *mut c_void, value: *const c_void);
    pub fn wxTextEntry_Clear(self_: *mut c_void);
    pub fn wxTextEntry_Copy(self_: *mut c_void);
    pub fn wxTextEntry_Cut(self_: *mut c_void);
    pub fn wxTextEntry_ForceUpper(self_: *mut c_void);
    pub fn wxTextEntry_GetInsertionPoint(self_: *const c_void) -> c_long;
    // NOT_SUPPORTED: pub fn wxTextEntry_GetLastPosition(self_: *const c_void) -> wxTextPos;
    pub fn wxTextEntry_GetRange(self_: *const c_void, from: c_long, to: c_long) -> *mut c_void;
    pub fn wxTextEntry_GetSelection(self_: *const c_void, from: *mut c_void, to: *mut c_void);
    pub fn wxTextEntry_GetStringSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_IsEditable(self_: *const c_void) -> bool;
    pub fn wxTextEntry_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxTextEntry_Paste(self_: *mut c_void);
    pub fn wxTextEntry_Redo(self_: *mut c_void);
    pub fn wxTextEntry_Remove(self_: *mut c_void, from: c_long, to: c_long);
    pub fn wxTextEntry_Replace(self_: *mut c_void, from: c_long, to: c_long, value: *const c_void);
    pub fn wxTextEntry_SetEditable(self_: *mut c_void, editable: bool);
    pub fn wxTextEntry_SetInsertionPoint(self_: *mut c_void, pos: c_long);
    pub fn wxTextEntry_SetInsertionPointEnd(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxTextEntry_SetMaxLength(self_: *mut c_void, len: unsigned long);
    pub fn wxTextEntry_SetSelection(self_: *mut c_void, from: c_long, to: c_long);
    pub fn wxTextEntry_SelectAll(self_: *mut c_void);
    pub fn wxTextEntry_SelectNone(self_: *mut c_void);
    pub fn wxTextEntry_SetHint(self_: *mut c_void, hint: *const c_void) -> bool;
    pub fn wxTextEntry_GetHint(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_SetMargins(self_: *mut c_void, pt: *const c_void) -> bool;
    pub fn wxTextEntry_SetMargins1(self_: *mut c_void, left: c_int, top: c_int) -> bool;
    pub fn wxTextEntry_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntry_SetValue(self_: *mut c_void, value: *const c_void);
    pub fn wxTextEntry_Undo(self_: *mut c_void);
    pub fn wxTextEntry_WriteText(self_: *mut c_void, text: *const c_void);

    // wxTextEntryDialog
    pub fn wxTextEntryDialog_CLASSINFO() -> *mut c_void;
    pub fn wxTextEntryDialog_new() -> *mut c_void;
    pub fn wxTextEntryDialog_new1(
        parent: *mut c_void,
        message: *const c_void,
        caption: *const c_void,
        value: *const c_void,
        style: c_long,
        pos: *const c_void,
    ) -> *mut c_void;
    pub fn wxTextEntryDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        message: *const c_void,
        caption: *const c_void,
        value: *const c_void,
        style: c_long,
        pos: *const c_void,
    ) -> bool;
    // DTOR: pub fn wxTextEntryDialog_~wxTextEntryDialog(self_: *mut c_void);
    pub fn wxTextEntryDialog_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxTextEntryDialog_SetTextValidator(self_: *mut c_void, validator: *const c_void);
    // NOT_SUPPORTED: pub fn wxTextEntryDialog_SetTextValidator1(self_: *mut c_void, style: wxTextValidatorStyle);
    // NOT_SUPPORTED: pub fn wxTextEntryDialog_SetMaxLength(self_: *mut c_void, len: unsigned long);
    pub fn wxTextEntryDialog_SetValue(self_: *mut c_void, value: *const c_void);
    pub fn wxTextEntryDialog_ForceUpper(self_: *mut c_void);

    // wxTextValidator
    pub fn wxTextValidator_CLASSINFO() -> *mut c_void;
    pub fn wxTextValidator_new(validator: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_new1(style: c_long, val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxTextValidator_GetCharExcludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetCharIncludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetExcludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetIncludes(self_: *const c_void) -> *mut c_void;
    pub fn wxTextValidator_GetStyle(self_: *const c_void) -> c_long;
    // NOT_SUPPORTED: pub fn wxTextValidator_HasFlag(self_: *const c_void, style: wxTextValidatorStyle) -> bool;
    pub fn wxTextValidator_OnChar(self_: *mut c_void, event: *mut c_void);
    pub fn wxTextValidator_SetExcludes(self_: *mut c_void, string_list: *const c_void);
    pub fn wxTextValidator_SetCharExcludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_SetIncludes(self_: *mut c_void, string_list: *const c_void);
    pub fn wxTextValidator_SetCharIncludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_AddExclude(self_: *mut c_void, exclude: *const c_void);
    pub fn wxTextValidator_AddInclude(self_: *mut c_void, include: *const c_void);
    pub fn wxTextValidator_AddCharExcludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_AddCharIncludes(self_: *mut c_void, chars: *const c_void);
    pub fn wxTextValidator_SetStyle(self_: *mut c_void, style: c_long);
    pub fn wxTextValidator_IsValid(self_: *const c_void, val: *const c_void) -> *mut c_void;

    // wxThreadEvent
    pub fn wxThreadEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxThreadEvent_new(event_type: wxEventType, id: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxThreadEvent_SetPayload(self_: *mut c_void, payload: *const c_void);
    // NOT_SUPPORTED: pub fn wxThreadEvent_GetPayload(self_: *const c_void) -> T;
    pub fn wxThreadEvent_GetExtraLong(self_: *const c_void) -> c_long;
    pub fn wxThreadEvent_GetInt(self_: *const c_void) -> c_int;
    pub fn wxThreadEvent_GetString(self_: *const c_void) -> *mut c_void;
    pub fn wxThreadEvent_SetExtraLong(self_: *mut c_void, extra_long: c_long);
    pub fn wxThreadEvent_SetInt(self_: *mut c_void, int_command: c_int);
    pub fn wxThreadEvent_SetString(self_: *mut c_void, string: *const c_void);

    // wxTimePickerCtrl
    pub fn wxTimePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTimePickerCtrl_new() -> *mut c_void;
    pub fn wxTimePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxTimePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxTimePickerCtrl_GetTime(
        self_: *const c_void,
        hour: *mut c_void,
        min: *mut c_void,
        sec: *mut c_void,
    ) -> bool;
    pub fn wxTimePickerCtrl_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxTimePickerCtrl_SetTime(
        self_: *mut c_void,
        hour: c_int,
        min: c_int,
        sec: c_int,
    ) -> bool;
    pub fn wxTimePickerCtrl_SetValue(self_: *mut c_void, dt: *const c_void);

    // wxTipProvider
    pub fn wxTipProvider_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxTipProvider_new(current_tip: usize) -> *mut c_void;
    // DTOR: pub fn wxTipProvider_~wxTipProvider(self_: *mut c_void);
    pub fn wxTipProvider_GetCurrentTip(self_: *const c_void) -> usize;
    pub fn wxTipProvider_GetTip(self_: *mut c_void) -> *mut c_void;

    // wxTipWindow
    pub fn wxTipWindow_CLASSINFO() -> *mut c_void;
    pub fn wxTipWindow_new(
        parent: *mut c_void,
        text: *const c_void,
        max_length: c_int,
        window_ptr: *mut c_void,
        rect_bounds: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTipWindow_SetBoundingRect(self_: *mut c_void, rect_bound: *const c_void);
    pub fn wxTipWindow_SetTipWindowPtr(self_: *mut c_void, window_ptr: *mut c_void);

    // wxToggleButton
    pub fn wxToggleButton_CLASSINFO() -> *mut c_void;
    pub fn wxToggleButton_new() -> *mut c_void;
    pub fn wxToggleButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxToggleButton_~wxToggleButton(self_: *mut c_void);
    pub fn wxToggleButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxToggleButton_GetValue(self_: *const c_void) -> bool;
    pub fn wxToggleButton_SetValue(self_: *mut c_void, state: bool);

    // wxToolBar
    pub fn wxToolBar_CLASSINFO() -> *mut c_void;
    pub fn wxToolBar_new() -> *mut c_void;
    pub fn wxToolBar_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxToolBar_~wxToolBar(self_: *mut c_void);
    pub fn wxToolBar_AddCheckTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap1: *const c_void,
        bmp_disabled: *const c_void,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddControl(
        self_: *mut c_void,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddRadioTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap1: *const c_void,
        bmp_disabled: *const c_void,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddStretchableSpace(self_: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddTool(self_: *mut c_void, tool: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddTool1(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        short_help: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxToolBar_AddTool2(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_ClearTools(self_: *mut c_void);
    pub fn wxToolBar_DeleteTool(self_: *mut c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_DeleteToolByPos(self_: *mut c_void, pos: usize) -> bool;
    pub fn wxToolBar_EnableTool(self_: *mut c_void, tool_id: c_int, enable: bool);
    pub fn wxToolBar_FindById(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_FindControl(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_FindToolForPosition(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxToolBar_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxToolBar_GetToolBitmapSize(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxToolBar_GetToolByPos(self_: *mut c_void, pos: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolByPos1(self_: *const c_void, pos: c_int) -> *const c_void;
    pub fn wxToolBar_GetToolClientData(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolEnabled(self_: *const c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_GetToolLongHelp(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolPacking(self_: *const c_void) -> c_int;
    pub fn wxToolBar_GetToolPos(self_: *const c_void, tool_id: c_int) -> c_int;
    pub fn wxToolBar_GetToolSeparation(self_: *const c_void) -> c_int;
    pub fn wxToolBar_GetToolShortHelp(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolSize(self_: *const c_void) -> *mut c_void;
    pub fn wxToolBar_GetToolState(self_: *const c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_GetToolsCount(self_: *const c_void) -> usize;
    pub fn wxToolBar_InsertControl(
        self_: *mut c_void,
        pos: usize,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_InsertSeparator(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxToolBar_InsertStretchableSpace(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxToolBar_InsertTool(
        self_: *mut c_void,
        pos: usize,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_InsertTool1(self_: *mut c_void, pos: usize, tool: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_OnLeftClick(self_: *mut c_void, tool_id: c_int, toggle_down: bool) -> bool;
    pub fn wxToolBar_OnMouseEnter(self_: *mut c_void, tool_id: c_int);
    pub fn wxToolBar_OnRightClick(self_: *mut c_void, tool_id: c_int, x: c_long, y: c_long);
    pub fn wxToolBar_Realize(self_: *mut c_void) -> bool;
    pub fn wxToolBar_RemoveTool(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_SetDropdownMenu(self_: *mut c_void, id: c_int, menu: *mut c_void) -> bool;
    pub fn wxToolBar_SetMargins(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxToolBar_SetMargins1(self_: *mut c_void, size: *const c_void);
    pub fn wxToolBar_SetToolBitmapSize(self_: *mut c_void, size: *const c_void);
    pub fn wxToolBar_SetToolClientData(self_: *mut c_void, id: c_int, client_data: *mut c_void);
    pub fn wxToolBar_SetToolDisabledBitmap(self_: *mut c_void, id: c_int, bitmap: *const c_void);
    pub fn wxToolBar_SetToolLongHelp(
        self_: *mut c_void,
        tool_id: c_int,
        help_string: *const c_void,
    );
    pub fn wxToolBar_SetToolNormalBitmap(self_: *mut c_void, id: c_int, bitmap: *const c_void);
    pub fn wxToolBar_SetToolPacking(self_: *mut c_void, packing: c_int);
    pub fn wxToolBar_SetToolSeparation(self_: *mut c_void, separation: c_int);
    pub fn wxToolBar_SetToolShortHelp(
        self_: *mut c_void,
        tool_id: c_int,
        help_string: *const c_void,
    );
    pub fn wxToolBar_ToggleTool(self_: *mut c_void, tool_id: c_int, toggle: bool);
    pub fn wxToolBar_CreateTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bmp_normal: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        client_data: *mut c_void,
        short_help: *const c_void,
        long_help: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_CreateTool1(
        self_: *mut c_void,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_CreateSeparator(self_: *mut c_void) -> *mut c_void;

    // wxToolTip
    pub fn wxToolTip_CLASSINFO() -> *mut c_void;
    pub fn wxToolTip_new(tip: *const c_void) -> *mut c_void;
    pub fn wxToolTip_GetTip(self_: *const c_void) -> *mut c_void;
    pub fn wxToolTip_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxToolTip_SetTip(self_: *mut c_void, tip: *const c_void);
    pub fn wxToolTip_Enable(flag: bool);
    pub fn wxToolTip_SetAutoPop(msecs: c_long);
    pub fn wxToolTip_SetDelay(msecs: c_long);
    pub fn wxToolTip_SetMaxWidth(width: c_int);
    pub fn wxToolTip_SetReshow(msecs: c_long);

    // wxToolbook
    pub fn wxToolbook_CLASSINFO() -> *mut c_void;
    pub fn wxToolbook_new() -> *mut c_void;
    pub fn wxToolbook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolbook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxToolbook_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxToolbook_EnablePage(self_: *mut c_void, page: usize, enable: bool) -> bool;
    pub fn wxToolbook_EnablePage1(self_: *mut c_void, page: *mut c_void, enable: bool) -> bool;

    // wxTopLevelWindow
    pub fn wxTopLevelWindow_CLASSINFO() -> *mut c_void;
    pub fn wxTopLevelWindow_new() -> *mut c_void;
    pub fn wxTopLevelWindow_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTopLevelWindow_~wxTopLevelWindow(self_: *mut c_void);
    pub fn wxTopLevelWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxTopLevelWindow_CenterOnScreen(self_: *mut c_void, direction: c_int);
    pub fn wxTopLevelWindow_CentreOnScreen(self_: *mut c_void, direction: c_int);
    pub fn wxTopLevelWindow_EnableCloseButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_GetDefaultItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetIcon(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxTopLevelWindow_GetIcons(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_Iconize(self_: *mut c_void, iconize: bool);
    pub fn wxTopLevelWindow_IsActive(self_: *mut c_void) -> bool;
    pub fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsFullScreen(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsIconized(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsMaximized(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_IsUsingNativeDecorations(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_Maximize(self_: *mut c_void, maximize: bool);
    pub fn wxTopLevelWindow_MSWGetSystemMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_RequestUserAttention(self_: *mut c_void, flags: c_int);
    pub fn wxTopLevelWindow_Restore(self_: *mut c_void);
    // BLOCKED: pub fn wxTopLevelWindow_RestoreToGeometry(self_: *mut c_void, ser: *mut c_void) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_SaveGeometry(self_: *const c_void, ser: *const c_void) -> bool;
    pub fn wxTopLevelWindow_SetDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxTopLevelWindow_SetIcons(self_: *mut c_void, icons: *const c_void);
    pub fn wxTopLevelWindow_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_OSXSetModified(self_: *mut c_void, modified: bool);
    pub fn wxTopLevelWindow_OSXIsModified(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut c_void, filename: *const c_void);
    pub fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut c_void);
    pub fn wxTopLevelWindow_EnableFullScreenView(
        self_: *mut c_void,
        enable: bool,
        style: c_long,
    ) -> bool;
    pub fn wxTopLevelWindow_ShowFullScreen(self_: *mut c_void, show: bool, style: c_long) -> bool;
    // NOT_SUPPORTED: pub fn wxTopLevelWindow_GetContentProtection(self_: *const c_void) -> wxContentProtection;
    // NOT_SUPPORTED: pub fn wxTopLevelWindow_SetContentProtection(self_: *mut c_void, content_protection: wxContentProtection) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorations(self_: *mut c_void, native: bool);
    // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorationsByDefault(self_: *mut c_void, native: bool);
    pub fn wxTopLevelWindow_GetDefaultSize() -> *mut c_void;

    // wxTreeCtrl
    pub fn wxTreeCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTreeCtrl_new() -> *mut c_void;
    pub fn wxTreeCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTreeCtrl_~wxTreeCtrl(self_: *mut c_void);
    pub fn wxTreeCtrl_AddRoot(
        self_: *mut c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_AppendItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_AssignButtonsImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_AssignStateImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_Collapse(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_CollapseAll(self_: *mut c_void);
    pub fn wxTreeCtrl_CollapseAllChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_CollapseAndReset(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxTreeCtrl_Delete(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxTreeCtrl_DeleteChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_EditLabel(
        self_: *mut c_void,
        item: *const c_void,
        text_ctrl_class: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_EnableBellOnNoMatch(self_: *mut c_void, on: bool);
    pub fn wxTreeCtrl_EndEditLabel(self_: *mut c_void, item: *const c_void, discard_changes: bool);
    pub fn wxTreeCtrl_EnsureVisible(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Expand(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_ExpandAll(self_: *mut c_void);
    pub fn wxTreeCtrl_ExpandAllChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_GetBoundingRect(
        self_: *const c_void,
        item: *const c_void,
        rect: *mut c_void,
        text_only: bool,
    ) -> bool;
    pub fn wxTreeCtrl_GetButtonsImageList(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetChildrenCount(
        self_: *const c_void,
        item: *const c_void,
        recursively: bool,
    ) -> usize;
    pub fn wxTreeCtrl_GetCount(self_: *const c_void) -> c_uint;
    pub fn wxTreeCtrl_GetEditControl(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetFirstChild(
        self_: *const c_void,
        item: *const c_void,
        cookie: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_GetFirstVisibleItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetFocusedItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_ClearFocusedItem(self_: *mut c_void);
    pub fn wxTreeCtrl_SetFocusedItem(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_GetIndent(self_: *const c_void) -> c_uint;
    pub fn wxTreeCtrl_GetSpacing(self_: *const c_void) -> c_uint;
    pub fn wxTreeCtrl_GetItemBackgroundColour(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemData(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemFont(self_: *const c_void, item: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeCtrl_GetItemImage(self_: *const c_void, item: *const c_void, which: wxTreeItemIcon) -> c_int;
    pub fn wxTreeCtrl_GetItemParent(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemState(self_: *const c_void, item: *const c_void) -> c_int;
    pub fn wxTreeCtrl_GetItemText(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetItemTextColour(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetLastChild(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetNextChild(
        self_: *const c_void,
        item: *const c_void,
        cookie: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_GetNextSibling(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetNextVisible(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetPrevSibling(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetPrevVisible(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetQuickBestSize(self_: *const c_void) -> bool;
    pub fn wxTreeCtrl_GetRootItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_GetSelections(self_: *const c_void, selection: *mut c_void) -> usize;
    pub fn wxTreeCtrl_GetStateImageList(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeCtrl_HitTest(
        self_: *const c_void,
        point: *const c_void,
        flags: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_InsertItem(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_InsertItem1(
        self_: *mut c_void,
        parent: *const c_void,
        pos: usize,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_IsBold(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsExpanded(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsSelected(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_IsVisible(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_ItemHasChildren(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxTreeCtrl_OnCompareItems(
        self_: *mut c_void,
        item1: *const c_void,
        item2: *const c_void,
    ) -> c_int;
    pub fn wxTreeCtrl_PrependItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        image: c_int,
        sel_image: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxTreeCtrl_ScrollTo(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_SelectItem(self_: *mut c_void, item: *const c_void, select: bool);
    pub fn wxTreeCtrl_SetButtonsImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_SetIndent(self_: *mut c_void, indent: c_uint);
    pub fn wxTreeCtrl_SetSpacing(self_: *mut c_void, spacing: c_uint);
    pub fn wxTreeCtrl_SetItemBackgroundColour(
        self_: *mut c_void,
        item: *const c_void,
        col: *const c_void,
    );
    pub fn wxTreeCtrl_SetItemBold(self_: *mut c_void, item: *const c_void, bold: bool);
    pub fn wxTreeCtrl_SetItemData(self_: *mut c_void, item: *const c_void, data: *mut c_void);
    pub fn wxTreeCtrl_SetItemDropHighlight(
        self_: *mut c_void,
        item: *const c_void,
        highlight: bool,
    );
    pub fn wxTreeCtrl_SetItemFont(self_: *mut c_void, item: *const c_void, font: *const c_void);
    pub fn wxTreeCtrl_SetItemHasChildren(
        self_: *mut c_void,
        item: *const c_void,
        has_children: bool,
    );
    // NOT_SUPPORTED: pub fn wxTreeCtrl_SetItemImage(self_: *mut c_void, item: *const c_void, image: c_int, which: wxTreeItemIcon);
    pub fn wxTreeCtrl_SetItemState(self_: *mut c_void, item: *const c_void, state: c_int);
    pub fn wxTreeCtrl_SetItemText(self_: *mut c_void, item: *const c_void, text: *const c_void);
    pub fn wxTreeCtrl_SetItemTextColour(
        self_: *mut c_void,
        item: *const c_void,
        col: *const c_void,
    );
    pub fn wxTreeCtrl_SetQuickBestSize(self_: *mut c_void, quick_best_size: bool);
    pub fn wxTreeCtrl_SetStateImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeCtrl_SetWindowStyle(self_: *mut c_void, styles: c_long);
    pub fn wxTreeCtrl_SortChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Toggle(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_ToggleItemSelection(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_Unselect(self_: *mut c_void);
    pub fn wxTreeCtrl_UnselectAll(self_: *mut c_void);
    pub fn wxTreeCtrl_UnselectItem(self_: *mut c_void, item: *const c_void);
    pub fn wxTreeCtrl_SelectChildren(self_: *mut c_void, parent: *const c_void);

    // wxTreeEvent
    pub fn wxTreeEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeEvent_new(command_type: wxEventType, tree: *mut c_void, item: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxTreeEvent_GetKeyEvent(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetOldItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_GetPoint(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeEvent_IsEditCancelled(self_: *const c_void) -> bool;
    pub fn wxTreeEvent_SetToolTip(self_: *mut c_void, tooltip: *const c_void);

    // wxTreeItemData
    pub fn wxTreeItemData_delete(self_: *mut c_void);
    pub fn wxTreeItemData_new() -> *mut c_void;
    // DTOR: pub fn wxTreeItemData_~wxTreeItemData(self_: *mut c_void);
    pub fn wxTreeItemData_GetId(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeItemData_SetId(self_: *mut c_void, id: *const c_void);

    // wxTreeItemId
    pub fn wxTreeItemId_delete(self_: *mut c_void);
    pub fn wxTreeItemId_new() -> *mut c_void;
    pub fn wxTreeItemId_IsOk(self_: *const c_void) -> bool;
    pub fn wxTreeItemId_GetID(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeItemId_Unset(self_: *mut c_void);

    // wxTreeListCtrl
    pub fn wxTreeListCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxTreeListCtrl_AssignImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeListCtrl_SetImageList(self_: *mut c_void, image_list: *mut c_void);
    pub fn wxTreeListCtrl_AppendColumn(
        self_: *mut c_void,
        title: *const c_void,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> c_int;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetColumnCount(self_: *const c_void) -> unsigned;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_DeleteColumn(self_: *mut c_void, col: unsigned) -> bool;
    pub fn wxTreeListCtrl_ClearColumns(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_SetColumnWidth(self_: *mut c_void, col: unsigned, width: c_int);
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetColumnWidth(self_: *const c_void, col: unsigned) -> c_int;
    pub fn wxTreeListCtrl_WidthFor(self_: *const c_void, text: *const c_void) -> c_int;
    // BLOCKED: pub fn wxTreeListCtrl_AppendItem(self_: *mut c_void, parent: wxTreeListItem, text: *const c_void, image_closed: c_int, image_opened: c_int, data: *mut c_void) -> wxTreeListItem;
    // BLOCKED: pub fn wxTreeListCtrl_InsertItem(self_: *mut c_void, parent: wxTreeListItem, previous: wxTreeListItem, text: *const c_void, image_closed: c_int, image_opened: c_int, data: *mut c_void) -> wxTreeListItem;
    // BLOCKED: pub fn wxTreeListCtrl_PrependItem(self_: *mut c_void, parent: wxTreeListItem, text: *const c_void, image_closed: c_int, image_opened: c_int, data: *mut c_void) -> wxTreeListItem;
    // BLOCKED: pub fn wxTreeListCtrl_DeleteItem(self_: *mut c_void, item: wxTreeListItem);
    pub fn wxTreeListCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxTreeListCtrl_GetRootItem(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxTreeListCtrl_GetItemParent(self_: *const c_void, item: wxTreeListItem) -> wxTreeListItem;
    // BLOCKED: pub fn wxTreeListCtrl_GetFirstChild(self_: *const c_void, item: wxTreeListItem) -> wxTreeListItem;
    // BLOCKED: pub fn wxTreeListCtrl_GetNextSibling(self_: *const c_void, item: wxTreeListItem) -> wxTreeListItem;
    pub fn wxTreeListCtrl_GetFirstItem(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxTreeListCtrl_GetNextItem(self_: *const c_void, item: wxTreeListItem) -> wxTreeListItem;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetItemText(self_: *const c_void, item: wxTreeListItem, col: unsigned) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_SetItemText(self_: *mut c_void, item: wxTreeListItem, col: unsigned, text: *const c_void);
    // BLOCKED: pub fn wxTreeListCtrl_SetItemText1(self_: *mut c_void, item: wxTreeListItem, text: *const c_void);
    // BLOCKED: pub fn wxTreeListCtrl_SetItemImage(self_: *mut c_void, item: wxTreeListItem, closed: c_int, opened: c_int);
    // BLOCKED: pub fn wxTreeListCtrl_GetItemData(self_: *const c_void, item: wxTreeListItem) -> *mut c_void;
    // BLOCKED: pub fn wxTreeListCtrl_SetItemData(self_: *mut c_void, item: wxTreeListItem, data: *mut c_void);
    // BLOCKED: pub fn wxTreeListCtrl_Expand(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_Collapse(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_IsExpanded(self_: *const c_void, item: wxTreeListItem) -> bool;
    pub fn wxTreeListCtrl_GetSelection(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_GetSelections(self_: *const c_void, selections: *mut c_void) -> unsigned;
    // BLOCKED: pub fn wxTreeListCtrl_Select(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_Unselect(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_IsSelected(self_: *const c_void, item: wxTreeListItem) -> bool;
    pub fn wxTreeListCtrl_SelectAll(self_: *mut c_void);
    pub fn wxTreeListCtrl_UnselectAll(self_: *mut c_void);
    // BLOCKED: pub fn wxTreeListCtrl_EnsureVisible(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_CheckItem(self_: *mut c_void, item: wxTreeListItem, state: c_int);
    // BLOCKED: pub fn wxTreeListCtrl_CheckItemRecursively(self_: *mut c_void, item: wxTreeListItem, state: c_int);
    // BLOCKED: pub fn wxTreeListCtrl_UncheckItem(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_UpdateItemParentStateRecursively(self_: *mut c_void, item: wxTreeListItem);
    // BLOCKED: pub fn wxTreeListCtrl_GetCheckedState(self_: *const c_void, item: wxTreeListItem) -> c_int;
    // BLOCKED: pub fn wxTreeListCtrl_AreAllChildrenInState(self_: *const c_void, item: wxTreeListItem, state: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxTreeListCtrl_SetSortColumn(self_: *mut c_void, col: unsigned, ascending_order: bool);
    pub fn wxTreeListCtrl_GetSortColumn(
        self_: *mut c_void,
        col: *mut c_void,
        ascending_order: *mut c_void,
    ) -> bool;
    pub fn wxTreeListCtrl_SetItemComparator(self_: *mut c_void, comparator: *mut c_void);
    pub fn wxTreeListCtrl_GetView(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeListCtrl_GetDataView(self_: *const c_void) -> *mut c_void;
    pub fn wxTreeListCtrl_new() -> *mut c_void;
    pub fn wxTreeListCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxTreeListCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxTreeListItem
    pub fn wxTreeListItem_delete(self_: *mut c_void);
    pub fn wxTreeListItem_new() -> *mut c_void;
    pub fn wxTreeListItem_IsOk(self_: *const c_void) -> bool;

    // wxTreeListItemComparator
    pub fn wxTreeListItemComparator_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxTreeListItemComparator_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTreeListItemComparator_Compare(self_: *mut c_void, treelist: *mut c_void, column: unsigned, first: wxTreeListItem, second: wxTreeListItem) -> c_int;
    // DTOR: pub fn wxTreeListItemComparator_~wxTreeListItemComparator(self_: *mut c_void);

    // wxTreebook
    pub fn wxTreebook_CLASSINFO() -> *mut c_void;
    pub fn wxTreebook_new() -> *mut c_void;
    pub fn wxTreebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTreebook_~wxTreebook(self_: *mut c_void);
    pub fn wxTreebook_AddSubPage(
        self_: *mut c_void,
        page: *mut c_void,
        text: *const c_void,
        b_select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxTreebook_CollapseNode(self_: *mut c_void, page_id: usize) -> bool;
    pub fn wxTreebook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxTreebook_ExpandNode(self_: *mut c_void, page_id: usize, expand: bool) -> bool;
    pub fn wxTreebook_GetPageParent(self_: *const c_void, page: usize) -> c_int;
    pub fn wxTreebook_InsertSubPage(
        self_: *mut c_void,
        page_pos: usize,
        page: *mut c_void,
        text: *const c_void,
        b_select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxTreebook_IsNodeExpanded(self_: *const c_void, page_id: usize) -> bool;

}
