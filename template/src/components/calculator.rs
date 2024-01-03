use std::ops::Rem;

pub fn is_divisible_3<T>(x: T) -> bool
    where
        T: Rem<Output = T> + Default + PartialEq + From<u8> + Copy,
{
    let three: T = 3.into();
    x % three == T::default()
}

pub fn is_divisible_x<T>(left: T, right: T) -> bool
    where
        T: Rem<Output = T> + Default + PartialEq + From<u8> + Copy,
{
    left % right == T::default()
}

pub trait Divisible<T>: Sized {
    #[must_use]
    fn divisible(self, x: T) -> bool;
}

// Only works if no type annotations are present
impl<T> Divisible<T> for T
    where
        <T as Rem>::Output: PartialEq<T>,
        T: Rem + Default,
{
    fn divisible(self, x: T) -> bool {
        self % x == T::default()
    }
}
