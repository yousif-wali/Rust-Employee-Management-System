struct Employee {
    id: u32,
    name: String,
    position: String,
    salary: f64,
}

impl Employee {
    fn new(id: u32, name: String, position: String, salary: f64) -> Employee {
        Employee { id, name, position, salary }
    }

    fn display(&self) {
        println!("ID: {}, Name: {}, Position: {}, Salary: {}", self.id, self.name, self.position, self.salary);
    }
}

struct Department {
    name: String,
    employees: Vec<Employee>,
}

impl Department {
    fn new(name: String) -> Department {
        Department { name, employees: Vec::new() }
    }

    fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    fn display(&self) {
        println!("Department: {}", self.name);
        for employee in &self.employees {
            employee.display();
        }
    }
}

struct Manager {
    employee: Employee,
    department_managed: String,
}

impl Manager {
    fn new(employee: Employee, department_managed: String) -> Manager {
        Manager { employee, department_managed }
    }

    fn display(&self) {
        println!("Manager of Department: {}", self.department_managed);
        self.employee.display();
    }
}

struct Payroll {
    employee_payroll: Vec<Employee>,
}

impl Payroll {
    fn new() -> Payroll {
        Payroll { employee_payroll: Vec::new() }
    }

    fn add_employee(&mut self, employee: Employee) {
        self.employee_payroll.push(employee);
    }

    fn display_payroll(&self) {
        for employee in &self.employee_payroll {
            println!("Payroll - Employee ID: {}, Salary: {}", employee.id, employee.salary);
        }
    }
}

fn main() {
    let employee1 = Employee::new(1, "John Doe".to_string(), "Developer".to_string(), 50000.0);
    let employee2 = Employee::new(2, "Jane Smith".to_string(), "Designer".to_string(), 45000.0);
    
    let mut dept = Department::new("IT".to_string());
    dept.add_employee(employee1);
    dept.add_employee(employee2);

    let manager = Manager::new(Employee::new(3, "Alice Johnson".to_string(), "Manager".to_string(), 70000.0), "IT".to_string());

    let mut payroll = Payroll::new();
    for employee in dept.employees {
        payroll.add_employee(employee);
    }
    payroll.add_employee(manager.employee);

    dept.display();
    manager.display();
    payroll.display_payroll();
}
