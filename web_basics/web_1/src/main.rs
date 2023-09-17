use std::net::TcpListener;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let stream = listener.accept();
    println!("The stream is {:?} the socket is {:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0);
}
