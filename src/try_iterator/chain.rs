use super::*;

enum State {
    Both,
    #[allow(unused)]
    Front,
    Back,
}

pub struct Chain<A, B> {
    a: A,
    b: B,
    state: State,
}

impl<A, B> Chain<A, B>
where
    A: TryIterator,
    B: TryIterator<Item = A::Item>,
    A::Error: From<B::Error>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        let state = State::Both;
        Self { a, b, state }
    }
}

impl<A, B> TryIterator for Chain<A, B>
where
    A: TryIterator,
    B: TryIterator<Item = A::Item>,
    A::Error: From<B::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        match self.state {
            State::Both => {
                let acc = self.a.try_fold(acc, &mut f)?;
                self.state = State::Back;
                self.b.map_err_mut(From::from).try_fold(acc, f)
            }
            State::Front => self.a.try_fold(acc, f),
            State::Back => self.b.map_err_mut(From::from).try_fold(acc, f),
        }
    }
}
