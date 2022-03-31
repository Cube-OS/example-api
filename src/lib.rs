// This file serves as an example, how to write an API in the Cube-OS framework
// 
// At the bottom of the file you can find examples of how to read and write via I2C and UART

// Dependencies
use failure::{Fail};
use serde::*;
use juniper::*;
use std::convert::From;
use cubeos_error::Error;

// Dependencies for UART and I2C
// use rust_i2c::{Command, Connection};
// use rust_uart::{Connection, SerialStream};

// Example Error type
// covers all Errors possible within your API, Service and Payload
#[derive(Debug, Fail, Clone, PartialEq)]
pub enum ExampleError {
    ///Example error
    #[fail(display = "Example error")]
    Err,
}
// Implementation of Conversion of Example Error type 
// to cubeos_error::Error (Error type that gets returned to GND)
// 
// cubeos-error::Error implements conversion for the following standard errors:
// failure::Error -> cubeos_error::Error::Failure(String)
// std::io::Error -> cubeos_error::Error::Io(u8)
// Infallible -> cubeos_error::Error::Infallible
// bincode::Error -> cubeos_error::Error::Bincode(u8)
// PoisonError<MutexGuard<'a,T>> -> cubeos_error::Error::PoisonError
// 
// Any Errors in ExampleError must be converted to cubeos_error::Error::ServiceError(u8)
impl From<ExampleError> for Error {
    fn from(e: ExampleError) -> cubeos_error::Error {
        match e {
            ExampleError::Err => cubeos_error::Error::ServiceError(0),
        }
    }
}

// Example of Result Type used in the API
pub type ExampleResult<T> = Result<T,ExampleError>;

// Example of an Enum
// Enums can be used as Input (e.g. to choose a telemetry item) or 
// Output (e.g to show the state of a device (e.g. ON,OFF,IDLE,etc.))
#[derive(Serialize,Deserialize,GraphQLEnum)]
pub enum ExampleEnum {
    Number,
    String,
    Boolean,
    All,
}

// Example of an Input/Output Struct
// It is necessary to also define a GraphQL equivalent for input structs
// (see example-service/graphql.rs)
#[derive(Serialize,Deserialize)]
pub struct ExampleInput {
    pub in_no: u16,
    pub in_str: String,
    pub in_bool: bool,
}
#[derive(Serialize,Deserialize)]
pub struct ExampleOutput {
    pub out_no: Option<u16>,
    pub out_str: Option<String>,
    pub out_bool: Option<bool>,
}

// Example of Struct containing the functions to connect to the payload
#[derive(Serialize,Deserialize)]
pub struct ExampleStruct {
    // Connection field describing the connection to I2C or UART
    // connection: Connection,
    // Buffer needed for UART connections
    // buffer: RefCell<Vec<u8>>
    ex_no: u16,
    ex_str: String,
    ex_bool: bool,
}
impl ExampleStruct {
    // basic function to initialise an instance of the ExampleStruct
    pub fn new() -> Self {
        Self{
            ex_no: 0u16,
            ex_str: "".to_string(),
            ex_bool: false,
        }
    }

    // examples of get and set functions that use the previously defined
    // Enum and Structs as In-/Output
    pub fn get(&self, g: ExampleEnum) -> ExampleResult<ExampleOutput> {
        match g {
            ExampleEnum::Number => Ok(ExampleOutput{
                out_no: Some(self.ex_no),
                out_str: None,
                out_bool: None,
            }),
            ExampleEnum::String => Ok(ExampleOutput{
                out_no: None,
                out_str: Some(String::from(&self.ex_str)),
                out_bool: None,
            }),
            ExampleEnum::Boolean => Ok(ExampleOutput{
                out_no: None,
                out_str: None,
                out_bool: Some(self.ex_bool),
            }),
            ExampleEnum::All => self.get_all()
        }
    }

