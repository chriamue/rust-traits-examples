pub trait Animal {
    fn name(&self) -> String;

    fn species(&self) -> &'static str;
}
