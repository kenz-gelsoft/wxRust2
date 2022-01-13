#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub const fn wxPG_IT_CHILDREN(A: u32) -> u32 {
    (A) << 16
}

// wxDeprecatedGUIConstants
/*  Text font families */
pub const wxDEFAULT: u32 = 70;
pub const wxDECORATIVE: u32 = 71;
pub const wxROMAN: u32 = 72;
pub const wxSCRIPT: u32 = 73;
pub const wxSWISS: u32 = 74;
pub const wxMODERN: u32 = 75;
pub const wxTELETYPE: u32 = 76;

/*  Proportional or Fixed width fonts (not yet used) */
pub const wxVARIABLE: u32 = 80;
pub const wxFIXED: u32 = 81;

pub const wxNORMAL: u32 = 90;
pub const wxLIGHT: u32 = 91;
pub const wxBOLD: u32 = 92;
/*  Also wxNORMAL for normal (non-italic text) */
pub const wxITALIC: u32 = 93;
pub const wxSLANT: u32 = 94;

/*  Pen styles */
pub const wxSOLID: u32 = 100;
pub const wxDOT: u32 = 101;
pub const wxLONG_DASH: u32 = 102;
pub const wxSHORT_DASH: u32 = 103;
pub const wxDOT_DASH: u32 = 104;
pub const wxUSER_DASH: u32 = 105;

pub const wxTRANSPARENT: u32 = 106;

/*  Brush & Pen Stippling. Note that a stippled pen cannot be dashed!! */
/*  Note also that stippling a Pen IS meaningful, because a Line is */
pub const wxSTIPPLE_MASK_OPAQUE: u32 = 107;
pub const wxSTIPPLE_MASK: u32 = 108;
/*  drawn with a Pen, and without any Brush -- and it can be stippled. */
pub const wxSTIPPLE: u32 = 110;
