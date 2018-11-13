extern crate raw_cpuid;

use raw_cpuid::CpuId;

// Architecture:          x86_64
// CPU op-mode(s):        32-bit, 64-bit
// Byte Order:            Little Endian
// CPU(s):                4
// On-line CPU(s) list:   0-3
// Thread(s) per core:    1
// Core(s) per socket:    4
// Socket(s):             1
// NUMA node(s):          1
// Vendor ID:             GenuineIntel
// CPU family:            6
// Model:                 70
// Model name:            Intel(R) Core(TM) i7-4770HQ CPU @ 2.20GHz
// Stepping:              1
// CPU MHz:               2194.918
// BogoMIPS:              4389.83
// Hypervisor vendor:     KVM
// Virtualization type:   full
// L1d cache:             32K
// L1i cache:             32K
// L2 cache:              256K
// L3 cache:              6144K
// L4 cache:              131072K
// NUMA node0 CPU(s):     0-3
// Flags:                 fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc eagerfpu pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt aes xsave avx rdrand hypervisor lahf_lm abm fsgsbase avx2 invpcid


fn main() {
    let cpu_info = CpuId::new();

    println!("{:?}", cpu_info.get_processor_frequency_info());

    if let Some(vendor_info) = cpu_info.get_vendor_info() {
        println!("Vendor: {}", vendor_info.as_string());
    }

    if let Some(features_info) = cpu_info.get_feature_info() {
        println!("Extended Family ID: {:?}", features_info.extended_family_id());
        println!("Extended Model ID: {:?}", features_info.extended_model_id());
        println!("Family ID: {:?}", features_info.family_id());
        println!("Model ID: {:?}", features_info.model_id());
        println!("Stepping ID: {:?}", features_info.stepping_id());
        println!("Brand Index: {:?}", features_info.brand_index());
        
    }

    if let Some(ext_func_info) = cpu_info.get_extended_function_info() {
        if let Some(brand_str) = ext_func_info.processor_brand_string() {
            println!("Brand Name: {}", brand_str);
        }

        if ext_func_info.has_64bit_mode() {
            println!("Arch: 64-bit");
        }
        
    }

    if let Some(cache_info_iter) = cpu_info.get_cache_info() {
        for cache_info in cache_info_iter {
            println!("Cache Info: {:?}", cache_info.desc() );
        }        
    }
    
    if let Some(cache_parameters) = cpu_info.get_cache_parameters() {
        for cache_parameter in cache_parameters {

            println!("L{} Cache: {}K physical_line_partitions({:?}))", 
                cache_parameter.level(),
                cache_parameter.max_cores_for_package() * cache_parameter.coherency_line_size() / cache_parameter.max_cores_for_cache(),
                cache_parameter.physical_line_partitions(),
                 );
        }        
    }
    
    // L1/L2/L3 ?
    // get_rdt_allocation_info
    if let Some(ext_topology_iter) = cpu_info.get_extended_topology_info() {
        for ext_topology_level in ext_topology_iter {
            println!("Processors: {:?} level_number: {:?} level_type: {:?}",
                ext_topology_level.processors(),
                ext_topology_level.level_number(),
                ext_topology_level.level_type(),
                );
        }
    }
    
    if let Some(hypervisor_info) = cpu_info.get_hypervisor_info() {
        println!("{:?}", hypervisor_info.tsc_frequency());
        match hypervisor_info.identify() {
            raw_cpuid::Hypervisor::Xen => println!("Hypervisor: XEN"),
            raw_cpuid::Hypervisor::VMware => println!("Hypervisor: VMware"),
            raw_cpuid::Hypervisor::HyperV => println!("Hypervisor: HyperV"),
            raw_cpuid::Hypervisor::KVM => println!("Hypervisor: KVM"),
            raw_cpuid::Hypervisor::Unknown(a, b, c) => println!("Hypervisor: Unknow({}, {}, {})", a, b, c),
        }
    } else {
        println!("Hypervisor: None");
    }

    if let Some(soc_vendor_info) = cpu_info.get_soc_vendor_info() {
        println!("Soc Vendor Info: {}", soc_vendor_info.get_vendor_brand().as_string());
    }
}
