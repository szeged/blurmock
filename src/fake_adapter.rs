use fake_device::FakeBluetoothDevice;
use std::cell::{Cell, RefCell};
use std::error::Error;
use rustc_serialize::hex::FromHex;

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

    pub fn get_object_path(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_object_path(&mut self, value: String) {
        *self.object_path.borrow_mut() = value;
    }

    pub fn is_present(&self) -> bool {
        self.isPresent.get()
    }

    pub fn set_present(&mut self, value: bool) {
        self.isPresent.set(value);
    }

    pub fn is_powered(&self) -> bool {
        self.isPowered.get()
    }

    pub fn set_powered(&mut self, value: bool) -> () {
        self.isPowered.set(value)
    }

    pub fn get_can_start_discovery(&self) -> bool {
        self.canStartDiscovery.get()
    }

    pub fn set_can_start_discovery(&mut self, value: bool) {
        self.canStartDiscovery.set(value);
    }

    pub fn get_can_stop_siscovery(&self) -> bool {
        self.canStopDiscovery.get()
    }

    pub fn set_can_stop_discovery(&mut self, value: bool) {
        self.canStopDiscovery.set(value);
    }

    pub fn get_devices(&self) -> Vec<FakeBluetoothDevice> {
        self.devices.borrow().clone()
    }

    pub fn push_devices(&mut self, devices: Vec<FakeBluetoothDevice>){
        for device in devices {
            self.devices.borrow_mut().push(device);
        }
    }

    pub fn get_first_device(&self) -> Result<FakeBluetoothDevice, Box<Error>> {
        if self.devices.borrow().is_empty() {
            return Err(Box::from("No device found."))
        }
        Ok(self.devices.borrow()[0].clone())
    }

    pub fn get_addatas(&self) -> Vec<String> {
        self.addatas.borrow().clone()
    }

    pub fn set_addatas(&mut self, addatas: Vec<String>) {
        for addata in addatas {
            self.addatas.borrow_mut().push(addata);
        }
    }

    pub fn get_first_addata(&self) -> Result<String, Box<Error>> {
        if self.addatas.borrow().is_empty() {
            return Err(Box::from("No addata found."))
        }
        Ok(self.addatas.borrow()[0].clone())
    }

    pub fn get_address(&self) -> String {
        self.address.borrow().clone()
    }

    pub fn set_address(&mut self, address: String) {
        *self.address.borrow_mut() = address;
    }

    pub fn get_name(&self) -> String {
        self.name.borrow().clone()
    }

    pub fn set_name(&mut self, name: String) {
        *self.name.borrow_mut() = name;
    }
}
