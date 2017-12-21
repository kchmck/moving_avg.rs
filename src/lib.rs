//! Moving average filter.

extern crate num_traits;

use num_traits::{Float, NumCast};

/// Computes a moving average over a ring buffer of numbers.
pub struct MovingAverage<T> {
    /// Current history of numbers.
    hist: Vec<T>,
    /// Size of the history, as T.
    size: T,
    /// Index in the history vector to replace next.
    pos: usize,
}

impl<T: Float + NumCast> MovingAverage<T> {
    /// Create a new `MovingAverage` that averages over the given amount of numbers.
    pub fn new(size: usize) -> Self {
        MovingAverage {
            hist: vec![T::zero(); size],
            size: T::from(size).unwrap(),
            pos: 0,
        }
    }

    /// Add the given number to the history, overwriting the oldest number, and return the
    /// resulting moving average.
    pub fn feed(&mut self, num: T) -> T {
        self.hist[self.pos] = num;

        self.pos += 1;
        self.pos %= self.hist.len();

        self.avg()
    }

    /// Calculate moving average based on the current history.
    fn avg(&self) -> T {
        self.hist.iter().fold(T::zero(), |s, &x| s + x) / self.size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ma() {
        let mut ma = MovingAverage::<f32>::new(10);
        assert_eq!(ma.feed(1.0), 0.1);
        assert_eq!(ma.feed(1.0), 0.2);
        assert_eq!(ma.feed(1.0), 0.3);
        assert_eq!(ma.feed(1.0), 0.4);
        assert_eq!(ma.feed(1.0), 0.5);
        assert_eq!(ma.feed(1.0), 0.6);
        assert_eq!(ma.feed(1.0), 0.7);
        assert_eq!(ma.feed(1.0), 0.8);
        assert_eq!(ma.feed(1.0), 0.9);
        assert_eq!(ma.feed(1.0), 1.0);
        assert_eq!(ma.feed(0.0), 0.9);
        assert_eq!(ma.feed(0.0), 0.8);
        assert_eq!(ma.feed(0.0), 0.7);
        assert_eq!(ma.feed(0.0), 0.6);
        assert_eq!(ma.feed(0.0), 0.5);
        assert_eq!(ma.feed(0.0), 0.4);
        assert_eq!(ma.feed(0.0), 0.3);
        assert_eq!(ma.feed(0.0), 0.2);
        assert_eq!(ma.feed(0.0), 0.1);
        assert_eq!(ma.feed(0.0), 0.0);
    }
}
