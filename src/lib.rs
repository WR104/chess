extern crate web_sys;

mod board;
mod game;
mod piece;
mod utils;

use board::Board;

use game::get_next_move;
use piece::{Color, Position};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use web_sys::Event;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement, HtmlImageElement, MouseEvent};

use crate::game::GameResult;
use crate::game::Move;

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

thread_local! {
    // ONly allow select one square at the time
    static IS_SELECTING: Cell<bool> = Cell::new(false);
}
const ROW: usize = 8;
const COL: usize = 8;
static TURN: Color = Color::White;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let board = Rc::new(RefCell::new(Board::new()));
    create_board();
    update_board(&board.borrow());

    // render loop goes here
    render_loop(Rc::clone(&board));

    Ok(())
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

            // convert the i & j to row and col of the chess board
            let row = 7 - i;
            let col = j;
            square
                .set_attribute("data-i", &row.to_string())
                .expect("failed to set data-i attribute");
            square
                .set_attribute("data-j", &col.to_string())
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
    img.set_src(&format!("https://raw.githubusercontent.com/WR104/chess/main/www/img/{}.svg", id));
    img
}

pub fn update_board(board: &Board) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.get_elements_by_class_name("square");

    for (i, square) in board.squares().iter().enumerate() {
        let square_element = squares
            .item(i as u32)
            .expect("should have a square element")
            .dyn_into::<Element>()
            .expect("failed to cast element");

        // Remove every things on a square (piece, hint)
        while let Some(child) = square_element.first_child() {
            square_element
                .remove_child(&child)
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
    let sender = Rc::new(RefCell::new(Some(sender)));

    let closure = Closure::wrap(Box::new(move |event: Event| {
        IS_SELECTING.with(|is_selecting| {
            if !is_selecting.get() {
                is_selecting.set(true);

                let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                let target = mouse_event.target().unwrap();
                let square = if target.dyn_ref::<HtmlImageElement>().is_some() {
                    target
                        .dyn_into::<HtmlElement>()
                        .expect("Failed to cast target into an HtmlElement")
                        .parent_element()
                        .expect("Failed to get parent element")
                        .dyn_into::<HtmlElement>()
                        .expect("Failed to cast parent element into an Element")
                } else {
                    target
                        .dyn_into::<HtmlElement>()
                        .expect("Failed to cast target into an HtmlElement")
                };

                let i = square
                    .get_attribute("data-i")
                    .and_then(|i| i.parse::<i32>().ok());
                let j = square
                    .get_attribute("data-j")
                    .and_then(|j| j.parse::<i32>().ok());

                match (i, j) {
                    (Some(i), Some(j)) => {
                        let position = Position::new(i, j);
                        if let Some(sender) = sender.borrow_mut().take() {
                            sender.send(Ok(position)).unwrap();
                        }
                    }
                    _ => {
                        if let Some(sender) = sender.borrow_mut().take() {
                            sender.send(Err("Invalid square")).unwrap();
                        }
                    }
                }

                is_selecting.set(false);
            }
        });
    }) as Box<dyn FnMut(_)>);

    let window = web_sys::window().expect("no global `window” exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.query_selector_all(".square").unwrap();

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

// !!! Must check if the selected square has a piece
pub fn get_hint_pos(board: &Board, pos: Position) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();
    let turn = board.get_turn_color();
    if let Some(piece) = board.get_piece(pos) {
        // get all
        let moves = piece.get_legal_moves(board);
        for m in moves {
            match m {
                Move::QueenSideCastle => match turn {
                    Color::White => result.push(Position::new(0, 6)),
                    Color::Black => result.push(Position::new(7, 6)),
                },
                Move::KingSideCastle => match turn {
                    Color::White => result.push(Position::new(0, 2)),
                    Color::Black => result.push(Position::new(7, 2)),
                },
                Move::Piece(_from, to) => {
                    result.push(to);
                }
                Move::Promotion(_from, to, _) => {
                    result.push(to);
                }
                _ => {}
            }
        }

    }

    result
}

pub fn update_hint_squares(hint_pos: Vec<Position>) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.get_elements_by_class_name("square");

    for pos in hint_pos {
        let index = (7 - pos.get_row()) * 8 + pos.get_col();

        let square_element = squares
            .item(index as u32)
            .expect("should have a square element")
            .dyn_into::<Element>()
            .expect("failed to cast element");

        let hint_box = document
            .create_element("div")
            .expect("failed to create element")
            .dyn_into::<Element>()
            .expect("failed to cast element");

        hint_box.set_class_name("hint");

        // Check if an <img> child is present
        if let Some(image_element) = square_element.query_selector("img").unwrap() {
            // Append the hint_box as the second child
            square_element
                .insert_before(&hint_box, Some(&image_element))
                .expect("failed to insert hint_box before img_element");
        } else {
            // Append the hint_box as the last child
            square_element
                .append_child(&hint_box)
                .expect("failed to append child");
        }
    }
}

// Render loop function
pub fn render_loop(board: Rc<RefCell<Board>>) {
    let mut board_clone = Rc::clone(&board);

    if board.borrow().get_turn_color() == TURN {
        // Get the first selected square
        let first_selected_square_future = get_selected_square();
        wasm_bindgen_futures::spawn_local(async move {
            let first_selected_square = first_selected_square_future.await;
            match first_selected_square {
                Ok(first_square) => {
                    let from: Position = first_square;

                    // Do something with the first selected square

                    // Get the hint squares
                    let hint_positions = get_hint_pos(&board.borrow(), from);
                    if !hint_positions.is_empty() {
                        // Check that the selected square has a piece
                        update_hint_squares(hint_positions);
                    }

                    // Wait for the user to select the second square
                    let second_selected_square: Result<Position, &'static str> =
                        get_selected_square().await;
                    match second_selected_square {
                        Ok(second_square) => {
                            // Do something with the second selected square

                            update_board(&board_clone.borrow());

                            let to = second_square;
                            let m = Move::Piece(from, to);
                            // Perform game logic based on the selected squares
                            match board.borrow_mut().play_move(m) {
                                GameResult::Continuing(next_board) => {
                                    log!("Continuing");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                }
                                GameResult::Victory(next_board, _) => {
                                    log!("You won the game!");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                    update_board(&board_clone.borrow());
                                    return;
                                }
                                GameResult::Stalemate => {
                                    log!("Drawn Game");
                                    return;
                                }
                                GameResult::IllegalMove(_) => {
                                    log!("IllegalMove");
                                }
                            }
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

            update_board(&board_clone.borrow());
            render_loop(Rc::clone(&board_clone));
        });
    } else {
        // Computer makes decisions
        let m = get_next_move(&board.borrow(), true);

        match board.borrow_mut().play_move(m) {
            GameResult::Continuing(next_board) => {
                log!("Continuing");
                board_clone = Rc::new(RefCell::new(next_board));
                update_board(&board_clone.borrow());
            }
            GameResult::Victory(next_board, _) => {
                log!("You lost the game!");
                board_clone = Rc::new(RefCell::new(next_board));
                update_board(&board_clone.borrow());
                return;
            }
            GameResult::Stalemate => {
                log!("Drawm game");
                update_board(&board_clone.borrow());
                return;
            }
            GameResult::IllegalMove(_) => {
                log!("IllegalMove");
            }
        }

        render_loop(Rc::clone(&board_clone));
    }
}
