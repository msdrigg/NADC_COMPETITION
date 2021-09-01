#[derive(Clone)]
struct Employee {
    pub employees: Vec<usize>,
    pub current_wage: i32,
}

pub fn method1() {
    let (employees, changes) = scan!(usize, usize);
    // let mut queries: Vec<Query> = Vec::with_capacity(changes.try_into().unwrap());
    let mut employee_tree: Vec<Employee> = Vec::with_capacity(employees);

    let boss_wage = scan!(usize);
    employee_tree.insert(
        1,
        Employee {
            // wage_changes: Vec::new(),
            employees: Vec::new(),
            current_wage: boss_wage as i32,
        },
    );

    for employee_next in 2..=employees {
        let (starting_wage, employer) = scan!(i32, i32);
        employee_tree
            .get_mut(employer as usize)
            .unwrap()
            .employees
            .push(employee_next);

        employee_tree.insert(
            employee_next,
            Employee {
                employees: Vec::new(),
                current_wage: starting_wage,
            },
        );
    }

    let mut i = 0;
    while i < changes {
        let next_action = scan!(char);
        if next_action == 'u' {
            let employee = scan!(i32);
            println!(
                "{}",
                employee_tree.get(employee as usize).unwrap().current_wage
            );
            // queries.push(Query { employee, time: i });
            i += 1
        } else if next_action == 'p' {
            let (_, employee, wage_change) = scan!(char, usize, i32);

            update_all_employees(&mut employee_tree, employee, wage_change);
            i += 1
        }
    }
    // for q in queries {}
}

fn update_all_employees(employee_tree: &mut Vec<Employee>, current_id: usize, wage_change: i32) {
    let current_employee = current_id;
    for child_id in employee_tree
        .get(current_employee)
        .unwrap()
        .clone()
        .employees
        .iter()
    {
        let child = employee_tree.get_mut(*child_id).unwrap();
        child.current_wage += wage_change;

        update_all_employees(employee_tree, *child_id, wage_change);
    }
}
