use std::rc::Rc;

use ethers::{types::Address, utils::keccak256};

#[derive(Clone, Debug)]
struct Node {
    hash: String,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

#[derive(Clone)]

struct Tree{
    root: Node,
    leaves: Vec<String>,
    layer: Vec<Vec<Node>>
}

impl Tree {
    fn new(leaves: Vec<String>)->Tree{
        // check the length
        let height = (leaves.len() as f64).log2();
        let mut layer: Vec<Node>= leaves.iter().map(|x| Node{hash: x.to_string(), left:None, right:None}).collect();
        let mut tree_layers = Vec::new();
        while layer.len() != 1 {
            let mut next_layer: Vec<Node> = Vec::new();
            for i in (0 .. layer.len()).step_by(2) {
                let data = format!("{}{}",layer[i].clone().hash,layer[i+1].clone().hash);
                let hash = hex::encode(keccak256(data.bytes().collect::<Vec<u8>>()));
                next_layer.push(Node { hash, left:Some(Rc::new(layer[i].clone())), right:Some(Rc::new(layer[i+1].clone())) });
            }

            layer = next_layer.clone();
            tree_layers.push(next_layer);

        }

        Tree { root: layer[0].clone(), leaves, layer:tree_layers }
    }
    fn get_root(&self)-> String{
        self.root.clone().hash
    }
    fn get_proof(&self, leaf: String)->Vec<String>{
        let mut index = 0;
        for (i, item) in self.leaves.iter().enumerate(){
            if *item == leaf{
                index = i;
            }
        }
        let path_indices = get_binary(index as u128);
        let mut proof = Vec::new();
        let mut new_index = 0;
        if index % 2 == 0 {
            proof.push(self.leaves[index+1].clone());
        }else {
            proof.push(self.leaves[index-1].clone());
        }
        new_index = index / 2;
        println!("new_index:{:?}", new_index);

        for i in self.layer.clone(){
            if i.len()!= 1{
                if new_index % 2 == 0 {
                    proof.push(i[new_index+1].hash.clone());
                }else {
                    proof.push(i[new_index-1].hash.clone());
                }
                new_index = new_index /2;
            }

        }


        proof


    }
    
}


fn get_binary(index: u128)-> Vec<u128>{
    let binary_len = 128 - index.leading_zeros();
    let mut binary: Vec<u128> = vec![0; binary_len as usize];
    for i in 0 .. binary_len as usize{
        binary[(binary_len as usize)-1-i]=(index >> i)  & 1 ;
    }
    binary
}
fn main() {
    println!("Hello, world!");
    let mut trx_hashes = Vec::new();
    for i in 0 .. 4{
        trx_hashes.push(hex::encode(keccak256(i.to_string())));
    }

    let a = format!("{}{}",trx_hashes[0], trx_hashes[1]);
    let b = format!("{}{}",trx_hashes[2], trx_hashes[3]);
    let aa = hex::encode(keccak256(a.bytes().collect::<Vec<u8>>()));
    let bb = hex::encode(keccak256(b.bytes().collect::<Vec<u8>>()));

    let c = format!("{}{}",aa,bb);
    let cc = hex::encode(keccak256(c.bytes().collect::<Vec<u8>>()));

    let t = Tree::new(trx_hashes.clone());
    println!("trx_hashes:{:?}", trx_hashes);
    println!("t.root:{:?}", t.root);
    println!("cc:{:?}", cc);
    println!("cc:{:?}", t.get_proof(trx_hashes[3].clone()));
}
