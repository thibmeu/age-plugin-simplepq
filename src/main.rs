use age_plugin_hpke::agile::{AeadAlg, KemAlg};
use anyhow::{anyhow, Result};
use std::{fs, io};

mod cli;

pub const PLUGIN_NAME: &str = "simplepq";
pub const KEM: KemAlg = KemAlg::X25519Kyber768Draft00;
pub const AEAD: AeadAlg = AeadAlg::ChaCha20Poly1305;
pub const ASSOCIATED_DATA: &str = "simplepq";

pub fn file_or_stdin(input: Option<String>) -> Result<Box<dyn io::Read>> {
    let reader: Box<dyn io::Read> = match input {
        Some(path) => Box::new(io::BufReader::new(
            fs::File::open(path).map_err(|_e| anyhow!("cannot read input file"))?,
        )),
        None => Box::new(io::BufReader::new(io::stdin())),
    };
    Ok(reader)
}

pub fn file_or_stdout(output: Option<String>) -> Result<Box<dyn io::Write>> {
    let writer: Box<dyn io::Write> = match output {
        Some(path) => Box::new(io::BufWriter::new(
            fs::File::create(path).map_err(|_e| anyhow!("cannot create output file"))?,
        )),
        None => Box::new(io::BufWriter::new(io::stdout())),
    };
    Ok(writer)
}

fn main() {
    let cli = cli::build();

    if let Some(state_machine) = cli.age_plugin {
        return age_plugin_hpke::run_state_machine(PLUGIN_NAME, &state_machine).unwrap();
    }

    let to_print = if cli.convert {
        let mut input = file_or_stdin(cli.input).unwrap();
        let mut identity = String::new();
        input.read_to_string(&mut identity).unwrap();
        let identity = age_plugin_hpke::identity_from_string(&identity);
        let recipient = age_plugin_hpke::convert_identity_to_recipient(&identity);
        age_plugin_hpke::recipient_to_string(PLUGIN_NAME, &recipient)
    } else {
        let (identity, recipient) = age_plugin_hpke::new_identity(KEM, AEAD, ASSOCIATED_DATA);
        age_plugin_hpke::new_identity_to_string(PLUGIN_NAME, &identity, &recipient)
    };

    let mut output = file_or_stdout(cli.output).unwrap();
    output.write_all(to_print.as_bytes()).unwrap();
}
