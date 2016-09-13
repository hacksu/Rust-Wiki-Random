extern crate rand;

mod wikipedia;
use wikipedia::get_pages;
use wikipedia::get_conent;
use rand::Rng;
use rand::distributions::{Sample, Range};

fn main() {
    let mut pages: Vec<Vec<String>> = vec![];
    if let Ok(page_ids) = get_pages(2) {
        for page_id in page_ids{
            if let Ok(content) = get_conent(page_id){
                pages.push(content.split(|c| c == '.' || c == '\n')
                .map(|s| s.trim().to_string())
                .filter(|s| s != "").collect());
            } else {
                println!("something went wrong");
            }
        }
    }
    let mut rng = rand::thread_rng();

    let single_pos = rng.gen_range(0, 3);
    println!("{}", single_pos);
    for i in 0..3 {
        if i != single_pos {
            println!("{}: {}", i, rng.choose(&pages[0]).unwrap());
        } else {
            println!("{}: {}", i, rng.choose(&pages[1]).unwrap());
        }
    }

}

