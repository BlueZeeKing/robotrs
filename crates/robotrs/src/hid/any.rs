use futures::Future;
use tracing::{instrument, trace};

use super::{ReleaseTrigger, Trigger};

/// A trigger that activates when any inner trigger is activated
pub struct AnyTrigger<T> {
    inner: T,
    last_pressed: usize,
}

impl<T: AnyTriggerTarget> Trigger for AnyTrigger<T> {
    type Output = T::Output;
    type Error = T::Error;

    #[instrument(skip_all, name = "any trigger wait for trigger")]
    async fn wait_for_trigger(&mut self) -> Result<Self::Output, Self::Error> {
        let (result, last_pressed) = self.inner.wait_for_trigger().await?;
        self.last_pressed = last_pressed;
        trace!(idx = last_pressed);
        Ok(result)
    }
}

impl<T: AnyReleaseTriggerTarget> ReleaseTrigger for AnyTrigger<T> {
    #[instrument(skip_all, name = "any trigger wait for release")]
    async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
        trace!(last_pressed = self.last_pressed);
        let res = self.inner.wait_for_release(self.last_pressed).await;
        trace!("Released");
        res
    }
}

/// An extension trait that is automatically implemented for tuples [Trigger]s of length 2 to 8 where all the
/// error and output types are the same.
pub trait AnyTriggerTarget: Sized {
    type Error;
    type Output;

    #[doc(hidden)]
    fn wait_for_trigger(
        &mut self,
    ) -> impl Future<Output = Result<(Self::Output, usize), Self::Error>>;

    /// Turn this tuple into an [AnyTrigger]
    fn any(self) -> AnyTrigger<Self> {
        AnyTrigger {
            inner: self,
            last_pressed: 0,
        }
    }
}

/// An extension trait that is automatically implemented for tuples [ReleaseTrigger]s of length 2 to 8 where all the
/// error and output types are the same.
pub trait AnyReleaseTriggerTarget: AnyTriggerTarget {
    #[doc(hidden)]
    fn wait_for_release(
        &mut self,
        last_pressed: usize,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

macro_rules! impl_any {
    ($($idx:tt, $name:ident),+) => {
        impl<O, E, $($name),+> AnyTriggerTarget for ($($name),+)
        where
            $($name: Trigger<Output = O, Error = E>,)+
        {
            type Error = E;
            type Output = O;

            fn wait_for_trigger(
                &mut self,
            ) -> impl Future<Output = Result<(Self::Output, usize), Self::Error>> {
                (
                    $(async { Ok((self.$idx.wait_for_trigger().await?, $idx)) }),+
                ).race()
            }
        }

        impl<O, E, $($name),+> AnyReleaseTriggerTarget for ($($name),+)
        where
            $($name: ReleaseTrigger<Output = O, Error = E>,)+
        {
            async fn wait_for_release(&mut self, last_pressed: usize) -> Result<Self::Output, Self::Error> {
                match last_pressed {
                    $($idx => self.$idx.wait_for_release().await,)+
                    _ => self.0.wait_for_release().await,
                }
            }
        }
    };
}

#[allow(non_snake_case, non_camel_case_types)]
#[rustfmt::skip]
mod any_impls {
    use super::{Trigger, ReleaseTrigger, AnyReleaseTriggerTarget, AnyTriggerTarget};
    use futures_concurrency::future::Race;
    use std::future::Future;

    impl_any!(0, T0, 1, T1);
    impl_any!(0, T0, 1, T1, 2, T2);
    impl_any!(0, T0, 1, T1, 2, T2, 3, T3);
    impl_any!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4);
    impl_any!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5);
    impl_any!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5, 6, T6);
    impl_any!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5, 6, T6, 7, T7);
}
