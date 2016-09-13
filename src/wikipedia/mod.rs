extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use std::io;
use self::hyper::Client;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json;
use std::convert::From;


pub enum Error {
    Hyper(hyper::Error),
    Io(io::Error),
    JsonParseError(json::ParserError),
    Str( &'static  str)
}

impl From<hyper::Error> for Error{
    fn from(e: hyper::Error) -> Error{
        return Error::Hyper(e);
    }
}

impl From<io::Error> for Error{
    fn from(e: io::Error) -> Error{
        return Error::Io(e);
    }
}

impl From<json::ParserError> for Error{
    fn from(e: json::ParserError) -> Error{
        return Error::JsonParseError(e);
    }
}

pub fn get_pages(n: u32) ->  Result<Vec<u64>, Error> {
    let client = Client::new();

    let list_api: String = format!("https://en.wikipedia.org/w/api.php?action=query&format=json&list=random&rnnamespace=0&rnlimit={}", n);

    let mut res = try!(client.get(&list_api).send());


    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));

    let data = try!(Json::from_str(&buf[..]));

    let object = try!(data.as_object().ok_or(Error::Str("Not object")));
    let query = try!(object.get("query").ok_or(Error::Str("no query")));
    let query = try!(query.as_object().ok_or(Error::Str("query not object")));
    let random = try!(query.get("random").ok_or(Error::Str("no random")));
    let pages = try!(random.as_array().ok_or(Error::Str("random not array")));

    let mut result: Vec<u64> = vec![];
    for p in pages.iter(){
        let p = try!(p.as_object().ok_or(Error::Str("page not object")));
        let id = try!(p.get("id").ok_or(Error::Str("no id")));
        if let Some(id) = id.as_u64() {
            result.push(id)
        };
    };
    return Ok(result);
}

pub fn get_conent(id: u64) ->  Result<String, Error>  {
    let client = Client::new();
    let page_api: String = format!("https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&pageids={}&explaintext=1&exsectionformat=plain", id);
    let mut res = try!(client.get(&page_api).send());
    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));
    let data = try!(Json::from_str(&buf[..]));
    let object = try!(data.as_object().ok_or(Error::Str("Not object")));
    let query = try!(object.get("query").ok_or(Error::Str("no query")));
    let query = try!(query.as_object().ok_or(Error::Str("query not object")));
    let pages = try!(query.get("pages").ok_or(Error::Str("no pages")));
    let pages = try!(pages.as_object().ok_or(Error::Str("pages is not an object")));
    let page = try!(pages.get(&id.to_string()).ok_or(Error::Str("can't find the page")));
    let page = try!(page.as_object().ok_or(Error::Str("the page is not an object")));
    let extract = try!(page.get("extract").ok_or(Error::Str("can't find extract")));
    return Ok(try!(extract.as_string().ok_or(Error::Str("extract not string"))).to_string());
}
