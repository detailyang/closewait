<p align="center">
  <b>
    <span style="font-size:larger;">closewait</span>
  </b>
  <br />
   <a href="https://travis-ci.com/detailyang/closewait"><img src="https://travis-ci.com/detailyang/closewait.svg?token=thDZbmEQtVwYMM6yT8Dv&branch=master"/></a>
   <br />
   <b>closewait closes closed-wait sockets via procfs and ptrace</b>
</p>

# Purpose

Kill the close-wait sockets on the fly via [procfs](https://man7.org/linux/man-pages//man5/procfs.5.html) like `ss state CLOSE-WAIT --kill` in low version kernel which do not have [SOCK Destory](https://lwn.net/Articles/666592/) kernel feature.

# Usage

## oneline command

`closewait -p pid`

```bash
closewait 0.1.0

USAGE:
    closewait [FLAGS] [OPTIONS] --pid <pid>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose mode

OPTIONS:
    -b, --batch <batch>          The number of sockets batch to close [default: 1024]
    -i, --interval <interval>    The interval time to close sockets [default: 1s]
    -p, --pid <pid>
```

# Install

Download the binary from [closewait/releases](https://github.com/detailyang/closewait/releases)

> only support linux platform

# License
closewait is under the [MIT license](/LICENSE). See the [LICENSE](/LICENSE) file for details.
