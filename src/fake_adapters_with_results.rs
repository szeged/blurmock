use std::cell::Cell;
use std::error::Error;
use rustc_serialize::hex::FromHex;

#[derive(Debug)]
struct FakeBluetoothAdapter {
    object_path: String,
    isPresent: bool,
    isPowered: Cell<bool>,
    canStartDiscovery: bool,
    canStopDiscovery: bool,
    devices: Vec<String>,
    addatas: Vec<String>,
    address: String,
    name: String,
    alias: String,
    class: u32,
    isDiscoverable: Cell<bool>,
    isPairable: Cell<bool>,
    pairableTimeout: Cell<u32>,
    discoverableTimeout: Cell<u32>,
    isDiscovering: bool,
    modalias: String,
}

impl FakeBluetoothAdapter {
    pub fn new(object_path: String,
               isPresent: bool,
               isPowered: bool,
               canStartDiscovery: bool,
               canStopDiscovery: bool,
               devices: Vec<String>,
               addatas: Vec<String>,
               address: String,
               name: String,
               alias: String,
               class: u32,
               isDiscoverable: bool,
               isPairable: bool,
               pairableTimeout: u32,
               discoverableTimeout: u32,
               isDiscovering: bool,
               modalias: String)
               -> FakeBluetoothAdapter {
    	FakeBluetoothAdapter {
            object_path: object_path,
    		isPresent: isPresent,
    		isPowered: Cell::new(isPowered),
    		canStartDiscovery: canStartDiscovery,
    		canStopDiscovery: canStopDiscovery,
            devices: devices,
            addatas: addatas,
            address: address,
            name: name,
            alias: alias,
            class: class,
            isDiscoverable: Cell::new(isDiscoverable),
            isPairable: Cell::new(isPairable),
            pairableTimeout: Cell::new(pairableTimeout),
            discoverableTimeout: Cell::new(discoverableTimeout),
            isDiscovering: isDiscovering,
            modalias: modalias,
    	}
    }

    pub fn get_object_path(&self) -> String {
        self.object_path.clone()
    }

    pub fn get_present(&self) -> bool {
    	self.isPresent.clone()
    }

    pub fn get_canStartDiscovery(&self) -> bool {
    	self.canStartDiscovery.clone()
    }

    pub fn get_canStopDiscovery(&self) -> bool {
    	self.canStopDiscovery.clone()
    }

    pub fn get_first_device(&self) -> Result<String, Box<Error>> {
        if self.devices.is_empty() {
            return Err(Box::from("No device found."))
        }
        Ok(self.devices[0].clone())
    }

    pub fn get_first_addata(&self) -> Result<String, Box<Error>> {
        if self.addatas.is_empty() {
            return Err(Box::from("No addata found."))
        }
        Ok(self.addatas[0].clone())
    }

    pub fn get_device_list(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.devices.clone())
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.clone())
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.clone())
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Ok(self.alias.clone())
    }

    /*pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        Ok(self.alias.set(value))
    }*/

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Ok(self.class.clone())
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Ok(self.isPowered.get())
    }

    pub fn set_powered(&self, value: bool) -> Result<(),Box<Error>> {
        Ok(self.isPowered.set(value))
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        Ok(self.isDiscoverable.get())
    }

    pub fn set_discoverable(&self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.isDiscoverable.set(value))
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        Ok(self.isPairable.get())
    }

    pub fn set_pairable(&self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.isPairable.set(value))
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        Ok(self.pairableTimeout.get())
    }

    pub fn set_pairable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        Ok(self.pairableTimeout.set(value))
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        Ok(self.discoverableTimeout.get())
    }

    pub fn set_discoverable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        Ok(self.discoverableTimeout.set(value))
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        Ok(self.isDiscovering.clone())
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
