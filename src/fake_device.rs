use fake_adapter::FakeBluetoothAdapter;
use std::cell::{Cell, RefCell};
use rustc_serialize::hex::FromHex;
use core::ops::Deref;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct FakeBluetoothDevice {
    object_path: String,
	adapter: RefCell<Option<FakeBluetoothAdapter>>,
	appearance: Cell<u16>,
	connectionError: Cell<u32>,
	deviceClass: Cell<u32>,
	isPaired: Cell<bool>,
	isConnected: Cell<bool>,
	isTrusted: Cell<bool>,
	isBlocked: Cell<bool>,
	isLegacyPairing: Cell<bool>,
	uuids: RefCell<Vec<String>>,
	modalias: RefCell<String>,
	name: RefCell<String>,
	productId: Cell<u32>,
	productVersion: Cell<u32>,
	rssi: Cell<i16>,
	txPower: Cell<i16>,
	vendorId: Cell<u32>,
	vendorIdSource: RefCell<String>,
}

impl FakeBluetoothDevice {
	pub fn new(object_path: String,
			   adapter: FakeBluetoothAdapter,
			   appearance: u16,
			   connectionError: u32,
			   deviceClass: u32,
			   isPaired: bool,
			   isConnected: bool,
			   isTrusted: bool,
			   isBlocked: bool,
			   isLegacyPairing: bool,
			   uuids: Vec<String>,
			   modalias: String,
			   name: String,
			   productId: u32,
			   productVersion: u32,
			   rssi: i16,
			   txPower: i16,
			   vendorId: u32,
			   vendorIdSource: String)
			   -> FakeBluetoothDevice {
		FakeBluetoothDevice{
		   	object_path: object_path,
			adapter: RefCell::new(Some(adapter)),
			appearance: Cell::new(appearance),
			connectionError: Cell::new(connectionError),
			deviceClass: Cell::new(deviceClass),
			isPaired: Cell::new(isPaired),
			isConnected: Cell::new(isConnected),
			isTrusted: Cell::new(isTrusted),
			isBlocked: Cell::new(isBlocked),
			isLegacyPairing: Cell::new(isLegacyPairing),
			uuids: RefCell::new(uuids),
			modalias: RefCell::new(modalias),
			name: RefCell::new(name),
			productId: Cell::new(productId),
			productVersion: Cell::new(productVersion),
			rssi: Cell::new(rssi),
			txPower: Cell::new(txPower),
			vendorId: Cell::new(vendorId),
			vendorIdSource: RefCell::new(vendorIdSource),
		}
	}

	pub fn new_empty(object_path: String) -> FakeBluetoothDevice {
		FakeBluetoothDevice{
		   	object_path: object_path,
			adapter: RefCell::new(None),
			appearance: Cell::new(0),
			connectionError: Cell::new(0),
			deviceClass: Cell::new(0),
			isPaired: Cell::new(false),
			isConnected: Cell::new(false),
			isTrusted: Cell::new(false),
			isBlocked: Cell::new(false),
			isLegacyPairing: Cell::new(false),
			uuids: RefCell::new(vec![]),
			modalias: RefCell::new(String::new()),
			name: RefCell::new(String::new()),
			productId: Cell::new(0),
			productVersion: Cell::new(0),
			rssi: Cell::new(0),
			txPower: Cell::new(0),
			vendorId: Cell::new(0),
			vendorIdSource: RefCell::new(String::new()),
		}
	}

    pub fn get_object_path(&self) -> String {
        self.object_path.clone()
    }

    pub fn get_adapter(&self) -> Result<FakeBluetoothAdapter, Box<Error>> {
        match self.adapter.borrow().clone() {
        	Some(adapter) => return Ok(adapter),
        	None => return Err(Box::from("Device has no adapter."))
        }
    }

    pub fn set_adapter(&mut self, adapter: FakeBluetoothAdapter) {
    	*self.adapter.borrow_mut() = Some(adapter);
    }

    pub fn get_appearance(&self) -> u16 {
        self.appearance.get()
    }

    pub fn set_appearance(&mut self, value: u16) {
        self.appearance.set(value);
    }

    pub fn get_connection_error(&self) -> u32 {
        self.connectionError.get()
    }

    pub fn set_connection_error(&mut self, value: u32) {
        self.connectionError.set(value);
    }

    pub fn get_device_class(&self) -> u32 {
        self.deviceClass.get()
    }

    pub fn set_device_class(&mut self, value: u32) {
        self.deviceClass.set(value);
    }

    pub fn is_paired(&self) -> bool {
        self.isPaired.get()
    }

    pub fn set_paired(&mut self, value: bool) {
    	self.isPaired.set(value);
    }

    pub fn is_connected(&self) -> bool {
        self.isConnected.get()
    }

    pub fn set_connected(&mut self, value: bool) {
    	self.isConnected.set(value);
    }

    pub fn is_trusted(&self) -> bool {
        self.isTrusted.get()
    }

    pub fn set_trusted(&mut self, value: bool) {
    	self.isTrusted.set(value);
    }

    pub fn is_blocked(&self) -> bool {
        self.isBlocked.get()
    }

    pub fn set_blocked(&mut self, value: bool) {
    	self.isBlocked.set(value);
    }

    pub fn is_legacy_pairing(&self) -> bool {
        self.isLegacyPairing.get()
    }

    pub fn set_legacy_pairing(&mut self, value: bool) {
    	self.isLegacyPairing.set(value);
    }

    pub fn get_uuids(&self) -> Vec<String> {
        self.uuids.borrow().clone()
    }

    pub fn set_uuids(&mut self, uuids: Vec<String>) {
    	for uuid in uuids {
    		self.uuids.borrow_mut().push(uuid);
    	}
    }

    pub fn get_name(&self) -> String {
        self.name.borrow().clone()
    }

    pub fn set_name(&mut self, name: String) {
    	*self.name.borrow_mut() = name;
    }

    pub fn get_product_id(&self) -> u32 {
        self.productId.get()
    }

    pub fn set_product_id(&mut self, value: u32) {
    	self.productId.set(value);
    }

    pub fn get_product_version(&self) -> u32 {
    	self.productVersion.get()
    }

    pub fn set_product_version(&mut self, value: u32) {
    	self.productVersion.set(value);
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

    pub fn get_vendor_id(&self) -> u32 {
        self.vendorId.get()
    }

    pub fn set_vendor_id(&mut self, value: u32) {
    	self.vendorId.set(value);
    }

    pub fn get_vendor_id_source(&self) -> String {
        self.vendorIdSource.borrow().clone()
    }

    pub fn set_vendor_id_source(&mut self, value: String) {
    	*self.vendorIdSource.borrow_mut() = value;
    }
}
