/******************************************************************************
* Name: assignment7_q1
*
* Author: Muhammad Osaid
* Roll #: PIAIC58189
* Date: 18 Jan, 2020
* 
*   1. Write a Rust Program,
*       a. Make a module with suitable name in main.rs
*       b. Make a sub module with an other name in first module
*       d. Define a simple function in sub module
*       e. Call that function from fn main
* 
*****************************************************************************/

fn main(){

	// module with suitable name
	pub mod comm_mod{

		// Sub module in first module
		pub mod serial_comm_mod{

			//Simple function in sub module
			pub fn drivers() {
            	println!("Protocols to support. 1. SPI, 2. UART, 3. I2C");
			}
		}
	}

	// Calling sub module's function from main.
	comm_mod::serial_comm_mod::drivers();
	
}
