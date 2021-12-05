/// Line path type meant for iteration from one usize value to another.
/// Effectively similar to Range type, except that Range cannot iterator
/// over a descending range, whereas Vector can iterate in either direction.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vector {
    start: usize,
    end: usize,
}

impl Vector {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn iter(&self) -> VectorIterator {
        VectorIterator {
            start: self.start,
            end: self.end,
            current: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VectorIterator {
    start: usize,
    end: usize,
    current: Option<usize>,
}

impl Iterator for VectorIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                self.current = Some(self.start);
                self.current
            }

            Some(current) if current == self.end => None,

            Some(current) => {
                self.current = Some(if self.start <= self.end {
                    current + 1
                } else {
                    current - 1
                });
                self.current
            }
        }
    }
}
