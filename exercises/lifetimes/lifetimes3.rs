// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let author = String::from("Douglas Hofstadter");
    let title = String::from("Gödel, Escher, Bach");
    let book;
    {
        let book = Book { author: &author, title: &title };
}
