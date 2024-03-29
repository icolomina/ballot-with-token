use soroban_sdk::{ contracttype, symbol_short, Address, Env, Symbol, Vec};

pub const VOTES: Symbol = symbol_short!("votes");
pub const PARTIES: Symbol = symbol_short!("parties");
pub const CONFIG: Symbol = symbol_short!("config");
pub const TOKEN: Symbol = symbol_short!("token");

#[derive(Debug)]
#[contracttype]
pub struct Config {
    pub from: u64,
    pub to: u64
}

impl Default for Config {
    fn default () -> Config {
        Config { from: 0, to: 0 }
    }
}

#[contracttype]
pub enum VCounter {
    Counter(Symbol)
}

pub fn get_candidates(env: &Env) -> Vec<Symbol> {
    let pts: Vec<Symbol> = env
        .storage()
        .instance()
        .get(&PARTIES)
        .unwrap_or(Vec::new(env))
    ;

    pts
}

pub fn store_party(env: &Env, p: &Symbol) -> bool {
    let mut pts: Vec<Symbol> = get_candidates(env);
    if !pts.contains(p) {
        pts.push_back(p.clone());
        env.storage().instance().set(&PARTIES, &pts);
        return true;
    }

    false
}

pub fn get_votes(env: &Env) -> Vec<Address>{
    let vts: Vec<Address> = env
        .storage()
        .instance()
        .get(&VOTES)
        .unwrap_or(Vec::new(env))
    ;

    vts
}

pub fn get_candidate_votes_count(env: &Env, candidate: &VCounter) -> u32 {
    let total_votes = env.storage().instance().get(candidate).unwrap_or(0);
    total_votes
}

pub fn update_candidate_count(env: &Env, candidate: VCounter, count: u32) {
    env.storage().instance().set(&candidate, &count);
}

pub fn update_votes(env: &Env, votes: Vec<Address>) {
    env.storage().instance().set(&VOTES, &votes);
}

pub fn store_config(env: &Env, ts_start: u64, ts_end: u64, token: Address) {
    let cfg = Config {
        from: ts_start,
        to: ts_end
    };

    env.storage().instance().set(&CONFIG, &cfg);
    env.storage().instance().set(&TOKEN, &token)
}

pub fn get_config(env: &Env) -> Config {
    let cfg = env
        .storage()
        .instance()
        .get(&CONFIG)
        .unwrap_or(Config::default())
    ;

    cfg
}

pub fn get_token(env: &Env) -> Address {
    let token: Address = env.storage().instance().get(&TOKEN).unwrap();
    token
}