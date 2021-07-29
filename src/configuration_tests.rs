#[cfg(test)]
mod config_tests {
    use crate::configuration;

    fn get_config_and_check_string() -> (configuration::ShardsCollection, String) {
        let mut coll = configuration::ShardsCollection::new();
        let shard1 = configuration::AvtanShard::new(String::from("avtan1"), 666);
        let shard2 = configuration::AvtanShard::new(String::from("avtan2"), 667);
        let shard3 = configuration::AvtanShard::new(String::from("avtan3"), 668);
        coll.add_shard(shard1);
        coll.add_shard(shard2);
        coll.add_shard(shard3);
        (coll, String::from("[{\"hostname\":\"avtan1\",\"port\":666},{\"hostname\":\"avtan2\",\"port\":667},{\"hostname\":\"avtan3\",\"port\":668}]"))
    }

    #[test]
    fn test_smth(){
        let check_config_tuple = get_config_and_check_string();
        let guarded_coll = check_config_tuple.0.get_shards_configuration();
        let config_string = configuration::get_cluster_configuration(guarded_coll);

        assert_eq!(check_config_tuple.1, config_string);
    }
}