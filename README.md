# <p align="center"> ♕ Chess </p>

<div align="center">

     ██████╗██╗  ██╗███████╗███████╗███████╗
    ██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝
    ██║     ███████║█████╗  ███████╗███████╗
    ██║     ██╔══██║██╔══╝  ╚════██║╚════██║
    ╚██████╗██║  ██║███████╗███████║███████║
     ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝


This project is a chess application that allows users to play chess online. It is built using Rust with WebAssembly.  
Play the game at [here](https://mikej.space/chess/)

![chess demo](https://raw.githubusercontent.com/WR104/chess/main/www/img/demo.JPG)

</div>

## Introduction

The chess project is an interactive web-based chess game that enables users to play against each other online. The application provides a visually appealing chessboard interface with intuitive controls, allowing players to make moves and track the game's progress.

## Technologies

The chess project utilizes the following technologies:

- Rust: The core chess logic and game engine are implemented in Rust using the `chess_engine` crate, providing efficient and reliable game processing.
- WebAssembly (WASM): The Rust code is compiled into WebAssembly to run in the browser environment.
- JavaScript: The interactive features and user interface enhancements are implemented using JavaScript.
- HTML: The web pages and layout structure of the application are defined using HTML.
- CSS: The visual styling and presentation of the application are achieved through CSS.

## Getting Started

### Prerequisites

Before we get started, make sure you have the following tools installed:

- Rust (https://www.rust-lang.org/tools/install)
- Node.js (https://nodejs.org/en/download/)
- wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)

To run the chess project locally on your machine, follow these steps:

1. Clone the repository: `git clone https://github.com/WR104/chess.git`
2. Navigate to the project directory: `cd chess`
3. Build the wasm files: `wasm-pack build`
4. Navigate to web dictionary: `cd www`
5. Install dependencies: `npm install`
6. Start the local development server: `npm run start`
7. Open your web browser and visit `http://localhost:8080` to access the chess application.


