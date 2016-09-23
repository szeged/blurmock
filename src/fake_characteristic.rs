use core::ops::Deref;
use fake_descriptor::FakeBluetoothGATTDescriptor;
use fake_service::FakeBluetoothGATTService;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTCharacteristic {
    id: Arc<Mutex<String>>,
    uuid: Arc<Mutex<String>>,
    service: Arc<FakeBluetoothGATTService>,
    value: Arc<Mutex<Option<Vec<u8>>>>,
    is_notifying: Arc<Mutex<bool>>,
    flags: Arc<Mutex<Vec<String>>>,
    gatt_descriptors: Arc<Mutex<Vec<Arc<FakeBluetoothGATTDescriptor>>>>,
}

impl FakeBluetoothGATTCharacteristic {
    pub fn new(id: String,
               uuid: String,
               service: Arc<FakeBluetoothGATTService>,
               value: Option<Vec<u8>>,
               is_notifying: bool,
               flags: Vec<String>,
               gatt_descriptors: Vec<Arc<FakeBluetoothGATTDescriptor>>)
               -> Arc<FakeBluetoothGATTCharacteristic> {
        if let Ok(existing_characteristic) = service.get_gatt_characteristic(id.clone()) {
            return existing_characteristic;
        }
        let characteristic = Arc::new(FakeBluetoothGATTCharacteristic {
            id: Arc::new(Mutex::new(id)),
            uuid: Arc::new(Mutex::new(uuid)),
            service: service.clone(),
            value: Arc::new(Mutex::new(value)),
            is_notifying: Arc::new(Mutex::new(is_notifying)),
            flags: Arc::new(Mutex::new(flags)),
            gatt_descriptors: Arc::new(Mutex::new(gatt_descriptors)),
        });
        let _ = service.add_characteristic(characteristic.clone());
        characteristic
    }

    pub fn new_empty(service: Arc<FakeBluetoothGATTService>,
                     characteristic_id: String)
                     -> Arc<FakeBluetoothGATTCharacteristic> {
        FakeBluetoothGATTCharacteristic::new(
            /*id*/ characteristic_id,
            /*uuid*/ String::new(),
            /*service*/ service,
            /*value*/ None,
            /*is_notifying*/ false,
            /*flags*/ vec!(),
            /*gatt_descriptors*/ vec!(),
        )
    }

    make_getter!(get_id, id);

    make_setter!(set_id, id);

    make_getter!(get_uuid, uuid, String);

    make_setter!(set_uuid, uuid, String);

    make_option_getter!(get_value, value, Vec<u8>);

    make_setter!(set_value, value, Option<Vec<u8>>);

    make_getter!(is_notifying);

    make_setter!(set_notifying, is_notifying, bool);

    make_getter!(get_flags, flags, Vec<String>);

    make_setter!(set_flags, flags, Vec<String>);

    make_getter!(get_gatt_descriptor_structs, gatt_descriptors, Vec<Arc<FakeBluetoothGATTDescriptor>>);

    pub fn get_service(&self) -> Result<Arc<FakeBluetoothGATTService>, Box<Error>> {
        Ok(self.service.clone())
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        self.set_notifying(true)
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        self.set_notifying(false)
    }

    pub fn get_gatt_descriptors(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.gatt_descriptors.clone();
        let gatt_descriptors = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_descriptors.into_iter().map(|s| s.get_id()).collect())
    }

    pub fn get_gatt_descriptor(&self, id: String) -> Result<Arc<FakeBluetoothGATTDescriptor>, Box<Error>> {
        let descriptors = try!(self.get_gatt_descriptor_structs());
        for descriptor in descriptors {
            let descriptor_id = descriptor.get_id();
            if descriptor_id == id {
                return Ok(descriptor);
            }
        }
        Err(Box::from("No descriptor exists with the given id."))
    }

    pub fn add_descriptor(&self, descriptor: Arc<FakeBluetoothGATTDescriptor>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_descriptors.clone();
        let mut gatt_descriptors = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_descriptors.push(descriptor))
    }

    pub fn remove_descriptor(&self, id: String) -> Result<(), Box<Error>> {
        let cloned = self.gatt_descriptors.clone();
        let mut gatt_descriptors = match cloned.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_descriptors.retain(|d| d.get_id() != id))
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_value()
    }

    pub fn write_value(&self, value: Vec<u8>) -> Result<(), Box<Error>> {
        self.set_value(Some(value))
    }
}
