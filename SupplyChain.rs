use std::collections::HashMap;
use std::io;
struct SupplyChain {
    inventory: HashMap<String, u32>,
}

impl SupplyChain {
    fn new() -> SupplyChain {
        SupplyChain {
            inventory: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: &str, quantity: u32) {
        let current_quantity = self.inventory.entry(item.to_string()).or_insert(0);
        *current_quantity += quantity;
    }

    fn remove_item(&mut self, item: &str, quantity: u32) -> Result<(), String> {
        if let Some(current_quantity) = self.inventory.get_mut(item) {
            if *current_quantity >= quantity {
                *current_quantity -= quantity;
                return Ok(());
            }
        }
        Err(format!("Insufficient quantity of {} in inventory", item))
    }

    fn check_inventory(&self, item: &str) -> u32 {
        *self.inventory.get(item).unwrap_or(&0)
    }
}

fn main() {
    let mut supply_chain = SupplyChain::new();
    let mut var=-1;
    while var==-1
    {
        println!("Enter the Operation:");
        println!(" 1)Order Ammo \n 2)Remove Order \n 3)Inventory Available \n 4)Exit(Press -1)");
        println!("Choice:");
        let mut a=String::new();
        io::stdin().read_line(&mut a).expect("Error");
        let mut ai:i32=a.trim().parse().unwrap();
        if ai==1
        {
            println!("Select the product:");
            println!(" 1) Pistol Auto 9mm 1A \n 2) Glock \n 3) Beretta px4 4) ShootEdge \n 5) Micro-uzi \n 6) AK 47 \n 7) M4 Carbine \n 8) AR-M1 \n 9) AKM \n 10) Heckler& Koch PSG1");
            let mut prodnum=String::new();
            println!("Enter the product No:");
            io::stdin().read_line(&mut prodnum).expect("error");
            let mut prodnumint:u32=prodnum.trim().parse().unwrap();
            let mut quantity=String::new();
            println!("Enter Quantity:");
            io::stdin().read_line(&mut quantity).expect("error");
            let mut quantityint:u32=quantity.trim().parse().unwrap();
            let choosen=match prodnumint{
                1=>"Pistol Auto 9mm 1A",
                2=>"Glock",
                3=>"Beretta px4",
                4=>"ShootEdge",
                5=>"Micro-uzi",
                6=>"AK 47",
                7=>"M4 Carbine",
                8=>"AR-M1",
                9=>"AKM",
                10=>"Heckler & Koch PSG1",
                _=>"New Weapons are yet to be uploaded"
            };
            supply_chain.add_item(choosen,quantityint);
            println!("Sucessfully placed order of {} '{}'",quantityint,choosen);
        }
        else if ai==2
        {
            let mut prodnum=String::new();
            println!("Enter product No:");
            io::stdin().read_line(&mut prodnum).expect("error");
            let mut prodnumint:u32=prodnum.trim().parse().unwrap();
            let mut quantity=String::new();
            println!("Enter Quantity:");
            io::stdin().read_line(&mut quantity).expect("error");
            let mut quantityint:u32=quantity.trim().parse().unwrap();
            let choosen_delete=match prodnumint{
                1=>"Pistol Auto 9mm 1A",
                2=>"Glock",
                3=>"Beretta px4",
                4=>"ShootEdge",
                5=>"Micro-uzi",
                6=>"AK 47",
                7=>"M4 Carbine",
                8=>"AR-M1",
                9=>"AKM",
                10=>"Heckler & Koch PSG1",
                _=>"New Weapons are yet to be uploaded"
            };
            match supply_chain.remove_item(choosen_delete,quantityint)
            {
                Ok(_)=>println!("{} '{}' Order has Cancelled",quantityint,choosen_delete),
                Err(e)=>println!("Try Again!"),
            }
        }
        else if ai==3
        {
            println!("Select the product:");
            println!(" 1) Pistol Auto 9mm 1A \n 2) Glock \n 3) Beretta px4 4) ShootEdge \n 5) Micro-uzi \n 6) AK 47 \n 7) M4 Carbine \n 8) AR-M1 \n 9) AKM \n 10) Heckler& Koch PSG1");
            let mut prodnum=String::new();
            println!("Enter the product No:");
            io::stdin().read_line(&mut prodnum).expect("error");
            let prodnumint:u32=prodnum.trim().parse().unwrap();
            let choosen_inventory=match prodnumint{
                1=>"Pistol Auto 9mm 1A",
                2=>"Glock",
                3=>"Beretta px4",
                4=>"ShootEdge",
                5=>"Micro-uzi",
                6=>"AK 47",
                7=>"M4 Carbine",
                8=>"AR-M1",
                9=>"AKM",
                10=>"Heckler & Koch PSG1",
                _=>"New Weapons are yet to be uploaded"
            };
            println!("{} '{}' Order is Live now!",supply_chain.check_inventory(choosen_inventory),choosen_inventory);
        }
        else if ai==-1
        {
            break;
        }
    }
}
