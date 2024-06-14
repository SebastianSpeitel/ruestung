pub trait Join: Sized {
    type Left<T>;
    type Right<T>;

    fn left<T>(self, param: T) -> Self::Left<T>;
    fn left_default<T: Default>(self) -> Self::Left<T> {
        self.left(T::default())
    }
    fn right<T>(self, param: T) -> Self::Right<T>;
    #[inline]
    fn right_default<T: Default>(self) -> Self::Right<T> {
        self.right(T::default())
    }
}

impl Join for () {
    type Left<T> = (T,);
    type Right<T> = (T,);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param,)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (param,)
    }
}
impl<T0> Join for (T0,) {
    type Left<T> = (T, T0);
    type Right<T> = (T0, T);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param, self.0)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (self.0, param)
    }
}
impl<T0, T1> Join for (T0, T1) {
    type Left<T> = (T, T0, T1);
    type Right<T> = (T0, T1, T);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param, self.0, self.1)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (self.0, self.1, param)
    }
}
impl<T0, T1, T2> Join for (T0, T1, T2) {
    type Left<T> = (T, T0, T1, T2);
    type Right<T> = (T0, T1, T2, T);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param, self.0, self.1, self.2)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (self.0, self.1, self.2, param)
    }
}
impl<T0, T1, T2, T3> Join for (T0, T1, T2, T3) {
    type Left<T> = (T, T0, T1, T2, T3);
    type Right<T> = (T0, T1, T2, T3, T);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param, self.0, self.1, self.2, self.3)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (self.0, self.1, self.2, self.3, param)
    }
}
impl<T0, T1, T2, T3, T4> Join for (T0, T1, T2, T3, T4) {
    type Left<T> = (T, T0, T1, T2, T3, T4);
    type Right<T> = (T0, T1, T2, T3, T4, T);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param, self.0, self.1, self.2, self.3, self.4)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (self.0, self.1, self.2, self.3, self.4, param)
    }
}
impl<T0, T1, T2, T3, T4, T5> Join for (T0, T1, T2, T3, T4, T5) {
    type Left<T> = (T, T0, T1, T2, T3, T4, T5);
    type Right<T> = (T0, T1, T2, T3, T4, T5, T);
    #[inline]
    fn left<T>(self, param: T) -> Self::Left<T> {
        (param, self.0, self.1, self.2, self.3, self.4, self.5)
    }
    #[inline]
    fn right<T>(self, param: T) -> Self::Right<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, param)
    }
}
