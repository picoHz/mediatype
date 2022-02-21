use super::{name::*, parse::*, value::*};

/// An iterator over the parameters.
#[derive(Debug)]
pub struct Params<'a> {
    data: &'a str,
    indices: &'a Indices,
    index: usize,
}

impl<'a> Params<'a> {
    pub(crate) fn new(data: &'a str, indices: &'a Indices) -> Self {
        Self {
            data,
            indices,
            index: 0,
        }
    }
}

impl<'a> Iterator for Params<'a> {
    type Item = (Name<'a>, Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index >= self.indices.params().len() {
            None
        } else {
            self.index += 1;
            let param = self.indices.params()[index];
            Some((
                Name(&self.data[param[0] as usize..param[1] as usize]),
                Value(&self.data[param[2] as usize..param[3] as usize]),
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.indices.params().len();
        (len, Some(len))
    }
}
