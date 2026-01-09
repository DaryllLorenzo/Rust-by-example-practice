pub struct Lifetimes{}

impl Lifetimes{
    pub fn run() -> () {
        println!("------------- Excercise 6 Lifetimes ---------------");
        let book1 = Book::new("Book#1", "author#1");
        let user1 = User::new("Matt");
        let book2 = Book::new("Book#223", "author#2");
        let b1 = Borrow::new(&book1, &user1);
        let b2 = Borrow::new(&book2, &user1);
        b1.show_data();
        b2.show_data();
        let borrows = vec![&b1,&b2];
        let user_reference = Lifetimes::find_borrowed_book(borrows, "Book#1");
        println!("User_reference_data: {:?}", user_reference);

        println!("Referencia de libro {:?}", Lifetimes::longest_borrow(&b1, &b2));

        let r1 = Renovation::new(&b1, "2026-01-25");
        r1.show_data();
    }

    fn find_borrowed_book<'a>(borrows: Vec<&'a Borrow>, book_name: &str) -> &'a User{
        for borrow in borrows.iter(){
            if borrow.book.title == book_name {
                return borrow.user;
            }
        }

        panic!("Borrow of book {} not found", book_name); 
    }
    
    fn foo(book: Book) -> () {
        println!("{:?}", book);
    }

    // references of results doesn't outlive original borrows
    fn longest_borrow<'a>(b1: &'a Borrow, b2: &'a Borrow) -> &'a Book{
        
        if b1.book.title >= b2.book.title {
            return b1.book; 
        }

        b2.book
    }

}


#[derive(Debug)]
struct Book {
    title : String,
    author : String
}

#[derive(Debug)]
struct User {
    name : String
}

#[derive(Debug)]
struct Borrow<'a> {
    book : &'a Book,
    user : &'a User
}

impl Book {
    fn new(title: &str, author : &str) -> Self {
        Book{
            title : title.to_string(),
            author : author.to_string()
        }
    }
}

impl User{
    fn new(name : &str) -> Self {
        User{
            name : name.to_string()
        }
    }
}

impl<'a> Borrow<'a> {
    fn new(book: &'a Book, user: &'a User) -> Self {
        Borrow {
            book,
            user
        }
    }

    fn show_data(&self) -> () {
        println!("Book: {:?}", self.book);
        println!("User: {:?}", self.user);
    }
}

// we need two lifetimes bc, Renovation will have its own lifetime in scope, so we need to tell to the compiler 
// that the Renovation lives at most as the Borrow, so 'a <= 'b, and the lifetime 'b would be the lifetime of the own Borrow,
// which specifies to Book and User how long the references are going to live.

// 'a: lifetime of the REFERENCE to the Borrow
// 'b: lifetime of the Borrow itself (and thus of the Book/User it references)
//
// The key relationship: 'a <= 'b
// (The reference cannot outlive the data it points to)
//
// This creates a chain: Renovation('a) <= Borrow('b) <= (Book, User)
struct Renovation<'a, 'b>{
    // lives for at least 'a
    borrow: &'a Borrow<'b>, // has its own lifetime 'b
    new_date: String
}

impl<'a, 'b> Renovation<'a, 'b> {
    fn new(borrow: &'a Borrow<'b>, new_date: &str) -> Self{
        Renovation { 
            borrow : borrow,
            new_date : new_date.to_string()
        }
    }

    fn show_data(&self) -> () {
        println!("Borrow: {:?}", self.borrow);
        println!("New_date: {:?}", self.new_date);
    }
}