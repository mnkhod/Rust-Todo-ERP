use std::io;
use std::process::Command;

fn main() {
    print_logo();

    let mut todos:Vec<String> = vec![];

    loop{
        output_menu();
        let answer = get_input_from_user();
        let menu_number:i32 = answer.to_string().trim().parse().unwrap();

        match menu_number {
            1 => { handle_view_todos(&todos) },
            2 => { handle_create_todo(&mut todos) },
            3 => { handle_remove_todo(&mut todos) },
            4 => { break },
            _ => { println!("\nThere is no menu command for that") }
        }

        take_input_to_continue();
    }

    println!("Good Bye 🙇");
}

fn handle_view_todos(_todos:&Vec<String>){
    clear_screen();

    for (index,todos) in _todos.iter().enumerate() {
        println!("{} - {}",index+1,todos);
    }
}

fn handle_create_todo(_todos:&mut Vec<String>){
    clear_screen();
    println!("New todo input");

    let mut todo_content = String::new();
    io::stdin().read_line(&mut todo_content).unwrap();

    _todos.push(todo_content);

}

fn handle_remove_todo(_todos:&mut Vec<String>){
    clear_screen();

    for (index,todo) in _todos.iter().enumerate(){
        println!("{} - {}",index+1,todo);
    }

    println!("Please input remove number");
    let mut remove_index = String::new();

    io::stdin().read_line(&mut remove_index).unwrap();

    let remove_index:usize = remove_index.trim().parse().unwrap();

    _todos.remove(remove_index-1);
}








fn output_menu(){
    println!("What would you like to do?");
    println!("Input menu number");
    println!("--------------------------");
    println!("1 - View Todos 👀");
    println!("2 - Create Todo 🗒");
    println!("3 - Delete Todo ❌");
    println!("4 - Exit ✨");
    println!("--------------------------");
}

fn get_input_from_user() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_data) => {
            return input;
        },
        Err(_e) => { panic!("Something happened with taking input") }
    }
}

fn take_input_to_continue(){
    println!("\nPRESS ANY KEY TO CONTINUE...");

    let mut _data = String::new();
    io::stdin().read_line(&mut _data).unwrap();

    clear_screen();
}


fn clear_screen() {
    match Command::new("clear").status(){
        Err(_e) => { panic!("Something happened with clearing screen with clear unix command") },
        _ => {},
    }
}

// Created with jp2a command line tool
fn print_logo(){
    println!("
-------------------------------
      Todo ERP System    
-------------------------------
        ..',,,;;;,,''.          
      ';;'''''''''''',;;.       
    .:,'''',,,,,'''''''';;      
    c',cxOKXXXXX0xc,''''',:     
   ,;c0NWWWWWWWNWWNk;''''':'    
   c;0WW0KWWWWXkNWWWk,'''',:    
   c'cOkco0XXKk;lOOdl:'''''c    
   :,;c;  'c;l    c,:c'''',:    
   ':,:l'',,','''',.;;'''':'    
    c''c....'.......c'''''c     
    ';',c...,,.....:;'''':.     
     :;',:,......'c,'''':.      
      ;;'';l;,,:l:'''';:.       
       .c,'':;;;'',;::;;,.      
      .;;:cc'',c:::,''''',:.    
    .:,'''l;::cl''''''''''',:.  
   :;''''::''',l'''''''''''''c. 
 .c,'''',c'''''l,'''''''''''',: 
 ':''''',;''''':;''''''''',;;'  
  .',;;;,,''''''',,,;;;,''.     
        ............            
-------------------------------
        @0xMnkhod
-------------------------------
");
}
