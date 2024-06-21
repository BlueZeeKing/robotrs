use futures::Future;

use super::{ReleaseTrigger, Trigger};

/// A trigger that activates if all the inner triggers are activated. Releases when all the inner
/// triggers are released
pub struct AllTrigger<T> {
    inner: T,
}

impl<T: AllTriggerTarget> Trigger for AllTrigger<T> {
    type Output = T::Output;
    type Error = T::Error;

    async fn wait_for_trigger(&mut self) -> Result<Self::Output, Self::Error> {
        self.inner.wait_for_trigger().await
    }
}

impl<T: AllReleaseTriggerTarget> ReleaseTrigger for AllTrigger<T> {
    async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
        self.inner.wait_for_release().await
    }
}

/// An extension trait that is implemented on tuples of triggers that have between 2 and 8 items.
/// All output and error types must be the same for all triggers.
pub trait AllTriggerTarget: Sized {
    type Error;
    type Output;

    #[doc(hidden)]
    fn wait_for_trigger(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;

    /// Create a trigger that activates when all triggers are active
    fn all(self) -> AllTrigger<Self> {
        AllTrigger { inner: self }
    }
}

/// An extension trait that is implemented on tuples of release triggers that have between 2 and 8 items.
/// All output and error types must be the same for all triggers.
pub trait AllReleaseTriggerTarget: AllTriggerTarget {
    #[doc(hidden)]
    fn wait_for_release(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

macro_rules! impl_all {
    ($($idx:tt, $name:ident),+) => {
        impl<E, $($name),+> AllTriggerTarget for ($($name),+)
        where
            $($name: Trigger<Error = E>,)+
        {
            type Error = E;
            type Output = ($($name::Output),+);

            async fn wait_for_trigger(
                &mut self,
            ) -> Result<Self::Output, Self::Error> {
                let res = (
                    $(self.$idx.wait_for_trigger()),+
                ).join().await;

                Ok(($(res.$idx?),+))
            }
        }

        impl<O, E, $($name),+> AllReleaseTriggerTarget for ($($name),+)
        where
            $($name: ReleaseTrigger<Output = O, Error = E>,)+
        {
            async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
                let res = (
                    $(self.$idx.wait_for_trigger()),+
                ).join().await;

                Ok(($(res.$idx?),+))
            }
        }
    };
}

#[allow(non_snake_case, non_camel_case_types)]
#[rustfmt::skip]
mod all_impls {
    use super::{Trigger, ReleaseTrigger, AllReleaseTriggerTarget, AllTriggerTarget};
    use futures_concurrency::future::Join;

    impl_all!(0, T0, 1, T1);
    impl_all!(0, T0, 1, T1, 2, T2);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5, 6, T6);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5, 6, T6, 7, T7);
}
