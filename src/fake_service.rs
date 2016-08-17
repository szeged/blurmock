use fake_characteristic::FakeBluetoothGATTCharacteristic;
use fake_device::FakeBluetoothDevice;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    id: String,
    device: Arc<FakeBluetoothDevice>,
    gatt_characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>,
    is_primary: bool,
    included_services: Vec<Arc<FakeBluetoothGATTService>>,
    uuid: String,
}

impl FakeBluetoothGATTService {
    pub fn new(id: String,
               device: Arc<FakeBluetoothDevice>,
               gatt_characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>,
               is_primary: bool,
               included_services: Vec<Arc<FakeBluetoothGATTService>>,
               uuid: String)
               -> Arc<FakeBluetoothGATTService> {
        let service = Arc::new(FakeBluetoothGATTService{
            id: id,
            device: device,
            gatt_characteristics: gatt_characteristics,
            is_primary: is_primary,
            included_services: included_services,
            uuid: uuid,
        });
        let _ = Arc::make_mut(&mut service.device.clone()).add_service(service.clone());
        service
    }

    pub fn new_empty(device: Arc<FakeBluetoothDevice>,
                     service_id: String)
                     -> Arc<FakeBluetoothGATTService> {
        FakeBluetoothGATTService::new(
            /*id*/ device.get_id() + &service_id,
            /*device*/ device,
            /*gatt_characteristics*/ vec!(),
            /*is_primary*/ false,
            /*included_services*/ vec!(),
            /*uuid*/ String::new(),
        )
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, path: String) {
        self.id = path;
    }

    pub fn get_device(&self) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        Ok(self.device.clone())
    }

    pub fn set_device(&mut self, device: Arc<FakeBluetoothDevice>) -> Result<(), Box<Error>> {
        Ok(self.device = device)
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<Arc<FakeBluetoothGATTCharacteristic>>, Box<Error>> {
        Ok(self.gatt_characteristics.clone())
    }

    pub fn set_gatt_characteristics(&mut self, characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>) -> Result<(), Box<Error>> {
        Ok(self.gatt_characteristics = characteristics)
    }

    pub fn add_characteristic(&mut self, characteristic: Arc<FakeBluetoothGATTCharacteristic>) -> Result<(), Box<Error>> {
        Ok(self.gatt_characteristics.push(characteristic))
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_primary)
    }

    pub fn set_is_primary(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_primary = value)
    }

    pub fn get_includes(&self) -> Result<Vec<Arc<FakeBluetoothGATTService>>, Box<Error>> {
        Ok(self.included_services.clone())
    }

    pub fn set_includes(&mut self, included_services: Vec<Arc<FakeBluetoothGATTService>>) -> Result<(), Box<Error>> {
        Ok(self.included_services = included_services)
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Ok(self.uuid.clone())
    }

    pub fn set_uuid(&mut self, uuid: String) -> Result<(), Box<Error>> {
        Ok(self.uuid = uuid)
    }
}
