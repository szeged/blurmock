use core::ops::Deref;
use fake_adapter::FakeBluetoothAdapter;
use fake_service::FakeBluetoothGATTService;
use rustc_serialize::hex::FromHex;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothDevice {
    id: Arc<Mutex<String>>,
    adapter: Arc<FakeBluetoothAdapter>,
    address: Arc<Mutex<String>>,
    appearance: Arc<Mutex<Option<u16>>>,
    class: Arc<Mutex<u32>>,
    gatt_services: Arc<Mutex<Vec<Arc<FakeBluetoothGATTService>>>>,
    is_paired: Arc<Mutex<bool>>,
    is_connectable: Arc<Mutex<bool>>,
    is_connected: Arc<Mutex<bool>>,
    is_trusted: Arc<Mutex<bool>>,
    is_blocked: Arc<Mutex<bool>>,
    is_legacy_pairing: Arc<Mutex<bool>>,
    uuids: Arc<Mutex<Vec<String>>>,
    name: Arc<Mutex<Option<String>>>,
    icon: Arc<Mutex<String>>,
    alias: Arc<Mutex<String>>,
    product_version: Arc<Mutex<u32>>,
    rssi: Arc<Mutex<Option<i16>>>,
    tx_power: Arc<Mutex<Option<i16>>>,
    modalias: Arc<Mutex<String>>,
}

impl FakeBluetoothDevice {
    pub fn new(id: String,
               adapter: Arc<FakeBluetoothAdapter>,
               address: String,
               appearance: Option<u16>,
               class: u32,
               gatt_services: Vec<Arc<FakeBluetoothGATTService>>,
               is_paired: bool,
               is_connectable: bool,
               is_connected: bool,
               is_trusted: bool,
               is_blocked: bool,
               is_legacy_pairing: bool,
               uuids: Vec<String>,
               name: Option<String>,
               icon: String,
               alias: String,
               product_version: u32,
               rssi: Option<i16>,
               tx_power: Option<i16>,
               modalias: String)
               -> Arc<FakeBluetoothDevice> {
        if let Ok(existing_device) = adapter.get_device(id.clone()) {
            return existing_device;
        }
        let device = Arc::new(FakeBluetoothDevice{
            id: Arc::new(Mutex::new(id)),
            adapter: adapter.clone(),
            address: Arc::new(Mutex::new(address)),
            appearance: Arc::new(Mutex::new(appearance)),
            class: Arc::new(Mutex::new(class)),
            gatt_services: Arc::new(Mutex::new(gatt_services)),
            is_paired: Arc::new(Mutex::new(is_paired)),
            is_connectable: Arc::new(Mutex::new(is_connectable)),
            is_connected: Arc::new(Mutex::new(is_connected)),
            is_trusted: Arc::new(Mutex::new(is_trusted)),
            is_blocked: Arc::new(Mutex::new(is_blocked)),
            is_legacy_pairing: Arc::new(Mutex::new(is_legacy_pairing)),
            uuids: Arc::new(Mutex::new(uuids)),
            name: Arc::new(Mutex::new(name)),
            icon: Arc::new(Mutex::new(icon)),
            alias: Arc::new(Mutex::new(alias)),
            product_version: Arc::new(Mutex::new(product_version)),
            rssi: Arc::new(Mutex::new(rssi)),
            tx_power: Arc::new(Mutex::new(tx_power)),
            modalias: Arc::new(Mutex::new(modalias)),
        });
        let _ = adapter.add_device(device.clone());
        device
    }

    pub fn new_empty(adapter: Arc<FakeBluetoothAdapter>, device_id: String)
            -> Arc<FakeBluetoothDevice> {
        FakeBluetoothDevice::new(
            /*id*/ device_id,
            /*adapter*/ adapter,
            /*address*/ String::new(),
            /*appearance*/ None,
            /*class*/ 0,
            /*gatt_services*/ vec!(),
            /*is_paired*/ false,
            /*is_connectable*/ false,
            /*is_connected*/ false,
            /*is_trusted*/ false,
            /*is_blocked*/ false,
            /*is_legacy_pairing*/ false,
            /*uuids*/ vec!(),
            /*name*/ None,
            /*icon*/ String::new(),
            /*alias*/ String::new(),
            /*product_version*/ 0,
            /*rssi*/ None,
            /*tx_power*/ None,
            /*modalias*/ String::new(),
        )
    }

