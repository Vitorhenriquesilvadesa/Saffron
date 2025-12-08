pub trait Parse {
    fn parse(source: impl Into<String>) -> Result<Self, crate::error::ParseError>
    where
        Self: Sized;
}
