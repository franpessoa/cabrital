pub fn write_head<W: std::io::Write>(writer: &mut csv::Writer<W>) -> Result<(), csv::Error> {
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

pub fn write_record<W: std::io::Write>(
    writer: &mut csv::Writer<W>,
    record: crate::simulation::SimulationStep,
) -> Result<(), csv::Error> {
    writer.write_record([
        record.mes.to_string(),
        record.matrizes.to_string(),
        record.cabritos.to_string(),
        record.idade_média_matrizes.to_string(),
        record.immediate.partos.to_string(),
        record.immediate.abates.to_string(),
        record.immediate.abates_femea.to_string(),
        record.immediate.abates_macho.to_string(),
        record.immediate.novas_matrizes.to_string(),
        record.immediate.mortes_matriz.to_string(),
    ])
}
