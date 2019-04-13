use futures::prelude::*;

pub enum OneOfTwo<I, E, F1: Future<Item = I, Error = E>, F2: Future<Item = I, Error = E>> {
    First(F1),
    Second(F2),
}

pub struct OneOfTwoFuture<I, E, F1: Future<Item = I, Error = E>, F2: Future<Item = I, Error = E>> {
    inner: OneOfTwo<I, E, F1, F2>,
}

impl<I, E, F1: Future<Item = I, Error = E>, F2: Future<Item = I, Error = E>>
    OneOfTwoFuture<I, E, F1, F2>
{
    pub fn new(inner: OneOfTwo<I, E, F1, F2>) -> OneOfTwoFuture<I, E, F1, F2> {
        OneOfTwoFuture { inner }
    }
}

impl<I, E, F1: Future<Item = I, Error = E>, F2: Future<Item = I, Error = E>> Future
    for OneOfTwoFuture<I, E, F1, F2>
where
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
{
    type Item = I;
    type Error = E;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.inner {
            OneOfTwo::First(fut) => fut.poll(),
            OneOfTwo::Second(fut) => fut.poll(),
        }
    }
}

pub enum OneOfTree<
    I,
    E,
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
> {
    First(F1),
    Second(F2),
    Third(F3),
}

pub struct OneOfTreeFuture<
    I,
    E,
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
> {
    inner: OneOfTree<I, E, F1, F2, F3>,
}

impl<
        I,
        E,
        F1: Future<Item = I, Error = E>,
        F2: Future<Item = I, Error = E>,
        F3: Future<Item = I, Error = E>,
    > OneOfTreeFuture<I, E, F1, F2, F3>
{
    pub fn new(inner: OneOfTree<I, E, F1, F2, F3>) -> OneOfTreeFuture<I, E, F1, F2, F3> {
        OneOfTreeFuture { inner }
    }
}

impl<I, E, F1: Future, F2: Future, F3: Future> Future for OneOfTreeFuture<I, E, F1, F2, F3>
where
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
{
    type Item = I;
    type Error = E;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.inner {
            OneOfTree::First(fut) => fut.poll(),
            OneOfTree::Second(fut) => fut.poll(),
            OneOfTree::Third(fut) => fut.poll(),
        }
    }
}

pub enum OneOfFour<
    I,
    E,
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
    F4: Future<Item = I, Error = E>,
> {
    First(F1),
    Second(F2),
    Third(F3),
    Fourth(F4),
}

pub struct OneOfFourFuture<
    I,
    E,
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
    F4: Future<Item = I, Error = E>,
> {
    inner: OneOfFour<I, E, F1, F2, F3, F4>,
}

impl<
        I,
        E,
        F1: Future<Item = I, Error = E>,
        F2: Future<Item = I, Error = E>,
        F3: Future<Item = I, Error = E>,
        F4: Future<Item = I, Error = E>,
    > OneOfFourFuture<I, E, F1, F2, F3, F4>
{
    pub fn new(inner: OneOfFour<I, E, F1, F2, F3, F4>) -> OneOfFourFuture<I, E, F1, F2, F3, F4> {
        OneOfFourFuture { inner }
    }
}

impl<I, E, F1: Future, F2: Future, F3: Future, F4: Future> Future
    for OneOfFourFuture<I, E, F1, F2, F3, F4>
where
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
    F4: Future<Item = I, Error = E>,
{
    type Item = I;
    type Error = E;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.inner {
            OneOfFour::First(fut) => fut.poll(),
            OneOfFour::Second(fut) => fut.poll(),
            OneOfFour::Third(fut) => fut.poll(),
            OneOfFour::Fourth(fut) => fut.poll(),
        }
    }
}

pub enum OneOfFive<
    I,
    E,
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
    F4: Future<Item = I, Error = E>,
    F5: Future<Item = I, Error = E>,
> {
    First(F1),
    Second(F2),
    Third(F3),
    Fourth(F4),
    Fifth(F5),
}

pub struct OneOfFiveFuture<
    I,
    E,
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
    F4: Future<Item = I, Error = E>,
    F5: Future<Item = I, Error = E>,
> {
    inner: OneOfFive<I, E, F1, F2, F3, F4, F5>,
}

impl<
        I,
        E,
        F1: Future<Item = I, Error = E>,
        F2: Future<Item = I, Error = E>,
        F3: Future<Item = I, Error = E>,
        F4: Future<Item = I, Error = E>,
        F5: Future<Item = I, Error = E>,
    > OneOfFiveFuture<I, E, F1, F2, F3, F4, F5>
{
    pub fn new(
        inner: OneOfFive<I, E, F1, F2, F3, F4, F5>,
    ) -> OneOfFiveFuture<I, E, F1, F2, F3, F4, F5> {
        OneOfFiveFuture { inner }
    }
}

impl<I, E, F1, F2, F3, F4, F5> Future for OneOfFiveFuture<I, E, F1, F2, F3, F4, F5>
where
    F1: Future<Item = I, Error = E>,
    F2: Future<Item = I, Error = E>,
    F3: Future<Item = I, Error = E>,
    F4: Future<Item = I, Error = E>,
    F5: Future<Item = I, Error = E>,
{
    type Item = I;
    type Error = E;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.inner {
            OneOfFive::First(fut) => fut.poll(),
            OneOfFive::Second(fut) => fut.poll(),
            OneOfFive::Third(fut) => fut.poll(),
            OneOfFive::Fourth(fut) => fut.poll(),
            OneOfFive::Fifth(fut) => fut.poll(),
        }
    }
}
