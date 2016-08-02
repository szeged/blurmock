use fake_characteristic::FakeBluetoothGATTCharacteristic;
use fake_device::FakeBluetoothDevice;
use std::cell::{Cell, RefCell};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    object_path: RefCell<String>,
    device: RefCell<FakeBluetoothDevice>,
    gattCharacteristics: RefCell<Vec<FakeBluetoothGATTCharacteristic>>,
    isPrimary: Cell<bool>,
    includedServices: RefCell<Vec<FakeBluetoothGATTService>>,
    uuid: RefCell<String>,
}

impl FakeBluetoothGATTService {
    pub fn new(object_path: String,
               device: FakeBluetoothDevice,
               gattCharacteristics: Vec<FakeBluetoothGATTCharacteristic>,
               isPrimary: bool,
               includedServices: Vec<FakeBluetoothGATTService>,
               uuid: String)
               -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: RefCell::new(object_path),
            device: RefCell::new(device),
            gattCharacteristics: RefCell::new(gattCharacteristics),
            isPrimary: Cell::new(isPrimary),
            includedServices: RefCell::new(includedServices),
            uuid: RefCell::new(uuid),
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: RefCell::new(String::new()),
            device: RefCell::new(FakeBluetoothDevice::new_empty()),
            gattCharacteristics: RefCell::new(vec![]),
            isPrimary: Cell::new(false),
            includedServices: RefCell::new(vec![]),
            uuid: RefCell::new(String::new()),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_id(&mut self, path: String) {
        *self.object_path.borrow_mut() = path;
    }

    pub fn get_device(&self) -> FakeBluetoothDevice {
        self.device.borrow().clone()
    }

    pub fn set_device(&mut self, device: FakeBluetoothDevice) {
        *self.device.borrow_mut() = device;
    }

    pub fn get_gatt_characteristics(&self) -> Vec<FakeBluetoothGATTCharacteristic> {
        self.gattCharacteristics.borrow().clone()
    }

    pub fn set_gatt_characteristics(&mut self, characteristics: Vec<FakeBluetoothGATTCharacteristic>) {
        *self.gattCharacteristics.borrow_mut() = characteristics;
    }

    pub fn is_primary(&self) -> bool {
        self.isPrimary.get()
    }

    pub fn set_is_primary(&mut self, value: bool) {
        self.isPrimary.set(value);
    }

    pub fn get_included_services(&self) -> Vec<FakeBluetoothGATTService> {
        self.includedServices.borrow().clone()
    }

    pub fn set_included_services(&mut self, includedServices: Vec<FakeBluetoothGATTService>) {
        *self.includedServices.borrow_mut() = includedServices;
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.borrow().clone()
    }

    pub fn set_uuid(&mut self, uuid: String) {
        *self.uuid.borrow_mut() = uuid;
    }
}
