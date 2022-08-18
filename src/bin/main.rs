use clap::{App,Arg, ArgGroup};

fn main() {
    let matches = App::new("snesgfx")
        .author("Selicre [https://selic.re/]")
        .version("0.1")
        .about("Converts to and from various retro graphics formats.")
        .arg(
            Arg::with_name("from-snes")
                .long("from-snes")
                .help("Conversion direction")
        )
        .arg(
            Arg::with_name("to-snes")
                .long("to-snes")
                .help("Conversion direction")
        )
        .arg(
            Arg::with_name("paletted")
                .long("paletted")
                .short("p")
                .help("For reading images, specifies that they have a palette row above the graphics data")
        )
        .group(
            ArgGroup::with_name("direction")
                .args(&["from-snes", "to-snes"])
                .required(false)
        )
        .arg(
            Arg::with_name("mode")
                .help("Conversion to perform")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("input")
                .help("Input file to use")
                .required(true)
                .index(2)
        )
        .arg(
            Arg::with_name("output")
                .help("Output file to use")
                .required(true)
                .index(3)
        )
        .get_matches();
    
    let mode = matches.value_of("mode").unwrap();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let to_snes = if matches.is_present("from-snes") {
        false
    } else if matches.is_present("to-snes") {
        true
    } else if output.ends_with(".png") {
        false
    } else if input.ends_with(".png") {
        true
    } else {
        clap::Error::with_description(
            "Can't infer conversion direction! Please use flags --from-snes or --to-snes to specify.",
            clap::ErrorKind::Format
        ).exit()
    };
    match (mode, to_snes) {
        ("pal", false) => {
            let f = std::fs::read(input).unwrap();
            snesgfx::color::Palette::from_format(snesgfx::color::Snes, &f)
                .to_image(16,16)
                .save(output).unwrap();
        },
        ("pal", true) => {
            let f = image::open(input).unwrap().to_rgba8();
            let mut buf = vec![];
            snesgfx::color::Palette::from_image(&f)
                .to_format(snesgfx::color::Snes, &mut buf);
            std::fs::write(output, &buf).unwrap();
        },
        ("gfx4", false) => {
            let f = std::fs::read(input).unwrap();
            snesgfx::gfx::Graphics::from_format(snesgfx::gfx::Snes::<4>, &f)
                .to_image()
                .save(output).unwrap();
        },
        ("gfx4", true) => {
            use image::GenericImageView;
            let f = image::open(input).unwrap().to_luma8();
            let mut buf = vec![];
            snesgfx::gfx::Graphics::from_image(&f)
                .to_format(snesgfx::gfx::Snes::<4>, &mut buf);
            std::fs::write(output, &buf).unwrap();
        },
        ("gfx2", false) => {
            let f = std::fs::read(input).unwrap();
            snesgfx::gfx::Graphics::from_format(snesgfx::gfx::Snes::<2>, &f)
                .to_image()
                .save(output).unwrap();
        },
        ("gfx2", true) => {
            use image::GenericImageView;
            let f = image::open(input).unwrap().to_luma8();
            let mut buf = vec![];
            snesgfx::gfx::Graphics::from_image(&f)
                .to_format(snesgfx::gfx::Snes::<2>, &mut buf);
            std::fs::write(output, &buf).unwrap();
        },
        ("gfx4p", false) => {
            let f = std::fs::read(input).unwrap();
            let pal = image::open("palette.png").unwrap().to_rgba8();
            let img = snesgfx::gfx::Graphics::from_format(snesgfx::gfx::Snes::<4>, &f)
                .to_image();
            let mut palette = pal.pixels().map(|c| (c.0[0], c.0[1], c.0[2])).collect::<Vec<_>>();
            img.expand_palette(&palette, Some(0))
                .save(output).unwrap();
        }
        ("gfx4p", true) => {
            use image::GenericImageView;
            let f = image::open(input).unwrap().to_rgba8();
            let mut buf = vec![];
            snesgfx::gfx::Graphics::from_headered_image(&f)
                .unwrap()
                .to_format(snesgfx::gfx::Snes::<4>, &mut buf);
            std::fs::write(output, &buf).unwrap();
        },
        ("gfx4pb", true) => {
            use image::GenericImageView;
            let f = image::open(input).unwrap().to_rgba8();
            let mut buf = vec![];
            snesgfx::gfx::Graphics::from_headered_image2(&f)
                .unwrap()
                .to_format(snesgfx::gfx::Snes::<4>, &mut buf);
            std::fs::write(output, &buf).unwrap();
        },
        _ => {
            eprintln!("unknown mode {} {}", mode, if to_snes { "to snes" } else { "from snes" });
        }
    }
}
