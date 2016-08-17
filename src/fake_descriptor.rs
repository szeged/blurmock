use fake_characteristic::FakeBluetoothGATTCharacteristic;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTDescriptor {
    id: String,
    uuid: String,
    characteristic: Arc<FakeBluetoothGATTCharacteristic>,
    value: Vec<u8>,
    flags: Vec<String>,
}

impl FakeBluetoothGATTDescriptor {
    pub fn new(id: String,
               uuid: String,
               characteristic: Arc<FakeBluetoothGATTCharacteristic>,
               value: Vec<u8>,
               flags: Vec<String>)
               -> Arc<FakeBluetoothGATTDescriptor> {
        let descriptor = Arc::new(FakeBluetoothGATTDescriptor{
            id: id,
            uuid: uuid,
            characteristic: characteristic,
            value: value,
            flags: flags,
        });
        let _ = Arc::make_mut(&mut descriptor.characteristic.clone()).add_descriptor(descriptor.clone());
        descriptor
    }

    pub fn new_empty(characteristic: Arc<FakeBluetoothGATTCharacteristic>,
                     descriptor_id: String)
                     -> Arc<FakeBluetoothGATTDescriptor> {
        FakeBluetoothGATTDescriptor::new(
            /*id*/ characteristic.get_id() + &descriptor_id,
            /*uuid*/ String::new(),
            /*characteristic*/ characteristic,
            /*value*/ vec!(),
            /*flags*/ vec!(),
        )
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, path: String) {
        self.id = path;
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Ok(self.uuid.clone())
    }

    pub fn set_uuid(&mut self, uuid: String) -> Result<(), Box<Error>> {
        Ok(self.uuid = uuid)
    }

    pub fn get_characteristic(&self) -> Result<Arc<FakeBluetoothGATTCharacteristic>, Box<Error>> {
        Ok(self.characteristic.clone())
    }

    pub fn set_characteristic(&mut self, characteristic: Arc<FakeBluetoothGATTCharacteristic>) -> Result<(), Box<Error>> {
        Ok(self.characteristic = characteristic)
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Ok(self.value.clone())
    }

    pub fn set_value(&mut self, value: Vec<u8>) -> Result<(), Box<Error>> {
        Ok(self.value = value)
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.flags.clone())
    }

    pub fn set_flags(&mut self, flags: Vec<String>) -> Result<(), Box<Error>> {
        Ok(self.flags = flags)
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_value()
    }

    pub fn write_value(&mut self, value: Vec<u8>) -> Result<(), Box<Error>> {
        self.set_value(value)
    }
}
