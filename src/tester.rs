pub mod ab {
    use conf;
    use util;
    use std::io::Command;
    use std::str::SendStr;
    use std::fmt::Show;
    use plotter::gnu;
    use uuid::Uuid;

    #[deriving(Clone)]
    pub struct AbResult {
        pub error: String,
        pub result: String,
        pub url: conf::ServerDef
    }

    #[deriving(Clone)]
    struct AbCommand<'a> {
        prog: &'a str,
        args: Vec<SendStr>
    }

    fn mostr<T: Show>(arg: T) -> SendStr {
        return arg.to_string().into_maybe_owned()
    }

    impl<'a> AbCommand<'a> {
        /// option for defining total number of requests
        fn add_n(&mut self, n: int) {
            self.args.push(mostr("-n"));
            self.args.push(mostr(n))
        }

        /// option for defining concurrency level
        fn add_c(&mut self, c: int) {
            self.args.push(mostr("-c"));
            self.args.push(mostr(c))
        }

        /// option for enabling generation of data for gnuplot
        fn add_g(&mut self, file: String) {
            self.args.push(mostr("-g"));
            self.args.push(mostr(file));
        }

        /// adding new url which will be tested
        fn add_url(&mut self, url: String) {
            self.args.push(mostr(url))
        }

        /// run tests for single url
        fn run(cfg: conf::JsonConfig, mut url: conf::ServerDef) -> Result<AbResult, AbResult> {
            let mut instance = AbCommand {
                args: vec![],
                prog: "ab"
            };

            let datafile = "._tmp/".to_string() + Uuid::new_v4().to_string();
            url.datafile = Some(datafile.clone());

            instance.add_n(cfg.n);
            instance.add_c(cfg.c);
            instance.add_g(datafile);
            instance.add_url(url.address.clone());

            // convert Vec<SendStr> to &'a [&'a str]
            let mut args = Vec::new();
            for arg in instance.args.iter() {
                args.push(arg.as_slice());
            }

            println!("will run with: {}", args.as_slice());
            // run ab command
            let output = match Command::new(instance.prog).args(args.as_slice()).output() {
                Ok(output) => output,
                Err(e) => {
                    util::rm_tmp_dir();
                    fail!(e)
                }
            };

            if !output.status.success() {
                let error = String::from_utf8_lossy(output.error.as_slice());
                return Err(AbResult {
                    result: String::new(),
                    error: error.into_string(),
                    url: url
                });
            };

            let result = String::from_utf8_lossy(output.output.as_slice()).into_string();
            return Ok(AbResult {
                result: result,
                error: String::new(),
                url: url
            })
        }
    }

    /// execute tests, and call function for making plots
    /// based on test results
    pub fn run(cfg: conf::JsonConfig) {
        let mut receivers = vec![];

        for url in cfg.urls.iter() {
            let _url = url.clone();
            let _cfg = cfg.clone();
            let (s, r): (Sender<AbResult>, Receiver<AbResult>) = channel();

            spawn(proc() {
                match AbCommand::run(_cfg, _url) {
                    Ok(result) => s.send(result),
                    Err(err) => {
                        util::rm_tmp_dir();
                        fail!(err.error)
                    }
                }
            });
            receivers.push(r);
        };

        let mut results = Vec::new();
        for r in receivers.iter() {
            let result = r.recv();
            println!("{}", result.result);
            results.push(result);
        }
        gnu::make_plot(results, cfg.clone());
    }
}
