#include "generated.h"

extern "C" {

// CLASS: wxPCXHandler
wxClassInfo *wxPCXHandler_CLASSINFO() {
    return wxCLASSINFO(wxPCXHandler);
}
wxPCXHandler *wxPCXHandler_new() {
    return new wxPCXHandler();
}

// CLASS: wxPNGHandler
wxClassInfo *wxPNGHandler_CLASSINFO() {
    return wxCLASSINFO(wxPNGHandler);
}
wxPNGHandler *wxPNGHandler_new() {
    return new wxPNGHandler();
}

// CLASS: wxPNMHandler
wxClassInfo *wxPNMHandler_CLASSINFO() {
    return wxCLASSINFO(wxPNMHandler);
}
wxPNMHandler *wxPNMHandler_new() {
    return new wxPNMHandler();
}

// CLASS: wxPaintDC
wxClassInfo *wxPaintDC_CLASSINFO() {
    return wxCLASSINFO(wxPaintDC);
}
wxPaintDC *wxPaintDC_new(wxWindow * window) {
    return new wxPaintDC(window);
}

// CLASS: wxPaintEvent
wxClassInfo *wxPaintEvent_CLASSINFO() {
    return wxCLASSINFO(wxPaintEvent);
}

// CLASS: wxPalette
wxClassInfo *wxPalette_CLASSINFO() {
    return wxCLASSINFO(wxPalette);
}
wxPalette *wxPalette_new() {
    return new wxPalette();
}
wxPalette *wxPalette_new1(const wxPalette * palette) {
    return new wxPalette(*palette);
}
wxPalette *wxPalette_new2(int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue) {
    return new wxPalette(n, red, green, blue);
}
bool wxPalette_Create(wxPalette * self, int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue) {
    return self->Create(n, red, green, blue);
}
int wxPalette_GetColoursCount(const wxPalette * self) {
    return self->GetColoursCount();
}
bool wxPalette_GetRGB(const wxPalette * self, int pixel, unsigned char * red, unsigned char * green, unsigned char * blue) {
    return self->GetRGB(pixel, red, green, blue);
}
bool wxPalette_IsOk(const wxPalette * self) {
    return self->IsOk();
}

// CLASS: wxPanel
wxClassInfo *wxPanel_CLASSINFO() {
    return wxCLASSINFO(wxPanel);
}
wxPanel *wxPanel_new() {
    return new wxPanel();
}
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPanel(parent, id, *pos, *size, style, *name);
}
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event) {
    return self->OnSysColourChanged(*event);
}
void wxPanel_SetFocusIgnoringChildren(wxPanel * self) {
    return self->SetFocusIgnoringChildren();
}

// CLASS: wxPasswordEntryDialog
wxClassInfo *wxPasswordEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxPasswordEntryDialog);
}
wxPasswordEntryDialog *wxPasswordEntryDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * default_value, long style, const wxPoint * pos) {
    return new wxPasswordEntryDialog(parent, *message, *caption, *default_value, style, *pos);
}

