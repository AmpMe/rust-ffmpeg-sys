mod format;
pub use self::format::*;

mod io;
pub use self::io::*;

#[cfg_attr(feature = "static", link(name = "avformat_ampme", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "avformat_ampme"))]
extern { }
