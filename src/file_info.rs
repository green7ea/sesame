pub struct FileInfo
{
    pub input: String,
    pub extension: Option<String>,
    pub protocol: Option<String>,
    pub mime: Option<String>,
}

impl FileInfo
{
    pub fn new(input: &str) -> FileInfo
    {
        FileInfo {
            input: String::from(input),
            extension: get_extension(&input),
            protocol: get_protocol(&input),
            mime: get_mime(&input),
        }
    }
}

pub fn get_protocol(path: &str) -> Option<String>
{
    if path.contains("://")
    {
        path.split("://").next().map(|x| String::from(x))
    }
    else
    {
        None
    }
}

pub fn get_extension(path: &str) -> Option<String>
{
    path.split('.').last().map(|x| String::from(x))
}

pub fn get_mime(path: &str) -> Option<String>
{
    mime_guess::from_path(path)
        .first()
        .map(|x| String::from(x.essence_str()))
}

pub fn is_in_list(pattern: &str, comma_list: &str) -> bool
{
    comma_list
        .split(',')
        .map(|x| x.trim())
        .any(|x| pattern == x)
}

pub fn start_is_in_list(pattern: &str, comma_list: &str) -> bool
{
    comma_list
        .split(',')
        .map(|x| x.trim())
        .any(|x| pattern.starts_with(x))
}
