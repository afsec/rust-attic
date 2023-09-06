use anyhow::anyhow;
use std::{fmt::Display, str::FromStr};

use super::Moeda;

#[derive(Debug)]
pub struct BrlCurrency {
    unidades: f64,
    centavos: f64,
}

impl Moeda for BrlCurrency {
    fn unidades(&self) -> f64 {
        self.unidades
    }
    fn centavos(&self) -> f64 {
        self.centavos
    }
    fn divisor_centavos() -> char {
        ','
    }
    fn divisor_milhar() -> char {
        '.'
    }
    fn codigo_internacional() -> &'static str {
        "BRL"
    }
    fn prefixo_textual() -> &'static str {
        "R$"
    }
}

impl FromStr for BrlCurrency {
    type Err = anyhow::Error;

    fn from_str(entrada: &str) -> Result<Self, Self::Err> {
        // let entrada = "1.234,56";
        let divisor_centavos = BrlCurrency::divisor_centavos();
        let vec_entrada: Vec<&str> = entrada.split(divisor_centavos).collect();
        if vec_entrada.len() != 2 {
            return Err(anyhow!("input input"));
        }

        let divisor_milhar = BrlCurrency::divisor_milhar();

        let unidades_str = vec_entrada
            .first()
            .map(|v| v.replace(divisor_milhar, ""))
            .ok_or(anyhow!("Invalid input for unidades"))?;

        let centavos_str = vec_entrada
            .last()
            .ok_or(anyhow!("Invalid input for centavos"))?;

        let centavos = centavos_str.parse::<f64>()?;
        if centavos > 99.0 {
            return Err(anyhow!("Invalid input for centavos"));
        }
        Ok(BrlCurrency {
            unidades: unidades_str.parse::<f64>()?,
            centavos,
        })
    }
}

impl Display for BrlCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // dbg!(numero_digitos_milhares);
        let unidades = self
            .unidades
            .to_string()
            .split('.')
            .next()
            .unwrap()
            .to_string();
        let centavos = self
            .centavos
            .to_string()
            .split('.')
            .next()
            .unwrap()
            .to_string();

        let output = format!(
            "{prefixo} {unidades}{divisor_centavos}{centavos} ({codigo})",
            prefixo = BrlCurrency::prefixo_textual(),
            divisor_centavos = BrlCurrency::divisor_centavos(),
            codigo = BrlCurrency::codigo_internacional()
        );
        write!(f, "{output}")
    }
}
