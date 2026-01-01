#[derive(Clone)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
}

impl Member {
    pub fn new(id: u32, name: String) -> Self {
        Member {
            id,
            name,
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn print(&self) {
        println!("Member {{ id: {}, name: {}, active: {} }}", self.id, self.name, self.is_active);
    }
}