    make_getter!(get_id, id);

    make_setter!(set_id, id);

    make_getter!(get_address, address, String);

    make_setter!(set_address, address, String);

    make_option_getter!(get_name, name, String);

    make_setter!(set_name, name, Option<String>);

    make_getter!(get_icon, icon, String);

    make_setter!(set_icon, icon, String);

    make_getter!(get_class, class, u32);

    make_setter!(set_class, class, u32);

    make_option_getter!(get_appearance, appearance, u16);

    make_setter!(set_appearance, appearance, Option<u16>);

    make_getter!(get_uuids, uuids, Vec<String>);

    make_setter!(set_uuids, uuids, Vec<String>);

    make_getter!(is_paired);

    make_setter!(set_paired, is_paired, bool);

    make_getter!(is_connectable);

    make_setter!(set_connectable, is_connectable, bool);

    make_getter!(is_connected);

    make_setter!(set_connected, is_connected, bool);

    make_getter!(is_trusted);

    make_setter!(set_trusted, is_trusted, bool);

    make_getter!(is_blocked);

    make_setter!(set_blocked, is_blocked, bool);

    make_getter!(get_alias, alias, String);

    make_setter!(set_alias, alias, String);

    make_getter!(is_legacy_pairing);

    make_setter!(set_legacy_pairing, is_legacy_pairing, bool);

    make_setter!(set_modalias, modalias, String);

    make_option_getter!(get_rssi, rssi, i16);

    make_setter!(set_rssi, rssi, Option<i16>);

    make_option_getter!(get_tx_power, tx_power, i16);

    make_setter!(set_tx_power, tx_power, Option<i16>);

    pub fn get_adapter(&self) -> Result<Arc<FakeBluetoothAdapter>, Box<Error>> {
        Ok(self.adapter.clone())
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        self.set_paired(true)
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        self.set_paired(false)
    }

    pub fn get_modalias(&self) ->  Result<(String, u32, u32, u32), Box<Error>> {
        let cloned = self.modalias.clone();
        let modalias = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };

        let ids: Vec<&str> = modalias.split(":").collect();
        let source = String::from(ids[0]);
        let vendor = ids[1][1..5].from_hex().unwrap();
        let product = ids[1][6..10].from_hex().unwrap();
        let device = ids[1][11..15].from_hex().unwrap();

        Ok((source,
        (vendor[0] as u32) * 16 * 16 + (vendor[1] as u32),
        (product[0] as u32) * 16 * 16 + (product[1] as u32),
        (device[0] as u32) * 16 * 16 + (device[1] as u32)))
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

    pub fn get_gatt_services(&self) -> Result<Vec<String>, Box<Error>> {
        if !(try!(self.is_connected())) {
            return Err(Box::from("Device not connected."));
        }

        let cloned = self.gatt_services.clone();
        let gatt_services = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_services.into_iter().map(|s| s.get_id()).collect())
    }

    pub fn get_gatt_service_structs(&self) -> Result<Vec<Arc<FakeBluetoothGATTService>>, Box<Error>> {
        if !(try!(self.is_connected())) {
            return Err(Box::from("Device not connected."));
        }

        let cloned = self.gatt_services.clone();
        let gatt_services = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_services)
    }

    pub fn get_gatt_service(&self, id: String) -> Result<Arc<FakeBluetoothGATTService>, Box<Error>> {
        let services = try!(self.get_gatt_service_structs());
        for service in services {
            let service_id = service.get_id();
            if service_id == id {
                return Ok(service);
            }
        }
        Err(Box::from("No service exists with the given id."))
    }

    pub fn add_service(&self, service: Arc<FakeBluetoothGATTService>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_services.clone();
        let mut gatt_services = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_services.push(service))
    }

    pub fn remove_service(&self, id: String) -> Result<(), Box<Error>> {
        let cloned = self.gatt_services.clone();
        let mut gatt_services = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_services.retain(|s| s.get_id() != id))
    }

    pub fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    pub fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        let is_connectable = try!(self.is_connectable());
        let is_connected = try!(self.is_connected());

        if is_connected {
            return Ok(());
        }
        if is_connectable {
            return self.set_connected(true);
        }
        return Err(Box::from("Could not connect to the device."));
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>>{
        let is_connected = try!(self.is_connected());

        if is_connected {
            return self.set_connected(false);
        }
        return Err(Box::from("The device is not connected."));
    }
}
