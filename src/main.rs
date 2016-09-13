mod wikipedia;
use wikipedia::get_pages;
use wikipedia::get_conent;
use wikipedia::Error;


fn main() {
    println!("Hello World!");
    if let Ok(pages) = get_pages(5) {
        for page_id in pages{
            if let Ok(content) = get_conent(page_id){
                println!("{:?}", content);
            } else if let Err(Error::Str(e)) = get_conent(page_id)  {
                println!("something went wrong {}", e);
            }
        }
    }
}

