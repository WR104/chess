// import {Board, Test} from "chess";

// var board = Board.new();
// console.log(board);

// //squares array as u8 from board.rs 
// var squaresU8Value = board.squares_u8_value_js();

// console.log(squaresU8Value);

// const ROW = 8;
// const COL = 8;
// const SquareNum = board.squares_num();    //number of squares of the board
// let selectedSquare = null;

// function createBoard() {
//     var board = document.getElementsByClassName("chessboard")[0];

//     for (let i = 0; i < ROW; i++) {
//         for (let j = 0; j < COL; j++) {
//             var square = document.createElement("div");
//             square.className = "square ";
//             square.className += (i + j) % 2 ? " darkSq" : " lightSq";

//             square.addEventListener("click", function () {
//                 handleSquareClick(i, j);
//             });

//             board.appendChild(square);
//         }
//     }
// }

// function handleSquareClick(row, col) {
//     selectedSquare = { row, col };
// }

// function createPieceImage(id) {
//     var img = document.createElement("img");
//     img.src = "./img/" + id + ".svg";
//     return img;
// }

// function placePiecesOnBoard() {
//     const SQUARES = document.getElementsByClassName("square");
//     const CHESS_TYPES = ["P", "N", "B", "R", "Q", "K"];

//     for (let i = 0; i < SquareNum; i++) {
//         const square = SQUARES[i];
//         const imageElement = square.querySelector("img");
//         if (imageElement) {
//             square.removeChild(imageElement);
//         }
//         let chessType = squaresU8Value[i];
//         if (chessType === 0) {
//             continue;
//         }
//         const chessColor = chessType < 7 ? "w" : "b";
//         chessType = (chessType % 6) || 6;
//         const chessId = `${chessColor}${CHESS_TYPES[chessType - 1]}`;
//         const img = createPieceImage(chessId);
//         square.appendChild(img);
//     }
// }

function renderLoop() {

    if (selectedSquare) {
        const { row, col } = selectedSquare;
        
        // send row and col to Rust file
        test.set_input(row, col);
        console.log(test.get_input()[0] + " " + test.get_input()[1]);

        selectedSquare = null;
    }

    requestAnimationFrame(renderLoop);
}

// var test = Test.new();
// createBoard();
// placePiecesOnBoard();
// renderLoop();

const wasm = import("chess");
wasm.then(module => {
});