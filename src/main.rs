use std::time::Instant;
use rand::{Rng, RngCore};
use snarkvm_algorithms::coinbase_puzzle::{Address, CoinbasePuzzle, EpochInfo};
use snarkvm_algorithms::fft::DensePolynomial;
use snarkvm_algorithms::polycommit::kzg10::{Powers, VerifierKey};
use snarkvm_curves::bls12_377::Bls12_377;
use snarkvm_utilities::rand::Uniform;
use snarkvm_utilities::ToBytes;

fn main() {
    // let max_degree = 1 << 15;
    // let mut rng = rand::thread_rng();
    //
    // let srs = CoinbasePuzzle::<Bls12_377>::setup(max_degree, &mut rng);
    // for log_degree in 5..10 {
    //     let degree = (1 << log_degree) - 1;
    //     let product_degree = (1 << (log_degree + 1)) - 1;
    //     let (pk, vk) = CoinbasePuzzle::trim(&srs, product_degree);
    //     // println!("PK: {:?}", pk.vk.to_bytes_le().unwrap());
    //     let epoch_info = EpochInfo { epoch_number: rng.next_u64() };
    //     let epoch_challenge = CoinbasePuzzle::init_for_epoch(&epoch_info, degree);
    //     let polynomial = DensePolynomial::from_coefficients_slice(&epoch_challenge.epoch_polynomial.coeffs);
    //     assert_eq!(epoch_challenge.epoch_polynomial, polynomial);
    //     for batch_size in 1..10 {
    //         let solutions = (0..batch_size)
    //             .map(|_| {
    //                 let address = Address(<[u8; 32]>::rand(&mut rng));
    //                 let x = address.to_bytes_le();
    //                 assert_eq!(hex::encode(address.0), hex::encode(x));
    //                 let nonce = u64::rand(&mut rng);
    //                 CoinbasePuzzle::prove(&pk, &epoch_info, &epoch_challenge, &address, nonce)
    //             })
    //             .collect::<Vec<_>>();
    //         let full_solution = CoinbasePuzzle::accumulate(&pk, &epoch_info, &epoch_challenge, &solutions);
    //         assert!(CoinbasePuzzle::verify(&vk, &epoch_info, &epoch_challenge, &full_solution));
    //         let bad_epoch_info = EpochInfo { epoch_number: rng.next_u64() };
    //         let bad_epoch_challenge = CoinbasePuzzle::init_for_epoch(&bad_epoch_info, degree);
    //         assert!(!CoinbasePuzzle::verify(&vk, &bad_epoch_info, &bad_epoch_challenge, &full_solution));
    //     }
    // }

    let start = Instant::now();
    let rand1 = uuid::Uuid::new_v4();
    println!("{}", rand1.to_string().as_str());
    println!("uuid cost: {:?}", Instant::now().saturating_duration_since(start));

    let start = Instant::now();
    let rand1 = rand::thread_rng().gen_range(0..u64::MAX);
    // let rand2 = rand::thread_rng().gen_range(0..u64::MAX);
    // let rand3 = rand::thread_rng().gen_range(0..u64::MAX);
    println!("{}", rand1.to_string().as_str());
    println!("u64 cost {:?}", Instant::now().saturating_duration_since(start));
    // let start = Instant::now();
    // let rand = format!("{}", hex::encode(rand1.to_le_bytes()));
    // let r = hex::encode(rand);
    // println!("{:?}", Instant::now().saturating_duration_since(start));
    // println!("{}", r);
    // let start = Instant::now();
    // let rand = hex::encode(rand1.to_le_bytes());
    // let r = hex::encode(rand);
    // println!("{:?}", Instant::now().saturating_duration_since(start));
    // println!("{}", r);
}
