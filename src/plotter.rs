pub mod gnu {
    use tester::ab::AbResult;
    use conf::JsonConfig;
    use conf::LinesGraph;
    use conf::PointsGraph;
    use conf::BoxesGraph;
    use std::io::Command;
    use std::io::File;
    use util;

    /// generating gnuplot options
    fn make_conf(datafile: &str, title: &str, results: Vec<AbResult>, plot_def: &str) -> String {
        let mut options = String::new();
        options.push_str("set output '");
        options.push_str(datafile);
        options.push_str("';set title '");
        options.push_str(title);

        options.push_str("';plot ");
        // set data source and line names
        for result in results.iter() {
            options.push_str("'");
            options.push_str(result.url.datafile.clone().unwrap().as_slice());
            options.push_str(plot_def);
            options.push_str(result.url.title.as_slice());
            options.push_str("',")
        }
        options.pop();

        return options;
    }

    /// running gnuplot command
    fn run_gnuplot(args: Vec<&str>) {
        match Command::new("gnuplot").args(args.as_slice()).output() {
            Ok(output) => {
                if !output.status.success() {
                    println!("{}", String::from_utf8_lossy(output.error.as_slice()));
                }
            },
            Err(e) => {
                util::rm_tmp_dir();
                fail!("failed to execute process: {}", e)
            }
        };

    }

    fn make_lines_plot(cfg: LinesGraph, results: Vec<AbResult>) {
        let plot_def = "' using 9 smooth sbezier with lines title '";
        let options = make_conf(cfg.file.as_slice(), cfg.title.as_slice(), results, plot_def);

        let mut args = Vec::new();
        args.push("scripts/lines.P");
        args.push("-e");
        args.push(options.as_slice());

        run_gnuplot(args);
    }

    fn make_points_plot(cfg: PointsGraph, results: Vec<AbResult>) {
        let plot_def = "' every ::2 using 2:5 with points title '";
        let options = make_conf(cfg.file.as_slice(), cfg.title.as_slice(), results, plot_def);

        let mut args = Vec::new();
        args.push("scripts/points.P");
        args.push("-e");
        args.push(options.as_slice());

        run_gnuplot(args);
    }

    fn make_boxes_plot(cfg: BoxesGraph, results: Vec<AbResult>) {
        let re = regex!(r"Requests per second:\s*(\d+.?\d+)");
        let datafile = "._tmp/boxes.txt";
        let mut file = File::create(&Path::new(datafile));

        let mut i: i32 = 0;
        for result in results.iter() {
            let cap = re.captures(result.result.as_slice()).unwrap();
            let mut line_to_write = String::new();
            line_to_write.push_str(i.to_string().as_slice());
            line_to_write.push_str(" ");
            line_to_write.push_str(result.url.title.as_slice());
            line_to_write.push_str(" ");
            line_to_write.push_str(cap.at(1));
            line_to_write.push_str("\n");

            match file.write_str(line_to_write.as_slice()) {
                Ok(_) => (),
                Err(e) => {
                    util::rm_tmp_dir();
                    fail!(e.desc)
                }
            }
            i += 1;
        }

        let mut options = String::new();
        options.push_str("set output '");
        options.push_str(cfg.file.as_slice());
        options.push_str("';set title '");
        options.push_str(cfg.title.as_slice());
        options.push_str("';plot '");
        options.push_str(datafile);
        options.push_str("' using 1:3:xtic(2) with boxes notitle;");

        let mut args = Vec::new();
        args.push("scripts/boxes.P");
        args.push("-e");
        args.push(options.as_slice());

        run_gnuplot(args);
    }

    /// based on json config, we are generating zero or more plots
    pub fn make_plot(results: Vec<AbResult>, cfg: JsonConfig) {
        match cfg.graphs.lines {
            Some(lines) => make_lines_plot(lines, results.clone()),
            None => ()
        }

        match cfg.graphs.points {
            Some(points) => make_points_plot(points, results.clone()),
            None => ()
        }
        match cfg.graphs.boxes {
            Some(lines) => make_boxes_plot(lines, results.clone()),
            None => ()
        }
    }
}
