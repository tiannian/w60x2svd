use crate::mode::AccessMode;
use crate::utils;
use regex::Regex;
use serde::{Deserialize, Serialize};
use svd_parser::svd::bitrange::{BitRange, BitRangeType};
use svd_parser::svd::enumeratedvalue::{
    EnumeratedValue as SvdEnumeratedValue, EnumeratedValueBuilder,
};
use svd_parser::svd::enumeratedvalues::{EnumeratedValues, EnumeratedValuesBuilder};
use svd_parser::svd::fieldinfo::FieldInfoBuilder;
use svd_parser::svd::Field as SvdField;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldCsv {
    #[serde(rename = "位")]
    pub bit: String,
    #[serde(rename = "访问")]
    pub access: String,
    #[serde(rename = "操作说明")]
    pub description: String,
    #[serde(rename = "复位值")]
    pub reset: String,
}

impl FieldCsv {
    pub fn to_field(self) -> Field {
        let access = utils::from_string_to_access(&self.access);
        let re = Regex::new(r"([a-z0-9_A-Z]*)\n?([\s\S]*)").unwrap();
        let caps = re.captures(&self.description).unwrap();
        let name = caps.get(1).unwrap().as_str().to_lowercase();
        let other = caps.get(2).unwrap().as_str();
        let splited_desc = other.split('\n');
        let mut enumerated_value = EnumeratedValue {
            name: name.clone(),
            usage: access.clone(),
            values: Vec::new(),
        };
        println!("field name: {}", name);

        let re = Regex::new(r"\[(\d{0,2})(\s?:\s?\d{0,2})?\]").unwrap();
        let caps = re.captures(&self.bit).unwrap();
        let begin = caps.get(1).unwrap().as_str();
        let begin_num: u32 = begin.parse().unwrap();
        let mut offset = 0;
        // println!("{}", name);
        let width = if let Some(end) = caps.get(2) {
            let end_str = &end.as_str().replace(":", "").replace(" ", "");
            offset = end_str.parse().unwrap();
            begin_num - offset + 1
        } else {
            1
        };

        let mut mdescription = String::new();

        for desc in splited_desc {
            // TODO: deal b or h.
            let re = Regex::new(r"\d’([bh])([0-9A-Fa-f]+)([\s\S]*)").unwrap();
            if let Some(caps) = re.captures(desc) {
                let flag = caps.get(1).unwrap().as_str();
                let value_str = caps.get(2).unwrap().as_str();
                let description = caps.get(3).unwrap().as_str().replace("：", "");
                // println!("{}", value_str);
                let mut value = 0;
                if flag == "b" {
                    value = u32::from_str_radix(value_str, 2).unwrap();
                } else if flag == "h" {
                    value = u32::from_str_radix(value_str, 16).unwrap();
                }
                let v = Value {
                    name: String::from(name.clone()) + "_" + value_str,
                    description: Some(String::from(description.trim())),
                    value,
                    default: false,
                };
                enumerated_value.values.push(v);
            } else {
                mdescription += desc
            }
        }

        let mut reset = None;
        if self.reset != "" {
            let re = Regex::new(r"\d’[bh]([0-9A-Fa-f]+)").unwrap();
            if let Some(caps) = re.captures(&self.reset) {
                let value_str = caps.get(1).unwrap().as_str();
                reset = utils::from_radix_to_u32(&(String::from("0x") + value_str))
            }
        }

        if enumerated_value.values.len() == 0 {
            Field {
                name,
                offset,
                width,
                access,
                description: Some(String::from(mdescription.trim())),
                enumerated_value: None,
                reset,
            }
        } else {
            Field {
                name,
                offset,
                width,
                access,
                description: Some(String::from(mdescription.trim())),
                enumerated_value: Some(enumerated_value),
                reset,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: u32,
    pub default: bool,
}

impl Value {
    pub fn to_svd(self, _reset: Option<u32>) -> SvdEnumeratedValue {
        let builder = EnumeratedValueBuilder::default();
        // let rst = if let Some(r) = reset {
        //     self.value == r
        // } else {
        //     false
        // };
        builder
            .name(self.name)
            .description(self.description)
            .value(Some(self.value.into()))
            // .is_default(Some(rst))
            .build()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumeratedValue {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<AccessMode>,
    pub values: Vec<Value>,
}

impl EnumeratedValue {
    pub fn to_svd(self, reset: Option<u32>) -> EnumeratedValues {
        let builder = EnumeratedValuesBuilder::default();
        let mut values = Vec::new();
        for v in self.values {
            values.push(v.to_svd(reset));
        }
        builder
            .name(Some(self.name))
            .usage(Some(self.usage.unwrap().into()))
            .values(values)
            .build()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub name: String,
    pub offset: u32,
    pub width: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumerated_value: Option<EnumeratedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<u32>,
}

impl Field {
    pub fn to_svd(self) -> SvdField {
        let mut builder = FieldInfoBuilder::default();
        // println!("{}, {}, {}", self.name, self.offset, self.width);
        builder = builder
            .name(self.name)
            .bit_range(BitRange {
                offset: self.offset,
                width: self.width,
                range_type: BitRangeType::OffsetWidth,
            })
            .derived_from(None)
            .description(self.description);
        if let Some(access) = self.access {
            builder = builder.access(Some(access.into()));
        }
        if let Some(enumerated_value) = self.enumerated_value {
            builder = builder.enumerated_values(vec![enumerated_value.to_svd(self.reset)]);
        }
        SvdField::Single(builder.build().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_csv_de() {
        let mut rdr = csv::Reader::from_path("./csvs/timer/I2S_CLK_CTRL.csv").unwrap();
        for r in rdr.deserialize() {
            let record: FieldCsv = r.unwrap();
            let _field = record.to_field();
        }
    }
}
