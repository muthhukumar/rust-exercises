// when we declare a module here it will look for module files in src/garden.rs or
// src/garden/mod.rs file. Or else in the mod garden {} inside the curly bracket

use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    let _veg = Asparagus {};
    let _veg2 = crate::garden::vegetables::Asparagus {};

    let _flower_field = garden::FlowerField {};
}
