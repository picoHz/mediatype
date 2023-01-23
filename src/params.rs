use super::{name::*, parse::*, value::*};

/// An iterator over the parameters.
#[derive(Debug)]
pub struct Params<'a> {
    source: ParamsSource<'a>,
    index: usize,
}

impl<'a> Params<'a> {
    pub(crate) const fn from_slice(s: &'a [(Name<'a>, Value<'a>)]) -> Self {
        Self {
            source: ParamsSource::Slice(s),
            index: 0,
        }
    }

    pub(crate) const fn from_indices(s: &'a str, i: &'a Indices) -> Self {
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
                    Some((
                        Name::new_unchecked(&s[param[0]..param[1]]),
                        Value::new_unchecked(&s[param[2]..param[3]]),
                    ))
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
    fn params(&self) -> Params;

    /// Gets the parameter value by its name.
    ///
    /// If the same name appears more than once, returns the last value.
    fn get_param(&self, name: Name) -> Option<Value>;
}

/// A trait for mutating parameter values.
pub trait WriteParams<'a>: ReadParams {
    /// Sets a parameter value.
    ///
    /// If the parameters with the name already exist, they will be removed.
    ///
    /// ```
    /// # use mediatype::{names::*, values::*, MediaType, WriteParams};
    /// let madia_type = "text/plain; charset=UTF-8; charset=US-ASCII; format=fixed";
    ///
    /// let mut text_plain = MediaType::parse(madia_type).unwrap();
    /// text_plain.set_param(CHARSET, UTF_8);
    ///
    /// assert_eq!(
    ///     text_plain.to_string(),
    ///     "text/plain; format=fixed; charset=UTF-8"
    /// );
    /// ```
    fn set_param<'n: 'a, 'v: 'a>(&mut self, name: Name<'n>, value: Value<'v>);

    /// Removes all parameters with the name.
    fn remove_params(&mut self, name: Name);

    /// Removes all parameters.
    fn clear_params(&mut self);
}
