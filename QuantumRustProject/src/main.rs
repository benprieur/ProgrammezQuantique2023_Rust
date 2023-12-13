use q1tsim::circuit::Circuit;

fn main() {
    bell_state();
}

fn bell_state() {

    let qubits = 2;
    let bits_classiques = 2;
    let mut circuit = Circuit::new(qubits, bits_classiques);
    
    circuit.h(0);
    circuit.cx(0, 1);
    
    circuit.measure_all(&[0, 1]);
    
    let nb_essais = 10000;
    circuit.execute(nb_essais);
    
    let hist = circuit.histogram_vec();
    for resultats in hist.iter()
    {
        for resultat in resultats.iter()
        {
            println!("{}", resultat);
        }
    }
}