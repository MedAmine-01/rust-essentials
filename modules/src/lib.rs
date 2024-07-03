

mod list {

    //even if the struct is pub we have to make item pub to acesss it in fn lets_add_task
    pub struct Tasks{
        pub item : String,
    }


    // we can have things to do module in seperate file
/*
    pub mod things_todo{
        pub fn add_activity(){
        }
        fn update_activity(){

        }
        fn marked_activiry(){

        }
    }
*/
    //submodule in things_todo
    /*
    mod items_completed{
        fn remove_task(){

        }
        fn move_back_todo(){

        }
    }
    */
}

mod things_todo;
use crate::things_todo::add_activity;

use things_todo::items_completed;


fn lets_add_task(){
    let task= list::Tasks{ item : String::from("clean house")};
    //list::things_todo::add_activity(); //relative path (without seperate module)
    //crate::list::things_todo::add_activity();//absolute path we started at the root crate (without seperate module)
    //things_todo::add_activity(); //relative path
    //crate::things_todo::add_activity();//absolute path we started at the root crate

    //or we can use "use keyword" (line 35)
    add_activity();
    items_completed::remove_task();

    crate::things_todo::items_completed::test::test();
}
