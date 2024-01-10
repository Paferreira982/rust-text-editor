# Rust-Text-Editor

<p align="center">
If you want to read this file in other language, click on the respective flag <br> <br>
  <a href="https://github.com/Paferreira982/rust-text-editor/blob/main/README.pt-br.md">
    <img src="https://img.shields.io/badge/lang-pt--br-green.svg" alt="pt-br">
  </a>
  <a href="https://github.com/Paferreira982/rust-text-editor/blob/main/README.md">
    <img src="https://img.shields.io/badge/lang-en-red.svg" alt="en">
  </a>
</p>

## Description
*Rust-Text-Editor* is a powerful and flexible text editor developed in Rust, designed to provide an efficient and secure editing experience. *Rust-Text-Editor* combines the efficiency of Rust with the familiarity of command mode to create a robust and reliable editing environment.

### Why a Text Editor?
A text editor is an essential tool for any developer. As I use such tools daily, I decided to develop my own text editor to learn and apply low-level concepts, and most importantly, to understand how a text editor works, how all its features are implemented, and how they integrate to create an efficient and reliable editing experience.

### Why Rust?
Rust is a system programming language that prioritizes safety, concurrency and performance. The main reason for my choice of Rust is learning low-level languages. Rust is a system programming language that offers a perfect balance between safety and performance, making it an ideal language to learn and apply low-level concepts.

Additionally, Rust has some features that caught my attention:

>**Memory Safety** <br>
> Rust's ownership system prevents common memory allocation errors, providing safer and more robust code.

>**Error-Free Concurrency** <br>
>  Rust facilitates concurrent programming without the worry of race conditions, ensuring that Rust-Text-Editor is reliable in multi-threaded environments.

>**Performance** <br>
>  Proximity to hardware and the absence of garbage collection allow Rust-Text-Editor to achieve exceptional performance.

Although not utilizing all presented features due to the project's scope, Rust seemed to be the perfect choice for developing an efficient and reliable text editor.

---
## Features
[x] **Text Display:** <small><i>Ability to specify a path to a text file to display its content.</i></small>

[x] **Navigation:** <small><i>Ability to scroll through the content of the text file.</i></small>

[x] **Status Bar:** <small><i>Ability to display information about the current text file, such as the file name, current line number, total number of lines, and file type.</i></small>

[x] **Prompt and Messages:** <small><i>Ability to display messages and capture user commands via a prompt.</i></small>

[x] **Text Editing:** <small><i>Ability to edit the content of a text file, editing existing lines or adding new lines.</i></small>

[x] **Initialization:** <small><i>Ability to initialize the text editor without specifying a text file.</i></small>

[x] **Save File:** <small><i>Ability to save the content of a text file to its original path.</i></small>

[x] **Save As:** <small><i>Ability to save the content of a text file to a new path.</i></small>

[x] **Detect Changes:** <small><i>Ability to detect if the content of a text file has changed since the last save.</i></small>

[x] **Confirm Exit Without Saving:** <small><i>Ability to confirm exiting the text editor without saving the content of the text file.</i></small>

[x] **Text Search:** <small><i>Ability to find a string in the content of a text file.</i></small>

[x] **Navigate in Text Search Mode:** <small><i>Ability to navigate between occurrences of a string in the content of a text file, both forward and backward.</i></small>

[x] **Text Highlighting:** <small><i>Ability to colorize text according to the syntax of the text file.</i></small>

[x] **Rust Support:** <small><i>Ability to colorize text according to the syntax of the Rust language.</i></small>

[x] **Themes** <small><i>Ability to choose between different themes to colorize the text. Avaliable themes: light, dark, neon, matrix, ocean and original.</i></small>

[ ] **Auto Indentation:** <small><i>Ability to automatically indent text according to the syntax of the text file.</i></small>

[ ] **Copy and Paste:** <small><i>Ability to copy and paste text with Ctrl+C and Ctrl+V commands.</i></small>

[ ] **Typescript/Javascript Support:** <small><i>Ability to colorize text according to the syntax of the Typescript language.</i></small>

[ ] **Python Support:** <small><i>Ability to colorize text according to the syntax of the Python language.</i></small>

---

## Installation
To run the project, it is necessary to have Rust installed on your machine. To install Rust, follow the instructions on the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Set up the environment
```bash
git clone https://github.com/Paferreira982/rust-text-editor.git

cd rust-text-editor
```

### Initialize without specifying a text file
```bash
cargo run
```

### Initialize editing an existing file
```bash
cargo run <file-path>
```

## Credits
This text editor was based on a tutorial from [Build Your Own Text Editor in Rust](https://www.flenker.blog/hecto/) developed by [pflenker](https://github.com/pflenker).