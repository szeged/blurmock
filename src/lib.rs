extern crate hex;
extern crate core;

macro_rules! make_getter(
    ($function_name: ident, $attr: ident, $ret_type:ty) => {
        pub fn $function_name(&self) -> Result<$ret_type, Box<Error>> {
            let cloned = self.$attr.clone();
            let attr_value = match cloned.lock() {
                Ok(guard) => guard.deref().clone(),
                Err(_) => return Err(Box::from("Could not get the value.")),
            };
            Ok(attr_value)
        }
    };

    ($function_name: ident, $attr: ident) => {
        pub fn $function_name(&self) -> String {
            let cloned = self.$attr.clone();
            let attr_value = match cloned.lock() {
                Ok(guard) => guard.deref().clone(),
                Err(_) => String::new(),
            };
            attr_value
        }
    };

    ($attr_name: ident) => {
        pub fn $attr_name(&self) -> Result<bool, Box<Error>> {
            let cloned = self.$attr_name.clone();
            let attr_value = match cloned.lock() {
                Ok(guard) => guard.deref().clone(),
                Err(_) => return Err(Box::from("Could not get the value.")),
            };
            Ok(attr_value)
        }
    };
);

macro_rules! make_option_getter(
    ($function_name: ident, $attr: ident, $ret_type:ty) => {
        pub fn $function_name(&self) -> Result<$ret_type, Box<Error>> {
            let cloned = self.$attr.clone();
            let attr_value = match cloned.lock() {
                Ok(guard) => guard.deref().clone(),
                Err(_) => return Err(Box::from("Could not get the value.")),
            };
            match attr_value {
                Some(value) => return Ok(value),
                None => return Err(Box::from("Could not get the value.")),
            }
        }
    };
);

macro_rules! make_setter(
    ($function_name: ident, $attr: ident, $attr_type:ty ) => {
        pub fn $function_name(&self, value: $attr_type) -> Result<(), Box<Error>> {
            let cloned = self.$attr.clone();
            let mut value_to_change = match cloned.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Box::from("Could not get the value.")),
            };
            Ok(*value_to_change = value)
        }
    };

    ($function_name: ident, $attr: ident) => {
        pub fn $function_name(&self, value: String) {
            let cloned = self.$attr.clone();
            let mut value_to_change = match cloned.lock() {
                Ok(guard) => guard,
                Err(_) => return (),
            };
            *value_to_change = value
        }
    };
);

pub mod fake_adapter;
pub mod fake_device;
pub mod fake_service;
pub mod fake_characteristic;
pub mod fake_descriptor;
pub mod fake_discovery_session;
