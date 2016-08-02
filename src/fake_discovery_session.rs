use fake_adapter::FakeBluetoothAdapter;
use std::error::Error;

#[derive(Debug)]
pub struct FakeBluetoothDiscoverySession {
    adapter: FakeBluetoothAdapter,
}

impl FakeBluetoothDiscoverySession {
    pub fn create_session(adapter: FakeBluetoothAdapter) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        Ok(FakeBluetoothDiscoverySession::new(adapter))
    }

    fn new(adapter: FakeBluetoothAdapter) -> FakeBluetoothDiscoverySession {
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
