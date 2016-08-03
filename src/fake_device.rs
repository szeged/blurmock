use fake_adapter::FakeBluetoothAdapter;
use fake_service::FakeBluetoothGATTService;
use std::cell::{Cell, RefCell};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct FakeBluetoothDevice {
    object_path: RefCell<String>,
    adapter: RefCell<FakeBluetoothAdapter>,
    address: RefCell<String>,
    appearance: Cell<u16>,
    //connectionError: Cell<u32>,
    //deviceClass: Cell<u32>,
    gatt_services: RefCell<Vec<FakeBluetoothGATTService>>,
    //isPaired: Cell<bool>,
    is_connectable: Cell<bool>,
    is_connected: Cell<bool>,
    //isTrusted: Cell<bool>,
    is_blocked: Cell<bool>,
    //isLegacyPairing: Cell<bool>,
    uuids: RefCell<Vec<String>>,
    name: RefCell<String>,
    //productId: Cell<u32>,
    //productVersion: Cell<u32>,
    rssi: Cell<i16>,
    tx_power: Cell<i16>,
    //vendorId: Cell<u32>,
    //vendorIdSource: RefCell<String>,
}

impl FakeBluetoothDevice {
    pub fn new(object_path: String,
               adapter: FakeBluetoothAdapter,
               address: String,
               appearance: u16,
               //connectionError: u32,
               //deviceClass: u32,
               gatt_services: Vec<FakeBluetoothGATTService>,
               //isPaired: bool,
               is_connectable: bool,
               is_connected: bool,
               //isTrusted: bool,
               is_blocked: bool,
               //isLegacyPairing: bool,
               uuids: Vec<String>,
               name: String,
               //productId: u32,
               //productVersion: u32,
               rssi: i16,
               tx_power: i16,
               //vendorId: u32,
               //vendorIdSource: String
               )
               -> FakeBluetoothDevice {
        FakeBluetoothDevice{
            object_path: RefCell::new(object_path),
            adapter: RefCell::new(adapter),
            address: RefCell::new(address),
            appearance: Cell::new(appearance),
            //connectionError: Cell::new(connectionError),
            //deviceClass: Cell::new(deviceClass),
            gatt_services: RefCell::new(gatt_services),
            //isPaired: Cell::new(isPaired),
            is_connectable: Cell::new(is_connectable),
            is_connected: Cell::new(is_connected),
            //isTrusted: Cell::new(isTrusted),
            is_blocked: Cell::new(is_blocked),
            //isLegacyPairing: Cell::new(isLegacyPairing),
            uuids: RefCell::new(uuids),
            name: RefCell::new(name),
            //productId: Cell::new(productId),
            //productVersion: Cell::new(productVersion),
            rssi: Cell::new(rssi),
            tx_power: Cell::new(tx_power),
            //vendorId: Cell::new(vendorId),
            //vendorIdSource: RefCell::new(vendorIdSource),
        }
    }

    pub fn new_empty() -> FakeBluetoothDevice {
        FakeBluetoothDevice{
            object_path: RefCell::new(String::new()),
            adapter: RefCell::new(FakeBluetoothAdapter::new_empty()),
            address: RefCell::new(String::new()),
            appearance: Cell::new(0),
            //connectionError: Cell::new(0),
            //deviceClass: Cell::new(0),
            gatt_services: RefCell::new(vec![]),
            //isPaired: Cell::new(false),
            is_connectable: Cell::new(false),
            is_connected: Cell::new(false),
            //isTrusted: Cell::new(false),
            is_blocked: Cell::new(false),
            //isLegacyPairing: Cell::new(false),
            uuids: RefCell::new(vec![]),
            name: RefCell::new(String::new()),
            //productId: Cell::new(0),
            //productVersion: Cell::new(0),
            rssi: Cell::new(0),
            tx_power: Cell::new(0),
            //vendorId: Cell::new(0),
            //vendorIdSource: RefCell::new(String::new()),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_id(&mut self, path: String) {
        *self.object_path.borrow_mut() = path;
    }

    pub fn get_adapter(&self) -> Result<FakeBluetoothAdapter, Box<Error>> {
        Ok(self.adapter.borrow().clone())
    }

    pub fn set_adapter(&mut self, adapter: FakeBluetoothAdapter) {
        *self.adapter.borrow_mut() = adapter;
    }


    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.borrow().clone())
    }

    pub fn set_address(&mut self, address: String) {
        *self.address.borrow_mut() = address;
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        Ok(self.appearance.get())
    }

    pub fn set_appearance(&mut self, value: u16) {
        self.appearance.set(value);
    }

    pub fn get_gatt_services(&self) -> Result<Vec<FakeBluetoothGATTService>, Box<Error>> {
        Ok(self.gatt_services.borrow().clone())
    }

    pub fn set_gatt_service(&mut self, services: Vec<FakeBluetoothGATTService>) {
        *self.gatt_services.borrow_mut() = services;
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_connected.get())
    }

    pub fn set_connected(&mut self, value: bool) {
        self.is_connected.set(value);
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_blocked.get())
    }

    pub fn set_blocked(&mut self, value: bool) {
        self.is_blocked.set(value);
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.uuids.borrow().clone())
    }

    pub fn set_uuids(&mut self, uuids: Vec<String>) {
        *self.uuids.borrow_mut() = uuids;
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.borrow().clone())
    }

    pub fn set_name(&mut self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        Ok(self.rssi.get())
    }

    pub fn set_rssi(&mut self, value: i16) {
        self.rssi.set(value);
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        Ok(self.tx_power.get())
    }

    pub fn set_tx_power(&mut self, value: i16) {
        self.tx_power.set(value);
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        if self.is_connectable.get() && !self.is_connected.get() {
            self.is_connected.set(true);
            return Ok(());
        } else {
            return Err(Box::from("Could not connect to the device."));
        }
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>>{
        if self.is_connected.get() {
            self.is_connected.set(false);
            return Ok(());
        } else {
            return Err(Box::from("The device is not connected."));
        }
    }
}
