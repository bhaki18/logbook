use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_action() -> u32{
    let options = &[
        "Write a new memory",
        "Modify a memory",
        "Add a file or a folder to your memories",
        "Remove a file or a folder from your memories",
        "Remove a memory",
        "Trace a new work",
        "Modify a work",
        "Add a file or a folder to your work",
        "Remove a file or a folder to your work",
        "Remove a work",
        "Lock a memory",
        "Unlock a memory",
        "Lock a work",
        "Unlock a work",
        "Exit"
    ];

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    return scelta.try_into().unwrap();
}