use super::*;
use std::collections::*;

#[derive(Default, Clone)]
pub struct Cfg<'a> {
    types: BTreeMap<&'a str, BTreeSet<Row>>,
    arches: BTreeSet<&'static str>,
}

impl<'a> Cfg<'a> {
    
}