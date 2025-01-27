use std::collections::HashMap;

fn main() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grisson"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    let third = astronauts.get(2);
    println!("third is {:?}", third);

    let countdown = vec![5, 4, 3, 2, 1];
    println!("countdown is {:?}", countdown);


    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0); // Barron has not flown any missions yet
    missions_flown.insert("Barron", 1); // Updated existing Barron to have flown 1 mission
    missions_flown.entry("Stone").or_insert(2); // Stone added because it doenst exsist. Stone has flown 2 missions

    let kayla = missions_flown.entry("Barron").or_insert(6); // Barron already exists, so return a reference to the value
    *kayla += 1; // Barron has flown 2 missions, so increment by 1
    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron"); // Option<&i32> type
    println!("barron_missions is {:?}", barron_missions); // Some(0)     
}
