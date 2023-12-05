use std::{error::Error, result};

use crate::runtime::PathOfBuilding;

mod runtime;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let pob = PathOfBuilding::create();
    Ok(pob.start()?)
}
