use crate::model::SocketType;

#[test]
fn socket_type_should_be_created_using_static_method_call() {
    assert_eq!(SocketType::None, SocketType::none());
}

#[test]
fn socket_type_from_str() {
    assert_eq!(SocketType::None, SocketType::from("none"));
    assert_eq!(SocketType::None, SocketType::from("None"));
    assert_eq!(SocketType::None, SocketType::from("NONE"));

    assert_eq!(SocketType::Input, SocketType::from("input"));
    assert_eq!(SocketType::Input, SocketType::from("Input"));
    assert_eq!(SocketType::Input, SocketType::from("INPUT"));

    assert_eq!(SocketType::Output, SocketType::from("output"));
    assert_eq!(SocketType::Output, SocketType::from("Output"));
    assert_eq!(SocketType::Output, SocketType::from("OUTPUT"));
}

#[test]
fn socket_type_display() {
    assert_eq!("None", format!("{}", SocketType::None));
    assert_eq!("Input", format!("{}", SocketType::Input));
    assert_eq!("Output", format!("{}", SocketType::Output));
}
