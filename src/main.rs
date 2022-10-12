use std::io;
use rand::Rng;

fn send_bitcoin() {
    println!("\nWe're going to send some Bitcoin!\n");

    let clients = vec!["Homer", "Marge", "Bart", "Lisa"];

    println!("Who do you want to sendd Bitcoin to?\n");
    for client in &clients {
        print!("{} ", client);
    }
    println!("\n");
}

fn recieve_bitcoin() {
    println!("\nWe're going to receive some Bitcoin!\n");

    let amount = rand::thread_rng().gen_range(1, 10);

    println!("You just received {} Bitcoin!\n", amount);
}

fn exit_console() {
    print!("Invalid option, must be (s) or (r)");
}

fn console() {
    println!("\nLet's have fun with Bitcoin!\n");

    println!("Do you want to send (s) or receive (r) Bitcoin?\n");
                // struct::method
    let mut command = String::new();

    io::stdin().read_line(&mut command);

    if command.trim().eq("s") {
        send_bitcoin()
    } else if command.trim().eq("r") {
        recieve_bitcoin()
    } else {
        exit_console()
    }
}


fn main() {
    console()
}
