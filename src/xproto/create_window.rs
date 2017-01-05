use ::std::io;

use ::futures;
use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;

use ::protocol::Request;
use ::protocol::VoidReply;
use ::Client;

const OPCODE: u8 = 1;

pub struct CreateWindow {
    /// The window resource id.
    pub wid: u32,

    /// The window parent.
    pub parent: u32,

    /// The class
    pub class: u16,

    /// Window bit depth.
    pub depth: u8,

    /// Window visual
    pub visual: u32,

    /// Window x coordinate on parent.
    pub x: u16,

    /// Window y coordinate on parent.
    pub y: u16,

    /// Window width.
    pub width: u16,

    /// Window height.
    pub height: u16,

    /// The window border width.
    pub border_width: u16, // TODO: Add value-list
}

impl Request for CreateWindow {
    type Reply = VoidReply;

    fn encode(&self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        try!(a.write_u8(OPCODE));
        try!(a.write_u8(self.depth));
        try!(a.write_u16::<NativeEndian>(10)); // TODO: length
        try!(a.write_u32::<NativeEndian>(self.wid));
        try!(a.write_u32::<NativeEndian>(self.parent));
        try!(a.write_u16::<NativeEndian>(self.x));
        try!(a.write_u16::<NativeEndian>(self.y));
        try!(a.write_u16::<NativeEndian>(self.width));
        try!(a.write_u16::<NativeEndian>(self.height));
        try!(a.write_u16::<NativeEndian>(self.border_width));
        try!(a.write_u16::<NativeEndian>(self.class));
        try!(a.write_u32::<NativeEndian>(self.visual));
        // TODO: actually create value-mask and value-list
        try!(a.write_u32::<NativeEndian>(0x2 /* background-pixel */ | 0x800 /* event-mask */));
        try!(a.write_u32::<NativeEndian>(0xccffcc));
        try!(a.write_u32::<NativeEndian>(0x1 /* KeyPress */ | 0x8000 /* Exposure */));

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        Box::new(futures::finished((client, ())))
    }
}