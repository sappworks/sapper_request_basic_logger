#[macro_use]
extern crate log;
extern crate time;
extern crate sapper;


use sapper::{Request, Response, Result, Key};

pub struct BasicLogger;
impl Key for BasicLogger { type Value = u64; }

pub fn init(req: &mut Request) -> Result<()> {
    
    req.ext_mut().insert::<BasicLogger>(time::precise_time_ns());
    
    Ok(())
}

pub fn log(req: &Request, res: &mut Response) -> Result<()> {
    let exit_time = time::precise_time_ns();
    let entry_time = *req.ext().get::<BasicLogger>().unwrap();
    let response_time_ms = (exit_time - entry_time) as f64 / 1000000.0;
    
    // info!()
    let timedate = format!("[{}] ", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap());
    let method = format!("{}", req.method());
    let (path, query) = req.uri();
    let status = format!("{}", res.status());
    let response_time = format!("{} ms", response_time_ms);
    println!("{} {} {} {:?} -> {} ({})", timedate, method, path, query, status, response_time);
    
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
