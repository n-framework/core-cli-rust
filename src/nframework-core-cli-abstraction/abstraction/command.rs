pub trait Command {
    fn name(&self) -> &str;
    fn args(&self) -> &[String];
    fn option(&self, name: &str) -> Option<&str>;
}
