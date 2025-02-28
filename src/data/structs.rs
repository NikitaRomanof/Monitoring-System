use std::collections::HashMap;
use gfx_backend_vulkan as back_v;
use gfx_hal::Instance;
use systemstat::Platform as _;


use sysinfo::{
    Disks, System, Networks, Components
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CpuData {
   pub count_physical_cores: usize,
   pub count_logical_cores: usize,
   pub cpu_brand: String,
   pub cpu_arch: String,
   pub global_cpu_usage: i32,
   pub speed: u64,
   pub cpu_temp: i32,
}

impl CpuData {

    pub fn new() -> CpuData {
        let z = systemstat::System::new();
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        let data: CpuData;
        let mut vec_fr: Vec<u64> = vec![];
        if sys.cpus().len() > 0 {
            data = CpuData {
                count_physical_cores: sys.physical_core_count().unwrap_or(0), 
                count_logical_cores: sys.cpus().len(),  
                cpu_brand: sys.cpus()[0].brand().to_string(), 
                cpu_arch: sysinfo::System::cpu_arch(),
                global_cpu_usage: (sys.global_cpu_usage()) as i32,
                speed: sys_info::cpu_speed().unwrap_or(0),
                cpu_temp: ((z.cpu_temp().unwrap_or(0.0)) as i32)
            };
        } else {
            vec_fr.push(0);
            data = CpuData {
                count_physical_cores: 0, 
                count_logical_cores: 0, 
                cpu_brand: String::from("unknown"), 
                cpu_arch: sysinfo::System::cpu_arch(),
                global_cpu_usage: 0,
                speed: 0,
                cpu_temp: 0
            };
        }
        data
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("**CPU data***");
        println!("count_physical_cores:..........{}", self.count_physical_cores);
        println!("count_logical_cores:...........{}", self.count_logical_cores);
        println!("cpu_brand:.....................{}", self.cpu_brand);
        println!("cpu_architecture:..............{}", self.cpu_arch);
        println!("global_cpu_usage:..............{}", self.global_cpu_usage);
        println!("speed:.........................{}", self.speed);
        println!("cpu_temp:......................{}", self.cpu_temp);
    }

    #[allow(dead_code)]
    pub fn update_cpu_data(&mut self) {
        *self = Self::new();
    }

}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GpuData {
    pub gpu_data_vulcan: Vec<String>,
}

impl GpuData {

    pub fn new() -> GpuData {
        let instance: gfx_backend_vulkan::Instance = back_v::Instance::create("monitoring", 1).unwrap();
        let adapters  = instance.enumerate_adapters();
        let mut names_v: Vec<String> = Vec::new();
        for adapter in adapters {
            names_v.push(adapter.info.name.to_string());
        }
        GpuData { 
            gpu_data_vulcan: names_v
        }
    }
    
    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("**GPU data***");
        for name in &self.gpu_data_vulcan {
            println!("gpu_data_vulcan:..........{}", name);
        }
    }

    #[allow(dead_code)]
    pub fn update_gpu(&mut self) {
        *self = Self::new();
    }

}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Disk {
    pub available_space: u64,
    pub disk_type: String,
    pub file_system: String,
    pub total_space: u64,
    pub is_removable: bool,
    pub is_read_only: bool,
}

impl Disk {

