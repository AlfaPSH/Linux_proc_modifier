#[derive(Debug, Clone, Copy)]
pub enum DataType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    String,
    Bytes,
}

#[derive(Debug, Clone)]
pub enum SearchFilter {
    Exact,
    Changed,
    Unchanged,
    Increased,
    Decreased,
    Range(f64, f64), // Para buscar valores en un rango
}