// CLASS: wxPen
wxClassInfo *wxPen_CLASSINFO() {
    return wxCLASSINFO(wxPen);
}
wxPen *wxPen_new() {
    return new wxPen();
}
#if wxCHECK_VERSION(3, 1, 0)
wxPen *wxPen_new1(const wxPenInfo * info) {
    return new wxPen(*info);
}
#endif
#ifndef __WXGTK__
wxPen *wxPen_new3(const wxBitmap * stipple, int width) {
    return new wxPen(*stipple, width);
}
#endif
wxPen *wxPen_new4(const wxPen * pen) {
    return new wxPen(*pen);
}
wxColour *wxPen_GetColour(const wxPen * self) {
    return new wxColour(self->GetColour());
}
int wxPen_GetDashes(const wxPen * self, wxDash ** dashes) {
    return self->GetDashes(dashes);
}
wxBitmap * wxPen_GetStipple(const wxPen * self) {
    return self->GetStipple();
}
int wxPen_GetWidth(const wxPen * self) {
    return self->GetWidth();
}
bool wxPen_IsOk(const wxPen * self) {
    return self->IsOk();
}
bool wxPen_IsNonTransparent(const wxPen * self) {
    return self->IsNonTransparent();
}
bool wxPen_IsTransparent(const wxPen * self) {
    return self->IsTransparent();
}
void wxPen_SetColour(wxPen * self, wxColour * colour) {
    return self->SetColour(*colour);
}
void wxPen_SetDashes(wxPen * self, int n, const wxDash * dash) {
    return self->SetDashes(n, dash);
}
void wxPen_SetStipple(wxPen * self, const wxBitmap * stipple) {
    return self->SetStipple(*stipple);
}
void wxPen_SetWidth(wxPen * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxPenList
void wxPenList_delete(wxPenList *self) {
    delete self;
}
wxPenList *wxPenList_new() {
    return new wxPenList();
}

// CLASS: wxPersistenceManager
void wxPersistenceManager_delete(wxPersistenceManager *self) {
    delete self;
}
void wxPersistenceManager_Set(wxPersistenceManager * manager) {
    return wxPersistenceManager::Set(*manager);
}
wxPersistenceManager * wxPersistenceManager_Get() {
    return &(wxPersistenceManager::Get());
}
void wxPersistenceManager_DisableSaving(wxPersistenceManager * self) {
    return self->DisableSaving();
}
void wxPersistenceManager_DisableRestoring(wxPersistenceManager * self) {
    return self->DisableRestoring();
}
wxPersistentObject * wxPersistenceManager_Register1(wxPersistenceManager * self, void * obj, wxPersistentObject * po) {
    return self->Register(obj, po);
}
wxPersistentObject * wxPersistenceManager_Find(const wxPersistenceManager * self, void * obj) {
    return self->Find(obj);
}
void wxPersistenceManager_Unregister(wxPersistenceManager * self, void * obj) {
    return self->Unregister(obj);
}
void wxPersistenceManager_Save(wxPersistenceManager * self, void * obj) {
    return self->Save(obj);
}
bool wxPersistenceManager_Restore(wxPersistenceManager * self, void * obj) {
    return self->Restore(obj);
}
void wxPersistenceManager_SaveAndUnregister(wxPersistenceManager * self, void * obj) {
    return self->SaveAndUnregister(obj);
}
bool wxPersistenceManager_RegisterAndRestore1(wxPersistenceManager * self, void * obj, wxPersistentObject * po) {
    return self->RegisterAndRestore(obj, po);
}

// CLASS: wxPickerBase
wxClassInfo *wxPickerBase_CLASSINFO() {
    return wxCLASSINFO(wxPickerBase);
}
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->CreateBase(parent, id, *text, *pos, *size, style, *validator, *name);
}
int wxPickerBase_GetInternalMargin(const wxPickerBase * self) {
    return self->GetInternalMargin();
}
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self) {
    return self->GetPickerCtrlProportion();
}
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self) {
    return self->GetTextCtrl();
}
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self) {
    return self->GetPickerCtrl();
}
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self) {
    return self->GetTextCtrlProportion();
}
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self) {
    return self->HasTextCtrl();
}
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self) {
    return self->IsPickerCtrlGrowable();
}
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self) {
    return self->IsTextCtrlGrowable();
}
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin) {
    return self->SetInternalMargin(margin);
}
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow) {
    return self->SetPickerCtrlGrowable(grow);
}
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop) {
    return self->SetPickerCtrlProportion(prop);
}
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow) {
    return self->SetTextCtrlGrowable(grow);
}
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop) {
    return self->SetTextCtrlProportion(prop);
}
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text) {
    return self->SetTextCtrl(text);
}
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker) {
    return self->SetPickerCtrl(picker);
}
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self) {
    return self->UpdatePickerFromTextCtrl();
}
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self) {
    return self->UpdateTextCtrlFromPicker();
}

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self) {
    delete self;
}
bool wxPoint_IsFullySpecified(const wxPoint * self) {
    return self->IsFullySpecified();
}
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt) {
    return self->SetDefaults(*pt);
}
wxPoint *wxPoint_new() {
    return new wxPoint();
}
wxPoint *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
wxPoint *wxPoint_new2(const wxRealPoint * pt) {
    return new wxPoint(*pt);
}

// CLASS: wxPopupTransientWindow
wxClassInfo *wxPopupTransientWindow_CLASSINFO() {
    return wxCLASSINFO(wxPopupTransientWindow);
}
wxPopupTransientWindow *wxPopupTransientWindow_new() {
    return new wxPopupTransientWindow();
}
wxPopupTransientWindow *wxPopupTransientWindow_new1(wxWindow * parent, int flags) {
    return new wxPopupTransientWindow(parent, flags);
}
void wxPopupTransientWindow_Popup(wxPopupTransientWindow * self, wxWindow * focus) {
    return self->Popup(focus);
}
void wxPopupTransientWindow_Dismiss(wxPopupTransientWindow * self) {
    return self->Dismiss();
}
bool wxPopupTransientWindow_ProcessLeftDown(wxPopupTransientWindow * self, wxMouseEvent * event) {
    return self->ProcessLeftDown(*event);
}

