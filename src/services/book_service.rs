use crate::models::book::{Book, Page};

pub struct BookService;

impl BookService {
    pub fn get_book(&self, library: &[Book], book_id: u32) -> Option<&Book> {
        library.iter().find(|b| b.id == book_id)
    }

    pub fn get_page(&self, book: &Book, page_id: u32) -> Option<&Page> {
        book.pages.iter().find(|p| p.id == page_id)
    }

    pub fn get_starting_page(&self, book: &Book) -> Option<&Page> {
        book.pages.iter().find(|p| p.id == book.starting_page)
    }
}
