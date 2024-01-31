/// Escreve o nome das colunas no arquivo
pub fn registra_cabecalho<W: std::io::Write>(
    writer: &mut csv::Writer<W>,
) -> Result<(), csv::Error> {
    writer.write_record([
        "Mês",
        "Número de matrizes",
        "Número de cabritos",
        "Idade mádia das matrizes",
        "Partos",
        "Abates",
        "Abates de fêmeas",
        "Abates de machos",
        "Novas matrizes",
        "Mortes de matrizes",
    ])
}

/// Escreve o equivalente a um mês no arquivo
pub fn registra_record<W: std::io::Write>(
    writer: &mut csv::Writer<W>,
    record: crate::simulation::SimStep,
) -> Result<(), csv::Error> {
    writer.write_record([
        record.mes.to_string(),
        record.matrizes.to_string(),
        record.cabritos.to_string(),
        record.idade_média_matrizes.to_string(),
        record.imediato.partos.to_string(),
        record.imediato.abates.to_string(),
        record.imediato.abates_femea.to_string(),
        record.imediato.abates_macho.to_string(),
        record.imediato.novas_matrizes.to_string(),
        record.imediato.mortes_matriz.to_string(),
    ])
}
