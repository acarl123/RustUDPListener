use std::net::UdpSocket;
use std::io::Error;


fn socket_send() -> Result<(), Error> {

    let mut socket = try!(UdpSocket::bind("0.0.0.0:0"));
    socket.set_broadcast(true);
    println!("{:?}", socket);

    // put message here
    let array:&[u8] = b"QUIT";
    try!(socket.send_to(&array, "255.255.255.255:13389"));

    let mut buf = [0; 255];
    let (amt, src) = try!(socket.recv_from(&mut buf));
    //println!("{:?}", buf);
    println!("{:?}", src);

    Ok(())
}   // the socket is closed here


fn main(){
    socket_send();
}