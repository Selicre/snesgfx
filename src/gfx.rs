use image::{imageops, GenericImageView, GrayImage, Luma, Rgba};
use std::collections::HashMap;
// Internal storage: 8bpp linear, 8x8 tileset
pub struct Graphics(pub Vec<u8>);

impl Graphics {
    pub fn tile(&self, idx: usize) -> Option<GrayImage> {
        // each tile is 8x8, so 64 bytes
        GrayImage::from_vec(8, 8, self.0.get(idx*64..(idx+1)*64)?.to_vec())
    }
    pub fn from_headered_image(
        img: &impl GenericImageView<Pixel=Rgba<u8>>,
    ) -> Result<Self,String> {
        let top = img.view(0,0,img.width(),16);
        let bottom = img.view(0,16,img.width(),img.height()-16);
        Self::from_paletted_image(&bottom, &top)
    }
    pub fn from_headered_image2(
        img: &impl GenericImageView<Pixel=Rgba<u8>>,
    ) -> Result<Self,String> {
        let top = img.view(0,0,img.width(),16);
        let bottom = img.view(0,16,img.width(),img.height()-16);
        Self::from_paletted_image2(&bottom, &top)
    }
    pub fn from_paletted_image(
        img: &impl GenericImageView<Pixel=Rgba<u8>>,
        pal: &impl GenericImageView<Pixel=Rgba<u8>>
    ) -> Result<Self,String> {
        let mut pal_map = HashMap::new();
        for (i,(_,_,mut c)) in pal.pixels().enumerate() {
            if i == 16 { break; }   // todo
            if c.0[3] == 0 { c = Rgba(Default::default()); }
            if pal_map.contains_key(&c) { continue; }
            pal_map.insert(c,i);
        }
        println!("{:?}", pal_map);
        let twidth = img.width()/8;
        let theight = img.height()/8;
        let get_tile = |id| (id%twidth*8, id/twidth*8);
        let mut out = vec![];

        for tile in 0..twidth*theight {
            let (xoff,yoff) = get_tile(tile);
            for i in 0..8 { for j in 0..8 {
                let mut v = img.get_pixel(xoff+j,yoff+i);
                if v.0[3] == 0 { v = Rgba(Default::default()); }
                let p = pal_map.get(&v).ok_or(format!("Unknown color {:X?} at ({},{})",v,xoff+j,yoff+i))?;
                out.push(*p as u8);
            } }
        }
        Ok(Self(out))
    }
    pub fn from_paletted_image2(
        img: &impl GenericImageView<Pixel=Rgba<u8>>,
        pal: &impl GenericImageView<Pixel=Rgba<u8>>
    ) -> Result<Self,String> {
        let mut pal_map = HashMap::new();
        for (i,(_,_,mut c)) in pal.pixels().enumerate() {
            if i == 16 { break; }   // todo
            if c.0[3] == 0 { c = Rgba(Default::default()); }
            if pal_map.contains_key(&c) { continue; }
            pal_map.insert(c,i);
        }
        println!("{:?}", pal_map);
        let twidth = img.width()/8;
        let theight = img.height()/8;
        //let get_tile = |id| (id%twidth*8, id/twidth*8);
        let mut out = vec![];

        //for tile in 0..twidth*theight {
        //    let (xoff,yoff) = get_tile(tile);
        for ybig in 0..theight/8 {
            for xbig in 0..twidth/8 {
                for yoff in 0..4 {
                    for xoff in 0..8 {
                        let xoff = (xbig*8 + xoff)*8;
                        let yoff = (ybig*8 + yoff)*8;
                        for i in 0..8 { for j in 0..8 {
                            let mut v = img.get_pixel(xoff+j,yoff+i);
                            if v.0[3] == 0 { v = Rgba(Default::default()); }
                            let p = pal_map.get(&v).ok_or(format!("Unknown color {:X?} at ({},{})",v,xoff+j,yoff+i))?;
                            out.push(*p as u8);
                        } }
                    }
                    for xoff in 0..8 {
                        let xoff = (xbig*8 + xoff)*8;
                        let yoff = (ybig*8 + yoff + 4)*8;
                        for i in 0..8 { for j in 0..8 {
                            let mut v = img.get_pixel(xoff+j,yoff+i);
                            if v.0[3] == 0 { v = Rgba(Default::default()); }
                            let p = pal_map.get(&v).ok_or(format!("Unknown color {:X?} at ({},{})",v,xoff+j,yoff+i))?;
                            out.push(*p as u8);
                        } }
                    }
                }
            }
        }
        Ok(Self(out))
    }
    pub fn from_image(img: &impl GenericImageView<Pixel=Luma<u8>>) -> Self {
        let twidth = img.width()/8;
        let theight = img.height()/8;
        let get_tile = |id| (id%twidth*8, id/twidth*8);
        let mut out = vec![];

        for tile in 0..twidth*theight {
            let (xoff,yoff) = get_tile(tile);
            for i in 0..8 { for j in 0..8 {
                let v = img.get_pixel(xoff+j,yoff+i);
                out.push(v.0[0]);
            } }
        }
        Self(out)
    }
    pub fn to_image(&self) -> GrayImage {
        // todo: create this directly
        let tiles = self.0.len()/64;
        let mut out = GrayImage::new(16*8,(tiles as u32 + 15)/16*8);
        for i in 0..self.0.len()/64 {
            let img = self.tile(i);
            if let Some(img) = img {
                //println!("{}, {}", (i % 16)*8, (i / 16) * 8);
                imageops::overlay(&mut out, &img, (i%16) as u32 * 8, (i/16) as u32 * 8);
            }
        }
        out
    }
    pub fn to_format<F: Format>(&self, mut f: F, output: &mut Vec<u8>) {
        let mut c = self.0.as_slice();
        while f.to_buf(&mut c, output) {}
    }
    pub fn from_format<F: Format>(mut f: F, mut input: &[u8]) -> Self {
        let mut buf = vec![];
        while f.from_buf(&mut input, &mut buf) {}
        Self(buf)
    }
}


