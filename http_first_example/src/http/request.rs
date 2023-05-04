use super::method::{Method, MethodError};
use super::QueryParam;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_params: Option<QueryParam<'buf>>,
    method: Method,
}
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_params(&self) -> Option<&QueryParam> {
        self.query_params.as_ref()
    }
}
/*
 * When we define `'a` that means to `struct Request` depends from another variable or value as lifetime
 **/

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_params = None;
        if let Some(i) = path.find('?') {
            query_params = Some(QueryParam::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path: path,
            query_params: query_params,
            method: method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, character) in request.chars().enumerate() {
        if character == ' ' || character == '\r' {
            return Some((&request[..index], &request[(index + 1)..]));
        }
    }
    return None;
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}", self.message());
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}", self.message());
    }
}

impl Error for ParseError {}
