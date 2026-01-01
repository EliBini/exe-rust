mod member;
mod book;
mod loan;
mod library;

use member::Member;
use book::Book;
use library::Library;

fn main() {
    // create members
    let m1 = Member::new(1, String::from("Alice"));
    let mut m2 = Member::new(2, String::from("Bob"));

    // create books
    let b1 = Book::new(1001, String::from("The Rust Book"), 1);
    let b2 = Book::new(2002, String::from("Rust By Example"), 2);

    // create library and add members/books
    let mut lib = Library::new(String::from("Central Library"));
    lib.add_member(m1);
    lib.add_member(m2.clone());
    lib.add_book(b1);
    lib.add_book(b2);

    println!("Initial state:");
    lib.print();

    // valid borrow: Alice borrows isbn 1001
    println!("\nAlice attempts to borrow isbn 1001:");
    let ok = lib.borrow_book(1, 1001, 7);
    println!("Borrow success: {}", ok);
    lib.print();

    // invalid borrow: Bob tries same book (no copies left)
    println!("\nBob attempts to borrow isbn 1001:");
    let ok2 = lib.borrow_book(2, 1001, 3);
    println!("Borrow success: {}", ok2);
    lib.print();

    // return book: Alice returns isbn 1001
    println!("\nAlice returns isbn 1001:");
    let ret = lib.return_book(1, 1001);
    println!("Return success: {}", ret);
    lib.print();

    // deactivate Bob and attempt borrow
    println!("\nDeactivating Bob and attempting borrow:");
    // find Bob by id and deactivate
    let idx = lib.find_member_index(2);
    if idx != -1 {
        let i = idx as usize;
        lib.members[i].deactivate();
    }

    let ok3 = lib.borrow_book(2, 2002, 5);
    println!("Borrow by deactivated member success: {}", ok3);
    lib.print();
}
