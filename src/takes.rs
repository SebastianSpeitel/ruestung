pub trait Takes<T, const SLOT: u8 = 0> {
    type Taken;
    fn take(self, param: T) -> Self::Taken;
}

impl<T, T0: Takes<T>> Takes<T, 0> for (T0,) {
    type Taken = (T0::Taken,);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0.take(param),)
    }
}

impl<T, T0: Takes<T>, T1> Takes<T, 0> for (T0, T1) {
    type Taken = (T0::Taken, T1);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0.take(param), self.1)
    }
}

impl<T, T0, T1: Takes<T>> Takes<T, 1> for (T0, T1) {
    type Taken = (T0, T1::Taken);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1.take(param))
    }
}

impl<T, T0: Takes<T>, T1, T2> Takes<T, 0> for (T0, T1, T2) {
    type Taken = (T0::Taken, T1, T2);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0.take(param), self.1, self.2)
    }
}

impl<T, T0, T1: Takes<T>, T2> Takes<T, 1> for (T0, T1, T2) {
    type Taken = (T0, T1::Taken, T2);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1.take(param), self.2)
    }
}

impl<T, T0, T1, T2: Takes<T>> Takes<T, 2> for (T0, T1, T2) {
    type Taken = (T0, T1, T2::Taken);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2.take(param))
    }
}

impl<T, T0: Takes<T>, T1, T2, T3> Takes<T, 0> for (T0, T1, T2, T3) {
    type Taken = (T0::Taken, T1, T2, T3);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0.take(param), self.1, self.2, self.3)
    }
}

impl<T, T0, T1: Takes<T>, T2, T3> Takes<T, 1> for (T0, T1, T2, T3) {
    type Taken = (T0, T1::Taken, T2, T3);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1.take(param), self.2, self.3)
    }
}

impl<T, T0, T1, T2: Takes<T>, T3> Takes<T, 2> for (T0, T1, T2, T3) {
    type Taken = (T0, T1, T2::Taken, T3);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2.take(param), self.3)
    }
}

impl<T, T0, T1, T2, T3: Takes<T>> Takes<T, 3> for (T0, T1, T2, T3) {
    type Taken = (T0, T1, T2, T3::Taken);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2, self.3.take(param))
    }
}

impl<T, T0: Takes<T>, T1, T2, T3, T4> Takes<T, 0> for (T0, T1, T2, T3, T4) {
    type Taken = (T0::Taken, T1, T2, T3, T4);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0.take(param), self.1, self.2, self.3, self.4)
    }
}

impl<T, T0, T1: Takes<T>, T2, T3, T4> Takes<T, 1> for (T0, T1, T2, T3, T4) {
    type Taken = (T0, T1::Taken, T2, T3, T4);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1.take(param), self.2, self.3, self.4)
    }
}

impl<T, T0, T1, T2: Takes<T>, T3, T4> Takes<T, 2> for (T0, T1, T2, T3, T4) {
    type Taken = (T0, T1, T2::Taken, T3, T4);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2.take(param), self.3, self.4)
    }
}

impl<T, T0, T1, T2, T3: Takes<T>, T4> Takes<T, 3> for (T0, T1, T2, T3, T4) {
    type Taken = (T0, T1, T2, T3::Taken, T4);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2, self.3.take(param), self.4)
    }
}

impl<T, T0, T1, T2, T3, T4: Takes<T>> Takes<T, 4> for (T0, T1, T2, T3, T4) {
    type Taken = (T0, T1, T2, T3, T4::Taken);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2, self.3, self.4.take(param))
    }
}

impl<T, T0: Takes<T>, T1, T2, T3, T4, T5> Takes<T, 0> for (T0, T1, T2, T3, T4, T5) {
    type Taken = (T0::Taken, T1, T2, T3, T4, T5);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0.take(param), self.1, self.2, self.3, self.4, self.5)
    }
}

impl<T, T0, T1: Takes<T>, T2, T3, T4, T5> Takes<T, 1> for (T0, T1, T2, T3, T4, T5) {
    type Taken = (T0, T1::Taken, T2, T3, T4, T5);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1.take(param), self.2, self.3, self.4, self.5)
    }
}

impl<T, T0, T1, T2: Takes<T>, T3, T4, T5> Takes<T, 2> for (T0, T1, T2, T3, T4, T5) {
    type Taken = (T0, T1, T2::Taken, T3, T4, T5);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2.take(param), self.3, self.4, self.5)
    }
}

impl<T, T0, T1, T2, T3: Takes<T>, T4, T5> Takes<T, 3> for (T0, T1, T2, T3, T4, T5) {
    type Taken = (T0, T1, T2, T3::Taken, T4, T5);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2, self.3.take(param), self.4, self.5)
    }
}

impl<T, T0, T1, T2, T3, T4: Takes<T>, T5> Takes<T, 4> for (T0, T1, T2, T3, T4, T5) {
    type Taken = (T0, T1, T2, T3, T4::Taken, T5);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2, self.3, self.4.take(param), self.5)
    }
}

impl<T, T0, T1, T2, T3, T4, T5: Takes<T>> Takes<T, 5> for (T0, T1, T2, T3, T4, T5) {
    type Taken = (T0, T1, T2, T3, T4, T5::Taken);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (self.0, self.1, self.2, self.3, self.4, self.5.take(param))
    }
}

impl<T, T0: Takes<T>, T1, T2, T3, T4, T5, T6> Takes<T, 0> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0::Taken, T1, T2, T3, T4, T5, T6);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0.take(param),
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
        )
    }
}

impl<T, T0, T1: Takes<T>, T2, T3, T4, T5, T6> Takes<T, 1> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0, T1::Taken, T2, T3, T4, T5, T6);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0,
            self.1.take(param),
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
        )
    }
}

impl<T, T0, T1, T2: Takes<T>, T3, T4, T5, T6> Takes<T, 2> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0, T1, T2::Taken, T3, T4, T5, T6);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0,
            self.1,
            self.2.take(param),
            self.3,
            self.4,
            self.5,
            self.6,
        )
    }
}

impl<T, T0, T1, T2, T3: Takes<T>, T4, T5, T6> Takes<T, 3> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0, T1, T2, T3::Taken, T4, T5, T6);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0,
            self.1,
            self.2,
            self.3.take(param),
            self.4,
            self.5,
            self.6,
        )
    }
}

impl<T, T0, T1, T2, T3, T4: Takes<T>, T5, T6> Takes<T, 4> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0, T1, T2, T3, T4::Taken, T5, T6);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4.take(param),
            self.5,
            self.6,
        )
    }
}

impl<T, T0, T1, T2, T3, T4, T5: Takes<T>, T6> Takes<T, 5> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0, T1, T2, T3, T4, T5::Taken, T6);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5.take(param),
            self.6,
        )
    }
}

impl<T, T0, T1, T2, T3, T4, T5, T6: Takes<T>> Takes<T, 6> for (T0, T1, T2, T3, T4, T5, T6) {
    type Taken = (T0, T1, T2, T3, T4, T5, T6::Taken);
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6.take(param),
        )
    }
}
