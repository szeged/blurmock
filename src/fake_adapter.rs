use fake_device::FakeBluetoothDevice;
use fake_discovery_session::FakeBluetoothDiscoverySession;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FakeBluetoothAdapter {
    object_path: String,
    is_present: bool,
    is_powered: bool,
    can_start_discovery: bool,
    can_stop_discovery: bool,
    devices: Vec<Arc<FakeBluetoothDevice>>,
    addatas: Vec<String>,
    address: String,
    name: String,
}

impl FakeBluetoothAdapter {
    pub fn new(object_path: String,
               is_present: bool,
               is_powered: bool,
               can_start_discovery: bool,
               can_stop_discovery: bool,
               devices: Vec<Arc<FakeBluetoothDevice>>,
               addatas: Vec<String>,
               address: String,
               name: String) 
               ->FakeBluetoothAdapter{
        FakeBluetoothAdapter{
            object_path: object_path,
            is_present: is_present,
            is_powered: is_powered,
            can_start_discovery: can_start_discovery,
            can_stop_discovery: can_stop_discovery,
            devices: devices,
            addatas: addatas,
            address: address,
            name: name,
        }
    }

    pub fn new_empty() -> FakeBluetoothAdapter {
        FakeBluetoothAdapter {
            object_path: String::new(),
            is_present: false,
            is_powered: false,
            can_start_discovery: false,
            can_stop_discovery: false,
            devices: vec![],
            addatas: vec![],
            address: String::new(),
            name: String::new(),
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    pub fn set_id(&mut self, value: String) {
        self.object_path = value;
    }

    pub fn is_present(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_present)
    }

    pub fn set_present(&mut self, value: bool) {
        self.is_present = value;
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_powered)
    }

    pub fn set_powered(&mut self, value: bool) {
        self.is_powered = value
    }

    pub fn get_can_start_discovery(&self) -> Result<bool, Box<Error>> {
        Ok(self.can_start_discovery)
    }

    pub fn set_can_start_discovery(&mut self, value: bool) {
        self.can_start_discovery = value;
    }

    pub fn get_can_stop_siscovery(&self) -> Result<bool, Box<Error>> {
        Ok(self.can_stop_discovery)
    }

    pub fn set_can_stop_discovery(&mut self, value: bool) {
        self.can_stop_discovery = value;
    }

    pub fn get_devices(&self) -> Result<Vec<Arc<FakeBluetoothDevice>>, Box<Error>> {
        Ok(self.devices.clone())
    }

    pub fn set_devices(&mut self, devices: Vec<Arc<FakeBluetoothDevice>>){
        self.devices = devices;
    }

    pub fn get_first_device(&self) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        if self.devices.is_empty() {
            return Err(Box::from("No device found."))
        }
        Ok(self.devices[0].clone())
    }

    pub fn get_addatas(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.addatas.clone())
    }

    pub fn set_addatas(&mut self, addatas: Vec<String>) {
        self.addatas = addatas;
    }

    pub fn get_first_addata(&self) -> Result<String, Box<Error>> {
        if self.addatas.is_empty() {
            return Err(Box::from("No addata found."))
        }
        Ok(self.addatas[0].clone())
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.clone())
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.clone())
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn create_discovery_session(&self) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        FakeBluetoothDiscoverySession::create_session(Arc::new(self.clone()))
    }
}
