use std::io::Read;
use std::error::Error;
use reqwest;

pub struct Response {
    pub url: String,
    pub content: String,
}

// HTTP GET
// URL Template : http://m.todayhumor.co.kr/list.php?table={table}&page={page}
pub fn get_page_html( table: &str, page: i32 ) -> Result<Response,String> {
	let pageurl = format!("http://m.todayhumor.co.kr/list.php?table={}&page={}",
		table, page );

    let mut resp = match reqwest::get(&pageurl[..]) {
        Ok(v) => v,
        Err(e) => return Err( format!("{} - {}", pageurl, e.description()) )
    };

    if resp.status().is_success() {
        let mut ret = Response{
            url: pageurl,
            content: String::new()
        };
        let _ = resp.read_to_string( &mut ret.content );

        Ok(ret)
    } else {
        Err( format!("{} - {}", pageurl,
            resp.status().canonical_reason().unwrap()))
    }
}
