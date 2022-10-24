/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a, Key> {
    key: &'a Key
}

impl<'a, Key> Xorcism<'a, Key> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new(key: &'a Key) -> Self 
    where Key: Iterator<Item = u8>
    {
        Self{key}
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        unimplemented!()
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> {
        unimplemented!();
        // this empty iterator silences a compiler complaint that
        // () doesn't implement ExactSizeIterator
        std::iter::empty()
    }
}