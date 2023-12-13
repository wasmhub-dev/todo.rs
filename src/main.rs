mod state;

use std::rc::Rc;

use state::{State, Task};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlButtonElement, HtmlElement};
use gloo_events::EventListener;
use gloo::console::log;
use gloo::utils::document;
use gloo::storage::{LocalStorage, Storage};
use gloo::dialogs::alert;

#[derive(Clone, Debug)]
struct TodoApp {
    input_box: HtmlInputElement,
    button: HtmlButtonElement,
    list: HtmlElement,
    state: State,
}

impl TodoApp {
    fn new() -> Self {
        let input_box: HtmlInputElement = document().get_element_by_id("input-box").unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let button = document().query_selector("button").unwrap().unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        let list = document().get_element_by_id("list-container").unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        let saved_json: String = LocalStorage::get("todo").unwrap_or_default();
        let state = serde_json::from_str(&saved_json).unwrap_or_else(|_| State::new());

        Self {
            input_box,
            button,
            list,
            state,
        }
    }

    fn add_task(&self) {
        if self.input_box.value().is_empty() {
            let _ = alert("Please enter a task");
        } else {
            let li = document().create_element("li").unwrap();
            li.set_inner_html(&self.input_box.value());
            self.list.append_child(&li).unwrap();
            self.input_box.set_value("");
            let span = document().create_element("span").unwrap();
            span.set_inner_html("&times;");
            li.append_child(&span).unwrap();
        }
        self.save_data();
    }

    fn save_data(&self) {
        let json = serde_json::to_string(&self.state).unwrap();
        let _ = LocalStorage::set("todo_list", json);
    }

    fn create_task_html(&self, task: &Task) -> String {
        format!("
            <li class=\"{}\">
                {}
                <span>&times;</span>
            </li>)
        ", if task.completed { "checked" } else { "" }, task.name)
    }

    fn show_task(&self) {
        let html = self.state.tasks.iter()
            .map(|task| self.create_task_html(task))
            .collect::<Vec<String>>().join("\n");
        self.list.set_inner_html(&html);
    }
}


#[wasm_bindgen(main)]
pub fn main() {
    log!("Welcome to the wasm world!");
    let todo_app = Rc::new(TodoApp::new());

    { 
        let todo_app_clone = todo_app.clone();
        EventListener::new(&todo_app.list, "click", move |event| {
            let target = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            if target.tag_name() == "LI" {
                let _ = target.class_list().toggle_with_force("checked", true);
                let _ = todo_app_clone.save_data();
            } else if target.tag_name() == "SPAN" {
                target.parent_element().unwrap().remove();
                let _ = todo_app_clone.save_data();
            }
        }).forget();
    }

    {
        let todo_app_clone = todo_app.clone();
        EventListener::new(&todo_app.button, "click", move |_| {
            let _ = todo_app_clone.add_task();
        }).forget();
    }

    todo_app.show_task();
}