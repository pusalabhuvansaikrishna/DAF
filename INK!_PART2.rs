enum Product {
    PistolAuto9mm1A,
    Glock,
    BerettaPx4,
    ShootEdge,
    MicroUzi,
    AK47,
    M4Carbine,
    ARM1,
    AKM,
    HecklerAndKochPSG1,
}

struct SupplyChain {
    inventory: ink_env::collections::HashMap<Product, u32>,
}

impl SupplyChain {
    #[ink(constructor)]
    fn new(&mut self) {
        self.inventory = ink_env::collections::HashMap::new();
    }

    #[ink(message)]
    fn add_item(&mut self, item: Product, quantity: u32) {
        let current_quantity = self.inventory.entry(item).or_insert(0);
        *current_quantity += quantity;
    }

    #[ink(message)]
    fn remove_item(&mut self, item: Product, quantity: u32) -> Result<(), String> {
        if let Some(current_quantity) = self.inventory.get_mut(&item) {
            if *current_quantity >= quantity {
                *current_quantity -= quantity;
                return Ok(());
            }
        }
        Err(format!("Insufficient quantity of {:?} in inventory", item))
    }

    #[ink(message)]
    fn check_inventory(&self, item: Product) -> u32 {
        *self.inventory.get(&item).unwrap_or(&0)
    }
}

#[ink::contract]
mod supply_chain {
    use super::{Product, SupplyChain};

    #[ink(storage)]
    pub struct SupplyChainContract {
        supply_chain: SupplyChain,
    }

    impl SupplyChainContract {
        #[ink(constructor)]
        pub fn new(&mut self) {
            self.supply_chain = SupplyChain::new();
        }

        #[ink(message)]
        pub fn add_item(&mut self, item: Product, quantity: u32) {
            self.supply_chain.add_item(item, quantity);
        }

        #[ink(message)]
        pub fn remove_item(&mut self, item: Product, quantity: u32) -> Result<(), String> {
            self.supply_chain.remove_item(item, quantity)
        }

        #[ink(message)]
        pub fn check_inventory(&self, item: Product) -> u32 {
            self.supply_chain.check_inventory(item)
        }
    }
}
