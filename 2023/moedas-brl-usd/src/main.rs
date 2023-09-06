use crate::moedas::{BrlCurrency, Moeda, UsdCurrency};
use std::str::FromStr;

mod moedas;

fn main() -> anyhow::Result<()> {
    testar_brl()?;
    // testar_usd()?;
    Ok(())
}

fn testar_brl() -> anyhow::Result<()> {
    let entrada_brl = "1.234,56";
    let brl = BrlCurrency::from_str(entrada_brl)?;

    dbg!(&brl);
    // let brl_cod_intl = BrlCurrency::codigo_internacional();
    // dbg!(&brl_cod_intl);
    // let usd_centavos = brl.centavos();
    // dbg!(&usd_centavos);

    // dbg!(&usd_centavos);
    println!("Moeda: {brl}");
    Ok(())
}

fn testar_usd() -> anyhow::Result<()> {
    let entrada_usd = "123,456.78";
    let usd = UsdCurrency::from_str(entrada_usd)?;
    // dbg!(&usd);
    let usd_centavos = usd.centavos();
    // TODO: Implement Display fo UsdCurrency
    // println!("Moeda: {brl}");
    Ok(())
}
