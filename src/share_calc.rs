pub struct Person {
    pub name: String,
    pub expences: f32,
    pub debt: f32,
    pub owed: f32,
}

pub fn debt_calc(persons: &mut Vec<Person>) {
    let mut total_exp: f32 = 0.0;

    for p in &mut *persons {
        total_exp += p.expences;
    }

    let debt_pr_person = total_exp / persons.len() as f32;

    for p in persons.iter_mut() {
        p.debt = debt_pr_person - p.expences;
    }
}

pub fn owed_calc(persons: &mut Vec<Person>) {
    let mut total_owed: f32 = 0.0;

    for p in &mut *persons {
        total_owed += p.expences;
    }

    let owed_pr_person = total_owed / persons.len() as f32;
    for p in persons.iter_mut() {
        p.owed = p.expences - owed_pr_person;
    }
}

// -----------------------------------------------------------------------------------------------------------------------------//

#[test]
fn test_owed_calc() {
    let mut persons = vec![
        Person {
            name: String::from("Kasper"),
            expences: 100.0,
            debt: 0.0,
            owed: 0.0,
        },
        Person {
            name: String::from("Runa"),
            expences: 300.0,
            debt: 0.0,
            owed: 0.0,
        },
        Person {
            name: String::from("Camilla"),
            expences: 200.0,
            debt: 0.0,
            owed: 0.0,
        },
    ];

    owed_calc(&mut persons);

    assert_eq!(persons[0].owed, -100.0);
    assert_eq!(persons[1].owed, 100.0);
    assert_eq!(persons[2].owed, 0.0);
}

#[test]
fn test_debt_calc() {
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

    assert_eq!(persons[0].debt, 50.0);
    assert_eq!(persons[1].debt, -50.0);
}
