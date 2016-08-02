use fake_characteristic::FakeBluetoothGATTCharacteristic;
use std::cell::{Cell, RefCell};


#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTDescriptor {
    object_path: RefCell<String>,
    uuid: RefCell<String>,
    characteristic: RefCell<FakeBluetoothGATTCharacteristic>,
    value: RefCell<Vec<u8>>,
    flags: RefCell<Vec<String>>,
}

impl FakeBluetoothGATTDescriptor {
    pub fn new(object_path: String,
               uuid: String,
               characteristic: FakeBluetoothGATTCharacteristic,
               value: Vec<u8>,
               flags: Vec<String>)
               -> FakeBluetoothGATTDescriptor {
        FakeBluetoothGATTDescriptor {
            object_path: RefCell::new(object_path),
            uuid: RefCell::new(uuid),
            characteristic: RefCell::new(characteristic),
            value: RefCell::new(value),
            flags: RefCell::new(flags),
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTDescriptor {
        FakeBluetoothGATTDescriptor {
            object_path: RefCell::new(String::new()),
            uuid: RefCell::new(String::new()),
            characteristic: RefCell::new(FakeBluetoothGATTCharacteristic::new_empty()),
            value: RefCell::new(vec![]),
            flags: RefCell::new(vec![]),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_id(&mut self, path: String) {
        *self.object_path.borrow_mut() = path;
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.borrow().clone()
    }

    pub fn set_uuid(&mut self, uuid: String) {
        *self.uuid.borrow_mut() = uuid;
    }

    pub fn get_characteristic(&self) -> FakeBluetoothGATTCharacteristic {
        self.characteristic.borrow().clone()
    }

    pub fn set_characteristic(&mut self, characteristic: FakeBluetoothGATTCharacteristic) {
        *self.characteristic.borrow_mut() = characteristic;
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.value.borrow().clone()
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        *self.value.borrow_mut() = value;
    }

    pub fn get_flags(&self) -> Vec<String> {
        self.flags.borrow().clone()
    }

    pub fn set_flags(&mut self, flags: Vec<String>) {
        *self.flags.borrow_mut() = flags;
    }

    pub fn read_value(&self) -> Vec<u8> {
        self.get_value()
    }

    pub fn write_value(&self, value: Vec<u8>) {
        *self.value.borrow_mut() = value;
    }
}
