use std::net::TcpListener;


/* For this challenge, I want you to make a quick and easy http server. that hosts whatever is in its current folder.

    If you are looking for inspiration, run `python3 -m http.server .`

    This is a comon way share files on a PRIVATE network, I like to use it in my vms, if i am testing some program on 
    various machines and don't want to copy it around with other tools you can instead call 
    `curl serverip:port/my_file -o my_file` to grab your file!

    Now go write it in RUST!!!
    I have a few stub functions to allow for some unit tests, BUT unit tests are not enough for this challenge.

    Hint: use std::net::TcpListener

    Bonus challenge, make it multi-threaded
 */
fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests;