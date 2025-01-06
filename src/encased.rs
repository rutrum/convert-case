#![cfg(feature = "encased")]
use crate::{Case, Casing};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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

impl<const C: Case> ToString for Encased<C>{
    fn to_string(&self) -> String {
        self.raw().clone()
    }
}