    fn get_all(&self) -> ExampleResult<ExampleOutput> {
        Ok(ExampleOutput{
            out_no: Some(self.ex_no),
            out_str: Some(String::from(&self.ex_str)),
            out_bool: Some(self.ex_bool),
        })
    }

    pub fn set(&mut self, s: ExampleInput) -> ExampleResult<()> {
        self.ex_no = s.in_no;
        self.ex_str = s.in_str;
        self.ex_bool = s.in_bool;

        Ok(())
    }

    // I2C Example Transfer (Write-Read)
    // This function serves as an example how to implement a write-read to payload via I2C
    // This is the most common function used for commanding that gives direct feedback
    // The examples for Write and Read are given below, but are 
    //
    // The I2C transfer function has the structure:
    // transfer(&self, command: Command, rx_len: usize, delay: Duration)
    // 
    // pub fn i2c_transfer(&self, i: ExampleInput) -> ExampleResult<Output> {
    //     let cmd: u8 = i.in_no as u8;
    //     let rx_len = 10;
    //     let delay = Duration::from_millis(50);

    //     if cmd != 0 {
    //         let data: Vec<u8> = Vec::new();
    //         data.push(i.in_str.to_vec());
    //         let command = Command{cmd, data};

    //         match self.connection.transfer(command, rx_len, delay) {
    //             Ok(x) => Ok(ExampleOutput{
    //                     out_no: x as u8,
    //                     out_str: "".to_string(),
    //                     out_bool: true,
    //                 }),
    //             Err(_) => Err(ExampleError::Err),
    //         }
    //     }
    // }

    // I2C Example Write
    // This function serves as an example how to write a payload via I2C
    //
    // The I2C write function has the structure:
    // write(&self, command: Command)
    // 
    // pub fn i2c_write(&self, i: ExampleInput) -> ExampleResult<()> {
    //     let cmd: u8 = i.in_no as u8;

    //     if cmd != 0 {
    //         let data: Vec<u8> = Vec::new();
    //         data.push(i.in_str.to_vec());
    //         let command = Command{cmd, data};

    //         match self.connection.write(command) {
    //             Ok(()) => Ok(()),
    //             Err(_) => Err(ExampleError::Err),
    //         }
    //     }
    // }
    
    // I2C Example Read
    // This function serves as an example how to read from a payload via I2C
    //
    // The I2C read function has the structure:
    // read(&self, command: Command, rx_len: usize)
    // 
    // pub fn i2c_read(&self, cmd: Command) -> ExampleResult<ExampleOutput> {       
    //     let rx_len: usize = 10;
    //     match self.connection.read(cmd.cmd, rx_len) {
    //         Ok(x) => Ok(ExampleOutput{
    //                 out_no: x as u8,
    //                 out_str: "".to_string(),
    //                 out_bool: true,
    //             }),
    //         Err(_) => Err(ExampleError::Err),
    //     }                
    // }

    
    // UART Examples
    // This function serves as an example how to communicate with a payload via UART
    // 
    // The UART read function has the structure:
    // read(&self, len: usize, timeout: Duration)
    // 
    // pub fn uart_read(&self) -> ExampleResult<ExampleOutput> {
    //     let mut buffer = self.buffer.borrow_mut();
    //     // Reads 1 byte, with a timeout of 1ms
    //     while let Ok(mut buf) = self.connection.read(1, Duration::from_millis(1)) {
    //         buffer.append(&mut buf);
    //         if buffer.len() > 4096 {
    //             break;
    //         }
    //     }
    //     Ok(ExampleOutput{
    //         out_no: buffer[0] as u16,
    //         out_str: buffer[1..].to_string(),
    //         out_bool: true,
    //     })
    // }
    // 
    // This function serves as an example how to write to a payload via UART
    // 
    // The UART write function has the structure:
    // write(&self, data: &[u8])
    // 
    // pub fn uart_write(&self, data: &[u8]) -> ExampleResult<()> {
    //     self.connection.write(data)
    // }
    // 
    // 
}