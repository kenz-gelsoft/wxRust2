#include "generated.h"

extern "C" {

// CLASS: wxQuantize
wxClassInfo *wxQuantize_CLASSINFO() {
    return wxCLASSINFO(wxQuantize);
}
wxQuantize *wxQuantize_new() {
    return new wxQuantize();
}
void wxQuantize_DoQuantize(unsigned int w, unsigned int h, unsigned char ** in_rows, unsigned char ** out_rows, unsigned char * palette, int desired_no_colours) {
    return wxQuantize::DoQuantize(w, h, in_rows, out_rows, palette, desired_no_colours);
}
bool wxQuantize_Quantize(const wxImage * src, wxImage * dest, wxPalette ** p_palette, int desired_no_colours, unsigned char ** eight_bit_data, int flags) {
    return wxQuantize::Quantize(*src, *dest, p_palette, desired_no_colours, eight_bit_data, flags);
}
bool wxQuantize_Quantize1(const wxImage * src, wxImage * dest, int desired_no_colours, unsigned char ** eight_bit_data, int flags) {
    return wxQuantize::Quantize(*src, *dest, desired_no_colours, eight_bit_data, flags);
}

// CLASS: wxQueryLayoutInfoEvent
wxClassInfo *wxQueryLayoutInfoEvent_CLASSINFO() {
    return wxCLASSINFO(wxQueryLayoutInfoEvent);
}
wxQueryLayoutInfoEvent *wxQueryLayoutInfoEvent_new(wxWindowID id) {
    return new wxQueryLayoutInfoEvent(id);
}
int wxQueryLayoutInfoEvent_GetFlags(const wxQueryLayoutInfoEvent * self) {
    return self->GetFlags();
}
int wxQueryLayoutInfoEvent_GetRequestedLength(const wxQueryLayoutInfoEvent * self) {
    return self->GetRequestedLength();
}
wxSize *wxQueryLayoutInfoEvent_GetSize(const wxQueryLayoutInfoEvent * self) {
    return new wxSize(self->GetSize());
}
void wxQueryLayoutInfoEvent_SetFlags(wxQueryLayoutInfoEvent * self, int flags) {
    return self->SetFlags(flags);
}
void wxQueryLayoutInfoEvent_SetRequestedLength(wxQueryLayoutInfoEvent * self, int length) {
    return self->SetRequestedLength(length);
}
void wxQueryLayoutInfoEvent_SetSize(wxQueryLayoutInfoEvent * self, const wxSize * size) {
    return self->SetSize(*size);
}

} // extern "C"

