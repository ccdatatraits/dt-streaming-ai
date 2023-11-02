use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();

    statics.add_files("./images").unwrap();
    statics.add_files("./dist").unwrap();
    ructe.compile_templates("./dist").unwrap();

    Ok(())
}
