#[derive(Debug)]
#[derive(Clone)]
pub struct Node(
    pub f32,
    pub f32,
    pub f32
);

impl Node {
    pub fn euc_dist(&self, other: &Node) -> f32 {
        f32::sqrt(
            (self.0 - other.0).powf(2.0) +
            (self.1 - other.1).powf(2.0) +
            (self.2 - other.2).powf(2.0)
            )
    }

    pub fn mul(&self, x: f32) -> Node {
        Node(
            x * self.0,
            x * self.1,
            x * self.2
            )
    }

    pub fn sub(&self, b: &Node) -> Node {
        Node(
            self.0 - b.0,
            self.1 - b.1,
            self.2 - b.2
            )
    }

    pub fn add(&self, b: &Node) -> Node {
        Node(
            self.0 + b.0,
            self.1 + b.1,
            self.2 + b.2
            )
    }
}
