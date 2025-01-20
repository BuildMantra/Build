// Contain Agent Profiles For Melania Trump, Kai Trump, Barron Trump, And Stormy Daniels

// Define the Agent trait that all agents must implement
trait Agent {
    fn get_name(&self) -> String;
    fn act(&mut self);
}

// Define the HumanAgent struct with a name and energy level
struct HumanAgent {
    name: String,
    energy: u32,
}

impl HumanAgent {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            energy: 100,
        }
    }

    // Method for resting, which increases the energy of the HumanAgent
    fn rest(&mut self) {
        self.energy += 10;
        println!("{} is resting, energy is now {}", self.name, self.energy);
    }

    // Method for working, which decreases the energy of the HumanAgent
    fn work(&mut self) {
        self.energy -= 20;
        println!("{} is working, energy is now {}", self.name, self.energy);
    }
}

// Implement the Agent trait for HumanAgent
impl Agent for HumanAgent {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn act(&mut self) {
        if self.energy < 30 {
            self.rest();
        } else {
            self.work();
        }
    }
}

// Define the RobotAgent struct with a name and battery level
struct RobotAgent {
    name: String,
    battery: u32,
}

impl RobotAgent {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            battery: 100,
        }
    }

    // Method for recharging the robot, increasing its battery
    fn recharge(&mut self) {
        self.battery += 20;
        println!("{} is recharging, battery is now {}", self.name, self.battery);
    }

    // Method for performing tasks, decreasing the robot's battery
    fn perform_task(&mut self) {
        self.battery -= 25;
        println!("{} is performing a task, battery is now {}", self.name, self.battery);
    }
}

// Implement the Agent trait for RobotAgent
impl Agent for RobotAgent {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn act(&mut self) {
        if self.battery < 30 {
            self.recharge();
        } else {
            self.perform_task();
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
    let human1 = HumanAgent::new("Alice");
    let robot1 = RobotAgent::new("Robo1");

    // Add agents to the environment
    environment.add_agent(Box::new(human1));
    environment.add_agent(Box::new(robot1));

    // Run the simulation
    environment.run_simulation();
}
