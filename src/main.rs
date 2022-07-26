use rust_gpu_tools::Device;
use fil_rustacuda::device::Device as Fil_Device;

fn main() {
    let devices = Device::all();
    for d in devices {
        println!("{:?}", d.cuda_device().unwrap());
    }
    println!("{:?}",Fil_Device::devices().unwrap());
}
