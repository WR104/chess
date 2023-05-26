// import * as wasm from "chess";
// wasm.greet();

import { Board } from "chess";

var board = Board.new();
console.log(board);

//squares array as u8 from board.rs 
var squaresU8Value = board.squares_u8_value_js();

console.log(squaresU8Value);

const ROW = 8;
const COL = 8;
const SquareNum = board.squares_num();    //number of squares of the board

function createBoard() {
    var board = document.getElementsByClassName("chessboard")[0];

    for (var i = 0; i < ROW; i++) {
        for (var j = 0; j < COL; j++) {
            var square = document.createElement("div");
            square.className = "square ";
            square.className += (i + j) % 2 ? " darkSq" : " lightSq";

            board.appendChild(square);
        }
    }
}

function createPieceImage(id) {
    var img = document.createElement("img");
    img.src = "./img/" + id + ".svg";
    return img;
}

function placePiecesOnBoard() {
    const SQUARES = document.getElementsByClassName("square");
    const CHESS_TYPES = ["P", "N", "B", "R", "Q", "K"];

    for (let i = 0; i < SquareNum; i++) {
        const square = SQUARES[i];
        const imageElement = square.querySelector("img");
        if (imageElement) {
            square.removeChild(imageElement);
        }
        let chessType = squaresU8Value[i];
        if (chessType === 0) {
            continue;
        }
        const chessColor = chessType < 7 ? "w" : "b";
        chessType = (chessType % 6) || 6;
        const chessId = `${chessColor}${CHESS_TYPES[chessType - 1]}`;
        const img = createPieceImage(chessId);
        square.appendChild(img);
    }
}
createBoard();
placePiecesOnBoard();