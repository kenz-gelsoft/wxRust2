#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_long;

pub const fn PG_IT_CHILDREN(A: c_long) -> c_long {
    (A) << 16
}

// wxDeprecatedGUIConstants
/*  Text font families */
pub const DEFAULT: i32 = 70;
pub const DECORATIVE: i32 = 71;
pub const ROMAN: i32 = 72;
pub const SCRIPT: i32 = 73;
pub const SWISS: i32 = 74;
pub const MODERN: i32 = 75;
pub const TELETYPE: i32 = 76;

/*  Proportional or Fixed width fonts (not yet used) */
pub const VARIABLE: i32 = 80;
pub const FIXED: i32 = 81;

pub const NORMAL: i32 = 90;
pub const LIGHT: i32 = 91;
pub const BOLD: i32 = 92;
/*  Also wxNORMAL for normal (non-italic text) */
pub const ITALIC: i32 = 93;
pub const SLANT: i32 = 94;

/*  Pen styles */
pub const SOLID: i32 = 100;
pub const DOT: i32 = 101;
pub const LONG_DASH: i32 = 102;
pub const SHORT_DASH: i32 = 103;
pub const DOT_DASH: i32 = 104;
pub const USER_DASH: i32 = 105;

pub const TRANSPARENT: i32 = 106;

/*  Brush & Pen Stippling. Note that a stippled pen cannot be dashed!! */
/*  Note also that stippling a Pen IS meaningful, because a Line is */
pub const STIPPLE_MASK_OPAQUE: i32 = 107;
pub const STIPPLE_MASK: i32 = 108;
/*  drawn with a Pen, and without any Brush -- and it can be stippled. */
pub const STIPPLE: i32 = 110;

#[cfg(target_os = "windows")]
pub const FILE_SELECTOR_DEFAULT_WILDCARD_STR: &str = "*.*";
#[cfg(not(target_os = "windows"))]
pub const FILE_SELECTOR_DEFAULT_WILDCARD_STR: &str = "*";
