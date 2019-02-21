use failure::{err_msg, Fallible};

macro_rules! parse_tag {
    (
        $context:ident;
        $attrs:ident;
        <$open:ident $($val:ident=$valname:literal($valkind:ty))* $(?$oval:ident=$ovalname:literal($ovalkind:ty))*>
        $(
            $content:ident
        )?
        $(
            <$childtag:ident> => $childparse:expr,
        )*
        </$close:ident>
    ) => {
        $( let mut $content = String::new(); )?
        $( let mut $childtag = Vec::new(); )*

        parse_tag!(
            $context;
            $attrs;
            <$open $($val=$valname($valkind))* $(?$oval=$ovalname($ovalkind))*>
        );

        loop {
            use xml::reader::XmlEvent;
            match $context.reader.next()? {
                XmlEvent::StartDocument { .. } => continue,

                XmlEvent::Whitespace(_) => (),
                $(
                    XmlEvent::Characters(c) => $content.push_str(&c),
                    XmlEvent::CData(c) => $content.push_str(&c),
                )?
                XmlEvent::StartElement { ref name, ref attributes, .. } => {
                    match name.local_name.as_str() {
                        $(
                            stringify!($childtag) => $childtag.push(($childparse)($context, attributes)?),
                        )*
                        _ => {
                            use $crate::tiled::raw::helpers::XmlParseHelp;
                            $context.warning(format!("unknown tag <{}> inside <{}>", name, stringify!($open)));
                            $context.reader.consume_unknown_child(name)?;
                        }
                    }
                    let _ = attributes;
                }
                XmlEvent::EndElement { ref name } => {
                    match name.local_name.as_str() {
                        stringify!($close) => break,
                        _ => return Err(failure::err_msg(
                            format!("unexpected close tag </{}> (expected </{}>)", name, stringify!($close))
                        ))
                    }
                }
                tok => return Err(failure::err_msg(
                    format!("Unexpected xml token {:?}", tok)
                ))
            }
        }
    };
    (
        $context:ident;
        $attrs:ident;
        <$open:ident $($val:ident=$valname:literal($valkind:ty))* $(?$oval:ident=$ovalname:literal($ovalkind:ty))*/>
    ) => {
        parse_tag!(
            $context;
            $attrs;
            <$open $($val=$valname($valkind))* $(?$oval=$ovalname($ovalkind))*>
            </$open>
        );
    };

    (
        $context:ident;
        $attrs:ident;
        <$open:ident $($val:ident=$valname:literal($valkind:ty))* $(?$oval:ident=$ovalname:literal($ovalkind:ty))*>
    ) => {
        $( let mut $val: Option<$valkind> = None; )*
        $( let mut $oval: Option<$ovalkind> = None; )*

        for attr in $attrs.iter() {
            match attr.name.local_name.as_str() {
                $(
                    stringify!($val) => parse_tag!(@getattr attr $valname $val $valkind),
                )*
                $(
                    stringify!($oval) => parse_tag!(@getattr attr $ovalname $oval $ovalkind),
                )*
                _ => (),
            }
        }

        $( let $val: $valkind = $val.ok_or_else(|| {
            failure::err_msg(concat!("missing attribute ", stringify!($open), ".", stringify!($val)))
        })?; )*
    };
    (@expectclose $context:ident $close:ident) => {{
        use $crate::tiled::raw::helpers::XmlParseHelp;
        $context.reader.expect_close_tag(stringify!($close))?;
    }};
    (@getattr $attr:ident $valname:literal $val:ident $valkind:ty) => {{
        use failure::ResultExt;
        let parse = $attr.value.parse::<$valkind>();
        let res = parse.context(concat!(stringify!($open), ".", $valname, " parse failed"));
        $val = Some(res?);
    }};
}

pub trait XmlParseHelp {
    fn expect_close_tag(&mut self, tag: &str) -> Fallible<()>;
    fn consume_unknown_child(&mut self, tag: &xml::name::OwnedName) -> Fallible<()>;
}

impl<R> XmlParseHelp for xml::EventReader<R>
where
    R: std::io::Read,
{
    fn expect_close_tag(&mut self, tag: &str) -> Fallible<()> {
        use xml::reader::XmlEvent;
        loop {
            return match self.next()? {
                XmlEvent::EndElement { ref name } if name.local_name == tag => Ok(()),
                XmlEvent::Whitespace(_) => continue,
                _ => Err(err_msg(format!("Expected </{}>", tag))),
            };
        }
    }

    fn consume_unknown_child(&mut self, tag: &xml::name::OwnedName) -> Fallible<()> {
        let mut tag_stack = vec![tag.clone()];
        loop {
            use xml::reader::XmlEvent;
            match self.next()? {
                XmlEvent::StartElement { name, .. } => tag_stack.push(name),
                XmlEvent::EndElement { name } => {
                    if tag_stack.pop() != Some(name) {
                        return Err(err_msg("Mismatched tags"));
                    }
                    if tag_stack.is_empty() {
                        return Ok(());
                    }
                }
                XmlEvent::EndDocument => return Err(err_msg("Unexpected end of document")),

                _ => (),
            }
        }
    }
}
