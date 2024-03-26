use iced::{Checkbox, text_input, Text, TextInput, Application, Color, Command, Element, Settings, executor, button, Button, Column};

#[derive(Debug, Clone)]
enum Tab {
    Home,
    Work,
    Personal,
}

struct Task {
    name: String,
    completed: bool,
}

impl Tab {
    fn title(&self) -> &str {
        match self {
            Tab::Home => "Home",
            Tab::Work => "Work",
            Tab::Personal => "Personal",
        }
    }
}

struct TaskListApp {
    current_tab: Tab,
    task_name: String,
    add_button_state: button::State,
    remove_button_state: button::State,
    input_state: text_input::State,
    tasks_home: Vec<Task>,
    tasks_work: Vec<Task>,
    tasks_personal: Vec<Task>,
    home_tab_state: button::State,
    work_tab_state: button::State,
    personal_tab_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTask(usize),
    AddTask,
    InputChanged(String),
    RemoveCompletedTasks,
    SwitchTab(Tab),
}

impl Application for TaskListApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (TaskListApp, Command<Self::Message>) {
        (
            TaskListApp {
                current_tab: Tab::Home,
                task_name: String::new(),
                input_state: text_input::State::new(),
                add_button_state: button::State::new(),
                remove_button_state: button::State::new(),
                tasks_home: Vec::new(),
                tasks_work: Vec::new(),
                tasks_personal: Vec::new(),
                home_tab_state: button::State::new(),
                work_tab_state: button::State::new(),
                personal_tab_state: button::State::new(),

            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Task List App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                self.task_name = value;
            },
            Message::SwitchTab(tab) => {
                self.current_tab = tab;
            },
            Message::AddTask => {
                if !self.task_name.trim().is_empty() {    
                    let task = Task {
                        name: self.task_name.clone(),
                        completed: false,
                    };
                    match self.current_tab {
                        Tab::Home => self.tasks_home.push(task),
                        Tab::Work => self.tasks_work.push(task),
                        Tab::Personal => self.tasks_personal.push(task),
                    }
                    self.task_name.clear();
                }
            },
            Message::ToggleTask(task_index) => {
                match self.current_tab {
                    Tab::Home => self.tasks_home[task_index].completed = !self.tasks_home[task_index].completed,
                    Tab::Work => self.tasks_work[task_index].completed = !self.tasks_work[task_index].completed,
                    Tab::Personal => self.tasks_personal[task_index].completed = !self.tasks_personal[task_index].completed,
                }
            },
            Message::RemoveCompletedTasks => {
                match self.current_tab {
                    Tab::Home => self.tasks_home.retain(|task| !task.completed),
                    Tab::Work => self.tasks_work.retain(|task| !task.completed),
                    Tab::Personal => self.tasks_personal.retain(|task| !task.completed),
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let home_tab = Button::new(
            &mut self.home_tab_state,
            Text::new(Tab::Home.title())
        )
        .on_press(Message::SwitchTab(Tab::Home))
        .padding(10);
            
        let work_tab = Button::new(
            &mut self.work_tab_state,
            Text::new(Tab::Work.title())
        ) 
        .on_press(Message::SwitchTab(Tab::Work))
        .padding(10);

        let personal_tab = Button::new(
            &mut self.personal_tab_state,
            Text::new(Tab::Personal.title())
        )
        .on_press(Message::SwitchTab(Tab::Personal))
        .padding(10);

        let tab_bar = iced::Row::new()
            .push(home_tab)
            .push(work_tab)
            .push(personal_tab)
            .spacing(20);

        let content = match self.current_tab {
            Tab::Home => &self.tasks_home,
            Tab::Work => &self.tasks_work,
            Tab::Personal => &self.tasks_personal,
        };

        let input = TextInput::new(
            &mut self.input_state,
            "Enter task...",
            &self.task_name,
            Message::InputChanged,
        )
        .padding(10);

        let add_button = Button::new(&mut self.add_button_state, Text::new("Add Task"))
            .on_press(Message::AddTask)
            .padding(10);

        let remove_button = Button::new(&mut self.remove_button_state, Text::new("Remove Completed Tasks"))
            .on_press(Message::RemoveCompletedTasks)
            .padding(10);

        let tasks: Element<_> = content.iter().enumerate().fold(
            Column::new().spacing(10), |column, (index, task)| {
                let checkbox = Checkbox::new(
                    task.completed,
                    &task.name,
                    move |_| Message::ToggleTask(index)
                )
                .spacing(10);

                column.push(checkbox)
            },
        ).into();

        Column::new()
            .spacing(20)
            .push(tab_bar)
            .push(input)
            .push(iced::Row::new().spacing(10).push(add_button).push(remove_button))
            .push(tasks)
            .into()
    }

    fn background_color(&self) -> iced::Color {
        match self.current_tab {
            Tab::Home => Color::from_rgb(1.0, 0.0, 0.0),
            Tab::Work => Color::from_rgb(0.0, 1.0, 0.0),
            Tab::Personal => Color::from_rgb(0.0, 0.0, 1.0),
        }
    }
}

fn main() {
    TaskListApp::run(Settings::default())
        .expect("An error occurred while starting the application");
}
