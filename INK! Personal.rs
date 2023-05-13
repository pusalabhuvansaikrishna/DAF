use ink_lang::contract;

#[contract]
mod emergency_contact {
    #[derive(Default)]
    pub struct Emergency {
        name: String,
        relationship: String,
        Phone_number: u64,
        alternate_phon_numbber: u64,
        email: String,
    }

    impl Emergency {
        #[ink(constructor)]
        pub fn new(name: String, relationship: String, phone_number: u64, alternate_phon_numbber: u64, email: String) -> Self {
            Self {
                name: name,
                relationship: relationship,
                Phone_number: phone_number,
                alternate_phon_numbber: alternate_phon_numbber,
                email: email,
            }
        }

        #[ink(message)]
        pub fn set_emergency_contact(&mut self, name: String, relationship: String, phone_number: u64, alternate_phon_numbber: u64, email: String) {
            self.name = name;
            self.relationship = relationship;
            self.Phone_number = phone_number;
            self.alternate_phon_numbber = alternate_phon_numbber;
            self.email = email;
        }

        #[ink(message)]
        pub fn get_emergency_contact(&self) -> (String, String, u64, u64, String) {
            (self.name.clone(), self.relationship.clone(), self.Phone_number, self.alternate_phon_numbber, self.email.clone())
        }
    }
