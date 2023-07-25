#![cfg(feature = "encased")]
use crate::{Case, Casing};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Encased<const CASE: Case>(String);

impl<const C: Case> Encased<C> {
    pub fn new<const CASE: Case, T: AsRef<str> + Sized>(input: &T) -> Encased<CASE>
    where
        String: PartialEq<T>,
    {
        Encased::<CASE>(input.to_case(CASE))
    }

    pub fn raw(&self) -> &String {
        &self.0
    }
}
