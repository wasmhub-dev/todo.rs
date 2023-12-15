use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlButtonElement, HtmlElement, HtmlUListElement};
use gloo_events::EventListener;
use gloo::utils::document;
use gloo::storage::{LocalStorage, Storage};
use gloo::dialogs::alert;

#[derive(Clone, Debug)]
pub struct TodoApp {
    input_box: HtmlInputElement,
    button: HtmlButtonElement,
    list: HtmlUListElement,
}

impl TodoApp {
    pub fn new() -> Self {
        let input_box: HtmlInputElement = document().get_element_by_id("input-box").unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let button = document().query_selector("button").unwrap().unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        let list = document().get_element_by_id("list-container").unwrap()
            .dyn_into::<HtmlUListElement>()
            .unwrap();

        let todo_app = Self {
            input_box,
            button,
            list,
        };

        let mut todo_app_clone = todo_app.clone();
        EventListener::new(&todo_app.button, "click", move |_| {
                let _ = todo_app_clone.add_task();
        }).forget();

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

        todo_app
    }

    fn add_task(&mut self) {
        if self.input_box.value().is_empty() {
            let _ = alert("Please enter a task");
        } else {
            let task = &self.input_box.value();
            let task_html = self.create_task_html(task);
            let _ = self.list.insert_adjacent_html("beforeend", &task_html);
            let _ = &self.input_box.set_value("");
            self.save_data();
        }
    }

    fn save_data(&self) {
        let _ = LocalStorage::set("todo_html", self.list.inner_html());
    }

    fn create_task_html(&self, task: &String) -> String {
        format!("
            <li>
                {}
                <span>&times;</span>
            </li>
        ", task)
    }

    pub fn show_task(&self) {
        let data:Result<String, _> = LocalStorage::get("todo_html");

        match data {
            Ok(html) => {
                self.list.set_inner_html(&html);
            },
            _ => {}
            
        }
    }
}