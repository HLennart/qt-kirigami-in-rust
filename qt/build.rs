use std::process::Command;

use cxx_build::CFG;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::create_all;

fn main() -> anyhow::Result<()> {
    // let kirigami_src_path = std::path::Path::new("kirigami/src");
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let mut out_dir_path = std::path::Path::new(&out_dir).to_path_buf().canonicalize()?;
    // out_dir_path.push("kirigami_cmake");
    
    // if !out_dir_path.exists() {
    //     create_all(&out_dir_path, true)?;

    //     Command::new("cmake")
    //         .current_dir(&out_dir_path)
    //         .arg(kirigami_src_path.canonicalize()?.as_os_str())
    //         .spawn()?;
    //     Command::new("make")
    //         .current_dir(&out_dir_path)
    //         .spawn()?;
    // }
    

    let mut paths = vec![];
    // let paths = pkg_config::Config::new().atleast_version("5.12.0").probe("Qt5").unwrap().include_paths;
    let mut quick = pkg_config::Config::new().probe("Qt5Quick").unwrap();
    let mut qml = pkg_config::Config::new().probe("Qt5Qml").unwrap(); 
    let mut gui = pkg_config::Config::new().probe("Qt5Gui").unwrap(); 
    let mut controls = pkg_config::Config::new().probe("Qt5QuickControls2").unwrap(); 
    let mut widgets = pkg_config::Config::new().probe("Qt5Widgets").unwrap();
    // let mut kirigami = pkg_config::Config::new().probe("KF5Kirigami2").unwrap();
    // let mut i18n = pkg_config::Config::new().probe("KF5I18n").unwrap();

    paths.append(&mut quick.include_paths);
    paths.append(&mut qml.include_paths);
    paths.append(&mut gui.include_paths);
    paths.append(&mut controls.include_paths);
    paths.append(&mut widgets.include_paths);
    // paths.append(&mut kirigami.include_paths);
    // paths.append(&mut i18n.include_paths);

    let mut linker_paths = vec![];
    linker_paths.append(&mut quick.link_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());
    linker_paths.append(&mut qml.include_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());
    linker_paths.append(&mut gui.include_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());
    linker_paths.append(&mut controls.include_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());
    linker_paths.append(&mut widgets.include_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());
    // linker_paths.append(&mut kirigami.include_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());
    // linker_paths.append(&mut i18n.include_paths.iter().map(|x| x.clone().into_os_string().into_string().unwrap()).collect::<Vec<_>>());

    let mut linker_paths_ref = linker_paths.iter().map(|x| x.as_str()).collect::<Vec<_>>();

    CFG.exported_header_links.append(&mut linker_paths_ref);

    cxx_build::bridge("src/lib.rs")
        .file("src/QtApplication.cpp")
        .file("src/QtQmlApplicationEngine.cpp")
        .includes(paths)
        .flag_if_supported("-std=c++17")
        .compile("qt");

    println!("cargo:rerun-if-changed=include/");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rustc-link-lib=static=qt");  
    
    
    Ok(())
}