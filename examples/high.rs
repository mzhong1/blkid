// For fetching a list of partitions, their file systems, PartLabels, UUIDs, and PartUUIDs.

extern crate blkid;

use blkid::Cache;

fn main() {
    let cache = Cache::new().expect("failed to get blkid cache");
    for device in cache.probe_all().expect("failed to probe") {
        let name = device.name().display();
        for (tag, value) in device.tags() {
            println!("{}: {}: {}", name, tag, value);
        }
    }
}
