
pub struct TodoList {
    lists : Vec<(String, State)>,
}

pub enum State {
    completed,
    pending
}

impl TodoList{

    pub fn new() -> Self {
        let lists:Vec<(String, State)> = vec![];
        Self {
            lists: lists
        }
    } 

    pub fn add_task(&mut self, task: String, state: State) {
        self.lists.push((task, state));
    }

    fn fill_list(&mut self){
        let t1 = String::from("Go shopping");
        let t2 = String::from("Go walking");
        let t3 = String::from("Do homework");
        let t4 = String::from("Go to work");
        let t5 = String::from("Rest");

        self.add_task(t1, State::pending);
        self.add_task(t2, State::pending);
        self.add_task(t3, State::pending);
        self.add_task(t4, State::pending);
        self.add_task(t5, State::pending);

    }

    pub fn complete_task(&mut self, name_of_task : &str) {
        for (task, state) in self.lists.iter_mut() {
            if task.to_lowercase() == name_of_task.to_lowercase() {
                match state {
                    State::completed => (),
                    State::pending => *state = State::completed
                };
            }
        }
    }

    pub fn run(&mut self){
        
        self.fill_list();

        self.complete_task("Do homework");
        self.complete_task("Rest");
        self.complete_task("Rest");
        self.complete_task("Go to work");

        println!("------------ Excercise 3 -----------");
        for element in self.lists.iter() {
            let status = match element.1 {
                State::completed => "Completed",
                State::pending => "Pending"
            };
            println!("{} {}", element.0, status)
        }
    }
}