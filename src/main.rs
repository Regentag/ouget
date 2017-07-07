#[macro_use(clap_app)]
extern crate clap;
extern crate csv;
extern crate regex;
extern crate reqwest;
extern crate select;

mod csvout;
mod httpclient;
mod parse;

use std::collections::BTreeMap;

fn main() {
    let matches = clap_app!( ouget =>
        (version: "1.0")
        (author: "REGENTAG@todayhumor.co.kr")
        (about: "오늘의 유머 게시판 목록 다운로드")
        (@arg TABLE: -t --table +takes_value "Table")
        (@arg PG_BEG: -b --beg +takes_value "Page Begin")
        (@arg PG_END: -e --end +takes_value "Page End")
        (@arg FILE: -f --file +takes_value "CSV File")
        (@arg URL: -u --url +takes_value "Include URL")
    ).get_matches();

    let table = matches.value_of("TABLE").unwrap_or("bestofbest");
    let pg_beg_s = matches.value_of("PG_BEG").unwrap_or("1");
    let pg_end_s = matches.value_of("PG_END").unwrap_or("1");
    let csvfile = matches.value_of("FILE").unwrap_or("a.csv");
    let url = matches.value_of("URL").unwrap_or("no");

    let pg_beg = pg_beg_s.parse::<i32>().unwrap_or(1);
    let pg_end = pg_end_s.parse::<i32>().unwrap_or(1);
    let include_url = url == "yes";

    println!( "게시판(table) = {}, 페이지 = {} ~ {}", table, pg_beg, pg_end );
    println!();

    // Verify args.
    if pg_beg < 1 {
        println!( "오류: 첫 페이지는 1보다 커야합니다." );
        return;
    }

    if pg_end < pg_beg {
        println!( "오류: 끝 페이지는 첫 페이지 보다 커야 합니다." );
        return;
    }

    if (pg_end - pg_beg) > 10 {
        println!( "경고: 많은 페이지를 지정하면 서버에 심한 부하를 줄 수 있습니다." );
    }

    // load todayhumor's article list
    let mut articles = BTreeMap::new();
    for pg in pg_beg .. (pg_end+1) {
        let res = httpclient::get_page_html( table, pg );
        match res {
            Ok(resp) => {
                println!( "성공: {}", resp.url );
                let art_cnt = parse::th_list( table, resp.content, &mut articles );
                println!( "      {}개의 글", art_cnt );
            },
            Err(e) => { println!("오류: {}", e); }
        }
    }

    // write to csv file
    match csvout::write( &csvfile[..], include_url, &articles ) {
        Ok(msg) => { println!( "{}", msg ); },
        Err(emsg) => {
            println!( "오류: {}", emsg );
        }
    }
}
