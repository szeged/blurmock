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
    gattServices: RefCell<Vec<FakeBluetoothGATTService>>,
    //isPaired: Cell<bool>,
    isConnectable: Cell<bool>,
    isConnected: Cell<bool>,
    //isTrusted: Cell<bool>,
    isBlocked: Cell<bool>,
    //isLegacyPairing: Cell<bool>,
    uuids: RefCell<Vec<String>>,
    name: RefCell<String>,
    //productId: Cell<u32>,
    //productVersion: Cell<u32>,
    rssi: Cell<i16>,
    txPower: Cell<i16>,
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
               gattServices: Vec<FakeBluetoothGATTService>,
               //isPaired: bool,
               isConnectable: bool,
               isConnected: bool,
               //isTrusted: bool,
               isBlocked: bool,
               //isLegacyPairing: bool,
               uuids: Vec<String>,
               name: String,
               //productId: u32,
               //productVersion: u32,
               rssi: i16,
               txPower: i16,
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
            gattServices: RefCell::new(gattServices),
            //isPaired: Cell::new(isPaired),
            isConnectable: Cell::new(isConnectable),
            isConnected: Cell::new(isConnected),
            //isTrusted: Cell::new(isTrusted),
            isBlocked: Cell::new(isBlocked),
            //isLegacyPairing: Cell::new(isLegacyPairing),
            uuids: RefCell::new(uuids),
            name: RefCell::new(name),
            //productId: Cell::new(productId),
            //productVersion: Cell::new(productVersion),
            rssi: Cell::new(rssi),
            txPower: Cell::new(txPower),
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
            gattServices: RefCell::new(vec![]),
            //isPaired: Cell::new(false),
            isConnectable: Cell::new(false),
            isConnected: Cell::new(false),
            //isTrusted: Cell::new(false),
            isBlocked: Cell::new(false),
            //isLegacyPairing: Cell::new(false),
            uuids: RefCell::new(vec![]),
            name: RefCell::new(String::new()),
            //productId: Cell::new(0),
            //productVersion: Cell::new(0),
            rssi: Cell::new(0),
            txPower: Cell::new(0),
            //vendorId: Cell::new(0),
            //vendorIdSource: RefCell::new(String::new()),
        }
    }

    pub fn get_object_path(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_object_path(&mut self, path: String) {
        *self.object_path.borrow_mut() = path;
    }

    pub fn get_adapter(&self) -> FakeBluetoothAdapter {
        self.adapter.borrow().clone()
    }

    pub fn set_adapter(&mut self, adapter: FakeBluetoothAdapter) {
        *self.adapter.borrow_mut() = adapter;
    }


    pub fn get_address(&self) -> String {
        self.address.borrow().clone()
    }

    pub fn set_address(&mut self, address: String) {
        *self.address.borrow_mut() = address;
    }

    pub fn get_appearance(&self) -> u16 {
        self.appearance.get()
    }

    pub fn set_appearance(&mut self, value: u16) {
        self.appearance.set(value);
    }

    pub fn get_gatt_services(&self) -> Vec<FakeBluetoothGATTService> {
        self.gattServices.borrow().clone()
    }

    pub fn set_gatt_service(&mut self, services: Vec<FakeBluetoothGATTService>) {
        *self.gattServices.borrow_mut() = services;
    }

    pub fn is_connected(&self) -> bool {
        self.isConnected.get()
    }

    pub fn set_connected(&mut self, value: bool) {
        self.isConnected.set(value);
    }

    pub fn is_blocked(&self) -> bool {
        self.isBlocked.get()
    }

    pub fn set_blocked(&mut self, value: bool) {
        self.isBlocked.set(value);
    }

    pub fn get_uuids(&self) -> Vec<String> {
        self.uuids.borrow().clone()
    }

    pub fn set_uuids(&mut self, uuids: Vec<String>) {
        *self.uuids.borrow_mut() = uuids;
    }

    pub fn get_name(&self) -> String {
        self.name.borrow().clone()
    }

    pub fn set_name(&mut self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn get_rssi(&self) -> i16 {
        self.rssi.get()
    }

    pub fn set_rssi(&mut self, value: i16) {
        self.rssi.set(value);
    }

    pub fn get_tx_power(&self) -> i16 {
        self.txPower.get()
    }

    pub fn set_tx_power(&mut self, value: i16) {
        self.txPower.set(value);
    }

    pub fn connect(&mut self) -> Result<(), Box<Error>> {
        if self.isConnectable.get() && !self.isConnected.get() {
            self.isConnected.set(true);
            return Ok(());
        } else {
            return Err(Box::from("Could not connect to the device."));
        }
    }

    pub fn disconnect(&mut self) -> Result<(), Box<Error>>{
        if self.isConnected.get() {
            self.isConnected.set(false);
            return Ok(());
        } else {
            return Err(Box::from("The device is not connected."));
        }
    }
}
