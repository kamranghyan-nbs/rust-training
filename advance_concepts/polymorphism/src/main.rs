trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

struct Sword {
    pub name: String,
    pub damage: u16,
    pub swing_time_ms: u16 
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        (self.damage * 1000_u16) / self.swing_time_ms * 10_u16
    }

    fn description(&self) -> String {
        format!("{}, damage: {}, swing_time: {}ms", self.name, self.damage, self.swing_time_ms)
    }
}

struct Shield {
    pub name: String,
    pub armor: u16,
    pub block: u16 
}

impl Sellable for Shield {
    fn price(&self) -> u16 {
        self.armor + self.block
    }

    fn description(&self) -> String {
        format!("{}, armor: {}, block: {}ms", self.name, self.armor, self.block)
    }
}

// Static dispatch
fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("Static: I offer you: {} [{}g]", item.description(), item.price())
}

fn vendor_text_dynamic(item: &dyn Sellable) -> String {
    format!("Dynamic: I offer you: {} [{}g]", item.description(), item.price())
}

fn main() {
    let sword = Sword {
        name: "Sword of Cowardice".into(),
        damage: 10,
        swing_time_ms: 1500
    };

    let shield = Shield {
        name: "Golder Barrier".into(),
        armor: 50,
        block: 35
    };

    println!("Sword: {}", vendor_text_static(&sword));
    println!("Shield: {}", vendor_text_static(&shield));

    // Dynamic dispatch via refs
    let sellables: Vec<&dyn Sellable> = vec![&sword, &shield];

    for s in sellables {
        println!("{}", vendor_text_dynamic(s))
    }

    // Box dispatch
    let owned_sellables: Vec<Box<dyn Sellable>> = vec![Box::new(Sword{ name: "Sword 2".into(), damage:10, swing_time_ms: 1500}), Box::new(Shield{ name: "Shield 2".into(), armor: 50, block: 45 })];

    for s in &owned_sellables {
        println!("Box: {}", vendor_text_dynamic(s.as_ref()));
    }
}
