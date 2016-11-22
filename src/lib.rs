pub mod math;
pub mod system;
pub mod renderer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

    #[test]
    fn fs_tests() {
        use system::filesystem;
        use system::config;

        // read random file
        let s = filesystem::read_file("data/config.json");
        
        // read and decode JSON config file
        let conf = config::Config::new("data/config.json");
        let fwinwidth = conf.get_f64("iWindowWidth");
        let fwinheight = conf.get_u64("iWindowHeight");
        println!("read window size : {} {}", fwinwidth, fwinheight);
    }
}
