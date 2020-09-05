pub struct Consts {
    pub name: Vec<String>,
    pub value: Vec<String>,
}

impl Consts {
    pub fn new() -> Self {
        let name = Vec::new();
        let value = Vec::new();
        Self { name, value }
    }

    pub fn add_val(&mut self, nam: &str, val: f64) {
        self.name.push(nam.to_owned());
        self.value.push(format!("{}", val));
    }
}