// CLASS: wxPopupWindow
wxClassInfo *wxPopupWindow_CLASSINFO() {
    return wxCLASSINFO(wxPopupWindow);
}
wxPopupWindow *wxPopupWindow_new() {
    return new wxPopupWindow();
}
wxPopupWindow *wxPopupWindow_new1(wxWindow * parent, int flags) {
    return new wxPopupWindow(parent, flags);
}
bool wxPopupWindow_Create(wxPopupWindow * self, wxWindow * parent, int flags) {
    return self->Create(parent, flags);
}
void wxPopupWindow_Position(wxPopupWindow * self, const wxPoint * pt_origin, const wxSize * size_popup) {
    return self->Position(*pt_origin, *size_popup);
}

// CLASS: wxPreferencesEditor
void wxPreferencesEditor_delete(wxPreferencesEditor *self) {
    delete self;
}
wxPreferencesEditor *wxPreferencesEditor_new(const wxString * title) {
    return new wxPreferencesEditor(*title);
}
void wxPreferencesEditor_AddPage(wxPreferencesEditor * self, wxPreferencesPage * page) {
    return self->AddPage(page);
}
void wxPreferencesEditor_Show(wxPreferencesEditor * self, wxWindow * parent) {
    return self->Show(parent);
}
void wxPreferencesEditor_Dismiss(wxPreferencesEditor * self) {
    return self->Dismiss();
}
bool wxPreferencesEditor_ShouldApplyChangesImmediately() {
    return wxPreferencesEditor::ShouldApplyChangesImmediately();
}
bool wxPreferencesEditor_ShownModally() {
    return wxPreferencesEditor::ShownModally();
}

// CLASS: wxPreferencesPage
void wxPreferencesPage_delete(wxPreferencesPage *self) {
    delete self;
}
wxString *wxPreferencesPage_GetName(const wxPreferencesPage * self) {
    return new wxString(self->GetName());
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxPreferencesPage_GetIcon(const wxPreferencesPage * self) {
    return new wxBitmapBundle(self->GetIcon());
}
#endif
wxBitmap *wxPreferencesPage_GetLargeIcon(const wxPreferencesPage * self) {
    return new wxBitmap(self->GetLargeIcon());
}
wxWindow * wxPreferencesPage_CreateWindow(wxPreferencesPage * self, wxWindow * parent) {
    return self->CreateWindow(parent);
}

// CLASS: wxPropertySheetDialog
wxClassInfo *wxPropertySheetDialog_CLASSINFO() {
    return wxCLASSINFO(wxPropertySheetDialog);
}
wxPropertySheetDialog *wxPropertySheetDialog_new() {
    return new wxPropertySheetDialog();
}
wxPropertySheetDialog *wxPropertySheetDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPropertySheetDialog(parent, id, *title, *pos, *size, style, *name);
}
void wxPropertySheetDialog_AddBookCtrl(wxPropertySheetDialog * self, wxSizer * sizer) {
    return self->AddBookCtrl(sizer);
}
bool wxPropertySheetDialog_Create(wxPropertySheetDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxBookCtrlBase * wxPropertySheetDialog_CreateBookCtrl(wxPropertySheetDialog * self) {
    return self->CreateBookCtrl();
}
void wxPropertySheetDialog_CreateButtons(wxPropertySheetDialog * self, int flags) {
    return self->CreateButtons(flags);
}
wxBookCtrlBase * wxPropertySheetDialog_GetBookCtrl(const wxPropertySheetDialog * self) {
    return self->GetBookCtrl();
}
wxSizer * wxPropertySheetDialog_GetInnerSizer(const wxPropertySheetDialog * self) {
    return self->GetInnerSizer();
}
void wxPropertySheetDialog_SetInnerSizer(wxPropertySheetDialog * self, wxSizer * sizer) {
    return self->SetInnerSizer(sizer);
}
long wxPropertySheetDialog_GetSheetStyle(const wxPropertySheetDialog * self) {
    return self->GetSheetStyle();
}
void wxPropertySheetDialog_LayoutDialog(wxPropertySheetDialog * self, int centre_flags) {
    return self->LayoutDialog(centre_flags);
}
void wxPropertySheetDialog_SetBookCtrl(wxPropertySheetDialog * self, wxBookCtrlBase * book_ctrl) {
    return self->SetBookCtrl(book_ctrl);
}
void wxPropertySheetDialog_SetSheetStyle(wxPropertySheetDialog * self, long style) {
    return self->SetSheetStyle(style);
}
void wxPropertySheetDialog_SetSheetOuterBorder(wxPropertySheetDialog * self, int border) {
    return self->SetSheetOuterBorder(border);
}
int wxPropertySheetDialog_GetSheetOuterBorder(const wxPropertySheetDialog * self) {
    return self->GetSheetOuterBorder();
}
void wxPropertySheetDialog_SetSheetInnerBorder(wxPropertySheetDialog * self, int border) {
    return self->SetSheetInnerBorder(border);
}
int wxPropertySheetDialog_GetSheetInnerBorder(const wxPropertySheetDialog * self) {
    return self->GetSheetInnerBorder();
}

} // extern "C"

