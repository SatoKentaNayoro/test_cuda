use rust_gpu_tools::Device;

fn main() {
    let devices = Device::all();
    println!("{:?}", devices);
}
