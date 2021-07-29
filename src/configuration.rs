use std::sync::{Arc, RwLock, RwLockReadGuard};
use serde_json;
use serde::{Serialize, Deserialize};

/// AvtanShard - current cluster shard object
#[derive(Serialize, Deserialize)]
pub struct AvtanShard {
    hostname: String,
    port: u16,
}

impl AvtanShard {
    pub fn new(hostname: String, port: u16) -> Self {
        AvtanShard {hostname, port}
    }

    pub fn read_from_file() -> Result<Self, ()> {
        todo!()
    }
}

/// Collection of all shards of the cluster
pub struct ShardsCollection {
    collection: Arc<RwLock<Vec<AvtanShard>>>   
}

impl ShardsCollection {
    pub fn new() -> Self {
        ShardsCollection{ collection: Arc::new(RwLock::new(Vec::new()))}
    }

    pub fn add_shard(&mut self, shard: AvtanShard){
        self.collection.write().unwrap().push(shard);
    }

    pub fn get_shards_configuration(&self) -> RwLockReadGuard<Vec<AvtanShard>> {
        self.collection.read().unwrap()
    }

    pub fn read_config_from_fs(&mut self) -> Result<(), ()>{
        Err(())
    }
}

pub fn get_cluster_configuration(collection: RwLockReadGuard<Vec<AvtanShard>>) -> String {
    let config_string = match serde_json::to_string(&*collection){
        Ok(x) => x,
        Err(_) => String::from("")
    };
    String::from(config_string)
}