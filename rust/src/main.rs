fn main() {
    println!("Hello, world!");
    let q = qt::QtApp::new("/home/rox/rusting/cpping/helloworld/rust/ui/main.qml");
    println!("{:?}", q.test());
    q.run();




}
