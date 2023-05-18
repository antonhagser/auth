use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Future, FutureExt};

#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct ExtendedSelect<A, B, C> {
    inner: Option<(A, B, C)>,
}

impl<A: Unpin, B: Unpin, C: Unpin> Unpin for ExtendedSelect<A, B, C> {}

pub fn select<A, B, C>(future1: A, future2: B, future3: C) -> ExtendedSelect<A, B, C>
where
    A: Future + Unpin,
    B: Future + Unpin,
    C: Future + Unpin,
{
    assert_future::<Option<C::Output>, _>(ExtendedSelect {
        inner: Some((future1, future2, future3)),
    })
}

impl<A, B, C> Future for ExtendedSelect<A, B, C>
where
    A: Future + Unpin,
    B: Future + Unpin,
    C: Future + Unpin,
{
    type Output = Option<C::Output>;

    // Returns if any of the futures exits, if the last future (future3: C) exits
    // the function returns Some(C::Output)
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (mut a, mut b, mut c) = self.inner.take().expect("cannot poll ExtendedSelect twice");
        match a.poll_unpin(cx) {
            Poll::Ready(_) => Poll::Ready(None),
            Poll::Pending => match b.poll_unpin(cx) {
                Poll::Ready(_) => Poll::Ready(None),
                Poll::Pending => match c.poll_unpin(cx) {
                    Poll::Ready(x) => Poll::Ready(Some(x)),
                    Poll::Pending => {
                        self.inner = Some((a, b, c));
                        Poll::Pending
                    }
                },
            },
        }
    }
}

pub(crate) fn assert_future<T, F>(future: F) -> F
where
    F: Future<Output = T>,
{
    future
}
