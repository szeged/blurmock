use core::ops::Deref;
use fake_characteristic::FakeBluetoothGATTCharacteristic;
use fake_device::FakeBluetoothDevice;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    id: Arc<Mutex<String>>,
    device: Arc<FakeBluetoothDevice>,
    gatt_characteristics: Arc<Mutex<Vec<Arc<FakeBluetoothGATTCharacteristic>>>>,
    is_primary: Arc<Mutex<bool>>,
    included_services: Arc<Mutex<Vec<Arc<FakeBluetoothGATTService>>>>,
    uuid: Arc<Mutex<String>>,
}

impl FakeBluetoothGATTService {
    pub fn new(id: String,
               device: Arc<FakeBluetoothDevice>,
               gatt_characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>,
               is_primary: bool,
               included_services: Vec<Arc<FakeBluetoothGATTService>>,
               uuid: String)
               -> Arc<FakeBluetoothGATTService> {
        if let Ok(existing_service) = device.get_gatt_service(id.clone()) {
            return existing_service;
        }
        let service = Arc::new(FakeBluetoothGATTService {
            id: Arc::new(Mutex::new(id)),
            device: device.clone(),
            gatt_characteristics: Arc::new(Mutex::new(gatt_characteristics)),
            is_primary: Arc::new(Mutex::new(is_primary)),
            included_services: Arc::new(Mutex::new(included_services)),
            uuid: Arc::new(Mutex::new(uuid)),
        });
        let _ = device.add_service(service.clone());
        service
    }

    pub fn new_empty(device: Arc<FakeBluetoothDevice>,
                     service_id: String)
                     -> Arc<FakeBluetoothGATTService> {
        FakeBluetoothGATTService::new(
            /*id*/ service_id,
            /*device*/ device,
            /*gatt_characteristics*/ vec!(),
            /*is_primary*/ false,
            /*included_services*/ vec!(),
            /*uuid*/ String::new(),
        )
    }

    make_getter!(get_id, id);

    make_setter!(set_id, id);

    make_getter!(get_gatt_characteristic_structs, gatt_characteristics, Vec<Arc<FakeBluetoothGATTCharacteristic>>);

    make_getter!(is_primary);

    make_setter!(set_is_primary, is_primary, bool);

    make_setter!(set_includes, included_services, Vec<Arc<FakeBluetoothGATTService>>);

    make_getter!(get_uuid, uuid, String);

    make_setter!(set_uuid, uuid, String);

    pub fn get_device(&self) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        Ok(self.device.clone())
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.gatt_characteristics.clone();
        let gatt_characteristics = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_characteristics.into_iter().map(|s| s.get_id()).collect())
    }

    pub fn get_gatt_characteristic(&self, id: String) -> Result<Arc<FakeBluetoothGATTCharacteristic>, Box<Error>> {
        let characteristics = try!(self.get_gatt_characteristic_structs());
        for characteristic in characteristics {
            let characteristic_id = characteristic.get_id();
            if characteristic_id == id {
                return Ok(characteristic);
            }
        }
        Err(Box::from("No characteristic exists with the given id."))
    }

    pub fn add_characteristic(&self, characteristic: Arc<FakeBluetoothGATTCharacteristic>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_characteristics.clone();
        let mut gatt_characteristics = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_characteristics.push(characteristic))
    }

    pub fn remove_characteristic(&self, id: String) -> Result<(), Box<Error>> {
        let cloned = self.gatt_characteristics.clone();
        let mut gatt_characteristics = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_characteristics.retain(|c| c.get_id() != id))
    }

    pub fn add_included_service(&self, service: Arc<FakeBluetoothGATTService>) -> Result<(), Box<Error>> {
        let cloned = self.included_services.clone();
        let mut included_services = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(included_services.push(service))
    }

    pub fn remove_included_service(&self, id: String) -> Result<(), Box<Error>> {
        let cloned = self.included_services.clone();
        let mut included_services = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(included_services.retain(|i| i.get_id() != id))
    }

    pub fn get_includes(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.included_services.clone();
        let included_services = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(included_services.into_iter().map(|s| s.get_id()).collect())
    }

}
