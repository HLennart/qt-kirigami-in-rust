#[cxx::bridge(namespace = qt)]
mod ffi {

    // QtApplication
    unsafe extern "C++" {
        include!("qt/include/QtApplication.h");

        type QtApplication;

        fn new_qapplication() -> UniquePtr<QtApplication>;

        fn run(&self);
    }

    // QtQmlApplicationEngine
    unsafe extern "C++" {
        include!("qt/include/QtQmlApplicationEngine.h");

        type QtQmlApplicationEngine;

        fn new_qt_qml_application_engine(path: &str) -> UniquePtr<QtQmlApplicationEngine>;
    }
}

use cxx::UniquePtr;
// use crate::ffi;

pub struct QtApp {
    instance: UniquePtr<ffi::QtApplication>,
    app_engine: UniquePtr<ffi::QtQmlApplicationEngine>
}

impl QtApp {
    pub fn new(qml_path: &str) -> Self {
        // make sure to create instance first always
        let instance = ffi::new_qapplication();


        let app_engine = ffi::new_qt_qml_application_engine(&format!("{}", qml_path));
        Self {
            instance,
            app_engine
        }
    }

    pub fn run(&self) {
        self.instance.run();
    }

    pub fn test(&self) -> String {
        "hello from qt crate!".into()
    }
} 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
