use crate::serial_parser::SerialParser;

#[derive(Debug)]
pub enum Data {
    XY(f32,f32),
    XYU(f32,f32,String,String),
    X(f32),
    XU(f32,String),
    Unknown(String)
}

fn get_as_string( buffer : &[u8] ) -> Option<String> {
    String::from_utf8(buffer.to_vec()).ok()
}

impl Data {
    pub fn parse_buffer( parser : &SerialParser, buffer : &mut Vec<u8> ) -> Option<Data> {
        if let Some( (a,b,i) ) = parser.read_xy_valid(buffer) {
            let res = parse_entries(a, b);
            buffer.drain(..i);
            res
        } else {
            let i = parser.drain_size_corrupt(buffer)?;
            let res = get_as_string(&buffer[..i]);
            buffer.drain(..i);
            Some(Data::Unknown(res?))
        }
    }
}

fn parse_entries<'a>( a : &'a[u8], b : &'a[u8] ) -> Option<Data> {
    let sa = String::from_utf8(a.to_vec()).ok()?;
    let sb = String::from_utf8(b.to_vec()).ok()?;
    let sa : Vec<&str> = sa.split_whitespace().collect();
    let sb : Vec<&str> = sb.split_whitespace().collect();
    if sa.len() == 2 && sb.len() == 2 {
        let x : f32 = sa[0].parse().ok()?;
        let y : f32 = sb[0].parse().ok()?;
        return Some( Data::XYU(
            x, y,
            sa[1].to_string(), sb[1].to_string()
        ) )
    }
    None
}
