use futures::Future;

use super::{ReleaseTrigger, Trigger};

/// A trigger that activates if all the inner triggers are activated. Releases when any the inner
/// triggers are released
///
/// # Example
///
/// ```rust
/// (xbox_controller.left_bumper(), xbox_controller.right_bumper()).all().on_pressed(|| async {
///     println!("hello, world!");
/// });
/// ```
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

impl<T: AllTriggerTarget> ReleaseTrigger for AllTrigger<T> {
    async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
        self.inner.wait_for_release().await
    }
}

/// An extension trait that is implemented on tuples of triggers that have between 2 and 8 items.
/// All error types must be the same for all triggers.
pub trait AllTriggerTarget: Sized {
    type Error;
    type Output;

    #[doc(hidden)]
    fn wait_for_trigger(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;

    #[doc(hidden)]
    fn wait_for_release(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;

    /// Create a trigger that activates when all triggers are active
    fn all(self) -> AllTrigger<Self> {
        AllTrigger { inner: self }
    }
}

macro_rules! impl_all {
    ($($idx:tt, $name:ident),+) => {
        impl<E, $($name),+> AllTriggerTarget for ($($name),+)
        where
            $($name: ReleaseTrigger<Error = E>,)+
        {
            type Error = E;
            type Output = ();

            async fn wait_for_trigger(
                &mut self,
            ) -> Result<Self::Output, Self::Error> {
                let mut held = ($({
                    $idx;
                    false
                }),+);

                loop {
                    (
                        $(async {
                            if !held.$idx {
                                self.$idx.wait_for_trigger().await?;

                                held.$idx = true;
                            } else {
                                self.$idx.wait_for_release().await?;

                                held.$idx = false;
                            }

                            Ok(())
                        }),+
                    ).race().await?;

                    if $(held.$idx)&&+ {
                        return Ok(());
                    }
                }
            }

            async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
                (
                    $(async {
                        self.$idx.wait_for_release().await?;
                        Ok(())
                    }),+
                )
                    .race()
                    .await
            }
        }
    };
}

#[allow(non_snake_case, non_camel_case_types)]
#[rustfmt::skip]
mod all_impls {
    use super::{ReleaseTrigger, AllTriggerTarget};
    use futures_concurrency::future::Race;

    impl_all!(0, T0, 1, T1);
    impl_all!(0, T0, 1, T1, 2, T2);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5, 6, T6);
    impl_all!(0, T0, 1, T1, 2, T2, 3, T3, 4, T4, 5, T5, 6, T6, 7, T7);
}
