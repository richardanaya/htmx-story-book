use crate::models::book::{Book, Choice, Page};

#[derive(Clone)]
pub struct BookService {
    library: Vec<Book>,
}

impl BookService {
    pub fn new() -> Self {
        Self {
            library: Self::generate_fake_library(),
        }
    }

    pub fn get_book(&self, book_id: u32) -> Option<&Book> {
        self.library.iter().find(|b| b.id == book_id)
    }

    pub fn get_page(&self, book_id: u32, page_id: u32) -> Option<&Page> {
        self.get_book(book_id)
            .and_then(|book| book.pages.iter().find(|p| p.id == page_id))
    }

    pub fn get_starting_page(&self, book_id: u32) -> Option<&Page> {
        self.get_book(book_id)
            .and_then(|book| book.pages.iter().find(|p| p.id == book.starting_page))
    }

    pub fn get_library(&self) -> &Vec<Book> {
        &self.library
    }

    fn generate_fake_library() -> Vec<Book> {
        vec![
            Book {
                id: 1,
                title: "The Haunted Mansion".to_string(),
                summary: "Explore a spooky mansion full of secrets".to_string(),
                starting_page: 101,
                pages: vec![
                    Page {
                        id: 101,
                        content: "You stand before a creaky old mansion. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Enter through the front door".to_string(),
                                target_page_id: 102,
                            },
                            Choice {
                                text: "Sneak around to the back".to_string(),
                                target_page_id: 103,
                            },
                        ],
                    },
                    Page {
                        id: 102,
                        content: "The front door creaks open. Inside is a dark hallway. Do you:"
                            .to_string(),
                        choices: vec![
                            Choice {
                                text: "Light a match and explore".to_string(),
                                target_page_id: 104,
                            },
                            Choice {
                                text: "Feel your way in the dark".to_string(),
                                target_page_id: 105,
                            },
                        ],
                    },
                    Page {
                        id: 103,
                        content: "You find a broken window at the back. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Climb through carefully".to_string(),
                                target_page_id: 106,
                            },
                            Choice {
                                text: "Look for another way in".to_string(),
                                target_page_id: 101,
                            },
                        ],
                    },
                    Page {
                        id: 104,
                        content: "The match flickers, revealing a grand staircase. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Go upstairs".to_string(),
                                target_page_id: 107,
                            },
                            Choice {
                                text: "Check the parlor".to_string(),
                                target_page_id: 108,
                            },
                        ],
                    },
                    Page {
                        id: 105,
                        content: "You stumble in the dark and hear a creak behind you. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Turn around slowly".to_string(),
                                target_page_id: 109,
                            },
                            Choice {
                                text: "Run forward blindly".to_string(),
                                target_page_id: 110,
                            },
                        ],
                    },
                    Page {
                        id: 106,
                        content: "You're in a dusty kitchen. A rat scurries by. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Search the cabinets".to_string(),
                                target_page_id: 111,
                            },
                            Choice {
                                text: "Exit through the pantry".to_string(),
                                target_page_id: 112,
                            },
                        ],
                    },
                    Page {
                        id: 107,
                        content: "At the top of the stairs, you see two doors. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Enter the left door".to_string(),
                                target_page_id: 113,
                            },
                            Choice {
                                text: "Enter the right door".to_string(),
                                target_page_id: 114,
                            },
                        ],
                    },
                    Page {
                        id: 108,
                        content: "The parlor has a strange painting. It seems to be watching you. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Examine the painting".to_string(),
                                target_page_id: 115,
                            },
                            Choice {
                                text: "Ignore it and look around".to_string(),
                                target_page_id: 116,
                            },
                        ],
                    },
                ],
            },
            Book {
                id: 2,
                title: "Space Station Omega".to_string(),
                summary: "A sci-fi adventure in deep space".to_string(),
                starting_page: 201,
                pages: vec![
                    Page {
                        id: 201,
                        content: "The space station alarms are blaring! Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Head to the control room".to_string(),
                                target_page_id: 202,
                            },
                            Choice {
                                text: "Check the engineering bay".to_string(),
                                target_page_id: 203,
                            },
                        ],
                    },
                    Page {
                        id: 202,
                        content:
                            "You reach the control room. The main console is sparking! Do you:"
                                .to_string(),
                        choices: vec![
                            Choice {
                                text: "Attempt to repair it".to_string(),
                                target_page_id: 204,
                            },
                            Choice {
                                text: "Call for help on the comms".to_string(),
                                target_page_id: 205,
                            },
                        ],
                    },
                    Page {
                        id: 203,
                        content: "In engineering, you see a coolant leak. Do you:".to_string(),
                        choices: vec![
                            Choice {
                                text: "Try to seal the leak".to_string(),
                                target_page_id: 206,
                            },
                            Choice {
                                text: "Evacuate the area".to_string(),
                                target_page_id: 207,
                            },
                        ],
                    },
                ],
            },
        ]
    }
}

// Implement thread safety traits
unsafe impl Send for BookService {}
unsafe impl Sync for BookService {}
