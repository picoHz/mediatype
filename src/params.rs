use super::{name::*, parse::*, value::*};

/// An iterator over the parameters.
#[derive(Debug)]
pub struct Params<'a> {
    source: ParamsSource<'a>,
    index: usize,
}

impl<'a> Params<'a> {
    pub(crate) fn from_slice(s: &'a [(Name<'a>, Value<'a>)]) -> Self {
        Self {
            source: ParamsSource::Slice(s),
            index: 0,
        }
    }

    pub(crate) fn from_indices(s: &'a str, i: &'a Indices) -> Self {
        Self {
            source: ParamsSource::Indices(s, i),
            index: 0,
        }
    }
}

#[derive(Debug)]
enum ParamsSource<'a> {
    Slice(&'a [(Name<'a>, Value<'a>)]),
    Indices(&'a str, &'a Indices),
}

impl<'a> Iterator for Params<'a> {
    type Item = (Name<'a>, Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        match self.source {
            ParamsSource::Slice(s) => {
                if index >= s.len() {
                    None
                } else {
                    self.index += 1;
                    Some((s[index].0, s[index].1))
                }
            }
            ParamsSource::Indices(s, i) => {
                if index >= i.params().len() {
                    None
                } else {
                    self.index += 1;
                    let param = i.params()[index];
                    let name = &s[param[0] as usize..param[1] as usize];
                    let value = &s[param[2] as usize..param[3] as usize];
                    Some((Name::new_unchecked(name), Value::new_unchecked(value)))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self.source {
            ParamsSource::Slice(s) => s.len(),
            ParamsSource::Indices(_, i) => i.params().len(),
        };
        (len, Some(len))
    }
}

/// A trait for getting parameter values.
pub trait ReadParams {
    /// Returns the parameters.
    ///
    /// The parameters are alphabetically sorted by their keys.
    fn params(&self) -> Params;

    /// Gets the parameter value by its key.
    fn get_param(&self, key: Name) -> Option<Value>;
}

/// A trait for mutating parameter values.
pub trait WriteParams<'a>: ReadParams {
    /// Sets a parameter value.
    ///
    /// If the parameter is already set, replaces it with a new value and
    /// returns the old value.
    fn set_param<'k: 'a, 'v: 'a>(&mut self, key: Name<'k>, value: Value<'v>) -> Option<Value>;

    /// Removes and returns a parameter value by its key.
    fn remove_param(&mut self, key: Name) -> Option<Value>;

    /// Removes all parameters.
    fn clear_params(&mut self);
}
