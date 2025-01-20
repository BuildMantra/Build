use std::env;
// add
use aios::{
    agent::AgentBuilder,
    completion::Prompt,
    loaders::FileLoader,
    providers::openai::{self, GPT_4O},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client =
        openai::Client::new(&env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));

    let model = openai_client.completion_model(GPT_4O);

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
