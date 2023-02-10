mod substitute;
use substitute::Memory;

fn main() {
    let traces = vec![
        "bzip.trace",
        "gcc.trace",
        "sixpack.trace",
        "swim.trace",
        "bigone.trace",
    ];
    let tamanhos_quadro = vec![4096, 8192, 16384, 32768];

    for trace in traces.iter() {
        for tamanho_quadro in tamanhos_quadro.iter() {
            let mut pr = Memory::new(100, *tamanho_quadro);
            pr.simular(trace, "LRU");
            println!(
                "Número de falhas de página com LRU e tamanho de quadro {} no trace {}: {}",
                tamanho_quadro, trace, pr.page_faults
            );

            let mut pr = Memory::new(100, *tamanho_quadro);
            pr.simular(trace, "FIFO");
            println!(
                "Número de falhas de página com FIFO e tamanho de quadro {} no trace {}: {}",
                tamanho_quadro, trace, pr.page_faults
            );

            let mut pr = Memory::new(100, *tamanho_quadro);
            pr.simular(trace, "VMS");
            println!(
                "Número de falhas de página com VMS e tamanho de quadro {} no trace {}: {}",
                tamanho_quadro, trace, pr.page_faults
            );
        }
    }
}
