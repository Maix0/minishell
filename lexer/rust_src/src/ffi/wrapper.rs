#![allow(dead_code)]
use super::ffi;
use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, Index, IndexMut},
};

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    LParens = ffi::e_token_type_LPARENS,
    RParens = ffi::e_token_type_RPARENS,
    LBracket = ffi::e_token_type_LBRACKET,
    RBracket = ffi::e_token_type_RBRACKET,
    LCurly = ffi::e_token_type_LCURLY,
    RCurly = ffi::e_token_type_RCURLY,
    DqString = ffi::e_token_type_DQ_STRING,
    SqString = ffi::e_token_type_SQ_STRING,
    NqString = ffi::e_token_type_NQ_STRING,
    BqString = ffi::e_token_type_BQ_STRING,
    DolarSign = ffi::e_token_type_DOLAR_SIGN,
    LessSign = ffi::e_token_type_LESS_SIGN,
    GreatSign = ffi::e_token_type_GREAT_SIGN,
    PipeSign = ffi::e_token_type_PIPE_SIGN,
    AndSign = ffi::e_token_type_AND_SIGN,
    SemiSign = ffi::e_token_type_SEMI_SIGN,
    Newline = ffi::e_token_type_NEWLINE,
}

impl TokenKind {
    fn into_c(self) -> ffi::t_token_type {
        self as _
    }
    fn from_c(c: ffi::t_token_type) -> Option<Self>  {
        match c {
            ffi::e_token_type_LPARENS => Some(Self::LParens),
            ffi::e_token_type_RPARENS => Some(Self::RParens),
            ffi::e_token_type_LBRACKET => Some(Self::LBracket),
            ffi::e_token_type_RBRACKET => Some(Self::RBracket),
            ffi::e_token_type_LCURLY => Some(Self::LCurly),
            ffi::e_token_type_RCURLY => Some(Self::RCurly),
            ffi::e_token_type_DQ_STRING => Some(Self::DqString),
            ffi::e_token_type_SQ_STRING => Some(Self::SqString),
            ffi::e_token_type_NQ_STRING => Some(Self::NqString),
            ffi::e_token_type_BQ_STRING => Some(Self::BqString),
            ffi::e_token_type_DOLAR_SIGN => Some(Self::DolarSign),
            ffi::e_token_type_LESS_SIGN => Some(Self::LessSign),
            ffi::e_token_type_GREAT_SIGN => Some(Self::GreatSign),
            ffi::e_token_type_PIPE_SIGN => Some(Self::PipeSign),
            ffi::e_token_type_AND_SIGN => Some(Self::AndSign),
            ffi::e_token_type_SEMI_SIGN => Some(Self::SemiSign),
            ffi::e_token_type_NEWLINE => Some(Self::Newline),
            _ => None
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Token(ffi::s_token);

impl Token {
    pub fn new(kind: TokenKind, s: Option<&std::ffi::CStr>, start: usize, len: usize) -> Self {
        Self(
            ffi::s_token {
                input: s.map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
                start,
                end: (start+len).clamp(0, s.map(|s| s.to_bytes().len()).unwrap_or(0)),
                type_: kind as ffi::t_token_type,
            }
        )
    }
    pub fn kind(&self) -> TokenKind {
        TokenKind::from_c(self.0.type_).unwrap()
    }
    pub fn start(&self) -> usize {
        self.0.start
    }

    pub fn end(&self) -> usize {
        self.0.end
    }

    pub fn input(&self) -> Option<&std::ffi::CStr>
    {
        if self.0.input.is_null()
        {
            return None;
        }
        unsafe {
            Some(std::ffi::CStr::from_ptr(self.0.input))
        }
    }
}

#[repr(u8)]
pub enum ErrorRespub  {
    Error = ffi::ERROR as u8,
    NoError = ffi::NO_ERROR as u8,
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TokenVec(ffi::t_vec_token);



impl Drop for TokenVec {
    fn drop(&mut self) {
        unsafe {
            let s: ManuallyDrop<TokenVec> = ManuallyDrop::new(std::ptr::read(self as _));
            ffi::vec_token_free(s.deref().0);
        }
    }
}

impl TokenVec {
    pub fn push(&mut self, element: Token) {
        unsafe {
            if ffi::vec_token_push(&mut self.0 as *mut _, element.0) {
                panic!("push failed !");
            }
        }
    }
    pub fn push_front(&mut self, element: Token) {
        unsafe {
            if ffi::vec_token_push_front(&mut self.0 as *mut _, element.0) {
                panic!("push_front failed !");
            }
        }
    }



    pub fn pop(&mut self) -> Option<Token> {
        unsafe {
            let mut element: MaybeUninit<Token> = MaybeUninit::uninit();

            if ffi::vec_token_pop(&mut self.0 as *mut _, element.as_mut_ptr() as _) {
                Some(element.assume_init())
            } else {
                None
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<Token> {
        unsafe {
            let mut element: MaybeUninit<Token> = MaybeUninit::uninit();

            if ffi::vec_token_pop_front(&mut self.0 as *mut _, element.as_mut_ptr() as _) {
                Some(element.assume_init())
            } else {
                None
            }
        }
    }

    pub fn reserve(&mut self, wanted: usize) {
        unsafe {
            if ffi::vec_token_reserve(&mut self.0 as *mut _, wanted) {
                panic!("reserve failed !");
            }
        }
    }

    pub fn reverse(&mut self) {
        unsafe { ffi::vec_token_reverse(&mut self.0 as *mut _) }
    }
    pub fn new(capacity: usize) -> Self {
        unsafe { Self(ffi::vec_token_new(capacity, None)) }
    }
}
impl TokenVec {
    pub fn len(&self) -> usize {
        self.0.len
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&*(self.0.buffer.wrapping_add(index) as *const Token)) }
    }
    pub fn get_mut(&self, index: usize) -> Option<&mut Token> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&mut *(self.0.buffer.wrapping_add(index) as *mut Token)) }
    }
}

impl Index<usize> for TokenVec {
    type Output = Token;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl IndexMut<usize> for TokenVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
