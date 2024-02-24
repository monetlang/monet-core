mod expr;

use fvm_sdk::initialize;
use fvm_sdk::actor::create_actor;
use cid::Cid;
use cid::multihash::Multihash;
use rand::{Rng, thread_rng};

// const Sha3_256: u64 = 0x16;

fn main() {
    // let digest_bytes = [
    //     0x16, 0x20, 0x64, 0x4b, 0xcc, 0x7e, 0x56, 0x43, 0x73, 0x04, 0x09, 0x99, 0xaa, 0xc8, 0x9e,
    //     0x76, 0x22, 0xf3, 0xca, 0x71, 0xfb, 0xa1, 0xd9, 0x72, 0xfd, 0x94, 0xa3, 0x1c, 0x3b, 0xfb,
    //     0xf2, 0x4e, 0x39, 0x38,
    // ];
    // initialize();
    // let mh = Multihash::<64>::from_bytes(&digest_bytes).unwrap();
    // // let cid = cid::Cid::new_v0(mh).unwrap() as Cid;
    // let cid = Cid::default();
    // let n = thread_rng().gen();
    // create_actor(n, cid, None).unwrap();
}
