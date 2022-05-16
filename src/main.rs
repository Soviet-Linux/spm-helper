extern crate serde_json;
extern crate serde;
extern crate serde_derive;
use std::fs::File;
use std::io::Write;
use serde_json::{json, Value};
use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() {

    let name = user_input("Enter the name of the package: ".to_owned());
    
    let pkg_type = ask_for_type();

    let version = user_input("Enter the version of the package: ".to_owned());

    let dependencies = user_input("Enter the dependencies of the package(example \"git\", \"make\"): ".to_owned());

    let prepare_info = user_input("Enter the the command to prepare the package (SPACE + ENTER if none) : ".to_owned());

    let make_info = user_input("Enter the the command to make the package (SPACE + ENTER if none) : ".to_owned());

    let test_info = user_input("Enter the the command to test the package (SPACE + ENTER if none) : ".to_owned());

    let install_info = user_input("Enter the the command to install the package (use $BUILD_ROOT for the install dir ) : ".to_owned());

    let special_info = user_input("Commands to be executed after the install : ".to_owned());


    

    let mut pkg_true_type = String::new();

    if pkg_type == "Source" {
        pkg_true_type = "src".to_owned();
    } else if pkg_type == "binary" {
        pkg_true_type = "bin".to_owned()
        
    }else if pkg_type == "local" {
        pkg_true_type = "local".to_owned()
        
    }
    let deps = dependencies.as_str().trim_end().replace('\"', "");

    let basic_pattern = json!({
        "name" : name.as_str(), 
        "type" : pkg_true_type.as_str(),
        "version" : version.as_str(),
        "dependencies" : deps,
        "info" : 
        {
            "prepare" : prepare_info.as_str(),
            "make" : make_info.as_str(),
            "test" : test_info.as_str(),
            "install" : install_info.as_str(),
            "special" : special_info.as_str()
            
        },
        "locations" : []
    });
    
    

    save_as_file(basic_pattern, name);
    




}

fn user_input(text: String) -> String {

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(text)
        .interact_text()
        .unwrap();



    return name;
}


fn save_as_file(text: Value, name: String) {

    let selections = &[
        "Yes",
        "No",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to save package as file")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if selections[selection] == "Yes" {

    
        let mut file = File::create(name + ".spm").unwrap();

        // Write a &str in the file (ignoring the result).
        //writeln!(&mut file, "{}", body.unwrap()).unwrap();


        let text_pretty = serde_json::to_string_pretty(&text);

        // Write a byte string.
        file.write(text_pretty.unwrap().to_owned().as_bytes()).unwrap();
    } else if selections[selection] == "No" {
        println!("Ok quiting...")
    }

    
}

fn ask_for_type() -> String  {
    let selections = &[
        "Source",
        "Binary",
        "local",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the type of the package?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();


    let value = selections[selection].to_owned();
    return value;
}
