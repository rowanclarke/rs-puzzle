pub struct Cartesian<I: Iterator, J: Iterator> {
    a: I,
    b: J,
    ac: Option<<I as Iterator>::Item>,
    bc: J,
}

impl<I: Iterator, J: Iterator + Clone> Cartesian<I, J>
where
    <I as Iterator>::Item: Clone,
{
    pub fn new(mut a: I, b: J) -> Self {
        Self {
            ac: a.next(),
            a,
            bc: b.clone(),
            b,
        }
    }
}

impl<I: Iterator, J: Iterator + Clone> Iterator for Cartesian<I, J>
where
    <I as Iterator>::Item: Clone,
{
    type Item = (<I as Iterator>::Item, <J as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.ac.clone() {
            if let Some(b) = self.b.next() {
                return Some((a, b));
            }
            self.ac = self.a.next();
            self.b = self.bc.clone();
            return self.next();
        }
        None
    }
}
