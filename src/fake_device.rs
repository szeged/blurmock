use fake_adapter::FakeBluetoothAdapter;
use fake_service::FakeBluetoothGATTService;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct FakeBluetoothDevice {
    object_path: String,
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
    //vendorIdSource: String,
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
            object_path: object_path,
            adapter: adapter,
            address: address,
            appearance: appearance,
            //connectionError: connectionError,
            //deviceClass: deviceClass,
            gatt_services: gatt_services,
            //isPaired: isPaired,
            is_connectable: is_connectable,
            is_connected: is_connected,
            //isTrusted: isTrusted,
            is_blocked: is_blocked,
            //isLegacyPairing: isLegacyPairing,
            uuids: uuids,
            name: name,
            //productId: productId,
            //productVersion: productVersion,
            rssi: rssi,
            tx_power: tx_power,
            //vendorId: vendorId,
            //vendorIdSource: vendorIdSource,
        }
    }

    pub fn new_empty() -> FakeBluetoothDevice {
        FakeBluetoothDevice{
            object_path: String::new(),
            adapter: FakeBluetoothAdapter::new_empty(),
            address: String::new(),
            appearance: 0,
            //connectionError: 0,
            //deviceClass: 0,
            gatt_services: vec![],
            //isPaired: false,
            is_connectable: false,
            is_connected: false,
            //isTrusted: false,
            is_blocked: false,
            //isLegacyPairing: false,
            uuids: vec![],
            name: String::new(),
            //productId: 0,
            //productVersion: 0,
            rssi: 0,
            tx_power: 0,
            //vendorId: 0,
            //vendorIdSource: String::new(),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    pub fn set_id(&mut self, object_path: String) {
        self.object_path = object_path;
    }

    pub fn get_adapter(&self) -> Result<FakeBluetoothAdapter, Box<Error>> {
        Ok(self.adapter.clone())
    }

    pub fn set_adapter(&mut self, adapter: FakeBluetoothAdapter) {
        self.adapter = adapter;
    }


    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.clone())
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        Ok(self.appearance)
    }

    pub fn set_appearance(&mut self, appearance: u16) {
        self.appearance = appearance;
    }

    pub fn get_gatt_services(&self) -> Result<Vec<FakeBluetoothGATTService>, Box<Error>> {
        Ok(self.gatt_services.clone())
    }

    pub fn set_gatt_service(&mut self, services: Vec<FakeBluetoothGATTService>) {
        self.gatt_services = services;
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_connected)
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_blocked)
    }

    pub fn set_blocked(&mut self, blocked: bool) {
        self.is_blocked = blocked;
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.uuids.clone())
    }

    pub fn set_uuids(&mut self, uuids: Vec<String>) {
        self.uuids = uuids;
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.clone())
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        Ok(self.rssi)
    }

    pub fn set_rssi(&mut self, rssi: i16) {
        self.rssi = rssi;
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        Ok(self.tx_power)
    }

    pub fn set_tx_power(&mut self, tx_power: i16) {
        self.tx_power = tx_power;
    }

    pub fn connect(&mut self) -> Result<(), Box<Error>> {
        if self.is_connectable && !self.is_connected {
            self.is_connected = true;
            return Ok(());
        } else {
            return Err(Box::from("Could not connect to the device."));
        }
    }

    pub fn disconnect(&mut self) -> Result<(), Box<Error>>{
        if self.is_connected {
            self.is_connected = false;
            return Ok(());
        } else {
            return Err(Box::from("The device is not connected."));
        }
    }
}
