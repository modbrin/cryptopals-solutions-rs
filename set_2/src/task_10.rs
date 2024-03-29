// Task: Decipher AES in CBC mode
use set_1::task_5::repeating_key_xor;
use set_1::task_7::{aes_decrypt_single_block, RepeatingKey, AES_BLOCK_SIZE};

pub fn decrypt_aes_cbc<T: AsRef<[u8]>>(cipher: T, key: T, iv: T) -> Vec<u8> {
    let cipher_ref = cipher.as_ref();
    let key_ref = key.as_ref();
    let mut iv = iv.as_ref().iter().copied().collect::<Vec<_>>();
    assert_eq!(cipher_ref.len() % AES_BLOCK_SIZE, 0);
    assert!(key_ref.len() > 0);
    let full_blocks = cipher_ref.len() / AES_BLOCK_SIZE;
    let mut rep_key = RepeatingKey::new(key_ref);
    let mut output = Vec::new();
    for i in 0..full_blocks {
        let block_ref = &cipher_ref[i * AES_BLOCK_SIZE..(i + 1) * AES_BLOCK_SIZE];
        let key_part = rep_key.take(AES_BLOCK_SIZE).unwrap();
        let res_block = aes_decrypt_single_block(block_ref, key_part.as_slice());
        let res_block = repeating_key_xor(res_block.as_slice(), iv.as_slice());
        output.extend(res_block.into_iter());
        iv = block_ref.iter().copied().collect::<Vec<_>>();
    }
    let last = *output.last().unwrap() as usize;
    output.truncate(output.len() - last);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use set_1::task_1::base64_to_bytes;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn decrypt_aes_cbc_should_pass() {
        let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
        let iv = vec![0u8; AES_BLOCK_SIZE];

        let file = File::open("res/task10.txt").expect("Failed to open file.");
        let mut file_content = String::new();
        BufReader::new(file)
            .lines()
            .for_each(|line| file_content.push_str(line.unwrap_or(String::from("")).as_str()));

        let cipher = base64_to_bytes(&file_content).expect("Failed to decode base64.");
        let cleartext = decrypt_aes_cbc(cipher.as_ref(), key, iv.as_slice())
            .iter()
            .map(|&v| v as char)
            .collect::<String>();

        let answer = "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
        assert_eq!(cleartext, answer);
    }
}
