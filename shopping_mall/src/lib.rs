
pub mod mall;

pub use std::cmp::Ordering;
pub use mall::*;
pub use mall::floor::store::Store;
pub use mall::floor::store;
pub use mall::floor::store::employee::Employee;
pub use mall::guard::Guard;

pub fn biggest_store(m : Mall) ->Store{
    let mut biggest = Store::new("", 0, Vec::new());
    for floor in m.floors {
        for store in floor.stores {
            if store.square_meters > biggest.square_meters {
                biggest = store;
            }
        }
    }
    biggest
}

pub fn highest_paid_employee(m : Mall) -> Vec<Employee> {
    let mut employees = Vec::new();
    let mut highest = Employee::new("", 0, 0, 0, 0.0);
    for floor in m.floors {
        for store in floor.stores {
            for employee in store.employees {
                if employee.salary > highest.salary {
                    employees.clear();
                    highest = employee;
                    employees.push(highest.clone());
                } else if employee.salary == highest.salary {
                    employees.push(employee.clone());
                }
            }
        }
    }
    employees
}
    
pub fn nbr_of_employees(m : Mall) -> usize {
    let mut nbr = 0; 
    for floor in m.floors {
        for store in floor.stores {
            nbr += store.employees.len() as usize;
        }
    }
    for _guard in m.guards {
        nbr += 1;
    }
    nbr
}


// If there is not at least 1 guard for every 200 square meters of floor size, a guard should be added to the Mall.guards.
pub fn check_for_securities( m : &mut Mall , mut guards : Vec<Guard>) {
    println!("old nbr {:?}", m.guards.len());
    println!("guards {:?}", guards.len());

    let mut size = 0;
    for floor in &m.floors {
        for store in &floor.stores {
            size += &store.square_meters;
        }
        
    }
    println!("size {:?}", size);
    let mut nbr_needed = size / 200;
    if size % 200 != 0 {
        nbr_needed += 1;
    }
    println!("nbr_needed {:?}", nbr_needed);

    let guard_missing = nbr_needed - m.guards.len() as u64;

    for _needed in 0..guard_missing {
        
        m.guards.push(guards[0].clone());
        // remove the guard from the list of guards
        guards.remove(0);

    }
    //println!("guards: {:?}", guards);
    println!("new nbr {:?}", m.guards.len());
    println!("guards names {:?}", m.guards); 
}


pub fn cut_or_raise( m : &mut Mall) {
    let new_mall = m.clone();

    for floor in new_mall.floors {
        for store in floor.stores {
            for  mut employee in store.employees {
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee.salary += &employee.salary * 0.1;
                } else if employee.working_hours.1 - employee.working_hours.0 < 10 {
                    employee.salary -= &employee.salary * 0.1;
                }
            }
        }
    }
}
