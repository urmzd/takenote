pub mod init {

    #[allow(dead_code)]
    struct DirConfig {
        parent: String,
        children: Vec<String>,
    }

    #[allow(dead_code)]
    struct Config {
        default_project: String,
    }

    #[allow(dead_code)]
    fn read_config() {
        let _random = DirConfig {
            parent: String::from("RANDOM"),
            children: Vec::new(),
        };
    }
}
