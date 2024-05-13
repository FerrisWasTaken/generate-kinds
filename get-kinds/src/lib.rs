pub trait Kind {
    fn kind<'a>(&self) -> &'a str;
}