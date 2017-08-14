use input::read_input_files;
use config::Config;


pub fn build(config: Config) -> Result<(), String> {
    let input = try!(read_input_files(config.input));
    println!("{}", input);
    return Ok(());
}
