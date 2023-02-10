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
            pr.simulate(trace, "LRU");
            println!(
                "Number of page faults with LRU and frame size {} in trace {}: {}",
                tamanho_quadro, trace, pr.page_faults
            );

            let mut pr = Memory::new(100, *tamanho_quadro);
            pr.simulate(trace, "FIFO");
            println!(
                "Number of page faults with FIFO and frame size {} in trace {}: {}",
                tamanho_quadro, trace, pr.page_faults
            );

            let mut pr = Memory::new(100, *tamanho_quadro);
            pr.simulate(trace, "VMS");
            println!(
                "Number of page faults with VMS and frame size {} in trace {}: {}",
                tamanho_quadro, trace, pr.page_faults
            );
        }
    }
}
