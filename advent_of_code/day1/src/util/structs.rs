pub struct Digit {
    pub name: &'static str,
    pub value: i32,
}

#[derive(Debug)]
pub struct SubstrLocation {
    pub value: i32,
    pub location: usize,
}
