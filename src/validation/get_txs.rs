use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::serde::get_u64;
use crate::types::enumeration::Enumeration;
use crate::types::mysql::Order;
use crate::validation::verify::verify_default;

pub fn verify(params: &Map<String, Value>) -> Result<(), ExpectedError> {
    if params.get("txhash").is_some() {
        return if params.get("txhash").unwrap().as_str().is_some() {
            Ok(())
        } else {
            Err(ExpectedError::TypeError(String::from("txhash must be string!")))
        }
    } else {
        verify_default(params, vec![("from_height", "u64"), ("to_height", "u64"), ("order", "string")])?;
        if !Order::valid(params.get("order").unwrap().as_str().unwrap()) {
            return Err(ExpectedError::TypeError(String::from("matched target does not exist! order=[asc, desc]")));
        }
        let to_height = get_u64(params, "to_height").unwrap();
        let from_height = get_u64(params, "from_height").unwrap();
        if to_height < from_height {
            return Err(ExpectedError::InvalidError(format!("to_height must be bigger than from_height! from_height={}, to_height={}", from_height, to_height)));
        }
        if to_height - from_height >= 30 {
            return Err(ExpectedError::InvalidError(format!("height range must be smaller than 30! input_range={}", to_height - from_height)));
        }
    }
    Ok(())
}
