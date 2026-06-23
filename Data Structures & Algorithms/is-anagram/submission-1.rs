impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 1. Si les longueurs (en octets) diffèrent, ce n'est pas un anagramme
        if s.len() != t.len() {
            return false;
        }

        // 2. Un dictionnaire de 26 entrées (de 'a' à 'z') initialisé à 0
        let mut dict = [0; 26];

        // 3. .bytes() redonne un accès direct et ultra-rapide à la mémoire (O(1))
        for (byte_s, byte_t) in s.bytes().zip(t.bytes()) {
            // b'a' vaut 97. Si byte_s vaut b'b' (98), alors 98 - 97 = index 1.
            dict[(byte_s - b'a') as usize] += 1;
            dict[(byte_t - b'a') as usize] -= 1;
        }

        // 4. On vérifie si tout est revenu à 0
        dict.iter().all(|&x| x == 0)
    }

}
