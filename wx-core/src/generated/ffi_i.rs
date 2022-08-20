use super::*;

extern "C" {

    // wxIFFHandler
    pub fn wxIFFHandler_CLASSINFO() -> *mut c_void;
    pub fn wxIFFHandler_new() -> *mut c_void;

    // wxIcon
    pub fn wxIcon_CLASSINFO() -> *mut c_void;
    pub fn wxIcon_new() -> *mut c_void;
    pub fn wxIcon_new1(icon: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIcon_new2(bits: char, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxIcon_new3(bits: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIcon_new4(name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> *mut c_void;
    pub fn wxIcon_new5(loc: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxIcon_~wxIcon(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxIcon_CreateFromHICON(self_: *mut c_void, icon: WXHICON) -> bool;
    // NOT_SUPPORTED: pub fn wxIcon_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    pub fn wxIcon_CopyFromBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxIcon_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxIcon_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxIcon_GetLogicalHeight(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetLogicalSize(self_: *const c_void) -> *mut c_void;
    pub fn wxIcon_GetLogicalWidth(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxIcon_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxIcon_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxIcon_LoadFile(self_: *mut c_void, name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> bool;
    pub fn wxIcon_SetDepth(self_: *mut c_void, depth: c_int);
    pub fn wxIcon_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxIcon_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxIcon_operator=(self_: *mut c_void, icon: *const c_void) -> *mut c_void;

    // wxIconBundle
    pub fn wxIconBundle_CLASSINFO() -> *mut c_void;
    pub fn wxIconBundle_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIconBundle_new1(file: *const c_void, type_: wxBitmapType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIconBundle_new2(stream: *mut c_void, type_: wxBitmapType) -> *mut c_void;
    pub fn wxIconBundle_new3(icon: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIconBundle_new4(resource_name: *const c_void, module: WXHINSTANCE) -> *mut c_void;
    pub fn wxIconBundle_new5(ic: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxIconBundle_~wxIconBundle(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxIconBundle_AddIcon(self_: *mut c_void, file: *const c_void, type_: wxBitmapType);
    // NOT_SUPPORTED: pub fn wxIconBundle_AddIcon1(self_: *mut c_void, stream: *mut c_void, type_: wxBitmapType);
    // NOT_SUPPORTED: pub fn wxIconBundle_AddIcon2(self_: *mut c_void, resource_name: *const c_void, module: WXHINSTANCE);
    pub fn wxIconBundle_AddIcon3(self_: *mut c_void, icon: *const c_void);
    pub fn wxIconBundle_GetIcon(
        self_: *const c_void,
        size: *const c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxIconBundle_GetIcon1(self_: *const c_void, size: c_int, flags: c_int) -> *mut c_void;
    pub fn wxIconBundle_GetIconOfExactSize(
        self_: *const c_void,
        size: *const c_void,
    ) -> *mut c_void;
    pub fn wxIconBundle_GetIconCount(self_: *const c_void) -> usize;
    pub fn wxIconBundle_GetIconByIndex(self_: *const c_void, n: usize) -> *mut c_void;
    pub fn wxIconBundle_IsEmpty(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxIconBundle_operator=(self_: *mut c_void, ic: *const c_void) -> *mut c_void;

    // wxIconizeEvent
    pub fn wxIconizeEvent_CLASSINFO() -> *mut c_void;
    pub fn wxIconizeEvent_new(id: c_int, iconized: bool) -> *mut c_void;
    pub fn wxIconizeEvent_IsIconized(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxIconizeEvent_Iconized(self_: *const c_void) -> bool;

    // wxIdManager
    pub fn wxIdManager_delete(self_: *mut c_void);
    pub fn wxIdManager_ReserveId(count: c_int) -> c_int;
    pub fn wxIdManager_UnreserveId(id: c_int, count: c_int);

    // wxImage
    pub fn wxImage_CLASSINFO() -> *mut c_void;
    pub fn wxImage_Copy(self_: *const c_void) -> *mut c_void;
    pub fn wxImage_Create(self_: *mut c_void, width: c_int, height: c_int, clear: bool) -> bool;
    pub fn wxImage_Create1(self_: *mut c_void, sz: *const c_void, clear: bool) -> bool;
    pub fn wxImage_Create2(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> bool;
    pub fn wxImage_Create3(
        self_: *mut c_void,
        sz: *const c_void,
        data: *mut c_void,
        static_data: bool,
    ) -> bool;
    pub fn wxImage_Create4(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> bool;
    pub fn wxImage_Create5(
        self_: *mut c_void,
        sz: *const c_void,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_Clear(self_: *mut c_void, value: unsigned char);
    pub fn wxImage_Destroy(self_: *mut c_void);
    pub fn wxImage_InitAlpha(self_: *mut c_void);
    pub fn wxImage_Blur(self_: *const c_void, blur_radius: c_int) -> *mut c_void;
    pub fn wxImage_BlurHorizontal(self_: *const c_void, blur_radius: c_int) -> *mut c_void;
    pub fn wxImage_BlurVertical(self_: *const c_void, blur_radius: c_int) -> *mut c_void;
    pub fn wxImage_Mirror(self_: *const c_void, horizontally: bool) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_Paste(self_: *mut c_void, image: *const c_void, x: c_int, y: c_int, alpha_blend: wxImageAlphaBlendMode);
    // NOT_SUPPORTED: pub fn wxImage_Replace(self_: *mut c_void, r1: unsigned char, g1: unsigned char, b1: unsigned char, r2: unsigned char, g2: unsigned char, b2: unsigned char);
    // NOT_SUPPORTED: pub fn wxImage_Rescale(self_: *mut c_void, width: c_int, height: c_int, quality: wxImageResizeQuality) -> *mut c_void;
    pub fn wxImage_Resize(
        self_: *mut c_void,
        size: *const c_void,
        pos: *const c_void,
        red: c_int,
        green: c_int,
        blue: c_int,
    ) -> *mut c_void;
    pub fn wxImage_Rotate(
        self_: *const c_void,
        angle: c_double,
        rotation_centre: *const c_void,
        interpolating: bool,
        offset_after_rotation: *mut c_void,
    ) -> *mut c_void;
    pub fn wxImage_Rotate90(self_: *const c_void, clockwise: bool) -> *mut c_void;
    pub fn wxImage_Rotate180(self_: *const c_void) -> *mut c_void;
    pub fn wxImage_RotateHue(self_: *mut c_void, angle: c_double);
    pub fn wxImage_ChangeSaturation(self_: *mut c_void, factor: c_double);
    pub fn wxImage_ChangeBrightness(self_: *mut c_void, factor: c_double);
    pub fn wxImage_ChangeHSV(
        self_: *mut c_void,
        angle_h: c_double,
        factor_s: c_double,
        factor_v: c_double,
    );
    // NOT_SUPPORTED: pub fn wxImage_Scale(self_: *const c_void, width: c_int, height: c_int, quality: wxImageResizeQuality) -> *mut c_void;
    pub fn wxImage_Size(
        self_: *const c_void,
        size: *const c_void,
        pos: *const c_void,
        red: c_int,
        green: c_int,
        blue: c_int,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_ConvertAlphaToMask(self_: *mut c_void, threshold: unsigned char) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_ConvertAlphaToMask1(self_: *mut c_void, mr: unsigned char, mg: unsigned char, mb: unsigned char, threshold: unsigned char) -> bool;
    pub fn wxImage_ConvertToGreyscale(
        self_: *const c_void,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    ) -> *mut c_void;
    pub fn wxImage_ConvertToGreyscale1(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_ConvertToMono(self_: *const c_void, r: unsigned char, g: unsigned char, b: unsigned char) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    pub fn wxImage_ChangeLightness(self_: *const c_void, alpha: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_ComputeHistogram(self_: *const c_void, histogram: *mut c_void) -> unsigned long;
    // NOT_SUPPORTED: pub fn wxImage_FindFirstUnusedColour(self_: *const c_void, r: *mut c_void, g: *mut c_void, b: *mut c_void, start_r: unsigned char, start_g: unsigned char, start_b: unsigned char) -> bool;
    // BLOCKED: pub fn wxImage_operator=(self_: *mut c_void, image: *const c_void) -> *mut c_void;
    pub fn wxImage_GetAlpha(self_: *const c_void) -> *mut c_void;
    pub fn wxImage_GetData(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_GetAlpha1(self_: *const c_void, x: c_int, y: c_int) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxImage_GetRed(self_: *const c_void, x: c_int, y: c_int) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxImage_GetGreen(self_: *const c_void, x: c_int, y: c_int) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxImage_GetBlue(self_: *const c_void, x: c_int, y: c_int) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxImage_GetMaskRed(self_: *const c_void) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxImage_GetMaskGreen(self_: *const c_void) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxImage_GetMaskBlue(self_: *const c_void) -> unsigned char;
    pub fn wxImage_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxImage_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxImage_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxImage_GetOption(self_: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxImage_GetOptionInt(self_: *const c_void, name: *const c_void) -> c_int;
    pub fn wxImage_GetOrFindMaskColour(
        self_: *const c_void,
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
    ) -> bool;
    // BLOCKED: pub fn wxImage_GetPalette(self_: *const c_void) -> *const c_void;
    pub fn wxImage_GetSubImage(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_GetType(self_: *const c_void) -> wxBitmapType;
    pub fn wxImage_HasAlpha(self_: *const c_void) -> bool;
    pub fn wxImage_HasMask(self_: *const c_void) -> bool;
    pub fn wxImage_HasOption(self_: *const c_void, name: *const c_void) -> bool;
    pub fn wxImage_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_IsTransparent(self_: *const c_void, x: c_int, y: c_int, threshold: unsigned char) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_LoadFile(self_: *mut c_void, stream: *mut c_void, type_: wxBitmapType, index: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_LoadFile1(self_: *mut c_void, name: *const c_void, type_: wxBitmapType, index: c_int) -> bool;
    pub fn wxImage_LoadFile2(
        self_: *mut c_void,
        name: *const c_void,
        mimetype: *const c_void,
        index: c_int,
    ) -> bool;
    pub fn wxImage_LoadFile3(
        self_: *mut c_void,
        stream: *mut c_void,
        mimetype: *const c_void,
        index: c_int,
    ) -> bool;
    pub fn wxImage_SaveFile(
        self_: *const c_void,
        stream: *mut c_void,
        mimetype: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_SaveFile1(self_: *const c_void, name: *const c_void, type_: wxBitmapType) -> bool;
    pub fn wxImage_SaveFile2(
        self_: *const c_void,
        name: *const c_void,
        mimetype: *const c_void,
    ) -> bool;
    pub fn wxImage_SaveFile3(self_: *const c_void, name: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxImage_SaveFile4(self_: *const c_void, stream: *mut c_void, type_: wxBitmapType) -> bool;
    pub fn wxImage_SetAlpha(self_: *mut c_void, alpha: *mut c_void, static_data: bool);
    // NOT_SUPPORTED: pub fn wxImage_SetAlpha1(self_: *mut c_void, x: c_int, y: c_int, alpha: unsigned char);
    pub fn wxImage_ClearAlpha(self_: *mut c_void);
    pub fn wxImage_SetData(self_: *mut c_void, data: *mut c_void, static_data: bool);
    pub fn wxImage_SetData1(
        self_: *mut c_void,
        data: *mut c_void,
        new_width: c_int,
        new_height: c_int,
        static_data: bool,
    );
    pub fn wxImage_SetLoadFlags(self_: *mut c_void, flags: c_int);
    pub fn wxImage_SetMask(self_: *mut c_void, has_mask: bool);
    // NOT_SUPPORTED: pub fn wxImage_SetMaskColour(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char);
    // NOT_SUPPORTED: pub fn wxImage_SetMaskFromImage(self_: *mut c_void, mask: *const c_void, mr: unsigned char, mg: unsigned char, mb: unsigned char) -> bool;
    pub fn wxImage_SetOption(self_: *mut c_void, name: *const c_void, value: *const c_void);
    pub fn wxImage_SetOption1(self_: *mut c_void, name: *const c_void, value: c_int);
    pub fn wxImage_SetPalette(self_: *mut c_void, palette: *const c_void);
    // NOT_SUPPORTED: pub fn wxImage_SetRGB(self_: *mut c_void, x: c_int, y: c_int, r: unsigned char, g: unsigned char, b: unsigned char);
    // NOT_SUPPORTED: pub fn wxImage_SetRGB1(self_: *mut c_void, rect: *const c_void, red: unsigned char, green: unsigned char, blue: unsigned char);
    // NOT_SUPPORTED: pub fn wxImage_SetType(self_: *mut c_void, type_: wxBitmapType);
    pub fn wxImage_SetDefaultLoadFlags(flags: c_int);
    pub fn wxImage_GetLoadFlags(self_: *const c_void) -> c_int;
    pub fn wxImage_AddHandler(handler: *mut c_void);
    pub fn wxImage_CleanUpHandlers();
    pub fn wxImage_FindHandler(name: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_FindHandler1(extension: *const c_void, image_type: wxBitmapType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_FindHandler2(image_type: wxBitmapType) -> *mut c_void;
    pub fn wxImage_FindHandlerMime(mimetype: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxImage_GetHandlers() -> *mut c_void;
    pub fn wxImage_InitStandardHandlers();
    pub fn wxImage_InsertHandler(handler: *mut c_void);
    pub fn wxImage_RemoveHandler(name: *const c_void) -> bool;
    pub fn wxImage_CanRead(filename: *const c_void) -> bool;
    pub fn wxImage_CanRead1(stream: *mut c_void) -> bool;
    pub fn wxImage_GetDefaultLoadFlags() -> c_int;
    // NOT_SUPPORTED: pub fn wxImage_GetImageCount(filename: *const c_void, type_: wxBitmapType) -> c_int;
    // NOT_SUPPORTED: pub fn wxImage_GetImageCount1(stream: *mut c_void, type_: wxBitmapType) -> c_int;
    pub fn wxImage_GetImageExtWildcard() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_RGBtoHSV(rgb: *const c_void) -> wxImage::HSVValue;
    // NOT_SUPPORTED: pub fn wxImage_HSVtoRGB(hsv: *const c_void) -> wxImage::RGBValue;
    pub fn wxImage_new() -> *mut c_void;
    pub fn wxImage_new1(width: c_int, height: c_int, clear: bool) -> *mut c_void;
    pub fn wxImage_new2(sz: *const c_void, clear: bool) -> *mut c_void;
    pub fn wxImage_new3(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> *mut c_void;
    pub fn wxImage_new4(sz: *const c_void, data: *mut c_void, static_data: bool) -> *mut c_void;
    pub fn wxImage_new5(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> *mut c_void;
    pub fn wxImage_new6(
        sz: *const c_void,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> *mut c_void;
    pub fn wxImage_new7(xpm_data: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_new8(name: *const c_void, type_: wxBitmapType, index: c_int) -> *mut c_void;
    pub fn wxImage_new9(name: *const c_void, mimetype: *const c_void, index: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImage_new10(stream: *mut c_void, type_: wxBitmapType, index: c_int) -> *mut c_void;
    pub fn wxImage_new11(stream: *mut c_void, mimetype: *const c_void, index: c_int)
        -> *mut c_void;
    // DTOR: pub fn wxImage_~wxImage(self_: *mut c_void);

    // wxImageHandler
    pub fn wxImageHandler_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxImageHandler_new() -> *mut c_void;
    // DTOR: pub fn wxImageHandler_~wxImageHandler(self_: *mut c_void);
    pub fn wxImageHandler_CanRead(self_: *mut c_void, stream: *mut c_void) -> bool;
    pub fn wxImageHandler_CanRead1(self_: *mut c_void, filename: *const c_void) -> bool;
    pub fn wxImageHandler_GetExtension(self_: *const c_void) -> *mut c_void;
    pub fn wxImageHandler_GetAltExtensions(self_: *const c_void) -> *mut c_void;
    pub fn wxImageHandler_GetImageCount(self_: *mut c_void, stream: *mut c_void) -> c_int;
    pub fn wxImageHandler_GetMimeType(self_: *const c_void) -> *mut c_void;
    pub fn wxImageHandler_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxImageHandler_GetType(self_: *const c_void) -> wxBitmapType;
    pub fn wxImageHandler_LoadFile(
        self_: *mut c_void,
        image: *mut c_void,
        stream: *mut c_void,
        verbose: bool,
        index: c_int,
    ) -> bool;
    pub fn wxImageHandler_SaveFile(
        self_: *mut c_void,
        image: *mut c_void,
        stream: *mut c_void,
        verbose: bool,
    ) -> bool;
    pub fn wxImageHandler_SetExtension(self_: *mut c_void, extension: *const c_void);
    pub fn wxImageHandler_SetAltExtensions(self_: *mut c_void, extensions: *const c_void);
    pub fn wxImageHandler_SetMimeType(self_: *mut c_void, mimetype: *const c_void);
    pub fn wxImageHandler_SetName(self_: *mut c_void, name: *const c_void);
    // NOT_SUPPORTED: pub fn wxImageHandler_SetType(self_: *mut c_void, type_: wxBitmapType);
    // NOT_SUPPORTED: pub fn wxImageHandler_GetLibraryVersionInfo() -> wxVersionInfo;

    // wxImageList
    pub fn wxImageList_CLASSINFO() -> *mut c_void;
    pub fn wxImageList_new() -> *mut c_void;
    pub fn wxImageList_new1(
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> *mut c_void;
    pub fn wxImageList_Add(self_: *mut c_void, bitmap: *const c_void, mask: *const c_void)
        -> c_int;
    pub fn wxImageList_Add1(
        self_: *mut c_void,
        bitmap: *const c_void,
        mask_colour: *const c_void,
    ) -> c_int;
    pub fn wxImageList_Add2(self_: *mut c_void, icon: *const c_void) -> c_int;
    pub fn wxImageList_Create(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> bool;
    pub fn wxImageList_Destroy(self_: *mut c_void);
    pub fn wxImageList_Draw(
        self_: *mut c_void,
        index: c_int,
        dc: *mut c_void,
        x: c_int,
        y: c_int,
        flags: c_int,
        solid_background: bool,
    ) -> bool;
    pub fn wxImageList_GetBitmap(self_: *const c_void, index: c_int) -> *mut c_void;
    pub fn wxImageList_GetIcon(self_: *const c_void, index: c_int) -> *mut c_void;
    pub fn wxImageList_GetImageCount(self_: *const c_void) -> c_int;
    pub fn wxImageList_GetSize(
        self_: *const c_void,
        index: c_int,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool;
    pub fn wxImageList_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxImageList_Remove(self_: *mut c_void, index: c_int) -> bool;
    pub fn wxImageList_RemoveAll(self_: *mut c_void) -> bool;
    pub fn wxImageList_Replace(
        self_: *mut c_void,
        index: c_int,
        bitmap: *const c_void,
        mask: *const c_void,
    ) -> bool;
    pub fn wxImageList_Replace1(self_: *mut c_void, index: c_int, icon: *const c_void) -> bool;

    // wxInfoBar
    pub fn wxInfoBar_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxInfoBar_SetShowHideEffects(self_: *mut c_void, show_effect: wxShowEffect, hide_effect: wxShowEffect);
    // NOT_SUPPORTED: pub fn wxInfoBar_GetShowEffect(self_: *const c_void) -> wxShowEffect;
    // NOT_SUPPORTED: pub fn wxInfoBar_GetHideEffect(self_: *const c_void) -> wxShowEffect;
    pub fn wxInfoBar_SetEffectDuration(self_: *mut c_void, duration: c_int);
    pub fn wxInfoBar_GetEffectDuration(self_: *const c_void) -> c_int;
    pub fn wxInfoBar_new() -> *mut c_void;
    pub fn wxInfoBar_new1(parent: *mut c_void, winid: c_int) -> *mut c_void;
    pub fn wxInfoBar_Create(self_: *mut c_void, parent: *mut c_void, winid: c_int) -> bool;
    pub fn wxInfoBar_AddButton(self_: *mut c_void, btnid: c_int, label: *const c_void);
    pub fn wxInfoBar_Dismiss(self_: *mut c_void);
    pub fn wxInfoBar_RemoveButton(self_: *mut c_void, btnid: c_int);
    pub fn wxInfoBar_ShowMessage(self_: *mut c_void, msg: *const c_void, flags: c_int);
    pub fn wxInfoBar_GetButtonCount(self_: *const c_void) -> usize;
    pub fn wxInfoBar_GetButtonId(self_: *const c_void, idx: usize) -> c_int;
    pub fn wxInfoBar_HasButtonId(self_: *const c_void, btnid: c_int) -> bool;

    // wxInitDialogEvent
    pub fn wxInitDialogEvent_CLASSINFO() -> *mut c_void;
    pub fn wxInitDialogEvent_new(id: c_int) -> *mut c_void;

    // wxItemContainer
    pub fn wxItemContainer_delete(self_: *mut c_void);
    pub fn wxItemContainer_Append(self_: *mut c_void, item: *const c_void) -> c_int;
    pub fn wxItemContainer_Append1(
        self_: *mut c_void,
        item: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append2(
        self_: *mut c_void,
        item: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append3(self_: *mut c_void, items: *const c_void) -> c_int;
    // BLOCKED: pub fn wxItemContainer_Append4(self_: *mut c_void, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Append5(
        self_: *mut c_void,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append6(
        self_: *mut c_void,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append7(self_: *mut c_void, n: c_uint, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Append8(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append9(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Clear(self_: *mut c_void);
    pub fn wxItemContainer_Delete(self_: *mut c_void, n: c_uint);
    pub fn wxItemContainer_DetachClientObject(self_: *mut c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_HasClientData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_HasClientObjectData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_HasClientUntypedData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_GetClientData(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_GetClientObject(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_SetClientData(self_: *mut c_void, n: c_uint, data: *mut c_void);
    pub fn wxItemContainer_SetClientObject(self_: *mut c_void, n: c_uint, data: *mut c_void);
    pub fn wxItemContainer_Insert(self_: *mut c_void, item: *const c_void, pos: c_uint) -> c_int;
    pub fn wxItemContainer_Insert1(
        self_: *mut c_void,
        item: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert2(
        self_: *mut c_void,
        item: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert3(self_: *mut c_void, items: *const c_void, pos: c_uint) -> c_int;
    // BLOCKED: pub fn wxItemContainer_Insert4(self_: *mut c_void, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Insert5(
        self_: *mut c_void,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert6(
        self_: *mut c_void,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert7(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
    ) -> c_int;
    pub fn wxItemContainer_Insert8(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert9(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Set(self_: *mut c_void, items: *const c_void);
    // BLOCKED: pub fn wxItemContainer_Set1(self_: *mut c_void, items: *const c_void);
    pub fn wxItemContainer_Set2(self_: *mut c_void, items: *const c_void, client_data: *mut c_void);
    pub fn wxItemContainer_Set3(self_: *mut c_void, items: *const c_void, client_data: *mut c_void);
    pub fn wxItemContainer_Set4(self_: *mut c_void, n: c_uint, items: *const c_void);
    pub fn wxItemContainer_Set5(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    );
    pub fn wxItemContainer_Set6(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    );

    // wxItemContainerImmutable
    pub fn wxItemContainerImmutable_delete(self_: *mut c_void);
    pub fn wxItemContainerImmutable_SetSelection(self_: *mut c_void, n: c_int);
    pub fn wxItemContainerImmutable_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxItemContainerImmutable_SetStringSelection(
        self_: *mut c_void,
        string: *const c_void,
    ) -> bool;
    pub fn wxItemContainerImmutable_GetStringSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxItemContainerImmutable_Select(self_: *mut c_void, n: c_int);
    // BLOCKED: pub fn wxItemContainerImmutable_new() -> *mut c_void;
    pub fn wxItemContainerImmutable_GetCount(self_: *const c_void) -> c_uint;
    pub fn wxItemContainerImmutable_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxItemContainerImmutable_GetString(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainerImmutable_GetStrings(self_: *const c_void) -> *mut c_void;
    pub fn wxItemContainerImmutable_SetString(self_: *mut c_void, n: c_uint, string: *const c_void);
    pub fn wxItemContainerImmutable_FindString(
        self_: *const c_void,
        string: *const c_void,
        case_sensitive: bool,
    ) -> c_int;

}
