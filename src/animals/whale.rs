#[derive(Debug)]
pub struct Whale {
    name: String,
    energy: u8,
}

impl Whale {
    pub fn new(name: String) -> Self {
        Self { name, energy: 100 }
    }
}

impl Animal for Whale {
    fn name(&self) -> &str {
        &self.name
    }
    fn species(&self) -> &'static str {
        "Whale"
    }
    fn energy(&self) -> u8 {
        self.energy
    }
    fn set_energy(&mut self, energy: u8) {
        self.energy = energy;
    }
}

// Whales can only swim (no walking or flying)
impl Swimming for Whale {
    fn max_depth(&self) -> u32 {
        2000
    }
}
