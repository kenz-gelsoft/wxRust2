#include "generated.h"

extern "C" {

// CLASS: wxMDIChildFrame
wxClassInfo *wxMDIChildFrame_CLASSINFO() {
    return wxCLASSINFO(wxMDIChildFrame);
}
wxMDIChildFrame *wxMDIChildFrame_new() {
    return new wxMDIChildFrame();
}
wxMDIChildFrame *wxMDIChildFrame_new1(wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxMDIChildFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxMDIChildFrame_Activate(wxMDIChildFrame * self) {
    return self->Activate();
}
bool wxMDIChildFrame_Create(wxMDIChildFrame * self, wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxMDIParentFrame * wxMDIChildFrame_GetMDIParent(const wxMDIChildFrame * self) {
    return self->GetMDIParent();
}
void wxMDIChildFrame_Restore(wxMDIChildFrame * self) {
    return self->Restore();
}

// CLASS: wxMDIParentFrame
wxClassInfo *wxMDIParentFrame_CLASSINFO() {
    return wxCLASSINFO(wxMDIParentFrame);
}
wxMDIParentFrame *wxMDIParentFrame_new() {
    return new wxMDIParentFrame();
}
wxMDIParentFrame *wxMDIParentFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxMDIParentFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxMDIParentFrame_ActivateNext(wxMDIParentFrame * self) {
    return self->ActivateNext();
}
void wxMDIParentFrame_ActivatePrevious(wxMDIParentFrame * self) {
    return self->ActivatePrevious();
}
void wxMDIParentFrame_ArrangeIcons(wxMDIParentFrame * self) {
    return self->ArrangeIcons();
}
void wxMDIParentFrame_Cascade(wxMDIParentFrame * self) {
    return self->Cascade();
}
bool wxMDIParentFrame_Create(wxMDIParentFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxMDIChildFrame * wxMDIParentFrame_GetActiveChild(const wxMDIParentFrame * self) {
    return self->GetActiveChild();
}
wxMDIClientWindowBase * wxMDIParentFrame_GetClientWindow(const wxMDIParentFrame * self) {
    return self->GetClientWindow();
}
wxMenu * wxMDIParentFrame_GetWindowMenu(const wxMDIParentFrame * self) {
    return self->GetWindowMenu();
}
wxMDIClientWindow * wxMDIParentFrame_OnCreateClient(wxMDIParentFrame * self) {
    return self->OnCreateClient();
}
void wxMDIParentFrame_SetWindowMenu(wxMDIParentFrame * self, wxMenu * menu) {
    return self->SetWindowMenu(menu);
}
bool wxMDIParentFrame_IsTDI() {
    return wxMDIParentFrame::IsTDI();
}

// CLASS: wxMenu
wxClassInfo *wxMenu_CLASSINFO() {
    return wxCLASSINFO(wxMenu);
}
wxMenu *wxMenu_new() {
    return new wxMenu();
}
wxMenu *wxMenu_new1(long style) {
    return new wxMenu(style);
}
wxMenu *wxMenu_new2(const wxString * title, long style) {
    return new wxMenu(*title, style);
}
wxMenuItem * wxMenu_Append(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Append(id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Append1(wxMenu * self, int id, const wxString * item, wxMenu * sub_menu, const wxString * help_string) {
    return self->Append(id, *item, sub_menu, *help_string);
}
wxMenuItem * wxMenu_Append2(wxMenu * self, wxMenuItem * menu_item) {
    return self->Append(menu_item);
}
wxMenuItem * wxMenu_AppendCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help) {
    return self->AppendCheckItem(id, *item, *help);
}
wxMenuItem * wxMenu_AppendRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help) {
    return self->AppendRadioItem(id, *item, *help);
}
wxMenuItem * wxMenu_AppendSeparator(wxMenu * self) {
    return self->AppendSeparator();
}
wxMenuItem * wxMenu_AppendSubMenu(wxMenu * self, wxMenu * submenu, const wxString * text, const wxString * help) {
    return self->AppendSubMenu(submenu, *text, *help);
}
void wxMenu_Break(wxMenu * self) {
    return self->Break();
}
void wxMenu_Check(wxMenu * self, int id, bool check) {
    return self->Check(id, check);
}
bool wxMenu_Delete(wxMenu * self, int id) {
    return self->Delete(id);
}
bool wxMenu_Delete1(wxMenu * self, wxMenuItem * item) {
    return self->Delete(item);
}
bool wxMenu_Destroy(wxMenu * self, int id) {
    return self->Destroy(id);
}
bool wxMenu_Destroy1(wxMenu * self, wxMenuItem * item) {
    return self->Destroy(item);
}
void wxMenu_Enable(wxMenu * self, int id, bool enable) {
    return self->Enable(id, enable);
}
wxMenuItem * wxMenu_FindChildItem(const wxMenu * self, int id, size_t * pos) {
    return self->FindChildItem(id, pos);
}
int wxMenu_FindItem(const wxMenu * self, const wxString * item_string) {
    return self->FindItem(*item_string);
}
wxMenuItem * wxMenu_FindItem1(const wxMenu * self, int id, wxMenu ** menu) {
    return self->FindItem(id, menu);
}
wxMenuItem * wxMenu_FindItemByPosition(const wxMenu * self, size_t position) {
    return self->FindItemByPosition(position);
}
wxString *wxMenu_GetHelpString(const wxMenu * self, int id) {
    return new wxString(self->GetHelpString(id));
}
wxString *wxMenu_GetLabel(const wxMenu * self, int id) {
    return new wxString(self->GetLabel(id));
}
wxString *wxMenu_GetLabelText(const wxMenu * self, int id) {
    return new wxString(self->GetLabelText(id));
}
size_t wxMenu_GetMenuItemCount(const wxMenu * self) {
    return self->GetMenuItemCount();
}
wxString *wxMenu_GetTitle(const wxMenu * self) {
    return new wxString(self->GetTitle());
}
wxMenuItem * wxMenu_Insert(wxMenu * self, size_t pos, wxMenuItem * menu_item) {
    return self->Insert(pos, menu_item);
}
wxMenuItem * wxMenu_Insert1(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Insert(pos, id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Insert2(wxMenu * self, size_t pos, int id, const wxString * text, wxMenu * submenu, const wxString * help) {
    return self->Insert(pos, id, *text, submenu, *help);
}
wxMenuItem * wxMenu_InsertCheckItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string) {
    return self->InsertCheckItem(pos, id, *item, *help_string);
}
wxMenuItem * wxMenu_InsertRadioItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string) {
    return self->InsertRadioItem(pos, id, *item, *help_string);
}
wxMenuItem * wxMenu_InsertSeparator(wxMenu * self, size_t pos) {
    return self->InsertSeparator(pos);
}
bool wxMenu_IsChecked(const wxMenu * self, int id) {
    return self->IsChecked(id);
}
bool wxMenu_IsEnabled(const wxMenu * self, int id) {
    return self->IsEnabled(id);
}
wxMenuItem * wxMenu_Prepend(wxMenu * self, wxMenuItem * item) {
    return self->Prepend(item);
}
wxMenuItem * wxMenu_Prepend1(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Prepend(id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Prepend2(wxMenu * self, int id, const wxString * text, wxMenu * submenu, const wxString * help) {
    return self->Prepend(id, *text, submenu, *help);
}
wxMenuItem * wxMenu_PrependCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help_string) {
    return self->PrependCheckItem(id, *item, *help_string);
}
wxMenuItem * wxMenu_PrependRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help_string) {
    return self->PrependRadioItem(id, *item, *help_string);
}
wxMenuItem * wxMenu_PrependSeparator(wxMenu * self) {
    return self->PrependSeparator();
}
wxMenuItem * wxMenu_Remove(wxMenu * self, int id) {
    return self->Remove(id);
}
wxMenuItem * wxMenu_Remove1(wxMenu * self, wxMenuItem * item) {
    return self->Remove(item);
}
void wxMenu_SetHelpString(wxMenu * self, int id, const wxString * help_string) {
    return self->SetHelpString(id, *help_string);
}
void wxMenu_SetLabel(wxMenu * self, int id, const wxString * label) {
    return self->SetLabel(id, *label);
}
void wxMenu_SetTitle(wxMenu * self, const wxString * title) {
    return self->SetTitle(*title);
}
void wxMenu_UpdateUI(wxMenu * self, wxEvtHandler * source) {
    return self->UpdateUI(source);
}
void wxMenu_SetInvokingWindow(wxMenu * self, wxWindow * win) {
    return self->SetInvokingWindow(win);
}
wxWindow * wxMenu_GetInvokingWindow(const wxMenu * self) {
    return self->GetInvokingWindow();
}
wxWindow * wxMenu_GetWindow(const wxMenu * self) {
    return self->GetWindow();
}
long wxMenu_GetStyle(const wxMenu * self) {
    return self->GetStyle();
}
void wxMenu_SetParent(wxMenu * self, wxMenu * parent) {
    return self->SetParent(parent);
}
wxMenu * wxMenu_GetParent(const wxMenu * self) {
    return self->GetParent();
}
void wxMenu_Attach(wxMenu * self, wxMenuBar * menubar) {
    return self->Attach(menubar);
}
void wxMenu_Detach(wxMenu * self) {
    return self->Detach();
}
bool wxMenu_IsAttached(const wxMenu * self) {
    return self->IsAttached();
}

// CLASS: wxMenuBar
wxClassInfo *wxMenuBar_CLASSINFO() {
    return wxCLASSINFO(wxMenuBar);
}
wxMenuBar *wxMenuBar_new(long style) {
    return new wxMenuBar(style);
}
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title) {
    return self->Append(menu, *title);
}
void wxMenuBar_Check(wxMenuBar * self, int id, bool check) {
    return self->Check(id, check);
}
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable) {
    return self->Enable(id, enable);
}
bool wxMenuBar_IsEnabledTop(const wxMenuBar * self, size_t pos) {
    return self->IsEnabledTop(pos);
}
void wxMenuBar_EnableTop(wxMenuBar * self, size_t pos, bool enable) {
    return self->EnableTop(pos, enable);
}
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu) {
    return self->FindItem(id, menu);
}
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title) {
    return self->FindMenu(*title);
}
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string) {
    return self->FindMenuItem(*menu_string, *item_string);
}
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id) {
    return new wxString(self->GetHelpString(id));
}
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id) {
    return new wxString(self->GetLabel(id));
}
wxMenu * wxMenuBar_GetMenu(const wxMenuBar * self, size_t menu_index) {
    return self->GetMenu(menu_index);
}
size_t wxMenuBar_GetMenuCount(const wxMenuBar * self) {
    return self->GetMenuCount();
}
wxString *wxMenuBar_GetMenuLabel(const wxMenuBar * self, size_t pos) {
    return new wxString(self->GetMenuLabel(pos));
}
wxString *wxMenuBar_GetMenuLabelText(const wxMenuBar * self, size_t pos) {
    return new wxString(self->GetMenuLabelText(pos));
}
bool wxMenuBar_Insert(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title) {
    return self->Insert(pos, menu, *title);
}
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id) {
    return self->IsChecked(id);
}
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id) {
    return self->IsEnabled(id);
}
wxMenu * wxMenuBar_Remove(wxMenuBar * self, size_t pos) {
    return self->Remove(pos);
}
wxMenu * wxMenuBar_Replace(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title) {
    return self->Replace(pos, menu, *title);
}
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string) {
    return self->SetHelpString(id, *help_string);
}
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label) {
    return self->SetLabel(id, *label);
}
void wxMenuBar_SetMenuLabel(wxMenuBar * self, size_t pos, const wxString * label) {
    return self->SetMenuLabel(pos, *label);
}
#ifdef __WXOSX__
wxMenu * wxMenuBar_OSXGetAppleMenu(const wxMenuBar * self) {
    return self->OSXGetAppleMenu();
}
#endif
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self) {
    return self->GetFrame();
}
bool wxMenuBar_IsAttached(const wxMenuBar * self) {
    return self->IsAttached();
}
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame) {
    return self->Attach(frame);
}
void wxMenuBar_Detach(wxMenuBar * self) {
    return self->Detach();
}
#ifdef __WXOSX__
void wxMenuBar_MacSetCommonMenuBar(wxMenuBar * menubar) {
    return wxMenuBar::MacSetCommonMenuBar(menubar);
}
wxMenuBar * wxMenuBar_MacGetCommonMenuBar() {
    return wxMenuBar::MacGetCommonMenuBar();
}
#endif

