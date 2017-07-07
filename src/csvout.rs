use std::collections::BTreeMap;
use std::error::Error;
use std::fs::OpenOptions;

use csv;
use parse::Article;

pub fn write( csvfile: &str, with_url: bool, list: &BTreeMap< i32, Article > ) -> Result<String,String> {
	let file =
		match OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open( csvfile )
		{
			Ok(f) => f,
			Err(e) => { return Err( format!("{} - {}", csvfile, e.description()) ) }
		};

	let mut wtr = csv::Writer::from_writer( file );
	if with_url {
		let _ = wtr.write_record(&[
			"게시판",
			"실제 게시판",
			"번호",
			"작성자 번호",
			"작성자",
			"게시판 등록일",
			"글제목",
			"댓글수",
			"조회수",
			"추천수",
			"링크",
			"작성자 링크"
		]);
	} else {
		let _ = wtr.write_record(&[
			"게시판",
			"실제 게시판",
			"번호",
			"작성자 번호",
			"작성자",
			"게시판 등록일",
			"글제목",
			"댓글수",
			"조회수",
			"추천수"
		]);
	}

	for (_, art) in list {
		if with_url {
			let art_url = format!("http://www.todayhumor.co.kr/board/view.php?table={}&no={}",
				&art.table, &art.article_no );
			let wtr_url = format!("http://www.todayhumor.co.kr/board/list.php?kind=member&mn={}",
				&art.writer_no );

			let _ = wtr.write_record(&[
				&art.table,
				&art.real_table,
				&art.article_no,
				&art.writer_no,
				&art.writer,
				&art.date,
				&art.subject,
				&art.comments,
				&art.view_count,
				&art.ok_count,
				&art_url,
				&wtr_url
			]);
		} else {
			let _ = wtr.write_record(&[
				&art.table,
				&art.real_table,
				&art.article_no,
				&art.writer_no,
				&art.writer,
				&art.date,
				&art.subject,
				&art.comments,
				&art.view_count,
				&art.ok_count
			]);
		}
	}
	let _ = wtr.flush();
	Ok( String::from("완료.") )
}
