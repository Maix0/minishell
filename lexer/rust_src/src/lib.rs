mod ffi;
pub use ffi::wrapper::*;
use std::{ffi::CStr, mem::MaybeUninit};
use ErrorRes::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum WordKind {
    SingleQuote,
    DoubleQuote,
    NoQuote,
}

#[no_mangle]
pub extern "C" fn tokenize(
    s: *const std::ffi::c_char,
    out_ptr: &mut MaybeUninit<TokenVec>,
) -> ErrorRes {
    if s.is_null() {
        return Error;
    }
    let cstr = unsafe { CStr::from_ptr(s) };
    let Ok(s) = cstr.to_str() else { return Error };
    let out = TokenVec::new(64);
    let mut word_kind = WordKind::NoQuote;
    let current_word_start = 0; 
    let mut last_was_backslash = false;
    let mut iter = s.bytes().enumerate().peekable();
    while let Some((idx, chr)) = iter.next() {
        match word_kind {
            WordKind::NoQuote => {
                match chr {
                    b'\\' => {
                        last_was_backslash = true;
                    }
                    b' ' | b'\n' | b'\x0C' |  b'\r' | b'\x0B'  => {
                        // Found a space, new need to check if it is a escaped one or not
                        if last_was_backslash {
                            // yes it was, just skip
                            continue;
                        }
                        else {
                            out.push(Token::new(TokenKind::NqString, Some(cstr), current_word_start, idx - current_word_start));
                            current_word_start = idx;
                            last_was_backslash = false;
                        }
                    }
                }
            }
            WordKind::SingleQuote => {
                
            }
            WordKind::DoubleQuote => {
                
            }
        }

    }
    *out_ptr = MaybeUninit::new(out);
    NoError
}
