use std::{error::Error, fmt::Debug, time::Duration};

use futures::Future;
use robotrs::time::delay;

/// Retry a function a given number of times, using exponential backoff.
pub async fn retry<Func, Fut, E, O>(func: Func, retries: u32) -> Result<O, E>
where
    Func: Fn() -> Fut,
    Fut: Future<Output = Result<O, E>>,
    E: Error,
{
    let mut failures = 0;
    loop {
        match func().await {
            Ok(value) => break Ok(value),
            Err(err) => {
                failures += 1;
                if failures > retries {
                    tracing::error!(
                        "Error occured on {}/{} try, max retries reached: {}",
                        failures,
                        retries,
                        err
                    );
                    break Err(err);
                }
                tracing::error!("Error occured on {}/{} try: {}", failures, retries, err);

                delay(Duration::from_millis(50) * 2u32.pow(failures - 1)).await;
            }
        }
    }
}

/// Log the result of a future, returning the result.
pub async fn log<Fut, E, O>(fut: Fut) -> Result<O, E>
where
    Fut: Future<Output = Result<O, E>>,
    E: Debug,
{
    let res = fut.await;

    if let Err(err) = &res {
        tracing::error!("Encountered an error: {:?}", err);
    }

    res
}
