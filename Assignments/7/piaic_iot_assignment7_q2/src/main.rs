/******************************************************************************
* Name: assignment7_q2
* Author: Muhammad Osaid
* Roll #: PIAIC58189
* Date: 18 Jan, 2020
* 
*   2. Write a Rust Program,
*       a. Make a library (lib.rs) alongwith main.rs
*       b. Make a module with suitable name in library
*       c. Make a sub module with an other name in first module
*       d. Define a simple function in sub module
*       e. use that library in main.rs
*       f. Call that function from fn main
* 
*****************************************************************************/

extern crate piaic_iot_assignment7_q2;

fn main(){
	println!("\r\nInvoking drivers function from submodule wired_comm_mod.\r\n");
	piaic_iot_assignment7_q2::comm_mod::wired_comm_mod::drivers();
}
