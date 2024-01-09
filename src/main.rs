fn main() {
    let flnm_vert: String = format!("vert.glsl");
    let flnm_frag: String = format!("frag.glsl");
    shader_comp(&string_to_bytes(&flnm_vert), &string_to_bytes(&flnm_frag))
}

fn string_to_bytes(file_path: &str) -> Vec<u8> {
    let file = std::fs::read_to_string(file_path).expect(&format!("Error opening file {}", file_path));
    file.as_bytes().to_vec()
}

fn shader_comp(vert: &[u8], frag: &[u8]) {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    let file = File::open("shader_code.rs").expect(&format!("Error opening file {}", "shader_code.rs"));
    let reader = BufReader::new(file);
    let mut file_contents: Vec<String> = Vec::new();

    let fl_name = format!("shader.rs");
    let file_path: PathBuf = [fl_name].iter().collect();
    let mut my_file = File::create(file_path).expect("Error creating file");

    let mut n = 0;
    for line in reader.lines() {
        file_contents.push(line.expect("Error reading line"));
        n += 1;
    }

    let vert_l = vert.len();
    let frag_l = frag.len();

    for j in 0..n {
        writeln!(my_file, "{}", file_contents[j]).expect("Error writing to file");
    }

    // VERTEX SHADER

    writeln!(my_file, "{}", " ").expect("Error writing to file");   
    writeln!(my_file, "pub const VS_SOURCE_GLSL330: [u8; {}] = [", vert_l+3).expect("Error writing to file");
    for v in vert {
        write!(my_file, "0x{:x}, ", v).expect("Error writing to file");
    }
    write!(my_file, "0x0a, 0x0a, 0x00,").expect("Error writing to file");
    write!(my_file, "\n").expect("Error writing to file");
    writeln!(my_file, "];").expect("Error writing to file");

    // FRAGMENT SHADER

    writeln!(my_file, "{}", " ").expect("Error writing to file");   
    writeln!(my_file, "pub const FS_SOURCE_GLSL330: [u8; {}] = [", frag_l+3).expect("Error writing to file");
    for f in frag {
        write!(my_file, "0x{:x}, ", f).expect("Error writing to file");
    }
    write!(my_file, "0x0a, 0x0a, 0x00,").expect("Error writing to file");
    write!(my_file, "\n").expect("Error writing to file");
    writeln!(my_file, "];").expect("Error writing to file");
    
}