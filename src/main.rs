use rust_gpu_tools::Device;

fn main() {
    let devices = Device::all();
    for d in devices {
        println!("{:?}", d.cuda_device().unwrap());
    }
}
