abplot
======

Concurrent execution of ab tests and visualizing results using gnuplot

## Dependencies
- [ab](http://httpd.apache.org/docs/2.2/en/programs/ab.html)
- [gnuplot](http://www.gnuplot.info/)
- [rust](http://www.rust-lang.org/install.html)
- [cargo](http://doc.crates.io/)

## How to run?
```
git clone git@github.com:ivpusic/abplot.git
cd abplot
cargo build
cargo run -- -c example/exampleconf.json
```

## Example results
### Lines plot
![alt tag](https://raw.githubusercontent.com/ivpusic/abplot/master/example/lines.png)
### Boxes plot (number of request per second)
![alt tag](https://raw.githubusercontent.com/ivpusic/abplot/master/example/boxes.png)
### Points plot
![alt tag](https://raw.githubusercontent.com/ivpusic/abplot/master/example/points.png)

### Configuration
You can provide your configuration using ``-c`` option. Configuration is in ``json`` format, and it looks like:

```json
{
    "urls": [{
    	"address": "http://localhost:9000/",
    	"title": "server1"
		}, {
			"address": "http://localhost:3000/",
			"title": "server2"
		}
	],
	"c": 100,
	"n": 10000,
	"graphs": {
		"lines": {
			"title": "Title of graph",
			"file": "lines.png"
		},
		"points": {
			"title": "Title of graph",
			"file": "points.png"
		},
		"boxes": {
			"title": "Title of graph",
			"file": "boxes.png"
		}
	}
}
```

##### urls
Array of urls/servers which you want to test. Every definition contains url and title 
which will be shown in graphic representation of results.

##### c (concurrency)
Number of multiple requests to perform at a time. Default is one request at a time.

##### n (requests)
Number of requests to perform for the benchmarking session.

##### graphs
Definitions of graphic representation of test results. You can use one or more. In example above we used all,
but you can omit some if you want.
Every definition contains title and file path where image representation will be saved.

## TODO:
- support for headers, cookies, etc. (-C, -H, etc. options)
- improve error handling (``scripts`` dir path, etc.)
- tests

# License
MIT
