use std::convert::TryFrom;

pub trait MaybeFrom<T>: Sized {
    fn maybe_from(value: T) -> Option<Self>;
}

impl<T, U: Sized> MaybeFrom<T> for U
where
    U: TryFrom<T>,
{
    fn maybe_from(value: T) -> Option<Self> {
        Self::try_from(value).ok()
    }
}
