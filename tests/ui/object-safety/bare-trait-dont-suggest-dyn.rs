// run-rustfix
#![deny(bare_trait_objects)]
fn ord_prefer_dot(s: String) -> Ord {
    //~^ ERROR trait objects without an explicit `dyn` are deprecated
    //~| ERROR the trait `Ord` cannot be made into an object
    //~| WARNING this is accepted in the current edition (Rust 2015)
    (s.starts_with("."), s)
}
fn main() {
    let _ = ord_prefer_dot(String::new());
}
