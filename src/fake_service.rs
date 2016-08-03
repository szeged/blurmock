use fake_characteristic::FakeBluetoothGATTCharacteristic;
use fake_device::FakeBluetoothDevice;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    object_path: String,
    device: Arc<FakeBluetoothDevice>,
    gatt_characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>,
    is_primary: bool,
    included_services: Vec<Arc<FakeBluetoothGATTService>>,
    uuid: String,
}

impl FakeBluetoothGATTService {
    pub fn new(object_path: String,
               device: Arc<FakeBluetoothDevice>,
               gatt_characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>,
               is_primary: bool,
               included_services: Vec<Arc<FakeBluetoothGATTService>>,
               uuid: String)
               -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: object_path,
            device: device,
            gatt_characteristics: gatt_characteristics,
            is_primary: is_primary,
            included_services: included_services,
            uuid: uuid,
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: String::new(),
            device: Arc::new(FakeBluetoothDevice::new_empty()),
            gatt_characteristics: vec![],
            is_primary: false,
            included_services: vec![],
            uuid: String::new(),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    pub fn set_id(&mut self, path: String) {
        self.object_path = path;
    }

    pub fn get_device(&self) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        Ok(self.device.clone())
    }

    pub fn set_device(&mut self, device: Arc<FakeBluetoothDevice>) {
        self.device = device;
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<Arc<FakeBluetoothGATTCharacteristic>>, Box<Error>> {
        Ok(self.gatt_characteristics.clone())
    }

    pub fn set_gatt_characteristics(&mut self, characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>) {
        self.gatt_characteristics = characteristics;
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_primary)
    }

    pub fn set_is_primary(&mut self, value: bool) {
        self.is_primary = value;
    }

    pub fn get_included_services(&self) -> Result<Vec<Arc<FakeBluetoothGATTService>>, Box<Error>> {
        Ok(self.included_services.clone())
    }

    pub fn set_included_services(&mut self, included_services: Vec<Arc<FakeBluetoothGATTService>>) {
        self.included_services = included_services;
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Ok(self.uuid.clone())
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }
}
