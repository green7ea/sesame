mod config;
mod dispatchers;
mod file_info;

use config::OneOrManyDispatch;
use dispatchers::Dispatch;
use file_info::FileInfo;
use nix::unistd::execv;
use std::ffi::CString;

fn exec(program: &str, args: &[String])
{
    let env = CString::new("/usr/bin/env").unwrap();
    let program = CString::new(program).unwrap();

    let args: Vec<CString> = vec![env.clone(), program]
        .into_iter()
        .chain(args.iter().map(|x| CString::new(x.as_str()).unwrap()))
        .collect();

    execv(&env, &args).unwrap();
}

fn main()
{
    let args: Vec<String> = std::env::args().skip(1).collect();

    let info = FileInfo::new(args.get(0).unwrap());

    let config_file = dirs::config_dir()
        .unwrap()
        .join("sesame/config.json");
    let config_file = config_file.as_path();
    let config = std::fs::File::open(config_file).unwrap();
    let config: OneOrManyDispatch = serde_json::from_reader(config).unwrap();

    println!(
        "{} (ext: {}, proto: {}, mime: {})",
        &info.input,
        info.extension.as_ref().map(String::as_str).unwrap_or("-"),
        info.protocol.as_ref().map(String::as_str).unwrap_or("-"),
        info.mime.as_ref().map(String::as_str).unwrap_or("-"),
    );
    let dispatch = config.apply(&info);

    if let Some(dispatch) = dispatch
    {
        exec(&dispatch, &args);
    }
}
