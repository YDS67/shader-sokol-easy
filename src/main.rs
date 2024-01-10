use std::path::Path;

fn main() {
    let gl_vert = Path::new("src/vert.glsl");
    let gl_frag = Path::new("src/frag.glsl");
    //let hl_vert: String = format!("vert.hlsl");
    //let hl_frag: String = format!("frag.hlsl");
    shader_comp(
        &string_to_bytes(gl_vert), 
        &string_to_bytes(gl_frag),
       // &string_to_bytes(&hl_vert), 
       // &string_to_bytes(&hl_frag),
    )
}

fn string_to_bytes(file_path: &Path) -> Vec<u8> {
    let file = std::fs::read_to_string(file_path).expect(&format!("Error opening file"));
    file.as_bytes().to_vec()
}

fn shader_comp(vert1: &[u8], frag1: &[u8]) {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    use std::io::Write;

    let file = File::open(Path::new("src/shader_code.rs")).expect(&format!("Error opening file {}", "shader_code.rs"));
    let reader = BufReader::new(file);
    let mut file_contents: Vec<String> = Vec::new();

    let file_path = Path::new("src/shader.rs");
    let mut my_file = File::create(file_path).expect("Error creating file");

    let mut n = 0;
    for line in reader.lines() {
        file_contents.push(line.expect("Error reading line"));
        n += 1;
    }

    for j in 0..n {
        writeln!(my_file, "{}", file_contents[j]).expect("Error writing to file");
    }

    let vert_l = vert1.len();
    let frag_l = frag1.len();

    // VERTEX SHADER

    writeln!(my_file, "{}", " ").expect("Error writing to file");   
    writeln!(my_file, "pub const VS_SOURCE_GLSL330: [u8; {}] = [", vert_l+3).expect("Error writing to file");
    for v in vert1 {
        write!(my_file, "0x{:x}, ", v).expect("Error writing to file");
    }
    write!(my_file, "0x0a, 0x0a, 0x00,").expect("Error writing to file");
    write!(my_file, "\n").expect("Error writing to file");
    writeln!(my_file, "];").expect("Error writing to file");

    // FRAGMENT SHADER

    writeln!(my_file, "{}", " ").expect("Error writing to file");   
    writeln!(my_file, "pub const FS_SOURCE_GLSL330: [u8; {}] = [", frag_l+3).expect("Error writing to file");
    for f in frag1 {
        write!(my_file, "0x{:x}, ", f).expect("Error writing to file");
    }
    write!(my_file, "0x0a, 0x0a, 0x00,").expect("Error writing to file");
    write!(my_file, "\n").expect("Error writing to file");
    writeln!(my_file, "];").expect("Error writing to file");
    
}