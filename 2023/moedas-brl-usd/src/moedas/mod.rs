mod brl;
mod usd;
pub use self::brl::BrlCurrency;
pub use self::usd::UsdCurrency;

pub trait Moeda {
    fn unidades(&self) -> f64;
    fn centavos(&self) -> f64;
    fn divisor_centavos() -> char;
    fn divisor_milhar() -> char;
    fn codigo_internacional() -> &'static str;
    fn prefixo_textual() -> &'static str;
}
