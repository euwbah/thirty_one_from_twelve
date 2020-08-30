extern crate midir;
extern crate midly;

extern crate lazy_static;
extern crate num_traits;
extern crate num_derive;
extern crate regex;

mod processor;
mod theory;

use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, MidiOutput, MidiIO, Ignore};
use std::sync::mpsc::channel;
use midly::EventKind;
use std::thread;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir forwarding output")?;

    let in_port = select_port(&midi_in, "input")?;
    println!();
    let out_port = select_port(&midi_out, "output")?;

    println!("\nOpening connections");
    let in_port_name = midi_in.port_name(&in_port)?;
    let out_port_name = midi_out.port_name(&out_port)?;

    let mut conn_out = midi_out.connect(&out_port, "midir-forward")?;

    let (tx, rx) = channel();
    let tx1 = tx.clone();

    let mut parser_running_status: Option<u8> = None;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(&in_port, "midir-forward", move |stamp, mut message, _| {

        let parsed_msg = EventKind::parse(&mut message, &mut parser_running_status);
        match parsed_msg {
            Ok(event) => {
                tx1.send(Some(event))
            }
            Err(e) => {
                println!("error parsing: {}", e);
            }
        }

        conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
        println!("{}: {:?} (len = {})", stamp, message, message.len());
    }, ())?;

    thread::spawn(move|| {
        processor::process(rx);
    });

    println!("Connections open, forwarding from '{}' to '{}' (press enter to exit) ...", in_port_name, out_port_name);

    let mut input = String::new();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connections");
    tx.send(None);
    Ok(())
}

fn select_port<T: MidiIO>(midi_io: &T, descr: &str) -> Result<T::Port, Box<dyn Error>> {
    println!("Available {} ports:", descr);
    let midi_ports = midi_io.ports();
    for (i, p) in midi_ports.iter().enumerate() {
        println!("{}: {}", i, midi_io.port_name(p)?);
    }
    print!("Please select {} port: ", descr);
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let port = midi_ports.get(input.trim().parse::<usize>()?)
        .ok_or("Invalid port number")?;
    Ok(port.clone())
}