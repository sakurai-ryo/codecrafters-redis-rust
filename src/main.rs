use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use thiserror::Error;

const BUF_SIZE: usize = 4096;

#[derive(Debug, Error)]
enum AppError {
    #[error("failed to write response data")]
    WriteResponseErr,

    #[error("failed to read request")]
    ReadRequestErr,
}

type Result<T> = std::result::Result<T, AppError>;

/*
*2\r\n$7\r\nCOMMAND\r\n$4\r\nDOCS\r\n
*1\r\n$4\r\nping\r\n
 */

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Server started on 127.0.0.1:6379");

    for stream in listener.incoming() {
        println!("request coming");
        match stream {
            Ok(mut _stream) => {
                handle_connection(&_stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(stream: &TcpStream) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    // https://github.com/tidwall/redcon/blob/9f71787fcde3a344846f585ee885acfd4c933925/redcon.go#LL752C3-L752C27
    let mut buf = [0u8; BUF_SIZE];
    loop {
        match reader.read(&mut buf) {
            Ok(b) => {
                println!(
                    "bytes={:?}, data={:?}",
                    b,
                    std::str::from_utf8(&buf).unwrap()
                );
                if b == 0 {
                    break;
                }

                write_response(&mut writer, "+PONG\r\n".as_bytes())?;
                writer.flush().unwrap();
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(AppError::ReadRequestErr);
            }
        }
    }

    Ok(())
}

fn write_response(writer: &mut BufWriter<&TcpStream>, data: &[u8]) -> Result<()> {
    if let Err(e) = writer.write(data) {
        eprint!("{:?}", e);
        return Err(AppError::WriteResponseErr);
    }

    Ok(())
}
