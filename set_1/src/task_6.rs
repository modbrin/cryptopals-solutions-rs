// Task: Break repeating-key XOR
// use crate::task_4;
use crate::task_3::{brute_single_byte_xor, rate_english_frequency};
use crate::task_5::repeating_key_xor;

pub fn hamming_distance<T: AsRef<[u8]>>(slice_a: T, slice_b: T) -> Option<u32> {
    let a_ref = slice_a.as_ref();
    let b_ref = slice_b.as_ref();
    if a_ref.len() != b_ref.len() {
        return None;
    }
    Some(
        a_ref
            .iter()
            .zip(b_ref.iter())
            .map(|(&a, &b)| a ^ b)
            .map(|v| {
                (0..8)
                    .map(|idx| (((1u8 << idx) & v) >> idx) as u32)
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn string_distance(str_a: &str, str_b: &str) -> Option<u32> {
    hamming_distance(str_a.as_bytes(), str_b.as_bytes())
}

pub fn normalized_edit_distance<T: AsRef<[u8]>>(data_a: T, data_b: T) -> Option<f64> {
    let len = data_a.as_ref().len();
    hamming_distance(data_a, data_b).map(|dist| dist as f64 / len as f64)
}

pub fn find_keysizes<T: AsRef<[u8]>>(
    cipher: T,
    best_count: Option<u8>,
    keymin: Option<u8>,
    keymax: Option<u8>,
) -> Option<Vec<u8>> {
    let cipher_ref = cipher.as_ref();
    let best_count = best_count.unwrap_or(3);
    let keymin = keymin.unwrap_or(2);
    let keymax = keymax.unwrap_or(40);
    if best_count > keymax - keymin {
        return None;
    }
    let mut distances = Vec::new();
    for keysize in keymin..keymax {
        let chunks_num = cipher_ref.len() / keysize as usize;
        let mut avg_distance: f64 = 0.0;
        for cn in 0..chunks_num - 1 {
            let ks = keysize as usize;
            avg_distance += normalized_edit_distance(
                &cipher_ref[cn..cn + ks],
                &cipher_ref[cn + ks..cn + 2 * ks],
            )?;
        }
        avg_distance /= chunks_num as f64 - 1.0;
        distances.push((keysize, avg_distance));
    }
    distances.sort_by(|(_a_ks, a_ed), (_b_ks, b_ed)| a_ed.partial_cmp(b_ed).unwrap());
    Some(
        distances
            .iter()
            .take(best_count as usize)
            .map(|(ks, _ed)| *ks)
            .collect(),
    )
}

pub fn split_into_transposed_chunks<T: AsRef<[u8]>>(data: T, chunk_size: usize) -> Vec<Vec<u8>> {
    let data_ref = data.as_ref();
    let mut result = Vec::new();
    for chunk_idx in 0..chunk_size {
        let mut chunk = Vec::new();
        let mut idx = chunk_idx;
        while let Some(val) = data_ref.get(idx) {
            chunk.push(*val);
            idx += chunk_size;
        }
        result.push(chunk);
    }
    result
}

/// returns best key
pub fn brute_repeating_key_xor<T: AsRef<[u8]>>(cipher: T) -> Option<Vec<u8>> {
    let cipher_ref = cipher.as_ref();
    let keysizes = find_keysizes(&cipher, None, None, None)?;
    let mut max_rate: f32 = 0.0;
    let mut best_key = Vec::new();
    for &ks in keysizes.iter() {
        let mut guess_key = Vec::new();
        for chunk in split_into_transposed_chunks(&cipher, ks as usize).iter() {
            guess_key.push(brute_single_byte_xor(chunk).map(|(k, ..)| k)?)
        }
        let guess = repeating_key_xor(&cipher_ref, &guess_key.as_ref());
        let rate = rate_english_frequency(&guess);
        if rate > max_rate {
            max_rate = rate;
            best_key = guess_key;
        }
    }
    Some(best_key)
}

#[cfg(test)]
mod tests {
    use crate::task_1::base64_to_bytes;

    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn break_repeating_key_xor_should_pass() {
        assert_eq!(
            string_distance("this is a test", "wokka wokka!!!"),
            Some(37)
        );
        assert_eq!(string_distance("abc", "abcd"), None);

        assert_eq!(
            split_into_transposed_chunks(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
        assert_eq!(
            split_into_transposed_chunks(&[1, 2, 3, 4, 5, 6, 7], 3),
            vec![vec![1, 4, 7], vec![2, 5], vec![3, 6]]
        );
        assert_eq!(
            split_into_transposed_chunks(&[], 3),
            vec![vec![], vec![], vec![]]
        );
        assert_eq!(
            split_into_transposed_chunks(&[1, 2, 3], 1),
            vec![vec![1, 2, 3]]
        );

        // assert_eq!(find_keysizes(&[1u8, 2, 3, 4, 5, 6], None, None, None), Some(vec![1u8, 2, 3]));

        let file = File::open("/home/modbrin/projects/cryptopals-solutions-rs/set_1/res/task6.txt")
            .expect("Failed to open file.");
        let mut file_content = String::new();
        BufReader::new(file)
            .lines()
            .for_each(|line| file_content.push_str(line.unwrap_or(String::from("")).as_str()));

        let cipher = base64_to_bytes(&file_content).expect("Failed to decode base64.");

        let best_key = brute_repeating_key_xor(&cipher).expect("Failed to guess a key.");

        let guess = repeating_key_xor(&cipher, &best_key)
            .iter()
            .map(|&v| v as char)
            .collect::<String>();

        let answer = "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
        assert_eq!(guess, answer);
    }
}
