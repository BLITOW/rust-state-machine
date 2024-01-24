mod balances;
mod system;

// This is our main runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    // Create a new inctance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {system: system::Pallet::new(), balances: balances::Pallet::new()}
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alise".to_string();
    let bob = "bob".to_string();
    let charlie = "charly".to_string();

    runtime.balances.set_balance(&alice, 100);
    
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);
    let _res = runtime
    .balances
    .transfer(alice.clone(), bob, 30)
    .map_err(|e| eprintln!("{}", e));

    runtime.system.inc_nonce(&alice);
    let _res = runtime
    .balances
    .transfer(alice.clone(), charlie, 20)
    .map_err(|e| eprintln!("{}", e));
    
}
