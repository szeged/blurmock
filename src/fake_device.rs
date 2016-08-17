use fake_adapter::FakeBluetoothAdapter;
use fake_service::FakeBluetoothGATTService;
use std::error::Error;
use std::sync::Arc;
use rustc_serialize::hex::FromHex;

#[derive(Clone, Debug)]
pub struct FakeBluetoothDevice {
    id: String,
    adapter: Arc<FakeBluetoothAdapter>,
    address: String,
    appearance: u16,
    class: u32,
    gatt_services: Vec<Arc<FakeBluetoothGATTService>>,
    is_paired: bool,
    is_connectable: bool,
    is_connected: bool,
    is_trusted: bool,
    is_blocked: bool,
    is_legacy_pairing: bool,
    uuids: Vec<String>,
    name: String,
    icon: String,
    alias: String,
    product_version: u32,
    rssi: i16,
    tx_power: i16,
    modalias: String,
}

impl FakeBluetoothDevice {
    pub fn new(id: String,
               adapter: Arc<FakeBluetoothAdapter>,
               address: String,
               appearance: u16,
               class: u32,
               gatt_services: Vec<Arc<FakeBluetoothGATTService>>,
               is_paired: bool,
               is_connectable: bool,
               is_connected: bool,
               is_trusted: bool,
               is_blocked: bool,
               is_legacy_pairing: bool,
               uuids: Vec<String>,
               name: String,
               icon: String,
               alias: String,
               product_version: u32,
               rssi: i16,
               tx_power: i16,
               modalias: String)
               -> Arc<FakeBluetoothDevice> {
        let device = Arc::new(FakeBluetoothDevice{
            id: id,
            adapter: adapter,
            address: address,
            appearance: appearance,
            class: class,
            gatt_services: gatt_services,
            is_paired: is_paired,
            is_connectable: is_connectable,
            is_connected: is_connected,
            is_trusted: is_trusted,
            is_blocked: is_blocked,
            is_legacy_pairing: is_legacy_pairing,
            uuids: uuids,
            name: name,
            icon: icon,
            alias: alias,
            product_version: product_version,
            rssi: rssi,
            tx_power: tx_power,
            modalias: modalias,
        });
        let _ = Arc::make_mut(&mut device.adapter.clone()).add_device(device.clone());
        device
    }

    pub fn new_empty(adapter: Arc<FakeBluetoothAdapter>, device_id: String)
            -> Arc<FakeBluetoothDevice> {
        FakeBluetoothDevice::new(
            /*id*/ adapter.get_id() + &device_id,
            /*adapter*/ adapter,
            /*address*/ String::new(),
            /*appearance*/ 0,
            /*class*/ 0,
            /*gatt_services*/ vec!(),
            /*is_paired*/ false,
            /*is_connectable*/ false,
            /*is_connected*/ false,
            /*is_trusted*/ false,
            /*is_blocked*/ false,
            /*is_legacy_pairing*/ false,
            /*uuids*/ vec!(),
            /*name*/ String::new(),
            /*icon*/ String::new(),
            /*alias*/ String::new(),
            /*product_version*/ 0,
            /*rssi*/ 0,
            /*tx_power*/ 0,
            /*modalias*/ String::new(),
        )
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn get_adapter(&self) -> Result<Arc<FakeBluetoothAdapter>, Box<Error>> {
        Ok(self.adapter.clone())
    }

    pub fn set_adapter(&mut self, adapter: Arc<FakeBluetoothAdapter>) -> Result<(), Box<Error>> {
        Ok(self.adapter = adapter)
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.clone())
    }

