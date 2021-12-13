pub mod codes {
    //Regular Colours
    pub const _BLACK: &str = "\x1b[0;30m";
    pub const _RED: &str = "\x1b[0;31m";
    pub const _YELLOW: &str = "\x1b[0;33m";
    pub const _PURPLE: &str = "\x1b[0;35m";
    pub const _WHITE: &str = "\x1b[0;37m";
    //Bold Colours
    pub const _bBLACK: &str = "\x1b[1;30m";
    pub const _bRED: &str = "\x1b[1;31m";
    pub const _bYELLOW: &str = "\x1b[1;33m";
    pub const _bPURPLE: &str = "\x1b[1;35m";
    pub const _bWHITE: &str = "\x1b[1;37m";

    pub const _RESET: &str = "\x1b[0m";
}

pub use self::codes::_BLACK as black;
pub use self::codes::_RED as red;
pub use self::codes::_YELLOW as yellow;
pub use self::codes::_PURPLE as purple;
pub use self::codes::_WHITE as white;
pub use self::codes::_bBLACK as bold_black;
pub use self::codes::_bRED as bold_red;
pub use self::codes::_bYELLOW as bold_yellow;
pub use self::codes::_bPURPLE as bold_purple;
pub use self::codes::_bWHITE as bold_white;
pub use self::codes::_RESET as reset;