use std::io;


struct PersonalData{
    Full_name:String,
    Dob: String,
    Nationality:String,
    Phone_number:u64,
    Address: String,
    Email_address: String,
    Maritial_status: String,
    Criminal_history: u32
}
impl Default for PersonalData
{
    fn default()-> Self
    {
        PersonalData{
            Full_name:"".to_string(),
            Dob:"DD/MM/YYYY/".to_string(),
            Nationality:"INDIAN".to_string(),
            Phone_number:0,
            Address:" ".to_string(),
            Email_address:" ".to_string(),
            Maritial_status:"UNMARRIED".to_string(),
            Criminal_history:0
            
        }
    }
}
struct Emergency
{
    name:String,
    relationship:String,
    Phone_number:u64,
    alternate_phon_numbber:u64,
    email:String
}
struct FamilyDetails
{
    Father_Name:String,
    Mother_Name:String,
    Silibigs:Vec<String>,
    annual_income:u64
}

fn main()
{
    println!("-------------------------");
    println!("     PERSONAL DATA       ");
    println!("-------------------------");
    
    println!("Full Name(as per Aadhar):");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("Error while taking input");
    println!("Enter Date-of-birth(DD/MM/YYYY):");
    let mut dob=String::new();
    io::stdin().read_line(&mut dob).expect("Error while taking input");
    println!("Phone Number:");
    let mut phone_num=String::new();
    io::stdin().read_line(&mut phone_num).expect("Error While taking from input");
    let num:u64=phone_num.trim().parse().unwrap();
    println!("Address:");
    let mut address=String::new();
    io::stdin().read_line(&mut address).expect("Error while taking input");
    println!("Email:");
    let mut email=String::new();
    io::stdin().read_line(&mut email).expect("Error while taking input");
    println!("Maritial Status:");
    let mut ms=String::new();
    io::stdin().read_line(&mut ms).expect("Errow while taking input");
    
    let mut soldier_1=PersonalData::default();
    soldier_1.Full_name=name.to_uppercase();
    soldier_1.Dob=dob;
    soldier_1.Phone_number=num;
    soldier_1.Address=address;
    soldier_1.Email_address=email;
    soldier_1.Maritial_status=ms.to_uppercase();
    println!("Personal Data is Successfully Entered");
    
    println!("-------------------------");
    println!("   EMERGENCY CONTACT     ");
    println!("-------------------------");
    
    let mut Name=String::new();
    println!("Emergency contact Name:");
    io::stdin().read_line(&mut Name).expect("Error in taking input");
    let mut Relationship=String::new();
    println!("Relation:");
    io::stdin().read_line(&mut Relationship).expect("Error in taking input");
    println!("Phone_number:");
    let mut pn=String::new();
    io::stdin().read_line(&mut pn).expect("Error in taking input");
    let mut num:u64=pn.trim().parse().unwrap();
    println!("Alter Phone Number:");
    let mut alt_ph_num=String::new();
    io::stdin().read_line(&mut alt_ph_num).expect("Error in taking input");
    let mut iapn:u64=alt_ph_num.trim().parse().unwrap();
    println!("E-mail:");
    let mut email_string=String::new();
    io::stdin().read_line(&mut email_string).expect("Error in taking input");
    let soldier_1=Emergency{name:Name,relationship:Relationship,Phone_number:num,alternate_phon_numbber:iapn,email:email_string};
    println!("Emergency Contact is Successfully Entered!");
    
    println!("-------------------------");
    println!("      FAMILY DETAILS     ");
    println!("-------------------------");
    
     println!("Father Name:");
    let mut fnn=String::new();
    io::stdin().read_line(&mut fnn).expect("Error while taking input");
    println!("Mother Name:");
    let mut mn=String::new();
    io::stdin().read_line(&mut mn).expect("Error while taking input");
    println!("No. of. Silibigs:");
    let mut sibili=String::new();
    io::stdin().read_line(&mut sibili).expect("Error while takng input");
    let isibili:u32=sibili.trim().parse().unwrap();
    let mut bs=Vec::new();
    for i in 0..isibili
    {
        let mut temp=String::new();
        println!("Enter the name of {} sibiling:",i+1);
        io::stdin().read_line(&mut temp).expect("Error while taking input:");
        bs.push(temp.trim().to_string());
    }
    println!("Annual Income:");
    let mut ai=String::new();
    io::stdin().read_line(&mut ai).expect("Error while taking input");
    let mut iai:u64=ai.trim().parse().unwrap();
    let soldier_1=FamilyDetails{Father_Name:fnn,Mother_Name:mn,Silibigs:bs,annual_income:iai};
    println!("Family Details are Successfully Entered");
}
