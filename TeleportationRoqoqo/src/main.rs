
use core::f64::consts::PI as Pi;
use qoqo_calculator::CalculatorFloat;
use roqoqo::backends::EvaluatingBackend;
use roqoqo::{operations as ops, Circuit};
use roqoqo_quest::Backend;

fn main() {


    fn creation_qubit_alice(angle_thet: CalculatorFloat, angle_phi: CalculatorFloat) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += ops::RotateY::new(0, angle_thet);
        circuit += ops::RotateZ::new(0, angle_phi);
        circuit
    }

    let circuit_initial = creation_qubit_alice(CalculatorFloat::Float(Pi)/6.0, CalculatorFloat::Float(0f64));

    // On réalise le sous-circuit d'intrication en bas entre le qubit d'intrication et le qubit de Bob
    let mut circuit_intrication_bob = Circuit::new();
    circuit_intrication_bob += ops::Hadamard::new(1);
    circuit_intrication_bob += ops::CNOT::new(1, 2);

    // On réalise le sous-circuit relatif aux deux premiers qubits
    let mut circuit_superieur = Circuit::new();
    circuit_superieur += ops::CNOT::new(0, 1);
    circuit_superieur += ops::Hadamard::new(0);

    // On réalise le sous-circuit de mesure des deux premiers qubits
    let mut circuit_mesure = Circuit::new();
    circuit_mesure += ops::DefinitionBit::new("registre".to_string(), 2, true);
    circuit_mesure += ops::MeasureQubit::new(0, "registre".to_string(), 0);
    circuit_mesure += ops::MeasureQubit::new(1, "registre".to_string(), 1);

    // On réalise le circuit conditionnel selon le résultat classique après mesure
    // Pour rappel I, X, Z, ou XZ
    let mut circuit_conditionnel_z = Circuit::new();
    circuit_conditionnel_z += ops::PauliZ::new(2);
    let mut circuit_conditionnel_x = Circuit::new();
    circuit_conditionnel_x += ops::PauliX::new(2);
    let mut circuit_conditionnel = Circuit::new();
    circuit_conditionnel += ops::PragmaConditional::new("registre".to_string(), 1, circuit_conditionnel_x);
    circuit_conditionnel += ops::PragmaConditional::new("registre".to_string(), 0, circuit_conditionnel_z);

    // On ajoute un élément de vérification, capable de nous donner le résultat reçu par Bob
    // Supposé être celui envoyé par Alice
    let mut verification = Circuit::new();
    verification += ops::DefinitionComplex::new("psi".to_string(), 8, true);
    verification += ops::PragmaGetStateVector::new("psi".to_string(), Some(Circuit::new()));

    // On a ici l'ensemble de nos sous-cirucuit, on fait l'assemblage
    let circuit_principal = circuit_initial
        + circuit_intrication_bob
        + circuit_superieur
        + circuit_mesure
        + circuit_conditionnel
        + verification;

    let backend = Backend::new(3);
    let resultat = backend.run_circuit(&circuit_principal);
    let (resultat_bits, resultat_flottant, resultat_complexe) =
        resultat.unwrap();

    println!("---");
    println!("Résultat Bob : {:?}", resultat_complexe["psi"]);      
}
