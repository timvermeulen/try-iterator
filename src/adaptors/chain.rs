use super::*;

enum State {
    Both,
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.state {
            State::Both => size_hint::add(self.a.size_hint(), self.b.size_hint()),
            State::Front => self.a.size_hint(),
            State::Back => self.b.size_hint(),
        }
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        try {
            match self.state {
                State::Both => match self.a.try_nth(n)? {
                    Ok(x) => Ok(x),
                    Err(n) => {
                        self.state = State::Back;
                        self.b.try_nth(n)?
                    }
                },
                State::Front => self.a.try_nth(n)?,
                State::Back => self.b.try_nth(n)?,
            }
        }
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

impl<A, B> DoubleEndedTryIterator for Chain<A, B>
where
    A: DoubleEndedTryIterator,
    B: DoubleEndedTryIterator<Item = A::Item>,
    A::Error: From<B::Error>,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        try {
            match self.state {
                State::Both => match self.b.try_nth_back(n)? {
                    Ok(x) => Ok(x),
                    Err(n) => {
                        self.state = State::Front;
                        self.a.try_nth_back(n)?
                    }
                },
                State::Front => self.a.try_nth_back(n)?,
                State::Back => self.b.try_nth_back(n)?,
            }
        }
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        match self.state {
            State::Both => {
                let acc = self.b.map_err_mut(From::from).try_rfold(acc, &mut f)?;
                self.state = State::Front;
                self.a.try_rfold(acc, f)
            }
            State::Front => self.a.try_rfold(acc, f),
            State::Back => self.b.map_err_mut(From::from).try_rfold(acc, f),
        }
    }
}

impl<A, B> FusedTryIterator for Chain<A, B>
where
    A: FusedTryIterator,
    B: FusedTryIterator<Item = A::Item>,
    A::Error: From<B::Error>,
{
}
