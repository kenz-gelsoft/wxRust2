#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_long;

// TODO auto generate
pub const wxRUST_EVT_BUTTON: i32 = 0;
pub const wxRUST_EVT_MENU: i32 = 0;

pub const fn wxPG_IT_CHILDREN(A: c_long) -> c_long {
    (A) << 16
}

// wxDeprecatedGUIConstants
/*  Text font families */
pub const wxDEFAULT: i32 = 70;
pub const wxDECORATIVE: i32 = 71;
pub const wxROMAN: i32 = 72;
pub const wxSCRIPT: i32 = 73;
pub const wxSWISS: i32 = 74;
pub const wxMODERN: i32 = 75;
pub const wxTELETYPE: i32 = 76;

/*  Proportional or Fixed width fonts (not yet used) */
pub const wxVARIABLE: i32 = 80;
pub const wxFIXED: i32 = 81;

pub const wxNORMAL: i32 = 90;
pub const wxLIGHT: i32 = 91;
pub const wxBOLD: i32 = 92;
/*  Also wxNORMAL for normal (non-italic text) */
pub const wxITALIC: i32 = 93;
pub const wxSLANT: i32 = 94;

/*  Pen styles */
pub const wxSOLID: i32 = 100;
pub const wxDOT: i32 = 101;
pub const wxLONG_DASH: i32 = 102;
pub const wxSHORT_DASH: i32 = 103;
pub const wxDOT_DASH: i32 = 104;
pub const wxUSER_DASH: i32 = 105;

pub const wxTRANSPARENT: i32 = 106;

/*  Brush & Pen Stippling. Note that a stippled pen cannot be dashed!! */
/*  Note also that stippling a Pen IS meaningful, because a Line is */
pub const wxSTIPPLE_MASK_OPAQUE: i32 = 107;
pub const wxSTIPPLE_MASK: i32 = 108;
/*  drawn with a Pen, and without any Brush -- and it can be stippled. */
pub const wxSTIPPLE: i32 = 110;
