//! Charset definitions and functions
//!
//! [Reference](http://www.destructor.de/firebird/charsets.htm)

use encoding::{all, types::EncodingRef, DecoderTrap, EncoderTrap};
use std::str;

use crate::FbError;

/// Charset definition. Used to encode/decode the
/// strings.
pub struct Charset {
    /// Charset used in firebird
    pub on_firebird: &'static str,

    /// Equivalent charset used on rust code
    pub on_rust: Option<EncodingRef>,
}

impl Charset {
    /// Decode the bytes using the current charset
    pub fn decode(&self, bytes: &[u8]) -> Result<String, FbError> {
        if let Some(charset) = self.on_rust {
            charset.decode(bytes, DecoderTrap::Strict).map_err(|e| {
                format!(
                    "Found column with an invalid {} string: {}",
                    charset.name(),
                    e
                )
                .into()
            })
        } else {
            str::from_utf8(bytes)
                .map(|str| str.to_string())
                .map_err(|e| format!("Found column with an invalid UTF-8 string: {}", e).into())
        }
    }

    // Encode the string into bytes using the current charset
    pub fn encode<S: Into<String>>(&self, str: S) -> Result<Vec<u8>, FbError> {
        if let Some(charset) = self.on_rust {
            charset
                .encode(&str.into(), EncoderTrap::Strict)
                .map_err(|e| {
                    format!(
                        "Found param with an invalid {} string: {}",
                        charset.name(),
                        e
                    )
                    .into()
                })
        } else {
            Ok(str.into().into_bytes())
        }
    }
}

impl Clone for Charset {
    fn clone(&self) -> Self {
        Self {
            on_firebird: self.on_firebird,
            on_rust: self.on_rust,
        }
    }
}

/// The default charset. Works in most cases
pub const UTF_8: Charset = Charset {
    on_firebird: "UTF8",
    on_rust: None, // Will use the std from_utf8
};

/// Western Europe. Latin 1
pub const ISO_8859_1: Charset = Charset {
    on_firebird: "ISO8859_1",
    on_rust: Some(all::ISO_8859_1),
};

/// Central Europe
pub const ISO_8859_2: Charset = Charset {
    on_firebird: "ISO8859_2",
    on_rust: Some(all::ISO_8859_2),
};

/// Southern Europe
pub const ISO_8859_3: Charset = Charset {
    on_firebird: "ISO8859_3",
    on_rust: Some(all::ISO_8859_3),
};

/// North European
pub const ISO_8859_4: Charset = Charset {
    on_firebird: "ISO8859_4",
    on_rust: Some(all::ISO_8859_4),
};

/// Cyrillic
pub const ISO_8859_5: Charset = Charset {
    on_firebird: "ISO8859_5",
    on_rust: Some(all::ISO_8859_5),
};

/// Arabic
pub const ISO_8859_6: Charset = Charset {
    on_firebird: "ISO8859_6",
    on_rust: Some(all::ISO_8859_6),
};

/// Modern Greek
pub const ISO_8859_7: Charset = Charset {
    on_firebird: "ISO8859_7",
    on_rust: Some(all::ISO_8859_7),
};

/// Baltic
pub const ISO_8859_13: Charset = Charset {
    on_firebird: "ISO8859_13",
    on_rust: Some(all::ISO_8859_13),
};

/// Central Europe
pub const WIN_1250: Charset = Charset {
    on_firebird: "WIN1250",
    on_rust: Some(all::WINDOWS_1250),
};

/// Cyrillic
pub const WIN_1251: Charset = Charset {
    on_firebird: "WIN1251",
    on_rust: Some(all::WINDOWS_1251),
};

/// Western Europe, America. Latin-1 with Windows extensions. Brazilian Portuguese
pub const WIN_1252: Charset = Charset {
    on_firebird: "WIN1252",
    on_rust: Some(all::WINDOWS_1252),
};

/// Modern Greek
pub const WIN_1253: Charset = Charset {
    on_firebird: "WIN1253",
    on_rust: Some(all::WINDOWS_1253),
};

/// Turkish
pub const WIN_1254: Charset = Charset {
    on_firebird: "WIN1254",
    on_rust: Some(all::WINDOWS_1254),
};

/// Arabic
pub const WIN_1256: Charset = Charset {
    on_firebird: "WIN1256",
    on_rust: Some(all::WINDOWS_1256),
};

/// Baltic
pub const WIN_1257: Charset = Charset {
    on_firebird: "WIN1257",
    on_rust: Some(all::WINDOWS_1257),
};

/// Vietnamese
pub const WIN_1258: Charset = Charset {
    on_firebird: "WIN1258",
    on_rust: Some(all::WINDOWS_1258),
};

/// English
pub const ASCII: Charset = Charset {
    on_firebird: "ASCII",
    on_rust: Some(all::ASCII),
};

/// Russian
pub const KOI8_R: Charset = Charset {
    on_firebird: "KOI8R",
    on_rust: Some(all::KOI8_R),
};

/// Ukrainian
pub const KOI8_U: Charset = Charset {
    on_firebird: "KOI8U",
    on_rust: Some(all::KOI8_U),
};

/// Japanese
pub const EUC_JP: Charset = Charset {
    on_firebird: "EUCJ_0208",
    on_rust: Some(all::EUC_JP),
};

/// Chinese
pub const BIG5_2003: Charset = Charset {
    on_firebird: "BIG_5",
    on_rust: Some(all::BIG5_2003),
};
