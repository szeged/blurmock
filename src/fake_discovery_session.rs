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

    pub fn get_adapter(&self) -> Arc<FakeBluetoothAdapter> {
        self.adapter.clone()
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        match self.adapter.get_can_start_discovery() {
            Ok(false) => Err(Box::from("Failed to start discovery session")),
            Ok(true) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        match self.adapter.get_can_stop_discovery() {
            Ok(false) => Err(Box::from("Failed to stop discovery session")),
            Ok(true) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
