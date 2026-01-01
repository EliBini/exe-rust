pub struct Loan {
    pub member_id: u32,
    pub isbn: u32,
    pub days: u32,
    pub approved: bool,
}

impl Loan {
    pub fn new(member_id: u32, isbn: u32, days: u32) -> Self {
        Loan {
            member_id,
            isbn,
            days,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print(&self) {
        println!(
            "Loan {{ member_id: {}, isbn: {}, days: {}, approved: {} }}",
            self.member_id, self.isbn, self.days, self.approved
        );
    }
}
