use image::{Rgba, GenericImageView, RgbaImage};

pub fn into_snes(c: Rgba<u8>) -> u16 {
    (c[0]as u16 >> 3)|((c[1]as u16 >> 3)<<5)|((c[2]as u16 >> 3)<<10)
}
// This extends the lower 3 bits to an 8-bit color.
// If you do not want this behaviour, cut them off with `& 0xF8`.
pub fn into_rgb8(c: u16) -> Rgba<u8> {
    // 0bbbbbgg gggrrrrr
    let extend = |c| ((c << 3) | (c >> 2)) as u8;
    let b = extend((c >> 10) & 0b11111);
    let g = extend((c >> 5 ) & 0b11111);
    let r = extend( c        & 0b11111);
    Rgba([r,g,b,0xFF])
}

pub struct Palette(pub Vec<Rgba<u8>>);

impl Palette {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn from_image<G: GenericImageView<Pixel=Rgba<u8>>>(img: &G) -> Self {
        Self(img.pixels().map(|(_,_,c)| c).collect())
    }
    pub fn to_image(&self, width: u32, height: u32) -> RgbaImage {
        let mut iter = self.0.iter();
        RgbaImage::from_fn(width, height, |_,_| {
            iter.next().copied().unwrap_or(Rgba([0,0,0,0]))
        })
    }
    pub fn from_format<F: Format>(mut f: F, mut buf: &[u8]) -> Self {
        Self(std::iter::from_fn(|| f.from_buf(&mut buf)).collect())
    }
    pub fn to_format<F: Format>(&self, mut f: F, buf: &mut Vec<u8>) {
        for i in self.0.iter() {
            f.to_buf(*i, buf);
        }
    }
}

pub trait Format {
    fn from_buf(&mut self, input: &mut &[u8]) -> Option<Rgba<u8>>;
    fn to_buf(&mut self, color: Rgba<u8>, output: &mut Vec<u8>);
}

pub struct Snes;
impl Format for Snes {
    fn from_buf(&mut self, input: &mut &[u8]) -> Option<Rgba<u8>> {
        if input.len() < 2 {
            return None;
        }
        let (a,b) = input.split_at(2);
        *input = b;
        Some(into_rgb8(u16::from_le_bytes([a[0], a[1]])))
    }
    fn to_buf(&mut self, color: Rgba<u8>, output: &mut Vec<u8>) {
        output.extend_from_slice(&into_snes(color).to_le_bytes());
    }
}
