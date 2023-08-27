use clap::Parser;

/// Plugin for age to interact with ML-KEM Post-Quantum algorithm (https://csrc.nist.gov/pubs/fips/203/ipd)
///
/// Example:
///     $ age-plugin-simplepq -o my_id.key
///     $ age-plugin-simplepq -y -o my_id.key.pub my_id.key
///     $ tar cvz ~/data | age -R my_id.key.pub > data.tar.gz.age
///     $ age --decrypt -i my_id.key -o data.tar.gz data.tar.gz.age
#[derive(Parser)]
#[command(author, version, about, verbatim_doc_comment)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long, hide = true, group = "action")]
    pub age_plugin: Option<String>,
    /// Write the result to the file at path OUTPUT.
    #[arg(short, long)]
    pub output: Option<String>,
    /// Convert an identity file to a recipients file.
    #[arg(short = 'y', default_value_t = false)]
    pub convert: bool,
    pub input: Option<String>,
}

#[allow(dead_code)]
pub fn build() -> Cli {
    Cli::parse()
}
