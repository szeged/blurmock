use core::ops::Deref;
use fake_characteristic::FakeBluetoothGATTCharacteristic;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTDescriptor {
    id: Arc<Mutex<String>>,
    uuid: Arc<Mutex<String>>,
    characteristic: Arc<FakeBluetoothGATTCharacteristic>,
    value: Arc<Mutex<Vec<u8>>>,
    flags: Arc<Mutex<Vec<String>>>,
}

impl FakeBluetoothGATTDescriptor {
    pub fn new(id: String,
               uuid: String,
               characteristic: Arc<FakeBluetoothGATTCharacteristic>,
               value: Vec<u8>,
               flags: Vec<String>)
               -> Arc<FakeBluetoothGATTDescriptor> {
        if let Ok(existing_descriptor) = characteristic.get_gatt_descriptor(id.clone()) {
            return existing_descriptor;
        }
        let descriptor = Arc::new(FakeBluetoothGATTDescriptor {
            id: Arc::new(Mutex::new(id)),
            uuid: Arc::new(Mutex::new(uuid)),
            characteristic: characteristic.clone(),
            value: Arc::new(Mutex::new(value)),
            flags: Arc::new(Mutex::new(flags)),
        });
        let _ = characteristic.add_descriptor(descriptor.clone());
        descriptor
    }

    pub fn new_empty(characteristic: Arc<FakeBluetoothGATTCharacteristic>,
                     descriptor_id: String)
                     -> Arc<FakeBluetoothGATTDescriptor> {
        FakeBluetoothGATTDescriptor::new(
            /*id*/ descriptor_id,
            /*uuid*/ String::new(),
            /*characteristic*/ characteristic,
            /*value*/ vec!(),
            /*flags*/ vec!(),
        )
    }

    make_getter!(get_id, id);

    make_setter!(set_id, id);

    make_getter!(get_uuid, uuid, String);

    make_setter!(set_uuid, uuid, String);

    make_getter!(get_value, value, Vec<u8>);

    make_setter!(set_value, value, Vec<u8>);

    make_getter!(get_flags, flags, Vec<String>);

    make_setter!(set_flags, flags, Vec<String>);

    pub fn get_characteristic(&self) -> Result<Arc<FakeBluetoothGATTCharacteristic>, Box<Error>> {
        Ok(self.characteristic.clone())
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_value()
    }

    pub fn write_value(&self, value: Vec<u8>) -> Result<(), Box<Error>> {
        self.set_value(value)
    }
}
