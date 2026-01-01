use crate::book::Book;
use crate::loan::Loan;
use crate::member::Member;

pub struct Library {
    pub name: String,
    pub members: Vec<Member>,
    pub books: Vec<Book>,
    pub loans: Vec<Loan>,
}

impl Library {
    pub fn new(name: String) -> Self {
        Library {
            name,
            members: Vec::new(),
            books: Vec::new(),
            loans: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn print(&self) {
        println!("Library: {}", self.name);
        println!("Members:");
        for i in 0..self.members.len() {
            self.members[i].print();
        }
        println!("Books:");
        for i in 0..self.books.len() {
            self.books[i].print();
        }
        println!("Loans:");
        for i in 0..self.loans.len() {
            self.loans[i].print();
        }
    }

    pub fn find_member_index(&self, id: u32) -> i32 {
        for i in 0..self.members.len() {
            if self.members[i].id == id {
                return i as i32;
            }
        }
        -1
    }

    pub fn find_book_index(&self, isbn: u32) -> i32 {
        for i in 0..self.books.len() {
            if self.books[i].isbn == isbn {
                return i as i32;
            }
        }
        -1
    }

    pub fn borrow_book(&mut self, member_id: u32, isbn: u32, days: u32) -> bool {
        let m_idx = self.find_member_index(member_id);
        let b_idx = self.find_book_index(isbn);

        let mut approved = false;

        if m_idx != -1 && b_idx != -1 {
            let midx = m_idx as usize;
            let bidx = b_idx as usize;
            if self.members[midx].is_active {
                // attempt to borrow a copy; the mutable borrow ends after this call
                if self.books[bidx].borrow_copy() {
                    approved = true;
                }
            }
        }

        let mut loan = Loan::new(member_id, isbn, days);
        if approved {
            loan.approve();
            self.loans.push(loan);
            true
        } else {
            self.loans.push(loan);
            false
        }
    }

    pub fn return_book(&mut self, member_id: u32, isbn: u32) -> bool {
        let b_idx = self.find_book_index(isbn);
        if b_idx == -1 {
            return false;
        }
        // find an approved matching loan
        let mut loan_found = false;
        for i in 0..self.loans.len() {
            if self.loans[i].member_id == member_id && self.loans[i].isbn == isbn && self.loans[i].approved {
                loan_found = true;
                break;
            }
        }

        if !loan_found {
            return false;
        }

        let bidx = b_idx as usize;
        self.books[bidx].return_copy();
        true
    }
}
