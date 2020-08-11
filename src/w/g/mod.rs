pub mod res;

pub enum Context {
    Some,
}

impl Context {
    pub fn assets(&self) -> res::Resources {
        res::Resources::Resources
    }
}

pub fn setup() -> Context {
    Context::Some
}
