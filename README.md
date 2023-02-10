# Page Replacement Algorithm Simulator

A program written in Rust that simulates the behavior of three page replacement algorithms:
- Least Recently Used (LRU)
- First In First Out (FIFO)
- Virtual Memory System (VMS)

## Usage
The program reads trace files containing a list of memory accesses and simulates how these algorithms would handle the memory. The program outputs the number of page faults for each algorithm and trace file combination. 

### Input
The program takes as input a list of trace files and a list of frame sizes. 

### Output
The program outputs the number of page faults for each combination of algorithm and trace file.

## Installation
To run this program, you will need to have the latest version of Rust installed. You can follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust. 

Once you have Rust installed, clone this repository and navigate to the project directory. Then, run the following command:

`cargo run`


## File Structure
The program consists of the following files:
- `main.rs`: The entry point of the program that sets up the input and outputs the results.
- `substitute.rs`: A module that implements the page replacement algorithms.

## Contributing
This project is open to contributions. Feel free to submit a pull request if you have any improvements or fixes.