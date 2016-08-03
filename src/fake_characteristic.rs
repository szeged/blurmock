use fake_descriptor::FakeBluetoothGATTDescriptor;
use fake_service::FakeBluetoothGATTService;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTCharacteristic {
    object_path: String,
    uuid: String,
    service: FakeBluetoothGATTService,
    value: Vec<u8>,
    is_notifying: bool,
    flags: Vec<String>,
    descriptors: Vec<FakeBluetoothGATTDescriptor>,
}

impl FakeBluetoothGATTCharacteristic {
    pub fn new(object_path: String,
               uuid: String,
               service: FakeBluetoothGATTService,
               value: Vec<u8>,
               is_notifying: bool,
               flags: Vec<String>,
               descriptors: Vec<FakeBluetoothGATTDescriptor>)
               -> FakeBluetoothGATTCharacteristic {
        FakeBluetoothGATTCharacteristic {
            object_path: object_path,
            uuid: uuid,
            service: service,
            value: value,
            is_notifying: is_notifying,
            flags: flags,
            descriptors: descriptors,
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTCharacteristic {
        FakeBluetoothGATTCharacteristic {
            object_path: String::new(),
            uuid: String::new(),
            service: FakeBluetoothGATTService::new_empty(),
            value: vec![],
            is_notifying: false,
            flags: vec![],
            descriptors: vec![],
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    pub fn set_id(&mut self, path: String) {
        self.object_path = path;
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Ok(self.uuid.clone())
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    pub fn get_service(&self) -> Result<FakeBluetoothGATTService, Box<Error>> {
        Ok(self.service.clone())
    }

    pub fn set_service(&mut self, service: FakeBluetoothGATTService) {
        self.service = service;
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Ok(self.value.clone())
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_notifying)
    }

    pub fn set_is_notifying(&mut self, value: bool) {
        self.is_notifying = value;
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.flags.clone())
    }

    pub fn set_flags(&mut self, flags: Vec<String>) {
        self.flags = flags;
    }

    pub fn get_descriptors(&self) -> Result<Vec<FakeBluetoothGATTDescriptor>, Box<Error>> {
        Ok(self.descriptors.clone())
    }

    pub fn set_descriptors(&mut self, descriptors: Vec<FakeBluetoothGATTDescriptor>) {
        self.descriptors = descriptors;
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_value()
    }

    // Try some other workout if &mut self not works!
    pub fn write_value(&mut self, value: Vec<u8>) -> Result<(), Box<Error>> {
        self.value = value;
        Ok(())
    }
}
