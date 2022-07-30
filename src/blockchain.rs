extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    reciever: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String, // Hash
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
pub struct Chain {
    pub chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.gen_new_block();
        chain
    }

    pub fn gen_new_block(&mut self) -> bool {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let header = Blockheader {
            difficulty: self.difficulty,
            timestamp: timestamp as i64,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            reciever: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkel(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);
        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    pub fn new_tx(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        let tx = Transaction {
            sender,
            reciever,
            amount,
        };

        self.curr_trans.push(tx);

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            // Return string with 64 zeros
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };

        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn get_merkel(txs: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for i in &txs {
            let hash = Chain::hash(i);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            // Odd number
            let last = merkle.last().cloned().unwrap();
            merkle.push(last)
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_result = res.to_vec();

        Chain::hex_to_string(vec_result.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("Unable to write");
        }
        s
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            println!("[proof_of_work]: hash: {hash}\t Slice: {slice}");
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }
}
