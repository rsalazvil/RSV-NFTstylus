use stylus::{account, contract, deploy};

#[contract]
pub struct RSV {
    next_token_id: u64,
    owner: account::Address,
}

impl RSV {
    
    pub fn new(owner: account::Address) -> Self {
        RSV {
            next_token_id: 0,
            owner,
        }
    }

    
    pub fn base_uri() -> &'static str {
        "https://gateway.lighthouse.storage/ipfs/bafkreibzidfnwztjteak3wip3lodfrd366omc4nhmmwypw4chaorwii5aa"
    }

    
    pub fn safe_mint(&mut self, to: account::Address) {
        if self.owner != to {
            panic!("Only the owner can mint tokens");
        }

        let token_id = self.next_token_id;
        self.next_token_id += 1;

        
        
    }
}


#[deploy]
pub fn deploy(owner: account::Address) -> RSV {
    RSV::new(owner)
}
