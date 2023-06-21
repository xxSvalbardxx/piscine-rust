pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

use std::ops::Add;

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self
    where
        T: Copy + PartialOrd + Add<Output = T>,
    {
        StepIterator {
            current: beg,
            end,
            step,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end { // <= instead of < to include the end value
            let value = self.current; // Copy the current value
            self.current = self.current + self.step; // Increment the current value
            Some(value) // Return the copied value
        } else {
            None
        }
    }
}
