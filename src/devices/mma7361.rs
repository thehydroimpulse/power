use device::Device;

pub struct Mma7361 {
    noop: int
}

impl Mma7361 {
    pub fn new() -> Mma7361 {
        Mma7361 {
            noop: 1
        }
    }
}

impl Device for Mma7361 {}
