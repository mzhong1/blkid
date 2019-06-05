extern crate blkid;

use blkid::Probe;
use std::fs;
use std::path::PathBuf;

fn format(kind: &str, value: Option<&str>) -> String {
    value
        .map(|value| format!("{}=\"{}\"", kind, value))
        .unwrap_or_default()
}

pub fn main() {
    let sys_block = fs::read_dir("/sys/block").expect("unable to read block devices");

    for path in sys_block {
        let path = path.unwrap();
        let device = path.file_name();
        let device = device.to_str().unwrap();
        let dev_path = ["/dev/", device].concat();

        if let Ok(mut probe) = Probe::new_from(&dev_path) {
            match probe.get_partitions() {
                Ok(list) => {
                    let table = list.get_table().expect("block did not have table");

                    eprintln!("{}: {}", dev_path, table.get_type());

                    if let Ok(nparts) = list.numof_partitions() {
                        for porder in 0..nparts as i32 {
                            let partition = list
                                .get_partition(porder)
                                .expect("unable to read partition");

                            let partno = partition.get_partno().expect("failed to get partno");
                            let nvme = dev_path.chars().last().map_or(false, char::is_numeric);
                            let modifier = if nvme { "p" } else { "" };
                            let part_path =
                                PathBuf::from(format!("{}{}{}", dev_path, modifier, partno));
                            let probe =
                                Probe::new_from(&part_path).expect("failed to probe partition");

                            probe.probe_full();

                            println!(
                                "├─{}: {} {} {} {}",
                                part_path.display(),
                                format("UUID", probe.lookup_value("UUID").ok()),
                                format("TYPE", probe.lookup_value("TYPE").ok()),
                                format("PARTLABEL", partition.get_name()),
                                format("PARTUUID", partition.get_uuid()),
                            );
                        }
                    }
                }
                Err(_) => {
                    probe.probe_full();

                    let dm_name = ["/sys/class/block/", device, "/dm/name"].concat();
                    let dm_;
                    let path: &str = match fs::read_to_string(&dm_name) {
                        Ok(device_map) => {
                            dm_ = ["/dev/mapper/", device_map.trim_end()].concat();
                            &dm_
                        }
                        Err(_) => &dev_path,
                    };

                    if let Ok(fstype) = probe.lookup_value("TYPE") {
                        println!(
                            "{}: {} TYPE=\"{}\"",
                            path,
                            format("UUID", probe.lookup_value("UUID").ok()),
                            fstype,
                        );
                    }
                }
            }
        }
    }
}
