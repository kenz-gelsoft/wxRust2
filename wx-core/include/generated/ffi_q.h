#pragma once

#include <wx/laywin.h>
#include <wx/quantize.h>

extern "C" {

// CLASS: wxQuantize
wxClassInfo *wxQuantize_CLASSINFO();
wxQuantize *wxQuantize_new();
void wxQuantize_DoQuantize(unsigned int w, unsigned int h, unsigned char ** in_rows, unsigned char ** out_rows, unsigned char * palette, int desired_no_colours);
bool wxQuantize_Quantize(const wxImage * src, wxImage * dest, wxPalette ** p_palette, int desired_no_colours, unsigned char ** eight_bit_data, int flags);
bool wxQuantize_Quantize1(const wxImage * src, wxImage * dest, int desired_no_colours, unsigned char ** eight_bit_data, int flags);

// CLASS: wxQueryLayoutInfoEvent
wxClassInfo *wxQueryLayoutInfoEvent_CLASSINFO();
wxQueryLayoutInfoEvent *wxQueryLayoutInfoEvent_new(wxWindowID id);
int wxQueryLayoutInfoEvent_GetFlags(const wxQueryLayoutInfoEvent * self);
int wxQueryLayoutInfoEvent_GetRequestedLength(const wxQueryLayoutInfoEvent * self);
wxSize *wxQueryLayoutInfoEvent_GetSize(const wxQueryLayoutInfoEvent * self);
void wxQueryLayoutInfoEvent_SetFlags(wxQueryLayoutInfoEvent * self, int flags);
void wxQueryLayoutInfoEvent_SetRequestedLength(wxQueryLayoutInfoEvent * self, int length);
void wxQueryLayoutInfoEvent_SetSize(wxQueryLayoutInfoEvent * self, const wxSize * size);

} // extern "C"

