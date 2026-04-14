pub struct SerialParser {
    sep : String,
    xun : String,
    yun : String,
}

impl SerialParser {
    pub fn new( sep : &str, xun : &str, yun : &str ) -> Self {
        SerialParser {
            sep: sep.to_string(),
            xun: xun.to_string(),
            yun: yun.to_string()
        }
    }
    pub fn drain_size_corrupt( self : &Self, buffer : &[u8] ) -> Option<usize> {
        let sep = self.sep.as_bytes();
        let l = sep.len();
        // Detect at least two complete entries
        let ix = find_sep(buffer, sep, 0)? + l;
        find_sep(buffer, sep,ix)?;
        // Drain the first entry
        Some(ix)
    }

    pub fn read_xy_valid<'a>( self : &Self, buffer : &'a[u8] ) -> Option<(&'a[u8], &'a[u8], usize)> {
        let (a,b,i) = self.read_xy(buffer)?;
        find_sep(a, self.xun.as_bytes(), 0)?;
        find_sep(b, self.yun.as_bytes(), 0)?;
        Some( (a,b,i) )
    }
    fn read_xy<'a>( self : &Self, buffer : &'a[u8] ) -> Option<(&'a[u8], &'a[u8], usize)> {
        let sep = self.sep.as_bytes();
        let l = sep.len();
        let ix = find_sep(buffer, sep, 0)? + l;
        let iy = find_sep(buffer, sep,ix)? + l;
        let i = ix + iy;
        let res = ( &buffer[..ix-l], &buffer[ix..i-l], i );
        Some(res)
    }
}

fn find_sep( buffer : &[u8], sep : &[u8], offset : usize ) -> Option<usize> {
    buffer[offset..]
        .windows(sep.len())
        .position(|win| win == sep)
}
