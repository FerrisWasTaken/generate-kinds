/// Implements just one function [`Kind::kind`]
pub trait Kind {
    /// ### Examples
    /// ```
    /// use generate_kinds::kinds;
    /// 
    /// #[kinds]
    /// enum Test {
    ///     T1,
    ///     T2
    /// }
    /// assert_eq!(Test::T1.kind(), "Test :: T1");
    /// ```
    fn kind<'a>(&self) -> &'a str;
}

pub use generate_kinds::kinds;