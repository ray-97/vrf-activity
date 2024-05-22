use rand::{rngs::OsRng};
use schnorrkel::{Keypair,Signature};
use rand::distributions::{Alphanumeric, DistString};
use std::collections::HashSet;
use schnorrkel::context::signing_context;

// our simplified implementation - no hands, show all at once
// one player gets 2 cards,
// each card is u8, and have a value of 0-51 (hence mod 52)
// a set of u8 keeps track of what is already drawn
// for score we just sum up hands value (hence table don't need to exist)
// (this has became a simplified blackjack instead xD),
// we didn't want to spend too much time coding poker rules etc
// for simplicity we have 3 players.

// vrf:
// - choose key then choose input.
//   eval and then only key holder knows output.
//   keyholder reveals it by publishing vrf proof / signature

// flow:
// 1. generate pk and sk for each player
// 2. each player choose their cards / inputs
// 3. system computes eval(sk, input=player_num*100+card_num) -> output for each player, where output is score
// 4. sign(sk, input) -> signature when player reveal card for verification

fn main() {
    let mut history = HashSet::new();
    let mut player_scores = vec![];
    while player_scores.len() < 2 {
        let keypair: Keypair = Keypair::generate_with(OsRng);
        let mut score = 0;
        let mut counter = 2;
        while counter > 0 {
            let context = signing_context(b"vrf");
            let rand_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
            let message: &[u8] = rand_string.as_bytes();
            let signature: Signature = keypair.sign(context.bytes(message));
            let out_hex = hex::encode(signature.to_bytes());
            let out_u32 = (sp_core::U512::from_str_radix(out_hex.as_str(), 16).unwrap()%52).as_u32();
            if !history.contains(&out_u32) {
                score+=out_u32;
                counter-=1;
                history.insert(out_u32);
            }
        }
        player_scores.push(score);
    }
    let max_val = player_scores.iter().min().unwrap();
    for (i, score) in player_scores.iter().enumerate() {
        if score == max_val {
            println!("Player {} wins with score {}", i+1, score);
        }
    }
}