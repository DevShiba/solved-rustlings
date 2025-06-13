// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
struct Book<'a> { // Adding a lifetime parameter `'a` to the struct definition
    // This indicates that the struct will hold references with the lifetime `'a`.
    author: &'a str,
    title: &'a str,
}

// struct Book { // Would work too.
//     author: &'static str,
//     title: &'static str,
// }

// This works with <'a> but NOT with &'static str.
// let author_string = String::from("George Orwell");
// let book = Book {
//     author: &author_string,  // This is NOT &'static str
//     title: "1984",
// };

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
