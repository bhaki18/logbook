use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_action() -> u32{
    let options = &[
        "write a new memory",
        "modify a memory",
        "add a file or a folder to your memories",
        "remove a file or a folder from your memories",
        "remove a memory",
        "lock memory",
        "unlock memory",
        "exit"
    ];

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    return scelta.try_into().unwrap();
}