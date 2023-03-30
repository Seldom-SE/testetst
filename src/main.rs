use std::fmt::{Debug, Formatter, Result};

use bevy::prelude::*;

fn main() {}

#[derive(Reflect)]
struct MyStruct;

struct MyError;

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "something occured")
    }
}
