use std::time::Duration;

use crate::{jprintln, utils::sleep};
use anyhow::{anyhow, Result};
use midir::{Ignore, MidiInput};

pub async fn setup_midi<F, T>(callback: F, data: T) -> Result<()>
where
    T: Send + 'static,
    F: FnMut(u64, &[u8], &mut T) + Send + 'static,
{
    let window = web_sys::window().ok_or(anyhow!("window not found"))?;
    let mut midi_input = MidiInput::new("NoteCards")?;
    midi_input.ignore(Ignore::None);

    let ports = loop {
        let ports = midi_input.ports();
        if ports.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        break ports;
    };

    let selected_port = loop {
        let Some(p) = window
            .prompt_with_message(&format!(
                "Select MIDI ports: {}",
                ports
                    .iter()
                    .fold((String::new(), 0), |(s, id), p| (
                        format!("{s}\n{id}: {}", midi_input.port_name(p).unwrap()),
                        id + 1
                    ))
                    .0
            ))
            .map_err(|e| anyhow!("error: {e:?}"))?
        else {
            jprintln!("Please select a port id");
            continue;
        };

        let index = p.parse::<usize>()?;
        let Some(p) = ports.get(index) else {
            jprintln!("invalid port id");
            continue;
        };
        break p;
    };

    let port_name = midi_input.port_name(selected_port)?;
    let conn_in = midi_input
        .connect(selected_port, &port_name, callback, data)
        .map_err(|e| anyhow!(e.to_string()))?;
    Box::leak(Box::new(conn_in));

    Ok(())
}
