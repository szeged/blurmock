use core::ops::Deref;
use fake_device::FakeBluetoothDevice;
use fake_discovery_session::FakeBluetoothDiscoverySession;
use rustc_serialize::hex::FromHex;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothAdapter {
    id: Arc<Mutex<String>>,
    is_present: Arc<Mutex<bool>>,
    is_powered: Arc<Mutex<bool>>,
    can_start_discovery: Arc<Mutex<bool>>,
    can_stop_discovery: Arc<Mutex<bool>>,
    devices: Arc<Mutex<Vec<Arc<FakeBluetoothDevice>>>>,
    ad_datas: Arc<Mutex<Vec<String>>>,
    address: Arc<Mutex<String>>,
    name: Arc<Mutex<String>>,
    alias: Arc<Mutex<String>>,
    class: Arc<Mutex<u32>>,
    is_discoverable: Arc<Mutex<bool>>,
    is_pairable: Arc<Mutex<bool>>,
    pairable_timeout: Arc<Mutex<u32>>,
    discoverable_timeout: Arc<Mutex<u32>>,
    is_discovering: Arc<Mutex<bool>>,
    uuids: Arc<Mutex<Vec<String>>>,
    modalias: Arc<Mutex<String>>,
}

impl FakeBluetoothAdapter {
    pub fn new(id: String,
               is_present: bool,
               is_powered: bool,
               can_start_discovery: bool,
               can_stop_discovery: bool,
               devices: Vec<Arc<FakeBluetoothDevice>>,
               ad_datas: Vec<String>,
               address: String,
               name: String,
               alias: String,
               class: u32,
               is_discoverable: bool,
               is_pairable: bool,
               pairable_timeout: u32,
               discoverable_timeout: u32,
               is_discovering: bool,
               uuids: Vec<String>,
               modalias: String)
               -> Arc<FakeBluetoothAdapter> {
        Arc::new(FakeBluetoothAdapter {
            id: Arc::new(Mutex::new(id)),
            is_present: Arc::new(Mutex::new(is_present)),
            is_powered: Arc::new(Mutex::new(is_powered)),
            can_start_discovery: Arc::new(Mutex::new(can_start_discovery)),
            can_stop_discovery: Arc::new(Mutex::new(can_stop_discovery)),
            devices: Arc::new(Mutex::new(devices)),
            ad_datas: Arc::new(Mutex::new(ad_datas)),
            address: Arc::new(Mutex::new(address)),
            name: Arc::new(Mutex::new(name)),
            alias: Arc::new(Mutex::new(alias)),
            class: Arc::new(Mutex::new(class)),
            is_discoverable: Arc::new(Mutex::new(is_discoverable)),
            is_pairable: Arc::new(Mutex::new(is_pairable)),
            pairable_timeout: Arc::new(Mutex::new(pairable_timeout)),
            discoverable_timeout: Arc::new(Mutex::new(discoverable_timeout)),
            is_discovering: Arc::new(Mutex::new(is_discovering)),
            uuids: Arc::new(Mutex::new(uuids)),
            modalias: Arc::new(Mutex::new(modalias)),
        })
    }

    pub fn new_empty() -> Arc<FakeBluetoothAdapter> {
        FakeBluetoothAdapter::new(
            /*id*/ String::new(),
            /*is_present*/ true,
            /*is_powered*/ false,
            /*can_start_discovery*/ true,
            /*can_stop_discovery*/ true,
            /*devices*/ vec![],
            /*ad_datas*/ vec![],
            /*address*/ String::new(),
            /*name*/ String::new(),
            /*alias*/ String::new(),
            /*class*/ 0,
            /*is_discoverable*/ false,
            /*is_pairable*/ false,
            /*pairable_timeout*/ 0,
            /*discoverable_timeout*/ 0,
            /*is_discovering*/ false,
            /*uuids*/ vec![],
            /*modalias*/ String::new(),
        )
    }

    make_getter!(get_id, id);

    make_setter!(set_id, id);

    make_getter!(is_present);

    make_setter!(set_present, is_present, bool);

    make_getter!(is_powered);

    make_setter!(set_powered, is_powered, bool);

    make_getter!(get_can_start_discovery, can_start_discovery, bool);

    make_setter!(set_can_start_discovery, can_start_discovery, bool);

    make_getter!(get_can_stop_discovery, can_stop_discovery, bool);

    make_setter!(set_can_stop_discovery, can_stop_discovery, bool);

    make_getter!(get_devices, devices, Vec<Arc<FakeBluetoothDevice>>);

    make_getter!(get_ad_datas, ad_datas, Vec<String>);

    make_setter!(set_ad_datas, ad_datas, Vec<String>);

    make_getter!(get_address, address, String);

    make_setter!(set_address, address, String);

    make_getter!(get_name, name, String);

    make_setter!(set_name, name, String);

    make_getter!(get_alias, alias, String);

    make_setter!(set_alias, alias, String);

    make_getter!(get_class, class, u32);

    make_setter!(set_class, class, u32);

    make_getter!(is_discoverable);

    make_setter!(set_discoverable, is_discoverable, bool);

    make_getter!(is_pairable);

    make_setter!(set_pairable, is_pairable, bool);

    make_getter!(get_pairable_timeout, pairable_timeout, u32);

    make_setter!(set_pairable_timeout, pairable_timeout, u32);

    make_getter!(get_discoverable_timeout, discoverable_timeout, u32);

    make_setter!(set_discoverable_timeout, discoverable_timeout, u32);

    make_getter!(is_discovering);

    make_setter!(set_discovering, is_discovering, bool);

    make_getter!(get_uuids, uuids, Vec<String>);

    make_setter!(set_uuids, uuids, Vec<String>);

    make_setter!(set_modalias, modalias, String);

    pub fn get_device(&self, id: String) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            let device_id = device.get_id();
            if device_id == id {
                return Ok(device);
            }
        }
        Err(Box::from("No device exists with the given id."))
    }

    pub fn get_device_list(&self) -> Result<Vec<String>, Box<Error>> {
        let devices = try!(self.get_devices());
        let mut ids = vec![];
        for device in &devices {
            let id = device.get_id();
            ids.push(id);
        }
        Ok(ids)
    }

    pub fn get_first_device(&self) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        if devices.is_empty() {
            return Err(Box::from("No device found."))
        }
        Ok(devices[0].clone())
    }

    pub fn add_device(&self, device: Arc<FakeBluetoothDevice>) -> Result<(), Box<Error>> {
        let cloned = self.devices.clone();
        let mut devices = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(devices.push(device))
    }

    pub fn remove_device(&self, id: String) -> Result<(), Box<Error>> {
        let cloned = self.devices.clone();
        let mut devices = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(devices.retain(|d| d.get_id() != id))
    }

    pub fn get_first_ad_data(&self) -> Result<String, Box<Error>> {
        let ad_datas = try!(self.get_ad_datas());
        if ad_datas.is_empty() {
            return Err(Box::from("No ad_data found."))
        }
        Ok(ad_datas[0].clone())
    }

     pub fn create_discovery_session(&self) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        FakeBluetoothDiscoverySession::create_session(Arc::new(self.clone()))
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
}
