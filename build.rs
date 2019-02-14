extern crate shaderc;

fn main() {
    use shaderc::{Compiler, CompileOptions, ShaderKind::{Vertex, Fragment}};

    let shaders = &[
        ("src/graphics/shaders/simple_quad/simple_quad.vert", "simple_quad.vert.spirv", Vertex),
        ("src/graphics/shaders/simple_quad/simple_quad.frag", "simple_quad.frag.spirv", Fragment),
    ];

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);

    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut fails = false;
    for &(inputf, outputf, kind) in shaders {
        println!("cargo:rerun-if-changed={}", inputf);
        let input = std::fs::read_to_string(inputf).expect(inputf);
        let bin = match compiler.compile_into_spirv(&input, kind, inputf, "main", None) {
            Ok(bin) => bin,
            Err(e) => {
                eprintln!("{}", e);
                fails = true;
                continue
            }
        };

        if bin.get_num_warnings() > 0 {
            eprintln!("{}", bin.get_warning_messages());
        } else {
            eprintln!("no warnings for {}", outputf);
        }

        let output = out_dir.join(outputf);
        std::fs::write(&output, bin.as_binary_u8()).expect(outputf);
    }
    if fails {
        eprintln!("Failed to compile {:?}", fails);
        panic!("Shader compilation failed");
    }
}
