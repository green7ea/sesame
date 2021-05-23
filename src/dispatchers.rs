use crate::{
    config::{
        AnyDispatch, ConditionalDispatch, ExtensionDispatch, MimeDispatch,
        OneOrManyDispatch, ProtocolDispatch,
    },
    file_info::{is_in_list, start_is_in_list, FileInfo},
};

pub trait Dispatch
{
    fn apply(&self, file_info: &FileInfo) -> Option<String>;
}

impl Dispatch for String {
    fn apply(&self, _: &FileInfo) -> Option<String> {
        Some(self.clone())
    }
}

impl Dispatch for ConditionalDispatch
{
    fn apply(&self, info: &FileInfo) -> Option<String>
    {
        if let Some(contains) = self.contains.as_ref()
        {
            if contains.iter().any(|part| info.input.contains(part))
            {
                println!("contains {:?}", contains);
                Some(self.use_elem.clone())
            }
            else
            {
                println!("doesn't contains {:?}", contains);
                None
            }
        }
        else
        {
            println!("contains []");
            Some(self.use_elem.clone())
        }
    }
}

impl Dispatch for ProtocolDispatch
{
    fn apply(&self, info: &FileInfo) -> Option<String>
    {
        if let Some(protocol) = info.protocol.as_ref()
        {
            self.protocol
                .iter()
                .filter(|(comma_list, _)| is_in_list(protocol, comma_list))
                .inspect(|(proto, _)| println!("protocol {} matches", proto))
                .map(|(_, res)| res.apply(info))
                .next()
                .flatten()
        }
        else
        {
            None
        }
    }
}

impl Dispatch for MimeDispatch
{
    fn apply(&self, info: &FileInfo) -> Option<String>
    {
        if let Some(mime) = info.mime.as_ref()
        {
            self.mime
                .iter()
                .filter(|(comma_list, _)| start_is_in_list(&mime, comma_list))
                .inspect(|(mime, _)| println!("mime {} matches", mime))
                .map(|(_, res)| res.apply(info))
                .next()
                .flatten()
        }
        else
        {
            None
        }
    }
}

impl Dispatch for ExtensionDispatch
{
    fn apply(&self, info: &FileInfo) -> Option<String>
    {
        if let Some(ext) = info.extension.as_ref()
        {
            self.extension
                .iter()
                .filter(|(comma_list, _)| is_in_list(ext, comma_list))
                .inspect(|(ext, _)| println!("ext {} matches", ext))
                .map(|(_, res)| res.apply(info))
                .next()
                .flatten()
        }
        else
        {
            None
        }
    }
}

impl Dispatch for AnyDispatch
{
    fn apply(&self, info: &FileInfo) -> Option<String>
    {
        match self
        {
            AnyDispatch::Direct(x) => x.apply(info),
            AnyDispatch::Conditional(x) => x.apply(info),
            AnyDispatch::Protocol(x) => x.apply(info),
            AnyDispatch::Mime(x) => x.apply(info),
            AnyDispatch::Extension(x) => x.apply(info),
        }
    }
}

impl Dispatch for OneOrManyDispatch
{
    fn apply(&self, info: &FileInfo) -> Option<String>
    {
        match self
        {
            OneOrManyDispatch::One(elem) => elem.apply(info),
            OneOrManyDispatch::Many(elems) =>
            {
                elems.iter().find_map(|x| x.apply(info))
            },
        }
    }
}
