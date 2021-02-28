use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

//https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys
pub trait KeyPair<A, B> {
    fn first(&self) -> &A;
    fn second(&self) -> &B;
}

impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
    where
        A: Eq + Hash + 'a,
        B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

impl<A: Hash, B: Hash> Hash for (dyn KeyPair<A, B> + '_)
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first().hash(state);
        self.second().hash(state);
    }
}

impl<A: Eq, B: Eq> PartialEq for (dyn KeyPair<A, B> + '_) {
    fn eq(&self, other: &Self) -> bool {
        self.first() == other.first() && self.second() == other.second()
    }
}

impl<A: Eq, B: Eq> Eq for (dyn KeyPair<A, B> + '_) {}

impl<A, B> KeyPair<A, B> for (A, B) {
    fn first(&self) -> &A {
        &self.0
    }
    fn second(&self) -> &B {
        &self.1
    }
}
impl<A, B> KeyPair<A, B> for (&A, &B) {
    fn first(&self) -> &A {
        self.0
    }
    fn second(&self) -> &B {
        self.1
    }
}