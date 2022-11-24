#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EnterLevel(usize);

impl EnterLevel {
    pub fn l1() -> Self {
        Self(1)
    }

    pub fn l2() -> Self {
        Self(2)
    }
}
