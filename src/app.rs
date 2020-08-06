use anyhow::{Result};
use humantime::parse_duration;
use procfs::net::TcpState;
use procfs::process;
use procfs::process::FDInfo;
use ptrace_do_rs::*;
use std::collections::{HashMap};
use std::time;
use std::time::{Instant};
use structopt::StructOpt;

pub struct App {
    opt: Opt,
}

impl App {
    pub fn new(opt: Opt) -> Self {
        Self { opt: opt }
    }

    pub fn close_closewait_sockets(&self) -> Result<()> {
        let sockets = self.get_closewait_sockets()?;
        let fds: Vec<_> = sockets.iter().map(|(_, info)| info.fd).collect();
        fds.chunks(self.opt.batch).into_iter().for_each(|fds| {
            let now = Instant::now();

            do_close(self.opt.pid, fds.to_vec());

            let elapsed = now.elapsed();

            if self.opt.verbose > 0 {
                println!(
                    "close pid({}) with {:?} sockets took {} milliseconds",
                    self.opt.pid,
                    fds.len(),
                    elapsed.as_millis()
                );
            } else {
                println!(
                    "close pid({}) with {} sockets took {} milliseconds",
                    self.opt.pid,
                    fds.len(),
                    elapsed.as_millis()
                );
            }
            std::thread::sleep(self.opt.interval);
        });

        Ok(())
    }

    pub fn get_closewait_sockets(&self) -> Result<HashMap<u32, FDInfo>> {
        self.get_sockets(TcpState::CloseWait)
    }

    pub fn get_sockets(&self, state: TcpState) -> Result<HashMap<u32, FDInfo>> {
        let tcp = procfs::net::tcp().unwrap_or_else(|_| vec![]);
        let tcp6 = procfs::net::tcp6().unwrap_or_else(|_| vec![]);
        let p = process::Process::new(self.opt.pid)?;
        let mut inodes = HashMap::<u32, FDInfo>::new();
        let mut closewait_sockets = HashMap::<u32, FDInfo>::new();

        p.fd().iter().for_each(|fdinfo| {
            fdinfo.iter().for_each(|fd| match fd.target {
                process::FDTarget::Socket(inode) => {
                    inodes.insert(inode, fd.clone());
                }
                _ => {}
            })
        });

        tcp.into_iter()
            .chain(tcp6)
            .filter(|t| return t.state == state)
            .for_each(|entry| {
                if let Some(info) = inodes.get(&entry.inode) {
                    closewait_sockets.insert(entry.inode, info.clone());
                }
            });

        Ok(closewait_sockets)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "closewait")]
pub struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences), help = "Enable verbose mode")]
    verbose: u8,

    #[structopt(short, long)]
    pid: i32,

    #[structopt(
        short,
        long,
        default_value = "1024",
        help = "The number of sockets batch close"
    )]
    batch: usize,

    #[structopt(
        short,
        long,
        default_value = "1s",
        parse(try_from_str = parse_duration),
        help = "The interval to close sockets"
    )]
    interval: time::Duration,
}

fn do_close(pid: i32, fds: Vec<u32>) {
    unsafe {
        let target = ptrace_do_init(pid);
        if target.is_null() {
            return;
        }
        fds.into_iter().for_each(|fd| {
            ptrace_do_syscall(target, __NR_close.into(), fd as u64, 0, 0, 0, 0, 0);
        });
        ptrace_do_cleanup(target);
    }
}