    pub fn to_string(&self) -> String {
        "available_space: ".to_owned() + &(self.available_space.to_string()) + 
        "\n" + 
        "disk type: " + &(self.disk_type) +
        "\n" + 
        "dfile system: " + &(self.file_system) +
        "\n" + 
        "total space: " + &(self.total_space.to_string())
        
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("available_space:..........{} Mb", self.available_space / 1024000);
        println!("disk_type:................{}", self.disk_type);
        println!("file_system:..............{}", self.file_system);
        println!("total_space:..............{} Mb", self.total_space / 1024000);
        println!("is_removable:.............{} ", self.is_removable);
        println!("is_read_only:.............{} ", self.is_read_only);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DramData {
    pub disks: HashMap<String, Disk>
}

impl DramData {
    pub fn new() -> DramData {
        let disks = Disks::new_with_refreshed_list();
        let mut map_disk: HashMap<String, Disk> = HashMap::new();
        for disk in &disks {
            if disk.total_space() > 0 {
                let di = Disk {
                    available_space: disk.available_space() / 1024000, 
                    disk_type: disk.kind().to_string(), 
                    file_system: disk.file_system().to_str().unwrap_or("unknown").to_owned(), 
                    total_space: disk.total_space() / 1024000,
                    is_removable: disk.is_removable(),
                    is_read_only: disk.is_read_only()
                };
                map_disk.insert(disk.name().to_str().unwrap_or("unknown").to_owned(), di);
            }
        }

        DramData {disks: map_disk}
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("**Disk data***");
        let mut count: i32 = 0;
        for map in &self.disks {
            println!("**Disk № {}***", count);
            println!("name............{}", map.0);
            map.1.print_data();
            count += 1;
        }
    }

    #[allow(dead_code)]
    pub fn update_disk_data(&mut self) {
        *self = Self::new();
    }

}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RamData {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
    pub available_memory: u64,
}

impl RamData {

    pub fn new() -> RamData {
        let mut sys = System::new();
        sys.refresh_memory();
        let data: RamData = RamData {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            free_swap: sys.free_swap(),
            used_swap: sys.used_swap(),
            available_memory: sys.available_memory()
        };
            data
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("**Ram data***");
        println!("total_memory:..............{} Mb", self.total_memory / 1024000);
        println!("used_memory:...............{} Mb", self.used_memory / 1024000);
        println!("total_swap:................{} Mb", self.total_swap / 1024000);
        println!("free_swap:.................{} Mb", self.free_swap / 1024000);
        println!("used_swap:.................{} Mb", self.used_swap / 1024000);
        println!("available_memory:..........{} Mb", self.available_memory / 1024000);
    }

    #[allow(dead_code)]
    pub fn update_ram(&mut self) {
        *self = Self::new();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperationSystem {
    pub os_type: String,
    pub name_os: Option<String>,
    pub kernel_version:Option<String>,
    pub os_version: Option<String>,
    pub distribution: String,
    pub host_name: Option<String>,
    pub cpu_arch: String,
}

impl OperationSystem {

    pub fn new() -> OperationSystem {
        let os_data: OperationSystem = OperationSystem {
            os_type: sys_info::os_type().unwrap_or("unknown".to_owned()),
            name_os: System::name(),
            kernel_version:System::kernel_version(),
            os_version: System::os_version(),
            distribution: System::distribution_id(),
            host_name: System::host_name(),
            cpu_arch: System::cpu_arch()
        };
        os_data
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("***Operation system***");
        println!("type OS.................{:?}", self.os_type);
        println!("name OS.................{:?}", self.name_os);
        println!("kernel version..........{:?}", self.kernel_version);
        println!("os version..............{:?}", self.os_version);
        println!("distribution............{}", self.distribution);
        println!("host_name...............{:?}", self.host_name);
        println!("cpu_arch................{}", self.cpu_arch);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AllNetworksData {
    interface_name: String,
    network_ip_networks:String,
    network_mac_address: String,
    total_errors_on_received: u64,
    total_errors_on_transmitted: u64,
    total_packets_received: u64,
    total_packets_transmitted: u64,
    mtu: u64,
}
impl AllNetworksData {
    pub fn to_string(&self) -> String {
        "interface name:              ".to_owned() + &(self.interface_name.to_string()) + 
        "\n" + 
        "network ip networks:         " + &(self.network_ip_networks) +
        "\n" + 
        "network mac address:         " + &(self.network_mac_address) +
        "\n" + 
        "total errors on received:    " + &(self.total_errors_on_received.to_string()) +
        "\n" + 
        "total errors on transmitted: " + &(self.total_errors_on_transmitted.to_string()) +
        "\n" + 
        "total packets received:      " + &(self.total_packets_received.to_string()) +
        "\n" + 
        "total packets transmitted:   " + &(self.total_packets_transmitted.to_string()) +
        "\n" + 
        "mtu:                         " + &(self.mtu.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NetworkData {
    pub data_network: Vec<AllNetworksData>,
    pub network_len: i32,
}

impl NetworkData {

    pub fn new() -> NetworkData {
        let networks = Networks::new_with_refreshed_list();
        let mut len: i32 = 0;
        let mut datas: Vec<AllNetworksData> = Vec::new();
        for (interface_name, network) in &networks {
            let ip_str: String = network.ip_networks().into_iter().map(|i| i.to_string()).collect::<String>();
            let tmp = AllNetworksData {
                interface_name: interface_name.to_string(),
                network_ip_networks: ip_str,
                network_mac_address: network.mac_address().to_string(),
                total_errors_on_received: network.total_errors_on_received(),
                total_errors_on_transmitted: network.total_errors_on_transmitted(),
                total_packets_received: network.total_packets_received(),
                total_packets_transmitted: network.total_packets_transmitted(),
                mtu: network.mtu()
            };
            datas.push(tmp);
            len += 1;
        }

        NetworkData {
            data_network: datas,
            network_len: len
        }
}

#[allow(dead_code)]
  pub fn print_data(&self) {
      println!("**NetworksData***");
      println!("count interfaces..................{}", self.network_len);
      for network in &self.data_network {
          println!("interface name................{}", network.interface_name);
          println!("ip............................{}", network.network_ip_networks);
          println!("mac_address...................{}", network.network_mac_address);
          println!("total_packets_received........{}", network.total_packets_received);
          println!("total_packets_transmitted.....{}", network.total_packets_transmitted);
          println!("total_errors_on_received......{}", network.total_errors_on_received);
          println!("total_errors_on_transmitted...{}", network.total_errors_on_transmitted);
          println!("mtu...........................{}", network.mtu);
      }
  }

#[allow(dead_code)]
  pub fn update_network_data(&mut self) {
      *self = Self::new();
  }

}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentData {
    label: String,
    temperature: i64,
    max_temp: i64,
    critical_temp: i64,
}

impl ComponentData {

    pub fn new(label_: String, temperature_: f32,
               max_temp_: f32, critical_temp_: f32) -> ComponentData {
        ComponentData {
            label: label_,
            temperature: ((temperature_) as i64),
            max_temp: ((max_temp_) as i64),
            critical_temp: ((critical_temp_) as i64)
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        
        if self.label.is_empty() {
            " ".to_owned()
        } else {
        "label:               ".to_owned() + &(self.label) + 
        "\n" + 
        "temperature:         " + &(self.temperature.to_string()) +
        "\n" + 
        "max_temp:            " + &(self.max_temp.to_string()) +
        "\n" + 
        "tcritical_temp:      " + &(self.critical_temp.to_string()) +
        "\n"
        }
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("**ComponentData***");
        println!("label...........................{}", self.label);
        println!("temperature.....................{}", self.temperature);
        println!("max_temp........................{}", self.max_temp);
        println!("critical_temp...................{}", self.critical_temp);
    }
}


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentsData {
    pub component_data_v: Vec<ComponentData>
}

impl ComponentsData {

    pub fn new() -> ComponentsData {
        
        let cmpts = Components::new_with_refreshed_list();
        let mut comp_data_v: Vec<ComponentData> = Vec::new();

        for cmpt in cmpts.list() {
            let tmp = ComponentData::new(cmpt.label().to_owned(),
                                                        cmpt.temperature().unwrap_or(0.0), 
                                                        cmpt.max().unwrap_or(0.0),
                                                        cmpt.critical().unwrap_or(0.0));
            comp_data_v.push(tmp);                                      
        }
        
        ComponentsData {
            component_data_v: comp_data_v
        }
    }

    #[allow(dead_code)]
    pub fn update_data(&mut self) {
        *self = Self::new();
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        println!("**ALL Components Data***");
         for cmp in &self.component_data_v {
            cmp.print_data();
         }
    }
}


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AllData {
    pub cpu_data: CpuData,
    pub gpu_data: GpuData,
    pub dram_data: DramData,
    pub ram_data: RamData,
    pub os_data: OperationSystem,
    pub network_data: NetworkData,
    pub components_data: ComponentsData
}

impl AllData {

    pub fn new() -> AllData {
        AllData {
            cpu_data: CpuData::new(),
            gpu_data: GpuData::new(),
            dram_data: DramData::new(),
            ram_data: RamData::new(),
            os_data: OperationSystem::new(),
            network_data: NetworkData::new(),
            components_data: ComponentsData::new()
        }
    }

    #[allow(dead_code)]
    pub fn update_all_data(&mut self) {
        *self = Self::new();
    }

    #[allow(dead_code)]
    pub fn update_all_cpu(&mut self) {
        self.cpu_data.update_cpu_data();
    }

    #[allow(dead_code)]
    pub fn update_all_gpu(&mut self) {
        self.gpu_data.update_gpu();
    }

    #[allow(dead_code)]
    pub fn update_all_disk(&mut self) {
        self.dram_data.update_disk_data();
    }

    #[allow(dead_code)]
    pub fn update_all_ram(&mut self) {
        self.ram_data.update_ram();
    }

    #[allow(dead_code)]
    pub fn update_all_os(&mut self) {
        self.os_data = OperationSystem::new();
    }

    #[allow(dead_code)]
    pub fn update_all_network(&mut self) {
        self.network_data.update_network_data();
    }

    #[allow(dead_code)]
    pub fn update_all_components(&mut self) {
        self.components_data.update_data();
    }
    
}
