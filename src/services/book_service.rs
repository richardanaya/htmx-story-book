use crate::models::book::{Book, Page};
use std::sync::Arc;

#[derive(Clone)]
pub struct BookService;

impl BookService {
    pub fn get_book<'a>(&self, library: &'a [Book], book_id: u32) -> Option<&'a Book> {
        library.iter().find(|b| b.id == book_id)
    }

    pub fn get_page<'a>(&self, book: &'a Book, page_id: u32) -> Option<&'a Page> {
        book.pages.iter().find(|p| p.id == page_id)
    }

    pub fn get_starting_page<'a>(&self, book: &'a Book) -> Option<&'a Page> {
        book.pages.iter().find(|p| p.id == book.starting_page)
    }
}

// Implement thread safety traits
unsafe impl Send for BookService {}
unsafe impl Sync for BookService {}
