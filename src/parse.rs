use std::collections::BTreeMap;
use select::document::Document;
use select::predicate::{ Class, Text };
use regex::Regex;

pub struct Article {
	pub table: String,
	pub real_table: String,

	pub article_no: String,
	pub date: String,
	pub writer: String,
	pub writer_no: String,
	pub subject: String,
	pub comments: String,
	pub view_count: String,
	pub ok_count: String,
}

impl Article {
	fn new() -> Article {
		Article {
			table: String::new(),
			real_table: String::new(),
			article_no: String::new(),
			date: String::new(),
			writer: String::new(),
			writer_no: String::new(),
			subject: String::new(),
			comments: String::new(),
			view_count: String::new(),
			ok_count: String::new()
		}
	}
}

fn get_comment_cnt( comments: String ) -> String {
	let re = Regex::new(r"\[*[0-9]\]").unwrap();
	if re.is_match( &comments[..] ) {
		let trmed = comments.trim();
		String::from( &trmed[1..trmed.len()-1] )
	} else {
		String::from( "0" )
	}
}

fn get_ok_cnt( ok_cnt: String ) -> String {
	match ok_cnt.find("/") {
		Some(n) => String::from( &ok_cnt[0..n] ),
		None => ok_cnt
	}
}

fn get_real_table( tableclass: &str ) -> String {
	match tableclass.find( " " ) {
		Some(n) => String::from(&tableclass[(n+1)..tableclass.len()]),
		None => String::from("badtable")
	}
}

pub fn th_list( table: &str, html: String, list: &mut BTreeMap< i32, Article > ) -> usize {
	let doc = Document::from( &html[..] );
	let mut art_cnt: usize = 0;

	// 글 목록에서 하나씩 분리한다.
	for row in doc.find( Class("listLineBox") ) {
		art_cnt += 1;
		let mut art = Article::new();

		let icondiv = row.find( Class("board_icon_mini") ).next().unwrap();
		let tableclass = icondiv.attr("class").unwrap();

		let art_no = row.find( Class("list_no") ).next().unwrap().text();
		let date = row.find( Class("listDate") ).next().unwrap().text();
		let writer = row.find( Class("list_writer") ).next().unwrap().text();
		let writer_no = row.attr("mn").unwrap();
		let subj = row.find( Class("listSubject") ).next().unwrap();
		let subj_text = subj.find( Text{} ).next().unwrap();
		let cmt = match row.find( Class("memo_count") ).next() {
			Some(node) => { get_comment_cnt(node.text()) },
			None => String::from("0")
		};
		let view_count = row.find( Class("list_viewCount") ).next().unwrap().text();
		let ok_count = row.find( Class("list_okNokCount") ).next().unwrap().text();

		art.table = String::from( table );
		art.real_table = get_real_table( tableclass );
		art.article_no = art_no;
		art.date = date;
		art.writer = writer;
		art.writer_no = String::from( writer_no );
		if let Some(s) = subj_text.as_text() {
			art.subject = String::from(s);
		}
		art.comments = cmt;
		art.view_count = view_count;
		art.ok_count = get_ok_cnt( ok_count );

		let iart_no = art.article_no.parse::<i32>().unwrap_or(-1);
		list.insert( iart_no, art );
	}

	art_cnt
}