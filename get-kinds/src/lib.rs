pub trait Kind {
    fn kind<'a>(&self) -> &'a str;
}

pub use generate_kinds::kinds;