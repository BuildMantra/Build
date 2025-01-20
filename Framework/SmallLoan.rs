// Define the Agent trait that all agents must implement
trait Agent {
    fn get_name(&self) -> String;
    fn act(&mut self);
}

// Define the SmallLoanAgent struct with a name and loan balance
struct SmallLoanAgent {
    name: String,
    loan_balance: f32,
}

impl SmallLoanAgent {
    fn new(name: &str, initial_loan: f32) -> Self {
        Self {
            name: name.to_string(),
            loan_balance: initial_loan,
        }
    }

    // Method to make a payment toward the loan
    fn make_payment(&mut self, payment: f32) {
        if payment > self.loan_balance {
            self.loan_balance = 0.0;
            println!("{} has paid off the loan completely!", self.name);
        } else {
            self.loan_balance -= payment;
            println!("{} made a payment of {}. Loan balance is now: {}", self.name, payment, self.loan_balance);
        }
    }

    // Method to accumulate more debt (e.g., taking out another small loan)
    fn take_loan(&mut self, amount: f32) {
        self.loan_balance += amount;
        println!("{} took out a loan of {}. Loan balance is now: {}", self.name, amount, self.loan_balance);
    }

    // Method to check the current loan balance
    fn check_balance(&self) {
        println!("{}'s current loan balance is: {}", self.name, self.loan_balance);
    }
}

// Implement the Agent trait for SmallLoanAgent
impl Agent for SmallLoanAgent {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn act(&mut self) {
        // Simulate the agent's action
        if self.loan_balance > 50.0 {
            // If the loan balance is high, make a payment
            let payment = 20.0; // Agent pays off 20
            self.make_payment(payment);
        } else if self.loan_balance > 0.0 {
            // If there's still some loan left, take out a smaller loan
            let new_loan = 10.0;
            self.take_loan(new_loan);
        } else {
            // If the loan is paid off, just check balance
            self.check_balance();
        }
    }
}

// Define the Environment struct that holds the list of agents
struct Environment {
    agents: Vec<Box<dyn Agent>>,
}

impl Environment {
    // Create a new environment
    fn new() -> Self {
        Self { agents: Vec::new() }
    }

    // Add an agent to the environment
    fn add_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.push(agent);
    }

    // Run the simulation by having each agent act
    fn run_simulation(&mut self) {
        println!("Starting simulation...\n");
        for agent in &mut self.agents {
            agent.act();
        }
    }
}

// The main function where the environment and agents are created
fn main() {
    // Create a new environment
    let mut environment = Environment::new();

    // Create agents
    let small_loan_agent = SmallLoanAgent::new("LoanMaster", 100.0);

    // Add agents to the environment
    environment.add_agent(Box::new(small_loan_agent));

    // Run the simulation
    environment.run_simulation();
}