pub trait Format {
    fn from_buf(&mut self, input: &mut &[u8], output: &mut Vec<u8>) -> bool;
    fn to_buf(&mut self, input: &mut &[u8], output: &mut Vec<u8>) -> bool;
}

pub struct Snes<const DEPTH: usize>;
impl<const DEPTH: usize> Format for Snes<DEPTH> {
    fn from_buf(&mut self, input: &mut &[u8], output: &mut Vec<u8>) -> bool {
        let len = 8 * DEPTH;
        if input.len() < len { return false; }
        let mut buf = [0; 64];
        tile_from_bitplane::<DEPTH>(input, &mut buf);
        *input = &input[len..];
        output.extend_from_slice(&buf);
        true
    }
    fn to_buf(&mut self, input: &mut &[u8], output: &mut Vec<u8>) -> bool {
        if input.len() < 64 { return false; }
        write_bitplanes::<DEPTH>(&input[..64], output);
        *input = &input[64..];
        true
    }
}

// 2bpp: bitplane 0
// 4bpp: bitplanes 0,2
// 8bpp: bitplanes 0,2,4,6
fn write_bitplanes<const DEPTH: usize>(bitmap: &[u8], f: &mut Vec<u8>) {
    let planes = DEPTH as u8;
    // idr what the hell this does
    fn get_bitrow(plane: u8, row: &[u8]) -> u8 {
        row.iter().map(|c| (c >> plane)&1).fold(0, |a,e| a*2+e)
    }
    for plane in 0..planes/2 {
        for row in bitmap.chunks(8) {
            //println!("row {:?}", row);
            //println!("plane 1: {:08b}", get_bitrow(plane*2, row));
            //println!("plane 2: {:08b}", get_bitrow(plane*2+1, row));
            f.extend_from_slice(&[
                get_bitrow(plane*2, row),
                get_bitrow(plane*2+1, row)
            ]);
        }
    }
}

pub fn tile_from_bitplane<const DEPTH: usize>(input: &[u8], output: &mut [u8]) {
    // I don't know what any of this does. Ask p4plus2.
    // https://github.com/p4plus2/p4paint/blob/master/graphics_formats/format_planar.cpp
    let depth = DEPTH;
    for row in 0..8 {
        let mut bytes = [0; DEPTH];
        for i in 0..depth {
            let odd_plane = (depth & 1 != 0) && (i == depth - 1);
            bytes[i] = input[row * if odd_plane { 1 } else { 2 } + (i & 1) + ((i & 0xFE) << 3)];
        }
        for bit in (0..8).rev() {
            let mut pixel = 0;
            for i in 0..depth {
                pixel |= ((bytes[i] & (1 << bit)) >> bit) << i;
            }
            output[row*8+(7-bit)] = pixel;
        }
    }
}
