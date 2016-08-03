use fake_characteristic::FakeBluetoothGATTCharacteristic;
use fake_device::FakeBluetoothDevice;
use std::cell::{Cell, RefCell};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    object_path: RefCell<String>,
    device: RefCell<FakeBluetoothDevice>,
    gatt_characteristics: RefCell<Vec<FakeBluetoothGATTCharacteristic>>,
    is_primary: Cell<bool>,
    included_services: RefCell<Vec<FakeBluetoothGATTService>>,
    uuid: RefCell<String>,
}

impl FakeBluetoothGATTService {
    pub fn new(object_path: String,
               device: FakeBluetoothDevice,
               gatt_characteristics: Vec<FakeBluetoothGATTCharacteristic>,
               is_primary: bool,
               included_services: Vec<FakeBluetoothGATTService>,
               uuid: String)
               -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: RefCell::new(object_path),
            device: RefCell::new(device),
            gatt_characteristics: RefCell::new(gatt_characteristics),
            is_primary: Cell::new(is_primary),
            included_services: RefCell::new(included_services),
            uuid: RefCell::new(uuid),
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: RefCell::new(String::new()),
            device: RefCell::new(FakeBluetoothDevice::new_empty()),
            gatt_characteristics: RefCell::new(vec![]),
            is_primary: Cell::new(false),
            included_services: RefCell::new(vec![]),
            uuid: RefCell::new(String::new()),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_id(&mut self, path: String) {
        *self.object_path.borrow_mut() = path;
    }

    pub fn get_device(&self) -> Result<FakeBluetoothDevice, Box<Error>> {
        Ok(self.device.borrow().clone())
    }

    pub fn set_device(&mut self, device: FakeBluetoothDevice) {
        *self.device.borrow_mut() = device;
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<FakeBluetoothGATTCharacteristic>, Box<Error>> {
        Ok(self.gatt_characteristics.borrow().clone())
    }

    pub fn set_gatt_characteristics(&mut self, characteristics: Vec<FakeBluetoothGATTCharacteristic>) {
        *self.gatt_characteristics.borrow_mut() = characteristics;
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_primary.get())
    }

    pub fn set_is_primary(&mut self, value: bool) {
        self.is_primary.set(value);
    }

    pub fn get_included_services(&self) -> Result<Vec<FakeBluetoothGATTService>, Box<Error>> {
        Ok(self.included_services.borrow().clone())
    }

    pub fn set_included_services(&mut self, included_services: Vec<FakeBluetoothGATTService>) {
        *self.included_services.borrow_mut() = included_services;
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Ok(self.uuid.borrow().clone())
    }

    pub fn set_uuid(&mut self, uuid: String) {
        *self.uuid.borrow_mut() = uuid;
    }
}
