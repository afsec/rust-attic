use std::{net::Ipv4Addr, str::FromStr};

trait Keyname {
    fn keyname() -> &'static str;
}

// const INPUT: &str = "";
const INPUT: &str = "--address=1192.168.222.254 --port=3876";
// const INPUT: &str = "--address=127.0.0.1 --port=8000 --index=default.html";

const MAX_CLI_FIELDS: usize = 2;
type AppResult<T> = Result<T, AppError>;

type AppError = ErrorStatus;

#[derive(Debug)]
enum ErrorStatus {
    FAILURE(String),
}

// Helper
#[derive(Debug)]
struct CliArg {
    key: String,
    value: String,
}

impl CliArg {
    fn key(&self) -> &String {
        &self.key
    }
    fn value(&self) -> &String {
        &self.value
    }
}

impl FromStr for CliArg {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cli_arg_vec = s.split('=').collect::<Vec<&str>>();

        dbg!(&cli_arg_vec);

        if cli_arg_vec.len() != 2 {
            return Err(AppError::FAILURE(format!(
                "Erro ao parsear CliArgs da string [{s}]"
            )))?;
        }
        let mut iter = cli_arg_vec.iter();
        let key = iter
            .next()
            .ok_or(AppError::FAILURE(format!(
                "Erro ao parsear chave CliArgs da string [{s}]"
            )))?
            .to_string();
        let value = iter
            .next()
            .ok_or(AppError::FAILURE(format!(
                "Erro ao parsear valor CliArgs da string [{s}]"
            )))?
            .to_string();

        Ok(Self { key, value })
    }
}

#[derive(Debug)]
struct Address(Ipv4Addr);

impl Keyname for Address {
    fn keyname() -> &'static str {
        "--address"
    }
}

impl Default for Address {
    fn default() -> Self {
        Self(Ipv4Addr::new(127, 0, 0, 1))
    }
}

impl TryFrom<&Vec<CliArg>> for Address {
    type Error = AppError;

    fn try_from(args: &Vec<CliArg>) -> Result<Self, Self::Error> {
        dbg!(&args);
        let vec_found = args
            .iter()
            .filter(|item| item.key() == Self::keyname())
            .collect::<Vec<&CliArg>>();
        // dbg!(&vec_found);
        // let found = vec_found
        //     .last()
        //     .map(|i| i.value().into())
        //     .unwrap_or_default();
        let maybe_found = vec_found.last().map(|item| item.value());
        let found = match maybe_found {
            Some(inner) => inner,
            None => return Ok(Default::default()),
        };

        let parsed = found.parse::<Ipv4Addr>().map_err(|error| {
            AppError::FAILURE(format!(
                "Error on parsing Port from String to u16. Reason: {error}"
            ))
        })?;
        Ok(Address(parsed))
    }
}

#[derive(Debug)]
struct Port(u16);
impl Keyname for Port {
    fn keyname() -> &'static str {
        "--port"
    }
}

impl Default for Port {
    fn default() -> Self {
        Self(8000)
    }
}

impl From<u16> for Port {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl TryFrom<&Vec<CliArg>> for Port {
    type Error = AppError;

    fn try_from(args: &Vec<CliArg>) -> Result<Self, Self::Error> {
        dbg!(&args);
        let vec_found = args
            .iter()
            .filter(|item| item.key() == Self::keyname())
            .collect::<Vec<&CliArg>>();
        dbg!(&vec_found);
        let maybe_found = vec_found.last().map(|item| item.value());
        let found = match maybe_found {
            Some(inner) => inner,
            None => return Ok(Default::default()),
        };

        let parsed = found.parse::<u16>().map_err(|error| {
            AppError::FAILURE(format!(
                "Error on parsing Port from String to u16. Reason: {error}"
            ))
        })?;

        Ok(Self(parsed))
    }
}

#[derive(Debug)]
struct Cli {
    address: Address,
    port: Port,
}
impl Default for Cli {
    fn default() -> Self {
        Self {
            address: Default::default(),
            port: Default::default(),
        }
    }
}
impl TryFrom<Vec<String>> for Cli {
    type Error = AppError;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        // 1. Validar o tamanho do campos;
        let args_lenv = args.len();
        if args_lenv > MAX_CLI_FIELDS {
            return Err(AppError::FAILURE(format!(
                "Quantidade de campos acima do limite: {MAX_CLI_FIELDS}"
            )))?;
        }
        dbg!(&args);

        // 2. Fatiar a entrada em chave-valor;
        let cli_args = args
            .into_iter()
            // 3. Identificar a chave. Ver se ela existe;
            .map(|s| CliArg::from_str(&s))
            .collect::<AppResult<Vec<CliArg>>>()?;
        // dbg!(&cli_args);

        // 4. Parsear e Validar o valor;
        let address = Address::try_from(&cli_args)?;
        // 4. Parsear e Validar o valor;
        let port = Port::try_from(&cli_args)?;
        dbg!(&address, &port);
        Ok(Self { address, port })
    }
}

fn main() -> AppResult<()> {
    let args = INPUT
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let cli = Cli::try_from(args)?;

    // 5. Imprimir estrutura.
    dbg!(&cli);
    Ok(())
}
