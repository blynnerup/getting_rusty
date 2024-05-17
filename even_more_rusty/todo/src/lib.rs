mod things_todo;
use crate::things_todo::{add_activity, items_completed, items_completed::test::test}; // method in things things_todo, submodule and sub-submodule from the items_completed file

mod list{
    pub struct Tasks {
        pub item: String,
    }
}

fn lets_add_task(){
    let task = list::Tasks {item: String::from("Tasks")};
    add_activity();
    items_completed::remove_task();
    test();
}
