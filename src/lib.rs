// Working
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, String, log, Symbol, symbol_short};

const COUNT_MEME : Symbol = symbol_short!("CNT_MEME");
const MEME_OV : Symbol = symbol_short!("MEME_OV");

#[contracttype]
pub enum Memeoverviewbook {
    Memeoverview(u32)
}

#[contracttype]
#[derive(Clone)]
pub struct Memeoverview {
    pub meme_id: u32, 
    pub likes: u32,
    pub dislikes: u32,
}

#[contracttype]
pub enum Memebook {
    Meme(u32)
}


#[contracttype]
#[derive(Clone, Debug)]
pub struct Meme {
    meme_id: u32,
    ipfs_cid: String,
}


#[contract]
pub struct Memepalace;


#[contractimpl]
impl Memepalace {
    pub fn post_meme(env: Env, meme_ipfs_cid: String) -> u32 {
        let mut meme_count: u32 = env.storage().instance().get(&COUNT_MEME).unwrap_or(0);
            meme_count += 1;

        let mut meme_details = Self::fetch_meme(env.clone(), meme_count.clone());
        
        meme_details.meme_id = meme_count;
        meme_details.ipfs_cid = meme_ipfs_cid;

        env.storage().instance().set(&Memebook:: Meme(meme_details.meme_id.clone()), &meme_details);
        env.storage().instance().set(&COUNT_MEME, &meme_details.meme_id.clone());
        env.storage().instance().extend_ttl(5000, 5000);

        return meme_details.meme_id ;        
    }


    pub fn like_meme(env: Env, meme_id: u32) {
        let mut meme_overview = Self::fetch_meme_overview(env.clone(), meme_id.clone());

        meme_overview.meme_id = meme_id.clone();
        meme_overview.likes += 1;

        env.storage().instance().set(&Memeoverviewbook:: Memeoverview(meme_id.clone()), &meme_overview);
    }

    
    pub fn dislike_meme(env: Env, meme_id: u32) {
        let mut meme_overview = Self::fetch_meme_overview(env.clone(), meme_id.clone());

        meme_overview.meme_id = meme_id.clone();
        meme_overview.dislikes += 1;

        env.storage().instance().set(&Memeoverviewbook:: Memeoverview(meme_id.clone()), &meme_overview);
    }
    

    pub fn fetch_meme_overview(env:Env, meme_id: u32) -> Memeoverview {
        let key = Memeoverviewbook::Memeoverview(meme_id.clone()); 
        
        env.storage().instance().get(&key).unwrap_or(Memeoverview {
            meme_id: 0,
            likes: 0,
            dislikes: 0,
        })
    }

    
    pub fn fetch_meme(env: Env, meme_id: u32) -> Meme {
        let key = Memebook::Meme(meme_id.clone());

        env.storage().instance().get(&key).unwrap_or(Meme {
            meme_id: 0,
            ipfs_cid: String::from_str(&env, "Invalid meme ID!")
        })
    }
}