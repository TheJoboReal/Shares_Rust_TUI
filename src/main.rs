mod share_calc;
use share_calc::Person;
use share_calc::debt_calc;
use share_calc::owed_calc;

fn main() {
    let mut persons = vec![
        Person {
            name: String::from("Kasper"),
            expences: 100.0,
            debt: 0.0,
            owed: 0.0,
        },
        Person {
            name: String::from("Runa"),
            expences: 200.0,
            debt: 0.0,
            owed: 0.0,
        },
    ];

    debt_calc(&mut persons);
    owed_calc(&mut persons);

    println!("Debt and owed calculations:");
    for person in &persons {
        println!(
            "{}: Debt = {}, Owed = {}",
            person.name, person.debt, person.owed
        );
    }
}
