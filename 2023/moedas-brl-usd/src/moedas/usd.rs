use anyhow::anyhow;
use std::str::FromStr;

use super::Moeda;

#[derive(Debug)]
pub struct UsdCurrency {
    unidades: f64,
    centavos: f64,
}

impl Moeda for UsdCurrency {
    fn unidades(&self) -> f64 {
        self.unidades
    }
    fn centavos(&self) -> f64 {
        self.centavos
    }
    fn divisor_centavos() -> char {
        '.'
    }
    fn divisor_milhar() -> char {
        ','
    }
    fn codigo_internacional() -> &'static str {
        "USD"
    }
    fn prefixo_textual() -> &'static str {
        "US$"
    }
}

impl FromStr for UsdCurrency {
    type Err = anyhow::Error;

    fn from_str(entrada: &str) -> Result<Self, Self::Err> {
        // let entrada = "1.234,56";
        let divisor_centavos = UsdCurrency::divisor_centavos();
        let vec_entrada: Vec<&str> = entrada.split(divisor_centavos).collect();
        if vec_entrada.len() != 2 {
            return Err(anyhow!("input input"));
        }

        let divisor_milhar = UsdCurrency::divisor_milhar();

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
        Ok(UsdCurrency {
            unidades: unidades_str.parse::<f64>()?,
            centavos,
        })
    }
}
