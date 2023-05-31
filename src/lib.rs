extern crate web_sys;

mod board;
mod game;
mod piece;
mod utils;

use board::Board;

use piece::{Color, Position};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Event;

use futures::channel::oneshot;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Element, HtmlElement, HtmlImageElement, MouseEvent};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
const ROW: usize = 8;
const COL: usize = 8;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let board = Rc::new(RefCell::new(Board::new()));
    create_board();
    place_pieces_on_board(&board.borrow());

    // render loop goes here
    render_loop(Rc::clone(&board));

    Ok(())
}

pub fn handle_square_click(row: usize, col: usize) {
    // Handle the square click event in Rust
    // You can perform any necessary logic here
    // For example, you can access the selected square and perform actions based on it
    // Here, we simply print the selected square
    log!("Selected square: ({}, {})", row, col);
}

fn create_board() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let board = document
        .get_elements_by_class_name("chessboard")
        .item(0)
        .expect("should have a chessboard element");

    for i in 0..ROW {
        for j in 0..COL {
            let square = document
                .create_element("div")
                .expect("failed to create element")
                .dyn_into::<Element>()
                .expect("failed to cast element");
            square.set_class_name("square");
            square
                .class_list()
                .add_1(if (i + j) % 2 == 0 {
                    "lightSq"
                } else {
                    "darkSq"
                })
                .unwrap();
            square.set_attribute("data-i", &i.to_string())
                .expect("failed to set data-i attribute");
            square.set_attribute("data-j", &j.to_string())
                .expect("failed to set data-j attribute");

            board.append_child(&square).unwrap();
        }
    }
}

pub fn create_piece_imgage(id: &str) -> HtmlImageElement {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let img = document
        .create_element("img")
        .expect("failed to create element")
        .dyn_into::<HtmlImageElement>()
        .expect("failed to cast element");
    img.set_src(&format!("./img/{}.svg", id));
    img
}

pub fn place_pieces_on_board(board: &Board) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.get_elements_by_class_name("square");

    for (i, square) in board.squares().iter().enumerate() {
        let square_element = squares
            .item(i as u32)
            .expect("should have a square element")
            .dyn_into::<Element>()
            .expect("failed to cast element");
        if let Some(image_element) = square_element.query_selector("img").unwrap() {
            square_element
                .remove_child(&image_element)
                .expect("failed to remove child");
        }

        if let Some(piece) = square.get_piece() {
            let chess_color = match piece.get_color() {
                Color::White => "w",
                Color::Black => "b",
            };
            let chess_type = piece.get_type();
            let chess_id = format!("{}{}", chess_color, chess_type);
            let img = create_piece_imgage(&chess_id);
            square_element
                .append_child(&img)
                .expect("failed to append child");
        }
    }
}

async fn get_selected_square() -> Result<Position, &'static str> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    let mut sender = Some(sender);

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
        let target = mouse_event.target().unwrap();
        let square = target
            .dyn_into::<HtmlElement>()
            .expect("Failed to cast target into an HtmlElement");

        let i = square
            .get_attribute("data-i")
            .and_then(|i| i.parse::<i32>().ok());
        let j = square
            .get_attribute("data-j")
            .and_then(|j| j.parse::<i32>().ok());

        match (i, j) {
            (Some(i), Some(j)) => {
                let position = Position::new(i, j);
                if let Some(sender) = sender.take() {
                    sender.send(Ok(position)).unwrap();
                }
            }
            _ => {
                if let Some(sender) = sender.take() {
                    sender.send(Err("Invalid square")).unwrap();
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.get_elements_by_class_name("square");

    for i in 0..squares.length() {
        if let Some(square) = squares.item(i) {
            square
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    let position = receiver.await.unwrap();

    for i in 0..squares.length() {
        if let Some(square) = squares.item(i) {
            square
                .remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    closure.forget();

    position
}


// Render loop function
pub fn render_loop(board: Rc<RefCell<Board>>) {
    let board_clone = Rc::clone(&board);

    let first_selected_square_future = get_selected_square();
    wasm_bindgen_futures::spawn_local(async move {
        let first_selected_square = first_selected_square_future.await;
        match first_selected_square {
            Ok(first_square) => {
                // Do something with the first selected square
                // ...
                log!("First square selected: {:?}", first_square);

                // Wait for the user to select the second square
                let second_selected_square: Result<Position, &'static str> =
                    get_selected_square().await;
                match second_selected_square {
                    Ok(second_square) => {
                        // Do something with the second selected square
                        // ...
                        log!("Second square selected: {:?}", second_square);
                        
                        // Perform game logic based on the selected squares

                    }
                    Err(err) => {
                        log!("Error selecting second square: {}", err);
                    }
                }
            }
            Err(err) => {
                log!("Error selecting first square: {}", err);
            }
        }
    });

    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        render_loop(Rc::clone(&board_clone));
    });

    if let Some(window) = window() {
        window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
    }

    closure.forget();
    log!("Continuing render loop...");
}

// pub fn render_loop(board: Rc<RefCell<Board>>, selected_square: Rc<RefCell<SquareSelection>>) {
//     // Check if the game is over
//     if unsafe { GAME_OVER } {
//         log!("Game is over.");
//         return;
//     }

//     // Wait for selected_square to become true
//     if !selected_square.borrow().is_selected() {
//         let board_clone = Rc::clone(&board);
//         let selected_square_clone = Rc::clone(&selected_square);
//         let closure: Closure<dyn FnMut()> = Closure::new(move || {
//             render_loop(Rc::clone(&board_clone), Rc::clone(&selected_square_clone));
//         });

//         if let Some(window) = window() {
//             window
//                 .request_animation_frame(closure.as_ref().unchecked_ref())
//                 .unwrap();
//         }
//         closure.forget();
//         log!("Waiting for square selection...");
//         return;
//     }

//     // When the selected_square becomes true, perform the following steps
//     log!("Square selected!");

//     // Perform the game logic based on the selected square
//     let row = 7  - selected_square.borrow().get_row().unwrap();
//     let col = selected_square.borrow().get_col().unwrap();
//     let from = Position::new(row as i32, col as i32);
//     let to = Position::new((row - 1) as i32, col as i32);

//     let m = Move::Piece(from, to);
//     log!("{}", m);

//     match board.borrow_mut().play_move(m) {
//         GameResult::Continuing(_) => {
//             log!("Continuing");
//         },
//         GameResult::Victory(_) => {
//             log!("Victory");
//            return;
//         },
//         GameResult::Stalemate => {
//             log!("Stalemate");
//             return;
//         },
//         GameResult::IllegalMove(_) => {
//             log!("IllegalMove");
//             selected_square.borrow_mut().set_off();
//             return render_loop(Rc::clone(&board), selected_square);
//         },
//     }

//     // Clear the selected square after each move
//     selected_square.borrow_mut().set_off();

//     let board_clone = Rc::clone(&board);
//     let selected_square_clone = Rc::clone(&selected_square);
//     let closure: Closure<dyn FnMut()> = Closure::new(move || {
//         render_loop(Rc::clone(&board_clone), Rc::clone(&selected_square_clone));
//     });

//     if let Some(window) = window() {
//         window
//             .request_animation_frame(closure.as_ref().unchecked_ref())
//             .unwrap();
//     }

//     closure.forget();
//     log!("Continuing render loop...");
// }

// pub fn update_board(board: &Board) {
//     place_pieces_on_board(board);
// }
