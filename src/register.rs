use crate::mode::AccessMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use svd_parser::{
    access::Access, registerinfo::RegisterInfoBuilder, Register as SvdRegister, RegisterCluster,
};
use v_eval::{Eval, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: Option<String>,   // Notice: This file is required in svd file.
    pub offset: Option<String>, // Notice: This file is required in svd file.
    pub description: Option<String>,
    pub mode: Option<AccessMode>,
    pub reset: Option<String>,
    pub fields: Option<String>,
    // Need svd_parser support.
    pub data_type: Option<String>,
    pub expressions: Option<HashMap<String, String>>,
}

impl Register {
    fn eval(&mut self, args: &HashMap<String, String>) {
        // init eval context.
        let mut context = Eval::default();
        for (k, v) in args {
            context = context.insert(k.as_str(), v.as_str()).unwrap();
        }

        if let Some(expressions) = &self.expressions {
            if let Some(exp) = expressions.get("offset") {
                if let Value::Int(v) = context.eval(exp).unwrap() {
                    let s = format!("{:#x}", v);
                    self.offset = Some(s.to_string());
                }
            }

            if let Some(exp) = expressions.get("name") {
                if let Value::Str(s) = context.eval(exp).unwrap() {
                    self.name = Some(s);
                }
            }

            if let Some(exp) = expressions.get("description") {
                if let Value::Str(s) = context.eval(exp).unwrap() {
                    self.description = Some(s);
                }
            }
        }
    }

    pub fn get_svd(mut self, args: &HashMap<String, String>) -> RegisterCluster {
        self.eval(args);
        println!("{:?}", self);
        let offset = u32::from_str_radix(&self.offset.unwrap()[2..], 16).unwrap();
        let reset_value = match self.reset {
            Some(v) => Some(u32::from_str_radix(&v[2..], 16).unwrap()),
            None => None,
        };
        let access = match self.mode {
            Some(v) => match v {
                AccessMode::Unknown => None,
                AccessMode::RW => Some(Access::ReadWrite),
                AccessMode::RO => Some(Access::ReadOnly),
                AccessMode::WO => Some(Access::WriteOnly),
            },
            None => None,
        };

        // TODO: Load registers.

        let info = RegisterInfoBuilder::default()
            .name(self.name.unwrap())
            .address_offset(offset)
            .description(self.description)
            .reset_value(reset_value)
            .access(access)
            .build()
            .unwrap();
        RegisterCluster::Register(SvdRegister::Single(info))
    }
}

#[cfg(test)]
mod tests {
    use super::Register;
    use std::fs::File;
    #[test]
    fn load_json() {
        let file = File::open("svdjson/dma/registers.json").unwrap();
        let registers: Vec<Register> = serde_json::from_reader(file).unwrap();
        println!("{:?}", registers);
    }
}
