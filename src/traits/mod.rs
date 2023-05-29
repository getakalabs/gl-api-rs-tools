pub mod prelude;

pub trait Decrypt {
    fn decrypt(&self) -> Option<Self> where Self: Sized;
}

pub trait Dedup {
    fn dedup(&self) -> Self where Self: Sized;
}

pub trait Encrypt {
    fn encrypt(&self) -> Option<Self> where Self: Sized;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}