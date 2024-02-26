mod bindings;

use bindings::exports::component::deleteme::foo::{GuestR1, R2};

pub struct R1;

impl GuestR1 for R1 {
    fn my_func(&self, p1: R2) {}
}
