/******************************************************************************
* Name: assignment7_q3
* File: /piaic_lib_a7_q3/src/lib.rs
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

// b. Make a module with suitable name in library

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
