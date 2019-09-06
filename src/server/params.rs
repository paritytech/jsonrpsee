use crate::types;
use std::fmt;

/// Access to the parameters of a request.
#[derive(Copy, Clone)]
pub struct ServerRequestParams<'a> {
    /// Raw parameters of the request.
    params: &'a types::Params,
}

impl<'a> ServerRequestParams<'a> {
    /// Wraps around a `&types::Params` and provides utility functions for the user.
    pub(crate) fn from(params: &'a types::Params) -> Self {
        ServerRequestParams { params }
    }

    /// Returns a parameter of the request by name.
    pub fn get<'k>(self, param: impl Into<ParamKey<'k>>) -> Option<&'a types::JsonValue> {
        match (self.params, param.into()) {
            (types::Params::None, _) => None,
            (types::Params::Map(map), ParamKey::String(key)) => map.get(key),
            (types::Params::Map(_), ParamKey::Index(_)) => None,
            (types::Params::Array(_), ParamKey::String(_)) => None,
            (types::Params::Array(array), ParamKey::Index(index)) => {
                if index < array.len() {
                    Some(&array[index])
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> IntoIterator for ServerRequestParams<'a> {
    type Item = (ParamKey<'a>, &'a types::JsonValue);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(match self.params {
            types::Params::None => IterInner::Empty,
            types::Params::Array(_) => unimplemented!(),
            types::Params::Map(map) => IterInner::Map(map.iter()),
        })
    }
}

impl<'a> fmt::Debug for ServerRequestParams<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.into_iter()).finish()
    }
}

impl<'a> AsRef<types::Params> for ServerRequestParams<'a> {
    fn as_ref(&self) -> &types::Params {
        self.params
    }
}

impl<'a> Into<&'a types::Params> for ServerRequestParams<'a> {
    fn into(self) -> &'a types::Params {
        self.params
    }
}

/// Key referring to a potential parameter of a request.
pub enum ParamKey<'a> {
    /// String key. Only valid when the parameters list is a map.
    String(&'a str),
    /// Integer key. Only valid when the parameters list is an array.
    Index(usize),
}

impl<'a> From<&'a str> for ParamKey<'a> {
    fn from(s: &'a str) -> Self {
        ParamKey::String(s)
    }
}

impl<'a> From<&'a String> for ParamKey<'a> {
    fn from(s: &'a String) -> Self {
        ParamKey::String(&s[..])
    }
}

impl<'a> From<u32> for ParamKey<'a> {
    fn from(i: u32) -> Self {
        ParamKey::Index(i as usize)     // TODO: stronger check
    }
}

impl<'a> From<usize> for ParamKey<'a> {
    fn from(i: usize) -> Self {
        ParamKey::Index(i)
    }
}

impl<'a> fmt::Debug for ParamKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamKey::String(s) => fmt::Debug::fmt(s, f),
            ParamKey::Index(s) => fmt::Debug::fmt(s, f),
        }
    }
}

/// Iterator to all the parameters of a request.
pub struct Iter<'a>(IterInner<'a>);

enum IterInner<'a> {
    Empty,
    Map(serde_json::map::Iter<'a>),
}

impl<'a> Iterator for Iter<'a> {
    type Item = (ParamKey<'a>, &'a types::JsonValue);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterInner::Empty => None,
            IterInner::Map(iter) => iter.next().map(|(k, v)| (ParamKey::String(&k[..]), v)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            IterInner::Empty => (0, Some(0)),
            IterInner::Map(iter) => iter.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
}

impl<'a> fmt::Debug for Iter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ServerRequestParamsIter").finish()
    }
}
