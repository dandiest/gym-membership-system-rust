use std::io;

#[derive(Debug)]
enum SubscriptionType {
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug)]
struct Member {
    name: String,
    age: u32,
    sub_type: SubscriptionType,
    price: f32,
}

fn main() {
    println!("How many new members do you want to register?");
    let mut total_members: Vec<Member> = Vec::new();
    let mut members_raw = String::new();
    io::stdin()
        .read_line(&mut members_raw)
        .expect("Error during reading.");

    let members_clean: usize = members_raw
        .trim()
        .parse()
        .expect("Please, enter a valid member number.");

    for i in 0..members_clean {
        println!("What is your name?");
        let mut name_r = String::new();

        io::stdin()
            .read_line(&mut name_r)
            .expect("Error during reading.");

        let name_c = name_r.trim(); // TODO .expect

        println!("How old are you?");
        let mut age_r = String::new();

        io::stdin()
            .read_line(&mut age_r)
            .expect("Error during reading.");

        let age_c: usize = age_r
            .trim()
            .parse()
            .expect("Please, insert a valid number.");

        println!("How much you paid?");
        let mut price_r = String::new();

        io::stdin()
            .read_line(&mut price_r)
            .expect("Error during reading.");

        let price_c: f32 = price_r
            .trim()
            .parse()
            .expect("Please, insert a valid positive number.");

        let new_member = Member {
            name: name_c.to_string(),
            age: age_c as u32,
            sub_type: SubscriptionType::Monthly, // default for now
            price: price_c,
        };
        total_members.push(new_member);
    }
    let total_revenue: f32 = total_members.iter().map(|m| m.price).sum();
    let total_age: u32 = total_members.iter().map(|m| m.age).sum();
    let average_age = total_age as f32 / total_members.len() as f32;
    println!("\n--- GYM REPORT ---");
    println!("Total Revenue: ${:.2}", total_revenue);
    println!("Average Member Age: {:.1} years", average_age);
    println!("List of members: {:?}", total_members);
}
