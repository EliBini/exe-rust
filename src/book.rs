#[derive(Clone)]
pub struct Book {
    pub isbn: u32,
    pub title: String,
    pub copies_total: u32,
    pub copies_available: u32,
}

impl Book {
    pub fn new(isbn: u32, title: String, copies: u32) -> Self {
        Book {
            isbn,
            title,
            copies_total: copies,
            copies_available: copies,
        }
    }

    pub fn borrow_copy(&mut self) -> bool {
        if self.copies_available == 0 {
            return false;
        }
        self.copies_available = self.copies_available.saturating_sub(1);
        true
    }

    pub fn return_copy(&mut self) {
        if self.copies_available < self.copies_total {
            self.copies_available += 1;
        }
    }

    pub fn print(&self) {
        println!(
            "Book {{ isbn: {}, title: {}, total: {}, available: {} }}",
            self.isbn, self.title, self.copies_total, self.copies_available
        );
    }
}
