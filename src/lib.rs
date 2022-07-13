//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)
// 
// This file is as an example, how to write an API in the Cube-OS framework
// 

// Dependencies
use failure::{Fail};
use serde::*;
use juniper::*;
use std::convert::From;
use cubeos_error::Error;

mod example;

// Make everything in example.rs public
pub use crate::example::*;

// Example Error type
// covers all Errors possible within your API, Service and Payload
#[derive(Debug, Fail, Clone, PartialEq)]
pub enum ExampleError {
    /// None
    #[fail(display = "None")]
    None,
    /// Example error
    #[fail(display = "Example error")]
    Err,
    /// Set Error
    #[fail(display = "Set error, only accepts ZERO or ONE")]
    SetErr,
    /// I2C Error
    #[fail(display = "I2C Error")]
    I2CError(std::io::ErrorKind),
    /// I2C Set Error
    #[fail(display = "I2C Set Error")]
    I2CSet,
    /// UART Error
    #[fail(display = "UART Error")]
    UARTError(rust_uart::UartError),
}
/// Implementation of Conversion of Example Error type 
/// to cubeos_error::Error (Error type that gets returned to GND)
/// 
/// cubeos-error::Error implements conversion for the following standard errors:
/// failure::Error -> cubeos_error::Error::Failure(String)
/// std::io::Error -> cubeos_error::Error::Io(u8)
/// Infallible -> cubeos_error::Error::Infallible
/// bincode::Error -> cubeos_error::Error::Bincode(u8)
/// PoisonError<MutexGuard<'a,T>> -> cubeos_error::Error::PoisonError
/// 
/// Any Errors in ExampleError must be converted to cubeos_error::Error::ServiceError(u8)
impl From<ExampleError> for Error {
    fn from(e: ExampleError) -> cubeos_error::Error {
        match e {
            ExampleError::None => cubeos_error::Error::ServiceError(0),
            ExampleError::Err => cubeos_error::Error::ServiceError(1),
            ExampleError::SetErr => cubeos_error::Error::ServiceError(2),
            ExampleError::I2CError(io) => cubeos_error::Error::from(io),
            ExampleError::I2CSet => cubeos_error::Error::ServiceError(3),
            ExampleError::UARTError(io) => cubeos_error::Error::from(io),
        }
    }
}
impl From<rust_uart::UartError> for ExampleError {
    fn from(e: rust_uart::UartError) -> ExampleError {
        ExampleError::UARTError(e)
    }
}

// Example of Result Type used in the API
pub type ExampleResult<T> = Result<T,ExampleError>;

// Example of an Enum
// Enums can be used as Input (e.g. to choose a telemetry item) or 
// Output (e.g to show the state of a device (e.g. ON,OFF,IDLE,etc.))
#[derive(Serialize,Deserialize,GraphQLEnum,Copy,Clone)]
pub enum ExampleEnum {
    Zero,
    One,
    All,
}

// Example of an Input/Output Struct
// It is necessary to also define a GraphQL equivalent for input structs
// (see example-service/graphql.rs)
#[derive(Serialize,Deserialize)]
pub struct ExampleInput {
    pub in_no: u16,
    pub in_no1: u32,
    pub in_no2: u16,
    pub in_str: String,
    pub in_bool: bool,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ExampleOutput {
    pub out_no: Vec<u16>,
    pub out_str: String,
    pub out_bool: Vec<bool>,
}