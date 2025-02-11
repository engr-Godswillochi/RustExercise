use std::fs;
use std::io;
// use zip::ZipArchive;
// use std::iter::zip;

fn main() {
    std::process::exit(logic())
}
fn logic() -> i32
{
    let args: Vec<_> =std::env::args().collect();

    if args.len() < 2
    {
        println!("Usage: {} <FileName>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len()
    {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name()
        {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty()
            {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (*file.name()).ends_with('/')
        {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        }
        else{
            println!(
                "File {} expected to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode()
            {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}