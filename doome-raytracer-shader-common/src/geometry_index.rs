use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GeometryIndex {
    data: [Vec4; 2048],
}

impl GeometryIndex {
    pub fn new(data: [Vec4; 2048]) -> Self {
        Self { data }
    }

    pub fn read(&self, ptr: usize) -> Vec4 {
        self.data[ptr]
    }
}

pub struct PendingNodes {
    ptrs: [usize; 128],
    len: usize,
}

impl PendingNodes {
    pub fn new() -> Self {
        let mut ptrs = [0; 128];

        ptrs[0] = 1;

        Self { ptrs, len: 1 }
    }

    pub fn push(&mut self, ptr: usize) {
        self.ptrs[self.len] = ptr;
        self.len += 1;
    }

    pub fn pop(&mut self) -> usize {
        if self.len == 0 {
            return 0;
        }

        self.len -= 1;
        self.ptrs[self.len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_nodes() {
        let mut target = PendingNodes::new();

        assert_eq!(1, target.pop());

        target.push(10);
        target.push(20);
        target.push(30);

        assert_eq!(30, target.pop());
        assert_eq!(20, target.pop());

        target.push(40);

        assert_eq!(40, target.pop());
        assert_eq!(10, target.pop());
        assert_eq!(0, target.pop());
    }
}
