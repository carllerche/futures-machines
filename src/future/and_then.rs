use func::MapOnce;

use futures::{Future, IntoFuture, Poll};
use super::chain::Chain;

/// Chain a computation onto the end of another future which completes
/// successfully.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct AndThen<A, B, F>
where A: Future,
      B: IntoFuture
{
    state: Chain<A, B::Future, F>,
}

impl<A, B, F> AndThen<A, B, F>
where A: Future,
      B: IntoFuture,
      F: MapOnce<A::Item, B>,
{
    /// Return a new `AndThen`
    pub fn new(future: A, f: F) -> Self {
        AndThen {
            state: Chain::new(future, f),
        }
    }
}

impl<A, B, F> Future for AndThen<A, B, F>
    where A: Future,
          B: IntoFuture<Error=A::Error>,
          F: MapOnce<A::Item, B>,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<B::Item, B::Error> {
        self.state.poll(|result, f| {
            result.map(|e| {
                Err(f.call(e).into_future())
            })
        })
    }
}
