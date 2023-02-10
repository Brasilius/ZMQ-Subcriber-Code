extern crate zmq;
fn main() {
    // Create a socket context with 1 IO thread.
    let context = zmq::Context::new();
    // Create the ZeroMQ Socket.
    let socket = context.socket(zmq::SUB).unwrap();
    
    // Connect the socket to a server url.
    let url = "tcp://127.0.0.1:5556";
    socket.connect(url).unwrap();
    
    // Subscribe to the topic we want to receive messages from.
    let topic = "topic1";
    socket.set_subscribe(topic.as_bytes()).unwrap();
    
    // Receive messages.
    loop {
       let mut msg = zmq::Message::new();
       socket.recv(&mut msg, 0).unwrap();
       // Process the message.
       println!("Received message: {}", msg.as_str().unwrap());
    }
    
}
