mod wikipedia;
use wikipedia::get_pages;


fn main() {
    println!("Hello World!");
    if let Ok(pages) = get_pages(5) {
        println!("{:?}", pages);
    }
}

