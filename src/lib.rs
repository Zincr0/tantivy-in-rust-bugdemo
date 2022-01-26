// extern crate libc;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy};


use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;




fn process() -> tantivy::Result<()> {


    let mut schema_builder = Schema::builder();

    let atext = schema_builder.add_text_field("atext", STRING | STORED);

    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema.clone());

    // This segfault on go:
    // let mut index_writer = index.writer(10_000_000)?;
    // let mut index_writer = index.writer(1_000_000).unwrap();
    let mut index_writer = index.writer_with_num_threads(1, 10_000_000)?;
    // This works on go:
    // let mut index_writer = index.writer_with_num_threads(1, 1_000_000)?;

    index_writer.add_document(doc!(atext => "good mach 1"));
    index_writer.add_document(doc!(atext => "good mach 2"));
    index_writer.add_document(doc!(atext => "no mach test"));

    index_writer.commit().unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![atext, ]);

    let query = query_parser.parse_query("good")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("retrieved_doc: {}", schema.to_json(&retrieved_doc));
    }
    
    Ok(())
}




#[no_mangle]
pub extern "C" fn filtra(s: *const c_char) -> *mut c_char {

    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let query_str = c_str.to_str().unwrap();
    println!("got from golang: {:?}", query_str);

    let _result = process();

    //println!("weird return from process inside rust: {:?}", _result);

    let fresult = String::from("fake result");
    let c_str_song = CString::new(fresult).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern "C" fn freeFiltra(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
