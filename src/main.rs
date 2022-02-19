mod config;
mod dispatchers;
mod file_info;

use config::OneOrManyDispatch;
use dispatchers::Dispatch;
use file_info::FileInfo;
use nix::unistd::execv;
use std::ffi::CString;

const PROJECT: &str = "https://github.com/green7ea/sesame";

fn exec(program: &str, args: &[String]) -> Result<(), String>
{
    let env = CString::new("/usr/bin/env").unwrap();
    let cstr_program = CString::new(program).unwrap();

    let args: Vec<CString> = vec![env.clone(), cstr_program]
        .into_iter()
        .chain(args.iter().map(|x| CString::new(x.as_str()).unwrap()))
        .collect();

    match execv(&env, &args)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Couldn't run program {}", program)),
    }
}

fn main()
{
    let mut args = std::env::args();

    let program = match args.next()
    {
        Some(x) => x,
        None => String::from("sesame"),
    };

    let arg = match args.next()
    {
        Some(x) => x,
        None =>
        {
            println!("Please provide a path to open as an argument.\n");
            println!("  {} http://test.com", program);
            println!("  {} ./file.pdf", program);
            return;
        },
    };

    let info = FileInfo::new(&arg);

    let config_file = match dirs::config_dir()
    {
        Some(x) => x,
        None =>
        {
            println!(
                "No configuration folder found, this is where your \
                 configurations should be stored"
            );
            return;
        },
    };
    let config_file = config_file.join("sesame/config.json");
    let config_file = config_file.as_path();
    let config = match std::fs::File::open(config_file)
    {
        Ok(x) => x,
        Err(_) =>
        {
            println!(
                "Couldn't find a configuration file, it should be placed in \
                 {}. You can find a sample here:\n",
                config_file.display()
            );
            println!("{}/blob/master/config.json", PROJECT);
            return;
        },
    };
    let config: OneOrManyDispatch = match serde_json::from_reader(config)
    {
        Ok(x) => x,
        Err(err) =>
        {
            println!("Error in the configuration file:");
            println!("{}", err);
            return;
        },
    };

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
        let args: Vec<String> = std::env::args().skip(1).collect();
        match exec(&dispatch, &args)
        {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
}