    pub fn set_address(&mut self, address: String) -> Result<(), Box<Error>> {
        Ok(self.address = address)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.clone())
    }

    pub fn set_name(&mut self, name: String) -> Result<(), Box<Error>> {
        Ok(self.name = name)
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        Ok(self.icon.clone())
    }

    pub fn set_icon(&mut self, value: String) -> Result<(), Box<Error>> {
        Ok(self.icon = value)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Ok(self.class)
    }

    pub fn set_class(&mut self, value: u32) -> Result<(), Box<Error>> {
        Ok(self.class = value)
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        Ok(self.appearance)
    }

    pub fn set_appearance(&mut self, appearance: u16) -> Result<(), Box<Error>> {
        Ok(self.appearance = appearance)
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.uuids.clone())
    }

    pub fn set_uuids(&mut self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        Ok(self.uuids = uuids)
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_paired)
    }

    pub fn pair(&mut self) -> Result<(), Box<Error>> {
        Ok(self.is_paired = true)
    }

    pub fn cancel_pairing(&mut self) -> Result<(), Box<Error>> {
        Ok(self.is_paired = false)
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_connected)
    }

    pub fn set_connected(&mut self, connected: bool) -> Result<(), Box<Error>> {
        Ok(self.is_connected = connected)
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_trusted)
    }

    pub fn set_trusted(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_trusted = value)
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_blocked)
    }

    pub fn set_blocked(&mut self, blocked: bool) -> Result<(), Box<Error>> {
        Ok(self.is_blocked = blocked)
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Ok(self.alias.clone())
    }

    pub fn set_alias(&mut self, value: String) -> Result<(), Box<Error>> {
        Ok(self.alias = value)
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_legacy_pairing)
    }

    pub fn set_legacy_pairing(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_legacy_pairing = value)
    }

    pub fn get_modalias(&self) ->  Result<(String, u32, u32, u32), Box<Error>> {
        let ids: Vec<&str> = self.modalias.split(":").collect();

        let source = String::from(ids[0]);
        let vendor = ids[1][1..5].from_hex().unwrap();
        let product = ids[1][6..10].from_hex().unwrap();
        let device = ids[1][11..15].from_hex().unwrap();

        Ok((source,
        (vendor[0] as u32) * 16 * 16 + (vendor[1] as u32),
        (product[0] as u32) * 16 * 16 + (product[1] as u32),
        (device[0] as u32) * 16 * 16 + (device[1] as u32)))
    }

    pub fn set_modalias(&mut self, value: String) -> Result<(), Box<Error>> {
        Ok(self.modalias = value)
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        let (vendor_id_source,_,_,_) = try!(self.get_modalias());
        Ok(vendor_id_source)
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        let (_,vendor_id,_,_) = try!(self.get_modalias());
        Ok(vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        let (_,_,product_id,_) = try!(self.get_modalias());
        Ok(product_id)
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        let (_,_,_,device_id) = try!(self.get_modalias());
        Ok(device_id)
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        Ok(self.rssi)
    }

    pub fn set_rssi(&mut self, rssi: i16) -> Result<(), Box<Error>> {
        Ok(self.rssi = rssi)
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        Ok(self.tx_power)
    }

    pub fn set_tx_power(&mut self, tx_power: i16) -> Result<(), Box<Error>> {
        Ok(self.tx_power = tx_power)
    }

    pub fn get_gatt_services(&self) -> Result<Vec<Arc<FakeBluetoothGATTService>>, Box<Error>> {
        Ok(self.gatt_services.clone())
    }

    pub fn set_gatt_services(&mut self, services: Vec<Arc<FakeBluetoothGATTService>>) -> Result<(), Box<Error>> {
        Ok(self.gatt_services = services)
    }

    pub fn add_service(&mut self, service: Arc<FakeBluetoothGATTService>) -> Result<(), Box<Error>> {
        Ok(self.gatt_services.push(service))
    }

    pub fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    pub fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    pub fn connect(&mut self) -> Result<(), Box<Error>> {
        if self.is_connectable && !self.is_connected {
            self.is_connected = true;
            return Ok(());
        }
        return Err(Box::from("Could not connect to the device."));
    }

    pub fn disconnect(&mut self) -> Result<(), Box<Error>>{
        if self.is_connected {
            self.is_connected = false;
            return Ok(());
        }
        return Err(Box::from("The device is not connected."));
    }
}
