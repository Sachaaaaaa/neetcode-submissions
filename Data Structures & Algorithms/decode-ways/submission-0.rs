impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        
        // Un '0' au tout début rend le décodage impossible d'office
        if bytes.is_empty() || bytes[0] == b'0' {
            return 0;
        }

        // On initialise nos deux variables pour suivre les combinaisons
        let mut two_back = 1; // Représente f(k-2)
        let mut one_back = 1; // Représente f(k-1)

        for i in 1..bytes.len() {
            let mut current = 0;

            // 1. Est-ce qu'on peut décoder le chiffre actuel tout seul ?
            // Oui, tant que ce n'est pas un '0'
            if bytes[i] != b'0' {
                current += one_back;
            }

            // 2. Est-ce qu'on peut le combiner avec le chiffre précédent ?
            // Oui, si ça forme un nombre entre 10 et 26
            if bytes[i - 1] == b'1' || (bytes[i - 1] == b'2' && bytes[i] <= b'6') {
                current += two_back;
            }

            // Mise à jour des pointeurs pour la prochaine itération
            two_back = one_back;
            one_back = current;
        }

        one_back
    }
}