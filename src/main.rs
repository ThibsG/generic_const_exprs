#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

#[allow(dead_code)]
#[derive(Debug)]
struct Chain<const WIDTH: usize> {
    length: usize,
    blocks: [u8; WIDTH],
}

struct Condition<const B: bool>();
trait IsTrue {}
impl IsTrue for Condition<true> {}

impl<const WIDTH: usize> Chain<WIDTH>
where
    Condition<{ WIDTH < 8 }>: IsTrue,
{
    pub fn new() -> Self {
        Self {
            length: 0,
            blocks: [0; WIDTH],
        }
    }
}

fn main() {
    let chain = Chain::<7>::new();

    // doesn't compile !
    // let chain = Chain::<8>::new();

    println!("{chain:#?}");
}
