use ink_storage::collections::Vec as InkVec;

#[ink::contract]
mod family_details {
    #[ink(storage)]
    pub struct FamilyDetails {
        father_name: String,
        mother_name: String,
        sibilings: InkVec<String>,
        occupation: String,
        annual_income: u64,
    }

    impl FamilyDetails {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                father_name: String::new(),
                mother_name: String::new(),
                sibilings: InkVec::new(),
                occupation: String::new(),
                annual_income: 0,
            }
        }

        #[ink(message)]
        pub fn add_family_details(&mut self, father_name: String, mother_name: String, sibilings: InkVec<String>, occupation: String, annual_income: u64) {
            self.father_name = father_name;
            self.mother_name = mother_name;
            self.sibilings = sibilings;
            self.occupation = occupation;
            self.annual_income = annual_income;
        }
    }
