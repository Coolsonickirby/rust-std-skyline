// compile-flags:--crate-name=test
// aux-build:coherence_lib.rs
// check-pass

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;
struct Local2<T>(Rc<T>);

impl<T> Remote2<Local, Box<T>> for u32 {}
impl<'a, T> Remote2<Local, &'a T> for u32 {}
impl<T> Remote2<Local2<T>, Box<T>> for u32 {}
impl<'a, T> Remote2<Local2<T>, &'a T> for u32 {}

fn main() {}
