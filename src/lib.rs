use anyhow::Result;
use aom_decode::{
  avif::{Avif, Image},
  Config,
};
pub use image;
use image::{DynamicImage, ImageBuffer, ImageFormat};

pub fn load_avif(bin: &[u8]) -> Result<DynamicImage> {
  let avif = Avif::decode(bin, &Config { threads: 1 })?.convert()?;
  match avif {
    Image::RGB8(avif) => {
      //dbg!("RGB8");
      let width = avif.width() as u32;
      let height = avif.height() as u32;
      let pxli = avif.pixels();
      let mut li = Vec::with_capacity(pxli.len() * 3);
      for px in pxli {
        li.push(px.r);
        li.push(px.g);
        li.push(px.b);
      }
      let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
      return Ok(img.into());
    }
    Image::RGB16(avif) => {
      //dbg!("RGB16");
      let width = avif.width() as u32;
      let height = avif.height() as u32;
      let pxli = avif.pixels();
      let mut li = Vec::with_capacity(pxli.len() * 3);
      for px in pxli {
        li.push((px.r >> 8) as u8);
        li.push((px.g >> 8) as u8);
        li.push((px.b >> 8) as u8);
      }
      let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
      return Ok(img.into());
    }
    Image::RGBA8(avif) => {
      //dbg!("RGBA8");
      let width = avif.width() as u32;
      let height = avif.height() as u32;
      let pxli = avif.pixels();
      let mut li = Vec::with_capacity(pxli.len() * 3);
      // PNG pixel RGB = (a * r + (255 - a)) / 255, (a * g + (255 - a)) / 255, (a * b + (255 - a)) / 255
      for px in pxli {
        if px.a == 0 {
          li.push(255);
          li.push(255);
          li.push(255);
        } else if px.a == 255 {
          li.push(px.r);
          li.push(px.g);
          li.push(px.b);
        } else {
          let a = (px.a as f64) / 255.0;
          let bg = (1.0 - a) * 255.0;
          let r = px.r as f64;
          let g = px.g as f64;
          let b = px.b as f64;
          li.push((r * a + bg) as u8);
          li.push((g * a + bg) as u8);
          li.push((b * a + bg) as u8);
        }
      }
      let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
      return Ok(img.into());
    }
    Image::RGBA16(avif) => {
      //dbg!("RGBA16");
      let width = avif.width() as u32;
      let height = avif.height() as u32;
      let pxli = avif.pixels();
      let mut li = Vec::with_capacity(pxli.len() * 3);
      const BG: u16 = 65535;
      const BG64: f64 = BG as f64;

      for px in pxli {
        if px.a == 0 {
          li.push(255);
          li.push(255);
          li.push(255);
        } else if px.a == BG {
          li.push((px.r >> 8) as u8);
          li.push((px.g >> 8) as u8);
          li.push((px.b >> 8) as u8);
        } else {
          let a = (px.a as f64) / BG64;
          let pxbg = (1.0 - a) * BG64;
          let r = px.r as f64;
          let g = px.g as f64;
          let b = px.b as f64;
          li.push(((r * a + pxbg) as u16 >> 8) as u8);
          li.push(((g * a + pxbg) as u16 >> 8) as u8);
          li.push(((b * a + pxbg) as u16 >> 8) as u8);
        }
      }
      let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
      return Ok(img.into());
    }
    _ => {
      todo!();
    }
  }
  //   Image::Gray8(avif) => {
  //     let (out, width, height) = avif.into_contiguous_buf();
  //     let buf = ImageBuffer::from_raw(width, height, out);
  //     lodepng::encode_file(&out_path, &out, width, height, lodepng::ColorType::GREY, 8)
  //   }
  //   Image::Gray16(avif) => {
  //     let mut out = Vec::new();
  //     for px in avif.pixels() {
  //       out.push((px >> 8) as u8);
  //     }
  //     lodepng::encode_file(
  //       &out_path,
  //       &out,
  //       avif.width(),
  //       avif.height(),
  //       lodepng::ColorType::GREY,
  //       8,
  //     )
  //   }
}

pub fn load_image(ext: Option<&str>, bin: &[u8]) -> Result<DynamicImage> {
  if let Some(ext) = ext {
    if ext == "avif" {
      return load_avif(&bin);
    }
    if let Some(format) = ImageFormat::from_extension(ext) {
      if format == ImageFormat::Avif {
        return load_avif(&bin);
      }
      if let Ok(r) = image::load_from_memory_with_format(bin, format) {
        return Ok(r);
      }
    }
  }
  let format = image::guess_format(bin)?;
  if format == ImageFormat::Avif {
    return load_avif(&bin);
  }
  Ok(image::load_from_memory_with_format(bin, format)?)
}
