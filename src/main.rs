use std::io::{BufWriter, BufReader,Write, BufRead};
use std::net::{TcpListener,TcpStream};
use thiserror::Error;


#[derive(Debug, Error)]
enum AppError {
    #[error("failed to write response data")]
    WriteResponseErr,

    #[error("failed to read request")]
    ReadRequestErr
}

type Result<T> = std::result::Result<T, AppError>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Server started on 127.0.0.1:6379");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let mut reader = BufReader::new(&_stream);
                read_response(&mut reader)?;

                let mut writer = BufWriter::new(&_stream);
                write_response(&mut writer, "+PONG\r\n")?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn read_response(reader: &mut BufReader<&TcpStream>) -> Result<()> {
    let mut msg = String::new();

    if let Err(e) = reader.read_line(&mut msg) {
        eprintln!("{:?}", e);
        return Err(AppError::ReadRequestErr);
    }
    println!("{}", msg);

    Ok(())
}

fn write_response(writer: &mut BufWriter<&TcpStream>, data: &str) -> Result<()> {
    if let Err(e) = writer.write(data.as_bytes()) {
        eprint!("{:?}", e);
        return Err(AppError::WriteResponseErr);
    }

    Ok(())
}
