use std::str;
use syn::Ident;

pub(crate) struct UniqueIdentifierIterator {
    buffer: Vec<u8>,
}

impl UniqueIdentifierIterator {
    pub(crate) fn new() -> Self {
        UniqueIdentifierIterator {
            buffer: vec!['a' as u8],
        }
    }
}

impl Iterator for UniqueIdentifierIterator {
    type Item = Ident;

    /// Generates infinite length strings from ASCII chars a-z
    fn next(&mut self) -> Option<Self::Item> {
        let ident = Ident::new(str::from_utf8(&self.buffer).unwrap());
        let last_char = self.buffer.len() - 1;

        if self.buffer[last_char] < 'z' as u8 {
            self.buffer[last_char] += 1;
        } else {
            self.buffer.push('a' as u8);
        }

        Some(ident)
    }
}
