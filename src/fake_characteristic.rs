use fake_descriptor::FakeBluetoothGATTDescriptor;
use fake_service::FakeBluetoothGATTService;
use std::cell::{Cell, RefCell};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTCharacteristic {
    object_path: RefCell<String>,
    uuid: RefCell<String>,
    service: RefCell<FakeBluetoothGATTService>,
    value: RefCell<Vec<u8>>,
    isNotifying: Cell<bool>,
    flags: RefCell<Vec<String>>,
    descriptors: RefCell<Vec<FakeBluetoothGATTDescriptor>>,
}

impl FakeBluetoothGATTCharacteristic {
    pub fn new(object_path: String,
               uuid: String,
		       service: FakeBluetoothGATTService,
		       value: Vec<u8>,
		       isNotifying: bool,
		       flags: Vec<String>,
		       descriptors: Vec<FakeBluetoothGATTDescriptor>)
               -> FakeBluetoothGATTCharacteristic {
        FakeBluetoothGATTCharacteristic {
            object_path: RefCell::new(object_path),
            uuid: RefCell::new(uuid),
            service: RefCell::new(service),
            value: RefCell::new(value),
            isNotifying: Cell::new(isNotifying),
            flags: RefCell::new(flags),
            descriptors: RefCell::new(descriptors),
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTCharacteristic {
        FakeBluetoothGATTCharacteristic {
            object_path: RefCell::new(String::new()),
            uuid: RefCell::new(String::new()),
            service: RefCell::new(FakeBluetoothGATTService::new_empty()),
            value: RefCell::new(vec![]),
            isNotifying: Cell::new(false),
            flags: RefCell::new(vec![]),
            descriptors: RefCell::new(vec![]),
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

    pub fn get_service(&self) -> FakeBluetoothGATTService {
        self.service.borrow().clone()
    }

    pub fn set_service(&mut self, service: FakeBluetoothGATTService) {
        *self.service.borrow_mut() = service;
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.value.borrow().clone()
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        *self.value.borrow_mut() = value;
    }

    pub fn is_notifying(&self) -> bool {
        self.isNotifying.get()
    }

    pub fn set_is_notifying(&mut self, value: bool) {
        self.isNotifying.set(value);
    }

    pub fn get_flags(&self) -> Vec<String> {
        self.flags.borrow().clone()
    }

    pub fn set_flags(&mut self, flags: Vec<String>) {
        *self.flags.borrow_mut() = flags;
    }

    pub fn get_descriptors(&self) -> Vec<FakeBluetoothGATTDescriptor> {
        self.descriptors.borrow().clone()
    }

    pub fn set_descriptors(&mut self, descriptors: Vec<FakeBluetoothGATTDescriptor>) {
        *self.descriptors.borrow_mut() = descriptors;
    }

    pub fn read_value(&self) -> Vec<u8> {
    	self.get_value()
    }

    pub fn write_value(&self, value: Vec<u8>) -> Result<(),()> {
    	*self.value.borrow_mut() = value;
    	Ok(())
    }
}