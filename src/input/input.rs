use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use num_bigint::BigInt;

use serde_json::Value;

pub struct R1CSInputs {
    pub inputs: HashMap<String, Vec<BigInt>>,
}

impl R1CSInputs {
    pub fn new<R: Read>(reader: R) -> Option<R1CSInputs> {
        let key_values: HashMap<String, Value> = serde_json::from_reader(reader).unwrap();

        let mut r1cs_inputs = R1CSInputs { inputs: Default::default() };
        for (key, value) in key_values.iter() {
            let vec = match value {
                Value::Number(n) => Some(vec![BigInt::from(n.as_u64().unwrap())]),
                Value::String(s) => Some(vec![BigInt::from_str(s).unwrap()]),
                Value::Array(v) => Some(
                    v.into_iter().map(|s|
                        BigInt::from_str(s.to_string().as_str()).unwrap()
                    ).collect()
                ),
                _ => None
            }?;
            r1cs_inputs.inputs.insert(key.to_string(), vec);
        }
        Some(r1cs_inputs)
    }
}