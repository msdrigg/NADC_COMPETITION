use std::collections::HashMap;

struct RevEmployee {
    pub employers: Vec<i32>,
    pub starting_wage: i32,
    pub children_changes: i32,
}

fn main() {
    let mut employee_tree: HashMap<i32, RevEmployee> = HashMap::new();

    let (employees, changes) = scan!(i32, i32);
    // let mut queries: Vec<Query> = Vec::with_capacity(changes.try_into().unwrap());

    let boss_wage = scan!(i32);
    employee_tree.insert(
        1,
        RevEmployee {
            // wage_changes: Vec::new(),
            employers: vec![],
            starting_wage: boss_wage,
            children_changes: 0,
        },
    );

    for employee_next in 2..=employees {
        let (starting_wage, employer) = scan!(i32, i32);
        let employers_squared: &[i32] = &employee_tree.get(&employer).unwrap().employers;
        let mut employers = Vec::with_capacity(employers_squared.len() + 1);
        employers.push(employer);
        employers.extend_from_slice(&employers_squared);
        employee_tree.insert(
            employee_next,
            RevEmployee {
                employers,
                starting_wage,
                children_changes: 0,
            },
        );
    }

    let mut i = 0;
    while i < changes {
        let next_action = scan!(char);
        if next_action == 'u' {
            let employee = scan!(i32);
            let RevEmployee {
                mut starting_wage,
                employers,
                ..
            } = employee_tree.get(&employee).unwrap();
            for employer_id in employers {
                starting_wage += employee_tree.get(&employer_id).unwrap().children_changes;
            }
            println!("{}", starting_wage);
            i += 1
        } else if next_action == 'p' {
            let (_, employee, wage_change) = scan!(char, i32, i32);

            employee_tree.get_mut(&employee).unwrap().children_changes += wage_change;
            i += 1
        }
    }
}
