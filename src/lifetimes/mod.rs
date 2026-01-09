pub struct Lifetimes{}

impl Lifetimes{
    pub fn run() -> () {
        println!("------------- Excercise 6 Lifetimes ---------------");
        let book1 = Book::new("Book#1", "author#1");
        let user1 = User::new("Matt");
        let book2 = Book::new("Book#2", "author#2");
        let b1 = Borrow::new(&book1, &user1);
        let b2 = Borrow::new(&book2, &user1);
        b1.show_data();
        b2.show_data();
        let borrows = vec![&b1,&b2];
        let user_reference = Lifetimes::find_borrowed_book(borrows, "Book#1");
        println!("User_reference_data: {:?}", user_reference);
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
