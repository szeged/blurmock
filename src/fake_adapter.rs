use fake_device::FakeBluetoothDevice;
use fake_discovery_session::FakeBluetoothDiscoverySession;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FakeBluetoothAdapter {
    object_path: RefCell<String>,
    isPresent: Cell<bool>,
    isPowered: Cell<bool>,
    canStartDiscovery: Cell<bool>,
    canStopDiscovery: Cell<bool>,
    devices: RefCell<Vec<FakeBluetoothDevice>>,
    addatas: RefCell<Vec<String>>,
    address: RefCell<String>,
    name: RefCell<String>,
}

impl FakeBluetoothAdapter {
    pub fn new(object_path: String,
               isPresent: bool,
               isPowered: bool,
               canStartDiscovery: bool,
               canStopDiscovery: bool,
               devices: Vec<FakeBluetoothDevice>,
               addatas: Vec<String>,
               address: String,
               name: String) 
               ->FakeBluetoothAdapter{
        FakeBluetoothAdapter{
            object_path: RefCell::new(object_path),
            isPresent: Cell::new(isPresent),
            isPowered: Cell::new(isPowered),
            canStartDiscovery: Cell::new(canStartDiscovery),
            canStopDiscovery: Cell::new(canStopDiscovery),
            devices: RefCell::new(devices),
            addatas: RefCell::new(addatas),
            address: RefCell::new(address),
            name: RefCell::new(name),
        }
    }

    pub fn new_empty() -> FakeBluetoothAdapter {
        FakeBluetoothAdapter {
            object_path: RefCell::new(String::new()),
            isPresent: Cell::new(false),
            isPowered: Cell::new(false),
            canStartDiscovery: Cell::new(false),
            canStopDiscovery: Cell::new(false),
            devices: RefCell::new(vec![]),
            addatas: RefCell::new(vec![]),
            address: RefCell::new(String::new()),
            name: RefCell::new(String::new()),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_id(&mut self, value: String) {
        *self.object_path.borrow_mut() = value;
    }

    pub fn is_present(&self) -> Result<bool, Box<Error>> {
        Ok(self.isPresent.get())
    }

    pub fn set_present(&mut self, value: bool) {
        self.isPresent.set(value);
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Ok(self.isPowered.get())
    }

    pub fn set_powered(&mut self, value: bool) {
        self.isPowered.set(value)
    }

    pub fn get_can_start_discovery(&self) -> Result<bool, Box<Error>> {
        Ok(self.canStartDiscovery.get())
    }

    pub fn set_can_start_discovery(&mut self, value: bool) {
        self.canStartDiscovery.set(value);
    }

    pub fn get_can_stop_siscovery(&self) -> Result<bool, Box<Error>> {
        Ok(self.canStopDiscovery.get())
    }

    pub fn set_can_stop_discovery(&mut self, value: bool) {
        self.canStopDiscovery.set(value);
    }

    pub fn get_devices(&self) -> Result<Vec<FakeBluetoothDevice>, Box<Error>> {
        Ok(self.devices.borrow().clone())
    }

    pub fn set_devices(&mut self, devices: Vec<FakeBluetoothDevice>){
        *self.devices.borrow_mut() = devices;
    }

    pub fn get_first_device(&self) -> Result<FakeBluetoothDevice, Box<Error>> {
        if self.devices.borrow().is_empty() {
            return Err(Box::from("No device found."))
        }
        Ok(self.devices.borrow()[0].clone())
    }

    pub fn get_addatas(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.addatas.borrow().clone())
    }

    pub fn set_addatas(&mut self, addatas: Vec<String>) {
        *self.addatas.borrow_mut() = addatas;
    }

    pub fn get_first_addata(&self) -> Result<String, Box<Error>> {
        if self.addatas.borrow().is_empty() {
            return Err(Box::from("No addata found."))
        }
        Ok(self.addatas.borrow()[0].clone())
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.borrow().clone())
    }

    pub fn set_address(&mut self, address: String) {
        *self.address.borrow_mut() = address;
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.borrow().clone())
    }

    pub fn set_name(&mut self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn create_discovery_session(&self) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        FakeBluetoothDiscoverySession::create_session(Arc::new(self.clone()))
    }
}
