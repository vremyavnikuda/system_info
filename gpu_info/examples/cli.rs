//gpu_info/examples/cli.rs
use gpu_info::GpuManager;

/// Prints out information about the first detected GPU, if any.
///
/// The following information is printed out:
/// - The vendor of the GPU
/// - The name of the GPU
/// - The current temperature of the GPU
/// - The current utilization of the GPU
/// - The current power usage of the GPU
/// - The current clock speed of the GPU
///
/// If no GPUs are detected, a message is printed to the console.
fn main() {
    let mut manager = GpuManager::init();
    manager.refresh();

    println!("Vendor (not format): {:?}", manager.vendor_gpu());
    println!("Name (not format): {}", manager.name_gpu());
    println!("{}", manager.format_get_temperature_gpu());
    println!("{}", manager.format_get_utilization_gpu());
    println!("{}", manager.format_get_power_usage_gpu());
    println!("{}", manager.format_get_clock_speed_gpu());
    println!("Utilization (not format): {}", manager.utilization_gpu());
    println!("Clock Speed (not format): {:?}", manager.clock_speed_gpu());
    println!("Active: {}", manager.format_is_active_gpu());
    println!("Power: {}", manager.format_power_usage_gpu());
}
