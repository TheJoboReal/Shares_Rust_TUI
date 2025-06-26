#[derive(Debug, Default)]
pub struct Person {
    pub _name: String,
    pub _expences: f32,
    pub _debt: f32,
    pub _owed: f32,
}

pub fn _debt_calc(persons: &mut Vec<Person>) {
    let mut total_exp: f32 = 0.0;

    for p in &mut *persons {
        total_exp += p._expences;
    }

    let _debt_pr_person = total_exp / persons.len() as f32;

    for p in persons.iter_mut() {
        p._debt = _debt_pr_person - p._expences;
    }
}

pub fn _owed_calc(persons: &mut Vec<Person>) {
    let mut total_owed: f32 = 0.0;

    for p in &mut *persons {
        total_owed += p._expences;
    }

    let _owed_pr_person = total_owed / persons.len() as f32;
    for p in persons.iter_mut() {
        p._owed = p._expences - _owed_pr_person;
    }
}

// -----------------------------------------------------------------------------------------------------------------------------//

#[test]
fn test_owed_calc() {
    let mut persons = vec![
        Person {
            _name: String::from("Kasper"),
            _expences: 100.0,
            _debt: 0.0,
            _owed: 0.0,
        },
        Person {
            _name: String::from("Runa"),
            _expences: 300.0,
            _debt: 0.0,
            _owed: 0.0,
        },
        Person {
            _name: String::from("Camilla"),
            _expences: 200.0,
            _debt: 0.0,
            _owed: 0.0,
        },
    ];

    _owed_calc(&mut persons);

    assert_eq!(persons[0]._owed, -100.0);
    assert_eq!(persons[1]._owed, 100.0);
    assert_eq!(persons[2]._owed, 0.0);
}

#[test]
fn test_debt_calc() {
    let mut persons = vec![
        Person {
            _name: String::from("Kasper"),
            _expences: 100.0,
            _debt: 0.0,
            _owed: 0.0,
        },
        Person {
            _name: String::from("Runa"),
            _expences: 200.0,
            _debt: 0.0,
            _owed: 0.0,
        },
    ];

    _debt_calc(&mut persons);
    _owed_calc(&mut persons);

    assert_eq!(persons[0]._debt, -persons[0]._owed);
    assert_eq!(persons[1]._debt, persons[0]._owed);
}
