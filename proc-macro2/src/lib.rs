#![allow(unused_variables)]

use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone)]
pub struct TokenStream {}
pub struct LexError {}

impl TokenStream {
    fn _new() -> TokenStream {
        todo!()
    }

    pub fn new() -> TokenStream {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
impl Default for TokenStream {
    fn default() -> Self {
        todo!()
    }
}
impl FromStr for TokenStream {
    type Err = LexError;

    fn from_str(src: &str) -> Result<TokenStream, LexError> {
        todo!()
    }
}

impl From<TokenTree> for TokenStream {
    fn from(token: TokenTree) -> Self {
        todo!()
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, streams: I) {
        todo!()
    }
}

impl Extend<TokenStream> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, streams: I) {
        todo!()
    }
}
impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self {
        todo!()
    }
}
impl FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
        todo!()
    }
}
impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
impl Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl LexError {
    pub fn span(&self) -> Span {
        todo!()
    }
}

impl Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Error for LexError {}
#[derive(Copy, Clone)]
pub struct Span {}

impl Span {
    fn _new() -> Span {
        todo!()
    }

    pub fn call_site() -> Span {
        todo!()
    }
    pub fn resolved_at(&self, other: Span) -> Span {
        todo!()
    }
    pub fn located_at(&self, other: Span) -> Span {
        todo!()
    }
    pub fn join(&self, other: Span) -> Option<Span> {
        todo!()
    }
}
#[derive(Clone)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

impl TokenTree {
    pub fn span(&self) -> Span {
        todo!()
    }
    pub fn set_span(&mut self, span: Span) {
        todo!()
    }
}

impl From<Group> for TokenTree {
    fn from(g: Group) -> TokenTree {
        TokenTree::Group(g)
    }
}

impl From<Ident> for TokenTree {
    fn from(g: Ident) -> TokenTree {
        TokenTree::Ident(g)
    }
}

impl From<Punct> for TokenTree {
    fn from(g: Punct) -> TokenTree {
        TokenTree::Punct(g)
    }
}

impl From<Literal> for TokenTree {
    fn from(g: Literal) -> TokenTree {
        TokenTree::Literal(g)
    }
}
#[derive(Clone)]
pub struct Group {}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}

impl Group {
    fn _new() -> Self {
        todo!()
    }

    pub fn new(delimiter: Delimiter, stream: TokenStream) -> Group {
        todo!()
    }
    pub fn delimiter(&self) -> Delimiter {
        todo!()
    }
    pub fn stream(&self) -> TokenStream {
        todo!()
    }
    pub fn span(&self) -> Span {
        todo!()
    }
    pub fn span_open(&self) -> Span {
        todo!()
    }
    pub fn span_close(&self) -> Span {
        todo!()
    }
    pub fn set_span(&mut self, span: Span) {
        todo!()
    }
}
#[derive(Clone)]
pub struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Spacing {
    Alone,
    Joint,
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Punct {
        Punct {
            ch,
            spacing,
            span: Span::call_site(),
        }
    }
    pub fn as_char(&self) -> char {
        self.ch
    }
    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
#[derive(Clone)]
pub struct Ident {}

impl Ident {
    fn _new() -> Ident {
        todo!()
    }
    pub fn new(string: &str, span: Span) -> Ident {
        todo!()
    }
    fn _new_raw(string: &str, span: Span) -> Ident {
        todo!()
    }
    pub fn span(&self) -> Span {
        todo!()
    }
    pub fn set_span(&mut self, span: Span) {
        todo!()
    }
}

#[derive(Clone)]
pub struct Literal {
    inner: String,
}

impl Literal {
    fn _new(inner: String) -> Literal {
        Literal { inner }
    }

    pub fn f64_unsuffixed(f: f64) -> Literal {
        assert!(f.is_finite());
        let mut s = f.to_string();
        if !s.contains('.') {
            s.push_str(".0");
        }
        Literal::_new(s)
    }
}

impl FromStr for Literal {
    type Err = LexError;

    fn from_str(repr: &str) -> Result<Self, LexError> {
        todo!()
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
pub mod token_stream {
    use crate::TokenTree;

    pub use crate::TokenStream;
    #[derive(Clone)]
    pub struct IntoIter {}

    impl Iterator for IntoIter {
        type Item = TokenTree;

        fn next(&mut self) -> Option<TokenTree> {
            todo!()
        }
    }

    impl IntoIterator for TokenStream {
        type Item = TokenTree;
        type IntoIter = IntoIter;

        fn into_iter(self) -> IntoIter {
            todo!()
        }
    }
}
