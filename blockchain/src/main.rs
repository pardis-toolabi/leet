use std::time;
use ethers::{core::k256::elliptic_curve::rand_core::block, utils::keccak256};

#[derive(Clone)]

struct Block{
    number: u128,
    hash: String,
    priv_hash : String,
    data: String,
    time_stamp: u128
}
impl Block {

    fn new(number: u128, priv_hash:String, data: String, time_stamp: u128)-> Block{

        let mut b = Block { number, hash: String::new(), priv_hash, data, time_stamp};
        b.hash();
        b
    }

    fn hash(&mut self)-> &mut Self{
        let data = format!("{}{}{}{}",self.number, self.priv_hash, self.data, self.time_stamp);
        let bytes: Vec<u8> = data.bytes().collect();
        let hash = hex::encode(keccak256(bytes));
        self.hash=hash;
        self
    }
    
}


#[derive(Clone)]
struct Blockchain {
    id: u64,
    blocks: Vec<Block>
}

impl Blockchain {

    fn new()->Blockchain{
        let genesis = Block::new(0, String::from("0"), String::from("genesis block"), 0);
        Blockchain{
            id: rand::random::<u64>(),
            blocks: [genesis].to_vec()
        }
    }
    fn add_block(&mut self, data: String){
        let priv_b = self.blocks.last().expect("falied to get the last block number");
        let block =Block::new(priv_b.number+1, priv_b.hash.clone(), data, priv_b.time_stamp+1);
        self.blocks.push(block);

    }
    fn get_block(&self, number: u128)-> Block{
        self.blocks[number as usize].clone()
    }
    fn validate(&self)->bool{


        let blocks = self.blocks.clone();

        if blocks[0].number != 0 || blocks[0].priv_hash != String::from("0"){
            return false;
        }
        for i in blocks.windows(2) {
            let (pri, cur)= (&i[0], &i[1]);
            if cur.number != pri.number+1{
                return false;
            }

            if cur.priv_hash != pri.hash {
                return false;
            }
        }
        true

    }
    
}
fn main() {
    let mut chain = Blockchain::new();

    chain.add_block(String::from("second block"));
    chain.add_block(String::from("third block"));
    chain.add_block(String::from("forth block"));

    assert!(chain.validate());

}
