use crate::Integer;

/// TODO
pub trait ILog: Integer + From<u8> {
    /// TODO
    fn checked_ilog2(&self) -> Option<u64>;

    /// TODO
    fn checked_ilog(&self, base: &Self) -> Option<u64>;

    /// TODO
    #[inline]
    fn checked_ilog10(&self) -> Option<u64> {
        self.checked_ilog(&Self::from(10u8))
    }

    /// TODO
    #[inline]
    fn ilog2(&self) -> u64 {
        self.checked_ilog2().unwrap()
    }

    /// TODO
    #[inline]
    fn ilog(&self, base: &Self) -> u64 {
        self.checked_ilog(base).unwrap()
    }

    /// TODO
    #[inline]
    fn ilog10(&self) -> u64 {
        self.checked_ilog10().unwrap()
    }
}

/// Returns the truncated principal square root of an integer --
#[inline]
pub fn checked_ilog2<T: ILog>(x: &T) -> Option<u64> {
    x.checked_ilog2()
}

/// Returns the truncated principal cube root of an integer --
#[inline]
pub fn checked_ilog<T: ILog>(x: &T, base: &T) -> Option<u64> {
    x.checked_ilog(base)
}

/// Returns the truncated principal `n`th root of an integer --
#[inline]
pub fn checked_ilog10<T: ILog>(x: &T) -> Option<u64> {
    x.checked_ilog10()
}

/// Returns the truncated principal square root of an integer --
#[inline]
pub fn ilog2<T: ILog>(x: &T) -> u64 {
    x.ilog2()
}

/// Returns the truncated principal cube root of an integer --
#[inline]
pub fn ilog<T: ILog>(x: &T, base: &T) -> u64 {
    x.ilog(base)
}

/// Returns the truncated principal `n`th root of an integer --
#[inline]
pub fn ilog10<T: ILog>(x: &T) -> u64 {
    x.ilog10()
}