// CLASS: wxMenuItem
wxClassInfo *wxMenuItem_CLASSINFO() {
    return wxCLASSINFO(wxMenuItem);
}
wxBitmap *wxMenuItem_GetBitmap(const wxMenuItem * self) {
    return new wxBitmap(self->GetBitmap());
}
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetBitmap1(const wxMenuItem * self, bool checked) {
    return new wxBitmap(self->GetBitmap(checked));
}
#endif
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxMenuItem_GetBitmapBundle(const wxMenuItem * self) {
    return new wxBitmapBundle(self->GetBitmapBundle());
}
#endif
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetDisabledBitmap(const wxMenuItem * self) {
    return new wxBitmap(self->GetDisabledBitmap());
}
#endif
wxString *wxMenuItem_GetHelp(const wxMenuItem * self) {
    return new wxString(self->GetHelp());
}
int wxMenuItem_GetId(const wxMenuItem * self) {
    return self->GetId();
}
wxString *wxMenuItem_GetItemLabel(const wxMenuItem * self) {
    return new wxString(self->GetItemLabel());
}
wxString *wxMenuItem_GetItemLabelText(const wxMenuItem * self) {
    return new wxString(self->GetItemLabelText());
}
wxItemKind wxMenuItem_GetKind(const wxMenuItem * self) {
    return self->GetKind();
}
#ifdef __WXMSW__
int wxMenuItem_GetMarginWidth(const wxMenuItem * self) {
    return self->GetMarginWidth();
}
#endif
wxMenu * wxMenuItem_GetMenu(const wxMenuItem * self) {
    return self->GetMenu();
}
wxMenu * wxMenuItem_GetSubMenu(const wxMenuItem * self) {
    return self->GetSubMenu();
}
wxAcceleratorEntry * wxMenuItem_GetAccel(const wxMenuItem * self) {
    return self->GetAccel();
}
bool wxMenuItem_IsCheck(const wxMenuItem * self) {
    return self->IsCheck();
}
bool wxMenuItem_IsCheckable(const wxMenuItem * self) {
    return self->IsCheckable();
}
bool wxMenuItem_IsChecked(const wxMenuItem * self) {
    return self->IsChecked();
}
bool wxMenuItem_IsEnabled(const wxMenuItem * self) {
    return self->IsEnabled();
}
bool wxMenuItem_IsRadio(const wxMenuItem * self) {
    return self->IsRadio();
}
bool wxMenuItem_IsSeparator(const wxMenuItem * self) {
    return self->IsSeparator();
}
bool wxMenuItem_IsSubMenu(const wxMenuItem * self) {
    return self->IsSubMenu();
}
#ifdef __WXMSW__
void wxMenuItem_SetBackgroundColour(wxMenuItem * self, const wxColour * colour) {
    return self->SetBackgroundColour(*colour);
}
#endif
void wxMenuItem_SetBitmap(wxMenuItem * self, const wxBitmapBundle * bmp) {
    return self->SetBitmap(*bmp);
}
#ifdef __WXMSW__
void wxMenuItem_SetBitmap1(wxMenuItem * self, const wxBitmapBundle * bmp, bool checked) {
    return self->SetBitmap(*bmp, checked);
}
void wxMenuItem_SetBitmaps(wxMenuItem * self, const wxBitmapBundle * checked, const wxBitmapBundle * unchecked) {
    return self->SetBitmaps(*checked, *unchecked);
}
void wxMenuItem_SetDisabledBitmap(wxMenuItem * self, const wxBitmapBundle * disabled) {
    return self->SetDisabledBitmap(*disabled);
}
void wxMenuItem_SetFont(wxMenuItem * self, const wxFont * font) {
    return self->SetFont(*font);
}
#endif
void wxMenuItem_SetHelp(wxMenuItem * self, const wxString * help_string) {
    return self->SetHelp(*help_string);
}
void wxMenuItem_SetItemLabel(wxMenuItem * self, const wxString * label) {
    return self->SetItemLabel(*label);
}
#ifdef __WXMSW__
void wxMenuItem_SetMarginWidth(wxMenuItem * self, int width) {
    return self->SetMarginWidth(width);
}
#endif
void wxMenuItem_SetMenu(wxMenuItem * self, wxMenu * menu) {
    return self->SetMenu(menu);
}
void wxMenuItem_SetSubMenu(wxMenuItem * self, wxMenu * menu) {
    return self->SetSubMenu(menu);
}
#ifdef __WXMSW__
void wxMenuItem_SetTextColour(wxMenuItem * self, const wxColour * colour) {
    return self->SetTextColour(*colour);
}
#endif
void wxMenuItem_SetAccel(wxMenuItem * self, wxAcceleratorEntry * accel) {
    return self->SetAccel(accel);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxMenuItem_AddExtraAccel(wxMenuItem * self, const wxAcceleratorEntry * accel) {
    return self->AddExtraAccel(*accel);
}
void wxMenuItem_ClearExtraAccels(wxMenuItem * self) {
    return self->ClearExtraAccels();
}
#endif
wxMenuItem *wxMenuItem_new(wxMenu * parent_menu, int id, const wxString * text, const wxString * help_string, wxItemKind kind, wxMenu * sub_menu) {
    return new wxMenuItem(parent_menu, id, *text, *help_string, kind, sub_menu);
}
void wxMenuItem_Check(wxMenuItem * self, bool check) {
    return self->Check(check);
}
void wxMenuItem_Enable(wxMenuItem * self, bool enable) {
    return self->Enable(enable);
}
wxString *wxMenuItem_GetLabelText(const wxString * text) {
    return new wxString(wxMenuItem::GetLabelText(*text));
}

} // extern "C"

