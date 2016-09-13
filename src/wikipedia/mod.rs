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

    let list_api: String = format!("https://en.wikipedia.org/w/api.php?action=query&format=json&list=random&rnlimit={}", n);

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
