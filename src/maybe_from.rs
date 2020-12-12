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

pub trait MaybeInto<T>: Sized {
    fn maybe_into(self) -> Option<T>;
}

impl<T: MaybeFrom<U>, U: Sized> MaybeInto<T> for U {
    fn maybe_into(self) -> Option<T> {
        T::maybe_from(self)
    }
}
