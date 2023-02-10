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
    let frame_sizes = vec![4096, 8192, 16384, 32768];

    for trace in traces.iter() {
        for frame_size in frame_sizes.iter() {
            let mut pr = Memory::new(100, *frame_size);
            pr.simulate(trace, "LRU");
            println!(
                "Number of page faults with LRU and frame size {} in trace {}: {}",
                frame_size, trace, pr.page_faults
            );

            let mut pr = Memory::new(100, *frame_size);
            pr.simulate(trace, "FIFO");
            println!(
                "Number of page faults with FIFO and frame size {} in trace {}: {}",
                frame_size, trace, pr.page_faults
            );

            let mut pr = Memory::new(100, *frame_size);
            pr.simulate(trace, "VMS");
            println!(
                "Number of page faults with VMS and frame size {} in trace {}: {}",
                frame_size, trace, pr.page_faults
            );
        }
    }
}
