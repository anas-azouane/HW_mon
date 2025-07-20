use sysinfo::Networks;
use std::{thread, time};

pub struct NetMon {
    network: Networks,
}

impl NetMon {
    pub fn new() -> Self {
        let network = Networks::new_with_refreshed_list();
        Self { network }
    }

    pub fn refresh(&mut self) {
        thread::sleep(time::Duration::from_millis(10));
        self.network.refresh(true);
    }

    pub fn get_net_traffic(&mut self) {
        println!("Network Traffic:");
        self.refresh();
        for (interface_name, net) in &self.network {
        println!("  {} in: {} B", interface_name, net.received());
        println!("  {} in: {} B", interface_name, net.transmitted());

        }
    }
}
