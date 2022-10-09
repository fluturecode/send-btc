use std::io;

fn console() {
    println!("\nLet's have fun with Bitcoin!\n")

    println!("Do you want to send (s) or receive (r) Bitcoin?\n");
                // struct::method
    let mut command = String::new();

    io::stdin().read_line(&mut command)

    if command.trim().eq('s') {
        send_bitcoin()
    } else if command.trim().eq('r) {
        recieve_bitcoin()
    }
}


fn main() {
    console()
}
