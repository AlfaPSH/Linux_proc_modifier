#[derive(Debug, Clone)]
pub enum SearchFilter {
    Exact,
    Changed,
    Unchanged,
    Increased,
    Decreased,
    Range(f64, f64), // Para buscar valores en un rango
}
