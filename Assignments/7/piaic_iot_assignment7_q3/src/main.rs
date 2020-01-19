/******************************************************************************
* Name: assignment7_q3
*
* Author: Muhammad Osaid
* Roll #: PIAIC58189
* Date: 18 Jan, 2019
* 
*   3. Write a Rust library,
*
*       a. Make a library package
*       b. Make a module with suitable name in library
*       c. Make a sub module with an other name in first module
*       d. Define a simple function in sub module
*       e. make a binary package
*       f. add your library in dependencies in cargo.toml
*       g. use your library in main.rs
*       h. Call that function from fn main
*
*****************************************************************************/
// a. Make a library package 
// Library created with cargo new piaic_lib_q3 command and lib.rs file 
// is created inside the src folder 

// b. Make a module with suitable name in library
// Module and sub module from question 2 is reused and put in For lib.rs

// For c and d please refer in lib.rs file

// e. Make a binary package
// Binary package was created using cargo new piaic_iot_assignment7_q3 --bin

// f. add your library in dependencies in cargo.toml
// Dependencies were added in Cargo.toml file using library's name and path
// [dependencies]
// piaic_lib_a7_q3 = { path = "piaic_lib_a7_q3" }
//
// Library dependency was valided. (commenting dependency gave can't find crate error)


// g. use your library in main.rs
extern crate piaic_lib_a7_q3;

fn main(){

    //Calling simple function created in submodule in library from fn main
    piaic_lib_a7_q3::comm_mod::wireless_comm_mod::drivers();
}
