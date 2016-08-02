use fake_adapter::FakeBluetoothAdapter;
use std::error::Error;
use std::sync::Arc; 

#[derive(Debug)]
pub struct FakeBluetoothDiscoverySession {
    adapter: Arc<FakeBluetoothAdapter>,
}

impl FakeBluetoothDiscoverySession {
    pub fn create_session(adapter: Arc<FakeBluetoothAdapter>) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        Ok(FakeBluetoothDiscoverySession::new(adapter))
    }

    fn new(adapter: Arc<FakeBluetoothAdapter>) -> FakeBluetoothDiscoverySession {
        FakeBluetoothDiscoverySession {
            adapter: adapter,
        }
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        Ok(())
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        Ok(())
    }
}
