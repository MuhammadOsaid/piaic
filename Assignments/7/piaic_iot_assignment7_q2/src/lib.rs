/******************************************************************************
* Name: assignment7_q2_library_file
*
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

//Make a library (lib.rs) alongwith main.rs
// Since there is no main function, cargo will build this crate as library


//Make a module with suitable name in library
pub mod comm_mod{

	// Make a sub module with an other name in first module
	pub mod wired_comm_mod{

	    //Simple function in sub module
	    pub fn drivers() {
              	println!("Protocols to support. \r\n\t1. SPI \r\n\t2. UART \r\n\t3. I2C");
	    }
	}

	// Relevant 2nd Sub module in first module
	pub mod wireless_comm_mod{

	    //Simple function in 2nd sub module
	    pub fn drivers() {
              	println!("Protocols to support. \r\n\t1. WiFi \r\n\t2. Bluetooth \r\n\t3. Infrared");
	    }
	}
}
