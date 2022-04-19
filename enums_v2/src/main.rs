enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,//no data
    Move { x: i32, y: i32}, //anonymous struct
    Write(String),//string
    ChangeColor(i32, i32, i32),//integer
}

impl Message{
    fn some_function(){
        println!("Let's Get Rusty!");
    }
}
struct IpAddr{

    kind: IpAddrKind,
    address: String,
}

struct QuitMessage; // unit struct
struct MoveMessage{
    x: i32,
    y: i32,
}

struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct
fn main() {
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    

}

fn route(ip_kind: IpAddrKind){}
