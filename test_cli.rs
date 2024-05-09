
import bs58 from 'bs58'
import * as prompt from 'prompt-sync

#[test]
fn base58_to_wallet() {
println!("Enter your name:");
let stdin = io::stdin();
let base58 = stdin.lock().lines().next().unwrap().unwrap(); //
gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7Vv
ksUGhPt4SZ8JHVSkt
let wallet = bs58::decode(base58).into_vec().unwrap();
println!("{:?}", wallet);
}
#[test]
fn wallet_to_base58() {
let wallet: Vec<u8> =
vec![112,129,31,130,75,43,197,220,12,122,1,198,86,217,111,126,104,121,138,98,127,101,108,107,189,90,43,114,166,242,64,212,246,156,176,249,26,26,237,156,225,104,3,113,206,99,176,155,84,224,231,235,60,131,136,45,65,232,126,171,132,190,139,54];
let base58 = bs58::encode(wallet).into_string();
println!("{:?}", base58);
}