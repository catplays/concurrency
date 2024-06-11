
use anyhow::{anyhow, Ok, Result};
use std::{
    collections::HashMap,
    sync::{Arc,RwLock}
};
#[derive(Debug, Clone)]
pub struct Metrics {
    data:Arc<RwLock<HashMap<String, i64>>>,

}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data:Arc::new(RwLock::new(HashMap::new()))
        }
    }
    pub fn incr(&self, key: impl Into<String>) ->Result<()> {

        let mut data = self.data.write()
        .map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter +=1 ;
        Ok(())
    }
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self.data
        .read()
        .map_err(|e| anyhow!(e.to_string()))?
        .clone())
        
    }
}