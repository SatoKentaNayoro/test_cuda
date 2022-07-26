use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
// use rust_gpu_tools::Device;
// use fil_rustacuda::device::Device as Fil_Device;

fn main() {
    // let devices = Device::all();
    // for d in devices {
    //     println!("{:?}", d.cuda_device().unwrap());
    // }
    // println!("{:?}", Fil_Device::devices().unwrap());
    let t_vec: RwLock<Vec<(u8, AtomicBool)>> = RwLock::new(Vec::new());
    let mut guard = t_vec.write().unwrap();
    guard.push((1, AtomicBool::new(false)));
    drop(guard);
    let mut guard_r = t_vec.write().unwrap();
    if !guard_r[0].1.load(Ordering::SeqCst) {
        guard_r[0].0 = 2
    }
    drop(guard_r);
    println!("{:?}", t_vec.read().unwrap()[0])
}
